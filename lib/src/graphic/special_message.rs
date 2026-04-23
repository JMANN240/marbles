use glam::{DVec2, dvec2};
use keyframe::AnimationSequence;
use mint::Vector2;
use palette::Srgba;
use render_agnostic::Renderer;

use crate::graphic::Graphic;

#[derive(Clone)]
pub struct SpecialMessage {
    pub time: f64,
    pub origin: AnimationSequence<Vector2<f64>>,
    pub viewport: (f64, f64),
    pub message: String,
    pub user: String,
}

impl SpecialMessage {
    pub fn new(origin: AnimationSequence<Vector2<f64>>, viewport: (f64, f64), message: String, user: String) -> Self {
        Self { time: 0.0, origin, viewport, message, user }
    }
}

impl Graphic for SpecialMessage {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.render_rectangle(
            dvec2(self.origin().x - self.viewport.0, self.viewport.1 * 0.8),
            self.viewport.0 + self.viewport.0 * 0.9,
            self.viewport.1 * 0.1,
            DVec2::ZERO,
            0.0,
            Srgba::new(0.0, 0.0, 0.0, 1.0),
        );

        renderer.render_rectangle_lines(
            dvec2(self.origin().x - self.viewport.0, self.viewport.1 * 0.8),
            self.viewport.0 + self.viewport.0 * 0.9,
            self.viewport.1 * 0.1,
            DVec2::ZERO,
            0.0,
            2.0,
            Srgba::new(1.0, 1.0, 1.0, 1.0),
        );

        let chars = self.message.chars().collect::<Vec<char>>();

        let font_size = 24.0;

        for (index, chunk) in chars
            .chunks(35)
            .map(|chunk| chunk.iter().collect::<String>())
            .enumerate()
        {
            renderer.render_text(
                &chunk,
                dvec2(
                    self.origin().x + self.viewport.0 * 0.4,
                    self.viewport.1 * 0.825 + font_size * index as f64,
                ),
                anchor2d::CGC,
                font_size,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
            );
        }

        renderer.render_text(
            "Submit your own message at https://quantummarbleracing.com",
            dvec2(self.origin().x, self.viewport.1 * 0.9 - 8.0),
            anchor2d::LGB,
            16.0,
            Srgba::new(0.5, 0.5, 0.5, 1.0),
        );

        renderer.render_text(
            &format!("-{}", self.user),
            dvec2(
                self.origin().x + self.viewport.0 * 0.9 - 8.0,
                self.viewport.1 * 0.85,
            ),
            anchor2d::RGB,
            16.0,
            Srgba::new(0.5, 0.5, 0.5, 1.0),
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
        matches!(self.origin_sequence().pair(), (Some(_), Some(_)))
    }
}
