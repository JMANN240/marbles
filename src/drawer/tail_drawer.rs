use macroquad::prelude::*;

use crate::{ball::{tracked_ball::TrackedBall, Ball}, draw_text_outline};

use super::Drawer;

pub struct TailDrawer {
    start_color: Color,
    end_color: Color,
}

impl TailDrawer {
    pub fn new(start_color: Color, end_color: Color) -> Self {
        Self {
            start_color,
            end_color,
        }
    }
}

impl Drawer for TailDrawer {
    type BallType = TrackedBall<Self>;

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

        for (index, position) in ball.get_positions().iter().enumerate() {
            let percent = (index + 1) as f32 / ball.get_positions().len() as f32;

            draw_circle(
                position.x as f32,
                position.y as f32,
                ball.get_radius() as f32 * percent,
                Color {
                    a: 1.0,
                    r: self.end_color.r * (1.0 - percent) + self.start_color.r * percent,
                    g: self.end_color.g * (1.0 - percent) + self.start_color.g * percent,
                    b: self.end_color.b * (1.0 - percent) + self.start_color.b * percent,
                },
            );
        }
    }
}
