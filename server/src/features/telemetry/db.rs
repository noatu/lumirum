use chrono::{
    DateTime,
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

#[derive(Debug, Serialize, ToSchema)]
pub struct Telemetry {
    pub id: i64,
    pub device_id: i64,

    pub event_type: String,
    pub motion_detected: Option<bool>,
    pub light_is_on: Option<bool>,
    pub brightness: Option<i16>,
    pub color_temp: Option<i16>,
    pub ambient_light: Option<i16>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[garde(allow_unvalidated)]
#[schema(as = CreateTelemetryRequest)]
pub struct CreateTelemetry {
    #[schema(example = "motion_detected")]
    pub event_type: String,

    pub motion_detected: Option<bool>,
    pub light_is_on: Option<bool>,

    #[garde(range(min = 0, max = 100))]
    #[schema(minimum = 0, maximum = 100)]
    pub brightness: Option<i16>,

    #[garde(range(min = 1800, max = 10000))]
    #[schema(minimum = 1800, maximum = 10000)]
    pub color_temp: Option<i16>,

    pub ambient_light: Option<i16>,
}

impl Telemetry {
    /// Create a new telemetry entry
    pub async fn create(
        pool: &PgPool,
        device_id: i64,
        data: Valid<CreateTelemetry>,
    ) -> Result<Self, Error> {
        let telemetry = sqlx::query_as!(
            Self,
            "INSERT INTO telemetry (
                device_id, event_type, motion_detected, light_is_on, brightness,
                color_temp, ambient_light
             ) VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING *
            ",
            device_id,
            data.event_type,
            data.motion_detected,
            data.light_is_on,
            data.brightness,
            data.color_temp,
            data.ambient_light
        )
        .fetch_one(pool)
        .await?;

        Ok(telemetry)
    }

    pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Self, Error> {
        sqlx::query_as!(Self, "SELECT * FROM telemetry WHERE id = $1", id)
            .fetch_optional(pool)
            .await?
            .ok_or(Error::TelemetryNotFound)
    }

    /// Get telemetry for a specific device
    pub async fn list(
        pool: &PgPool,
        device_id: i64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM telemetry
             WHERE device_id = $1 AND created_at >= $2 AND created_at < $3
             ORDER BY created_at DESC",
            device_id,
            start,
            end
        )
        .fetch_all(pool)
        .await?)
    }

    /// Get telemetry for owner's devices and their users' public devices
    pub async fn list_as_owner(
        pool: &PgPool,
        owner_id: i64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT t.* FROM telemetry t
             INNER JOIN devices d ON d.id = t.device_id
             WHERE (d.owner_id = $1 OR (
                d.owner_id IN (SELECT id FROM users WHERE parent_id = $1) AND d.is_public = true)
             ) AND t.created_at >= $2 AND t.created_at < $3
             ORDER BY t.created_at DESC",
            owner_id,
            start,
            end
        )
        .fetch_all(pool)
        .await?)
    }

    /// Get telemetry for user's devices and their parent's public devices
    pub async fn list_as_user(
        pool: &PgPool,
        user_id: i64,
        parent_id: i64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Self>, Error> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT t.* FROM telemetry t
             INNER JOIN devices d ON d.id = t.device_id
             WHERE (d.owner_id = $1 OR (d.owner_id = $2 AND d.is_public = true))
                AND t.created_at >= $3 AND t.created_at < $4
             ORDER BY t.created_at DESC",
            user_id,
            parent_id,
            start,
            end
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn delete(
        pool: &PgPool,
        device_id: i64,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<u64, Error> {
        Ok(sqlx::query!(
            "DELETE FROM telemetry WHERE device_id = $1 AND created_at >= $2 AND created_at < $3",
            device_id,
            start,
            end
        )
        .execute(pool)
        .await?
        .rows_affected())
    }
}
