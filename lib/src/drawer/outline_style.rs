use glam::DVec2;
use palette::Srgba;
use render_agnostic::Renderer;

use crate::{
    ball::{Ball, PhysicsBall},
    drawer::BallStyle,
};

#[derive(Clone)]
pub struct OutlineStyle {
    color: Srgba,
}

impl OutlineStyle {
    pub fn new(color: Srgba) -> Self {
        Self { color }
    }

    pub fn get_color(&self) -> Srgba {
        self.color
    }
}

impl BallStyle for OutlineStyle {
    fn init(&mut self, _ball: &PhysicsBall) {}
    fn update(&self, _ball: &PhysicsBall, _dt: f64) -> Box<dyn BallStyle> {
        Box::new(self.clone())
    }

    fn render(&self, ball: &Ball, renderer: &mut dyn Renderer) {
        renderer.render_circle_lines(
            ball.get_position(),
            ball.get_radius(),
            2.0,
            self.get_color(),
        );

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
