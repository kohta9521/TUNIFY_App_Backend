use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::presentation::handlers::{health::health_check, status::api_status};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v1/status", get(api_status))
        .route("/api/v1/health", get(health_check))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|_request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::info!("📥 Incoming request");
                })
                .on_response(|_response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!("📤 Response sent (latency: {:?})", latency);
                })
        )
}