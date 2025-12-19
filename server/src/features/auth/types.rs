use garde::Validate;
use serde::{
    Deserialize,
    Serialize,
};
use sqlx::{
    FromRow,
    Type,
};
use utoipa::{
    IntoResponses,
    ToSchema,
};

use super::db::User;

#[derive(Clone, Copy, Type, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum Role {
    Admin,
    Owner,
    User,
}

#[derive(FromRow, Serialize, ToSchema, IntoResponses)]
#[response(status = OK)]
pub struct AuthResponse {
    #[serde(flatten)]
    pub user: User,
    pub token: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct AuthRequest {
    /// Username consisting of alphanumeric charactors
    #[garde(alphanumeric, length(chars, min = 3, max = 25))]
    #[schema(min_length = 3, max_length = 25, example = "john42")]
    pub username: String,
    #[garde(length(min = 8))]
    #[schema(min_length = 8, example = "password!")]
    pub password: String,
}
