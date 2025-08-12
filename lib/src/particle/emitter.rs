use macroquad::prelude::*;
use particula_rs::ParticleEmitter;

use crate::particle::LayeredParticle;

type ParticleFunction = dyn Fn(DVec2) -> Box<dyn LayeredParticle>;

pub struct BallParticleEmitter {
    position: DVec2,
    time: f64,
    last_emitted_time: Option<f64>,
    frequency: f64,
    particle_function: Box<ParticleFunction>
}

impl BallParticleEmitter {
    pub fn new(position: DVec2, frequency: f64, particle_function: Box<ParticleFunction>) -> Self {
        Self { position, time: 0.0, last_emitted_time: None, frequency, particle_function }
    }

    pub fn get_period(&self) -> f64 {
        1.0 / self.frequency
    }

    pub fn set_position(&mut self, position: DVec2) {
        self.position = position;
    }
}

impl ParticleEmitter for BallParticleEmitter {
    type ParticleType = dyn LayeredParticle;

    fn update(&mut self, dt: f64) -> Vec<Box<Self::ParticleType>> {
        self.time += dt;
        
        if self.last_emitted_time.is_none_or(|last_emitted_time| self.time - last_emitted_time > self.get_period()) {
            self.last_emitted_time = Some(self.time);
            vec![(self.particle_function)(self.position)]
        } else {
            vec![]
        }
    }

    fn is_alive(&self) -> bool {
        true
    }
}
