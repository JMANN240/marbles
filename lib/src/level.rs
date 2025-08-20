use glam::DVec2;
use rand::{Rng, seq::SliceRandom};

use crate::{
    powerup::{change_density::ChangeDensity, change_elasticity::ChangeElasticity, random_powerup, Powerup}, scene::Scene, util::space_evenly, wall::Wall, BallConfig
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

    pub fn build_scene(&self, rng: &mut impl Rng, ball_configs: Vec<BallConfig>) -> Scene {
        let mut ball_spaces = self.ball_spaces.clone();
        ball_spaces.shuffle(rng);

        let balls = ball_configs
            .iter()
            .zip(ball_spaces)
            .map(|(ball_config, ball_space)| {
                ball_config.build(ball_space.position, ball_space.velocity)
            })
            .collect();

        let powerups = self
            .powerup_spaces
            .iter()
            .map(|powerup_space| {
                random_powerup(rng, powerup_space.position)
            })
            .collect();

        Scene::new(balls, powerups, self.walls.clone())
    }
}
