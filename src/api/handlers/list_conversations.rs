use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn list_conversations(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(agent_id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let conversations = state
        .services
        .conversation_service
        .get_conversations_list(agent_id, user_id)
        .await?;
    Ok(Json(json!(conversations)))
}
