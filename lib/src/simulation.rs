use glam::{DVec2, dvec2};
use palette::Srgba;
use render_agnostic::Renderer;

use crate::{
    collision::Collision, graphic::Graphic, rendering::Render, scene::Scene, util::ValueOverTime,
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
    maybe_any_won_time: Option<f64>,
    countdown_seconds: f64,
    reset_seconds: f64,
    graphics: Vec<Box<dyn Graphic>>,
    zoom: ValueOverTime<f64>,
    focus: ValueOverTime<DVec2>,
}

impl Simulation {
    pub fn new(
        scene: Scene,
        viewport: (f64, f64),
        countdown_seconds: f64,
        reset_seconds: f64,
        graphics: Vec<Box<dyn Graphic>>,
    ) -> Self {
        let zoom = ValueOverTime::new(0.875);

        let focus = ValueOverTime::new(DVec2::splat(0.5));

        Self {
            time: 0.0,
            viewport_width: viewport.0,
            viewport_height: viewport.1,
            scene,
            maybe_all_won_time: None,
            maybe_any_won_time: None,
            countdown_seconds,
            reset_seconds,
            graphics,
            zoom,
            focus,
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

    pub fn get_maybe_any_won_time(&self) -> Option<f64> {
        self.maybe_any_won_time
    }

    pub fn get_countdown_seconds(&self) -> f64 {
        self.countdown_seconds
    }

    pub fn get_reset_seconds(&self) -> f64 {
        self.reset_seconds
    }

    pub fn get_phase(&self) -> SimulationPhase {
        if self.get_time() < self.countdown_seconds {
            SimulationPhase::Countdown
        } else {
            SimulationPhase::Running
        }
    }

    pub fn update(&self, dt: f64, timescale: f64, physics_steps: usize) -> (Self, Vec<Collision>) {
        let (new_scene, collisions) = match self.get_phase() {
            SimulationPhase::Countdown => (self.scene.clone(), vec![]),
            SimulationPhase::Running => self.scene.update(dt, timescale, physics_steps),
        };

        let mut new_simulation = self.clone();
        new_simulation.scene = new_scene;

        new_simulation
            .graphics
            .iter_mut()
            .for_each(|graphic| graphic.update(dt));

        if new_simulation.scene.any_won() && new_simulation.maybe_any_won_time.is_none() {
            new_simulation.maybe_any_won_time = Some(new_simulation.time);
        }

        if new_simulation.scene.all_won() && new_simulation.maybe_all_won_time.is_none() {
            new_simulation.maybe_all_won_time = Some(new_simulation.time);
        }

        new_simulation.time += dt;

        (new_simulation, collisions)
    }

    pub fn is_finished(&self) -> bool {
        self.get_scene().get_finished_condition()(self)
    }

    pub fn zoom(&self, time: f64) -> f64 {
        *self.zoom.get_value(time)
    }

    pub fn focus(&self, time: f64) -> DVec2 {
        *self.focus.get_value(time)
    }
}

impl Render for Simulation {
    fn render(&self, renderer: &mut dyn Renderer) {
        self.get_scene().render(renderer);

        for graphic in self.graphics.iter() {
            graphic.render(renderer);
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
            let text = format!("{}. {} ({})", index + 1, winner.get_id(), time_string);
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
