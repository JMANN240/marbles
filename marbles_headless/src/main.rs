use std::{collections::HashMap, path::Path, sync::{Arc, Mutex}};

use chrono::{Local, TimeZone};
use clap::Parser;
use dotenvy::dotenv;
use image::imageops::{resize, FilterType};
use lib::{collision::{render_collisions, Collision}, posting::{cloudinary::Cloudinary, instagram::InstagramPoster}, rendering::{image::ImageRenderer, Render}, scenes::{scene_1, scene_2, scene_3, scene_4, scene_5, scene_6, scene_7}, simulation::Simulation, util::{render_video, upload_to_youtube}, Config, ENGAGEMENTS};
use rand::{rng, seq::IndexedRandom};
use rayon::prelude::*;
use toml::from_str;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
pub struct Cli {
    #[arg(long, default_value_t = 1)]
    renders: usize,

    #[arg(short, long)]
    instagram: bool,

    #[arg(short, long)]
    youtube: bool,

    #[arg(long, default_value_t = 3)]
    countdown_seconds: usize,

    #[arg(long, default_value_t = 10)]
    reset_seconds: usize,

    #[arg(long, default_value_t = 1.0)]
    timescale: f64,

    #[arg(long, default_value_t = 100)]
    physics_steps: usize,

    #[arg(long, default_value_t = 0)]
    race_offset: usize,
}

fn main() {
    dotenv().unwrap();
    tracing::subscriber::set_global_default(FmtSubscriber::builder().pretty().with_max_level(Level::DEBUG).finish()).unwrap();
    let mut rng = rng();
    let cli = Cli::parse();

    const WIDTH: u32 = 1080 / 2;
    const HEIGHT: u32 = 1920 / 2;

    let mut render_number = 0;

    while render_number < cli.renders {
        render_number += 1;

        let images_path = Path::new("images");

        if images_path.exists() {
            info!("Clearing previous frames!");
            std::fs::remove_dir_all(images_path).unwrap();
        }

        std::fs::create_dir(images_path).unwrap();

        let config_string = std::fs::read_to_string("config.toml").unwrap();
        let config = from_str::<Config>(&config_string).unwrap();

        let mut scenes = vec![
            scene_1(
                config.get_balls().clone(),
                WIDTH as f64,
                HEIGHT as f64,
            ),
            scene_2(
                config.get_balls().clone(),
                WIDTH as f64,
                HEIGHT as f64,
            ),
            scene_3(
                config.get_balls().clone(),
                WIDTH as f64,
                HEIGHT as f64,
            ),
            scene_4(
                config.get_balls().clone(),
                WIDTH as f64,
                HEIGHT as f64,
            ),
            scene_5(
                config.get_balls().clone(),
                WIDTH as f64,
                HEIGHT as f64,
            ),
            scene_6(
                config.get_balls().clone(),
                WIDTH as f64,
                HEIGHT as f64,
            ),
            scene_7(WIDTH as f64, HEIGHT as f64),
        ];

        let scene = scenes.remove(config.get_scene() - 1);
        let mut collisions: HashMap<usize, Vec<Collision>> = HashMap::new();
        let engagement = ENGAGEMENTS.choose(&mut rng).unwrap();
        let mut maybe_all_won_time = None;

        let mut simulation = Simulation::new(
            scene,
            1080.0 / 2.0,
            1920.0 / 2.0,
            cli.countdown_seconds as f64,
            cli.reset_seconds as f64,
            engagement.to_string(),
        );

        let mut simulation_states = Vec::new();

        loop {
            debug!(simulation_time=simulation.get_time());
            let update_collisions = simulation.update(1.0 / 60.0, cli.timescale, cli.physics_steps);

            collisions.insert(simulation_states.len(), update_collisions);
            simulation_states.push(simulation.clone());

            if simulation.get_scene().get_winners().len()
                == simulation.get_scene().get_balls().len()
                && maybe_all_won_time.is_none()
            {
                maybe_all_won_time = Some(simulation.get_time());
            }

            if let Some(all_won_time) = maybe_all_won_time
                && simulation.get_time() >= all_won_time + cli.reset_seconds as f64
            {
                break;
            }
        }

        let number_of_frames = simulation_states.len();
        let frames_rendered: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

        simulation_states.par_iter().enumerate().for_each(|(frame_number, simulation)| {
            let mut renderer = ImageRenderer::new(WIDTH, HEIGHT, 0.875);
            simulation.render(&mut renderer);
            let image = renderer.get_image();
            image.save(&format!("images/frame_{:06}.png", frame_number)).unwrap();
            let mut frames_rendered_lock = frames_rendered.lock().unwrap();
            *frames_rendered_lock += 1;
            debug!("Rendered {}/{} frames", *frames_rendered_lock, number_of_frames);
        });

        render_collisions(&collisions, 300.0, 44100);

        let video_filename = Local::now()
            .format("videos/video_%Y-%m-%d_%H-%M-%S.mp4")
            .to_string();

        info!("Rendering video...");
        let status = render_video(&video_filename, "images/frame_%06d.png", "output.wav")
            .expect("Failed to execute ffmpeg");

        if status.success() {
            info!("Video saved as {}!", video_filename);

            let today = Local::now().date_naive();

            let count = std::fs::read_dir("videos")
                .unwrap()
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let metadata = entry.metadata().ok()?;
                    let created = metadata.created().ok()?;
                    let created_date = created
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()
                        .map(|d| Local.timestamp_opt(d.as_secs() as i64, 0).single())
                        .flatten()?
                        .date_naive();

                    if created_date == today {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();

            if cli.instagram {
                let cloudinary = Cloudinary::from_env();

                let cloudinary_response = cloudinary.post(&video_filename).unwrap();

                let instagram = InstagramPoster::from_env();

                instagram.post(
                    "Want to learn how to make and monetize your own simulations? Let me know down in the comments.\n\n#satisfying #marblerace",
                    &cloudinary_response.secure_url,
                    (cloudinary_response.duration * 0.4 * 1000.0).floor(),
                ).unwrap();

                cloudinary.delete(&cloudinary_response.public_id).unwrap();
            }

            if cli.youtube {
                let status = upload_to_youtube(
                    &video_filename,
                    &format!("Marble Race {}, {} #satisfying #marblerace", count + cli.race_offset, Local::now().format("%B %-d, %Y").to_string()),
                    "Want to learn how to make and monetize your own simulations? Let me know down in the comments.",
                    "marble racing,marble race,simulation,satisfying",
                )
                    .expect("Failed to upload to YouTube");

                if status.success() {
                    info!("Video uploaded to YouTube!");
                } else {
                    info!("YouTube upload failed!");
                }
            }
        } else {
            info!("Rendering failed!");
        }
    }
}
