use particula_rs::{ParticleEmitter, ParticleSystem};

use crate::{
    particle::{LayeredParticle, ParticleLayer, emitter::BallParticleEmitter},
    rendering::Renderer,
};

#[derive(Clone, Default)]
pub struct BallParticleSystem {
    particles: Vec<Box<dyn LayeredParticle>>,
    emitters: Vec<BallParticleEmitter>,
}

impl BallParticleSystem {
    pub fn render_front(&self, renderer: &mut dyn Renderer) {
        for particle in self
            .iter_particles()
            .filter(|particle| particle.get_particle_layer() == ParticleLayer::Front)
        {
            particle.render(renderer);
        }
    }

    pub fn render_back(&self, renderer: &mut dyn Renderer) {
        for particle in self
            .iter_particles()
            .filter(|particle| particle.get_particle_layer() == ParticleLayer::Back)
        {
            particle.render(renderer);
        }
    }
}

impl ParticleSystem for BallParticleSystem {
    type ParticleType = Box<dyn LayeredParticle>;
    type EmitterType = BallParticleEmitter;

    fn iter_particles(&self) -> impl Iterator<Item = &Self::ParticleType> {
        self.particles.iter()
    }

    fn iter_particles_mut(&mut self) -> impl Iterator<Item = &mut Self::ParticleType> {
        self.particles.iter_mut()
    }

    fn iter_emitters(&self) -> impl Iterator<Item = &Self::EmitterType> {
        self.emitters.iter()
    }

    fn iter_emitters_mut(&mut self) -> impl Iterator<Item = &mut Self::EmitterType> {
        self.emitters.iter_mut()
    }

    fn add_particle(&mut self, particle: Self::ParticleType) {
        self.particles.push(particle);
    }

    fn add_emitter(&mut self, emitter: Self::EmitterType) {
        self.emitters.push(emitter);
    }

    fn clean_particles(&mut self) {
        self.particles.retain(|particle| particle.is_alive());
    }

    fn clean_emitters(&mut self) {
        self.emitters.retain(|emitter| emitter.is_alive());
    }
}
