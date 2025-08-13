use std::f64::consts::PI;

use ::rand::random_range;
use glam::{DVec2, dvec2};
use palette::Srgba;
use particula_rs::{ParticleEmitter, ParticleSystem, VecParticleSystem};

use crate::{
    ball::Ball,
    collision::Collision,
    particle::{ConfettiParticle, ParticleLayer, RenderParticle, ShrinkingParticle},
    rendering::{Render, Renderer},
    wall::Wall,
};

const MIN_OVERLAP: f64 = 0.01;

pub trait SceneParticleEmitter: ParticleEmitter<ParticleType = Box<dyn RenderParticle<DVec2>>> + Send + Sync {
    fn clone_box(&self) -> Box<dyn SceneParticleEmitter>;
}

pub type SceneParticleSystem = VecParticleSystem<
    Box<dyn RenderParticle<DVec2>>,
    Box<dyn SceneParticleEmitter>,
>;

impl Clone for Box<dyn RenderParticle<DVec2>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn SceneParticleEmitter> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Scene {
    balls: Vec<Ball>,
    walls: Vec<Box<dyn Wall>>,
    winners: Vec<usize>,
    particles: SceneParticleSystem,
}

impl Scene {
    pub fn new(balls: Vec<Ball>, walls: Vec<Box<dyn Wall>>) -> Self {
        Self {
            balls,
            walls,
            winners: Vec::new(),
            particles: VecParticleSystem::default(),
        }
    }

    pub fn get_balls(&self) -> &Vec<Ball> {
        &self.balls
    }

    pub fn get_walls(&self) -> &Vec<Box<dyn Wall>> {
        &self.walls
    }

    pub fn get_winners(&self) -> &Vec<usize> {
        &self.winners
    }

    pub fn get_particles(&self) -> &SceneParticleSystem {
        &self.particles
    }

    pub fn update(&mut self, dt: f64, timescale: f64, physics_steps: usize) -> Vec<Collision> {
        let step_dt = dt * timescale / physics_steps as f64;

        let mut collisions = Vec::new();

        for _ in 0..physics_steps {
            collisions.append(&mut self.step_physics(step_dt));
        }

        collisions
    }

    pub fn step_physics(&mut self, dt: f64) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for wall in self.walls.iter_mut() {
            wall.update(dt);
        }

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
                        let maybe_intersection_point = ball.get_intersection_point(wall.as_ref());

                        if let Some(intersection_point) = maybe_intersection_point {
                            let intersection_vector = ball.get_position() - intersection_point;

                            let v_dot = ball
                                .get_velocity()
                                .dot(intersection_vector.normalize())
                                .abs();
                            if v_dot >= 100.0 {
                                self.particles.add_particle(Box::new(ShrinkingParticle::new(
                                    intersection_point,
                                    DVec2::ZERO,
                                    v_dot.sqrt() / 2.0,
                                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                                    0.2,
                                    ParticleLayer::Front,
                                )));
                            }

                            if wall.is_goal() && !self.winners.contains(&index) {
                                self.winners.push(index);

                                for _ in 0..100 {
                                    self.particles.add_particle(Box::new(ConfettiParticle::new(
                                        ball.get_position()
                                            + ball.get_radius()
                                                * DVec2::from_angle(random_range(0.0..(2.0 * PI))),
                                        DVec2::from_angle(random_range((1.25 * PI)..(1.75 * PI)))
                                            * random_range(100.0..=1000.0),
                                        random_range(4.0..=8.0),
                                        2.0,
                                        ParticleLayer::random(),
                                    )));
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
                            self.particles.add_particle(Box::new(ShrinkingParticle::new(
                                ball.get_position().midpoint(other_ball.get_position()),
                                DVec2::ZERO,
                                v_dot.sqrt() / 2.0,
                                Srgba::new(1.0, 1.0, 1.0, 1.0),
                                0.2,
                                ParticleLayer::Front,
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
                    let volume = ((dv - 100.0) / 2000.0).min(1.0) as f32;

                    collisions.push(Collision::new(ball.get_sound_path().to_path_buf(), volume));
                }

                (new_position, new_velocity)
            })
            .collect();

        for (ball, (new_position, new_velocity)) in self.balls.iter_mut().zip(new_attributes) {
            ball.set_position(new_position);

            let dv = new_velocity.distance(ball.get_velocity());
            if dv >= 100.0 {
                ball.handle_collision(new_velocity);
            }
            ball.set_velocity(new_velocity);

            ball.update(dt);
        }

        self.particles.update(dt);

        collisions
    }
}

impl Render for Scene {
    fn render(&self, renderer: &mut dyn Renderer) {
        for wall in self.get_walls().iter() {
            wall.render(renderer);
        }

        for ball in self.get_balls().iter() {
            ball.render(renderer);
        }

        for particle in self.get_particles().iter_particles() {
            particle.render(renderer);
        }
    }
}
