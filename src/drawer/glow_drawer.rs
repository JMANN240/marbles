use macroquad::prelude::*;

use crate::{
    ball::{Ball, PhysicsBall},
    draw_text_outline,
    util::lerp_color,
};

use super::Drawer;

pub struct GlowDrawer {
    ball_color: Color,
    glow_color: Color,
    size: usize,
}

impl GlowDrawer {
    pub fn new(ball_color: Color, glow_color: Color, size: usize) -> Self {
        Self {
            ball_color,
            glow_color,
            size,
        }
    }
}

impl Drawer for GlowDrawer {
    fn init(&mut self, _ball: &PhysicsBall) {}

    fn update(&mut self, _ball: &PhysicsBall) {}

    fn draw(&self, ball: &Ball) {
        for i in 0..=self.size {
            let i = self.size - i;

            draw_circle_lines(
                ball.get_position().x as f32,
                ball.get_position().y as f32,
                ball.get_radius() as f32 + i as f32,
                1.0,
                lerp_color(
                    self.glow_color,
                    Color {
                        a: 0.0,
                        ..self.glow_color
                    },
                    (1.0 + i as f32 / self.size as f32) / 2.0,
                ),
            );
        }

        draw_circle(
            ball.get_position().x as f32,
            ball.get_position().y as f32,
            ball.get_radius() as f32,
            self.ball_color,
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
