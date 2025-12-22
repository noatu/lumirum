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
        DeleteProfile,
        GetProfile,
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
        .routes(routes!(get, put, delete))
}

/// Get profile info
///
/// - Owner or User may get their own profiles.
/// - Users may get their Owner's shared profiles.
/// - Owners may get their Users' shared profiles.
#[utoipa::path(
    get,
    path = "/{id}",
    responses(GetProfile),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    auth: Authenticated,
    Path(id): Path<i64>,
) -> Result<Json<Profile>, Error> {
    let profile = Profile::get_by_id(&state.pool, id).await?;

    Ok(Json(match auth.role {
        Role::Admin => profile,
        Role::Owner | Role::User(_) if profile.owner_id == auth.id => profile,
        Role::User(parent) if profile.owner_id == parent && profile.is_shared => profile,
        Role::Owner
            if profile.is_shared
                && User::is_child(&state.pool, profile.owner_id, auth.id).await? =>
        {
            profile
        }
        _ => return Err(Error::ProfileNotFound),
    }))
}

/// List all profiles
///
/// - Owner may get their own profiles and their Users' shared profiles.
/// - Users may get their own profiles and their Owner's shared profiles.
#[utoipa::path(
    get,
    path = "",
    responses(GetProfiles),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get_all(
    State(state): State<AppState>,
    auth: Authenticated,
) -> Result<Json<Vec<Profile>>, Error> {
    Ok(Json(match auth.role {
        Role::Admin | Role::Owner => Profile::list_as_owner(&state.pool, auth.id).await?,
        Role::User(parent) => Profile::list_as_user(&state.pool, auth.id, parent).await?,
    }))
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
///
/// - Owner or User may update their own profiles.
/// - Users may update their Owner's shared profiles.
/// - Owners may update their Users' shared profiles.
#[utoipa::path(
    put,
    path = "/{id}",
    request_body = CreateProfile,
    responses(PutProfile),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn put(
    State(state): State<AppState>,
    auth: Authenticated,
    Path(id): Path<i64>,
    Validated(payload): Validated<CreateProfile>,
) -> Result<Json<Profile>, Error> {
    let children = User::get_children(&state.pool, auth.id).await?;

    let payload = payload.into_inner();
    let profile = Profile::update(&state.pool, id, |profile| match auth.role {
        Role::Admin => Ok(profile.patch(payload)),
        Role::Owner | Role::User(_) if profile.owner_id == auth.id => Ok(profile.patch(payload)),
        Role::User(parent) if profile.owner_id == parent && profile.is_shared => {
            if !payload.is_shared {
                return Err(Error::CannotSetOthersProfilePrivate);
            }
            Ok(profile.patch(payload))
        }
        Role::Owner if profile.is_shared && children.contains(&profile.owner_id) => {
            if !payload.is_shared {
                return Err(Error::CannotSetOthersProfilePrivate);
            }
            Ok(profile.patch(payload))
        }
        _ => Err(Error::ProfileNotFound),
    })
    .await?;

    Ok(Json(profile))
}

/// Delete a profile
///
/// Owner or User may delete **only** their own profiles.
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
        _ => Err(Error::ProfileNotFound),
    })
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
