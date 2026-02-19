use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn delete_conversation(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path((agent_id, id)): Path<(Uuid, Uuid)>,
) -> AppResult<()> {
    let _conversations = state
        .services
        .conversation_service
        .delete_conversation_by_user_id_and_agent_id_and_conversation_id(user_id, agent_id, id)
        .await?;
    Ok(())
}
