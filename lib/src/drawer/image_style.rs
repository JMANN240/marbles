use glam::DVec2;
use palette::Srgba;
use render_agnostic::Renderer;

use crate::{
    ball::{Ball, PhysicsBall},
    drawer::BallStyle,
};

#[derive(Clone)]
pub struct ImageStyle {
    color: Srgba,
    image_name: String,
    theta: f64,
}

impl ImageStyle {
    pub fn new(color: Srgba, image_name: String,) -> Self {
        Self { color, image_name, theta: 0.0 }
    }

    pub fn get_color(&self) -> Srgba {
        self.color
    }

    pub fn get_image_name(&self) -> &str {
        &self.image_name
    }
}

impl BallStyle for ImageStyle {
    fn init(&mut self, _ball: &PhysicsBall) {}
    fn update(&self, ball: &PhysicsBall, dt: f64) -> Box<dyn BallStyle> {
        let mut new_style = self.clone();

        new_style.theta += ball.get_velocity().x / (10.0 * ball.get_radius() / 8.0) * dt;

        Box::new(new_style)
    }

    fn render(&self, ball: &Ball, renderer: &mut dyn Renderer) {
        renderer.render_image(self.get_image_name(), ball.get_position(), ball.get_radius() * 2.0, ball.get_radius() * 2.0, DVec2::splat(0.5), self.theta);

        renderer.render_circle_lines(ball.get_position(), ball.get_radius(), 2.0, self.get_color());

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
