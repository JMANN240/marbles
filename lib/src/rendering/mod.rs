use ::image::DynamicImage;

use crate::{scene::Scene, simulation::Simulation};

#[cfg(feature = "image-rendering")]
pub mod image;
pub mod macroquad;

pub trait Renderer {
    fn render_simulation(&self, simulation: &Simulation) -> DynamicImage;
    fn render_scene(&self, scene: &Scene) -> DynamicImage;
}
