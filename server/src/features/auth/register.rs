use argon2::{
    Argon2,
    password_hash::{
        PasswordHasher,
        SaltString,
        rand_core::OsRng,
    },
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
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
        Role,
    },
};

/// Register a new user
#[utoipa::path(
    post,
    path = "/register",
    request_body = AuthRequest,
    responses(crate::responses::Register),
    tag = super::TAG
)]
pub async fn register(
    State(state): State<crate::AppState>,
    Validated(payload): Validated<AuthRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), Error> {
    if User::exits(&state.pool, &payload.username).await? {
        return Err(Error::UsernameTaken);
    }

    let password_hash = Argon2::default()
        .hash_password(
            payload.password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        )?
        .to_string();

    let user = User::create(&state.pool, payload.username, password_hash, Role::Owner).await?;
    let token = sign(user.id, &user.username, user.role, &state.jwt_secret)?;
    Ok((StatusCode::CREATED, Json(AuthResponse { user, token })))
}
