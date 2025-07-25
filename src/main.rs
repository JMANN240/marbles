use std::{collections::HashMap, path::Path};

use ::rand::{rng, seq::IndexedRandom};
use chrono::Local;
use clap::Parser;
use collision::{Collision, render_collisions};
use macroquad::prelude::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use scenes::{scene_1, scene_2, scene_3, scene_4, scene_5, scene_6};
use serde::Deserialize;
use toml::from_str;
use util::draw_text_outline;

mod ball;
mod collision;
mod drawer;
mod particle;
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

#[derive(Deserialize)]
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
}

const ENGAGEMENTS: [&str; 4] = [
    "Pick one!",
    "Choose a winner!",
    "Who will win?",
    "Choose one!",
];

#[macroquad::main(window_conf)]
async fn main() {
    let mut rng = rng();
    let cli = Cli::parse();

    let mut render_time = 0.0;
    let mut time_offset = 0.0;

    let zoom = 1.125;

    let camera = Camera2D {
        zoom: vec2(2.0 / (1080.0 * SCALE * zoom), 2.0 / (1920.0 * SCALE * zoom)),
        offset: vec2(-1.0 / zoom, 1.0 / zoom),
        ..Camera2D::default()
    };

    set_camera(&camera);

    loop {
        let config_string = std::fs::read_to_string("config.toml").unwrap();
        let config = from_str::<Config>(&config_string).unwrap();

        let mut scene = if config.scene == 1 {
            scene_1(config.balls, cli.timescale, cli.physics_steps).await
        } else if config.scene == 2 {
            scene_2(config.balls, cli.timescale, cli.physics_steps).await
        } else if config.scene == 3 {
            scene_3(config.balls, cli.timescale, cli.physics_steps).await
        } else if config.scene == 4 {
            scene_4(config.balls, cli.timescale, cli.physics_steps).await
        } else if config.scene == 5 {
            scene_5(config.balls, cli.timescale, cli.physics_steps).await
        } else {
            scene_6(config.balls, cli.timescale, cli.physics_steps).await
        };

        let mut frame_number = 0;
        let mut collisions: HashMap<usize, Vec<Collision>> = HashMap::new();
        let engagement = ENGAGEMENTS.choose(&mut rng).unwrap();
        let mut maybe_all_won_time = None;


        if cli.render {
            let images_path = Path::new("images");

            if images_path.exists() {
                info!("Clearing previous frames!");
                std::fs::remove_dir_all(images_path).unwrap();
            }

            std::fs::create_dir(images_path).unwrap();
        }

        loop {
            let scene_time = if cli.render { render_time } else { get_time() } - time_offset;

            if scene_time >= cli.countdown_seconds as f64 {
                let update_collisions = scene.update();
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
                );

                if (scene_time * 2.0 + 1.5).floor() % 2.0 == 0.0 {
                    draw_text_outline(
                        engagement,
                        screen_width() / 2.0 - measure_text(engagement, None, 64, 1.0).width / 2.0,
                        screen_height() / 2.0 + 100.0,
                        64.0,
                        WHITE,
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
            } else {
                info!("Rendering failed!");
            }
        }

        if cli.endless {
            time_offset = get_time();
        } else {
            break;
        }
    }
}
