use axum::{extract::{Path, State}, response::Json, http::StatusCode};
use serde::Serialize;
use uuid::Uuid;

use crate::application::user_service::UserService;

/// ユーザー情報レスポンス
#[derive(Serialize)]
pub struct UserInfoResponse {
    id: String,
    email: String,
    name: Option<String>,
    created_at: String,
    updated_at: String,
}

/// エラーレスポンス
#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

/// ユーザー情報を取得するハンドラー
/// 
/// # エンドポイント
/// GET /api/v1/users/:user_id
/// 
/// # 例
/// GET /api/v1/users/550e8400-e29b-41d4-a716-446655440000
pub async fn get_user_info(
    Path(user_id): Path<String>,
    State(service): State<UserService>,
) -> Result<Json<UserInfoResponse>, (StatusCode, Json<ErrorResponse>)> {
    tracing::debug!("Handling GET /api/v1/users/{} request", user_id);

    // UUIDに変換
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            tracing::warn!("Invalid UUID format: {}", user_id);
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid UUID format".to_string(),
                }),
            ));
        }
    };

    // ユーザー情報を取得
    match service.get_user_by_id(uuid).await {
        Ok(Some(user)) => {
            tracing::debug!("User found: {}", user.email);
            Ok(Json(UserInfoResponse {
                id: user.id.to_string(),
                email: user.email,
                name: user.name,
                created_at: user.created_at.to_rfc3339(),
                updated_at: user.updated_at.to_rfc3339(),
            }))
        }
        Ok(None) => {
            tracing::warn!("User not found: {}", user_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "User not found".to_string(),
                }),
            ))
        }
        Err(e) => {
            tracing::error!("Database error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            ))
        }
    }
}

