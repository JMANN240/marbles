use glam::DVec2;
use palette::Srgba;

use crate::{
    ball::Ball,
    powerup::Powerup,
    rendering::{HorizontalTextAnchor, Render, Renderer, TextAnchor2D, VerticalTextAnchor},
};

pub struct ChangeElasticityConfig {
    radius: f64,
    amount: f64,
}

impl ChangeElasticityConfig {
    pub fn new(radius: f64, amount: f64) -> Self {
        Self { radius, amount }
    }

    pub fn build(&self, position: DVec2) -> ChangeElasticity {
        ChangeElasticity::new(position, self.radius, self.amount)
    }
}

#[derive(Clone)]
pub struct ChangeElasticity {
    time: f64,
    position: DVec2,
    radius: f64,
    amount: f64,
}

impl ChangeElasticity {
    pub fn new(position: DVec2, radius: f64, amount: f64) -> Self {
        Self {
            time: 0.0,
            position,
            radius,
            amount,
        }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position + 2.0 * (self.time * 4.0).sin() * DVec2::Y
    }
}

impl Powerup for ChangeElasticity {
    fn is_colliding_with(&self, ball: &Ball) -> bool {
        self.get_position().distance(ball.get_position()) < self.radius + ball.get_radius()
    }

    fn on_collision(&self, ball: &mut Ball) {
        ball.set_elasticity(ball.get_elasticity() * self.amount);
    }

    fn update(&mut self, dt: f64) {
        self.time += dt;
    }
}

impl Render for ChangeElasticity {
    fn render(&self, renderer: &mut dyn Renderer) {
        let color = Srgba::new(0.0, 0.5, 1.0, 1.0);

        renderer.render_circle_lines(
            self.get_position(),
            8.0,
            1.0,
            color,
        );


        renderer.render_text(
            &format!("Elasticity x{:.1}", self.amount),
            self.get_position() - DVec2::Y * 2.0 * self.radius,
            TextAnchor2D {
                horizontal: HorizontalTextAnchor::Center,
                vertical: VerticalTextAnchor::Bottom,
            },
            16.0,
            color,
        );
    }
}
