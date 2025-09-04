use std::{
    f64::consts::PI,
    path::{Path, PathBuf},
};

use ::rand::random_range;
use glam::DVec2;
use palette::Srgba;
use particula_rs::ParticleSystem;
use render_agnostic::Renderer;

use crate::{
    drawer::BallStyle,
    particle::{ParticleLayer, ShrinkingParticle, system::BallParticleSystem},
    util::lerp_color,
    wall::Wall,
};

#[derive(Clone)]
pub struct Ball {
    name: String,
    name_color: Srgba,
    physics_ball: PhysicsBall,
    style: Box<dyn BallStyle>,
    sound_path: PathBuf,
    particles: BallParticleSystem,
}

impl Ball {
    pub fn new(
        name: String,
        name_color: Srgba,
        physics_ball: PhysicsBall,
        mut style: Box<dyn BallStyle>,
        sound_path: PathBuf,
    ) -> Self {
        style.init(&physics_ball);

        Self {
            name,
            name_color,
            physics_ball,
            style,
            sound_path,
            particles: BallParticleSystem::default(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_name_color(&self) -> Srgba {
        self.name_color
    }

    pub fn get_physics_ball(&self) -> &PhysicsBall {
        &self.physics_ball
    }

    pub fn get_style(&self) -> &dyn BallStyle {
        self.style.as_ref()
    }

    pub fn render(&self, renderer: &mut dyn Renderer) {
        self.get_particles().render_back(renderer);
        self.get_style().render(self, renderer);
        self.get_particles().render_front(renderer);
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

    pub fn get_density(&self) -> f64 {
        self.physics_ball.get_density()
    }

    pub fn set_density(&mut self, density: f64) {
        self.physics_ball.set_density(density)
    }

    pub fn get_elasticity(&self) -> f64 {
        self.physics_ball.get_elasticity()
    }

    pub fn set_elasticity(&mut self, elasticity: f64) {
        self.physics_ball.set_elasticity(elasticity)
    }

    pub fn get_sound_path(&self) -> &Path {
        &self.sound_path
    }

    pub fn get_particles(&self) -> &BallParticleSystem {
        &self.particles
    }

    pub fn get_particles_mut(&mut self) -> &mut BallParticleSystem {
        &mut self.particles
    }

    pub fn handle_collision(&mut self, new_velocity: DVec2) {
        let dv = new_velocity.distance(self.get_velocity());

        if (self.get_name() == "Deep Blue" || self.get_name() == "Blue's Wife") && dv >= 150.0 {
            // TODO: Fix this to not be hard coded.
            let velocity = self.get_velocity();

            for _ in 0..20 {
                self.particles.add_particle(Box::new(ShrinkingParticle::new(
                    self.get_position()
                        + self.get_radius() * DVec2::from_angle(random_range(0.0..(2.0 * PI))),
                    random_range(0.125..=0.375)
                        * velocity.length()
                        * DVec2::from_angle(
                            velocity.to_angle() + random_range((-PI / 2.0)..(PI / 2.0)),
                        ),
                    random_range(2.0..=6.0),
                    lerp_color(
                        Srgba::new(0.0, 0.5, 1.0, 1.0),
                        Srgba::new(0.25, 0.0, 1.0, 1.0),
                        random_range(0.0..=1.0),
                    ),
                    random_range(0.25..=0.75),
                    ParticleLayer::random(),
                )));
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.set_position(self.get_position() + self.get_velocity() * dt);

        let position = self.get_position();

        for emitter in self.get_particles_mut().iter_emitters_mut() {
            emitter.set_position(position);
        }

        self.get_particles_mut().update(dt);
        self.style.update(&self.physics_ball, dt);
    }

    pub fn get_intersection_point(&self, wall: &dyn Wall) -> Option<DVec2> {
        wall.get_intersection_point(self.get_physics_ball())
    }
}

#[derive(Clone)]
pub struct PhysicsBall {
    position: DVec2,
    velocity: DVec2,
    radius: f64,
    density: f64,
    elasticity: f64,
}

impl PhysicsBall {
    pub fn new(
        position: DVec2,
        velocity: DVec2,
        radius: f64,
        density: f64,
        elasticity: f64,
    ) -> Self {
        Self {
            position,
            velocity,
            radius,
            density,
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

    pub fn get_density(&self) -> f64 {
        self.density
    }

    pub fn set_density(&mut self, density: f64) {
        self.density = density;
    }

    pub fn get_elasticity(&self) -> f64 {
        self.elasticity
    }

    pub fn set_elasticity(&mut self, elasticity: f64) {
        self.elasticity = elasticity;
    }

    pub fn get_volume(&self) -> f64 {
        PI * self.get_radius().powi(2)
    }

    pub fn get_mass(&self) -> f64 {
        self.get_volume() * self.get_density()
    }
}
