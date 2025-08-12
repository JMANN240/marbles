use macroquad::prelude::*;

use crate::{
    ball::{Ball, PhysicsBall},
    draw_text_outline,
};

use super::Drawer;

pub struct OutlineDrawer {
    color: Color,
}

impl OutlineDrawer {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Drawer for OutlineDrawer {
    fn init(&mut self, _ball: &PhysicsBall) {}

    fn update(&mut self, _ball: &PhysicsBall) {}

    fn draw(&self, ball: &Ball) {
        draw_circle_lines(
            ball.get_position().x as f32,
            ball.get_position().y as f32,
            ball.get_radius() as f32 - 1.0,
            2.0,
            self.color,
        );

        let text = ball.get_name();
        let font_size = 24.0;

        draw_text_outline(
            text,
            ball.get_position().x as f32
                - measure_text(text, None, font_size as u16, 1.0).width / 2.0,
            ball.get_position().y as f32 - ball.get_radius() as f32 - 8.0,
            font_size,
            ball.get_name_color(),
            if ball.get_name_color() != BLACK {
                BLACK
            } else {
                WHITE
            },
        );
    }
}
