use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn get_conversation(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path((agent_id, id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<Value>> {
    let conversations = state
        .services
        .conversation_service
        .get_conversation_by_conversation_id_and_agent_id_and_user_id(id, agent_id, user_id)
        .await?;
    Ok(Json(json!(conversations)))
}
