use std::f64::consts::PI;

use macroquad::{audio::Sound, prelude::*};

use crate::wall::Wall;

pub trait Ball {
    fn get_name(&self) -> &str;
    fn get_position(&self) -> DVec2;
    fn set_position(&mut self, position: DVec2);
    fn get_velocity(&self) -> DVec2;
    fn set_velocity(&mut self, velocity: DVec2);
    fn get_radius(&self) -> f64;
    fn get_elasticity(&self) -> f64;
    fn get_color(&self) -> Color;
    fn get_sound(&self) -> &Sound;
    fn draw(&self);

    fn get_mass(&self) -> f64 {
        PI * self.get_radius() * self.get_radius()
    }

    fn get_intersection_point(&self, wall: &Wall) -> Option<DVec2> {
        let x1 = wall.get_start().x;
        let y1 = wall.get_start().y;
        let x2 = wall.get_end().x;
        let y2 = wall.get_end().y;

        let dx = x2 - x1;
        let dy = y2 - y1;

        let fx = x1 - self.get_position().x;
        let fy = y1 - self.get_position().y;

        let a = dx * dx + dy * dy;
        let b = 2.0 * (fx * dx + fy * dy);
        let c = fx * fx + fy * fy - self.get_radius() * self.get_radius();

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = if discriminant == 0.0 {
                -b / (2.0 * a)
            } else {
                let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
                let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

                let t1_valid = (0.0..=1.0).contains(&t1);
                let t2_valid = (0.0..=1.0).contains(&t2);

                match (t1_valid, t2_valid) {
                    (true, true) => t1.midpoint(t2),
                    (true, false) => t1,
                    (false, true) => t2,
                    (false, false) => return None,
                }
            };

            if (0.0..=1.0).contains(&t) {
                Some(dvec2(x1 + t * dx, y1 + t * dy))
            } else {
                None
            }
        }
    }
}

#[derive(Clone)]
pub struct BaseBall {
    name: String,
    position: DVec2,
    velocity: DVec2,
    radius: f64,
    elasticity: f64,
    color: Color,
    sound: Sound,
}

impl BaseBall {
    pub fn new(
        name: String,
        position: DVec2,
        velocity: DVec2,
        radius: f64,
        elasticity: f64,
        color: Color,
        sound: Sound,
    ) -> Self {
        Self {
            name,
            position,
            velocity,
            radius,
            elasticity,
            color,
            sound,
        }
    }
}

impl Ball for BaseBall {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_position(&self) -> DVec2 {
        self.position
    }

    fn set_position(&mut self, position: DVec2) {
        self.position = position;
    }

    fn get_velocity(&self) -> DVec2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: DVec2) {
        self.velocity = velocity;
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }

    fn get_elasticity(&self) -> f64 {
        self.elasticity
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_sound(&self) -> &Sound {
        &self.sound
    }

    fn draw(&self) {
        draw_circle(
            self.get_position().x as f32,
            self.get_position().y as f32,
            self.get_radius() as f32,
            self.get_color(),
        );
    }
}

#[derive(Clone)]
pub struct TrackedBall {
    name: String,
    positions: Vec<DVec2>,
    velocity: DVec2,
    radius: f64,
    elasticity: f64,
    color: Color,
    sound: Sound,
}

impl TrackedBall {
    pub fn new(
        name: String,
        position: DVec2,
        velocity: DVec2,
        radius: f64,
        elasticity: f64,
        color: Color,
        sound: Sound,
    ) -> Self {
        Self {
            name,
            positions: vec![position],
            velocity,
            radius,
            elasticity,
            color,
            sound,
        }
    }
}

impl Ball for TrackedBall {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_position(&self) -> DVec2 {
        *self.positions.last().unwrap()
    }

    fn set_position(&mut self, position: DVec2) {
        self.positions.push(position);
        if self.positions.len() > 1000 {
            self.positions.remove(0);
        }
    }

    fn get_velocity(&self) -> DVec2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: DVec2) {
        self.velocity = velocity;
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }

    fn get_elasticity(&self) -> f64 {
        self.elasticity
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_sound(&self) -> &Sound {
        &self.sound
    }

    fn draw(&self) {
        for (index, position) in self.positions.iter().enumerate() {
            let percent = (index + 1) as f32 / self.positions.len() as f32;
            draw_circle(
                position.x as f32,
                position.y as f32,
                self.get_radius() as f32 * percent,
                Color {
                    a: 1.0,
                    r: self.get_color().r * percent,
                    g: self.get_color().g * percent,
                    b: self.get_color().b * percent,
                },
            );
        }

        // draw_line(
        //     self.get_position().x as f32,
        //     self.get_position().y as f32,
        //     (self.get_position() + self.get_velocity()).x as f32,
        //     (self.get_position() + self.get_velocity()).y as f32,
        //     2.0,
        //     WHITE,
        // );
    }
}
