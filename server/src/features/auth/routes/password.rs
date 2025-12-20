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
        jwt::Authenticated,
    },
    responses::ChangePassword,
};

#[derive(Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[garde(skip)]
    #[schema(example = "lumirum!")]
    pub old_password: String,
    #[garde(length(min = 8))]
    #[schema(min_length = 8, example = "lumirum!changed")]
    pub new_password: String,
}

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
