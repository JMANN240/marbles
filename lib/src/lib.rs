use serde::Deserialize;

pub mod ball;
pub mod collision;
pub mod drawer;
pub mod engagement;
pub mod graphic;
pub mod level;
pub mod levels;
pub mod particle;
pub mod posting;
pub mod powerup;
pub mod rendering;
pub mod scene;
pub mod scenes;
pub mod simulation;
pub mod util;
pub mod wall;

#[derive(Deserialize)]
pub struct Config {
    scene: usize,
}

impl Config {
    pub fn get_scene(&self) -> usize {
        self.scene
    }
}

pub const ENGAGEMENTS: [&str; 8] = [
    "Pick one!",
    "Choose a winner!",
    "Who will win?",
    "Choose one!",
    "Take a guess!",
    "Guess the winner!",
    "Ok, now THIS is epic!",
    "You'll never guess!",
];
