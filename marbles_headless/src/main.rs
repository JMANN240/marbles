use std::{
    collections::HashMap,
    env,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use ab_glyph::FontArc;
use chrono::{Local, TimeZone};
use clap::Parser;
use dotenvy::dotenv;
use glam::DVec2;
use lib::{
    Config, ENGAGEMENTS,
    collision::{Collision, render_collisions},
    posting::{cloudinary::Cloudinary, instagram::InstagramPoster},
    rendering::Render,
    simulation::Simulation,
    util::{
        MaybeMessage, Message, get_formatted_frame_name, get_frame_template, get_scene,
        prepare_images_path, prepare_videos_path, render_video, upload_to_instagram,
        upload_to_youtube,
    },
};
use rand::{rng, seq::IndexedRandom};
use rayon::prelude::*;
use render_agnostic::renderers::image::ImageRenderer;
use reqwest::blocking::Client;
use toml::from_str;
use tracing::{Level, debug, error, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
pub struct Cli {
    #[arg(long, default_value_t = 1)]
    renders: usize,

    #[arg(short, long)]
    instagram: bool,

    #[arg(short, long)]
    youtube: bool,

    #[arg(short, long)]
    consume_message: bool,

    #[arg(long, default_value_t = 3)]
    countdown_seconds: usize,

    #[arg(long, default_value_t = 5)]
    reset_seconds: usize,

    #[arg(long, default_value_t = 1.0)]
    timescale: f64,

    #[arg(long, default_value_t = 100)]
    physics_steps: usize,

    #[arg(long, default_value_t = 0)]
    race_offset: usize,
}

const FRAME_PADDING: usize = 6;

fn main() {
    dotenv().unwrap();
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .pretty()
            .with_max_level(Level::DEBUG)
            .finish(),
    )
    .unwrap();
    let mut rng = rng();
    let cli = Cli::parse();

    let images_path = Path::new("images/headless/");
    let videos_path = Path::new("videos/headless/");

    const WIDTH: u32 = 1080 / 2;
    const HEIGHT: u32 = 1920 / 2;

    let config_string = std::fs::read_to_string("config.toml").unwrap();
    let config = from_str::<Config>(&config_string).unwrap();

    for _ in 0..cli.renders {
        prepare_images_path(images_path).unwrap();
        prepare_videos_path(videos_path).unwrap();

        let scene = get_scene(
            &mut rand::rng(),
            config.get_scene(),
            &config,
            WIDTH as f64,
            HEIGHT as f64,
        );
        let mut collisions: HashMap<usize, Vec<Collision>> = HashMap::new();
        let engagement = ENGAGEMENTS.choose(&mut rng).unwrap();

        let mut query = HashMap::new();

        if cli.consume_message {
            query.insert(
                "consumption_key",
                env::var("CONSUMPTION_KEY")
                    .expect("CONSUMPTION_KEY environment variable is not set"),
            );
        }

        let client = Client::new();

        let special_message_text = client
            .get("https://quantummarbleracing.com/api/next_message")
            .query(&query)
            .send()
            .unwrap()
            .text()
            .unwrap();

        println!("{special_message_text}");

        let special_message = serde_json::from_str::<MaybeMessage>(&special_message_text)
            .unwrap()
            .message
            .unwrap_or(Message {
                message: "Want to reach thousands of    people? Buy a custom message!".to_string(),
                user: "QMR".to_string(),
            });

        let mut simulation = Simulation::new(
            scene,
            1080.0 / 2.0,
            1920.0 / 2.0,
            cli.countdown_seconds as f64,
            cli.reset_seconds as f64,
            engagement.to_string(),
            special_message.message,
            special_message.user,
        );

        let mut simulation_states = Vec::new();

        loop {
            debug!(simulation_time = simulation.get_time());
            let (new_simulation, update_collisions) = simulation.update(1.0 / 60.0, cli.timescale, cli.physics_steps);

            simulation = new_simulation;

            collisions.insert(simulation_states.len(), update_collisions);
            simulation_states.push(simulation.clone());

            if simulation
                .get_maybe_all_won_time()
                .is_some_and(|all_won_time| {
                    simulation.get_time() >= all_won_time + cli.reset_seconds as f64
                })
            {
                break;
            }
        }

        let number_of_frames = simulation_states.len();
        let frames_rendered: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

        simulation_states
            .par_iter()
            .enumerate()
            .for_each(|(frame_number, simulation)| {
                let mut renderer = ImageRenderer::new(WIDTH, HEIGHT, 0.875, DVec2::splat(0.5), 2, FontArc::try_from_slice(include_bytes!("../../roboto.ttf")).unwrap());

                simulation.render(&mut renderer);

                let image = renderer.render_image_onto(renderer.black());
                let image_name = get_formatted_frame_name(FRAME_PADDING, frame_number);

                image.save(images_path.join(image_name)).unwrap();

                let mut frames_rendered = frames_rendered.lock().unwrap();
                *frames_rendered += 1;
                debug!("Rendered {}/{} frames", *frames_rendered, number_of_frames);
            });

        render_collisions(
            "output.wav",
            &collisions,
            Duration::from_secs_f64(300.0),
            44100,
        );

        let video_name = Local::now()
            .format("video_%Y-%m-%d_%H-%M-%S.mp4")
            .to_string();

        let video_path = videos_path.join(video_name);

        info!("Rendering video...");
        let status = render_video(
            &video_path,
            images_path.join(get_frame_template(FRAME_PADDING)),
            "output.wav",
        )
        .expect("Failed to execute ffmpeg");

        if status.success() {
            info!("Video saved as {:?}!", video_path);

            let today = Local::now().date_naive();

            let count = std::fs::read_dir(videos_path)
                .unwrap()
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let metadata = entry.metadata().ok()?;
                    let created = metadata.created().ok()?;
                    let created_date = created
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()
                        .and_then(|d| Local.timestamp_opt(d.as_secs() as i64, 0).single())?
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
                let instagram = InstagramPoster::from_env();

                match upload_to_instagram(
                    cloudinary,
                    instagram,
                    &video_path,
                    "Want to learn how to make and monetize your own simulations? Let me know down in the comments.\n\n#satisfying #marblerace",
                ) {
                    Some(media_publish_response_result) => match media_publish_response_result {
                        Ok(media_publish_response) => info!(?media_publish_response),
                        Err(error) => error!(error),
                    },
                    None => {
                        error!("Could not post to instagram!");
                    }
                }
            }

            if cli.youtube {
                let status = upload_to_youtube(
                    &video_path,
                    &format!("Marble Race {}, {} #satisfying #marblerace", count + cli.race_offset, Local::now().format("%B %-d, %Y")),
                    "Want to learn how to make and monetize your own simulations? Let me know down in the comments.",
                    ["marble racing","marble race","simulation","satisfying"],
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
