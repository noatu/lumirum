use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

mod health;

pub const TAG: &str = "System";

pub fn router() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::new().routes(routes!(health::health))
}
