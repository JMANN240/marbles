use glam::{DVec2, dvec2};
use palette::Srgba;

use crate::{
    ball::{Ball, PhysicsBall},
    drawer::BallStyle,
    rendering::{Anchor2D, HorizontalAnchor, Renderer, VerticalAnchor},
    wall::straight_wall::Line,
};

#[derive(Clone)]
pub struct IkeaStyle;

impl BallStyle for IkeaStyle {
    fn init(&mut self, _ball: &PhysicsBall) {}
    fn update(&mut self, _ball: &PhysicsBall) {}

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
                    ball.get_position().x - ball.get_radius(),
                    ball.get_position().y,
                ),
                dvec2(
                    ball.get_position().x + ball.get_radius(),
                    ball.get_position().y,
                ),
            ),
            2.0,
            yellow.into(),
        );

        renderer.render_line(
            &Line::new(
                dvec2(
                    ball.get_position().x - ball.get_radius() / 3.0,
                    ball.get_position().y - ball.get_radius() / 1.2,
                ),
                dvec2(
                    ball.get_position().x - ball.get_radius() / 3.0,
                    ball.get_position().y + ball.get_radius() / 1.2,
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
