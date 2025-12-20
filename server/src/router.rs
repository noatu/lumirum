#![allow(clippy::needless_for_each)] // HACK: OpenApi macro silencing

use axum::Router;
use utoipa::OpenApi;
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
            SecurityAddon,
        },
        system,
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
    components(schemas(AuthResponse, ErrorResponse))
)]
struct ApiDoc;

pub fn router() -> Router<crate::AppState> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(system::router())
        .nest("/auth", auth::router())
        .split_for_parts();

    tracing::info!("Scalar is available at /");
    router.merge(Scalar::with_url("/", api))
}
