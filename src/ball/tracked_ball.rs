use macroquad::{audio::Sound, prelude::*};

use crate::drawer::Drawer;

use super::Ball;

#[derive(Clone)]
pub struct TrackedBall<D> {
    name: String,
    name_color: Color,
    positions: Vec<DVec2>,
    velocity: DVec2,
    radius: f64,
    elasticity: f64,
    drawer: D,
    sound: Sound,
}

impl<D: Drawer<BallType = Self>> TrackedBall<D> {
    pub fn new(
        name: String,
        name_color: Color,
        position: DVec2,
        velocity: DVec2,
        radius: f64,
        elasticity: f64,
        drawer: D,
        sound: Sound,
    ) -> Self {
        Self {
            name,
            name_color,
            positions: vec![position],
            velocity,
            radius,
            elasticity,
            drawer,
            sound,
        }
    }

    pub fn get_positions(&self) -> &Vec<DVec2> {
        &self.positions
    }
}

impl<D: Drawer<BallType = Self>> Ball for TrackedBall<D> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_color(&self) -> Color {
        self.name_color
    }

    fn get_position(&self) -> DVec2 {
        *self.positions.last().unwrap()
    }

    fn set_position(&mut self, position: DVec2) {
        self.positions.push(position);
        if self.positions.len() > 1000 {
            self.positions.remove(0);
        }
    }

    fn get_velocity(&self) -> DVec2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: DVec2) {
        self.velocity = velocity;
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }

    fn get_elasticity(&self) -> f64 {
        self.elasticity
    }

    fn get_sound(&self) -> &Sound {
        &self.sound
    }

    fn draw(&self) {
        self.drawer.draw(self);
    }
}