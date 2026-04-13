use std::{f64::consts::PI, path::{Path, PathBuf}, sync::Arc};

use glam::{DVec2, dvec2};
use palette::Srgba;
use particula_rs::ParticleSystem;
use rand::random_range;
use serde::{Deserialize, Serialize};

use crate::{ball::{Ball, PhysicsBall}, drawer::{base_style::BaseStyle, glow_style::GlowStyle, ikea_style::IkeaStyle, image_style::ImageStyle, outline_style::OutlineStyle, tail_style::TailStyle}, particle::{FireParticle, ParticleLayer, ShrinkingParticle, emitter::BallParticleEmitter}};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Marble {
    pub id: i64,
    pub name: String,
    #[serde(with = "palette::serde::as_array")]
    pub color: Srgba,
    pub radius: f64,
    pub density: f64,
    pub elasticity: f64,
    pub sound_path: PathBuf,
    pub maybe_image_path: Option<PathBuf>,
    pub active: bool,
}

impl Marble {
    pub fn build(&self, position: DVec2, velocity: DVec2) -> Ball {
        let position = dvec2(
            position.x + random_range(-8.0..=8.0),
            position.y + random_range(-8.0..=8.0),
        );

        let id = match self.name.as_str() {
            "Blue's Wife" => String::from("Deep Blue"),
            "White's Brother" => String::from("White Light"),
            name => name.to_string(),
        };

        let mut ball = Ball::new(
            id,
            self.name.clone(),
            self.color,
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
            } else if self.name == "White Light" || self.name == "White's Brother" {
                Box::new(GlowStyle::new(
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                    16,
                ))
            } else if self.name == "IKEA" || self.name == "IKEA Jr." {
                Box::new(IkeaStyle::default())
            } else if self.name == "Black Hole" {
                Box::new(GlowStyle::new(
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                    Srgba::new(0.5, 0.0, 1.0, 1.0),
                    12,
                ))
            } else if self.name == "Green Machine" {
                Box::new(OutlineStyle::new(Srgba::new(0.0, 0.9, 0.1, 1.0)))
            } else if self.name == "Deep Blue" || self.name == "Blue's Wife" {
                Box::new(TailStyle::new(
                    self.color,
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                    100,
                    10,
                ))
            } else if let Some(image_path) = &self.maybe_image_path {
                Box::new(ImageStyle::new(self.color, image_path.to_str().unwrap().to_string()))
            } else {
                Box::new(BaseStyle::new(self.color))
            },
            Path::new("ball_sounds").join(&self.sound_path),
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

        if self.name == "White Light" || self.name == "White's Brother" {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteMarble {
    pub name: String,
    #[serde(with = "palette::serde::as_array")]
    pub color: Srgba,
    pub radius: f64,
    pub density: f64,
    pub elasticity: f64,
    pub sound_path: PathBuf,
    pub maybe_image_path: Option<PathBuf>,
    pub active: bool,
}
