use std::collections::VecDeque;

use glam::DVec2;
use palette::Srgba;

use crate::{
    ball::{Ball, PhysicsBall},
    drawer::BallStyle,
    rendering::{HorizontalTextAnchor, Renderer, TextAnchor2D, VerticalTextAnchor},
    util::lerp_color,
};

#[derive(Clone)]
pub struct TailStyle {
    positions: VecDeque<DVec2>,
    max_positions: usize,
    start_color: Srgba,
    end_color: Srgba,
    every: usize,
    every_count: usize,
}

impl TailStyle {
    pub fn new(start_color: Srgba, end_color: Srgba, max_positions: usize, every: usize) -> Self {
        Self {
            positions: VecDeque::default(),
            max_positions,
            start_color,
            end_color,
            every,
            every_count: 0,
        }
    }

    pub fn get_positions(&self) -> &VecDeque<DVec2> {
        &self.positions
    }

    pub fn get_start_color(&self) -> Srgba {
        self.start_color
    }

    pub fn get_end_color(&self) -> Srgba {
        self.end_color
    }
}

impl BallStyle for TailStyle {
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

    fn render(&self, ball: &Ball, renderer: &mut dyn Renderer) {
        for (index, position) in self.get_positions().iter().copied().enumerate() {
            let percent = (index + 1) as f64 / self.get_positions().len() as f64;

            renderer.render_circle(
                position,
                ball.get_radius() * percent,
                lerp_color(
                    self.get_start_color(),
                    self.get_end_color(),
                    1.0 - percent as f32,
                ),
            );
        }

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
