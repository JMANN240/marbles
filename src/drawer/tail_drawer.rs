use std::collections::VecDeque;

use macroquad::prelude::*;

use crate::{
    ball::{Ball, PhysicsBall},
    draw_text_outline,
    util::lerp_color,
};

use super::Drawer;

pub struct TailDrawer {
    positions: VecDeque<DVec2>,
    max_positions: usize,
    start_color: Color,
    end_color: Color,
    every: usize,
    every_count: usize,
}

impl TailDrawer {
    pub fn new(start_color: Color, end_color: Color, max_positions: usize, every: usize) -> Self {
        Self {
            positions: VecDeque::default(),
            max_positions,
            start_color,
            end_color,
            every,
            every_count: 0,
        }
    }
}

impl Drawer for TailDrawer {
    fn init(&mut self, ball: &PhysicsBall) {
        self.positions.clear();
        self.update(ball);
    }

    fn update(&mut self, ball: &PhysicsBall) {
        if self.every_count % self.every == 0 {
            self.positions.push_back(ball.get_position());
            if self.positions.len() > self.max_positions {
                self.positions.pop_front();
            }
        }

        self.every_count += 1;
        self.every_count %= self.every;
    }

    fn draw(&self, ball: &Ball) {
        for (index, position) in self.positions.iter().enumerate() {
            let percent = (index + 1) as f32 / self.positions.len() as f32;

            draw_circle(
                position.x as f32,
                position.y as f32,
                ball.get_radius() as f32 * percent,
                lerp_color(self.start_color, self.end_color, 1.0 - percent),
            );
        }

        let text = ball.get_name();
        let font_size = 24.0;

        draw_text_outline(
            text,
            ball.get_position().x as f32
                - measure_text(text, None, font_size as u16, 1.0).width / 2.0,
            ball.get_position().y as f32 - 2.0 * ball.get_radius() as f32,
            font_size,
            ball.get_name_color(),
        );
    }
}
