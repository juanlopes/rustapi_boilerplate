use axum::Json;
use crate::dto::health_response::HealthResponse;

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        message: "API is running".into(),
    })
}
