use axum::{
    Json,
    extract::{
        Path,
        Query,
        State,
    },
    http::StatusCode,
};
use chrono::{
    DateTime,
    Utc,
};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use crate::{
    AppState,
    errors::Error,
    extractors::Validated,
    features::{
        auth::{
            Authenticated,
            Role,
            User,
        },
        devices::{
            AuthDevice,
            Device,
        },
    },
    responses::{
        DeleteTelemetry,
        GetDeviceTelemetry,
        GetOneTelemetry,
        GetTelemetry,
        PostTelemetry,
    },
};

mod db;

use db::CreateTelemetry;

pub use db::Telemetry;

pub const TAG: &str = "Telemetry";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get))
        .routes(routes!(get_all, post))
        .routes(routes!(get_by_device, delete))
}

/// Get telemetry entry by ID
///
/// - Admin can get any telemetry.
/// - Owner can get telemetry from their devices and their Users' public devices.
/// - User can get telemetry from their devices and their Owner's public devices.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(GetOneTelemetry),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    auth: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<Telemetry>, Error> {
    let telemetry = Telemetry::get_by_id(&state.pool, id).await?;
    let device = Device::get_by_id(&state.pool, telemetry.device_id).await?;

    Ok(Json(match auth.role {
        Role::Admin => telemetry,
        Role::Owner | Role::User(_) if device.owner_id == auth.id => telemetry,
        Role::User(parent) if device.owner_id == parent && device.is_public => telemetry,
        Role::Owner
            if device.is_public
                && User::is_child(&state.pool, device.owner_id, auth.id).await? =>
        {
            telemetry
        }
        _ => return Err(Error::TelemetryNotFound),
    }))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TelemetryTimeframe {
    /// Start time for telemetry data (RFC3339 format)
    #[param(example = "2025-12-10T00:00:00Z")]
    pub start: DateTime<Utc>,

    /// End time for telemetry data (RFC3339 format)
    #[param(example = "2025-12-31T00:00:00Z")]
    pub end: DateTime<Utc>,
}

/// List all telemetry
///
/// - Owner gets telemetry from their devices and their Users' public devices.
/// - User gets telemetry from their devices and their Owner's public devices.
#[utoipa::path(
    get,
    path = "",
    params(TelemetryTimeframe),
    responses(GetTelemetry),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get_all(
    State(state): State<AppState>,
    auth: Authenticated,
    Query(TelemetryTimeframe { start, end }): Query<TelemetryTimeframe>,
) -> Result<Json<Vec<Telemetry>>, Error> {
    Ok(Json(match auth.role {
        Role::Admin | Role::Owner => {
            Telemetry::list_as_owner(&state.pool, auth.id, start, end).await?
        }
        Role::User(parent) => {
            Telemetry::list_as_user(&state.pool, auth.id, parent, start, end).await?
        }
    }))
}

/// Get telemetry for a device
///
/// - Owner can get telemetry for their devices and their Users' public devices.
/// - User can get telemetry for their devices and their Owner's public devices.
#[utoipa::path(
    get,
    path = "/device/{device_id}",
    params(TelemetryTimeframe),
    responses(GetDeviceTelemetry),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get_by_device(
    State(state): State<AppState>,
    auth: Authenticated,
    Path(device_id): Path<i64>,
    Query(timeframe): Query<TelemetryTimeframe>,
) -> Result<Json<Vec<Telemetry>>, Error> {
    let device = Device::get_by_id(&state.pool, device_id).await?;

    match auth.role {
        Role::Admin => (),
        Role::Owner | Role::User(_) if device.owner_id == auth.id => (),
        Role::User(parent) if device.owner_id == parent && device.is_public => (),
        Role::Owner
            if device.is_public
                && User::is_child(&state.pool, device.owner_id, auth.id).await? => {}
        _ => return Err(Error::DeviceNotFound),
    }

    let telemetry = Telemetry::list(&state.pool, device_id, timeframe.start, timeframe.end).await?;

    Ok(Json(telemetry))
}

/// Create telemetry entry
///
/// Called by devices using their key authentication.
#[utoipa::path(
    post,
    path = "",
    request_body = CreateTelemetry,
    responses(PostTelemetry),
    tag = TAG,
    security(("api_key" = []))
)]
pub async fn post(
    State(state): State<AppState>,
    AuthDevice(device): AuthDevice,
    Validated(data): Validated<CreateTelemetry>,
) -> Result<(StatusCode, Json<Telemetry>), Error> {
    let telemetry = Telemetry::create(&state.pool, device.id, data).await?;

    Ok((StatusCode::CREATED, Json(telemetry)))
}

/// Delete device telemetry entries
///
/// Owner or User may delete **only** their own telemetry.
#[utoipa::path(
    delete,
    path = "/device/{device_id}",
    params(TelemetryTimeframe),
    responses(DeleteTelemetry),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn delete(
    State(state): State<AppState>,
    auth: Authenticated,
    Path(device_id): Path<i64>,
    Query(timeframe): Query<TelemetryTimeframe>,
) -> Result<Json<u64>, Error> {
    let device = Device::get_by_id(&state.pool, device_id).await?;

    match auth.role {
        Role::Admin => (),
        Role::Owner | Role::User(_) if device.owner_id == auth.id => (),
        _ => return Err(Error::TelemetryNotFound),
    }

    Ok(Json(
        Telemetry::delete(&state.pool, device_id, timeframe.start, timeframe.end).await?,
    ))
}
