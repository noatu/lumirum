use argon2::{
    Argon2,
    PasswordHash,
    PasswordVerifier,
    password_hash::{
        PasswordHasher,
        SaltString,
        rand_core::OsRng,
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
        TAG,
        db::User,
        jwt::Authenticated,
        types::{
            AuthResponse,
            ChangePasswordRequest,
        },
    },
    responses::ChangePassword,
};

/// Change password
#[utoipa::path(
    post,
    path = "/password",
    request_body = ChangePasswordRequest,
    responses(ChangePassword),
    tag = TAG
)]
pub async fn change_password(
    State(state): State<AppState>,
    user: Authenticated,
    Validated(payload): Validated<ChangePasswordRequest>,
) -> Result<Json<AuthResponse>, Error> {
    let mut db_user = User::read_by_id(&state.pool, user.id).await?;

    Argon2::default().verify_password(
        payload.old_password.as_bytes(),
        &PasswordHash::new(&db_user.password_hash)?,
    )?;

    let password_hash = Argon2::default()
        .hash_password(
            payload.new_password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        )?
        .to_string();

    db_user.update_password(&state.pool, password_hash).await?;

    Ok(Json(AuthResponse {
        user: db_user,
        token: user.token,
    }))
}
