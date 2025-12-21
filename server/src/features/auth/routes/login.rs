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
use garde::Validate;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    AppState,
    errors::Error,
    extractors::Validated,
    features::auth::{
        AuthResponse,
        TAG,
        db::User,
        jwt::sign,
    },
    responses::Login,
};

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    /// Username consisting of alphanumeric characters
    #[garde(alphanumeric, length(chars, min = 1))]
    #[schema(min_length = 1, example = "john")]
    pub username: String,
    #[garde(length(chars, min = 1))]
    #[schema(min_length = 1, example = "lumirum!")]
    pub password: String,
}

/// Log into an existing user
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(Login),
    tag = TAG
)]
pub async fn login(
    State(state): State<AppState>,
    Validated(payload): Validated<LoginRequest>,
) -> Result<Json<AuthResponse>, Error> {
    let user = User::get_by_username(&state.pool, &payload.username).await?;

    Argon2::default().verify_password(
        payload.password.as_bytes(),
        &PasswordHash::new(&user.password_hash)?,
    )?;

    let token = sign(user.id, user.role, &state.jwt_secret)?;

    Ok(Json(AuthResponse { user, token }))
}
