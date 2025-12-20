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
    Modify,
    ToSchema,
    openapi::{
        OpenApi,
        security::{
            ApiKey,
            ApiKeyValue,
            Http,
            HttpAuthScheme,
            SecurityScheme,
        },
    },
};
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use crate::AppState;

mod db;
mod jwt;
mod routes {
    pub mod login;
    pub mod me;
    pub mod register;
}

pub use jwt::Authenticated;

pub const TAG: &str = "Authentication";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(routes::register::register))
        .routes(routes!(routes::login::login))
        .routes(routes!(
            routes::me::get,
            routes::me::patch,
            routes::me::delete
        ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize, Deserialize, ToSchema)]
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
    pub user: db::User,
    pub token: String,
}

#[derive(Deserialize, Validate, ToSchema)]
struct AuthRequest {
    /// Username consisting of alphanumeric charactors
    #[garde(alphanumeric, length(chars, min = 3, max = 25))]
    #[schema(min_length = 3, max_length = 25, example = "john")]
    pub username: String,
    // FIXME: login should have no minimal limit for length
    #[garde(length(min = 8))]
    #[schema(min_length = 8, example = "lumirum!")]
    pub password: String,
}

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-api-key"))),
            );
        }
    }
}
