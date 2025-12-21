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
    },
    responses::{
        DeleteProfile,
        GetProfiles,
        PostProfile,
        PutProfile,
    },
};

mod db;

use db::CreateProfile;
pub use db::Profile;

pub const TAG: &str = "Profiles";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(post, get_all))
        .routes(routes!(get, patch, delete))
}

/// Get profile info
#[utoipa::path(
    get,
    path = "/{id}",
    responses(GetProfiles),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    user: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<Profile>, Error> {
    let profile = Profile::get_by_id(&state.pool, id).await?;

    if profile.owner_id == user.id || Role::User(profile.owner_id) == user.role {
        return Ok(Json(profile));
    }

    Err(Error::ProfileNotFound)
}

/// List all profiles
#[utoipa::path(
    get,
    path = "",
    responses(GetProfiles),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get_all(
    State(state): State<AppState>,
    user: Authenticated,
) -> Result<Json<Vec<Profile>>, Error> {
    let mut profiles = Profile::list_by_owner(&state.pool, user.id).await?;

    if let Role::User(parent_id) = user.role {
        profiles.extend(Profile::list_by_owner(&state.pool, parent_id).await?);
    }

    Ok(Json(profiles))
}

/// Create a new profile
#[utoipa::path(
    post,
    path = "",
    request_body = CreateProfile,
    responses(PostProfile),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn post(
    State(state): State<AppState>,
    user: Authenticated,
    Validated(payload): Validated<CreateProfile>,
) -> Result<Json<Profile>, Error> {
    Ok(Json(Profile::create(&state.pool, user.id, payload).await?))
}

/// Update a profile
#[utoipa::path(
    put,
    path = "/{id}",
    request_body = CreateProfile,
    responses(PutProfile),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn patch(
    State(state): State<AppState>,
    user: Authenticated,
    Path(id): Path<i64>,
    Validated(payload): Validated<CreateProfile>,
) -> Result<Json<Profile>, Error> {
    let payload = payload.into_inner();
    let profile = Profile::update(&state.pool, id, |profile| match user.role {
        Role::User(parent) if profile.owner_id == parent => Err(Error::CantModifyParentProfile),
        Role::Owner | Role::User(_) if profile.owner_id == user.id => Ok(profile.patch(payload)),
        _ => Err(Error::ProfileNotFound),
    })
    .await?;

    Ok(Json(profile))
}

/// Delete a profile
#[utoipa::path(
    delete,
    path = "/{id}",
    responses(DeleteProfile),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn delete(
    State(state): State<AppState>,
    user: Authenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, Error> {
    Profile::delete(&state.pool, id, |profile| match user.role {
        Role::Admin => Ok(true),
        Role::Owner | Role::User(_) if profile.owner_id == user.id => Ok(true),
        Role::User(parent) if profile.owner_id == parent => Err(Error::CantModifyParentProfile),
        _ => Err(Error::ProfileNotFound),
    })
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
