use std::collections::VecDeque;

use glam::DVec2;
use palette::Srgba;
use render_agnostic::Renderer;

use crate::{
    ball::{Ball, PhysicsBall},
    drawer::BallStyle,
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

    fn update_internal(&self, ball: &PhysicsBall, _dt: f64) -> Self {
        let mut new_style = self.clone();

        if new_style.every_count % new_style.every == 0 {
            new_style.positions.push_back(ball.get_position());
            if new_style.positions.len() > new_style.max_positions {
                new_style.positions.pop_front();
            }
        }

        new_style.every_count += 1;
        new_style.every_count %= new_style.every;

        new_style
    }
}

impl BallStyle for TailStyle {
    fn init(&mut self, ball: &PhysicsBall) {
        self.positions.clear();
        *self = self.update_internal(ball, 0.0);
    }

    fn update(&self, ball: &PhysicsBall, dt: f64) -> Box<dyn BallStyle> {
        Box::new(self.update_internal(ball, dt))
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
