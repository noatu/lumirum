use axum::{
    Json,
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
};
use axum_extra::typed_header::{
    TypedHeaderRejection,
    TypedHeaderRejectionReason,
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
    #[error("device name is already in use")]
    DeviceNameTaken,
    #[error("cannot delete an administrator account")]
    CannotDeleteAnAdmin,

    // NOTE: register tells that username is taken,
    // so it's ok if login tells that username is not found
    #[error("credentials are wrong")]
    UserNotFound,

    #[error("profile does not exist")]
    ProfileNotFound,
    #[error("device does not exist")]
    DeviceNotFound,
    #[error("telemetry does not exist")]
    TelemetryNotFound,

    #[error("credentials are wrong")]
    WrongCredentials,
    #[error("credentials are missing")]
    MissingCredentials,
    #[error("token is invalid")]
    InvalidToken,
    #[error("token has expired")]
    TokenExpired,

    #[error("sub-user cannot create another user")]
    UsersCannotCreateUsers,
    #[error("cannot set others' prfile as private")]
    CannotSetOthersProfilePrivate,
    #[error("cannot set others' device as private")]
    CannotSetOthersDevicePrivate,
    #[error("users cannot regenerate an owner's device key")]
    CannotChangeDeviceKey,

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
    #[error("data corrupted: {0}")]
    DataCorruption(String),
    #[error("invalide time: {0}")]
    InvalidTime(#[from] chrono_tz::ParseError),
}

impl From<&Error> for StatusCode {
    fn from(value: &Error) -> Self {
        match value {
            Error::UsernameTaken
            | Error::ProfileNameTaken
            | Error::DeviceNameTaken
            | Error::CannotDeleteAnAdmin => Self::CONFLICT,

            Error::WrongCredentials
            | Error::MissingCredentials
            | Error::InvalidToken
            | Error::TokenExpired => Self::UNAUTHORIZED,

            Error::UsersCannotCreateUsers
            | Error::CannotSetOthersProfilePrivate
            | Error::CannotSetOthersDevicePrivate
            | Error::CannotChangeDeviceKey => Self::FORBIDDEN,

            Error::UserNotFound
            | Error::ProfileNotFound
            | Error::DeviceNotFound
            | Error::TelemetryNotFound => Self::NOT_FOUND,

            Error::InvalidJson(_) => Self::BAD_REQUEST,
            Error::InvalidData(_) => Self::UNPROCESSABLE_ENTITY,

            Error::Database(_)
            | Error::PasswordHash(_)
            | Error::Jwt(_)
            | Error::DataCorruption(_)
            | Error::InvalidTime(_) => Self::INTERNAL_SERVER_ERROR,
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
            && let Some(constraint) = err.constraint()
        {
            return match constraint {
                "users_username_key" => Self::UsernameTaken,
                "profiles_owner_id_name_key" => Self::ProfileNameTaken,
                "devices_owner_id_name_key" => Self::DeviceNameTaken,
                "devices_profile_id_fkey" => Self::ProfileNotFound,
                _ => Self::Database(error),
            };
        }
        Self::Database(error)
    }
}

impl From<TypedHeaderRejection> for Error {
    fn from(value: TypedHeaderRejection) -> Self {
        match value.reason() {
            TypedHeaderRejectionReason::Missing => Self::MissingCredentials,
            _ => Self::InvalidToken,
        }
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
