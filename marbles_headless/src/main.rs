use std::{
    collections::HashMap,
    env, fs,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use ab_glyph::FontArc;
use chrono::{Local, TimeDelta, TimeZone};
use clap::Parser;
use dotenvy::dotenv;
use image::ImageReader;
use lib::{
    Config,
    api::Marble,
    collision::{Collision, render_collisions},
    database::{marble::DbMarble, race::DbRace},
    engagement::get_engagement_for_scene,
    posting::{cloudinary::Cloudinary, instagram::InstagramPoster},
    rendering::Render,
    simulation::Simulation,
    util::{
        MaybeMessage, Message, get_formatted_frame_name, get_frame_template, get_scene,
        render_video, upload_to_instagram, upload_to_youtube,
    },
};
use rand::rngs::SmallRng;
use rayon::prelude::*;
use render_agnostic::renderers::image::ImageRenderer;
use reqwest::blocking::Client;
use sqlx::SqlitePool;
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

    #[arg(short, long)]
    keep_audio: bool,

    #[arg(short, long)]
    keep_frames: bool,

    #[arg(short, long)]
    keep_video: bool,

    #[arg(short, long)]
    stats: bool,
}

const FRAME_PADDING: usize = 6;

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
    let mut rng = rand::make_rng::<SmallRng>();
    let cli = Cli::parse();

    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set"),
    )
    .await
    .unwrap();

    let renders_path = Path::new("renders/headless/");
    fs::create_dir_all(renders_path).unwrap();

    const WIDTH: u32 = 1080 / 2;
    const HEIGHT: u32 = 1920 / 2;

    let config_string = std::fs::read_to_string("config.toml").unwrap();
    let config = from_str::<Config>(&config_string).unwrap();

    let marbles = DbMarble::get_all_active(&pool)
        .await
        .unwrap()
        .into_iter()
        .map(|db_marble| db_marble.into())
        .collect::<Vec<Marble>>();

    for _ in 0..cli.renders {
        let now = Local::now();

        let render_path = renders_path.join(now.format("%Y-%m-%d-%H-%M-%S").to_string());
        fs::create_dir_all(&render_path).unwrap();

        let frames_path = render_path.join("frames/");
        fs::create_dir_all(&frames_path).unwrap();

        let scene = get_scene(
            &mut rng,
            config.get_scene(),
            &marbles,
            WIDTH as f64,
            HEIGHT as f64,
        );
        let mut collisions: HashMap<usize, Vec<Collision>> = HashMap::new();
        let engagement = get_engagement_for_scene(&pool, &mut rng, &scene)
            .await
            .unwrap();

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
                message: "Want to reach THOUSANDS of people  for just $1? Buy a custom message!"
                    .to_string(),
                user: "QMR".to_string(),
            });

        let mut simulation = Simulation::new(
            scene,
            (1080.0 / 2.0, 1920.0 / 2.0),
            cli.countdown_seconds as f64,
            cli.reset_seconds as f64,
            textwrap::fill(&engagement, 20),
            special_message.message,
            special_message.user,
        );

        let mut simulation_states = Vec::new();

        loop {
            debug!(simulation_time = simulation.get_time());
            let (new_simulation, update_collisions) =
                simulation.update(1.0 / 60.0, cli.timescale, cli.physics_steps);

            simulation = new_simulation;

            collisions.insert(simulation_states.len(), update_collisions);
            simulation_states.push(simulation.clone());

            if simulation.is_finished() {
                if cli.stats {
                    let race = DbRace::insert(&pool, now.timestamp())
                        .await
                        .expect("Could not insert race into database");

                    for (winner_index, win_time) in simulation
                        .get_scene()
                        .get_winners()
                        .iter()
                        .zip(simulation.get_scene().get_win_times())
                    {
                        let winner = simulation
                            .get_scene()
                            .get_balls()
                            .get(*winner_index)
                            .unwrap();

                        race.insert_participant(
                            &pool,
                            winner.get_name().to_string(),
                            TimeDelta::from_std(*win_time).unwrap(),
                        )
                        .await
                        .expect("Could not insert race participant into database");
                    }
                }

                break;
            }
        }

        let number_of_frames = simulation_states.len();
        let frames_rendered: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

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

        simulation_states
            .par_iter()
            .enumerate()
            .for_each(|(frame_number, simulation)| {
                let t = frame_number as f64 / 60.0;

                let mut renderer = ImageRenderer::new(
                    WIDTH,
                    HEIGHT,
                    simulation.zoom(t),
                    simulation.focus(t),
                    2,
                    FontArc::try_from_slice(include_bytes!("../../roboto.ttf")).unwrap(),
                );

                for (image_name, image) in ball_images.iter() {
                    renderer
                        .register_image(image_name.to_str().unwrap().to_string(), image.clone());
                }

                simulation.render(&mut renderer);

                let image = renderer.render_image_onto(renderer.black());
                let image_name = get_formatted_frame_name(FRAME_PADDING, frame_number);

                image.save(frames_path.join(image_name)).unwrap();

                let mut frames_rendered = frames_rendered.lock().unwrap();
                *frames_rendered += 1;
                debug!("Rendered {}/{} frames", *frames_rendered, number_of_frames);
            });

        let audio_path = render_path.join("audio.wav");

        render_collisions(
            &audio_path,
            &collisions,
            Duration::from_secs_f64(300.0),
            44100,
        );

        let video_name = Local::now().format("video.mp4").to_string();

        let video_path = render_path.join(video_name);

        info!("Rendering video...");
        let status = render_video(
            &video_path,
            frames_path.join(get_frame_template(FRAME_PADDING)),
            &audio_path,
        )
        .expect("Failed to execute ffmpeg");

        if !cli.keep_audio && audio_path.exists() {
            fs::remove_file(audio_path).expect("Could not delete audio");
        }

        if !cli.keep_frames && frames_path.exists() {
            fs::remove_dir_all(frames_path).expect("Could not delete frames");
        }

        if status.success() {
            info!("Video saved as {:?}!", video_path);

            let today = Local::now().date_naive();

            let count = std::fs::read_dir(renders_path)
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
                    "Want to learn how to make and monetize your own simulations? Check the link in my bio!\n\n#satisfying #marblerace",
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
                    "Want to learn how to make and monetize your own simulations? Check the link in my bio!",
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

        if !cli.keep_video && video_path.exists() {
            fs::remove_file(video_path).expect("Could not delete video");
        }

        if !cli.keep_audio && !cli.keep_frames && !cli.keep_video {
            fs::remove_dir_all(render_path).expect("Could not delete render directory");
        }
    }
}
