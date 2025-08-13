use std::{collections::HashMap, fs, path::Path};

use ::rand::{rng, seq::IndexedRandom};
use chrono::{Local, TimeZone};
use clap::Parser;
use dotenvy::dotenv;
use lib::collision::{Collision, render_collisions};
use lib::rendering::Render;
use lib::rendering::macroquad::MacroquadRenderer;
use lib::scenes::{scene_1, scene_2, scene_3, scene_4, scene_5, scene_6, scene_7};
use lib::simulation::Simulation;
use lib::util::{render_video, upload_to_youtube};
use lib::{Config, ENGAGEMENTS};
use macroquad::audio::{PlaySoundParams, load_sound, play_sound};
use macroquad::prelude::*;
use toml::from_str;
use tracing_subscriber::FmtSubscriber;

use lib::posting::{cloudinary::Cloudinary, instagram::InstagramPoster};

use crate::util::draw_text_outline;

mod util;

const SCALE: f32 = 0.5;

fn window_conf() -> Conf {
    Conf {
        window_width: (1080.0 * SCALE) as i32,
        window_height: (1920.0 * SCALE) as i32,
        window_title: "Marbles".to_owned(),
        sample_count: 8,
        ..Default::default()
    }
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

#[macroquad::main(window_conf)]
async fn main() {
    dotenv().unwrap();
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();
    let mut rng = rng();
    let cli = Cli::parse();

    let mut renderer = MacroquadRenderer::new("roboto.ttf").await;

    let mut render_number = 0;
    let mut time_offset = 0.0;

    let zoom = 1.125;

    let mut sounds = HashMap::new();
    sounds.insert("piano_c6.wav", load_sound("piano_c6.wav").await.unwrap());
    sounds.insert("piano_e6.wav", load_sound("piano_e6.wav").await.unwrap());
    sounds.insert("piano_g6.wav", load_sound("piano_g6.wav").await.unwrap());
    sounds.insert("piano_c7.wav", load_sound("piano_c7.wav").await.unwrap());

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
            scene_1(
                config.get_balls().clone(),
                screen_width() as f64,
                screen_height() as f64,
            ),
            scene_2(
                config.get_balls().clone(),
                screen_width() as f64,
                screen_height() as f64,
            ),
            scene_3(
                config.get_balls().clone(),
                screen_width() as f64,
                screen_height() as f64,
            ),
            scene_4(
                config.get_balls().clone(),
                screen_width() as f64,
                screen_height() as f64,
            ),
            scene_5(
                config.get_balls().clone(),
                screen_width() as f64,
                screen_height() as f64,
            ),
            scene_6(
                config.get_balls().clone(),
                screen_width() as f64,
                screen_height() as f64,
            ),
            scene_7(screen_width() as f64, screen_height() as f64),
        ];

        let scene = scenes.remove(config.get_scene() - 1);
        let mut frame_number = 0;
        let mut collisions: HashMap<usize, Vec<Collision>> = HashMap::new();
        let engagement = ENGAGEMENTS.choose(&mut rng).unwrap();
        let mut maybe_all_won_time = None;

        let mut simulation = Simulation::new(
            scene,
            screen_width() as f64,
            screen_height() as f64,
            cli.countdown_seconds as f64,
            cli.reset_seconds as f64,
            engagement.to_string(),
        );

        loop {
            let camera = Camera2D {
                zoom: vec2(2.0 / (1080.0 * SCALE * zoom), 2.0 / (1920.0 * SCALE * zoom)),
                target: vec2(screen_width() / 2.0, screen_height() / 2.0),
                ..Camera2D::default()
            };

            set_camera(&camera);

            let update_collisions =
                simulation.update(get_frame_time() as f64, cli.timescale, cli.physics_steps);

            for collision in update_collisions.iter() {
                play_sound(
                    sounds.get(collision.sound_path.to_str().unwrap()).unwrap(),
                    PlaySoundParams {
                        looped: false,
                        volume: collision.volume,
                    },
                );
            }

            collisions.insert(frame_number, update_collisions);

            clear_background(BLACK);

            simulation.render(&mut renderer);

            if simulation.get_scene().get_winners().len()
                == simulation.get_scene().get_balls().len()
                && maybe_all_won_time.is_none()
            {
                maybe_all_won_time = Some(simulation.get_time());
            }

            if let Some(all_won_time) = maybe_all_won_time {
                if cli.endless {
                    let text = format!(
                        "{}",
                        cli.reset_seconds as f64 - (simulation.get_time() - all_won_time).floor()
                    );

                    draw_text_outline(
                        &text,
                        screen_width() / 2.0 - measure_text(&text, None, 196, 1.0).width / 2.0,
                        screen_height() / 2.0,
                        196.0,
                        WHITE,
                        BLACK,
                    );
                }

                if (cli.endless || cli.render)
                    && simulation.get_time() >= all_won_time + cli.reset_seconds as f64
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
            let status = render_video(&video_filename, "images/frame_%06d.png", "output.wav")
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

        if cli.endless || render_number < cli.renders {
            time_offset = get_time();
        } else {
            break;
        }
    }
}
