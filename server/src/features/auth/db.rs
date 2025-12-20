use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::errors::Error;

use super::Role;

pub enum TypedRole {
    Admin,
    Owner,
    User(i64),
}

#[derive(Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip)]
    pub password_hash: String,
    pub role: Role,
    #[serde(skip)]
    pub parent_id: Option<i64>,
    pub created_at: DateTime<Utc>,
}

impl User {
    // pub async fn exists(pool: &PgPool, username: &str) -> Result<bool, Error> {
    //     let res = sqlx::query!("SELECT id FROM users WHERE username = $1", username)
    //         .fetch_optional(pool)
    //         .await?;
    //     Ok(res.is_some())
    // }

    pub async fn create(
        pool: &PgPool,
        username: &str,
        password_hash: &str,
        role: TypedRole,
    ) -> Result<Self, Error> {
        let mut tx = pool.begin().await?;

        let (role, parent_id) = match role {
            TypedRole::Admin => (Role::Admin, None),
            TypedRole::Owner => (Role::Owner, None),
            TypedRole::User(id) => (Role::User, Some(id)),
        };

        let user = sqlx::query_as!(
            Self,
            r#"INSERT INTO users (username, password_hash, role, parent_id)
               VALUES ($1, $2, $3, $4)
               RETURNING id, username, password_hash, role AS "role: Role",
                         parent_id, created_at"#,
            username,
            password_hash,
            role as Role,
            parent_id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Self, Error> {
        sqlx::query_as!(
            Self,
            r#"SELECT id, username, password_hash, role AS "role: Role",
                      parent_id, created_at
               FROM users WHERE id = $1"#,
            id,
        )
        .fetch_optional(pool)
        .await?
        .ok_or(Error::UserNotFound)
    }
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Self, Error> {
        sqlx::query_as!(
            Self,
            r#"SELECT id, username, password_hash, role AS "role: Role",
                      parent_id, created_at
               FROM users WHERE username = $1"#,
            username,
        )
        .fetch_optional(pool)
        .await?
        .ok_or(Error::UserNotFound)
    }

    pub async fn update<F>(pool: &PgPool, id: i64, func: F) -> Result<Self, Error>
    where
        F: FnOnce(&mut Self) -> Result<bool, Error>,
    {
        let mut tx = pool.begin().await?;

        let mut user = sqlx::query_as!(
            Self,
            r#"SELECT id, username, password_hash, role AS "role: Role",
                      parent_id, created_at
               FROM users WHERE id = $1 FOR UPDATE"#,
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        let updated = func(&mut user)?;

        if !updated {
            return Ok(user);
        }

        let user = sqlx::query_as!(
            Self,
            r#"UPDATE users 
               SET username = $1, password_hash = $2, role = $3, parent_id = $4
               WHERE id = $5
               RETURNING id, username, password_hash, role AS "role: Role", 
                         parent_id, created_at"#,
            user.username,
            user.password_hash,
            user.role as Role,
            user.parent_id,
            user.id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), Error> {
        let mut tx = pool.begin().await?;

        let role = sqlx::query_scalar!(
            r#"SELECT role AS "role: Role" FROM users WHERE id = $1 FOR UPDATE"#,
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        if role == Role::Admin {
            let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE role = 'admin'")
                .fetch_one(&mut *tx)
                .await?;
            if count == Some(1) {
                return Err(Error::CannotDeleteLastAdmin);
            }
        }

        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
