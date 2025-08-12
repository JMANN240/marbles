use macroquad::prelude::*;

use crate::{collision::Collision, scene::Scene, util::draw_text_outline};

pub enum SimulationPhase {
    Countdown,
    Running,
}

pub struct Simulation {
    time: f64,
    scene: Scene,
    countdown_seconds: f64,
    reset_seconds: f64,
    engagement: String,
}

impl Simulation {
    pub fn new(scene: Scene, countdown_seconds: f64, reset_seconds: f64, engagement: String) -> Self {
        Self {
            time: 0.0,
            scene,
            countdown_seconds,
            reset_seconds,
            engagement,
        }
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_scene(&self) -> &Scene {
        &self.scene
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

        self.time += dt;

        collisions
    }

    pub fn draw(&self) {
        self.scene.draw();

        if self.get_time().floor() < self.countdown_seconds {
            let text = format!(
                "{}",
                self.countdown_seconds - self.get_time().floor()
            );
            draw_text_outline(
                &text,
                screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                screen_height() / 2.0,
                256.0,
                WHITE,
                BLACK,
            );

            if (self.get_time() * 2.0 + 1.5).floor() % 2.0 == 0.0 {
                draw_text_outline(
                    &self.engagement,
                    screen_width() / 2.0 - measure_text(&self.engagement, None, 64, 1.0).width / 2.0,
                    screen_height() / 2.0 + 100.0,
                    64.0,
                    WHITE,
                    BLACK,
                );
            }
        } else if self.get_time().floor() < self.countdown_seconds + 1.0 {
            let text = "Go!";
            draw_text_outline(
                text,
                screen_width() / 2.0 - measure_text(text, None, 256, 1.0).width / 2.0,
                screen_height() / 2.0,
                256.0,
                WHITE,
                BLACK,
            );
        }
    }
}
