use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use api::User;
use sqlx::{QueryBuilder, SqlitePool, prelude::FromRow, query_as};

#[derive(Clone, FromRow)]
pub struct DbUser {
    pub id: i64,
    pub username: String,
    pub passhash: String,
}

impl DbUser {
    pub async fn insert(pool: &SqlitePool, username: &str, password: &str) -> sqlx::Result<DbUser> {
        let salt = SaltString::generate(&mut OsRng);

        let passhash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        query_as!(
            Self,
            "INSERT INTO user (username, passhash) VALUES (?, ?) RETURNING *",
            username,
            passhash,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<DbUser>> {
        query_as!(DbUser, "SELECT * FROM user WHERE id = ?", id,)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_by_ids(pool: &SqlitePool, ids: &[i64]) -> sqlx::Result<Vec<DbUser>> {
        let mut query_builder = QueryBuilder::new("SELECT * FROM user WHERE id IN (");

        let mut separated = query_builder.separated(", ");
        for id in ids {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");

        let query_as = query_builder.build_query_as();

        query_as.fetch_all(pool).await
    }

    pub async fn get_by_username(
        pool: &SqlitePool,
        username: &str,
    ) -> sqlx::Result<Option<DbUser>> {
        query_as!(DbUser, "SELECT * FROM user WHERE username = ?", username,)
            .fetch_optional(pool)
            .await
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let passhash = PasswordHash::new(&self.passhash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &passhash)
            .is_ok()
    }
}

impl From<DbUser> for User {
    fn from(value: DbUser) -> Self {
        Self {
            id: value.id,
            username: value.username,
        }
    }
}
