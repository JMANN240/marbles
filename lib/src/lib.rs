use serde::Deserialize;

pub mod ball;
pub mod collision;
pub mod drawer;
pub mod particle;
pub mod posting;
pub mod rendering;
pub mod scene;
pub mod scenes;
pub mod simulation;
pub mod util;
pub mod wall;

#[derive(Deserialize, Clone)]
pub struct BallConfig {
    name: String,
    r: f32,
    g: f32,
    b: f32,
    radius: f64,
    elasticity: f64,
    sound: String,
}

#[derive(Deserialize)]
pub struct Config {
    balls: Vec<BallConfig>,
    scene: usize,
}

impl Config {
    pub fn get_balls(&self) -> &Vec<BallConfig> {
        &self.balls
    }

    pub fn get_scene(&self) -> usize {
        self.scene
    }
}

pub const ENGAGEMENTS: [&str; 4] = [
    "Pick one!",
    "Choose a winner!",
    "Who will win?",
    "Choose one!",
];
