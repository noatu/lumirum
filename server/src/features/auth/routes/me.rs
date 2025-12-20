use axum::{
    Json,
    extract::State,
};

use crate::{
    AppState,
    errors::Error,
    features::auth::{
        AuthResponse,
        TAG,
        db::User,
        jwt::Authenticated,
    },
    responses::GetMe,
};

/// Get current user information
#[utoipa::path(
    get,
    path = "/me",
    responses(GetMe),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get_me(
    State(state): State<AppState>,
    user: Authenticated,
) -> Result<Json<AuthResponse>, Error> {
    Ok(Json(AuthResponse {
        user: User::read_by_id(&state.pool, user.id).await?,
        token: user.token,
    }))
}
