use std::path::PathBuf;

use crate::{
    BallConfig,
    ball::{Ball, PhysicsBall},
    drawer::base_style::BaseStyle,
    levels::{level_1, level_2, level_3, level_4, level_5, level_6, level_8, level_9, level_10},
    powerup::{change_position::ChangePosition, random_powerup},
    scene::Scene,
    wall::{Wall, straight_wall::StraightWall},
};
use glam::dvec2;
use palette::Srgba;
use rand::{Rng, seq::SliceRandom};

pub fn scene_1(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    level_1(ball_configs.len(), scene_width, scene_height).build_scene(
        rng,
        ball_configs,
        |powerup_space| {
            random_powerup(
                &mut rand::rng(),
                powerup_space.get_position(),
                scene_width,
                scene_height,
            )
        },
    )
}

pub fn scene_2(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    level_2(ball_configs.len(), scene_width, scene_height).build_scene(
        rng,
        ball_configs,
        |powerup_space| {
            random_powerup(
                &mut rand::rng(),
                powerup_space.get_position(),
                scene_width,
                scene_height,
            )
        },
    )
}

pub fn scene_3(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    level_3(ball_configs.len(), scene_width, scene_height).build_scene(
        rng,
        ball_configs,
        |powerup_space| {
            random_powerup(
                &mut rand::rng(),
                powerup_space.get_position(),
                scene_width,
                scene_height,
            )
        },
    )
}

pub fn scene_4(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    level_4(ball_configs.len(), scene_width, scene_height).build_scene(
        rng,
        ball_configs,
        |powerup_space| {
            random_powerup(
                &mut rand::rng(),
                powerup_space.get_position(),
                scene_width,
                scene_height,
            )
        },
    )
}

pub fn scene_5(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    level_5(ball_configs.len(), scene_width, scene_height).build_scene(
        rng,
        ball_configs,
        |powerup_space| {
            random_powerup(
                &mut rand::rng(),
                powerup_space.get_position(),
                scene_width,
                scene_height,
            )
        },
    )
}

pub fn scene_6(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    level_6(ball_configs.len(), scene_width, scene_height).build_scene(
        rng,
        ball_configs,
        |powerup_space| {
            random_powerup(
                &mut rand::rng(),
                powerup_space.get_position(),
                scene_width,
                scene_height,
            )
        },
    )
}

pub fn scene_7(scene_width: f64, scene_height: f64) -> Scene {
    let balls = vec![
        Ball::new(
            "Big Red".to_string(),
            "Big Red".to_string(),
            Srgba::new(0.9, 0.1, 0.1, 1.0),
            PhysicsBall::new(dvec2(100.0, 100.0), dvec2(100.0, 0.0), 32.0, 1.0, 0.99),
            Box::new(BaseStyle::new(Srgba::new(0.9, 0.1, 0.1, 1.0))),
            PathBuf::from("piano_c6.wav"),
        ),
        Ball::new(
            "Little Blue".to_string(),
            "Little Blue".to_string(),
            Srgba::new(0.0, 0.5, 1.0, 1.0),
            PhysicsBall::new(dvec2(300.0, 100.0), dvec2(300.0, 0.0), 8.0, 1.0, 0.99),
            Box::new(BaseStyle::new(Srgba::new(0.0, 0.5, 1.0, 1.0))),
            PathBuf::from("piano_c7.wav"),
        ),
    ];

    let walls: Vec<Box<dyn Wall>> = StraightWall::rect(0.0, 0.0, scene_width, scene_height, true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    Scene::new(balls, Vec::new(), walls)
}

pub fn scene_8(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    level_8(ball_configs.len(), scene_width, scene_height).build_scene(
        rng,
        ball_configs,
        |powerup_space| {
            Box::new(ChangePosition::new(
                powerup_space.get_position(),
                "Teleport",
                8.0,
                16.0..=(scene_width - 16.0),
                16.0..=(scene_height - 16.0),
            ))
        },
    )
}

pub fn scene_9(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    level_9(ball_configs.len(), scene_width, scene_height).build_scene(
        rng,
        ball_configs,
        |powerup_space| {
            random_powerup(
                &mut rand::rng(),
                powerup_space.get_position(),
                scene_width,
                scene_height,
            )
        },
    )
}

pub fn scene_10(
    rng: &mut impl Rng,
    ball_configs: Vec<BallConfig>,
    scene_width: f64,
    scene_height: f64,
) -> Scene {
    let mut shuffled_ball_configs = ball_configs.clone();
    shuffled_ball_configs.shuffle(rng);
    level_10(4, scene_width, scene_height).build_scene(
        rng,
        shuffled_ball_configs,
        |powerup_space| {
            random_powerup(
                &mut rand::rng(),
                powerup_space.get_position(),
                scene_width,
                scene_height,
            )
        },
    )
}
