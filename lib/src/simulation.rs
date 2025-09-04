use glam::{DVec2, dvec2};
use palette::Srgba;
use render_agnostic::Renderer;

use crate::{collision::Collision, rendering::Render, scene::Scene};

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
    special_message: String,
    special_message_user: String,
    special_message_x: f64,
    special_message_target_x: f64,
}

impl Simulation {
    pub fn new(
        scene: Scene,
        viewport_width: f64,
        viewport_height: f64,
        countdown_seconds: f64,
        reset_seconds: f64,
        engagement: String,
        special_message: String,
        special_message_user: String,
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
            special_message,
            special_message_user,
            special_message_x: -viewport_width,
            special_message_target_x: -viewport_width,
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

        if self.get_time() > self.get_countdown_seconds() + 12.0 {
            self.special_message_target_x = -self.viewport_width;
        } else if self.get_time() > self.get_countdown_seconds() + 2.0 {
            self.special_message_target_x = 0.0;
        }

        self.special_message_x +=
            (self.special_message_target_x - self.special_message_x) * 8.0 * dt;

        if self.scene.all_won() && self.maybe_all_won_time.is_none() {
            self.maybe_all_won_time = Some(self.time);
        }

        self.time += dt;

        collisions
    }
}

impl Render for Simulation {
    fn render(&self, renderer: &mut dyn Renderer) {
        self.get_scene().render(renderer);

        if self.get_time().floor() < self.get_countdown_seconds() {
            let text = format!("{}", self.get_countdown_seconds() - self.get_time().floor());
            renderer.render_text_outline(
                &text,
                dvec2(self.viewport_width / 2.0, self.viewport_height / 2.0),
                anchor2d::CGB,
                196.0,
                1.0,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
                Srgba::new(0.0, 0.0, 0.0, 1.0),
            );

            if (self.get_time() * 2.0 + 1.5).floor() % 2.0 == 0.0 {
                renderer.render_text_outline(
                    self.get_engagement(),
                    dvec2(
                        self.viewport_width / 2.0,
                        self.viewport_height / 2.0 + 100.0,
                    ),
                    anchor2d::CGB,
                    48.0,
                    1.0,
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                );
            }
        } else if self.get_time().floor() < self.get_countdown_seconds() + 1.0 {
            renderer.render_text_outline(
                "Go!",
                dvec2(self.viewport_width / 2.0, self.viewport_height / 2.0),
                anchor2d::CGB,
                196.0,
                1.0,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
                Srgba::new(0.0, 0.0, 0.0, 1.0),
            );
        }

        if self.get_time() > self.get_countdown_seconds() + 2.0
            && self.get_time() < self.get_countdown_seconds() + 12.0
        {
            renderer.render_rectangle(
                dvec2(
                    self.special_message_x - self.viewport_width,
                    self.viewport_height * 0.8,
                ),
                self.viewport_width + self.viewport_width * 0.9,
                self.viewport_height * 0.1,
                DVec2::ZERO,
                0.0,
                Srgba::new(0.0, 0.0, 0.0, 1.0),
            );

            renderer.render_rectangle_lines(
                dvec2(
                    self.special_message_x - self.viewport_width,
                    self.viewport_height * 0.8,
                ),
                self.viewport_width + self.viewport_width * 0.9,
                self.viewport_height * 0.1,
                DVec2::ZERO,
                0.0,
                2.0,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
            );

            renderer.render_text(
                &self.special_message,
                dvec2(
                    self.special_message_x + self.viewport_width / 2.0,
                    self.viewport_height * 0.825,
                ),
                anchor2d::CGC,
                24.0,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
            );

            renderer.render_text(
                "Submit your own message at https://quantummarbleracing.com",
                dvec2(self.special_message_x, self.viewport_height * 0.9 - 8.0),
                anchor2d::LGB,
                16.0,
                Srgba::new(0.5, 0.5, 0.5, 1.0),
            );

            renderer.render_text(
                &format!("-{}", self.special_message_user),
                dvec2(
                    self.special_message_x + self.viewport_width * 0.9 - 8.0,
                    self.viewport_height * 0.85,
                ),
                anchor2d::RGB,
                16.0,
                Srgba::new(0.5, 0.5, 0.5, 1.0),
            );
        }

        for (index, (winner_index, win_time)) in self
            .get_scene()
            .get_winners()
            .iter()
            .zip(self.get_scene().get_win_times())
            .enumerate()
        {
            let winner = self.get_scene().get_balls().get(*winner_index).unwrap();
            let time_string = format!(
                "{:02}:{:02}.{:03}",
                (win_time.as_secs_f64() / 60.0).floor(),
                (win_time.as_secs_f64() % 60.0).floor(),
                win_time.subsec_millis()
            );
            let text = format!("{}. {} ({})", index + 1, winner.get_name(), time_string);
            let font_size = 40.0;

            renderer.render_text_outline(
                &text,
                dvec2(
                    self.viewport_width / 2.0,
                    font_size + font_size * index as f64,
                ),
                anchor2d::CGB,
                font_size,
                1.0,
                winner.get_name_color(),
                Srgba::new(0.0, 0.0, 0.0, 1.0),
            );
        }
    }
}
