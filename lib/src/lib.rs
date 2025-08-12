use std::{cmp::Ordering, collections::HashMap, env, fs, path::Path};

use ::rand::{random_range, rng, seq::IndexedRandom};
use chrono::{Local, TimeZone};
use clap::Parser;
use collision::{Collision, render_collisions};
use dotenvy::dotenv;
use macroquad::prelude::*;
use scenes::{scene_1, scene_2, scene_3, scene_4, scene_5, scene_6, scene_7};
use serde::Deserialize;
use toml::from_str;
use tracing_subscriber::FmtSubscriber;
use util::draw_text_outline;

use crate::posting::{cloudinary::Cloudinary, instagram::InstagramPoster};

pub mod ball;
pub mod collision;
pub mod drawer;
pub mod particle;
pub mod posting;
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
