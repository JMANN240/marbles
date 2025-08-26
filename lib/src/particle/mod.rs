use std::f64::consts::PI;

use ::rand::{random_bool, random_range};
use dyn_clone::DynClone;
use glam::DVec2;
use palette::{FromColor, Hsla, Srgba};
use particula_rs::{Aging, MaxAging, Particle};

use crate::rendering::{Render, Renderer};

pub mod emitter;
pub mod system;

pub trait RenderParticle<C>: Render + Particle<Coordinate = C> + Send + Sync + DynClone {}

impl<C, T> RenderParticle<C> for T where
    T: Render + Particle<Coordinate = C> + Send + Sync + DynClone
{
}

dyn_clone::clone_trait_object!(<C> RenderParticle<C>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleLayer {
    Front,
    Back,
}

impl ParticleLayer {
    pub fn random() -> Self {
        if random_bool(0.5) {
            Self::Front
        } else {
            Self::Back
        }
    }
}

pub trait LayeredParticle: Particle<Coordinate = DVec2> + Render + Send + Sync + DynClone {
    fn get_particle_layer(&self) -> ParticleLayer;
    fn clone_box(&self) -> Box<dyn LayeredParticle>;
}

dyn_clone::clone_trait_object!(LayeredParticle);

#[derive(Clone)]
pub struct ShrinkingParticle {
    position: DVec2,
    velocity: DVec2,
    radius: f64,
    color: Srgba,
    age: f64,
    max_age: f64,
    layer: ParticleLayer,
}

impl ShrinkingParticle {
    pub fn new(
        position: DVec2,
        velocity: DVec2,
        radius: f64,
        color: Srgba,
        max_age: f64,
        layer: ParticleLayer,
    ) -> Self {
        Self {
            position,
            velocity,
            radius,
            color,
            age: 0.0,
            max_age,
            layer,
        }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_color(&self) -> Srgba {
        self.color
    }
}

impl LayeredParticle for ShrinkingParticle {
    fn get_particle_layer(&self) -> ParticleLayer {
        self.layer
    }

    fn clone_box(&self) -> Box<dyn LayeredParticle> {
        Box::new(self.clone())
    }
}

impl Particle for ShrinkingParticle {
    type Coordinate = DVec2;

    fn get_position(&self) -> DVec2 {
        self.position
    }

    fn update(&mut self, dt: f64) {
        self.position += self.velocity * dt;
        self.age += dt;
    }

    fn is_alive(&self) -> bool {
        MaxAging::is_alive(self)
    }
}

impl Render for ShrinkingParticle {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_circle(
            self.get_position(),
            self.get_radius() * (1.0 - self.get_age_percent()),
            self.get_color(),
        );
    }
}

impl Aging for ShrinkingParticle {
    fn get_age(&self) -> f64 {
        self.age
    }

    fn set_age(&mut self, age: f64) {
        self.age = age;
    }
}

impl MaxAging for ShrinkingParticle {
    fn get_max_age(&self) -> f64 {
        self.max_age
    }
}

#[derive(Clone)]
pub struct FireParticle {
    position: DVec2,
    radius: f64,
    age: f64,
    max_age: f64,
    layer: ParticleLayer,
}

impl FireParticle {
    pub fn new(position: DVec2, radius: f64, max_age: f64, layer: ParticleLayer) -> Self {
        Self {
            position,
            radius,
            age: 0.0,
            max_age,
            layer,
        }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Particle for FireParticle {
    type Coordinate = DVec2;

    fn get_position(&self) -> DVec2 {
        self.position
    }

    fn update(&mut self, dt: f64) {
        self.set_age(self.get_age() + dt);
        self.position -= DVec2::Y * 50.0 * dt;
    }

    fn is_alive(&self) -> bool {
        MaxAging::is_alive(self)
    }
}

impl Render for FireParticle {
    fn render(&self, renderer: &mut dyn Renderer) {
        let color = Srgba::new(
            1.0 * (1.0 - self.get_age_percent()) as f32 + 1.0 * self.get_age_percent() as f32,
            1.0 * (1.0 - self.get_age_percent()) as f32 + 0.0 * self.get_age_percent() as f32,
            0.0 * (1.0 - self.get_age_percent()) as f32 + 0.0 * self.get_age_percent() as f32,
            1.0,
        );

        renderer.render_circle(
            self.get_position(),
            self.get_radius() * (1.0 - self.get_age_percent()),
            color,
        );
    }
}

impl LayeredParticle for FireParticle {
    fn get_particle_layer(&self) -> ParticleLayer {
        self.layer
    }

    fn clone_box(&self) -> Box<dyn LayeredParticle> {
        Box::new(self.clone())
    }
}

impl Aging for FireParticle {
    fn get_age(&self) -> f64 {
        self.age
    }

    fn set_age(&mut self, age: f64) {
        self.age = age;
    }
}

impl MaxAging for FireParticle {
    fn get_max_age(&self) -> f64 {
        self.max_age
    }
}

#[derive(Clone)]
pub struct ConfettiParticle {
    position: DVec2,
    velocity: DVec2,
    radius: f64,
    color: Srgba,
    rotation: f64,
    rotation_speed: f64,
    age: f64,
    max_age: f64,
    layer: ParticleLayer,
}

impl ConfettiParticle {
    pub fn new(
        position: DVec2,
        velocity: DVec2,
        radius: f64,
        max_age: f64,
        layer: ParticleLayer,
    ) -> Self {
        Self {
            position,
            velocity,
            radius,
            color: Srgba::from_color(Hsla::new(random_range(0.0..360.0), 1.0, 0.5, 1.0)),
            rotation: random_range(0.0..=(PI / 2.0)),
            rotation_speed: random_range(1.0..=8.0),
            age: 0.0,
            max_age,
            layer,
        }
    }

    pub fn get_rotation(&self) -> f64 {
        self.rotation
    }

    pub fn get_rotation_speed(&self) -> f64 {
        self.rotation_speed
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_color(&self) -> Srgba {
        self.color
    }
}

impl Particle for ConfettiParticle {
    type Coordinate = DVec2;

    fn get_position(&self) -> DVec2 {
        self.position
    }

    fn update(&mut self, dt: f64) {
        self.set_age(self.get_age() + dt);
        self.position += self.velocity * dt;
        self.velocity += -0.01 * self.velocity * self.velocity.length() * dt;
        self.velocity += DVec2::Y * 100.0 * dt;
        self.rotation += self.rotation_speed * dt;
    }

    fn is_alive(&self) -> bool {
        MaxAging::is_alive(self)
    }
}

impl Render for ConfettiParticle {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_rectangle(
            self.get_position(),
            self.get_radius() * (1.0 - self.get_age_percent()),
            self.get_radius() * (1.0 - self.get_age_percent()),
            ::glam::dvec2(0.5, 0.5),
            self.rotation,
            self.color,
        );
    }
}

impl LayeredParticle for ConfettiParticle {
    fn get_particle_layer(&self) -> ParticleLayer {
        self.layer
    }

    fn clone_box(&self) -> Box<dyn LayeredParticle> {
        Box::new(self.clone())
    }
}

impl Aging for ConfettiParticle {
    fn get_age(&self) -> f64 {
        self.age
    }

    fn set_age(&mut self, age: f64) {
        self.age = age;
    }
}

impl MaxAging for ConfettiParticle {
    fn get_max_age(&self) -> f64 {
        self.max_age
    }
}

#[derive(Clone)]
pub struct StaticParticle {
    position: DVec2,
    radius: f64,
    color: Srgba,
    age: f64,
    max_age: f64,
    layer: ParticleLayer,
}

impl StaticParticle {
    pub fn new(
        position: DVec2,
        radius: f64,
        color: Srgba,
        max_age: f64,
        layer: ParticleLayer,
    ) -> Self {
        Self {
            position,
            radius,
            color,
            age: 0.0,
            max_age,
            layer,
        }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_color(&self) -> Srgba {
        self.color
    }
}

impl LayeredParticle for StaticParticle {
    fn get_particle_layer(&self) -> ParticleLayer {
        self.layer
    }

    fn clone_box(&self) -> Box<dyn LayeredParticle> {
        Box::new(self.clone())
    }
}

impl Particle for StaticParticle {
    type Coordinate = DVec2;

    fn get_position(&self) -> DVec2 {
        self.position
    }

    fn update(&mut self, dt: f64) {
        self.age += dt;
    }

    fn is_alive(&self) -> bool {
        MaxAging::is_alive(self)
    }
}

impl Render for StaticParticle {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_circle(
            self.get_position(),
            self.get_radius(),
            self.get_color(),
        );
    }
}

impl Aging for StaticParticle {
    fn get_age(&self) -> f64 {
        self.age
    }

    fn set_age(&mut self, age: f64) {
        self.age = age;
    }
}

impl MaxAging for StaticParticle {
    fn get_max_age(&self) -> f64 {
        self.max_age
    }
}
