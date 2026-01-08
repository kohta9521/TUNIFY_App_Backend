use axum::response::Json;
use serde::Serialize;

use crate::application::status_service::StatusService;
use crate::domains::status::ApiStatus;

#[derive(Serialize)]
pub struct StatusResponse {
    message: String,
}

pub async fn api_status() -> Json<StatusResponse> {
    tracing::debug!("Handling /api/v1/status request");
    
    let service = StatusService::new();
    let ApiStatus { message } = service.get_status();
    
    tracing::debug!("Status response: {}", message);
    Json(StatusResponse { message })
}