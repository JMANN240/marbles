use crate::{
    BallConfig, ConfigPosition, Scene,
    ball::Ball,
    drawer::tail_drawer::TailDrawer,
    particle::{
        FireParticle, ShrinkingParticle,
        emitter::{BaseParticleEmitter, FrequencyParticleEmitter},
        system::ParticleSystem,
    },
    wall::Wall,
};
use macroquad::{audio::load_sound, prelude::*, rand::ChooseRandom};

pub async fn build_balls(
    ball_positions: &mut Vec<ConfigPosition>,
    ball_configs: &Vec<BallConfig>,
) -> Vec<Ball> {
    let mut balls: Vec<Ball> = Vec::new();

    ball_positions.shuffle();

    for (ball_position, ball_config) in ball_positions.iter().zip(ball_configs) {
        let color = Color {
            r: ball_config.r,
            b: ball_config.b,
            g: ball_config.g,
            a: 1.0,
        };

        let position = dvec2(
            ball_position.x + rand::gen_range(-1.0, 1.0),
            ball_position.y,
        );

        let mut ball = Ball::new(
            ball_config.name.clone(),
            color,
            position,
            dvec2(ball_position.vx, ball_position.vy),
            ball_config.radius,
            ball_config.elasticity,
            Box::new(TailDrawer::new(color, BLACK, 1000)),
            load_sound(&ball_config.sound).await.unwrap(),
        );

        // ball.get_particles_mut()
        //     .add_emitter(Box::new(FrequencyParticleEmitter::new(
        //         position,
        //         10.0,
        //         120.0,
        //         |position, _spread| {
        //             Box::new(FireParticle::new(position, 4.0, 0.5))
        //         },
        //     )));

        balls.push(ball);
    }

    balls
}

pub async fn scene_1(balls: Vec<Ball>) -> Scene {
    let offset = 100.0;

    let mut walls = vec![
        Wall::horizontal(0.0, false),
        Wall::vertical(0.0, false),
        Wall::horizontal(screen_height() as f64, true),
        Wall::vertical(screen_width() as f64, false),
    ];

    for i in 0..8 {
        walls.push(Wall::new(
            dvec2(0.0, 100.0 + offset * i as f64),
            dvec2(
                screen_width() as f64 * 0.5 - 16.0,
                125.0 + offset * i as f64,
            ),
            false,
        ));
        walls.push(Wall::new(
            dvec2(
                screen_width() as f64 * 0.5 + 16.0,
                125.0 + offset * i as f64,
            ),
            dvec2(screen_width() as f64, 100.0 + offset * i as f64),
            false,
        ));
        walls.push(Wall::new(
            dvec2(screen_width() as f64 * 0.5, 150.0 + offset * i as f64),
            dvec2(32.0, 175.0 + offset * i as f64),
            false,
        ));
        walls.push(Wall::new(
            dvec2(screen_width() as f64 * 0.5, 150.0 + offset * i as f64),
            dvec2(screen_width() as f64 - 32.0, 175.0 + offset * i as f64),
            false,
        ));
    }

    Scene {
        balls,
        walls,
        winners: Vec::new(),
        particles: ParticleSystem::default(),
    }
}

pub async fn scene_2(balls: Vec<Ball>) -> Scene {
    let mut walls = vec![
        Wall::horizontal(0.0, false),
        Wall::vertical(0.0, false),
        Wall::horizontal(screen_height() as f64, true),
        Wall::vertical(screen_width() as f64, false),
    ];

    let max_columns = 8;
    let x_spacing = screen_width() as f64 / (max_columns as f64 + 1.0);

    for j in 0..20 {
        let column_offset = j % 2;
        let columns = max_columns + 2 - column_offset;

        for i in 0..columns {
            let x = (x_spacing * 0.5 * column_offset as f64) + x_spacing * i as f64;
            let y = 100.0 + 36.0 * j as f64;

            walls.push(Wall::new(dvec2(x - 12.0, y), dvec2(x, y - 6.0), false));
            walls.push(Wall::new(dvec2(x + 12.0, y), dvec2(x, y - 6.0), false));
        }
    }

    Scene {
        balls,
        walls,
        winners: Vec::new(),
        particles: ParticleSystem::default(),
    }
}

pub async fn scene_3(balls: Vec<Ball>) -> Scene {
    let walls = vec![
        Wall::horizontal(0.0, false),
        Wall::vertical(0.0, false),
        Wall::horizontal(screen_height() as f64, true),
        Wall::vertical(screen_width() as f64, false),
    ];

    Scene {
        balls,
        walls,
        winners: Vec::new(),
        particles: ParticleSystem::default(),
    }
}

pub async fn scene_4(balls: Vec<Ball>) -> Scene {
    let walls = vec![
        Wall::horizontal(0.0, false),
        Wall::vertical(0.0, false),
        Wall::horizontal(screen_height() as f64, true),
        Wall::vertical(screen_width() as f64, false),
        Wall::new(
            dvec2(screen_width() as f64 * 0.25, 0.0),
            dvec2(screen_width() as f64 * 0.475, screen_height() as f64 * 0.9),
            false,
        ),
        Wall::new(
            dvec2(screen_width() as f64 * 0.75, 0.0),
            dvec2(screen_width() as f64 * 0.525, screen_height() as f64 * 0.9),
            false,
        ),
    ];

    Scene {
        balls,
        walls,
        winners: Vec::new(),
        particles: ParticleSystem::default(),
    }
}
