use dyn_clone::DynClone;
use render_agnostic::Renderer;

use crate::ball::{Ball, PhysicsBall};

pub mod base_style;
pub mod glow_style;
pub mod ikea_style;
pub mod outline_style;
pub mod tail_style;

pub trait BallStyle: Send + Sync + DynClone {
    fn init(&mut self, ball: &PhysicsBall);
    fn update(&mut self, ball: &PhysicsBall, dt: f64);
    fn render(&self, ball: &Ball, renderer: &mut dyn Renderer);
}

dyn_clone::clone_trait_object!(BallStyle);
