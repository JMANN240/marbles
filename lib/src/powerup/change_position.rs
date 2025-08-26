use std::{f64::consts::PI, ops::RangeInclusive, sync::Arc};

use anchor2d::{
    Anchor2D, HorizontalAnchor, VerticalAnchor, VerticalAnchorContext, VerticalAnchorValue,
};
use glam::{DVec2, dvec2};
use palette::Srgba;
use particula_rs::ParticleSystem;
use rand::random_range;

use crate::{
    ball::Ball,
    particle::{
        ParticleLayer, StaticParticle, emitter::BallParticleEmitter, system::BallParticleSystem,
    },
    powerup::Powerup,
    rendering::{Render, Renderer},
};

pub struct ChangePositionConfig {
    radius: f64,
    x_range: RangeInclusive<f64>,
    y_range: RangeInclusive<f64>,
}

impl ChangePositionConfig {
    pub fn new(radius: f64, x_range: RangeInclusive<f64>, y_range: RangeInclusive<f64>) -> Self {
        Self {
            radius,
            x_range,
            y_range,
        }
    }

    pub fn build(&self, position: DVec2, name: impl Into<String>) -> ChangePosition {
        ChangePosition::new(
            position,
            name,
            self.radius,
            self.x_range.clone(),
            self.y_range.clone(),
        )
    }
}

#[derive(Clone)]
pub struct ChangePosition {
    time: f64,
    position: DVec2,
    name: String,
    radius: f64,
    x_range: RangeInclusive<f64>,
    y_range: RangeInclusive<f64>,
    particles: BallParticleSystem,
    is_active: bool,
}

impl ChangePosition {
    pub fn new(
        position: DVec2,
        name: impl Into<String>,
        radius: f64,
        x_range: RangeInclusive<f64>,
        y_range: RangeInclusive<f64>,
    ) -> Self {
        let mut particles = BallParticleSystem::default();

        particles.add_emitter(BallParticleEmitter::new(
            position,
            8.0,
            Arc::new(move |position| {
                let position_offset =
                    DVec2::from_angle(random_range(0.0..(2.0 * PI))) * random_range(16.0..32.0);

                let value = random_range(0.5..=1.0);

                Box::new(StaticParticle::new(
                    position + position_offset,
                    random_range(1.0..=4.0),
                    Srgba::new(value, value, value, 1.0),
                    0.125,
                    ParticleLayer::random(),
                ))
            }),
        ));

        Self {
            time: 0.0,
            position,
            name: name.into(),
            radius,
            x_range,
            y_range,
            particles,
            is_active: true,
        }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position + 2.0 * (self.time * 4.0).sin() * DVec2::Y
    }
}

impl Powerup for ChangePosition {
    fn is_colliding_with(&self, ball: &Ball) -> bool {
        self.get_position().distance(ball.get_position()) < self.radius + ball.get_radius()
    }

    fn on_collision(&mut self, ball: &mut Ball) {
        if self.is_active {
            ball.set_position(dvec2(
                random_range(self.x_range.clone()),
                random_range(self.y_range.clone()),
            ));

            let mut new_particles = BallParticleSystem::default();

            for particle in self.particles.iter_particles() {
                new_particles.add_particle(particle.clone());
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

impl Render for ChangePosition {
    fn render(&self, renderer: &mut dyn Renderer) {
        let color = Srgba::new(1.0, 1.0, 1.0, 1.0);

        self.particles.render_back(renderer);

        if self.is_active {
            renderer.render_circle_lines(self.get_position(), 8.0, 1.0, color);

            renderer.render_text(
                &self.name,
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
