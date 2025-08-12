use particula_rs::{ParticleEmitter, ParticleSystem};

use crate::particle::{emitter::BallParticleEmitter, LayeredParticle, ParticleLayer};

#[derive(Default)]
pub struct BallParticleSystem {
    particles: Vec<Box<dyn LayeredParticle>>,
    emitters: Vec<Box<BallParticleEmitter>>,
}

impl BallParticleSystem {
    pub fn draw_front(&self) {
        for particle in self
            .iter_particles()
            .filter(|particle| particle.get_particle_layer() == ParticleLayer::Front)
        {
            particle.draw();
        }
    }

    pub fn draw_back(&self) {
        for particle in self
            .iter_particles()
            .filter(|particle| particle.get_particle_layer() == ParticleLayer::Back)
        {
            particle.draw();
        }
    }
}

impl ParticleSystem for BallParticleSystem {
    type ParticleType = dyn LayeredParticle;
    type EmitterType = BallParticleEmitter;

    fn iter_particles(
        &self,
    ) -> impl Iterator<Item = &Box<Self::ParticleType>> {
        self.particles.iter()
    }

    fn iter_particles_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<Self::ParticleType>> {
        self.particles.iter_mut()
    }

    fn iter_emitters(
        &self,
    ) -> impl Iterator<Item = &Box<Self::EmitterType>> {
        self.emitters.iter()
    }

    fn iter_emitters_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Box<Self::EmitterType>> {
        self.emitters.iter_mut()
    }

    fn add_particle(&mut self, particle: Box<Self::ParticleType>) {
        self.particles.push(particle);
    }

    fn add_emitter(&mut self, emitter: Box<Self::EmitterType>) {
        self.emitters.push(emitter);
    }

    fn clean_particles(&mut self) {
        self.particles.retain(|particle| particle.is_alive());
    }

    fn clean_emitters(&mut self) {
        self.emitters.retain(|emitter| emitter.is_alive());
    }
}
