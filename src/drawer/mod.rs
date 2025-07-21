use crate::ball::{Ball, PhysicsBall};

pub mod base_drawer;
pub mod tail_drawer;

pub trait Drawer {
    fn init(&mut self, ball: &PhysicsBall);
    fn update(&mut self, ball: &PhysicsBall);
    fn draw(&self, ball: &Ball);
}
