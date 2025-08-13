use glam::dvec2;
use palette::Srgba;

use crate::{
    collision::Collision,
    rendering::{HorizontalTextAnchor, Render, TextAnchor2D, VerticalTextAnchor},
    scene::Scene,
};

pub enum SimulationPhase {
    Countdown,
    Running,
}

#[derive(Clone)]
pub struct Simulation {
    time: f64,
    viewport_width: f64,
    viewport_height: f64,
    scene: Scene,
    maybe_all_won_time: Option<f64>,
    countdown_seconds: f64,
    reset_seconds: f64,
    engagement: String,
}

impl Simulation {
    pub fn new(
        scene: Scene,
        viewport_width: f64,
        viewport_height: f64,
        countdown_seconds: f64,
        reset_seconds: f64,
        engagement: String,
    ) -> Self {
        Self {
            time: 0.0,
            viewport_width,
            viewport_height,
            scene,
            maybe_all_won_time: None,
            countdown_seconds,
            reset_seconds,
            engagement,
        }
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_viewport_width(&self) -> f64 {
        self.viewport_width
    }

    pub fn get_viewport_height(&self) -> f64 {
        self.viewport_height
    }

    pub fn get_scene(&self) -> &Scene {
        &self.scene
    }

    pub fn get_maybe_all_won_time(&self) -> Option<f64> {
        self.maybe_all_won_time
    }

    pub fn get_countdown_seconds(&self) -> f64 {
        self.countdown_seconds
    }

    pub fn get_reset_seconds(&self) -> f64 {
        self.reset_seconds
    }

    pub fn get_engagement(&self) -> &str {
        &self.engagement
    }

    pub fn get_phase(&self) -> SimulationPhase {
        if self.get_time() < self.countdown_seconds {
            SimulationPhase::Countdown
        } else {
            SimulationPhase::Running
        }
    }

    pub fn update(&mut self, dt: f64, timescale: f64, physics_steps: usize) -> Vec<Collision> {
        let collisions = match self.get_phase() {
            SimulationPhase::Countdown => vec![],
            SimulationPhase::Running => self.scene.update(dt, timescale, physics_steps),
        };

        if self.scene.all_won() && self.maybe_all_won_time.is_none() {
            self.maybe_all_won_time = Some(self.time);
        }

        self.time += dt;

        collisions
    }
}

impl Render for Simulation {
    fn render(&self, renderer: &mut dyn crate::rendering::Renderer) {
        self.get_scene().render(renderer);

        if self.get_time().floor() < self.get_countdown_seconds() {
            let text = format!("{}", self.get_countdown_seconds() - self.get_time().floor());
            renderer.render_text(
                &text,
                dvec2(self.viewport_width / 2.0, self.viewport_height / 2.0),
                TextAnchor2D {
                    horizontal: HorizontalTextAnchor::Center,
                    vertical: VerticalTextAnchor::Bottom,
                },
                196.0,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
            );

            if (self.get_time() * 2.0 + 1.5).floor() % 2.0 == 0.0 {
                renderer.render_text(
                    self.get_engagement(),
                    dvec2(
                        self.viewport_width / 2.0,
                        self.viewport_height / 2.0 + 100.0,
                    ),
                    TextAnchor2D {
                        horizontal: HorizontalTextAnchor::Center,
                        vertical: VerticalTextAnchor::Bottom,
                    },
                    48.0,
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                );
            }
        } else if self.get_time().floor() < self.get_countdown_seconds() + 1.0 {
            renderer.render_text(
                "Go!",
                dvec2(self.viewport_width / 2.0, self.viewport_height / 2.0),
                TextAnchor2D {
                    horizontal: HorizontalTextAnchor::Center,
                    vertical: VerticalTextAnchor::Bottom,
                },
                196.0,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
            );
        }

        for (index, winner_index) in self.get_scene().get_winners().iter().enumerate() {
            let winner = self.get_scene().get_balls().get(*winner_index).unwrap();
            let text = format!("{}. {}", index + 1, winner.get_name());
            let font_size = 48.0;

            renderer.render_text(
                &text,
                dvec2(
                    self.viewport_width / 2.0,
                    font_size + font_size * index as f64,
                ),
                TextAnchor2D {
                    horizontal: HorizontalTextAnchor::Center,
                    vertical: VerticalTextAnchor::Bottom,
                },
                font_size,
                winner.get_name_color(),
            );
        }
    }
}
