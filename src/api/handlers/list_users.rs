use crate::errors::AppResult;
use crate::{api::extractors::auth_user::AuthUserAdmin, app_state::AppState};
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

pub async fn list_users(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
) -> AppResult<Json<Value>> {
    Ok(Json(json!(state.services.user_service.list_users().await?)))
}
