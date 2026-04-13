use std::sync::Arc;

use glam::DVec2;
use rand::{Rng, seq::IndexedRandom};

use crate::{
    api::Marble, powerup::Powerup, scene::Scene, simulation::Simulation, util::space_evenly, wall::Wall
};

#[derive(Clone)]
pub struct BallSpace {
    position: DVec2,
    velocity: DVec2,
}

impl BallSpace {
    pub fn new(position: DVec2, velocity: DVec2) -> Self {
        Self { position, velocity }
    }

    pub fn spaced_evenly(n: usize, start: DVec2, end: DVec2, velocity: DVec2) -> Vec<Self> {
        space_evenly(n, start, end)
            .into_iter()
            .map(|position| Self::new(position, velocity))
            .collect()
    }
}

pub struct PowerupSpace {
    position: DVec2,
}

impl PowerupSpace {
    pub fn new(position: DVec2) -> Self {
        Self { position }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position
    }
}

pub struct Level {
    ball_spaces: Vec<BallSpace>,
    powerup_spaces: Vec<PowerupSpace>,
    walls: Vec<Box<dyn Wall>>,
}

impl Level {
    pub fn new(
        ball_spaces: Vec<BallSpace>,
        powerup_spaces: Vec<PowerupSpace>,
        walls: Vec<Box<dyn Wall>>,
    ) -> Self {
        Self {
            ball_spaces,
            powerup_spaces,
            walls,
        }
    }

    pub fn build_scene(
        &self,
        rng: &mut impl Rng,
        marbles: &[Marble],
        powerup_function: impl Fn(&PowerupSpace) -> Box<dyn Powerup>,
        finished_condition: impl Fn(&Simulation) -> bool + Send + Sync + 'static,
    ) -> Scene {
        let balls = marbles
            .sample(rng, self.ball_spaces.len())
            .zip(self.ball_spaces.iter())
            .map(|(marble, ball_space)| {
                marble.build(ball_space.position, ball_space.velocity)
            })
            .collect();

        let powerups = self.powerup_spaces.iter().map(powerup_function).collect();

        Scene::new(
            balls,
            powerups,
            self.walls.clone(),
            Arc::new(finished_condition),
        )
    }
}
