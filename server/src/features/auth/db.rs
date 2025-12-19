use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::errors::Error;

use super::Role;

#[derive(Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip)]
    pub password_hash: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn count(pool: &PgPool) -> Result<i64, Error> {
        Ok(sqlx::query_scalar!(r#"SELECT COUNT(*) as "c!" FROM users"#)
            .fetch_one(pool)
            .await?)
    }

    pub async fn read_by_id(pool: &PgPool, id: i64) -> Result<Self, Error> {
        sqlx::query_as!(
        Self,
        r#"SELECT id, username, password_hash, role AS "role: Role", created_at FROM users WHERE id = $1"#,
        id,
    )
    .fetch_optional(pool)
    .await?.ok_or(Error::UserNotFound)
    }

    pub async fn read_by_username(pool: &PgPool, username: &str) -> Result<Self, Error> {
        sqlx::query_as!(
        Self,
        r#"SELECT id, username, password_hash, role AS "role: Role", created_at FROM users WHERE username = $1"#,
        username,
    )
    .fetch_optional(pool)
    .await?.ok_or(Error::UserNotFound)
    }

    pub async fn exits(pool: &PgPool, username: &str) -> Result<bool, Error> {
        let res = sqlx::query!("SELECT FROM users WHERE username = $1", username)
            .fetch_optional(pool)
            .await?;
        Ok(res.is_some())
    }

    pub async fn create(
        pool: &PgPool,
        username: String,
        password_hash: String,
        role: Role,
    ) -> Result<Self, Error> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3)
                RETURNING id, username, password_hash, role AS "role: Role", created_at"#,
            username,
            password_hash,
            role as Role
        )
        .fetch_one(pool)
        .await
        .map_err(Error::from)
    }

    pub async fn update_password(
        &mut self,

        pool: &PgPool,
        password_hash: String,
    ) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE users SET password_hash = $1 WHERE id = $2",
            password_hash,
            self.id
        )
        .execute(pool)
        .await?;
        self.password_hash = password_hash;
        Ok(())
    }
}
