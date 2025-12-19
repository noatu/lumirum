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
    errors::Error,
    extractors::Validated,
};

use super::{
    db::User,
    jwt::AuthUser,

    types::{
        AuthResponse,
        ChangePasswordRequest,
    },
};

/// Change password
#[utoipa::path(
    post,
    path = "/change-password",
    request_body = ChangePasswordRequest,
    responses(crate::responses::ChangePassword),
    tag = super::TAG
)]
pub async fn change_password(
    State(state): State<crate::AppState>,
    auth_user: AuthUser,
    Validated(payload): Validated<ChangePasswordRequest>,
) -> Result<Json<AuthResponse>, Error> {
    let mut user = User::read_by_id(&state.pool, auth_user.id).await?;

    Argon2::default().verify_password(
        payload.old_password.as_bytes(),
        &PasswordHash::new(&user.password_hash)?,
    )?;

    let password_hash = Argon2::default()
        .hash_password(
            payload.new_password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        )?
        .to_string();

    user.update_password(&state.pool, password_hash).await?;

    Ok(Json(AuthResponse {
        user,
        token: auth_user.token,
    }))
}
