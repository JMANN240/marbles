use std::sync::Arc;

use glam::{DVec2, dvec2};
use keyframe::AnimationSequence;
use mint::Vector2;
use palette::Srgba;
use render_agnostic::Renderer;

use crate::{rendering::Render, util::ValueOverTime};

#[derive(Clone)]
pub struct Graphic<S> {
    time: f64,
    visible: ValueOverTime<bool>,
    render_function: Arc<dyn Fn(&mut dyn Renderer, DVec2, &S) + Send + Sync + 'static>,
    state: S,
    origin: AnimationSequence<Vector2<f64>>,
}

impl<S> Graphic<S> {
    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        self.origin.advance_to(self.time());
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn visible(&self) -> bool {
        *self.visible.get_value(self.time())
    }

    pub fn origin(&self) -> DVec2 {
        self.origin
            .now_strict()
            .map(DVec2::from)
            .unwrap_or_default()
    }
}

impl Graphic<SpecialMessageState> {
    pub fn special_message(
        origin: AnimationSequence<Vector2<f64>>,
        countdown_seconds: f64,
        state: SpecialMessageState,
    ) -> Self {
        let mut visible = ValueOverTime::new(false);

        visible.add_modifier((countdown_seconds + 0.0)..=(countdown_seconds + 12.0), true);

        Self {
            time: 0.0,
            visible,
            render_function: Arc::new(|renderer, origin, state: &SpecialMessageState| {
                renderer.render_rectangle(
                    dvec2(origin.x - state.viewport_width, state.viewport_height * 0.8),
                    state.viewport_width + state.viewport_width * 0.9,
                    state.viewport_height * 0.1,
                    DVec2::ZERO,
                    0.0,
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                );

                renderer.render_rectangle_lines(
                    dvec2(origin.x - state.viewport_width, state.viewport_height * 0.8),
                    state.viewport_width + state.viewport_width * 0.9,
                    state.viewport_height * 0.1,
                    DVec2::ZERO,
                    0.0,
                    2.0,
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                );

                let chars = state.message.chars().collect::<Vec<char>>();

                let font_size = 24.0;

                for (index, chunk) in chars
                    .chunks(35)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .enumerate()
                {
                    renderer.render_text(
                        &chunk,
                        dvec2(
                            origin.x + state.viewport_width * 0.4,
                            state.viewport_height * 0.825 + font_size * index as f64,
                        ),
                        anchor2d::CGC,
                        font_size,
                        Srgba::new(1.0, 1.0, 1.0, 1.0),
                    );
                }

                renderer.render_text(
                    "Submit your own message at https://quantummarbleracing.com",
                    dvec2(origin.x, state.viewport_height * 0.9 - 8.0),
                    anchor2d::LGB,
                    16.0,
                    Srgba::new(0.5, 0.5, 0.5, 1.0),
                );

                renderer.render_text(
                    &format!("-{}", state.user),
                    dvec2(
                        origin.x + state.viewport_width * 0.9 - 8.0,
                        state.viewport_height * 0.85,
                    ),
                    anchor2d::RGB,
                    16.0,
                    Srgba::new(0.5, 0.5, 0.5, 1.0),
                );
            }),
            state,
            origin,
        }
    }
}

impl<S> Render for Graphic<S> {
    fn render(&self, renderer: &mut dyn Renderer) {
        if self.visible() {
            (self.render_function)(
                renderer,
                self.origin
                    .now_strict()
                    .map(DVec2::from)
                    .unwrap_or_default(),
                &self.state,
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpecialMessageState {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub message: String,
    pub user: String,
}
