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
    AppState,
    errors::Error,
    extractors::Validated,

    features::auth::{
        AuthRequest,
        AuthResponse,
        TAG,
        db::{
            TypedRole,
            User,
        },
        jwt::sign,
    },
    responses::Register,
};

/// Register a new user
#[utoipa::path(
    post,
    path = "/register",
    request_body = AuthRequest,
    responses(Register),
    tag = TAG,
    // security((), ("jwt" = [])) TODO: register other users
)]
pub async fn register(
    State(state): State<AppState>,
    Validated(payload): Validated<AuthRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), Error> {
    let password_hash = Argon2::default()
        .hash_password(
            payload.password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        )?
        .to_string();

    let user = User::create(
        &state.pool,
        &payload.username,
        &password_hash,
        TypedRole::Owner,
    )
    .await?;

    let token = sign(user.id, &user.username, user.role, &state.jwt_secret)?;
    Ok((StatusCode::CREATED, Json(AuthResponse { user, token })))
}
