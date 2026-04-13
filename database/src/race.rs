use chrono::TimeDelta;
use sqlx::{SqlitePool, query_as};

use crate::race_participant::DbRaceParticipant;

pub struct DbRace {
    pub id: i64,
}

impl DbRace {
    pub async fn insert(pool: &SqlitePool, id: i64) -> sqlx::Result<Self> {
        query_as!(Self, "INSERT INTO race VALUES (?) RETURNING *", id,)
            .fetch_one(pool)
            .await
    }

    pub async fn insert_participant(
        &self,
        pool: &SqlitePool,
        name: String,
        time: TimeDelta,
    ) -> sqlx::Result<DbRaceParticipant> {
        DbRaceParticipant::insert(pool, self.id, name, time).await
    }

    pub async fn get_participants(
        &self,
        pool: &SqlitePool,
    ) -> sqlx::Result<Vec<DbRaceParticipant>> {
        DbRaceParticipant::get_by_race_id(pool, self.id).await
    }
}
