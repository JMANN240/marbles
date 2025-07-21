use macroquad::prelude::*;

use super::{Particle, emitter::ParticleEmitter};

#[derive(Default)]
pub struct ParticleSystem {
    particles: Vec<Box<dyn Particle>>,
    emitters: Vec<Box<dyn ParticleEmitter>>,
}

impl ParticleSystem {
    pub fn update(&mut self, dt: f64) {
        for emitter in self.get_emitters_mut() {
            emitter.update();
        }

        let new_particles = self
            .get_emitters()
            .iter()
            .filter(|emitter| emitter.should_generate_particle())
            .map(|emitter| emitter.generate_particle())
            .collect::<Vec<Box<dyn Particle>>>();

        for new_particle in new_particles {
            self.spawn(new_particle);
        }

        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }

        self.particles
            .retain(|particle| !particle.should_be_removed());
    }

    pub fn draw(&self) {
        for particle in self.get_particles().iter() {
            particle.draw();
        }
    }

    pub fn spawn(&mut self, particle: Box<dyn Particle>) {
        self.particles.push(particle);
    }

    pub fn get_particles(&self) -> &Vec<Box<dyn Particle>> {
        &self.particles
    }

    pub fn get_emitters(&self) -> &Vec<Box<dyn ParticleEmitter>> {
        &self.emitters
    }

    pub fn get_emitters_mut(&mut self) -> &mut Vec<Box<dyn ParticleEmitter>> {
        &mut self.emitters
    }

    pub fn add_emitter(&mut self, emitter: Box<dyn ParticleEmitter>) {
        self.emitters.push(emitter);
    }
}
