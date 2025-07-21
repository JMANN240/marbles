use macroquad::prelude::*;

use crate::ball::Ball;

pub mod circle_wall;
pub mod straight_wall;

pub trait Wall {
    fn draw(&self);
    fn get_intersection_point(&self, ball: &Ball) -> Option<DVec2>;
    fn is_goal(&self) -> bool;
}