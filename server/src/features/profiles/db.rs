use chrono::{
    DateTime,
    NaiveTime,
    Utc,
};
use garde::{
    Valid,
    Validate,
};
use serde::{
    Deserialize,
    Serialize,
};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::errors::Error;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Profile {
    pub id: i64,
    pub name: String,

    pub owner_id: i64,
    /// Whether the sub-users will see the profile
    pub is_shared: bool,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    pub timezone: String, // TODO: chrono-tz

    #[schema(value_type = String, example = "22:00:00")]
    pub sleep_start: NaiveTime,
    #[schema(value_type = String, example = "07:00:00")]
    pub sleep_end: NaiveTime,

    pub night_mode_enabled: bool,
    pub min_color_temp: i32,
    pub max_color_temp: i32,
    pub motion_timeout_seconds: i32,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[garde(allow_unvalidated)]
#[schema(as = CreateProfileRequest)]
pub struct CreateProfile {
    #[garde(length(chars, min = 1))]
    #[schema(min_length = 1, example = "Living Room")]
    pub name: String,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    /// Whether the sub-users will see the profile
    #[schema(default = true)]
    pub is_shared: bool,

    #[schema(default = "UTC", example = "Europe/Kyiv")]
    pub timezone: String,

    #[schema(value_type = String, example = "22:00:00")]
    pub sleep_start: NaiveTime,

    #[schema(value_type = String, example = "07:00:00")]
    pub sleep_end: NaiveTime,

    pub night_mode_enabled: bool,

    #[garde(range(min = 1800, max = 10000))]
    #[schema(default = 2000, minimum = 1800, maximum = 10000)]
    pub min_color_temp: i32,

    #[garde(range(min = self.min_color_temp, max = 10000))]
    #[schema(default = 6500, minimum = 1800, maximum = 10000)]
    pub max_color_temp: i32,

    #[schema(default = 300)]
    pub motion_timeout_seconds: i32,
}

impl Profile {
    pub async fn create(
        pool: &PgPool,
        owner_id: i64,
        data: Valid<CreateProfile>,
    ) -> Result<Self, Error> {
        let profile = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO profiles (
                owner_id, name, latitude, longitude, timezone,
                sleep_start, sleep_end, night_mode_enabled,
                min_color_temp, max_color_temp, motion_timeout_seconds
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
            owner_id,
            data.name,
            data.latitude,
            data.longitude,
            data.timezone,
            data.sleep_start,
            data.sleep_end,
            data.night_mode_enabled,
            data.min_color_temp,
            data.max_color_temp,
            data.motion_timeout_seconds
        )
        .fetch_one(pool)
        .await?;

        Ok(profile)
    }

    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Self, Error> {
        sqlx::query_as!(Self, "SELECT * FROM profiles WHERE id = $1", id)
            .fetch_optional(pool)
            .await?
            .ok_or(Error::ProfileNotFound)
    }

    pub async fn list_as_owner(pool: &PgPool, owner_id: i64) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM profiles WHERE owner_id = $1 OR (
                owner_id IN (SELECT id FROM users WHERE parent_id = $1)
                AND is_shared = true
             ) ORDER BY created_at DESC",
            owner_id
        )
        .fetch_all(pool)
        .await?)
    }
    pub async fn list_as_user(
        pool: &PgPool,
        user_id: i64,
        parent_id: i64,
    ) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM profiles WHERE owner_id = $1 OR (
                owner_id = $2 AND is_shared = true
             ) ORDER BY created_at DESC",
            user_id,
            parent_id,
        )
        .fetch_all(pool)
        .await?)
    }

    async fn get_by_id_for_update(conn: &mut sqlx::PgConnection, id: i64) -> Result<Self, Error> {
        sqlx::query_as!(Self, "SELECT * FROM profiles WHERE id = $1 FOR UPDATE", id)
            .fetch_optional(conn)
            .await?
            .ok_or(Error::ProfileNotFound)
    }

    pub async fn update<F>(pool: &PgPool, id: i64, func: F) -> Result<Self, Error>
    where
        F: FnOnce(&mut Self) -> Result<bool, Error>,
    {
        let mut tx = pool.begin().await?;

        let mut profile = Self::get_by_id_for_update(&mut tx, id).await?;

        let updated = func(&mut profile)?;

        if !updated {
            return Ok(profile);
        }

        // TODO: Validation?
        let profile = sqlx::query_as!(
            Self,
            r#"
            UPDATE profiles
            SET
                name = $1,
                latitude = $2,
                longitude = $3,
                timezone = $4,
                sleep_start = $5,
                sleep_end = $6,
                night_mode_enabled = $7,
                min_color_temp = $8,
                max_color_temp = $9,
                motion_timeout_seconds = $10
            WHERE id = $11
            RETURNING *
            "#,
            profile.name,
            profile.latitude,
            profile.longitude,
            profile.timezone,
            profile.sleep_start,
            profile.sleep_end,
            profile.night_mode_enabled,
            profile.min_color_temp,
            profile.max_color_temp,
            profile.motion_timeout_seconds,
            profile.id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(profile)
    }

    pub async fn delete<F>(pool: &PgPool, id: i64, func: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Self) -> Result<bool, Error>,
    {
        let mut tx = pool.begin().await?;

        let mut profile = Self::get_by_id_for_update(&mut tx, id).await?;

        let delete = func(&mut profile)?;

        if !delete {
            return Ok(());
        }

        sqlx::query!("DELETE FROM profiles WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    pub fn patch(&mut self, new: CreateProfile) -> bool {
        // TODO: this begs for refactor
        #[allow(clippy::useless_let_if_seq)]
        let mut updated = false;

        if self.name != new.name {
            self.name = new.name;
            updated = true;
        }
        if self.is_shared != new.is_shared {
            self.is_shared = new.is_shared;
            updated = true;
        }
        if self.latitude != new.latitude {
            self.latitude = new.latitude;
            updated = true;
        }
        if self.longitude != new.longitude {
            self.longitude = new.longitude;
            updated = true;
        }
        if self.timezone != new.timezone {
            self.timezone = new.timezone;
            updated = true;
        }
        if self.sleep_start != new.sleep_start {
            self.sleep_start = new.sleep_start;
            updated = true;
        }
        if self.sleep_end != new.sleep_end {
            self.sleep_end = new.sleep_end;
            updated = true;
        }
        if self.night_mode_enabled != new.night_mode_enabled {
            self.night_mode_enabled = new.night_mode_enabled;
            updated = true;
        }
        if self.min_color_temp != new.min_color_temp {
            self.min_color_temp = new.min_color_temp;
            updated = true;
        }
        if self.max_color_temp != new.max_color_temp {
            self.max_color_temp = new.max_color_temp;
            updated = true;
        }
        if self.motion_timeout_seconds != new.motion_timeout_seconds {
            self.motion_timeout_seconds = new.motion_timeout_seconds;
            updated = true;
        }

        updated
    }
}
