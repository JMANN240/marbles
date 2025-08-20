use glam::{dvec2, DVec2};

use crate::{level::{BallSpace, Level, PowerupSpace}, wall::{circle_wall::CircleWall, straight_wall::{Line, StraightWall}, Wall}};

pub fn level_1(n: usize, scene_width: f64, scene_height: f64) -> Level {
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

    Level::new(
        BallSpace::spaced_evenly(n, dvec2(0.0, 50.0), dvec2(scene_width, 50.0), DVec2::ZERO),
        vec![
            PowerupSpace::new(dvec2(32.0, 258.0)),
            PowerupSpace::new(dvec2(scene_width - 32.0, 258.0)),
        ],
        walls,
    )
}

pub fn level_2(n: usize, scene_width: f64, scene_height: f64) -> Level {
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

    Level::new(
        BallSpace::spaced_evenly(n, dvec2(0.0, 50.0), dvec2(scene_width, 50.0), DVec2::ZERO),
        Vec::new(),
        walls,
    )
}

pub fn level_3(n: usize, scene_width: f64, scene_height: f64) -> Level {
    let walls: Vec<Box<dyn Wall>> = StraightWall::rect(0.0, 0.0, scene_width, scene_height, true)
        .into_iter()
        .map(|straight_wall| Box::new(straight_wall) as Box<dyn Wall>)
        .collect();

    Level::new(
        BallSpace::spaced_evenly(n, dvec2(0.0, 50.0), dvec2(scene_width, 50.0), DVec2::ZERO),
        Vec::new(),
        walls,
    )
}

pub fn level_4(n: usize, scene_width: f64, scene_height: f64) -> Level {
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

    Level::new(
        BallSpace::spaced_evenly(
            n,
            dvec2(scene_width / 2.0, 0.0),
            dvec2(scene_width / 2.0, n as f64 * 100.0),
            DVec2::ZERO,
        ),
        Vec::new(),
        walls,
    )
}

pub fn level_5(n: usize, scene_width: f64, scene_height: f64) -> Level {
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

    Level::new(
        BallSpace::spaced_evenly(n, dvec2(0.0, 50.0), dvec2(scene_width, 50.0), DVec2::ZERO),
        Vec::new(),
        walls,
    )
}

pub fn level_6(n: usize, scene_width: f64, scene_height: f64) -> Level {
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

    Level::new(
        BallSpace::spaced_evenly(n, dvec2(0.0, 50.0), dvec2(scene_width, 50.0), dvec2(100.0, 0.0)),
        Vec::new(),
        walls,
    )
}
