use axum::{
    Json,
    extract::State,
};
use serde::Serialize;
use utoipa::{
    IntoResponses,
    ToSchema,
};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
enum HealthStatus {
    Healthy,
    NoDatabaseConnection,
}

/// Health of the server
#[derive(Serialize, ToSchema, IntoResponses)]
#[response(status = OK)]
pub struct HealthResponse {
    #[schema(inline)]
    status: HealthStatus,
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Check if the server is up and running
#[utoipa::path(get, path = "/health", responses(HealthResponse), tag = super::TAG)]
pub async fn health(State(state): State<crate::AppState>) -> Json<HealthResponse> {
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
