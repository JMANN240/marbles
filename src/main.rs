use clap::Parser;
use macroquad::prelude::*;
use scenes::{scene_1, scene_2, scene_3, scene_4, scene_5, scene_6};
use serde::Deserialize;
use toml::from_str;
use util::draw_text_outline;

mod ball;
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
    endless: bool,

    #[arg(short, long)]
    render: bool,

    #[arg(short, long, default_value_t = 3)]
    countdown_seconds: usize,

    #[arg(short, long, default_value_t = 10)]
    reset_seconds: usize,

    #[arg(short, long, default_value_t = 1.0)]
    timescale: f64,

    #[arg(short, long, default_value_t = 100)]
    physics_steps: usize,
}

#[macroquad::main(window_conf)]
async fn main() {
    let cli = Cli::parse();

    let mut time_offset = 0.0;

    let render_target = render_target_ex(
        (1080.0 * SCALE) as u32,
        (1920.0 * SCALE) as u32,
        RenderTargetParams {
            sample_count: 8,
            depth: false,
        },
    );

    let zoom = 1.2;

    let camera = Camera2D {
        zoom: vec2(
            2.0 / (1080.0 * SCALE * zoom),
            if cli.render { -2.0 } else { 2.0 } / (1920.0 * SCALE * zoom),
        ),
        offset: vec2(-1.0 / zoom, 1.0 / zoom),
        render_target: if cli.render {
            Some(render_target)
        } else {
            None
        },
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

        let mut maybe_all_won_time = None;
        loop {
            let scene_time = get_time() - time_offset;

            if scene_time >= cli.countdown_seconds as f64 {
                scene.update();
            }

            scene.draw();

            if scene_time.floor() < cli.countdown_seconds as f64 {
                let text = format!("{}", cli.countdown_seconds as f64 - scene_time.floor(),);
                draw_text_outline(
                    &text,
                    screen_width() / 2.0 - measure_text(&text, None, 256, 1.0).width / 2.0,
                    screen_height() / 2.0,
                    256.0,
                    WHITE,
                );
            }

            if scene.get_winners().len() == scene.get_balls().len() && maybe_all_won_time.is_none()
            {
                maybe_all_won_time = Some(scene_time);
            }

            if cli.endless
                && let Some(all_won_time) = maybe_all_won_time
            {
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

                if scene_time >= all_won_time + cli.reset_seconds as f64 {
                    time_offset = get_time();
                    break;
                }
            }

            next_frame().await;
        }
    }
}
