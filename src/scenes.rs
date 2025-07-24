use crate::{
    BallConfig,
    ball::{Ball, PhysicsBall},
    drawer::tail_drawer::TailDrawer,
    particle::{FireParticle, emitter::FrequencyParticleEmitter},
    scene::Scene,
    util::space_evenly,
    wall::{Wall, circle_wall::CircleWall, straight_wall::StraightWall},
};
use macroquad::{
    audio::load_sound,
    color::{BLACK, Color, RED, YELLOW},
    math::{DVec2, dvec2},
    window::{screen_height, screen_width},
};
use rand::{Rng, seq::SliceRandom};

pub async fn build_balls(ball_configs: &[BallConfig], positions: &mut [DVec2]) -> Vec<Ball> {
    let mut rng = rand::rng();

    let mut balls: Vec<Ball> = Vec::new();

    positions.shuffle(&mut rng);

    for (position, ball_config) in positions.iter().zip(ball_configs) {
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
                dvec2(0.0, 0.0),
                ball_config.radius,
                ball_config.elasticity,
            ),
            Box::new(if ball_config.name == "Fireball" {
                TailDrawer::new(YELLOW, RED, 100, 10)
            } else {
                TailDrawer::new(color, BLACK, 100, 10)
            }),
            load_sound(&ball_config.sound).await.unwrap(),
        );

        if ball_config.name == "Fireball" {
            ball.get_particles_mut()
                .add_emitter(Box::new(FrequencyParticleEmitter::new(
                    position,
                    10.0,
                    120.0,
                    |position, _spread| Box::new(FireParticle::new(position, 4.0, 0.5)),
                )));
        }

        balls.push(ball);
    }

    balls
}

pub async fn scene_1(balls: Vec<BallConfig>, timescale: f64, physics_steps: usize) -> Scene {
    let offset = 100.0;

    let mut walls: Vec<Box<dyn Wall>> = vec![
        Box::new(StraightWall::horizontal(0.0, false)),
        Box::new(StraightWall::vertical(0.0, false)),
        Box::new(StraightWall::horizontal(screen_height() as f64, true)),
        Box::new(StraightWall::vertical(screen_width() as f64, false)),
    ];

    for i in 0..8 {
        walls.push(Box::new(StraightWall::new(
            dvec2(0.0, 100.0 + offset * i as f64),
            dvec2(
                screen_width() as f64 * 0.5 - 16.0,
                125.0 + offset * i as f64,
            ),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            dvec2(
                screen_width() as f64 * 0.5 + 16.0,
                125.0 + offset * i as f64,
            ),
            dvec2(screen_width() as f64, 100.0 + offset * i as f64),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            dvec2(screen_width() as f64 * 0.5, 150.0 + offset * i as f64),
            dvec2(32.0, 175.0 + offset * i as f64),
            false,
        )));
        walls.push(Box::new(StraightWall::new(
            dvec2(screen_width() as f64 * 0.5, 150.0 + offset * i as f64),
            dvec2(screen_width() as f64 - 32.0, 175.0 + offset * i as f64),
            false,
        )));
    }

    let mut positions = space_evenly(
        balls.len(),
        dvec2(0.0, 50.0),
        dvec2(screen_width() as f64, 50.0),
    );

    Scene::new(
        build_balls(&balls, &mut positions).await,
        walls,
        timescale,
        physics_steps,
    )
}

pub async fn scene_2(balls: Vec<BallConfig>, timescale: f64, physics_steps: usize) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> = vec![
        Box::new(StraightWall::horizontal(0.0, false)),
        Box::new(StraightWall::vertical(0.0, false)),
        Box::new(StraightWall::horizontal(screen_height() as f64, true)),
        Box::new(StraightWall::vertical(screen_width() as f64, false)),
    ];

    let max_columns = 8;
    let x_spacing = screen_width() as f64 / (max_columns as f64 + 1.0);

    for j in 0..20 {
        let column_offset = j % 2;
        let columns = max_columns + 2 - column_offset;

        for i in 0..columns {
            let x = (x_spacing * 0.5 * column_offset as f64) + x_spacing * i as f64;
            let y = 100.0 + 36.0 * j as f64;

            walls.push(Box::new(StraightWall::new(
                dvec2(x - 12.0, y),
                dvec2(x, y - 6.0),
                false,
            )));
            walls.push(Box::new(StraightWall::new(
                dvec2(x + 12.0, y),
                dvec2(x, y - 6.0),
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
        build_balls(&balls, &mut positions).await,
        walls,
        timescale,
        physics_steps,
    )
}

pub async fn scene_3(balls: Vec<BallConfig>, timescale: f64, physics_steps: usize) -> Scene {
    let walls: Vec<Box<dyn Wall>> = vec![
        Box::new(StraightWall::horizontal(0.0, false)),
        Box::new(StraightWall::vertical(0.0, false)),
        Box::new(StraightWall::horizontal(screen_height() as f64, true)),
        Box::new(StraightWall::vertical(screen_width() as f64, false)),
    ];

    let mut positions = space_evenly(
        balls.len(),
        dvec2(0.0, 50.0),
        dvec2(screen_width() as f64, 50.0),
    );

    Scene::new(
        build_balls(&balls, &mut positions).await,
        walls,
        timescale,
        physics_steps,
    )
}

pub async fn scene_4(balls: Vec<BallConfig>, timescale: f64, physics_steps: usize) -> Scene {
    let walls: Vec<Box<dyn Wall>> = vec![
        Box::new(StraightWall::horizontal(0.0, false)),
        Box::new(StraightWall::vertical(0.0, false)),
        Box::new(StraightWall::horizontal(screen_height() as f64, true)),
        Box::new(StraightWall::vertical(screen_width() as f64, false)),
        Box::new(StraightWall::new(
            dvec2(screen_width() as f64 * 0.25, 0.0),
            dvec2(screen_width() as f64 * 0.475, screen_height() as f64 * 0.9),
            false,
        )),
        Box::new(StraightWall::new(
            dvec2(screen_width() as f64 * 0.75, 0.0),
            dvec2(screen_width() as f64 * 0.525, screen_height() as f64 * 0.9),
            false,
        )),
    ];

    let mut positions = space_evenly(
        balls.len(),
        dvec2(screen_width() as f64 / 2.0, 0.0),
        dvec2(screen_width() as f64 / 2.0, 200.0),
    );

    Scene::new(
        build_balls(&balls, &mut positions).await,
        walls,
        timescale,
        physics_steps,
    )
}

pub async fn scene_5(balls: Vec<BallConfig>, timescale: f64, physics_steps: usize) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> = vec![
        Box::new(StraightWall::horizontal(0.0, false)),
        Box::new(StraightWall::vertical(0.0, false)),
        Box::new(StraightWall::horizontal(screen_height() as f64, true)),
        Box::new(StraightWall::vertical(screen_width() as f64, false)),
    ];

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
        build_balls(&balls, &mut positions).await,
        walls,
        timescale,
        physics_steps,
    )
}

pub async fn scene_6(balls: Vec<BallConfig>, timescale: f64, physics_steps: usize) -> Scene {
    let mut walls: Vec<Box<dyn Wall>> = vec![
        Box::new(StraightWall::horizontal(0.0, false)),
        Box::new(StraightWall::vertical(0.0, false)),
        Box::new(StraightWall::horizontal(screen_height() as f64, true)),
        Box::new(StraightWall::vertical(screen_width() as f64, false)),
    ];

    let wall_size = screen_width() as f64 / 2.0 - 9.0;

    walls.push(Box::new(StraightWall::new(
        dvec2(0.0, 400.0),
        dvec2(wall_size, 400.0 + wall_size),
        false,
    )));
    walls.push(Box::new(StraightWall::new(
        dvec2(screen_width() as f64, 400.0),
        dvec2(screen_width() as f64 - wall_size, 400.0 + wall_size),
        false,
    )));

    let mut positions = space_evenly(
        balls.len(),
        dvec2(0.0, 50.0),
        dvec2(screen_width() as f64, 50.0),
    );

    Scene::new(
        build_balls(&balls, &mut positions).await,
        walls,
        timescale,
        physics_steps,
    )
}
