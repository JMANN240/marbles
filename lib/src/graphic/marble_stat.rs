use api::marble::Marble;
use database::marble::DbMarble;
use glam::DVec2;
use keyframe::AnimationSequence;
use mint::Vector2;
use palette::Srgba;
use rand::{RngExt, rng};
use render_agnostic::Renderer;
use sqlx::SqlitePool;

use crate::{graphic::Graphic, username::generate_username};

#[derive(Clone)]
pub struct MarbleStat {
    pub time: f64,
    pub marble: Marble,
    wins: usize,
    pub origin: AnimationSequence<Vector2<f64>>,
    pub viewport: (f64, f64),
    pub maybe_sponsor_name: Option<String>,
}

impl MarbleStat {
    pub async fn new(
        pool: &SqlitePool,
        origin: AnimationSequence<Vector2<f64>>,
        marble_name: String,
        viewport: (f64, f64),
    ) -> Self {
        let db_marble = DbMarble::get_by_name(pool, &marble_name)
            .await
            .unwrap()
            .unwrap();

        let db_race_marbles = db_marble.get_race_marbles(pool).await.unwrap();

        Self {
            time: 0.0,
            marble: Marble::from(db_marble),
            wins: db_race_marbles
                .iter()
                .filter(|db_race_marble| db_race_marble.place == 1)
                .count(),
            origin,
            viewport,
            maybe_sponsor_name: rng()
                .random_bool(0.2)
                .then_some(generate_username(&mut rng())),
        }
    }
}

impl Graphic for MarbleStat {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.render_text_outline(
            &format!("{}, {} wins", self.marble.name, self.wins),
            self.origin(),
            anchor2d::CGC,
            48.0,
            1.0,
            self.marble.color,
            Srgba::new(0.0, 0.0, 0.0, 1.0),
        );

        renderer.render_text_outline(
            &if let Some(sponsor_name) = &self.maybe_sponsor_name {
                format!("Sponsored by {}!", sponsor_name)
            } else {
                format!("Sponsor {}, link in bio!", self.marble.name)
            },
            self.origin() + DVec2::Y * 36.0,
            anchor2d::CGC,
            24.0,
            1.0,
            Srgba::new(1.0, 1.0, 1.0, 1.0),
            Srgba::new(0.0, 0.0, 0.0, 1.0),
        );
    }

    fn origin_sequence(&self) -> &AnimationSequence<Vector2<f64>> {
        &self.origin
    }

    fn origin_sequence_mut(&mut self) -> &mut AnimationSequence<Vector2<f64>> {
        &mut self.origin
    }

    fn time(&self) -> f64 {
        self.time
    }

    fn set_time(&mut self, new_time: f64) {
        self.time = new_time;
    }

    fn visible(&self) -> bool {
        matches!(self.origin_sequence().pair(), (Some(_), Some(_)))
    }
}
