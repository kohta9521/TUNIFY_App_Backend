use axum::response::Json;
use serde::Serialize;

use crate::application::health_service::{HealthService, HealthStatus};

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn health_check() -> Json<HealthResponse> {
    tracing::debug!("Handling /api/v1/health request");
    
    let service = HealthService::new();
    let HealthStatus { status } = service.get_health();
    
    tracing::debug!("Health response: {}", status);
    Json(HealthResponse { status })
}