use std::{f64::consts::PI, sync::Arc};

use anchor2d::{
    Anchor2D, HorizontalAnchor, VerticalAnchor, VerticalAnchorContext, VerticalAnchorValue,
};
use glam::DVec2;
use palette::Srgba;
use particula_rs::ParticleSystem;
use rand::random_range;

use crate::{
    ball::Ball,
    particle::{
        ParticleLayer, ShrinkingParticle, emitter::BallParticleEmitter, system::BallParticleSystem,
    },
    powerup::Powerup,
    rendering::{Render, Renderer},
};

pub struct ChangeDensityConfig {
    radius: f64,
    amount: f64,
}

impl ChangeDensityConfig {
    pub fn new(radius: f64, amount: f64) -> Self {
        Self { radius, amount }
    }

    pub fn build(&self, position: DVec2) -> ChangeDensity {
        ChangeDensity::new(position, self.radius, self.amount)
    }
}

#[derive(Clone)]
pub struct ChangeDensity {
    time: f64,
    position: DVec2,
    radius: f64,
    amount: f64,
    particles: BallParticleSystem,
    is_active: bool,
}

impl ChangeDensity {
    pub fn new(position: DVec2, radius: f64, amount: f64) -> Self {
        let mut particles = BallParticleSystem::default();

        particles.add_emitter(BallParticleEmitter::new(
            position,
            16.0,
            Arc::new(move |position| {
                Box::new(ShrinkingParticle::new(
                    position
                        + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                            * random_range(8.0..12.0),
                    DVec2::from_angle(random_range(0.0..(2.0 * PI))) * random_range(8.0..16.0)
                        + 10.0 * amount * DVec2::Y,
                    random_range(1.0..=4.0),
                    Srgba::new(random_range(0.4..=0.6), 0.0, 0.0, 1.0),
                    random_range(0.5..1.0),
                    ParticleLayer::random(),
                ))
            }),
        ));

        Self {
            time: 0.0,
            position,
            radius,
            amount,
            particles,
            is_active: true,
        }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position + 2.0 * (self.time * 4.0).sin() * DVec2::Y
    }
}

impl Powerup for ChangeDensity {
    fn is_colliding_with(&self, ball: &Ball) -> bool {
        self.get_position().distance(ball.get_position()) < self.radius + ball.get_radius()
    }

    fn on_collision(&mut self, ball: &mut Ball) {
        if self.is_active {
            ball.set_density(ball.get_density() * self.amount);

            let mut new_particles = BallParticleSystem::default();

            for particle in self.particles.iter_particles() {
                new_particles.add_particle(particle.clone());
            }

            for emitter in self.particles.iter_emitters() {
                ball.get_particles_mut().add_emitter(emitter.clone());
            }

            self.particles = new_particles;

            self.is_active = false;
        }
    }

    fn update(&mut self, dt: f64) {
        self.time += dt;

        self.particles.update(dt);
    }
}

impl Render for ChangeDensity {
    fn render(&self, renderer: &mut dyn Renderer) {
        let color = Srgba::new(1.0, 0.25, 0.25, 1.0);

        self.particles.render_back(renderer);

        if self.is_active {
            renderer.render_circle_lines(self.get_position(), 8.0, 1.0, color);

            renderer.render_text(
                &format!("Density x{:.1}", self.amount),
                self.get_position() - DVec2::Y * 2.0 * self.radius,
                Anchor2D::new(
                    HorizontalAnchor::Center,
                    VerticalAnchor::new(
                        VerticalAnchorContext::Graphics,
                        VerticalAnchorValue::Bottom,
                    ),
                ),
                20.0,
                color,
            );
        }

        self.particles.render_front(renderer);
    }
}
