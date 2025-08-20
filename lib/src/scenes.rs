use std::{f64::consts::PI, path::PathBuf, sync::Arc};

use crate::{
    BallConfig,
    ball::{Ball, PhysicsBall},
    drawer::{
        base_style::BaseStyle, glow_style::GlowStyle, outline_style::OutlineStyle,
        tail_style::TailStyle,
    },
    particle::{FireParticle, ParticleLayer, ShrinkingParticle, emitter::BallParticleEmitter},
    scene::Scene,
    util::space_evenly,
    wall::{
        Wall,
        circle_wall::CircleWall,
        straight_wall::{Line, StraightWall},
    },
};
use glam::{DVec2, dvec2};
use itertools::izip;
use palette::Srgba;
use particula_rs::ParticleSystem;
use rand::{Rng, random_range, seq::SliceRandom};

pub fn build_balls(
    ball_configs: &[BallConfig],
    positions: &mut [DVec2],
    velocities: &[DVec2],
) -> Vec<Ball> {
    let mut rng = rand::rng();

    let mut balls: Vec<Ball> = Vec::new();

    positions.shuffle(&mut rng);

    for (ball_config, position, velocity) in izip!(ball_configs, positions, velocities) {
        let color = Srgba::new(ball_config.r, ball_config.g, ball_config.b, 1.0);

        let position = dvec2(
            position.x + rng.random_range(-8.0..=8.0),
            position.y + rng.random_range(-8.0..=8.0),
        );

        let mut ball = Ball::new(
            ball_config.name.clone(),
            color,
            PhysicsBall::new(
                position,
                *velocity,
                ball_config.radius,
                ball_config.density,
                ball_config.elasticity,
            ),
            if ball_config.name == "Fireball" {
                Box::new(TailStyle::new(
                    Srgba::new(1.0, 1.0, 0.0, 1.0),
                    Srgba::new(0.9, 0.1, 0.1, 1.0),
                    100,
                    10,
                ))
            } else if ball_config.name == "White Light" {
                Box::new(GlowStyle::new(
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                    16,
                ))
            } else if ball_config.name == "Black Hole" {
                Box::new(GlowStyle::new(
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                    Srgba::new(0.5, 0.0, 1.0, 1.0),
                    12,
                ))
            } else if ball_config.name == "Green Machine" {
                Box::new(OutlineStyle::new(Srgba::new(0.0, 0.9, 0.1, 1.0)))
            } else {
                Box::new(TailStyle::new(
                    color,
                    Srgba::new(0.0, 0.0, 0.0, 1.0),
                    100,
                    10,
                ))
            },
            PathBuf::from(ball_config.sound.clone()),
        );

        if ball_config.name == "Fireball" {
            ball.get_particles_mut()
                .add_emitter(BallParticleEmitter::new(
                    position,
                    120.0,
                    Arc::new(|position| {
                        Box::new(FireParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(0.0..=8.0),
                            4.0,
                            0.5,
                            ParticleLayer::random(),
                        ))
                    }),
                ));
        }

        if ball_config.name == "White Light" {
            ball.get_particles_mut()
                .add_emitter(BallParticleEmitter::new(
                    position,
                    32.0,
                    Arc::new(|position| {
                        Box::new(ShrinkingParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(8.0..12.0),
                            DVec2::ZERO,
                            1.0,
                            Srgba::new(1.0, 1.0, 1.0, 1.0),
                            0.125,
                            ParticleLayer::random(),
                        ))
                    }),
                ));
        }

        if ball_config.name == "Black Hole" {
            let _radius = ball.get_radius();

            ball.get_particles_mut()
                .add_emitter(BallParticleEmitter::new(
                    position,
                    16.0,
                    Arc::new(|position| {
                        Box::new(ShrinkingParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(8.0..12.0),
                            DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                * random_range(8.0..16.0),
                            random_range(1.0..=4.0),
                            Srgba::new(
                                random_range(0.25..=0.5),
                                0.0,
                                random_range(0.75..=1.0),
                                1.0,
                            ),
                            random_range(0.25..0.75),
                            ParticleLayer::Back,
                        ))
                    }),
                ));
        }

        // if ball_config.name == "White Light" {
        //     let radius = ball.get_radius();
        //     let color = ball.get_name_color();

        //     ball.get_particles_mut()
        //         .add_emitter(Box::new(FrequencyParticleEmitter::new(
        //             position,
        //             DVec2::ZERO,
        //             0.0,
        //             10.0,
        //             move |position, velocity, _spread| {
        //                 Box::new(PuffParticle::new(position, velocity, radius, color, 0.5))
        //             },
        //         )));
        // }

        balls.push(ball);
    }

    balls
}

pub fn scene_1(balls: Vec<BallConfig>, scene_width: f64, scene_height: f64) -> Scene {
    let offset = 100.0;

    let mut walls: Vec<Box<dyn Wall>> =
        StraightWall::rect(0.0, 0.0, scene_width, scene_height, true)
            .into_iter()
            .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
            .collect();

    for i in 0..8 {
        walls.push(Box::new(StraightWall::new(
            Line::new(
                dvec2(0.0, 100.0 + offset * i as f64),
                dvec2(scene_width * 0.5 - 16.0, 125.0 + offset * i as f64),
            ),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            Line::new(
                dvec2(scene_width * 0.5 + 16.0, 125.0 + offset * i as f64),
                dvec2(scene_width, 100.0 + offset * i as f64),
            ),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            Line::new(
                dvec2(scene_width * 0.5, 150.0 + offset * i as f64),
                dvec2(32.0, 175.0 + offset * i as f64),
            ),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            Line::new(
                dvec2(scene_width * 0.5, 150.0 + offset * i as f64),
                dvec2(scene_width - 32.0, 175.0 + offset * i as f64),
            ),
            false,
        )));
    }

    let mut positions = space_evenly(balls.len(), dvec2(0.0, 50.0), dvec2(scene_width, 50.0));

    Scene::new(
        build_balls(&balls, &mut positions, &vec![dvec2(0.0, 0.0); balls.len()]),
        walls,
    )
}

pub fn scene_2(balls: Vec<BallConfig>, scene_width: f64, scene_height: f64) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> =
        StraightWall::rect(0.0, 0.0, scene_width, scene_height, true)
            .into_iter()
            .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
            .collect();

    let max_columns = 8;
    let x_spacing = scene_width / (max_columns as f64 + 1.0);

    for j in 0..20 {
        let column_offset = j % 2;
        let columns = max_columns + 2 - column_offset;

        for i in 0..columns {
            let x = (x_spacing * 0.5 * column_offset as f64) + x_spacing * i as f64;
            let y = 100.0 + 36.0 * j as f64;

            walls.push(Box::new(StraightWall::new(
                Line::new(dvec2(x - 12.0, y), dvec2(x, y - 6.0)),
                false,
            )));
            walls.push(Box::new(StraightWall::new(
                Line::new(dvec2(x + 12.0, y), dvec2(x, y - 6.0)),
                false,
            )));
        }
    }

    let mut positions = space_evenly(balls.len(), dvec2(0.0, 50.0), dvec2(scene_width, 50.0));

    Scene::new(
        build_balls(&balls, &mut positions, &vec![dvec2(0.0, 0.0); balls.len()]),
        walls,
    )
}

pub fn scene_3(balls: Vec<BallConfig>, scene_width: f64, scene_height: f64) -> Scene {
    let walls: Vec<Box<dyn Wall>> = StraightWall::rect(0.0, 0.0, scene_width, scene_height, true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    let mut positions = space_evenly(balls.len(), dvec2(0.0, 50.0), dvec2(scene_width, 50.0));

    Scene::new(
        build_balls(&balls, &mut positions, &vec![dvec2(0.0, 0.0); balls.len()]),
        walls,
    )
}

pub fn scene_4(balls: Vec<BallConfig>, scene_width: f64, scene_height: f64) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> =
        StraightWall::rect(0.0, 0.0, scene_width, scene_height, true)
            .into_iter()
            .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
            .collect();

    walls.push(Box::new(StraightWall::new(
        Line::new(
            dvec2(scene_width * 0.25, 0.0),
            dvec2(scene_width * 0.475, scene_height * 0.9),
        ),
        false,
    )));

    walls.push(Box::new(StraightWall::new(
        Line::new(
            dvec2(scene_width * 0.75, 0.0),
            dvec2(scene_width * 0.525, scene_height * 0.9),
        ),
        false,
    )));

    let mut positions = space_evenly(
        balls.len(),
        dvec2(scene_width / 2.0, 0.0),
        dvec2(scene_width / 2.0, 400.0),
    );

    let velocities = vec![dvec2(500.0, 0.0); balls.len()];

    Scene::new(build_balls(&balls, &mut positions, &velocities), walls)
}

pub fn scene_5(balls: Vec<BallConfig>, scene_width: f64, scene_height: f64) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> =
        StraightWall::rect(0.0, 0.0, scene_width, scene_height, true)
            .into_iter()
            .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
            .collect();

    let max_columns = 12;
    let x_spacing = scene_width / (max_columns as f64 + 1.0);

    for j in 0..24 {
        let column_offset = j % 2;
        let columns = max_columns + 2 - column_offset;

        for i in 0..columns {
            let x = (x_spacing * 0.5 * column_offset as f64) + x_spacing * i as f64;
            let y = 100.0 + x_spacing * (2.0f64.sqrt() / 2.0) * j as f64;

            walls.push(Box::new(CircleWall::new(
                dvec2(x, y),
                4.0,
                0.0,
                360.0,
                false,
            )));
        }
    }

    let mut positions = space_evenly(balls.len(), dvec2(0.0, 50.0), dvec2(scene_width, 50.0));

    Scene::new(
        build_balls(&balls, &mut positions, &vec![dvec2(0.0, 0.0); balls.len()]),
        walls,
    )
}

pub fn scene_6(balls: Vec<BallConfig>, scene_width: f64, scene_height: f64) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> =
        StraightWall::rect(0.0, 0.0, scene_width, scene_height, true)
            .into_iter()
            .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
            .collect();

    let wall_size = scene_width / 2.0 - 9.0;

    walls.push(Box::new(StraightWall::new(
        Line::new(dvec2(0.0, 400.0), dvec2(wall_size, 400.0 + wall_size)),
        false,
    )));
    walls.push(Box::new(StraightWall::new(
        Line::new(
            dvec2(scene_width, 400.0),
            dvec2(scene_width - wall_size, 400.0 + wall_size),
        ),
        false,
    )));

    let mut positions = space_evenly(balls.len(), dvec2(0.0, 50.0), dvec2(scene_width, 50.0));

    Scene::new(
        build_balls(
            &balls,
            &mut positions,
            &vec![dvec2(100.0, 0.0); balls.len()],
        ),
        walls,
    )
}

pub fn scene_7(scene_width: f64, scene_height: f64) -> Scene {
    let balls = vec![
        Ball::new(
            "Big Red".to_string(),
            Srgba::new(0.9, 0.1, 0.1, 1.0),
            PhysicsBall::new(dvec2(100.0, 100.0), dvec2(100.0, 0.0), 32.0, 1.0, 0.99),
            Box::new(BaseStyle::new(Srgba::new(0.9, 0.1, 0.1, 1.0))),
            PathBuf::from("piano_c6.wav"),
        ),
        Ball::new(
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

    Scene::new(balls, walls)
}
