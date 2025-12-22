use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{
        Authorization,
        authorization::Bearer,
    },
    typed_header::TypedHeaderRejectionReason,
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

use crate::{
    AppState,
    errors::Error,
};

use super::Role;

pub fn sign(sub: i64, role: Role, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    #[allow(clippy::expect_used)]
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("never out of date")
        .timestamp();
    let claims = Claims {
        sub,
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
    pub role: Role,
    pub exp: u64,
}

pub struct Authenticated {
    pub id: i64,
    pub role: Role,
    pub token: String,
    // pub expires: DateTime<Utc>,
}

impl TryFrom<(Bearer, &str)> for Authenticated {
    type Error = Error;

    fn try_from((bearer, secret): (Bearer, &str)) -> Result<Self, Self::Error> {
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
            role: token_data.claims.role,
            token: bearer.token().to_owned(),
        })
    }
}

impl FromRequestParts<AppState> for Authenticated {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        Self::try_from((bearer, state.jwt_secret.as_str()))
    }
}

pub struct MaybeAuthenticated(pub Option<Authenticated>);

impl FromRequestParts<AppState> for MaybeAuthenticated {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let bearer = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(TypedHeader(Authorization(bearer))) => bearer,
            Err(err) => match err.reason() {
                TypedHeaderRejectionReason::Missing => return Ok(Self(None)),
                _ => return Err(Error::InvalidToken),
            },
        };

        Authenticated::try_from((bearer, state.jwt_secret.as_str())).map(|x| Self(Some(x)))
    }
}
