use dyn_clone::DynClone;
use glam::DVec2;
use keyframe::AnimationSequence;
use mint::Vector2;
use render_agnostic::Renderer;

use crate::rendering::Render;

pub mod countdown;
pub mod engagement;
pub mod special_message;

dyn_clone::clone_trait_object!(Graphic);

pub trait Graphic: Send + Sync + DynClone {
    fn draw(&self, renderer: &mut dyn Renderer);

    fn update(&mut self, dt: f64) {
        let time = self.time() + dt;
        self.set_time(time);
        self.origin_sequence_mut().advance_to(time);
    }

    fn time(&self) -> f64;

    fn set_time(&mut self, new_time: f64);

    fn visible(&self) -> bool;

    fn origin_sequence(&self) -> &AnimationSequence<Vector2<f64>>;

    fn origin_sequence_mut(&mut self) -> &mut AnimationSequence<Vector2<f64>>;

    fn origin(&self) -> DVec2 {
        self.origin_sequence()
            .now_strict()
            .map(DVec2::from)
            .unwrap_or_default()
    }
}

impl Render for Box<dyn Graphic> {
    fn render(&self, renderer: &mut dyn Renderer) {
        if self.visible() {
            self.draw(renderer)
        }
    }
}
