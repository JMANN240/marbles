use dyn_clone::DynClone;
use glam::DVec2;

use crate::{ball::PhysicsBall, rendering::Render};

pub mod circle_wall;
pub mod straight_wall;

pub trait Wall: Render + Send + Sync + DynClone {
    fn update(&self, dt: f64) -> Box<dyn Wall>;
    fn get_intersection_point(&self, ball: &PhysicsBall) -> Option<DVec2>;
    fn is_goal(&self) -> bool;
}

dyn_clone::clone_trait_object!(Wall);
