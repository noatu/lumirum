use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};
use sqlx::{
    PgPool,
    Type,
};
use utoipa::ToSchema;

use crate::errors::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum Role {
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
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
enum DbRole {
    Admin,
    Owner,
    User,
}
struct DbUser {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: DbRole,
    pub parent_id: Option<i64>,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<DbUser> for User {
    type Error = Error;

    fn try_from(value: DbUser) -> Result<Self, Self::Error> {
        let role = match value.role {
            DbRole::Admin => Role::Admin,
            DbRole::Owner => Role::Owner,
            DbRole::User => Role::User(value.parent_id.ok_or(Error::DataCorruption(format!(
                "user {} has role User, but no parent_id set!",
                value.id
            )))?),
        };
        Ok(Self {
            id: value.id,
            username: value.username,
            password_hash: value.password_hash,
            role,
            created_at: value.created_at,
        })
    }
}

impl From<User> for DbUser {
    fn from(value: User) -> Self {
        let (role, parent_id) = match value.role {
            Role::Admin => (DbRole::Admin, None),
            Role::Owner => (DbRole::Owner, None),
            Role::User(id) => (DbRole::User, Some(id)),
        };
        Self {
            id: value.id,
            username: value.username,
            password_hash: value.password_hash,
            role,
            parent_id,
            created_at: value.created_at,
        }
    }
}

impl User {
    pub async fn create(
        pool: &PgPool,
        username: &str,
        password_hash: &str,
        role: Role,
    ) -> Result<Self, Error> {
        let (role, parent_id) = match role {
            Role::Admin => (DbRole::Admin, None),
            Role::Owner => (DbRole::Owner, None),
            Role::User(id) => (DbRole::User, Some(id)),
        };

        let user = sqlx::query_as!(
            DbUser,
            r#"INSERT INTO users (username, password_hash, role, parent_id)
               VALUES ($1, $2, $3, $4)
               RETURNING id, username, password_hash, role AS "role: DbRole",
                         parent_id, created_at"#,
            username,
            password_hash,
            role as DbRole,
            parent_id
        )
        .fetch_one(pool)
        .await?;

        user.try_into()
    }

    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Self, Error> {
        sqlx::query_as!(
            DbUser,
            r#"SELECT id, username, password_hash, role AS "role: DbRole",
                      parent_id, created_at
               FROM users WHERE id = $1"#,
            id,
        )
        .fetch_optional(pool)
        .await?
        .ok_or(Error::UserNotFound)?
        .try_into()
    }
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Self, Error> {
        sqlx::query_as!(
            DbUser,
            r#"SELECT id, username, password_hash, role AS "role: DbRole",
                      parent_id, created_at
               FROM users WHERE username = $1"#,
            username,
        )
        .fetch_optional(pool)
        .await?
        .ok_or(Error::UserNotFound)?
        .try_into()
    }

    pub async fn update<F>(pool: &PgPool, id: i64, func: F) -> Result<Self, Error>
    where
        F: FnOnce(&mut Self) -> Result<bool, Error>,
    {
        let mut tx = pool.begin().await?;

        let mut user = sqlx::query_as!(
            DbUser,
            r#"SELECT id, username, password_hash, role AS "role: DbRole",
                      parent_id, created_at
               FROM users WHERE id = $1 FOR UPDATE"#,
            id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(Error::UserNotFound)?
        .try_into()?;

        let updated = func(&mut user)?;

        if !updated {
            return Ok(user);
        }

        let user: DbUser = user.into();

        let user = sqlx::query_as!(
            DbUser,
            r#"UPDATE users
               SET username = $1, password_hash = $2, role = $3, parent_id = $4
               WHERE id = $5
               RETURNING id, username, password_hash, role AS "role: DbRole",
                         parent_id, created_at"#,
            user.username,
            user.password_hash,
            user.role as DbRole,
            user.parent_id,
            user.id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        user.try_into()
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), Error> {
        let mut tx = pool.begin().await?;

        let role = sqlx::query_scalar!(
            r#"SELECT role AS "role: DbRole" FROM users WHERE id = $1 FOR UPDATE"#,
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        if role == DbRole::Admin {
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
