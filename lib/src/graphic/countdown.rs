use std::borrow::Cow;

use keyframe::AnimationSequence;
use mint::Vector2;
use palette::Srgba;
use render_agnostic::Renderer;

use crate::graphic::Graphic;

#[derive(Clone)]
pub struct Countdown {
    pub time: f64,
    pub start: f64,
    pub length: f64,
    pub message_length: f64,
    pub message: String,
    pub origin: AnimationSequence<Vector2<f64>>,
    pub viewport: (f64, f64),
}

impl Countdown {
    pub fn new(
        origin: AnimationSequence<Vector2<f64>>,
        start: f64,
        length: f64,
        message_length: f64,
        message: String,
        viewport: (f64, f64),
    ) -> Self {
        Self {
            time: 0.0,
            start,
            length,
            message_length,
            message,
            origin,
            viewport,
        }
    }

    pub fn text(&self) -> Option<Cow<'_, str>> {
        if self.time() < self.start + self.length {
            Some(Cow::Owned(format!("{}", self.length - self.time().floor())))
        } else if self.time() < self.start + self.length + self.message_length {
            Some(Cow::Borrowed(&self.message))
        } else {
            None
        }
    }
}

impl Graphic for Countdown {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let position = self.origin();

        let maybe_text = self.text();

        if let Some(text) = maybe_text {
            renderer.render_text_outline(
                &text,
                position,
                anchor2d::CGB,
                196.0,
                1.0,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
                Srgba::new(0.0, 0.0, 0.0, 1.0),
            );
        }
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
        (self.start..(self.start + self.length + self.message_length)).contains(&self.time())
    }
}
