use axum::{
    Json,
    extract::State,
};

use crate::errors::Error;

use super::{
    db::User,
    jwt::AuthUser,
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
    auth_user: AuthUser,
) -> Result<Json<AuthResponse>, Error> {
    let user = User::read_by_id(&state.pool, auth_user.id).await?;

    Ok(Json(AuthResponse {
        user,
        token: auth_user.token,
    }))
}
