use std::{f64::consts::PI, path::PathBuf, sync::Arc};

use glam::{dvec2, DVec2};
use palette::Srgba;
use particula_rs::ParticleSystem;
use rand::random_range;
use serde::Deserialize;

use crate::{ball::{Ball, PhysicsBall}, drawer::{glow_style::GlowStyle, ikea_style::IkeaStyle, outline_style::OutlineStyle, tail_style::TailStyle}, particle::{emitter::BallParticleEmitter, FireParticle, ParticleLayer, ShrinkingParticle}};

pub mod ball;
pub mod collision;
pub mod drawer;
pub mod level;
pub mod levels;
pub mod particle;
pub mod posting;
pub mod powerup;
pub mod rendering;
pub mod scene;
pub mod scenes;
pub mod simulation;
pub mod util;
pub mod wall;

#[derive(Deserialize, Clone)]
pub struct BallConfig {
    name: String,
    r: f32,
    g: f32,
    b: f32,
    radius: f64,
    density: f64,
    elasticity: f64,
    sound: String,
}

impl BallConfig {
    pub fn build(&self, position: DVec2, velocity: DVec2) -> Ball {
        let color = Srgba::new(self.r, self.g, self.b, 1.0);

        let position = dvec2(
            position.x + random_range(-8.0..=8.0),
            position.y + random_range(-8.0..=8.0),
        );

        let mut ball = Ball::new(
            self.name.clone(),
            color,
            PhysicsBall::new(
                position,
                velocity,
                self.radius,
                self.density,
                self.elasticity,
            ),
            if self.name == "Fireball" {
                Box::new(TailStyle::new(
                    Srgba::new(1.0, 1.0, 0.0, 1.0),
                    Srgba::new(0.9, 0.1, 0.1, 1.0),
                    100,
                    10,
                ))
            } else if self.name == "White Light" {
                Box::new(GlowStyle::new(
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                    16,
                ))
            } else if self.name == "IKEA" {
                Box::new(IkeaStyle::default())
            } else if self.name == "Black Hole" {
                Box::new(GlowStyle::new(
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                    Srgba::new(0.5, 0.0, 1.0, 1.0),
                    12,
                ))
            } else if self.name == "Green Machine" {
                Box::new(OutlineStyle::new(Srgba::new(0.0, 0.9, 0.1, 1.0)))
            } else {
                Box::new(TailStyle::new(
                    color,
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                    100,
                    10,
                ))
            },
            PathBuf::from(self.sound.clone()),
        );

        if self.name == "Fireball" {
            ball.get_particles_mut()
                .add_emitter(BallParticleEmitter::new(
                    position,
                    120.0,
                    Arc::new(|position| {
                        Box::new(FireParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(0.0..=8.0),
                            4.0,
                            0.5,
                            ParticleLayer::random(),
                        ))
                    }),
                ));
        }

        if self.name == "White Light" {
            ball.get_particles_mut()
                .add_emitter(BallParticleEmitter::new(
                    position,
                    32.0,
                    Arc::new(|position| {
                        Box::new(ShrinkingParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(8.0..12.0),
                            DVec2::ZERO,
                            1.0,
                            Srgba::new(1.0, 1.0, 1.0, 1.0),
                            0.125,
                            ParticleLayer::random(),
                        ))
                    }),
                ));
        }

        if self.name == "Black Hole" {
            let _radius = ball.get_radius();

            ball.get_particles_mut()
                .add_emitter(BallParticleEmitter::new(
                    position,
                    16.0,
                    Arc::new(|position| {
                        Box::new(ShrinkingParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(8.0..12.0),
                            DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                * random_range(8.0..16.0),
                            random_range(1.0..=4.0),
                            Srgba::new(
                                random_range(0.25..=0.5),
                                0.0,
                                random_range(0.75..=1.0),
                                1.0,
                            ),
                            random_range(0.25..0.75),
                            ParticleLayer::Back,
                        ))
                    }),
                ));
        }

        ball
    }
}

#[derive(Deserialize)]
pub struct Config {
    balls: Vec<BallConfig>,
    scene: usize,
}

impl Config {
    pub fn get_balls(&self) -> &Vec<BallConfig> {
        &self.balls
    }

    pub fn get_scene(&self) -> usize {
        self.scene
    }
}

pub const ENGAGEMENTS: [&str; 8] = [
    "Pick one!",
    "Choose a winner!",
    "Who will win?",
    "Choose one!",
    "Take a guess!",
    "Guess the winner!",
    "Ok, now THIS is epic!",
    "You'll never guess!",
];
