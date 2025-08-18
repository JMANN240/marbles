use glam::DVec2;
use palette::Srgba;

use crate::wall::straight_wall::Line;

#[cfg(feature = "macroquad")]
pub mod macroquad;

#[cfg(feature = "image")]
pub mod image;

pub trait Renderer {
    fn render_line(&mut self, line: &Line, thickness: f64, color: Srgba);
    fn render_circle(&mut self, position: DVec2, radius: f64, color: Srgba);
    fn render_circle_lines(&mut self, position: DVec2, radius: f64, thickness: f64, color: Srgba);
    fn render_arc(
        &mut self,
        position: DVec2,
        radius: f64,
        rotation: f64,
        arc: f64,
        thickness: f64,
        color: Srgba,
    );
    fn render_text(
        &mut self,
        text: &str,
        position: DVec2,
        anchor: TextAnchor2D,
        size: f64,
        color: Srgba,
    );
    fn render_rectangle(
        &mut self,
        position: DVec2,
        width: f64,
        height: f64,
        offset: DVec2,
        rotation: f64,
        color: Srgba,
    );
    fn render_rectangle_lines(
        &mut self,
        position: DVec2,
        width: f64,
        height: f64,
        offset: DVec2,
        rotation: f64,
        thickness: f64,
        color: Srgba,
    );
}

pub trait Render {
    fn render(&self, renderer: &mut dyn Renderer);
}

pub struct TextAnchor2D {
    pub horizontal: HorizontalTextAnchor,
    pub vertical: VerticalTextAnchor,
}

pub enum HorizontalTextAnchor {
    Left,
    Center,
    Right,
}

pub enum VerticalTextAnchor {
    Top,
    Center,
    Bottom,
}
