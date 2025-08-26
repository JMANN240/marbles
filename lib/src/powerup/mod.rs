use dyn_clone::DynClone;
use glam::DVec2;
use rand::{Rng, seq::IndexedRandom};

use crate::{
    ball::Ball,
    powerup::{
        change_density::ChangeDensity, change_elasticity::ChangeElasticity,
        change_position::ChangePosition,
    },
    rendering::Render,
};

pub mod change_density;
pub mod change_elasticity;
pub mod change_position;
pub trait Powerup: Render + Send + Sync + DynClone {
    fn is_colliding_with(&self, ball: &Ball) -> bool;
    fn on_collision(&mut self, ball: &mut Ball);
    fn update(&mut self, dt: f64);
}

dyn_clone::clone_trait_object!(Powerup);

pub fn random_powerup(
    rng: &mut impl Rng,
    position: DVec2,
    viewport_width: f64,
    viewport_height: f64,
) -> Box<dyn Powerup> {
    let powerups: Vec<Box<dyn Powerup>> = vec![
        Box::new(ChangeElasticity::new(position, 8.0, 0.9)),
        Box::new(ChangeDensity::new(position, 8.0, 4.0)),
        Box::new(ChangePosition::new(
            position,
            "Teleport",
            8.0,
            16.0..=(viewport_width - 16.0),
            16.0..=(viewport_height - 16.0),
        )),
    ];

    powerups.choose(rng).unwrap().clone()
}
