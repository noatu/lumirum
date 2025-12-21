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
        db::{
            Role,
            User,
        },
        jwt::sign,
    },
    responses::Register,
};

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    /// Username consisting of alphanumeric characters
    #[garde(alphanumeric, length(chars, min = 3, max = 25))]
    #[schema(min_length = 3, max_length = 25, example = "john")]
    pub username: String,
    #[garde(length(chars, min = 8))]
    #[schema(min_length = 8, example = "lumirum!")]
    pub password: String,
}

/// Register a new user
#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(Register),
    tag = TAG,
    // security((), ("jwt" = [])) TODO: register other users
)]
pub async fn register(
    State(state): State<AppState>,
    Validated(payload): Validated<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), Error> {
    let password_hash = Argon2::default()
        .hash_password(
            payload.password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        )?
        .to_string();

    let user = User::create(&state.pool, &payload.username, &password_hash, Role::Owner).await?;

    let token = sign(user.id, user.role, &state.jwt_secret)?;
    Ok((StatusCode::CREATED, Json(AuthResponse { user, token })))
}
