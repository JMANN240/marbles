use image::{DynamicImage, RgbImage};

use crate::{rendering::Renderer, scene::Scene, simulation::Simulation, SCALE};

pub struct ImageRenderer;

impl Renderer for ImageRenderer {
    fn render_simulation(&self, simulation: &Simulation) -> DynamicImage {
        let scene_image = self.render_scene(simulation.get_scene());

        if self.get_time().floor() < self.countdown_seconds {
            let text = format!(
                "{}",
                self.countdown_seconds - self.get_time().floor()
            );
            draw_text_outline(
                &text,
                screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                screen_height() / 2.0,
                256.0,
                WHITE,
                BLACK,
            );

            if (self.get_time() * 2.0 + 1.5).floor() % 2.0 == 0.0 {
                draw_text_outline(
                    &self.engagement,
                    screen_width() / 2.0 - measure_text(&self.engagement, None, 64, 1.0).width / 2.0,
                    screen_height() / 2.0 + 100.0,
                    64.0,
                    WHITE,
                    BLACK,
                );
            }
        } else if self.get_time().floor() < self.countdown_seconds + 1.0 {
            let text = "Go!";
            draw_text_outline(
                text,
                screen_width() / 2.0 - measure_text(text, None, 256, 1.0).width / 2.0,
                screen_height() / 2.0,
                256.0,
                WHITE,
                BLACK,
            );
        }

        scene_image
    }
    
    fn render_scene(&self, scene: &Scene) -> DynamicImage {
        let mut image = RgbImage::new((1080.0 * SCALE) as u32, (1920.0 * SCALE) as u32);
    
        for wall in scene.get_walls() {
            image = wall.render_to_image(&image);
        }
    
        DynamicImage::ImageRgb8(image)
    }
}