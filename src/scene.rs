use std::f64::consts::PI;

use ::rand::random_range;
use macroquad::{
    audio::{PlaySoundParams, play_sound},
    prelude::*,
};

use crate::{
    ball::Ball,
    draw_text_outline,
    particle::{
        ConfettiParticle, ShrinkingParticle,
        emitter::{BaseParticleEmitter, ParticleEmitter},
        system::ParticleSystem,
    },
    wall::Wall,
};

const MIN_OVERLAP: f64 = 0.01;

pub struct Scene {
    balls: Vec<Ball>,
    walls: Vec<Wall>,
    winners: Vec<usize>,
    particles: ParticleSystem,
    timescale: f64,
    physics_steps: usize,
}

impl Scene {
    pub fn new(balls: Vec<Ball>, walls: Vec<Wall>, timescale: f64, physics_steps: usize) -> Self {
        Self {
            balls,
            walls,
            winners: Vec::new(),
            particles: ParticleSystem::default(),
            timescale,
            physics_steps,
        }
    }

    pub fn get_balls(&self) -> &[Ball] {
        &self.balls
    }

    pub fn get_winners(&self) -> &[usize] {
        &self.winners
    }

    pub fn get_timescale(&self) -> f64 {
        self.timescale
    }

    pub fn get_physics_steps(&self) -> usize {
        self.physics_steps
    }

    pub fn update(&mut self) {
        let dt = get_frame_time() as f64 * self.get_timescale() / self.get_physics_steps() as f64;

        for _ in 0..self.get_physics_steps() {
            self.step_physics(dt);
        }
    }

    pub fn step_physics(&mut self, dt: f64) {
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
                    let overlap = MIN_OVERLAP.max(ball.get_radius() - intersection_vector.length());
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

                    if intersection_vector.length() < ball.get_radius() + other_ball.get_radius() {
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

    pub fn draw(&self) {
        for wall in self.walls.iter() {
            wall.draw();
        }

        for ball in self.balls.iter() {
            ball.draw();
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
            );
        }
    }
}
