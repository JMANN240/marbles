use even_odd_traits::IsEven;
use glam::dvec2;
use keyframe::AnimationSequence;
use mint::Vector2;
use palette::Srgba;
use render_agnostic::Renderer;

use crate::graphic::Graphic;

#[derive(Clone)]
pub struct Engagement {
    pub time: f64,
    pub start: f64,
    pub length: f64,
    pub message: String,
    pub origin: AnimationSequence<Vector2<f64>>,
    pub viewport: (f64, f64),
}

impl Engagement {
    pub fn new(
        origin: AnimationSequence<Vector2<f64>>,
        start: f64,
        length: f64,
        message: String,
        viewport: (f64, f64),
    ) -> Self {
        Self {
            time: 0.0,
            start,
            length,
            message: message,
            origin,
            viewport,
        }
    }
}

impl Graphic for Engagement {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.render_text_outline(
            &self.message,
            self.origin() + dvec2(self.viewport.0 / 2.0, self.viewport.1 / 2.0 + 100.0),
            anchor2d::CGB,
            48.0,
            1.0,
            Srgba::new(1.0, 1.0, 1.0, 1.0),
            Srgba::new(0.0, 0.0, 0.0, 1.0),
        );
    }

    fn origin_sequence(&self) -> &AnimationSequence<Vector2<f64>> {
        &self.origin
    }

    fn origin_sequence_mut(&mut self) -> &mut AnimationSequence<Vector2<f64>> {
        &mut self.origin
    }

    fn time(&self) -> f64 {
        self.time
    }

    fn set_time(&mut self, new_time: f64) {
        self.time = new_time;
    }

    fn visible(&self) -> bool {
        (self.time() < self.start + self.length) && (self.time() * 2.0 + 1.5).floor().is_even()
    }
}
