use std::f64::consts::PI;

use ::rand::random_range;
use ball::Ball;
use macroquad::{
    audio::{PlaySoundParams, play_sound},
    prelude::*,
};
use particle::{
    ConfettiParticle, ShrinkingParticle,
    emitter::{BaseParticleEmitter, ParticleEmitter},
    system::ParticleSystem,
};
use scenes::{build_balls, scene_1, scene_2, scene_3, scene_4};
use serde::Deserialize;
use toml::from_str;
use wall::Wall;

mod ball;
mod drawer;
mod particle;
mod scenes;
mod wall;

const PHYSICS_STEPS: usize = 100;
const TIMESCALE: f64 = 1.0;

const SCALE: f64 = 0.5;

fn window_conf() -> Conf {
    Conf {
        window_width: (1080.0 * SCALE) as i32,
        window_height: (1920.0 * SCALE) as i32,
        window_title: "BallRace".to_owned(),
        sample_count: 8,
        ..Default::default()
    }
}

const COUNTDOWN_SECONDS: usize = 3;
const RESET_SECONDS: usize = 10;

#[derive(Deserialize)]
pub struct ConfigPosition {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

#[derive(Deserialize)]
pub struct BallConfig {
    name: String,
    r: f32,
    g: f32,
    b: f32,
    radius: f64,
    elasticity: f64,
    sound: String,
}

#[derive(Deserialize)]
pub struct Config {
    balls: Vec<BallConfig>,
    ball_positions: Vec<ConfigPosition>,
    scene: usize,
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        let config_string = std::fs::read_to_string("config.toml").unwrap();
        let mut config = from_str::<Config>(&config_string).unwrap();

        let balls = build_balls(&mut config.ball_positions, &config.balls).await;

        let mut scene = if config.scene == 1 {
            scene_1(balls).await
        } else if config.scene == 2 {
            scene_2(balls).await
        } else if config.scene == 3 {
            scene_3(balls).await
        } else {
            scene_4(balls).await
        };

        let mut maybe_all_won_time = None;
        loop {
            if get_time() >= COUNTDOWN_SECONDS as f64 {
                scene.update();
            }

            scene.draw();

            if get_time().floor() < COUNTDOWN_SECONDS as f64 {
                let text = format!("{}", COUNTDOWN_SECONDS as f64 - get_time().floor(),);
                draw_text_outline(
                    &text,
                    screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                    screen_height() / 2.0,
                    256.0,
                    WHITE,
                    16,
                );
            }

            if scene.winners.len() == scene.balls.len() && maybe_all_won_time.is_none() {
                maybe_all_won_time = Some(get_time());
            }

            if let Some(all_won_time) = maybe_all_won_time {
                let text = format!(
                    "{}",
                    RESET_SECONDS as f64 - (get_time() - all_won_time).floor()
                );

                draw_text_outline(
                    &text,
                    screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                    screen_height() / 2.0,
                    256.0,
                    WHITE,
                    16,
                );

                if get_time() >= all_won_time + RESET_SECONDS as f64 {
                    break;
                }
            }

            next_frame().await;
        }
    }
}

pub struct Scene {
    balls: Vec<Ball>,
    walls: Vec<Wall>,
    winners: Vec<usize>,
    particles: ParticleSystem,
}

const MIN_OVERLAP: f64 = 0.01;

impl Scene {
    pub fn update(&mut self) {
        let dt = get_frame_time() as f64 * TIMESCALE / PHYSICS_STEPS as f64;

        for _ in 0..PHYSICS_STEPS {
            let new_attributes: Vec<(DVec2, DVec2)> = self
                .balls
                .iter()
                .enumerate()
                .map(|(index, ball)| {
                    let mut position_offsets = Vec::new();
                    let mut velocity_offsets = Vec::new();

                    // Gravity
                    velocity_offsets.push(dvec2(0.0, 500.0) * dt);

                    // Walls
                    let wall_intersection_points = self
                        .walls
                        .iter()
                        .filter_map(|wall| {
                            let maybe_intersection_point = ball.get_intersection_point(wall);

                            if let Some(intersection_point) = maybe_intersection_point {
                                let intersection_vector = ball.get_position() - intersection_point;

                                let v_dot = ball
                                    .get_velocity()
                                    .dot(intersection_vector.normalize())
                                    .abs();
                                if v_dot >= 100.0 {
                                    self.particles.spawn(Box::new(ShrinkingParticle::new(
                                        intersection_point,
                                        v_dot.sqrt() / 2.0,
                                        WHITE,
                                        0.2,
                                    )));
                                }

                                if wall.is_goal() && !self.winners.contains(&index) {
                                    self.winners.push(index);

                                    let emitter = BaseParticleEmitter::new(
                                        ball.get_position(),
                                        ball.get_radius(),
                                        |position, _spread| {
                                            Box::new(ConfettiParticle::new(
                                                position,
                                                DVec2::from_angle(random_range(
                                                    (1.25 * PI)..(1.75 * PI),
                                                )) * random_range(100.0..=1000.0),
                                                random_range(4.0..=8.0),
                                                2.0,
                                            ))
                                        },
                                    );

                                    for _ in 0..100 {
                                        self.particles.spawn(emitter.generate_particle());
                                    }
                                }
                            }

                            maybe_intersection_point
                        })
                        .collect::<Vec<DVec2>>();
                    let number_of_wall_intersection_points = wall_intersection_points.len();

                    for intersection_point in wall_intersection_points {
                        let intersection_vector = ball.get_position() - intersection_point;
                        let overlap =
                            MIN_OVERLAP.max(ball.get_radius() - intersection_vector.length());
                        position_offsets.push(intersection_vector.normalize() * overlap);
                        velocity_offsets.push(
                            -((2.0 * ball.get_velocity()).dot(intersection_vector)
                                / (intersection_vector.length() * intersection_vector.length()))
                                * (intersection_vector)
                                * ball.get_elasticity()
                                / number_of_wall_intersection_points as f64,
                        );
                    }

                    // Other balls {
                    for other_ball in self
                        .balls
                        .iter()
                        .filter(|other_ball| ball.get_position() != other_ball.get_position())
                    {
                        let intersection_vector = ball.get_position() - other_ball.get_position();

                        if intersection_vector.length()
                            < ball.get_radius() + other_ball.get_radius()
                        {
                            let v_dot = ball
                                .get_velocity()
                                .dot(intersection_vector.normalize())
                                .abs();
                            if v_dot >= 100.0 {
                                self.particles.spawn(Box::new(ShrinkingParticle::new(
                                    ball.get_position().midpoint(other_ball.get_position()),
                                    v_dot.sqrt() / 2.0,
                                    WHITE,
                                    0.2,
                                )));
                            }

                            let overlap = MIN_OVERLAP.max(
                                ball.get_radius() + other_ball.get_radius()
                                    - intersection_vector.length(),
                            );
                            position_offsets.push(intersection_vector.normalize() * overlap);
                            velocity_offsets.push(
                                -(2.0 * other_ball.get_mass()
                                    / (ball.get_mass() + other_ball.get_mass()))
                                    * ((ball.get_velocity() - other_ball.get_velocity())
                                        .dot(intersection_vector)
                                        / (intersection_vector.length()
                                            * intersection_vector.length()))
                                    * (intersection_vector)
                                    * ball.get_elasticity(),
                            );
                        }
                    }

                    let new_position = ball.get_position() + position_offsets.iter().sum::<DVec2>();
                    let new_velocity = ball.get_velocity() + velocity_offsets.iter().sum::<DVec2>();

                    let dv = new_velocity.distance(ball.get_velocity());
                    let _v_dot = new_velocity.dot(ball.get_velocity());

                    if dv >= 100.0 {
                        play_sound(
                            ball.get_sound(),
                            PlaySoundParams {
                                looped: false,
                                volume: ((dv - 100.0) / 2000.0).min(1.0) as f32,
                            },
                        );
                    }

                    (new_position, new_velocity)
                })
                .collect();

            for (ball, (new_position, new_velocity)) in self.balls.iter_mut().zip(new_attributes) {
                ball.set_position(new_position);
                ball.set_velocity(new_velocity);

                ball.update(dt);
            }

            self.particles.update(dt);
        }
    }

    pub fn draw(&self) {
        for ball in self.balls.iter() {
            ball.draw();
        }

        for wall in self.walls.iter() {
            wall.draw();
        }

        self.particles.draw();

        for (index, winner_index) in self.winners.iter().enumerate() {
            let winner = self.balls.get(*winner_index).unwrap();
            let text = format!("{}. {}", index + 1, winner.get_name());
            let font_size = 64.0;

            draw_text_outline(
                &text,
                screen_width() / 2.0 - measure_text(&text, None, font_size as u16, 1.0).width / 2.0,
                font_size + font_size * index as f32,
                font_size,
                winner.get_name_color(),
                4,
            );
        }
    }
}

pub fn draw_text_outline(text: &str, x: f32, y: f32, font_size: f32, color: Color, thickness: i32) {
    for i in -thickness..=thickness {
        for j in -thickness..=thickness {
            draw_text(&text, x + i as f32, y + j as f32, font_size, BLACK);
        }
    }

    draw_text(&text, x, y, font_size, color);
}
