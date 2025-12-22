use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

use crate::{
    AppState,
    errors::Error,
};

use super::Device;

pub struct AuthDevice(pub Device);

impl FromRequestParts<AppState> for AuthDevice {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let api_key = parts
            .headers
            .get("x-api-key")
            .ok_or(Error::MissingCredentials)?
            .to_str()
            .map_err(|_| Error::InvalidToken)?;

        let device = Device::get_by_secret_key(&state.pool, api_key)
            .await?
            .ok_or(Error::TokenExpired)?;

        Ok(Self(device))
    }
}
