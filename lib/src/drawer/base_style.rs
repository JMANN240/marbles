use anchor2d::{
    Anchor2D, HorizontalAnchor, VerticalAnchor, VerticalAnchorContext, VerticalAnchorValue,
};
use glam::DVec2;
use palette::Srgba;
use render_agnostic::Renderer;

use crate::{
    ball::{Ball, PhysicsBall},
    drawer::BallStyle,
};

#[derive(Clone)]
pub struct BaseStyle {
    color: Srgba,
}

impl BaseStyle {
    pub fn new(color: Srgba) -> Self {
        Self { color }
    }

    pub fn get_color(&self) -> Srgba {
        self.color
    }
}

impl BallStyle for BaseStyle {
    fn init(&mut self, _ball: &PhysicsBall) {}
    fn update(&mut self, _ball: &PhysicsBall, _dt: f64) {}

    fn render(&self, ball: &Ball, renderer: &mut dyn Renderer) {
        renderer.render_circle(ball.get_position(), ball.get_radius(), self.get_color());

        renderer.render_text_outline(
            ball.get_name(),
            ball.get_position() - DVec2::Y * 2.0 * ball.get_radius(),
            anchor2d::CGB,
            20.0,
            1.0,
            ball.get_name_color(),
            Srgba::new(0.0, 0.0, 0.0, 1.0),
        );
    }
}
