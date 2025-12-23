use axum::{
    Json,
    extract::{
        Path,
        State,
    },
    http::StatusCode,
};

use crate::{
    AppState,
    errors::Error,
    features::auth::{
        AdminAuthenticated,
        User,
    },
    responses::{
        DeleteMe,
        GetUser,
        GetUsers,
    },
};

use super::TAG;

/// Get user
#[utoipa::path(
    get,
    path = "/{id}",
    responses(GetUser),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    AdminAuthenticated(_auth): AdminAuthenticated,
    Path(id): Path<i64>,
) -> Result<Json<User>, Error> {
    Ok(Json(User::get_by_id(&state.pool, id).await?))
}

/// List all users
#[utoipa::path(
    get,
    path = "",
    responses(GetUsers),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get_all(
    State(state): State<AppState>,
    AdminAuthenticated(_auth): AdminAuthenticated,
) -> Result<Json<Vec<User>>, Error> {
    User::get_all(&state.pool).await.map(Json)
}

/// Delete a user
#[utoipa::path(
    delete,
    path = "/{id}",
    responses(DeleteMe),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn delete(
    State(state): State<AppState>,
    AdminAuthenticated(auth): AdminAuthenticated,
    Path(id): Path<i64>,
) -> Result<StatusCode, Error> {
    if auth.id == id {
        return Err(Error::CannotDeleteAnAdmin);
    }

    User::delete(&state.pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
