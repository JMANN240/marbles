use macroquad::prelude::*;

pub struct Wall {
    start: DVec2,
    end: DVec2,
    is_goal: bool,
}

impl Wall {
    pub fn new(start: DVec2, end: DVec2, is_goal: bool) -> Self {
        Self { start, end, is_goal }
    }

    pub fn vertical(x: f64, is_goal: bool) -> Self {
        Self::new(dvec2(x, -10000.0), dvec2(x, 10000.0), is_goal)
    }

    pub fn horizontal(y: f64, is_goal: bool) -> Self {
        Self::new(dvec2(-10000.0, y), dvec2(10000.0, y), is_goal)
    }

    pub fn get_start(&self) -> DVec2 {
        self.start
    }

    pub fn get_end(&self) -> DVec2 {
        self.end
    }

    pub fn is_goal(&self) -> bool {
        self.is_goal
    }

    pub fn draw(&self) {
        draw_line(
            self.start.x as f32,
            self.start.y as f32,
            self.end.x as f32,
            self.end.y as f32,
            2.0,
            WHITE,
        );
    }
}
