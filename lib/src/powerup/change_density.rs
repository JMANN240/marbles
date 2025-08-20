use std::{f64::consts::PI, sync::Arc};

use glam::DVec2;
use palette::Srgba;
use particula_rs::ParticleSystem;
use rand::random_range;

use crate::{
    ball::Ball,
    particle::{emitter::BallParticleEmitter, system::BallParticleSystem, ParticleLayer, ShrinkingParticle},
    powerup::Powerup,
    rendering::{HorizontalTextAnchor, Render, Renderer, TextAnchor2D, VerticalTextAnchor},
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
}

impl ChangeDensity {
    pub fn new(position: DVec2, radius: f64, amount: f64) -> Self {
        let mut particles = BallParticleSystem::default();

        particles.add_emitter(
            BallParticleEmitter::new(
                position,
                16.0,
                Arc::new(move |position| {
                    Box::new(ShrinkingParticle::new(
                        position
                            + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                * random_range(8.0..12.0),
                        DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                            * random_range(8.0..16.0) + 10.0 * amount * DVec2::Y,
                        random_range(1.0..=4.0),
                        Srgba::new(
                            random_range(0.4..=0.6),
                            0.0,
                            0.0,
                            1.0,
                        ),
                        random_range(0.5..1.0),
                        ParticleLayer::Back,
                    ))
                }),
            )
        );

        Self {
            time: 0.0,
            position,
            radius,
            amount,
            particles,
        }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position + 2.0 * (self.time * 4.0).sin() * DVec2::Y
    }

    pub fn get_particles(&self) -> &BallParticleSystem {
        &self.particles
    }
}

impl Powerup for ChangeDensity {
    fn is_colliding_with(&self, ball: &Ball) -> bool {
        self.get_position().distance(ball.get_position()) < self.radius + ball.get_radius()
    }

    fn on_collision(&self, ball: &mut Ball) {
        ball.set_density(ball.get_density() * self.amount);
    }

    fn update(&mut self, dt: f64) {
        self.time += dt;

        self.particles.update(dt);
    }
}

impl Render for ChangeDensity {
    fn render(&self, renderer: &mut dyn Renderer) {
        let color = Srgba::new(0.5, 0.0, 0.0, 1.0);

        self.particles.render_back(renderer);

        renderer.render_circle_lines(self.get_position(), 8.0, 1.0, color);

        renderer.render_text(
            &format!("Density x{:.1}", self.amount),
            self.get_position() - DVec2::Y * 2.0 * self.radius,
            TextAnchor2D {
                horizontal: HorizontalTextAnchor::Center,
                vertical: VerticalTextAnchor::Bottom,
            },
            16.0,
            color,
        );

        self.particles.render_front(renderer);
    }
}
