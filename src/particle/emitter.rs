use ::rand::random_range;
use macroquad::prelude::*;

use super::Particle;

pub trait ParticleEmitter {
    fn update(&mut self);
    fn set_position(&mut self, position: DVec2);
    fn should_generate_particle(&self) -> bool;
    fn generate_particle(&self) -> Box<dyn Particle>;
}

pub struct BaseParticleEmitter<F> {
    position: DVec2,
    spread: f64,
    particle_generator: F,
}

impl<F> BaseParticleEmitter<F>
where
    F: Fn(DVec2, f64) -> Box<dyn Particle>,
{
    pub fn new(position: DVec2, spread: f64, particle_generator: F) -> Self {
        Self {
            position,
            spread,
            particle_generator,
        }
    }
}

impl<F> ParticleEmitter for BaseParticleEmitter<F>
where
    F: Fn(DVec2, f64) -> Box<dyn Particle>,
{
    fn update(&mut self) {}

    fn set_position(&mut self, position: DVec2) {
        self.position = position;
    }

    fn should_generate_particle(&self) -> bool {
        true
    }

    fn generate_particle(&self) -> Box<dyn Particle> {
        let position = self.position
            + dvec2(
                random_range(-self.spread..=self.spread),
                random_range(-self.spread..=self.spread),
            );

        (self.particle_generator)(position, self.spread)
    }
}

pub struct FrequencyParticleEmitter<F> {
    position: DVec2,
    spread: f64,
    frequency: f64,
    last_generated_time: Option<f64>,
    should_generate_particle: bool,
    particle_generator: F,
}

impl<F> FrequencyParticleEmitter<F>
where
    F: Fn(DVec2, f64) -> Box<dyn Particle>,
{
    pub fn new(position: DVec2, spread: f64, frequency: f64, particle_generator: F) -> Self {
        Self {
            position,
            spread,
            frequency,
            last_generated_time: None,
            should_generate_particle: false,
            particle_generator,
        }
    }

    pub fn get_period(&self) -> f64 {
        1.0 / self.frequency
    }
}

impl<F> ParticleEmitter for FrequencyParticleEmitter<F>
where
    F: Fn(DVec2, f64) -> Box<dyn Particle>,
{
    fn update(&mut self) {
        let time = get_time();

        if self.should_generate_particle {
            self.should_generate_particle = false;
        } else if self
            .last_generated_time
            .is_none_or(|last_generated_time| time > last_generated_time + self.get_period())
        {
            self.should_generate_particle = true;
            self.last_generated_time = Some(time);
        }
    }

    fn set_position(&mut self, position: DVec2) {
        self.position = position;
    }

    fn should_generate_particle(&self) -> bool {
        self.should_generate_particle
    }

    fn generate_particle(&self) -> Box<dyn Particle> {
        let position = self.position
            + dvec2(
                random_range(-self.spread..=self.spread),
                random_range(-self.spread..=self.spread),
            );

        (self.particle_generator)(position, self.spread)
    }
}
