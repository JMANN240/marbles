use std::{collections::HashMap, env, fs, path::Path};

use ab_glyph::FontArc;
use api::marble::Marble;
use database::marble::DbMarble;
use dotenvy::dotenv;
use glam::DVec2;
use image::ImageReader;
use lib::{rendering::Render, simulation::Simulation, util::get_scenes};
use render_agnostic::{image_registries::image_image_registry::ImageImageRegistry, renderers::image::ImageRenderer};
use sqlx::SqlitePool;
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

    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set"),
    )
    .await
    .unwrap();

    let scenes_path = Path::new("scenes/");
    fs::create_dir_all(scenes_path).unwrap();

    const WIDTH: u32 = 1080 / 2;
    const HEIGHT: u32 = 1920 / 2;

    let marbles = DbMarble::get_all_active(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(Marble::from)
        .collect::<Vec<Marble>>();

    let ball_images = marbles
        .iter()
        .filter_map(|marble| {
            marble.maybe_image_path.as_ref().map(|image_name| {
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

    for (scene_index, scene) in get_scenes(&mut rand::rng(), &marbles, WIDTH as f64, HEIGHT as f64)
        .into_iter()
        .enumerate()
    {
        let scene_number = scene_index + 1;

        let simulation = Simulation::new(
            scene,
            (1080.0 / 2.0, 1920.0 / 2.0),
            0.0,
            0.0,
            Vec::default(),
        );

        let mut image_registry = ImageImageRegistry::default();

        for (image_name, image) in ball_images.iter() {
            image_registry.register_image(image_name.to_str().unwrap().to_string(), image.clone());
        }

        let mut renderer = ImageRenderer::new(
            WIDTH,
            HEIGHT,
            0.875,
            DVec2::splat(0.5),
            2,
            FontArc::try_from_slice(include_bytes!("../../roboto.ttf")).unwrap(),
            image_registry,
        );

        simulation.render(&mut renderer);

        let image = renderer.render_image_onto(renderer.black());

        image
            .save(scenes_path.join(format!("scene_{}.png", scene_number)))
            .unwrap();
    }
}
