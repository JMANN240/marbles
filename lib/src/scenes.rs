use std::{f64::consts::PI, path::PathBuf};

use crate::{
    BallConfig,
    ball::{Ball, PhysicsBall},
    drawer::{
        base_drawer::BaseDrawer, glow_drawer::GlowDrawer, outline_drawer::OutlineDrawer,
        tail_drawer::TailDrawer,
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
use itertools::izip;
use macroquad::{
    audio::load_sound,
    color::{BLACK, BLUE, Color, GREEN, RED, WHITE, YELLOW},
    math::{DVec2, dvec2},
    window::{screen_height, screen_width},
};
use particula_rs::ParticleSystem;
use rand::{Rng, random_range, seq::SliceRandom};

pub async fn build_balls(
    ball_configs: &[BallConfig],
    positions: &mut [DVec2],
    velocities: &[DVec2],
) -> Vec<Ball> {
    let mut rng = rand::rng();

    let mut balls: Vec<Ball> = Vec::new();

    positions.shuffle(&mut rng);

    for (ball_config, position, velocity) in izip!(ball_configs, positions, velocities) {
        let color = Color {
            r: ball_config.r,
            b: ball_config.b,
            g: ball_config.g,
            a: 1.0,
        };

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
                ball_config.elasticity,
            ),
            if ball_config.name == "Fireball" {
                Box::new(TailDrawer::new(YELLOW, RED, 100, 10))
            } else if ball_config.name == "White Light" {
                Box::new(GlowDrawer::new(WHITE, WHITE, 16))
            } else if ball_config.name == "Black Hole" {
                Box::new(GlowDrawer::new(
                    BLACK,
                    Color {
                        r: 0.5,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    12,
                ))
            } else if ball_config.name == "Green Machine" {
                Box::new(OutlineDrawer::new(GREEN))
            } else {
                Box::new(TailDrawer::new(color, BLACK, 100, 10))
            },
            PathBuf::from(ball_config.sound.clone()),
            load_sound(&ball_config.sound).await.unwrap(),
        );

        if ball_config.name == "Fireball" {
            ball.get_particles_mut()
                .add_emitter(Box::new(BallParticleEmitter::new(
                    position,
                    120.0,
                    Box::new(|position| {
                        Box::new(FireParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(0.0..=8.0),
                            4.0,
                            0.5,
                            ParticleLayer::random(),
                        ))
                    }),
                )));
        }

        if ball_config.name == "White Light" {
            ball.get_particles_mut()
                .add_emitter(Box::new(BallParticleEmitter::new(
                    position,
                    32.0,
                    Box::new(|position| {
                        Box::new(ShrinkingParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(8.0..12.0),
                            DVec2::ZERO,
                            1.0,
                            WHITE,
                            0.125,
                            ParticleLayer::random(),
                        ))
                    }),
                )));
        }

        if ball_config.name == "Black Hole" {
            let _radius = ball.get_radius();

            ball.get_particles_mut()
                .add_emitter(Box::new(BallParticleEmitter::new(
                    position,
                    16.0,
                    Box::new(|position| {
                        Box::new(ShrinkingParticle::new(
                            position
                                + DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                    * random_range(8.0..12.0),
                            DVec2::from_angle(random_range(0.0..(2.0 * PI)))
                                * random_range(8.0..16.0),
                            random_range(1.0..=4.0),
                            Color {
                                r: random_range(0.25..=0.5),
                                g: 0.0,
                                b: random_range(0.75..=1.0),
                                a: 1.0,
                            },
                            random_range(0.25..0.75),
                            ParticleLayer::Back,
                        ))
                    }),
                )));
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

pub async fn scene_1(balls: Vec<BallConfig>) -> Scene {
    let offset = 100.0;

    let mut walls: Vec<Box<dyn Wall>> = StraightWall::screen(true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    for i in 0..8 {
        walls.push(Box::new(StraightWall::new(
            Line::new(
                dvec2(0.0, 100.0 + offset * i as f64),
                dvec2(
                    screen_width() as f64 * 0.5 - 16.0,
                    125.0 + offset * i as f64,
                ),
            ),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            Line::new(
                dvec2(
                    screen_width() as f64 * 0.5 + 16.0,
                    125.0 + offset * i as f64,
                ),
                dvec2(screen_width() as f64, 100.0 + offset * i as f64),
            ),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            Line::new(
                dvec2(screen_width() as f64 * 0.5, 150.0 + offset * i as f64),
                dvec2(32.0, 175.0 + offset * i as f64),
            ),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            Line::new(
                dvec2(screen_width() as f64 * 0.5, 150.0 + offset * i as f64),
                dvec2(screen_width() as f64 - 32.0, 175.0 + offset * i as f64),
            ),
            false,
        )));
    }

    let mut positions = space_evenly(
        balls.len(),
        dvec2(0.0, 50.0),
        dvec2(screen_width() as f64, 50.0),
    );

    Scene::new(
        build_balls(&balls, &mut positions, &vec![dvec2(0.0, 0.0); balls.len()]).await,
        walls,
    )
}

pub async fn scene_2(balls: Vec<BallConfig>) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> = StraightWall::screen(true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    let max_columns = 8;
    let x_spacing = screen_width() as f64 / (max_columns as f64 + 1.0);

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

    let mut positions = space_evenly(
        balls.len(),
        dvec2(0.0, 50.0),
        dvec2(screen_width() as f64, 50.0),
    );

    Scene::new(
        build_balls(&balls, &mut positions, &vec![dvec2(0.0, 0.0); balls.len()]).await,
        walls,
    )
}

pub async fn scene_3(balls: Vec<BallConfig>) -> Scene {
    let walls: Vec<Box<dyn Wall>> = StraightWall::screen(true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    let mut positions = space_evenly(
        balls.len(),
        dvec2(0.0, 50.0),
        dvec2(screen_width() as f64, 50.0),
    );

    Scene::new(
        build_balls(&balls, &mut positions, &vec![dvec2(0.0, 0.0); balls.len()]).await,
        walls,
    )
}

pub async fn scene_4(balls: Vec<BallConfig>) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> = StraightWall::screen(true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    walls.push(Box::new(StraightWall::new(
        Line::new(
            dvec2(screen_width() as f64 * 0.25, 0.0),
            dvec2(screen_width() as f64 * 0.475, screen_height() as f64 * 0.9),
        ),
        false,
    )));

    walls.push(Box::new(StraightWall::new(
        Line::new(
            dvec2(screen_width() as f64 * 0.75, 0.0),
            dvec2(screen_width() as f64 * 0.525, screen_height() as f64 * 0.9),
        ),
        false,
    )));

    let mut positions = space_evenly(
        balls.len(),
        dvec2(screen_width() as f64 / 2.0, 0.0),
        dvec2(screen_width() as f64 / 2.0, 400.0),
    );

    let velocities = vec![dvec2(500.0, 0.0); balls.len()];

    Scene::new(
        build_balls(&balls, &mut positions, &velocities).await,
        walls,
    )
}

pub async fn scene_5(balls: Vec<BallConfig>) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> = StraightWall::screen(true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    let max_columns = 12;
    let x_spacing = screen_width() as f64 / (max_columns as f64 + 1.0);

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

    let mut positions = space_evenly(
        balls.len(),
        dvec2(0.0, 50.0),
        dvec2(screen_width() as f64, 50.0),
    );

    Scene::new(
        build_balls(&balls, &mut positions, &vec![dvec2(0.0, 0.0); balls.len()]).await,
        walls,
    )
}

pub async fn scene_6(balls: Vec<BallConfig>) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> = StraightWall::screen(true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    let wall_size = screen_width() as f64 / 2.0 - 9.0;

    walls.push(Box::new(StraightWall::new(
        Line::new(dvec2(0.0, 400.0), dvec2(wall_size, 400.0 + wall_size)),
        false,
    )));
    walls.push(Box::new(StraightWall::new(
        Line::new(
            dvec2(screen_width() as f64, 400.0),
            dvec2(screen_width() as f64 - wall_size, 400.0 + wall_size),
        ),
        false,
    )));

    let mut positions = space_evenly(
        balls.len(),
        dvec2(0.0, 50.0),
        dvec2(screen_width() as f64, 50.0),
    );

    Scene::new(
        build_balls(
            &balls,
            &mut positions,
            &vec![dvec2(100.0, 0.0); balls.len()],
        )
        .await,
        walls,
    )
}

pub async fn scene_7() -> Scene {
    let balls = vec![
        Ball::new(
            "Big Red".to_string(),
            RED,
            PhysicsBall::new(dvec2(100.0, 100.0), dvec2(100.0, 0.0), 32.0, 0.99),
            Box::new(BaseDrawer::new(RED)),
            PathBuf::from("piano_c6.wav"),
            load_sound("piano_c6.wav").await.unwrap(),
        ),
        Ball::new(
            "Little Blue".to_string(),
            BLUE,
            PhysicsBall::new(dvec2(300.0, 100.0), dvec2(300.0, 0.0), 8.0, 0.99),
            Box::new(BaseDrawer::new(BLUE)),
            PathBuf::from("piano_c7.wav"),
            load_sound("piano_c7.wav").await.unwrap(),
        ),
    ];

    let walls: Vec<Box<dyn Wall>> = StraightWall::screen(false)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    Scene::new(balls, walls)
}
