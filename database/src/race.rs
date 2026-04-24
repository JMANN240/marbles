use api::race::Race;
use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use sqlx::{SqlitePool, query_as};

use crate::race_marble::DbRaceMarble;

#[derive(Debug, Clone, Copy)]
pub struct DbRace {
    pub id: i64,
    pub level_id: i64,
    pub time: Option<i64>,
}

impl DbRace {
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<Self>> {
        query_as!(Self, "SELECT * FROM race WHERE id = ?", id)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_by_date(pool: &SqlitePool, date: NaiveDate) -> sqlx::Result<Vec<Self>> {
        let date_string = date.format("%F").to_string();

        query_as!(Self, "SELECT * FROM race WHERE DATE(time) == ?", date_string)
            .fetch_all(pool)
            .await
    }

    pub async fn insert(pool: &SqlitePool, id: i64, level_id: i64, time: DateTime<Utc>) -> sqlx::Result<Self> {
        let time = time.timestamp();

        query_as!(
            Self,
            "INSERT INTO race VALUES (?, ?, ?) RETURNING *",
            id,
            level_id,
            time,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn insert_marble(
        &self,
        pool: &SqlitePool,
        marble_id: i64,
        time: TimeDelta,
        place: i64,
    ) -> sqlx::Result<DbRaceMarble> {
        DbRaceMarble::insert(pool, self.id, marble_id, time, place).await
    }

    pub async fn get_marbles(&self, pool: &SqlitePool) -> sqlx::Result<Vec<DbRaceMarble>> {
        DbRaceMarble::get_by_race_id(pool, self.id).await
    }
}

impl From<DbRace> for Race {
    fn from(value: DbRace) -> Self {
        Race {
            id: value.id,
            level_id: value.level_id,
            time: value.time.map(|time| DateTime::from_timestamp_secs(time)).flatten()
        }
    }
}
