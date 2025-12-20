use utoipa::{
    Modify,
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
mod types;
mod routes {
    pub mod login;
    pub mod me;
    pub mod password;
    pub mod register;
}

pub use jwt::Authenticated;
pub use types::{
    AuthResponse,
    Role,
};

pub const TAG: &str = "Authentication";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(routes::register::register))
        .routes(routes!(routes::login::login))
        .routes(routes!(routes::password::change_password))
        .routes(routes!(routes::me::get_me))
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
