use axum::{
    Json,
    extract::State,
};

use crate::errors::Error;

use super::{
    db::User,
    jwt::Authenticated,
    types::AuthResponse,
};

/// Get current user information
#[utoipa::path(
    get,
    path = "/me",
    responses(crate::responses::GetMe),
    tag = super::TAG,
    security(("jwt" = []))
)]
pub async fn get_me(
    State(state): State<crate::AppState>,
    user: Authenticated,
) -> Result<Json<AuthResponse>, Error> {
    Ok(Json(AuthResponse {
        user: User::read_by_id(&state.pool, user.id).await?,
        token: user.token,
    }))
}
