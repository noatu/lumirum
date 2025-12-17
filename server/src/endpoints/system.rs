use axum::{
    Json,
    extract::State,
};
use serde::Serialize;
use utoipa::{
    IntoResponses,
    ToSchema,
};
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use crate::AppState;

pub const TAG: &str = "System";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(health))
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
enum HealthStatus {
    Healthy,
    NoDatabaseConnection,
}

/// Health status
#[derive(Serialize, ToSchema, IntoResponses)]
#[response(status = OK)]
struct HealthResponse {
    #[schema(inline)]
    status: HealthStatus,
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Check if the server is up and running
#[utoipa::path(get, path = "/health", responses(HealthResponse), tag = TAG)]
async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let db_connected = sqlx::query("SELECT 1").execute(&state.pool).await.is_ok();

    let status = if db_connected {
        HealthStatus::Healthy
    } else {
        HealthStatus::NoDatabaseConnection
    };

    Json(HealthResponse {
        status,
        timestamp: chrono::Utc::now(),
    })
}
