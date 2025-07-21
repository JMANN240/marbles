use macroquad::prelude::*;
use scenes::{build_balls, scene_1, scene_2, scene_3, scene_4};
use serde::Deserialize;
use toml::from_str;

mod ball;
mod drawer;
mod particle;
mod scene;
mod scenes;
mod wall;

const PHYSICS_STEPS: usize = 100;
const TIMESCALE: f64 = 1.0;

const SCALE: f64 = 0.5;

fn window_conf() -> Conf {
    Conf {
        window_width: (1080.0 * SCALE) as i32,
        window_height: (1920.0 * SCALE) as i32,
        window_title: "BallRace".to_owned(),
        sample_count: 8,
        ..Default::default()
    }
}

const COUNTDOWN_SECONDS: usize = 3;
const RESET_SECONDS: usize = 10;

#[derive(Deserialize)]
pub struct ConfigPosition {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
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
    ball_positions: Vec<ConfigPosition>,
    scene: usize,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut time_offset = 0.0;

    loop {
        let config_string = std::fs::read_to_string("config.toml").unwrap();
        let mut config = from_str::<Config>(&config_string).unwrap();

        let balls = build_balls(&mut config.ball_positions, &config.balls).await;

        let mut scene = if config.scene == 1 {
            scene_1(balls).await
        } else if config.scene == 2 {
            scene_2(balls).await
        } else if config.scene == 3 {
            scene_3(balls).await
        } else {
            scene_4(balls).await
        };

        let mut maybe_all_won_time = None;
        loop {
            let scene_time = get_time() - time_offset;

            if scene_time >= COUNTDOWN_SECONDS as f64 {
                scene.update();
            }

            scene.draw();

            if scene_time.floor() < COUNTDOWN_SECONDS as f64 {
                let text = format!("{}", COUNTDOWN_SECONDS as f64 - scene_time.floor(),);
                draw_text_outline(
                    &text,
                    screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                    screen_height() / 2.0,
                    256.0,
                    WHITE,
                    16,
                );
            }

            if scene.get_winners().len() == scene.get_balls().len() && maybe_all_won_time.is_none()
            {
                maybe_all_won_time = Some(scene_time);
            }

            if let Some(all_won_time) = maybe_all_won_time {
                let text = format!(
                    "{}",
                    RESET_SECONDS as f64 - (scene_time - all_won_time).floor()
                );

                draw_text_outline(
                    &text,
                    screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                    screen_height() / 2.0,
                    256.0,
                    WHITE,
                    16,
                );

                if scene_time >= all_won_time + RESET_SECONDS as f64 {
                    time_offset = get_time();
                    break;
                }
            }

            next_frame().await;
        }
    }
}

pub fn draw_text_outline(text: &str, x: f32, y: f32, font_size: f32, color: Color, thickness: i32) {
    for i in -thickness..=thickness {
        for j in -thickness..=thickness {
            draw_text(text, x + i as f32, y + j as f32, font_size, BLACK);
        }
    }

    draw_text(text, x, y, font_size, color);
}

pub fn lerp_f64(start: f64, end: f64, t: f64) -> f64 {
    start * (1.0 * t) + end * t
}

pub fn lerp_f32(start: f32, end: f32, t: f32) -> f32 {
    start * (1.0 * t) + end * t
}

pub fn lerp_color(start: Color, end: Color, t: f32) -> Color {
    Color {
        r: lerp_f32(start.r, end.r, t),
        g: lerp_f32(start.g, end.g, t),
        b: lerp_f32(start.b, end.b, t),
        a: lerp_f32(start.a, end.a, t),
    }
}
