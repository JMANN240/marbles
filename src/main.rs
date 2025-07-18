use ball::Ball;
use macroquad::{
    audio::{PlaySoundParams, play_sound},
    prelude::*,
};
use scenes::{scene_1, scene_2, scene_3, scene_4};
use wall::Wall;

mod ball;
mod drawer;
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

const COUNTDOWN_SECONDS: usize = 5;

#[macroquad::main(window_conf)]
async fn main() {
    let mut scene = scene_4().await;

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
        next_frame().await;
    }
}

pub struct Scene {
    balls: Vec<Box<dyn Ball>>,
    walls: Vec<Wall>,
    winners: Vec<usize>,
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
                            let intersection_point = ball.get_intersection_point(wall);

                            if wall.is_goal()
                                && intersection_point.is_some()
                                && !self.winners.contains(&index)
                            {
                                self.winners.push(index);
                            }

                            intersection_point
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
                ball.set_position(new_position + new_velocity * dt);
                ball.set_velocity(new_velocity);
            }
        }
    }

    pub fn draw(&self) {
        for ball in self.balls.iter() {
            ball.draw();
        }

        for wall in self.walls.iter() {
            wall.draw();
        }

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
