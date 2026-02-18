use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

pub async fn create_conversation(
    State(state): State<AppState>,
    Path(agent_id): Path<Uuid>,
    AuthUser { user_id }: AuthUser,
) -> AppResult<Json<Value>> {
    // tracing::info!("agent_id = {}", agent_id);

    let id = state
        .services
        .conversation_service
        .new_conversation_with_user_id_and_agent_id(user_id, agent_id)
        .await?;

    Ok(Json(json!({"conversation_id": id})))
}
