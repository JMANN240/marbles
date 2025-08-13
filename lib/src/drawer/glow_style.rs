use glam::DVec2;
use palette::Srgba;

use crate::{
    ball::{Ball, PhysicsBall},
    drawer::BallStyle,
    rendering::{HorizontalTextAnchor, Renderer, TextAnchor2D, VerticalTextAnchor},
    util::lerp_color,
};

#[derive(Clone)]
pub struct GlowStyle {
    ball_color: Srgba,
    glow_color: Srgba,
    size: usize,
}

impl GlowStyle {
    pub fn new(ball_color: Srgba, glow_color: Srgba, size: usize) -> Self {
        Self {
            ball_color,
            glow_color,
            size,
        }
    }

    pub fn get_ball_color(&self) -> Srgba {
        self.ball_color
    }

    pub fn get_glow_color(&self) -> Srgba {
        self.glow_color
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl BallStyle for GlowStyle {
    fn init(&mut self, _ball: &PhysicsBall) {}
    fn update(&mut self, _ball: &PhysicsBall) {}

    fn render(&self, ball: &Ball, renderer: &mut dyn Renderer) {
        for i in 0..=self.get_size() {
            let i = self.get_size() - i;

            renderer.render_circle_lines(
                ball.get_position(),
                ball.get_radius() + i as f64 - 1.0,
                2.0,
                lerp_color(
                    self.get_glow_color(),
                    Srgba::new(
                        self.get_glow_color().red,
                        self.get_glow_color().green,
                        self.get_glow_color().blue,
                        0.0,
                    ),
                    (1.0 + i as f32 / self.get_size() as f32) / 2.0,
                ),
            );
        }

        renderer.render_circle(
            ball.get_position(),
            ball.get_radius(),
            self.get_ball_color(),
        );

        renderer.render_text(
            ball.get_name(),
            ball.get_position() - DVec2::Y * 2.0 * ball.get_radius(),
            TextAnchor2D {
                horizontal: HorizontalTextAnchor::Center,
                vertical: VerticalTextAnchor::Bottom,
            },
            20.0,
            ball.get_name_color(),
        );
    }
}
