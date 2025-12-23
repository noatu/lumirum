use axum::{
    Json,
    extract::State,
};
use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;
use utoipa::{
    IntoResponses,
    ToSchema,
};

use crate::{
    AppState,
    errors::Error,
    features::auth::{
        AdminAuthenticated,
        Authenticated,
        Role,
    },
    responses::StatsResponse,
};

/// Stats of the server
#[derive(Serialize, ToSchema, IntoResponses)]
#[response(status = OK)]
pub struct Stats {
    users: i64,
    profiles: i64,
    devices: i64,
    telemetry: i64,
    timestamp: DateTime<Utc>,
}

/// Get stats of the server
#[utoipa::path(get, path = "/stats", responses(StatsResponse), tag = super::TAG)]
pub async fn stats(
    State(state): State<AppState>,
    AdminAuthenticated(_auth): AdminAuthenticated,
) -> Result<Json<Stats>, Error> {
    Ok(Json(Stats {
        users: sqlx::query_scalar!("SELECT COUNT(id) AS \"c!\" FROM users")
            .fetch_one(&state.pool)
            .await?,
        profiles: sqlx::query_scalar!("SELECT COUNT(id) AS \"c!\" FROM profiles")
            .fetch_one(&state.pool)
            .await?,
        devices: sqlx::query_scalar!("SELECT COUNT(id) AS \"c!\" FROM devices")
            .fetch_one(&state.pool)
            .await?,
        telemetry: sqlx::query_scalar!("SELECT COUNT(id) AS \"c!\" FROM telemetry")
            .fetch_one(&state.pool)
            .await?,
        timestamp: Utc::now(),
    }))
}
