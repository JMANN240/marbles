use macroquad::prelude::*;

use crate::ball::Ball;

use super::Wall;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    start: DVec2,
    end: DVec2,
}

impl Line {
    pub fn new(start: DVec2, end: DVec2) -> Self {
        Self { start, end }
    }

    pub fn get_start(&self) -> DVec2 {
        self.start
    }

    pub fn get_end(&self) -> DVec2 {
        self.end
    }

    fn get_vector(&self) -> DVec2 {
        self.get_end() - self.get_start()
    }

    pub fn get_t(&self, point: DVec2) -> f64 {
        (point - self.get_start())
            .project_onto(self.get_vector())
            .length()
            / self.get_vector().length()
    }

    pub fn get_point(&self, t: f64) -> DVec2 {
        self.get_start() + self.get_vector() * t
    }
}

pub struct StraightWall {
    line: Line,
    is_goal: bool,
}

impl StraightWall {
    pub fn new(line: Line, is_goal: bool) -> Self {
        Self { line, is_goal }
    }

    pub fn vertical(x: f64, is_goal: bool) -> Self {
        Self::new(Line::new(dvec2(x, -10000.0), dvec2(x, 10000.0)), is_goal)
    }

    pub fn horizontal(y: f64, is_goal: bool) -> Self {
        Self::new(Line::new(dvec2(-10000.0, y), dvec2(10000.0, y)), is_goal)
    }

    pub fn screen(with_goal: bool) -> Vec<Self> {
        vec![
            Self::new(
                Line::new(dvec2(0.0, 0.0), dvec2(screen_width() as f64, 0.0)),
                false,
            ),
            Self::new(
                Line::new(
                    dvec2(screen_width() as f64, 0.0),
                    dvec2(screen_width() as f64, screen_height() as f64),
                ),
                false,
            ),
            Self::new(
                Line::new(
                    dvec2(screen_width() as f64, screen_height() as f64),
                    dvec2(0.0, screen_height() as f64),
                ),
                with_goal,
            ),
            Self::new(
                Line::new(dvec2(0.0, screen_height() as f64), dvec2(0.0, 0.0)),
                false,
            ),
        ]
    }

    pub fn get_line(&self) -> Line {
        self.line
    }
}

impl Wall for StraightWall {
    fn update(&mut self, _dt: f64) {}

    fn draw(&self) {
        draw_line(
            self.get_line().get_start().x as f32,
            self.get_line().get_start().y as f32,
            self.get_line().get_end().x as f32,
            self.get_line().get_end().y as f32,
            2.0,
            WHITE,
        );
    }

    fn get_intersection_point(&self, ball: &Ball) -> Option<DVec2> {
        let x1 = self.get_line().get_start().x;
        let y1 = self.get_line().get_start().y;
        let x2 = self.get_line().get_end().x;
        let y2 = self.get_line().get_end().y;

        let dx = x2 - x1;
        let dy = y2 - y1;

        let fx = x1 - ball.get_position().x;
        let fy = y1 - ball.get_position().y;

        let a = dx * dx + dy * dy;
        let b = 2.0 * (fx * dx + fy * dy);
        let c = fx * fx + fy * fy - ball.get_radius() * ball.get_radius();

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

    fn is_goal(&self) -> bool {
        self.is_goal
    }
}
