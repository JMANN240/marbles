use std::any::Any;

use dyn_clone::DynClone;
use glam::DVec2;
use rand::{Rng, seq::IndexedRandom};

use crate::{
    ball::Ball,
    powerup::{
        change_density::ChangeDensity, change_elasticity::ChangeElasticity,
        change_position::ChangePosition, special::Special,
    },
    rendering::Render,
};

pub mod change_density;
pub mod change_elasticity;
pub mod change_position;
pub mod special;

pub trait Powerup: Render + Send + Sync + DynClone + Any {
    fn is_colliding_with(&self, ball: &Ball) -> bool;
    fn apply(&self, ball: &mut Ball);
    fn consume(&mut self);
    fn is_active(&self) -> bool;
    fn update(&self, dt: f64) -> Box<dyn Powerup>;
}

dyn_clone::clone_trait_object!(Powerup);

pub fn random_powerup(
    rng: &mut impl Rng,
    position: DVec2,
    viewport_width: f64,
    viewport_height: f64,
) -> Box<dyn Powerup> {
    let powerups: Vec<Box<dyn Powerup>> = vec![
        Box::new(ChangeElasticity::new(position, 8.0, 0.95)),
        Box::new(ChangeDensity::new(position, 8.0, 2.0)),
        Box::new(ChangePosition::new(
            position,
            "Teleport",
            8.0,
            16.0..=(viewport_width - 16.0),
            16.0..=(viewport_height - 16.0),
        )),
        Box::new(Special::new(position, 8.0)),
    ];

    powerups.choose(rng).unwrap().clone()
}
