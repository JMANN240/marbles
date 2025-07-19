use macroquad::prelude::*;

use crate::{ball::{base_ball::BaseBall, Ball}, draw_text_outline};

use super::Drawer;

pub struct BaseDrawer {
    color: Color,
}

impl BaseDrawer {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Drawer for BaseDrawer {
    type BallType = BaseBall<Self>;

    fn draw(&self, ball: &Self::BallType) {
        let text = ball.get_name();
        let font_size = 24.0;

        draw_text_outline(
            &text,
            ball.get_position().x as f32 - measure_text(&text, None, font_size as u16, 1.0).width / 2.0,
            ball.get_position().y as f32 - 2.0 * ball.get_radius() as f32,
            font_size,
            ball.get_name_color(),
            1,
        );

        draw_circle(
            ball.get_position().x as f32,
            ball.get_position().y as f32,
            ball.get_radius() as f32,
            self.color,
        );
    }
}