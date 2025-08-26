use std::f64::consts::PI;

use glam::{DVec2, dvec2};
use palette::Srgba;

use crate::{
    ball::{Ball, PhysicsBall},
    drawer::BallStyle,
    rendering::{Anchor2D, HorizontalAnchor, Renderer, VerticalAnchor},
    wall::straight_wall::Line,
};

#[derive(Clone)]
pub struct IkeaStyle {
    theta: f64,
}

impl Default for IkeaStyle {
    fn default() -> Self {
        Self { theta: 0.0 }
    }
}

impl BallStyle for IkeaStyle {
    fn init(&mut self, _ball: &PhysicsBall) {}

    fn update(&mut self, ball: &PhysicsBall, dt: f64) {
        self.theta += ball.get_velocity().x * 0.1 * dt;
    }

    fn render(&self, ball: &Ball, renderer: &mut dyn Renderer) {
        renderer.render_circle(
            ball.get_position(),
            ball.get_radius(),
            Srgba::new(0x00, 0x6A, 0xA7, 255).into(),
        );

        let yellow = Srgba::new(0xFE, 0xCC, 0x02, 255);

        renderer.render_line(
            &Line::new(
                dvec2(
                    ball.get_position().x + ball.get_radius() * self.theta.cos(),
                    ball.get_position().y + ball.get_radius() * self.theta.sin(),
                ),
                dvec2(
                    ball.get_position().x + ball.get_radius() * (self.theta + PI).cos(),
                    ball.get_position().y + ball.get_radius() * (self.theta + PI).sin(),
                ),
            ),
            2.0,
            yellow.into(),
        );

        renderer.render_line(
            &Line::new(
                dvec2(
                    ball.get_position().x + ball.get_radius() * (self.theta + 2.0 / 3.0 * PI).cos(),
                    ball.get_position().y + ball.get_radius() * (self.theta + 2.0 / 3.0 * PI).sin(),
                ),
                dvec2(
                    ball.get_position().x + ball.get_radius() * (self.theta + 4.0 / 3.0 * PI).cos(),
                    ball.get_position().y + ball.get_radius() * (self.theta + 4.0 / 3.0 * PI).sin(),
                ),
            ),
            2.0,
            yellow.into(),
        );

        renderer.render_text(
            ball.get_name(),
            ball.get_position() - DVec2::Y * 2.0 * ball.get_radius(),
            Anchor2D {
                horizontal: HorizontalAnchor::Center,
                vertical: VerticalAnchor::Bottom,
            },
            20.0,
            yellow.into(),
        );
    }
}
