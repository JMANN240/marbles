use std::{
    f64::consts::PI,
    path::{Path, PathBuf},
};

use macroquad::{
    audio::Sound,
    prelude::*,
};

use crate::{drawer::Drawer, particle::system::ParticleSystem, wall::Wall};

pub struct Ball {
    name: String,
    name_color: Color,
    physics_ball: PhysicsBall,
    drawer: Box<dyn Drawer>,
    sound_path: PathBuf,
    sound: Sound,
    particles: ParticleSystem,
}

impl Ball {
    pub async fn new(
        name: String,
        name_color: Color,
        physics_ball: PhysicsBall,
        mut drawer: Box<dyn Drawer>,
        sound_path: PathBuf,
        sound: Sound,
    ) -> Self {
        drawer.init(&physics_ball);

        Self {
            name,
            name_color,
            physics_ball,
            drawer,
            sound_path,
            sound,
            particles: ParticleSystem::default(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_name_color(&self) -> Color {
        self.name_color
    }

    pub fn get_position(&self) -> DVec2 {
        self.physics_ball.get_position()
    }

    pub fn set_position(&mut self, position: DVec2) {
        self.physics_ball.set_position(position);
    }

    pub fn get_velocity(&self) -> DVec2 {
        self.physics_ball.get_velocity()
    }

    pub fn set_velocity(&mut self, velocity: DVec2) {
        self.physics_ball.set_velocity(velocity);
    }

    pub fn get_radius(&self) -> f64 {
        self.physics_ball.get_radius()
    }

    pub fn get_elasticity(&self) -> f64 {
        self.physics_ball.get_elasticity()
    }

    pub fn get_sound_path(&self) -> &Path {
        &self.sound_path
    }

    pub fn get_sound(&self) -> &Sound {
        &self.sound
    }

    pub fn get_particles(&self) -> &ParticleSystem {
        &self.particles
    }

    pub fn get_particles_mut(&mut self) -> &mut ParticleSystem {
        &mut self.particles
    }

    pub fn update(&mut self, dt: f64) {
        self.set_position(self.get_position() + self.get_velocity() * dt);

        let position = self.get_position();

        for emitter in self.get_particles_mut().get_emitters_mut() {
            emitter.set_position(position);
        }

        self.get_particles_mut().update(dt);
        self.drawer.update(&self.physics_ball);
    }

    pub fn draw(&self) {
        self.drawer.draw(self);

        self.get_particles().draw();
    }

    pub fn get_mass(&self) -> f64 {
        PI * self.get_radius() * self.get_radius()
    }

    pub fn get_intersection_point(&self, wall: &dyn Wall) -> Option<DVec2> {
        wall.get_intersection_point(self)
    }
}

pub struct PhysicsBall {
    position: DVec2,
    velocity: DVec2,
    radius: f64,
    elasticity: f64,
}

impl PhysicsBall {
    pub fn new(position: DVec2, velocity: DVec2, radius: f64, elasticity: f64) -> Self {
        Self {
            position,
            velocity,
            radius,
            elasticity,
        }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position
    }

    pub fn set_position(&mut self, position: DVec2) {
        self.position = position;
    }

    pub fn get_velocity(&self) -> DVec2 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: DVec2) {
        self.velocity = velocity;
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_elasticity(&self) -> f64 {
        self.elasticity
    }
}
