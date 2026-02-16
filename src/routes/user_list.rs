use crate::{app_state::AppState, auth_user::AuthUserAdmin};
use axum::{
    Json,
    extract::State,
    http::{Response, StatusCode},
};
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct UserResponse {
    id: Uuid,
    email: String,
}

pub async fn user_list(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
) -> Result<Json<Value>, Response<String>> {
    let users = sqlx::query_as!(UserResponse, "SELECT id, email FROM users")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("数据库错误:{:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("数据库错误: {:?}", e))
                .unwrap()
        })?;

    Ok(Json(json!(users)))
}
