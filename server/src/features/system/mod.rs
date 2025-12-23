use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

mod health;
mod stats;
mod users;

pub use stats::Stats;

pub const TAG: &str = "System";

pub fn router() -> OpenApiRouter<crate::AppState> {
    OpenApiRouter::new()
        .routes(routes!(health::health))
        .routes(routes!(stats::stats))
        .nest(
            "/users",
            OpenApiRouter::new()
                .routes(routes!(users::get, users::delete))
                .routes(routes!(users::get_all)),
        )
}
