use std::f64::consts::TAU;

use glam::DVec2;
use palette::{FromColor, Oklcha, Srgba};
use rand::random_range;
use render_agnostic::Renderer;

use crate::{
    ball::Ball,
    powerup::Powerup,
    rendering::Render,
};

pub struct SpecialConfig {
    radius: f64,
}

impl SpecialConfig {
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
        }
    }

    pub fn build(&self, position: DVec2) -> Special {
        Special::new(
            position,
            self.radius,
        )
    }
}

#[derive(Clone)]
pub struct Special {
    time: f64,
    position: DVec2,
    radius: f64,
    is_active: bool,
    last_color_change_time: Option<f64>,
    color: Srgba,
}

impl Special {
    pub fn new(
        position: DVec2,
        radius: f64,
    ) -> Self {
        Self {
            time: 0.0,
            position,
            radius,
            is_active: true,
            last_color_change_time: None,
            color: Srgba::from_color(Oklcha::new(1.0, 0.5, random_range(0.0..360.0), 1.0)),
        }
    }

    pub fn get_position(&self) -> DVec2 {
        self.position + 2.0 * (self.time * 4.0).sin() * DVec2::Y
    }
}

impl Powerup for Special {
    fn is_colliding_with(&self, ball: &Ball) -> bool {
        self.get_position().distance(ball.get_position()) < self.radius + ball.get_radius()
    }

    fn apply(&self, _ball: &mut Ball) {}

    fn consume(&mut self) {
        self.is_active = false;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn update(&self, dt: f64) -> Box<dyn Powerup> {
        let mut new_powerup = self.clone();

        new_powerup.time += dt;

        if new_powerup.last_color_change_time.is_none_or(|last_color_change_time| new_powerup.time > last_color_change_time + 0.1) {
            new_powerup.color = Srgba::from_color(Oklcha::new(1.0, 0.5, random_range(0.0..360.0), 1.0));
            new_powerup.last_color_change_time = Some(new_powerup.time);
        }

        Box::new(new_powerup)
    }
}

impl Render for Special {
    fn render(&self, renderer: &mut dyn Renderer) {
        if self.is_active {
            renderer.render_circle_lines(self.get_position(), 8.0, 1.0, self.color);

            renderer.render_text_outline(
                "Special",
                self.get_position() - DVec2::Y * 2.0 * self.radius,
                anchor2d::CGB,
                20.0,
                1.0,
                self.color,
                Srgba::new(0.0, 0.0, 0.0, 1.0),
            );

            for i in 0..16 {
                let start = self.get_position() + DVec2::from_angle(self.time * 2.123123 + TAU * i as f64 / 16.0) * self.radius * (1.0 + (self.time * 4.123123).cos() / 2.0);
                let end = self.get_position() + DVec2::from_angle(self.time * 3.456456 + TAU * i as f64 / 16.0) * self.radius * (1.0 - (self.time * 5.456456).cos() / 2.0);
                renderer.render_line(start, end, 1.0, self.color);
            }
        }
    }
}
