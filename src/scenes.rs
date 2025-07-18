use crate::{
    ball::{base_ball::BaseBall, tracked_ball::TrackedBall}, drawer::{base_drawer::BaseDrawer, tail_drawer::TailDrawer}, wall::Wall, Scene
};
use macroquad::{audio::load_sound, prelude::*};

pub async fn scene_1() -> Scene {
    let piano_c6 = load_sound("piano_c6.wav").await.unwrap();
    let piano_e6 = load_sound("piano_e6.wav").await.unwrap();
    let piano_g6 = load_sound("piano_g6.wav").await.unwrap();
    let piano_c7 = load_sound("piano_c7.wav").await.unwrap();

    let mut walls = vec![
        Wall::horizontal(0.0, false),
        Wall::vertical(0.0, false),
        Wall::horizontal(screen_height() as f64, true),
        Wall::vertical(screen_width() as f64, false),
    ];

    let offset = 100.0;

    let elasticity = 0.9;

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
        balls: vec![
            Box::new(TrackedBall::new(
                "Fireball".to_string(),
                RED,
                dvec2(
                    100.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(RED, BLACK),
                piano_c6,
            )),
            Box::new(TrackedBall::new(
                "Deep Blue".to_string(),
                BLUE,
                dvec2(
                    400.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(BLUE, BLACK),
                piano_e6,
            )),
            Box::new(TrackedBall::new(
                "White Light".to_string(),
                WHITE,
                dvec2(
                    200.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(WHITE, BLACK),
                piano_g6,
            )),
            Box::new(TrackedBall::new(
                "Green Machine".to_string(),
                GREEN,
                dvec2(
                    300.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(GREEN, BLACK),
                piano_c7,
            )),
        ],
        walls,
        winners: Vec::new(),
    }
}

pub async fn scene_2() -> Scene {
    let piano_c6 = load_sound("piano_c6.wav").await.unwrap();
    let piano_e6 = load_sound("piano_e6.wav").await.unwrap();
    let piano_g6 = load_sound("piano_g6.wav").await.unwrap();
    let piano_c7 = load_sound("piano_c7.wav").await.unwrap();

    let mut walls = vec![
        Wall::horizontal(0.0, false),
        Wall::vertical(0.0, false),
        Wall::horizontal(screen_height() as f64, true),
        Wall::vertical(screen_width() as f64, false),
    ];

    let elasticity = 0.9;

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
        balls: vec![
            Box::new(TrackedBall::new(
                "Fireball".to_string(),
                RED,
                dvec2(
                    255.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(RED, BLACK),
                piano_c6,
            )),
            Box::new(BaseBall::new(
                "Deep Blue".to_string(),
                BLUE,
                dvec2(
                    285.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                BaseDrawer::new(BLUE),
                piano_e6,
            )),
            Box::new(TrackedBall::new(
                "White Light".to_string(),
                WHITE,
                dvec2(
                    225.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(WHITE, BLACK),
                piano_g6,
            )),
            Box::new(TrackedBall::new(
                "Green Machine".to_string(),
                GREEN,
                dvec2(
                    315.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(GREEN, BLACK),
                piano_c7,
            )),
        ],
        walls,
        winners: Vec::new(),
    }
}

pub async fn scene_3() -> Scene {
    let piano_c6 = load_sound("piano_c6.wav").await.unwrap();
    let piano_e6 = load_sound("piano_e6.wav").await.unwrap();
    let piano_g6 = load_sound("piano_g6.wav").await.unwrap();
    let piano_c7 = load_sound("piano_c7.wav").await.unwrap();

    let walls = vec![
        Wall::horizontal(0.0, false),
        Wall::vertical(0.0, false),
        Wall::horizontal(screen_height() as f64, true),
        Wall::vertical(screen_width() as f64, false),
    ];

    let elasticity = 0.9;

    Scene {
        balls: vec![
            Box::new(TrackedBall::new(
                "Fireball".to_string(),
                RED,
                dvec2(
                    255.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(RED, BLACK),
                piano_c6,
            )),
            Box::new(TrackedBall::new(
                "Deep Blue".to_string(),
                BLUE,
                dvec2(
                    285.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(BLUE, BLACK),
                piano_e6,
            )),
            Box::new(TrackedBall::new(
                "White Light".to_string(),
                WHITE,
                dvec2(
                    225.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(WHITE, BLACK),
                piano_g6,
            )),
            Box::new(TrackedBall::new(
                "Green Machine".to_string(),
                GREEN,
                dvec2(
                    315.0 + rand::gen_range(-5.0, 5.0),
                    50.0 + rand::gen_range(-5.0, 5.0),
                ),
                dvec2(0.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(GREEN, BLACK),
                piano_c7,
            )),
        ],
        walls,
        winners: Vec::new(),
    }
}

pub async fn scene_4() -> Scene {
    let piano_c6 = load_sound("piano_c6.wav").await.unwrap();
    let piano_e6 = load_sound("piano_e6.wav").await.unwrap();
    let piano_g6 = load_sound("piano_g6.wav").await.unwrap();
    let piano_c7 = load_sound("piano_c7.wav").await.unwrap();

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

    let elasticity = 0.99;

    Scene {
        balls: vec![
            Box::new(TrackedBall::new(
                "Fireball".to_string(),
                RED,
                dvec2(
                    screen_width() as f64 * 0.5 + rand::gen_range(-1.0, 1.0),
                    16.0,
                ),
                dvec2(1000.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(YELLOW, RED),
                piano_c6,
            )),
            Box::new(TrackedBall::new(
                "Deep Blue".to_string(),
                BLUE,
                dvec2(
                    screen_width() as f64 * 0.5 + rand::gen_range(-1.0, 1.0),
                    48.0,
                ),
                dvec2(1000.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(BLUE, BLACK),
                piano_e6,
            )),
            Box::new(TrackedBall::new(
                "White Light".to_string(),
                WHITE,
                dvec2(
                    screen_width() as f64 * 0.5 + rand::gen_range(-1.0, 1.0),
                    80.0,
                ),
                dvec2(1000.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(WHITE, WHITE),
                piano_g6,
            )),
            Box::new(TrackedBall::new(
                "Green Machine".to_string(),
                GREEN,
                dvec2(
                    screen_width() as f64 * 0.5 + rand::gen_range(-1.0, 1.0),
                    112.0,
                ),
                dvec2(1000.0, 0.0),
                8.0,
                elasticity,
                TailDrawer::new(GREEN, BLACK),
                piano_c7,
            )),
        ],
        walls,
        winners: Vec::new(),
    }
}
