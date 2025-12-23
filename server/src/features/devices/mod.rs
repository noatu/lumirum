use axum::{
    Json,
    extract::{
        Path,
        State,
    },
    http::StatusCode,
};
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
        profiles,
    },
    responses::{
        DeleteDevice,
        GetDevice,
        GetDevices,
        PostDevice,
        PutDevice,
        RegenerateDeviceKey,
    },
};

mod auth;
mod db;

use db::CreateDevice;

pub use auth::AuthDevice;
pub use db::Device;

pub const TAG: &str = "Devices";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(post, get_all))
        .routes(routes!(get, put, delete, regenerate_key))
}

/// Get device info
///
/// - Owner or User may get their own devices.
/// - Users may get their Owner's public devices.
/// - Owners may get their Users' public devices.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(GetDevice),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    auth: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<Device>, Error> {
    let device = Device::get_by_id(&state.pool, id).await?;

    Ok(Json(match auth.role {
        Role::Admin => device,
        Role::Owner | Role::User(_) if device.owner_id == auth.id => device,
        Role::User(parent) if device.owner_id == parent && device.is_public => device,
        Role::Owner
            if device.is_public
                && User::is_child(&state.pool, device.owner_id, auth.id).await? =>
        {
            device
        }
        _ => return Err(Error::DeviceNotFound),
    }))
}

/// List all devices
///
/// - Owner may get their own devices and their Users' public devices.
/// - Users may get their own devices and their Owner's public devices.
#[utoipa::path(
    get,
    path = "",
    responses(GetDevices),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get_all(
    State(state): State<AppState>,
    auth: Authenticated,
) -> Result<Json<Vec<Device>>, Error> {
    Ok(Json(match auth.role {
        Role::Admin | Role::Owner => Device::list_as_owner(&state.pool, auth.id).await?,
        Role::User(parent) => Device::list_as_user(&state.pool, auth.id, parent).await?,
    }))
}

/// Create a new device
#[utoipa::path(
    post,
    path = "",
    request_body = CreateDevice,
    responses(PostDevice),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn post(
    State(state): State<AppState>,
    auth: Authenticated,
    Validated(payload): Validated<CreateDevice>,
) -> Result<Json<Device>, Error> {
    // HACK: permissions check
    if let Some(id) = payload.profile_id {
        profiles::get_raw(&state, &auth, id).await?;
    }

    Ok(Json(Device::create(&state.pool, auth.id, payload).await?))
}

/// Update a device
///
/// - Owner or User may update their own devices.
/// - Users may update their Owner's public devices.
/// - Owners may update their Users' public devices.
#[utoipa::path(
    put,
    path = "/{id}",
    request_body = CreateDevice,
    responses(PutDevice),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn put(
    State(state): State<AppState>,
    auth: Authenticated,
    Path(id): Path<i64>,
    Validated(payload): Validated<CreateDevice>,
) -> Result<Json<Device>, Error> {
    let children = User::get_children(&state.pool, auth.id).await?;

    // HACK: permissions check
    if let Some(id) = payload.profile_id {
        profiles::get_raw(&state, &auth, id).await?;
    }

    let payload = payload.into_inner();
    let device = Device::update(&state.pool, id, |device| match auth.role {
        Role::Admin => Ok(device.patch(payload)),
        Role::Owner | Role::User(_) if device.owner_id == auth.id => Ok(device.patch(payload)),
        Role::User(parent) if device.owner_id == parent && device.is_public => {
            if !payload.is_public {
                return Err(Error::CannotSetOthersDevicePrivate);
            }
            Ok(device.patch(payload))
        }
        Role::Owner if device.is_public && children.contains(&device.owner_id) => {
            if !payload.is_public {
                return Err(Error::CannotSetOthersDevicePrivate);
            }
            Ok(device.patch(payload))
        }
        _ => Err(Error::DeviceNotFound),
    })
    .await?;

    Ok(Json(device))
}

/// Regenerate device secret key
///
/// - Owner or User may regenerate the key for their own devices.
/// - Users may **not** regenerate the key for their Owner's public devices.
/// - Owners may regenerate the key for their Users' public devices.
#[utoipa::path(
    post,
    path = "/{id}/key",
    responses(RegenerateDeviceKey),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn regenerate_key(
    State(state): State<AppState>,
    auth: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<Device>, Error> {
    let children = User::get_children(&state.pool, auth.id).await?;

    let device = Device::update(&state.pool, id, |device| match auth.role {
        Role::Admin => Ok(device.regenerate_key()),
        Role::Owner | Role::User(_) if device.owner_id == auth.id => Ok(device.regenerate_key()),
        Role::User(parent) if device.owner_id == parent && device.is_public => {
            Err(Error::CannotChangeDeviceKey)
        }
        Role::Owner if device.is_public && children.contains(&device.owner_id) => {
            Ok(device.regenerate_key())
        }
        _ => Err(Error::DeviceNotFound),
    })
    .await?;

    Ok(Json(device))
}

/// Delete a device
///
/// Owner or User may delete **only** their own devices.
#[utoipa::path(
    delete,
    path = "/{id}",
    responses(DeleteDevice),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn delete(
    State(state): State<AppState>,
    user: Authenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, Error> {
    Device::delete(&state.pool, id, |device| match user.role {
        Role::Admin => Ok(true),
        Role::Owner | Role::User(_) if device.owner_id == user.id => Ok(true),
        // Role::User(parent_id) if device.owner_id == parent_id && device.is_public => Ok(true),
        // Role::Owner if device.is_public && children.contains(&device.owner_id) => Ok(true),
        _ => Err(Error::DeviceNotFound),
    })
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
