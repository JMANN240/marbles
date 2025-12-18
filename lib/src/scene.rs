use std::{any::Any, f64::consts::PI, time::Duration};

use ::rand::random_range;
use dyn_clone::DynClone;
use glam::{DVec2, dvec2};
use palette::Srgba;
use particula_rs::{ParticleEmitter, ParticleSystem, VecParticleSystem};
use rand::{rng, seq::SliceRandom};
use render_agnostic::Renderer;

use crate::{
    ball::Ball,
    collision::Collision,
    particle::{ConfettiParticle, ParticleLayer, RenderParticle, ShrinkingParticle},
    powerup::{Powerup, special::Special},
    rendering::Render,
    wall::Wall,
};

const MIN_OVERLAP: f64 = 0.01;

pub trait SceneParticleEmitter:
    ParticleEmitter<ParticleType = Box<dyn RenderParticle<DVec2>>> + Send + Sync + DynClone
{
}

dyn_clone::clone_trait_object!(SceneParticleEmitter);

pub type SceneParticleSystem =
    VecParticleSystem<Box<dyn RenderParticle<DVec2>>, Box<dyn SceneParticleEmitter>>;

#[derive(Clone)]
pub struct Scene {
    time: f64,
    balls: Vec<Ball>,
    powerups: Vec<Box<dyn Powerup>>,
    walls: Vec<Box<dyn Wall>>,
    winners: Vec<usize>,
    win_times: Vec<Duration>,
    particles: SceneParticleSystem,
}

impl Scene {
    pub fn new(
        balls: Vec<Ball>,
        powerups: Vec<Box<dyn Powerup>>,
        walls: Vec<Box<dyn Wall>>,
    ) -> Self {
        Self {
            time: 0.0,
            balls,
            powerups,
            walls,
            winners: Vec::new(),
            win_times: Vec::new(),
            particles: VecParticleSystem::default(),
        }
    }

    pub fn get_balls(&self) -> &Vec<Ball> {
        &self.balls
    }

    pub fn get_balls_mut(&mut self) -> &mut Vec<Ball> {
        &mut self.balls
    }

    pub fn get_current_winner(&self) -> Option<&Ball> {
        self.get_balls().iter().max_by_key(|ball| ball.get_position().y as i32)
    }

    pub fn get_walls(&self) -> &Vec<Box<dyn Wall>> {
        &self.walls
    }

    pub fn get_powerups(&self) -> &Vec<Box<dyn Powerup>> {
        &self.powerups
    }

    pub fn get_winners(&self) -> &Vec<usize> {
        &self.winners
    }

    pub fn get_win_times(&self) -> &Vec<Duration> {
        &self.win_times
    }

    pub fn all_won(&self) -> bool {
        self.get_balls().len() == self.get_winners().len()
    }

    pub fn get_particles(&self) -> &SceneParticleSystem {
        &self.particles
    }

    pub fn update(&self, dt: f64, timescale: f64, physics_steps: usize) -> (Self, Vec<Collision>) {
        let mut new_scene = self.clone();

        let step_dt = dt * timescale / physics_steps as f64;

        let mut collisions = Vec::new();

        for _ in 0..physics_steps {
            let (update_scene, mut update_collisions) = new_scene.step_physics(step_dt);
            collisions.append(&mut update_collisions);
            new_scene = update_scene;
        }

        (new_scene, collisions)
    }

    pub fn step_physics(&self, dt: f64) -> (Self, Vec<Collision>) {
        let stepped_velocities_scene = self.step_velocities(dt); // Overlapping old positions, new velocities
        let (resolved_collisions_scene, collisions) = stepped_velocities_scene.resolve_collisions(); // NOT overlapping old positions, new velocities

        let new_time = resolved_collisions_scene.time + dt;

        let new_walls = resolved_collisions_scene
            .get_walls()
            .iter()
            .map(|wall| wall.update(dt))
            .collect::<Vec<Box<dyn Wall>>>();

        let mut should_shuffle = false;

        let mut new_balls = resolved_collisions_scene // Overlapping new positions, new velocities, powered up
            .get_balls()
            .iter()
            .map(|ball| {
                let mut new_ball = ball.update(dt);

                for powerup in resolved_collisions_scene.get_powerups() {
                    if powerup.is_active() {
                        if powerup.is_colliding_with(ball) {
                            powerup.apply(&mut new_ball);

                            if (powerup.as_ref() as &dyn Any).is::<Special>() {
                                if new_ball.get_id() == "Black Hole" {
                                    new_ball.set_density(1000.0);
                                    new_ball.set_name(format!("Supermassive {}", ball.get_name()));
                                } else if new_ball.get_id() == "Green Machine" {
                                    let new_physics_ball = new_ball.get_physics_ball_mut();
                                    let new_physics_ball_time = new_physics_ball.get_time();
                                    new_physics_ball
                                        .get_velocity_coefficient_mut()
                                        .add_modifier(
                                            new_physics_ball_time..=(new_physics_ball_time + 10.0),
                                            2.0,
                                        );
                                    new_physics_ball
                                        .get_gravity_coefficient_mut()
                                        .add_modifier(
                                            new_physics_ball_time..=(new_physics_ball_time + 10.0),
                                            2.0,
                                        );
                                } else if new_ball.get_id() == "IKEA" {
                                    new_ball.set_radius(ball.get_radius() * 0.5);
                                    new_ball.set_name(format!("{} Junior", ball.get_name()));
                                } else if new_ball.get_id() == "Psycho" {
                                    let new_physics_ball = new_ball.get_physics_ball_mut();
                                    let new_physics_ball_time = new_physics_ball.get_time();
                                    new_physics_ball
                                        .get_bloodbath_mut()
                                        .add_modifier(
                                            new_physics_ball_time..=(new_physics_ball_time + 10.0),
                                            true,
                                        );
                                } else if new_ball.get_id() == "Instabwillity" {
                                    should_shuffle = true;
                                }
                            }
                        }

                        for other_ball in resolved_collisions_scene
                            .get_balls()
                            .iter()
                            .filter(|other_ball| ball.get_position() != other_ball.get_position())
                        {
                            if (powerup.as_ref() as &dyn Any).is::<Special>()
                                && powerup.is_colliding_with(other_ball)
                            {
                                if other_ball.get_id() == "Fireball" {
                                    let direction =
                                        new_ball.get_position() - other_ball.get_position();

                                    new_ball.set_velocity(
                                        direction.normalize() * 200000.0
                                            / (direction.length() + 200.0),
                                    );
                                } else if other_ball.get_id() == "White Light" {
                                    new_ball.set_velocity(DVec2::ZERO);

                                    let new_physics_ball = new_ball.get_physics_ball_mut();
                                    let new_physics_ball_time = new_physics_ball.get_time();
                                    new_physics_ball
                                        .get_gravity_coefficient_mut()
                                        .add_modifier(
                                            new_physics_ball_time..=(new_physics_ball_time + 10.0),
                                            0.0,
                                        );
                                } else if other_ball.get_id() == "Deep Blue" {
                                    let new_physics_ball = new_ball.get_physics_ball_mut();
                                    let new_physics_ball_time = new_physics_ball.get_time();
                                    new_physics_ball
                                        .get_velocity_coefficient_mut()
                                        .add_modifier(
                                            new_physics_ball_time..=(new_physics_ball_time + 10.0),
                                            0.5,
                                        );
                                    new_physics_ball.get_gravity_coefficient_mut().add_modifier(
                                        new_physics_ball_time..=(new_physics_ball_time + 10.0),
                                        0.05,
                                    );
                                } else if other_ball.get_id() == "Timmy J" {
                                    if let Some(current_winner) = self.get_current_winner() {
                                        new_ball.set_position(
                                            if new_ball.get_id() == "Deep Blue" {
                                                current_winner.get_position()
                                            } else if new_ball.get_id() == current_winner.get_id() {
                                                if let Some(deep_blue) = self.get_balls().iter().find(|ball| ball.get_id() == "Deep Blue") {
                                                    deep_blue.get_position()
                                                } else {
                                                    new_ball.get_position()
                                                }
                                            } else {
                                                new_ball.get_position()
                                            }
                                        );
                                    }
                                }
                            }
                        }

                        for any_ball in resolved_collisions_scene
                            .get_balls()
                            .iter()
                        {
                            if (powerup.as_ref() as &dyn Any).is::<Special>()
                                && powerup.is_colliding_with(any_ball)
                            {
                                if any_ball.get_id() == "Timmy J" {
                                    if let Some(current_winner) = self.get_current_winner() {
                                        let target_ball = if new_ball.get_id() == "Deep Blue" {
                                            current_winner.clone()
                                        } else if new_ball.get_id() == current_winner.get_id() {
                                            if let Some(deep_blue) = self.get_balls().iter().find(|ball| ball.get_id() == "Deep Blue") {
                                                deep_blue.clone()
                                            } else {
                                                new_ball.clone()
                                            }
                                        } else {
                                            new_ball.clone()
                                        };

                                        new_ball.set_position(target_ball.get_position());
                                        new_ball.set_velocity(target_ball.get_velocity());
                                    }
                                }
                            }
                        }
                    }
                }

                new_ball
            })
            .collect::<Vec<Ball>>();

        let new_powerups = resolved_collisions_scene
            .get_powerups()
            .iter()
            .map(|powerup| {
                let mut new_powerup = powerup.update(dt);

                for ball in resolved_collisions_scene.get_balls() {
                    if powerup.is_active() {
                        if powerup.is_colliding_with(ball) {
                            new_powerup.consume();

                            if let Some(special) = (new_powerup.as_mut() as &mut dyn Any).downcast_mut::<Special>() {
                                if ball.get_id() == "Black Hole" {
                                    special.set_text("SUPERMASSIVE");
                                } else if ball.get_id() == "Green Machine" {
                                    special.set_text("FASTFORWARD");
                                } else if ball.get_id() == "Fireball" {
                                    special.set_text("EXPLOSION");
                                } else if ball.get_id() == "White Light" {
                                    special.set_text("FREEZEFRAME");
                                } else if ball.get_id() == "Deep Blue" {
                                    special.set_text("UNDERWATER");
                                } else if ball.get_id() == "IKEA" {
                                    special.set_text("JUNIOR");
                                } else if ball.get_id() == "Psycho" {
                                    special.set_text("BLOODBATH");
                                } else if ball.get_id() == "Timmy J" {
                                    special.set_text("BLUE IS KING");
                                } else if ball.get_id() == "Instabwillity" {
                                    special.set_text("CHAOS");
                                } else {
                                    special.set_text("nothing");
                                }
                            }
                        }
                    }
                }

                new_powerup
            })
            .collect();

        if should_shuffle {
            let mut new_new_balls = new_balls.clone();
            new_new_balls.shuffle(&mut rng());

            for (new_ball, new_new_ball) in new_balls.iter_mut().zip(new_new_balls.iter()) {
                new_ball.set_position(new_new_ball.get_position());
            }
        }

        let mut new_winners = resolved_collisions_scene.get_winners().clone();
        let mut new_win_times = resolved_collisions_scene.get_win_times().clone();
        let mut new_particles = resolved_collisions_scene.particles.clone();

        for (index, ball) in new_balls.iter().enumerate() {
            for wall in new_walls.iter() {
                let maybe_intersection_point = ball.get_intersection_point(wall.as_ref());

                if maybe_intersection_point.is_some()
                    && wall.is_goal()
                    && !self.winners.contains(&index)
                {
                    new_winners.push(index);
                    new_win_times.push(Duration::from_secs_f64(self.time));

                    for _ in 0..100 {
                        new_particles.add_particle(Box::new(ConfettiParticle::new(
                            ball.get_position()
                                + ball.get_radius()
                                    * DVec2::from_angle(random_range(0.0..(2.0 * PI))),
                            DVec2::from_angle(random_range((1.25 * PI)..(1.75 * PI)))
                                * random_range(100.0..=1000.0),
                            random_range(4.0..=8.0),
                            2.0,
                            ParticleLayer::random(),
                        )));
                    }
                }
            }
        }

        let mut updated_scene = Scene {
            time: new_time,
            balls: new_balls,
            powerups: new_powerups,
            walls: new_walls,
            winners: new_winners,
            win_times: new_win_times,
            particles: new_particles,
        };

        for collision in &collisions {
            updated_scene
                .particles
                .add_particle(Box::new(ShrinkingParticle::new(
                    collision.position,
                    DVec2::ZERO,
                    collision.volume as f64 * 10.0,
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                    0.2,
                    ParticleLayer::Front,
                )));
        }

        updated_scene.particles.update(dt);

        (updated_scene, collisions)
    }

    pub fn step_velocities(&self, dt: f64) -> Self {
        let new_balls = self
            .balls
            .iter()
            .map(|ball| {
                let mut new_ball = ball.clone();

                let mut velocity_offsets = Vec::new();

                // Gravity
                velocity_offsets.push(dvec2(0.0, 500.0) * dt * new_ball.get_physics_ball().get_gravity_coefficient().get_value(new_ball.get_time()));

                // Walls
                let wall_intersection_points = self
                    .get_walls()
                    .iter()
                    .filter_map(|wall| ball.get_intersection_point(wall.as_ref()))
                    .collect::<Vec<DVec2>>();
                let number_of_wall_intersection_points = wall_intersection_points.len();

                for intersection_point in wall_intersection_points {
                    let intersection_vector = ball.get_position() - intersection_point;
                    velocity_offsets.push(
                        -((2.0 * ball.get_velocity()).dot(intersection_vector)
                            / (intersection_vector.length() * intersection_vector.length()))
                            * (intersection_vector)
                            * ball.get_elasticity()
                            / number_of_wall_intersection_points as f64,
                    );
                }

                // Other balls {
                for other_ball in self
                    .get_balls()
                    .iter()
                    .filter(|other_ball| ball.get_position() != other_ball.get_position())
                {
                    let intersection_vector = ball.get_position() - other_ball.get_position();

                    if intersection_vector.length() < ball.get_radius() + other_ball.get_radius() {
                        velocity_offsets.push(
                            -(2.0 * other_ball.get_physics_ball().get_mass()
                                / (ball.get_physics_ball().get_mass()
                                    + other_ball.get_physics_ball().get_mass()))
                                * ((ball.get_velocity() - other_ball.get_velocity())
                                    .dot(intersection_vector)
                                    / (intersection_vector.length()
                                        * intersection_vector.length()))
                                * (intersection_vector)
                                * ball.get_elasticity(),
                        );
                    }
                }

                let velocity_offset = velocity_offsets.iter().sum::<DVec2>();

                let new_velocity = ball.get_velocity() + velocity_offset;

                let dv = new_velocity.distance(ball.get_velocity());

                if dv >= 100.0 {
                    new_ball.handle_collision(new_velocity);
                }

                new_ball.set_velocity(new_velocity);

                new_ball
            })
            .collect();

        Self {
            balls: new_balls,
            ..self.clone()
        }
    }

    pub fn resolve_collisions(&self) -> (Self, Vec<Collision>) {
        let mut collisions = Vec::new();

        let new_balls = self
            .balls
            .iter()
            .map(|ball| {
                let mut new_ball = ball.clone();

                let mut position_offsets = Vec::new();

                // Walls
                let wall_intersection_points = self
                    .get_walls()
                    .iter()
                    .filter_map(|wall| ball.get_intersection_point(wall.as_ref()))
                    .collect::<Vec<DVec2>>();

                for intersection_point in wall_intersection_points {
                    let intersection_vector = ball.get_position() - intersection_point;
                    let overlap = MIN_OVERLAP.max(ball.get_radius() - intersection_vector.length());
                    position_offsets.push(intersection_vector.normalize() * overlap);

                    let v_proj = ball.get_velocity().project_onto(intersection_vector);

                    if v_proj.length() > 30.0 {
                        collisions.push(Collision::new(
                            ball.get_sound_path().to_path_buf(),
                            ((v_proj.length() as f32 - 30.0) * 0.005).min(1.0),
                            intersection_point,
                        ));
                    }
                }

                // Other balls {
                for other_ball in self
                    .get_balls()
                    .iter()
                    .filter(|other_ball| ball.get_position() != other_ball.get_position())
                {
                    let intersection_point =
                        ball.get_position().midpoint(other_ball.get_position());
                    let intersection_vector = ball.get_position() - other_ball.get_position();

                    if intersection_vector.length() < ball.get_radius() + other_ball.get_radius() {
                        let overlap = MIN_OVERLAP.max(
                            ball.get_radius() + other_ball.get_radius()
                                - intersection_vector.length(),
                        );
                        position_offsets.push(intersection_vector.normalize() * overlap);

                        let v_proj = ball.get_velocity().project_onto(intersection_vector);

                        if v_proj.length() > 30.0 {
                            collisions.push(Collision::new(
                                ball.get_sound_path().to_path_buf(),
                                ((v_proj.length() as f32 - 30.0) * 0.005).min(1.0),
                                intersection_point,
                            ));
                        }

                        if *other_ball.get_physics_ball().get_bloodbath().get_value(other_ball.get_time()) {
                            let new_ball_density = *new_ball.get_physics_ball().get_density().get_value(0.0);
                            let new_ball_gravity_coefficient = *new_ball.get_physics_ball().get_gravity_coefficient().get_value(0.0);
                            let new_physics_ball = new_ball.get_physics_ball_mut();
                            let new_physics_ball_time = new_physics_ball.get_time();
                            new_physics_ball.get_density_mut().add_modifier(
                                new_physics_ball_time..=(new_physics_ball_time + 10.0),
                                new_ball_density / 2.0
                            );
                            new_physics_ball.get_gravity_coefficient_mut().add_modifier(
                                new_physics_ball_time..=(new_physics_ball_time + 10.0),
                                new_ball_gravity_coefficient / 100.0
                            );
                        }
                    }
                }

                let position_offset = position_offsets.iter().sum::<DVec2>();

                new_ball.set_position(ball.get_position() + position_offset);

                new_ball
            })
            .collect();

        (
            Self {
                balls: new_balls,
                ..self.clone()
            },
            collisions,
        )
    }
}

impl Render for Scene {
    fn render(&self, renderer: &mut dyn Renderer) {
        for wall in self.get_walls().iter() {
            wall.render(renderer);
        }

        for ball in self.get_balls().iter() {
            ball.render(renderer);
        }

        for powerup in self.get_powerups().iter() {
            powerup.render(renderer);
        }

        for particle in self.get_particles().iter_particles() {
            particle.render(renderer);
        }
    }
}
