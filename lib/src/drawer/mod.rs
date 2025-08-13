use crate::{
    ball::{Ball, PhysicsBall},
    rendering::Renderer,
};

pub mod base_style;
pub mod glow_style;
pub mod outline_style;
pub mod tail_style;

pub trait BallStyle: Send + Sync {
    fn init(&mut self, ball: &PhysicsBall);
    fn update(&mut self, ball: &PhysicsBall);
    fn render(&self, ball: &Ball, renderer: &mut dyn Renderer);
    fn clone_box(&self) -> Box<dyn BallStyle + Send>;
}

impl Clone for Box<dyn BallStyle> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
