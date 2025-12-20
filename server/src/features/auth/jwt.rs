use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
};
use axum_extra::{
    TypedHeader,
    headers::{
        Authorization,
        authorization::Bearer,
    },
};
use chrono::{
    Duration,
    Utc,
};
use jsonwebtoken::{
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
    decode,
    encode,
    errors::ErrorKind,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::errors::Error;

use super::types::Role;

pub fn sign(
    sub: i64,
    username: &str,
    role: Role,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    #[allow(clippy::expect_used)]
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("never out of date")
        .timestamp();
    let claims = Claims {
        sub,
        username: username.into(),
        role,
        exp: expiration.cast_unsigned(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

#[derive(Serialize, Deserialize)]
struct Claims {
    pub sub: i64,
    pub username: String,
    pub role: Role,
    pub exp: u64,
}

pub struct Authenticated {
    pub id: i64,
    pub username: String,
    pub role: Role,
    pub token: String,
}

impl FromRequestParts<crate::AppState> for Authenticated {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &crate::AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::MissingCredentials)?;

        let secret = &state.jwt_secret;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => Error::TokenExpired,
            _ => Error::InvalidToken,
        })?;

        Ok(Self {
            id: token_data.claims.sub,
            username: token_data.claims.username,
            role: token_data.claims.role,
            token: bearer.token().to_owned(),
        })
    }
}
