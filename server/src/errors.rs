use axum::{
    Json,
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
};
use serde_json::json;
use strum::IntoStaticStr;
use thiserror::Error;

// TODO: this can be done granularly with error_set
#[derive(Error, Debug, IntoStaticStr)]
pub enum Error {
    #[error("username is taken")]
    UsernameTaken,
    #[error("profile name is already in use")]
    ProfileNameTaken,
    #[error("device MAC address is already registered")]
    DeviceMACTaken,
    #[error("device name is already in use")]
    DeviceNameTaken,
    #[error("cannot delete last administrator account")]
    CannotDeleteLastAdmin,

    #[error("credentials are wrong")]
    UserNotFound, // NOTE: IntoStaticStr leaks this detail

    #[error("credentials are wrong")]
    WrongCredentials,
    #[error("credentials are missing")]
    MissingCredentials,
    #[error("token is invalid")]
    InvalidToken,
    #[error("token has expired")]
    TokenExpired,

    #[error(transparent)]
    InvalidData(#[from] garde::Report),
    #[error(transparent)]
    InvalidJson(#[from] axum::extract::rejection::JsonRejection),

    // Internal
    #[error("database: {0}")]
    Database(sqlx::Error),
    #[error("password hash: {0}")]
    PasswordHash(argon2::password_hash::Error),
    #[error("json web token: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

impl From<&Error> for StatusCode {
    fn from(value: &Error) -> Self {
        match value {
            Error::UsernameTaken
            | Error::ProfileNameTaken
            | Error::DeviceMACTaken
            | Error::DeviceNameTaken
            | Error::CannotDeleteLastAdmin => Self::CONFLICT,

            Error::UserNotFound
            | Error::WrongCredentials
            | Error::MissingCredentials
            | Error::InvalidToken
            | Error::TokenExpired => Self::UNAUTHORIZED,

            Error::InvalidData(_) => Self::UNPROCESSABLE_ENTITY,
            Error::InvalidJson(_) => Self::BAD_REQUEST,
            Error::Database(_) | Error::PasswordHash(_) | Error::Jwt(_) => {
                Self::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(error: argon2::password_hash::Error) -> Self {
        match error {
            argon2::password_hash::Error::Password => Self::WrongCredentials,
            _ => Self::PasswordHash(error),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        if let sqlx::Error::Database(err) = &error
            && err.is_unique_violation()
            && let Some(constraint) = err.constraint()
        {
            return match constraint {
                "users_username_key" => Self::UsernameTaken,
                "profiles_owner_id_name_key" => Self::ProfileNameTaken,
                "devices_owner_id_mac_address_key" => Self::DeviceMACTaken,
                "devices_owner_id_name_key" => Self::DeviceNameTaken,
                _ => Self::Database(error),
            };
        }
        Self::Database(error)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status: StatusCode = (&self).into();

        let (code, message) = if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!("{self}:\n{self:#?}");
            (
                "InternalError",
                "an internal server error has occurred".to_string(),
            )
        } else {
            ((&self).into(), self.to_string())
        };

        let body = Json(json!({"error": {
            "code": code,
            "message": message
        }}));

        (status, body).into_response()
    }
}
