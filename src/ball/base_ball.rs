use macroquad::{audio::Sound, prelude::*};

use crate::drawer::Drawer;

use super::Ball;

#[derive(Clone)]
pub struct BaseBall<D> {
    name: String,
    name_color: Color,
    position: DVec2,
    velocity: DVec2,
    radius: f64,
    elasticity: f64,
    drawer: D,
    sound: Sound,
}

impl<D: Drawer<BallType = Self>> BaseBall<D> {
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
            position,
            velocity,
            radius,
            elasticity,
            drawer,
            sound,
        }
    }
}

impl<D: Drawer<BallType = Self>> Ball for BaseBall<D> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_color(&self) -> Color {
        self.name_color
    }

    fn get_position(&self) -> DVec2 {
        self.position
    }

    fn set_position(&mut self, position: DVec2) {
        self.position = position;
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