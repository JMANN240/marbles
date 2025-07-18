use macroquad::prelude::*;

use crate::ball::{base_ball::BaseBall, Ball};

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
        draw_circle(
            ball.get_position().x as f32,
            ball.get_position().y as f32,
            ball.get_radius() as f32,
            self.color,
        );
    }
}