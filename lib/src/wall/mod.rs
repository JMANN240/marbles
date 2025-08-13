use glam::DVec2;

use crate::{ball::PhysicsBall, rendering::Render};

pub mod circle_wall;
pub mod straight_wall;

pub trait Wall: Render + Send + Sync {
    fn update(&mut self, dt: f64);
    fn get_intersection_point(&self, ball: &PhysicsBall) -> Option<DVec2>;
    fn is_goal(&self) -> bool;

    fn clone_box(&self) -> Box<dyn Wall + Send>;
}

impl Clone for Box<dyn Wall> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
