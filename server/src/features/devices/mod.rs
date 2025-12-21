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
    features::auth::{
        Authenticated,
        Role,
        User,
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

mod db;

use db::CreateDevice;

pub use db::Device;

pub const TAG: &str = "Devices";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(post, get_all))
        .routes(routes!(get, patch, delete, regenerate_key))
}

/// Get device info
#[utoipa::path(
    get,
    path = "/{id}",
    responses(GetDevice),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    user: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<Device>, Error> {
    let device = Device::get_by_id(&state.pool, id).await?;

    Ok(Json(match user.role {
        Role::Admin => device,
        Role::Owner | Role::User(_) if device.owner_id == user.id => device,
        Role::User(parent) if device.owner_id == parent && device.is_public => device,
        Role::Owner if User::is_child(&state.pool, device.owner_id, user.id).await? => device,
        _ => return Err(Error::DeviceNotFound),
    }))
}

/// List all relevant devices
#[utoipa::path(
    get,
    path = "",
    responses(GetDevices),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get_all(
    State(state): State<AppState>,
    user: Authenticated,
) -> Result<Json<Vec<Device>>, Error> {
    Ok(Json(match user.role {
        Role::Admin => vec![],
        Role::Owner => Device::list_owner_devices(&state.pool, user.id).await?,
        Role::User(parent) => Device::list_user_devices(&state.pool, user.id, parent).await?,
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
    user: Authenticated,
    Validated(payload): Validated<CreateDevice>,
) -> Result<Json<Device>, Error> {
    Ok(Json(Device::create(&state.pool, user.id, payload).await?))
}

/// Update a device
#[utoipa::path(
    put,
    path = "/{id}",
    request_body = CreateDevice,
    responses(PutDevice),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn patch(
    State(state): State<AppState>,
    user: Authenticated,
    Path(id): Path<i64>,
    Validated(payload): Validated<CreateDevice>,
) -> Result<Json<Device>, Error> {
    let payload = payload.into_inner();
    let children = User::get_children(&state.pool, user.id).await?;

    let device = Device::update(&state.pool, id, |device| match user.role {
        Role::Admin => Ok(device.patch(payload)),
        Role::Owner | Role::User(_) if device.owner_id == user.id => Ok(device.patch(payload)),
        Role::Owner if children.contains(&device.owner_id) => {
            if !payload.is_public {
                return Err(Error::CannotSetOthersDevicePrivate);
            }
            Ok(device.patch(payload))
        }
        Role::User(parent_id) if device.owner_id == parent_id && device.is_public => {
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
#[utoipa::path(
    post,
    path = "/{id}/key",
    responses(RegenerateDeviceKey),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn regenerate_key(
    State(state): State<AppState>,
    user: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<Device>, Error> {
    let children = User::get_children(&state.pool, user.id).await?;

    let device = Device::update(&state.pool, id, |device| match user.role {
        Role::Admin => Ok(device.regenerate_key()),
        Role::Owner | Role::User(_) if device.owner_id == user.id => Ok(device.regenerate_key()),
        Role::Owner if children.contains(&device.owner_id) => Ok(device.regenerate_key()),
        _ => Err(Error::DeviceNotFound),
    })
    .await?;

    Ok(Json(device))
}

/// Delete a device
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
        _ => Err(Error::DeviceNotFound),
    })
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
