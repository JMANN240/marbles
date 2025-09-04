use std::f64::consts::PI;

use glam::{DVec2, dvec2};
use palette::Srgba;
use render_agnostic::Renderer;

use crate::{ball::PhysicsBall, rendering::Render};

use super::Wall;

#[derive(Clone)]
pub struct CircleWall {
    position: DVec2,
    radius: f64,
    start: f64,
    end: f64,
    is_goal: bool,
}

impl CircleWall {
    pub fn new(position: DVec2, radius: f64, start: f64, end: f64, is_goal: bool) -> Self {
        Self {
            position,
            radius,
            start,
            end,
            is_goal,
        }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_start(&self) -> f64 {
        self.start
    }

    pub fn get_start_mod(&self) -> f64 {
        self.get_start() % 360.0
    }

    pub fn get_end(&self) -> f64 {
        self.end
    }

    pub fn get_end_mod(&self) -> f64 {
        self.get_end() % 360.0
    }
}

impl Wall for CircleWall {
    fn update(&mut self, _dt: f64) {}

    fn get_intersection_point(&self, ball: &PhysicsBall) -> Option<DVec2> {
        let wx = self.get_position().x;
        let wy = self.get_position().y;
        let wr = self.get_radius();

        let bx = ball.get_position().x;
        let by = ball.get_position().y;
        let br = ball.get_radius();

        let dx = bx - wx;
        let dy = by - wy;

        let d = (dx.powi(2) + dy.powi(2)).sqrt();

        if d > wr + br || d < (wr - br).abs() {
            None
        } else {
            let a = (wr.powi(2) - br.powi(2) + d.powi(2)) / (2.0 * d);
            let h = (wr.powi(2) - a.powi(2)).sqrt();

            let p2x = wx + a * (bx - wx) / d;
            let p2y = wy + a * (by - wy) / d;

            let ix1 = p2x + h * (by - wy) / d;
            let iy1 = p2y - h * (bx - wx) / d;
            let i1 = dvec2(ix1, iy1);

            let ix2 = p2x - h * (by - wy) / d;
            let iy2 = p2y + h * (bx - wx) / d;
            let i2 = dvec2(ix2, iy2);

            let theta1 =
                ((i1 - self.get_position()).to_angle() as f64 / PI * 180.0 + 360.0) % 360.0;
            let theta2 =
                ((i2 - self.get_position()).to_angle() as f64 / PI * 180.0 + 360.0) % 360.0;

            let theta1_valid = if self.get_start_mod() < self.get_end_mod() {
                (self.get_start_mod()..=self.get_end_mod()).contains(&theta1)
            } else {
                !(self.get_end_mod()..=self.get_start_mod()).contains(&theta1)
            };

            let theta2_valid = if self.get_start_mod() < self.get_end_mod() {
                (self.get_start_mod()..=self.get_end_mod()).contains(&theta2)
            } else {
                !(self.get_end_mod()..=self.get_start_mod()).contains(&theta2)
            };

            Some(match (theta1_valid, theta2_valid) {
                (true, true) => i1.midpoint(i2),
                (true, false) => i1,
                (false, true) => i2,
                (false, false) => return None,
            })
        }
    }

    fn is_goal(&self) -> bool {
        self.is_goal
    }
}

impl Render for CircleWall {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_arc_lines(
            self.get_position(),
            self.get_radius(),
            self.get_start(),
            self.get_end() - self.get_start(),
            2.0,
            Srgba::new(1.0, 1.0, 1.0, 1.0),
        );
    }
}
