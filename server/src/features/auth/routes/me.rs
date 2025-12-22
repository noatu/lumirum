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
        jwt::Authenticated,
    },
    responses::{
        DeleteMe,
        GetMe,
        PatchMe,
    },
};

/// Get current user information
#[utoipa::path(
    get,
    path = "/me",
    responses(GetMe),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn get(
    State(state): State<AppState>,
    auth: Authenticated,
) -> Result<Json<AuthResponse>, Error> {
    Ok(Json(AuthResponse {
        user: User::get_by_id(&state.pool, auth.id).await?,
        token: auth.token,
    }))
}

#[derive(Deserialize, Validate, ToSchema)]
#[schema(as = ChangeAccountRequest)]
pub struct ChangeRequest {
    #[garde(length(chars, min = 1))]
    #[schema(example = "lumirum!")]
    pub password: String,
    #[garde(alphanumeric, length(chars, min = 3, max = 25))]
    #[schema(min_length = 3, max_length = 25, example = "johnchanged")]
    pub new_username: Option<String>,
    #[garde(length(chars, min = 8))]
    #[schema(min_length = 8, example = "lumirum!changed")]
    pub new_password: Option<String>,
}

/// Update current user
#[utoipa::path(
    patch,
    path = "/me",
    request_body = ChangeRequest,
    responses(PatchMe),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn patch(
    State(state): State<AppState>,
    auth: Authenticated,
    Validated(payload): Validated<ChangeRequest>,
) -> Result<Json<AuthResponse>, Error> {
    let token = auth.token;
    let new_password_hash = match &payload.new_password {
        Some(pass) => Some(
            Argon2::default()
                .hash_password(pass.as_bytes(), &SaltString::generate(&mut OsRng))?
                .to_string(),
        ),
        None => None,
    };

    let payload = payload.into_inner();
    let user = User::update(&state.pool, auth.id, |user| {
        // FIXME: performance hit, hashing inside a transaction
        Argon2::default().verify_password(
            payload.password.as_bytes(),
            &PasswordHash::new(&user.password_hash)?,
        )?;

        #[allow(clippy::useless_let_if_seq)]
        let mut updated = false;

        if let Some(name) = payload.new_username
            && name != user.username
        {
            user.username = name;
            updated = true;
        }
        if let Some(pass) = new_password_hash
            && pass != user.password_hash
        {
            user.password_hash = pass;
            updated = true;
        }

        Ok(updated)
    })
    .await?;

    Ok(Json(AuthResponse { user, token }))
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct DeleteRequest {
    #[garde(length(chars, min = 1))]
    #[schema(example = "lumirum!")]
    pub password: String,
}

/// Delete current user
#[utoipa::path(
    delete,
    path = "/me",
    request_body = DeleteRequest,
    responses(DeleteMe),
    tag = TAG,
    security(("jwt" = []))
)]
pub async fn delete(
    State(state): State<AppState>,
    auth: Authenticated,
    Validated(payload): Validated<DeleteRequest>,
) -> Result<StatusCode, Error> {
    if auth.role == Role::Admin {
        return Err(Error::CannotDeleteAnAdmin);
    }

    let user = User::get_by_id(&state.pool, auth.id).await?;

    Argon2::default().verify_password(
        payload.password.as_bytes(),
        &PasswordHash::new(&user.password_hash)?,
    )?;

    User::delete(&state.pool, user.id).await?;

    Ok(StatusCode::NO_CONTENT)
}
