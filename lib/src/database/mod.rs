use chrono::TimeDelta;
use sqlx::{SqlitePool, query_as};

pub struct DbRace {
    pub id: i64
}

impl DbRace {
    pub async fn insert(pool: &SqlitePool, id: i64) -> sqlx::Result<Self> {
        query_as!(
            Self,
            "INSERT INTO race VALUES (?) RETURNING *",
            id,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn insert_participant(&self, pool: &SqlitePool, name: String, time: TimeDelta) -> sqlx::Result<DbRaceParticipant> {
        DbRaceParticipant::insert(pool, self.id, name, time).await
    }
}

pub struct DbRaceParticipant {
    pub id: i64,
    pub name: String,
    pub time: f64,
}

impl DbRaceParticipant {
    pub async fn insert(pool: &SqlitePool, id: i64, name: String, time: TimeDelta) -> sqlx::Result<Self> {
        let seconds = time.as_seconds_f64();

        query_as!(
            Self,
            "INSERT INTO race_participant VALUES (?, ?, ?) RETURNING *",
            id,
            name,
            seconds,
        )
        .fetch_one(pool)
        .await
    }
}
