use std::{cmp::Ordering, collections::HashMap, env, fs, path::Path};

use ::rand::{random_range, rng, seq::IndexedRandom};
use chrono::{Local, TimeZone};
use clap::Parser;
use collision::{Collision, render_collisions};
use dotenvy::dotenv;
use macroquad::prelude::*;
use scenes::{scene_1, scene_2, scene_3, scene_4, scene_5, scene_6, scene_7};
use serde::Deserialize;
use toml::from_str;
use tracing_subscriber::FmtSubscriber;
use util::draw_text_outline;

use crate::posting::{cloudinary::Cloudinary, instagram::InstagramPoster};

mod ball;
mod collision;
mod drawer;
mod particle;
mod posting;
mod scene;
mod scenes;
mod util;
mod wall;

const SCALE: f32 = 0.5;

fn window_conf() -> Conf {
    Conf {
        window_width: (1080.0 * SCALE) as i32,
        window_height: (1920.0 * SCALE) as i32,
        window_title: "BallRace".to_owned(),
        sample_count: 8,
        ..Default::default()
    }
}

fn _window_conf_square() -> Conf {
    Conf {
        window_width: (1024.0 * SCALE) as i32,
        window_height: (1024.0 * SCALE) as i32,
        window_title: "BallRace".to_owned(),
        sample_count: 8,
        ..Default::default()
    }
}

#[derive(Deserialize, Clone)]
pub struct BallConfig {
    name: String,
    r: f32,
    g: f32,
    b: f32,
    radius: f64,
    elasticity: f64,
    sound: String,
}

#[derive(Deserialize)]
pub struct Config {
    balls: Vec<BallConfig>,
    scene: usize,
}

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    render: bool,

    #[arg(long, default_value_t = 1)]
    renders: usize,

    #[arg(short, long)]
    instagram: bool,

    #[arg(short, long)]
    youtube: bool,

    #[arg(short, long)]
    endless: bool,

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

const ENGAGEMENTS: [&str; 4] = [
    "Pick one!",
    "Choose a winner!",
    "Who will win?",
    "Choose one!",
];

#[macroquad::main(window_conf)]
async fn main() {
    dotenv().unwrap();
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();
    let mut rng = rng();
    let cli = Cli::parse();

    let mut render_number = 0;
    let mut time_offset = 0.0;

    let zoom = 1.125;

    let goal = dvec2(
        screen_width() as f64 / 2.0,
        400.0 + screen_width() as f64 / 2.0 - 9.0,
    );

    loop {
        render_number += 1;
        let mut render_time = 0.0;

        if cli.render {
            let images_path = Path::new("images");

            if images_path.exists() {
                info!("Clearing previous frames!");
                std::fs::remove_dir_all(images_path).unwrap();
            }

            std::fs::create_dir(images_path).unwrap();
        }

        let config_string = std::fs::read_to_string("config.toml").unwrap();
        let config = from_str::<Config>(&config_string).unwrap();

        let mut scenes = vec![
            scene_1(config.balls.clone()).await,
            scene_2(config.balls.clone()).await,
            scene_3(config.balls.clone()).await,
            scene_4(config.balls.clone()).await,
            scene_5(config.balls.clone()).await,
            scene_6(config.balls.clone()).await,
            scene_7().await,
        ];

        let scene = scenes.get_mut(config.scene - 1).unwrap();
        let mut frame_number = 0;
        let mut collisions: HashMap<usize, Vec<Collision>> = HashMap::new();
        let engagement = ENGAGEMENTS.choose(&mut rng).unwrap();
        let mut maybe_all_won_time = None;

        loop {
            let scene_time = if cli.render {
                render_time
            } else {
                get_time() - time_offset
            };

            let closest_ball_to_goal = scene
                .get_balls()
                .iter()
                .min_by(|l, r| {
                    let left_distance = goal.distance(l.get_position());
                    let right_distance = goal.distance(r.get_position());

                    if left_distance < right_distance {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap();

            let _smallest_distance_to_goal = closest_ball_to_goal.get_position().distance(goal);

            let bullet_time: f64 = 0.0; // 1.0 - (smallest_distance_to_goal / 200.0).min(1.0);

            let camera = Camera2D {
                zoom: vec2(
                    (2.0 + bullet_time.powi(2) as f32 * 8.0) / (1080.0 * SCALE * zoom),
                    (2.0 + bullet_time.powi(2) as f32 * 8.0) / (1920.0 * SCALE * zoom),
                ),
                offset: vec2(0.0, 0.0 - goal.y as f32 / screen_height() / 2.0),
                target: goal.as_vec2(),
                rotation: random_range((-1.0 * bullet_time)..=(1.0 * bullet_time)) as f32,
                ..Camera2D::default()
            };

            set_camera(&camera);

            if scene_time >= cli.countdown_seconds as f64 {
                let update_collisions = scene.update(
                    cli.timescale.min(0.1 + 1.0 - bullet_time),
                    cli.physics_steps,
                );
                collisions.insert(frame_number, update_collisions);
            }

            scene.draw();

            if scene_time.floor() < cli.countdown_seconds as f64 {
                let text = format!("{}", cli.countdown_seconds as f64 - scene_time.floor());
                draw_text_outline(
                    &text,
                    screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                    screen_height() / 2.0,
                    256.0,
                    WHITE,
                    BLACK,
                );

                if (scene_time * 2.0 + 1.5).floor() % 2.0 == 0.0 {
                    draw_text_outline(
                        engagement,
                        screen_width() / 2.0 - measure_text(engagement, None, 64, 1.0).width / 2.0,
                        screen_height() / 2.0 + 100.0,
                        64.0,
                        WHITE,
                        BLACK,
                    );
                }
            } else if scene_time.floor() < (cli.countdown_seconds + 1) as f64 {
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

            if scene.get_winners().len() == scene.get_balls().len() && maybe_all_won_time.is_none()
            {
                maybe_all_won_time = Some(scene_time);
            }

            if let Some(all_won_time) = maybe_all_won_time {
                if cli.endless {
                    let text = format!(
                        "{}",
                        cli.reset_seconds as f64 - (scene_time - all_won_time).floor()
                    );

                    draw_text_outline(
                        &text,
                        screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                        screen_height() / 2.0,
                        256.0,
                        WHITE,
                        BLACK,
                    );
                }

                if (cli.endless || cli.render)
                    && scene_time >= all_won_time + cli.reset_seconds as f64
                {
                    break;
                }
            }

            if cli.render {
                let screen_data = get_screen_data();
                screen_data.export_png(&format!("images/frame_{:06}.png", frame_number));
                frame_number += 1;
                render_time += 1.0 / 60.0;
            }

            next_frame().await;
        }

        if cli.render {
            render_collisions(&collisions, 300.0, 44100);

            let video_filename = Local::now()
                .format("videos/video_%Y-%m-%d_%H-%M-%S.mp4")
                .to_string();

            info!("Rendering video...");
            let status = std::process::Command::new("ffmpeg")
                .args([
                    "-framerate",
                    "60",
                    "-i",
                    "images/frame_%06d.png",
                    "-i",
                    "output.wav",
                    "-c:v",
                    "libx264",
                    "-crf",
                    "18",
                    "-preset",
                    "slow",
                    "-pix_fmt",
                    "yuv420p",
                    "-c:a",
                    "aac",
                    "-shortest",
                    &video_filename,
                ])
                .status()
                .expect("Failed to execute ffmpeg");

            if status.success() {
                info!("Video saved as {}!", video_filename);

                let today = Local::now().date_naive();

                let count = fs::read_dir("videos")
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
                    let cloudinary = Cloudinary::new(
                        env::var("CLOUDINARY_CLOUD_NAME")
                            .expect("Missing CLOUDINARY_CLOUD_NAME environment variable!"),
                        env::var("CLOUDINARY_API_KEY")
                            .expect("Missing CLOUDINARY_API_KEY environment variable!"),
                        env::var("CLOUDINARY_API_SECRET")
                            .expect("Missing CLOUDINARY_API_SECRET environment variable!"),
                    );

                    let cloudinary_response = cloudinary.post(&video_filename).await.unwrap();

                    let instagram = InstagramPoster::new(
                        env::var("INSTAGRAM_SCOPED_ACCOUNT_ID")
                            .expect("Missing INSTAGRAM_SCOPED_ACCOUNT_ID environment variable!"),
                        env::var("INSTAGRAM_USER_ACCESS_TOKEN")
                            .expect("Missing INSTAGRAM_USER_ACCESS_TOKEN environment variable!"),
                    );

                    instagram.post(
                        "Want to learn how to make and monetize your own simulations? Let me know down in the comments.\n\n#satisfying #marblerace",
                        &cloudinary_response.secure_url,
                        (cloudinary_response.duration * 0.4 * 1000.0).floor(),
                    ).await.unwrap();

                    cloudinary.delete(&cloudinary_response.public_id).unwrap();
                }

                if cli.youtube {
                    let status = std::process::Command::new("python3")
                        .args([
                            "youtube_uploader.py",
                            "--path",
                            &video_filename,
                            "--title",
                            &format!("Marble Race {}, {} #satisfying #marblerace", count + cli.race_offset, Local::now().format("%B %-d, %Y").to_string()),
                            "--description",
                            "Want to learn how to make and monetize your own simulations? Let me know down in the comments.",
                            "--tags",
                            "marble racing,marble race,simulation,satisfying",
                        ])
                        .status()
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

        if cli.endless || render_number < cli.renders {
            time_offset = get_time();
        } else {
            break;
        }
    }
}
