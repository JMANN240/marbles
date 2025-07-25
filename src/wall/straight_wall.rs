use macroquad::prelude::*;

use crate::ball::Ball;

use super::Wall;

pub struct StraightWall {
    start: DVec2,
    end: DVec2,
    is_goal: bool,
}

impl StraightWall {
    pub fn new(start: DVec2, end: DVec2, is_goal: bool) -> Self {
        Self {
            start,
            end,
            is_goal,
        }
    }

    pub fn vertical(x: f64, is_goal: bool) -> Self {
        Self::new(dvec2(x, -10000.0), dvec2(x, 10000.0), is_goal)
    }

    pub fn horizontal(y: f64, is_goal: bool) -> Self {
        Self::new(dvec2(-10000.0, y), dvec2(10000.0, y), is_goal)
    }

    pub fn screen() -> Vec<Self> {
        vec![
            Self::new(dvec2(0.0, 0.0), dvec2(screen_width() as f64, 0.0), false),
            Self::new(
                dvec2(screen_width() as f64, 0.0),
                dvec2(screen_width() as f64, screen_height() as f64),
                false,
            ),
            Self::new(
                dvec2(screen_width() as f64, screen_height() as f64),
                dvec2(0.0, screen_height() as f64),
                true,
            ),
            Self::new(dvec2(0.0, screen_height() as f64), dvec2(0.0, 0.0), false),
        ]
    }

    pub fn get_start(&self) -> DVec2 {
        self.start
    }

    pub fn get_end(&self) -> DVec2 {
        self.end
    }
}

impl Wall for StraightWall {
    fn draw(&self) {
        draw_line(
            self.start.x as f32,
            self.start.y as f32,
            self.end.x as f32,
            self.end.y as f32,
            2.0,
            WHITE,
        );
    }

    fn get_intersection_point(&self, ball: &Ball) -> Option<DVec2> {
        let x1 = self.get_start().x;
        let y1 = self.get_start().y;
        let x2 = self.get_end().x;
        let y2 = self.get_end().y;

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
