#![allow(clippy::needless_for_each)] // HACK: OpenApi macro silencing

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{
    Scalar,
    Servable,
};

use crate::AppState;

mod system;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "LumiRum",
        version = env!("CARGO_PKG_VERSION"),
        description = "LumiRum OpenAPI Specification",
    ),
    tags(
        (name = system::TAG, description = "System endpoints"),
    )
)]
struct ApiDoc;

pub fn router() -> Router<AppState> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(system::router())
        // .nest("/nested", nested::router())
        .split_for_parts();

    tracing::info!("Scalar is available at /");
    router.merge(Scalar::with_url("/", api))
}
