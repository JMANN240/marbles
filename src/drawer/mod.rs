use crate::ball::Ball;

pub mod base_drawer;
pub mod tail_drawer;

pub trait Drawer {
    type BallType: Ball;

    fn draw(&self, ball: &Self::BallType);
}
