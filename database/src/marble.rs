use std::path::PathBuf;

use palette::Srgba;
use sqlx::{SqlitePool, query_as};

use api::marble::{Marble, WriteMarble};

use crate::race_marble::DbRaceMarble;

#[derive(Debug, Clone)]
pub struct DbMarble {
    pub id: i64,
    pub name: String,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub radius: f64,
    pub density: f64,
    pub elasticity: f64,
    pub sound: String,
    pub maybe_image: Option<String>,
    pub active: i64,
}

impl DbMarble {
    pub async fn get_all(pool: &SqlitePool) -> sqlx::Result<Vec<Self>> {
        query_as!(Self, "SELECT * FROM marble")
            .fetch_all(pool)
            .await
    }

    pub async fn get_all_active(pool: &SqlitePool) -> sqlx::Result<Vec<Self>> {
        query_as!(Self, "SELECT * FROM marble WHERE active = TRUE")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_name(pool: &SqlitePool, name: &str) -> sqlx::Result<Option<Self>> {
        query_as!(Self, "SELECT * FROM marble WHERE name = ?", name,)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_by_league(pool: &SqlitePool, league_name: &str) -> sqlx::Result<Vec<Self>> {
        query_as!(Self, "SELECT marble.* FROM marble INNER JOIN league_marble ON marble.id = league_marble.marble_id INNER JOIN league ON league_marble.league_id = league.id WHERE league.name = ?", league_name)
            .fetch_all(pool)
            .await
    }

    pub async fn upsert_by_id(
        pool: &SqlitePool,
        id: i64,
        write_marble: WriteMarble,
    ) -> sqlx::Result<Self> {
        let sound_path_str = write_marble.sound_path.to_str().unwrap();
        let maybe_image_path_string = write_marble
            .maybe_image_path
            .map(|image_path| image_path.to_str().unwrap().to_string());

        query_as!(
            Self,
            r#"INSERT INTO marble
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT DO UPDATE SET
            name = ?,
            red = ?,
            green = ?,
            blue = ?,
            radius = ?,
            density = ?,
            elasticity = ?,
            sound = ?,
            maybe_image = ?,
            active = ?
            RETURNING *"#,
            id,
            write_marble.name,
            write_marble.color.red,
            write_marble.color.green,
            write_marble.color.blue,
            write_marble.radius,
            write_marble.density,
            write_marble.elasticity,
            sound_path_str,
            maybe_image_path_string,
            write_marble.active,
            write_marble.name,
            write_marble.color.red,
            write_marble.color.green,
            write_marble.color.blue,
            write_marble.radius,
            write_marble.density,
            write_marble.elasticity,
            sound_path_str,
            maybe_image_path_string,
            write_marble.active,
        )
        .fetch_one(pool)
        .await
    }

    // pub async fn insert(pool: &SqlitePool, id: i64) -> sqlx::Result<Self> {
    //     query_as!(Self, "INSERT INTO race VALUES (?) RETURNING *", id)
    //         .fetch_one(pool)
    //         .await
    // }

    // pub async fn insert_participant(
    //     &self,
    //     pool: &SqlitePool,
    //     name: String,
    //     time: TimeDelta,
    // ) -> sqlx::Result<DbRaceParticipant> {
    //     DbRaceParticipant::insert(pool, self.id, name, time).await
    // }

    pub async fn get_race_marbles(&self, pool: &SqlitePool) -> sqlx::Result<Vec<DbRaceMarble>> {
        DbRaceMarble::get_by_marble_id(pool, self.id).await
    }
}

impl From<DbMarble> for Marble {
    fn from(value: DbMarble) -> Self {
        Marble {
            id: value.id,
            name: value.name,
            color: Srgba::new(value.red as f32, value.green as f32, value.blue as f32, 1.0),
            radius: value.radius,
            density: value.density,
            elasticity: value.elasticity,
            sound_path: PathBuf::from(value.sound),
            maybe_image_path: value.maybe_image.map(PathBuf::from),
            active: value.active > 0,
        }
    }
}
