#![allow(clippy::needless_for_each)] // HACK: OpenApi macro silencing

use axum::Router;
use utoipa::{
    Modify,
    OpenApi,
    openapi::security::{
        ApiKey,
        ApiKeyValue,
        Http,
        HttpAuthScheme,
        SecurityScheme,
    },
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{
    Scalar,
    Servable,
};

use crate::{
    features::{
        auth::{
            self,
            AuthResponse,
        },
        circadian::LightingSchedule,
        devices::{
            self,
            Device,
        },
        profiles::{
            self,
            Profile,
        },
        system::{
            self,
            Stats,
        },
        telemetry::{
            self,
            Telemetry,
        },
    },
    responses::ErrorResponse,
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(
        title = "LumiRum",
        version = env!("CARGO_PKG_VERSION"),
        description = "LumiRum OpenAPI Specification",
    ),
    components(schemas(ErrorResponse, Stats, AuthResponse, Profile, Device, Telemetry, LightingSchedule))
)]
struct ApiDoc;

pub fn router() -> Router<crate::AppState> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(system::router())
        .nest("/auth", auth::router()) // TODO: manual user creation
        .nest("/profiles", profiles::router())
        .nest("/devices", devices::router())
        .nest("/telemetry", telemetry::router())
        .split_for_parts();

    tracing::info!("Scalar is available at /");
    router.merge(Scalar::with_url("/", api))
}

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
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
