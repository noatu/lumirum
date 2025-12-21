use chrono::{
    DateTime,
    Utc,
};
use garde::{
    Valid,
    Validate,
};
use rand::{
    Rng,
    distr::Alphanumeric,
};
use serde::{
    Deserialize,
    Serialize,
};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::errors::Error;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Device {
    pub id: i64,
    pub name: String,
    pub secret_key: String,

    pub profile_id: Option<i64>,
    pub owner_id: i64,
    pub is_public: bool,

    pub firmware_version: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[garde(allow_unvalidated)]
#[schema(as = CreateDeviceRequest)]
pub struct CreateDevice {
    #[garde(length(chars, min = 1, max = 200))]
    #[schema(min_length = 1, max_length = 200, example = "Kitchen Light")]
    pub name: String,

    #[schema(example = 1)]
    pub profile_id: Option<i64>,

    #[schema(default = true)]
    pub is_public: bool,
}

impl Device {
    /// Generates a random 32-character alphanumeric key
    fn generate_key() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    }

    pub async fn create(
        pool: &PgPool,
        owner_id: i64,
        data: Valid<CreateDevice>,
    ) -> Result<Self, Error> {
        Ok(sqlx::query_as!(
            Self,
            "INSERT INTO devices (owner_id, name, profile_id, is_public, secret_key)
            VALUES ($1, $2, $3, $4, $5) RETURNING *",
            owner_id,
            data.name,
            data.profile_id,
            data.is_public,
            Self::generate_key()
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Self, Error> {
        sqlx::query_as!(Self, "SELECT * FROM devices WHERE id = $1", id)
            .fetch_optional(pool)
            .await?
            .ok_or(Error::DeviceNotFound)
    }

    pub async fn list_owner_devices(pool: &PgPool, owner_id: i64) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM devices WHERE owner_id = $1
               OR owner_id IN (
                   SELECT id
                   FROM users
                   WHERE parent_id = $1
               )
            ORDER BY created_at DESC",
            owner_id
        )
        .fetch_all(pool)
        .await?)
    }
    pub async fn list_user_devices(
        pool: &PgPool,
        user_id: i64,
        parent_id: i64,
    ) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM devices
             WHERE owner_id = $1 OR ( owner_id = $2 AND is_public = true )
             ORDER BY created_at DESC",
            user_id,
            parent_id,
        )
        .fetch_all(pool)
        .await?)
    }

    /// Transactional update helper
    pub async fn update<F>(pool: &PgPool, id: i64, func: F) -> Result<Self, Error>
    where
        F: FnOnce(&mut Self) -> Result<bool, Error>,
    {
        let mut tx = pool.begin().await?;

        let mut device =
            sqlx::query_as!(Self, "SELECT * FROM devices WHERE id = $1 FOR UPDATE", id)
                .fetch_optional(&mut *tx)
                .await?
                .ok_or(Error::DeviceNotFound)?;

        let updated = func(&mut device)?;

        if !updated {
            return Ok(device);
        }

        //     pub name: String,
        //     pub secret_key: String,
        //     pub profile_id: Option<i64>,
        //     pub is_public: bool,
        //     pub firmware_version: Option<String>,
        //     pub last_seen: Option<DateTime<Utc>>,

        let device = sqlx::query_as!(
            Self,
            "UPDATE devices SET
                name = $1,
                secret_key = $2,
                profile_id = $3,
                is_public = $4,
                firmware_version = $5,
                last_seen = $6
             WHERE id = $7 RETURNING *",
            device.name,
            device.secret_key,
            device.profile_id,
            device.is_public,
            device.firmware_version,
            device.last_seen,
            device.id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(device)
    }

    pub async fn delete<F>(pool: &PgPool, id: i64, func: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Self) -> Result<bool, Error>,
    {
        let mut tx = pool.begin().await?;

        // We select for update to ensure concurrency safety during permission checks
        let mut device = sqlx::query_as!(
            Self,
            r#"SELECT
                id, owner_id, profile_id, name, secret_key, is_public,
                firmware_version, last_seen, created_at
               FROM devices WHERE id = $1 FOR UPDATE"#,
            id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(Error::ProfileNotFound)?;

        let delete = func(&mut device)?;

        if !delete {
            return Ok(());
        }

        sqlx::query!("DELETE FROM devices WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    /// Applies changes from the DTO. Returns true if anything changed.
    pub fn patch(&mut self, new: CreateDevice) -> bool {
        #[allow(clippy::useless_let_if_seq)]
        let mut updated = false;

        if self.name != new.name {
            self.name = new.name;
            updated = true;
        }
        if self.profile_id != new.profile_id {
            self.profile_id = new.profile_id;
            updated = true;
        }
        if self.is_public != new.is_public {
            self.is_public = new.is_public;
            updated = true;
        }

        updated
    }

    pub fn regenerate_key(&mut self) -> bool {
        self.secret_key = Self::generate_key();
        true
    }
}
