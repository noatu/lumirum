use argon2::{
    Argon2,
    password_hash::{
        PasswordHash,
        PasswordVerifier,
    },
};
use axum::{
    Json,
    extract::State,
};

use crate::{
    AppState,
    errors::Error,
    extractors::Validated,
    features::auth::{
        AuthRequest,
        AuthResponse,
        TAG,
        db::User,
        jwt::sign,
    },
    responses::Login,
};

/// Log into an existing user
#[utoipa::path(
    post,
    path = "/login",
    request_body = AuthRequest,
    responses(Login),
    tag = TAG
)]
pub async fn login(
    State(state): State<AppState>,
    Validated(payload): Validated<AuthRequest>,
) -> Result<Json<AuthResponse>, Error> {
    let user = User::get_by_username(&state.pool, &payload.username).await?;

    Argon2::default().verify_password(
        payload.password.as_bytes(),
        &PasswordHash::new(&user.password_hash)?,
    )?;

    let token = sign(user.id, &user.username, user.role, &state.jwt_secret)?;

    Ok(Json(AuthResponse { user, token }))
}
