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
mod get_me;
mod jwt;
mod login;
mod register;
mod types;

pub use jwt::AuthUser;
pub use types::AuthResponse;
pub use types::Role;

pub const TAG: &str = "Authentication";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(register::register))
        .routes(routes!(login::login))
        .routes(routes!(get_me::get_me))
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
