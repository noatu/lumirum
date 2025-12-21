use serde::Serialize;
use sqlx::FromRow;
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

pub use db::Role;
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

#[derive(FromRow, Serialize, ToSchema, IntoResponses)]
#[response(status = OK)]
pub struct AuthResponse {
    #[serde(flatten)]
    user: db::User,
    token: String,
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
