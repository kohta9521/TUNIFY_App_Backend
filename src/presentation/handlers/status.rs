use axum::response::Json;
use serde::Serialize;

use crate::application::status_service::StatusService;
use crate::domains::status::ApiStatus;

#[derive(Serialize)]
pub struct StatusResponse {
    status: String,
    message: String,
    status_code: u16,
}

pub async fn api_status() -> Json<StatusResponse> {
    tracing::debug!("Handling /api/v1/status request");
    
    let service = StatusService::new();
    let ApiStatus { status, message, status_code } = service.get_status();
    
    tracing::debug!("Status response: {} (code: {})", message, status_code);
    Json(StatusResponse { 
        status, 
        message, 
        status_code 
    })
}