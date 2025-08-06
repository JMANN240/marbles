use ::rand::{random_bool, random_range};
use macroquad::{color::hsl_to_rgb, prelude::*};

pub mod emitter;
pub mod system;

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

pub trait Particle {
    fn update(&mut self, dt: f64);
    fn draw(&self);
    fn get_layer(&self) -> ParticleLayer;

    fn should_be_removed(&self) -> bool;
}

pub trait AgingParticle: Particle {
    fn get_age(&self) -> f64;
    fn set_age(&mut self, age: f64);
}

pub trait MaxAgingParticle: AgingParticle {
    fn get_max_age(&self) -> f64;

    fn get_age_percent(&self) -> f64 {
        self.get_age() / self.get_max_age()
    }
}

pub struct BaseParticle {
    position: DVec2,
    radius: f64,
    color: Color,
    age: f64,
    max_age: f64,
    layer: ParticleLayer,
}

impl BaseParticle {
    pub fn _new(
        position: DVec2,
        radius: f64,
        color: Color,
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
}

impl Particle for BaseParticle {
    fn update(&mut self, dt: f64) {
        self.set_age(self.get_age() + dt);
    }

    fn draw(&self) {
        draw_circle(
            self.position.x as f32,
            self.position.y as f32,
            self.radius as f32,
            self.color,
        );
    }

    fn get_layer(&self) -> ParticleLayer {
        self.layer
    }

    fn should_be_removed(&self) -> bool {
        self.get_age() >= self.get_max_age()
    }
}

impl AgingParticle for BaseParticle {
    fn get_age(&self) -> f64 {
        self.age
    }

    fn set_age(&mut self, age: f64) {
        self.age = age;
    }
}

impl MaxAgingParticle for BaseParticle {
    fn get_max_age(&self) -> f64 {
        self.max_age
    }
}

pub struct ShrinkingParticle {
    position: DVec2,
    velocity: DVec2,
    radius: f64,
    color: Color,
    age: f64,
    max_age: f64,
    layer: ParticleLayer,
}

impl ShrinkingParticle {
    pub fn new(
        position: DVec2,
        velocity: DVec2,
        radius: f64,
        color: Color,
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
}

impl Particle for ShrinkingParticle {
    fn update(&mut self, dt: f64) {
        self.position += self.velocity * dt;
        self.age += dt;
    }

    fn draw(&self) {
        draw_circle(
            self.position.x as f32,
            self.position.y as f32,
            (self.radius * (1.0 - self.get_age_percent())) as f32,
            self.color,
        );
    }

    fn get_layer(&self) -> ParticleLayer {
        self.layer
    }

    fn should_be_removed(&self) -> bool {
        self.age >= self.max_age
    }
}

impl AgingParticle for ShrinkingParticle {
    fn get_age(&self) -> f64 {
        self.age
    }

    fn set_age(&mut self, age: f64) {
        self.age = age;
    }
}

impl MaxAgingParticle for ShrinkingParticle {
    fn get_max_age(&self) -> f64 {
        self.max_age
    }
}

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
}

impl Particle for FireParticle {
    fn update(&mut self, dt: f64) {
        self.set_age(self.get_age() + dt);
        self.position -= DVec2::Y * 50.0 * dt;
    }

    fn draw(&self) {
        let color = Color {
            r: YELLOW.r * (1.0 - self.get_age_percent()) as f32
                + RED.r * self.get_age_percent() as f32,
            g: YELLOW.g * (1.0 - self.get_age_percent()) as f32
                + RED.g * self.get_age_percent() as f32,
            b: YELLOW.b * (1.0 - self.get_age_percent()) as f32
                + RED.b * self.get_age_percent() as f32,
            a: 1.0,
        };

        draw_circle(
            self.position.x as f32,
            self.position.y as f32,
            (self.radius * (1.0 - self.get_age_percent())) as f32,
            color,
        );
    }

    fn get_layer(&self) -> ParticleLayer {
        self.layer
    }

    fn should_be_removed(&self) -> bool {
        self.age >= self.max_age
    }
}

impl AgingParticle for FireParticle {
    fn get_age(&self) -> f64 {
        self.age
    }

    fn set_age(&mut self, age: f64) {
        self.age = age;
    }
}

impl MaxAgingParticle for FireParticle {
    fn get_max_age(&self) -> f64 {
        self.max_age
    }
}

pub struct ConfettiParticle {
    position: DVec2,
    velocity: DVec2,
    radius: f64,
    color: Color,
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
            color: hsl_to_rgb(random_range(0.0..1.0), 1.0, 0.5),
            rotation_speed: random_range(1.0..=8.0),
            age: 0.0,
            max_age,
            layer,
        }
    }
}

impl Particle for ConfettiParticle {
    fn update(&mut self, dt: f64) {
        self.set_age(self.get_age() + dt);
        self.position += self.velocity * dt;
        self.velocity += -0.01 * self.velocity * self.velocity.length() * dt;
        self.velocity += DVec2::Y * 100.0 * dt;
    }

    fn draw(&self) {
        draw_rectangle_ex(
            self.position.x as f32,
            self.position.y as f32,
            self.radius as f32 * (1.0 - self.get_age_percent()) as f32,
            self.radius as f32 * (1.0 - self.get_age_percent()) as f32,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: get_time() as f32 * self.rotation_speed as f32,
                color: self.color,
            },
        );
    }

    fn get_layer(&self) -> ParticleLayer {
        self.layer
    }

    fn should_be_removed(&self) -> bool {
        self.age >= self.max_age
    }
}

impl AgingParticle for ConfettiParticle {
    fn get_age(&self) -> f64 {
        self.age
    }

    fn set_age(&mut self, age: f64) {
        self.age = age;
    }
}

impl MaxAgingParticle for ConfettiParticle {
    fn get_max_age(&self) -> f64 {
        self.max_age
    }
}
