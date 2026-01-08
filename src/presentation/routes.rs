use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use sqlx::PgPool;

use crate::application::user_service::UserService;
use crate::presentation::handlers::{health::health_check, status::api_status, user::get_user_info};

/// ルーターを作成
/// 
/// # 引数
/// - `pool`: データベース接続プール
/// 
/// # 戻り値
/// - `Router`: Axumルーター
pub async fn create_router(pool: PgPool) -> Router {
    // ユーザーサービスを作成
    let user_service = UserService::new(pool);

    Router::new()
        .route("/api/v1/status", get(api_status))
        .route("/api/v1/health", get(health_check))
        .route("/api/v1/users/{user_id}", get(get_user_info))
        .with_state(user_service)  // ステートとしてUserServiceを渡す
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