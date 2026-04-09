use std::{collections::HashMap, fs, path::Path};

use ab_glyph::FontArc;
use dotenvy::dotenv;
use glam::DVec2;
use image::ImageReader;
use lib::{Config, rendering::Render, simulation::Simulation, util::get_scenes};
use render_agnostic::renderers::image::ImageRenderer;
use toml::from_str;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .pretty()
            .with_max_level(Level::DEBUG)
            .finish(),
    )
    .unwrap();

    let scenes_path = Path::new("scenes/");
    fs::create_dir_all(scenes_path).unwrap();

    const WIDTH: u32 = 1080 / 2;
    const HEIGHT: u32 = 1920 / 2;

    let config_string = std::fs::read_to_string("config.toml").unwrap();
    let config = from_str::<Config>(&config_string).unwrap();

    let ball_images = config
        .get_balls()
        .iter()
        .filter_map(|ball_config| {
            ball_config.image.as_ref().map(|image_name| {
                (
                    image_name,
                    ImageReader::open(Path::new("ball_images").join(image_name))
                        .unwrap()
                        .decode()
                        .unwrap()
                        .into_rgba8(),
                )
            })
        })
        .collect::<HashMap<_, _>>();

    for (scene_index, scene) in get_scenes(&mut rand::rng(), &config, WIDTH as f64, HEIGHT as f64)
        .into_iter()
        .enumerate()
    {
        let scene_number = scene_index + 1;

        let simulation = Simulation::new(
            scene,
            (1080.0 / 2.0, 1920.0 / 2.0),
            0.0,
            0.0,
            String::default(),
            String::default(),
            String::default(),
        );

        let mut renderer = ImageRenderer::new(
            WIDTH,
            HEIGHT,
            0.875,
            DVec2::splat(0.5),
            2,
            FontArc::try_from_slice(include_bytes!("../../roboto.ttf")).unwrap(),
        );

        for (image_name, image) in ball_images.iter() {
            renderer.register_image(image_name.to_string(), image.clone());
        }

        simulation.render(&mut renderer);

        let image = renderer.render_image_onto(renderer.black());

        image
            .save(scenes_path.join(format!("scene_{}.png", scene_number)))
            .unwrap();
    }
}
