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
    errors::Error,
    extractors::Validated,
};

use super::{
    db::User,
    jwt::sign,
    types::{
        AuthRequest,
        AuthResponse,
    },
};

/// Log into an existing user
#[utoipa::path(
    post,
    path = "/login",
    request_body = AuthRequest,
    responses(crate::responses::Login),
    tag = super::TAG
)]
pub async fn login(
    State(state): State<crate::AppState>,
    Validated(payload): Validated<AuthRequest>,
) -> Result<Json<AuthResponse>, Error> {
    let user = User::read_by_username(&state.pool, &payload.username).await?;

    Argon2::default().verify_password(
        payload.password.as_bytes(),
        &PasswordHash::new(&user.password_hash)?,
    )?;

    let token = sign(user.id, &user.username, user.role, &state.jwt_secret)?;

    Ok(Json(AuthResponse { user, token }))
}
