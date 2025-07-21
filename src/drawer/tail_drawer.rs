use macroquad::prelude::*;

use crate::{
    ball::{Ball, PhysicsBall},
    draw_text_outline,
};

use super::Drawer;

pub struct TailDrawer {
    positions: Vec<DVec2>,
    max_positions: usize,
    start_color: Color,
    end_color: Color,
}

impl TailDrawer {
    pub fn new(start_color: Color, end_color: Color, max_positions: usize) -> Self {
        Self {
            positions: Vec::default(),
            max_positions,
            start_color,
            end_color,
        }
    }
}

impl Drawer for TailDrawer {
    fn init(&mut self, ball: &PhysicsBall) {
        self.positions.clear();
        self.update(ball);
    }

    fn update(&mut self, ball: &PhysicsBall) {
        self.positions.push(ball.get_position());
        if self.positions.len() > self.max_positions {
            self.positions.remove(0);
        }
    }

    fn draw(&self, ball: &Ball) {
        let text = ball.get_name();
        let font_size = 24.0;

        draw_text_outline(
            text,
            ball.get_position().x as f32
                - measure_text(text, None, font_size as u16, 1.0).width / 2.0,
            ball.get_position().y as f32 - 2.0 * ball.get_radius() as f32,
            font_size,
            ball.get_name_color(),
            1,
        );

        for (index, position) in self.positions.iter().enumerate() {
            let percent = (index + 1) as f32 / self.positions.len() as f32;

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
