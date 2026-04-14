use chrono::TimeDelta;
use sqlx::{SqlitePool, query_as};

pub struct DbRaceMarble {
    pub race_id: i64,
    pub marble_id: i64,
    pub time: f64,
    pub place: i64,
}

impl DbRaceMarble {
    pub async fn get_by_race_id(pool: &SqlitePool, race_id: i64) -> sqlx::Result<Vec<Self>> {
        query_as!(
            Self,
            "SELECT * FROM race_marble WHERE race_id = ? ORDER BY time ASC",
            race_id,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_by_marble_id(pool: &SqlitePool, marble_id: i64) -> sqlx::Result<Vec<Self>> {
        query_as!(
            Self,
            "SELECT * FROM race_marble WHERE marble_id = ?",
            marble_id,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn insert(
        pool: &SqlitePool,
        race_id: i64,
        marble_id: i64,
        time: TimeDelta,
        place: i64,
    ) -> sqlx::Result<Self> {
        let seconds = time.as_seconds_f64();

        query_as!(
            Self,
            "INSERT INTO race_marble VALUES (?, ?, ?, ?) RETURNING *",
            race_id,
            marble_id,
            seconds,
            place,
        )
        .fetch_one(pool)
        .await
    }
}
