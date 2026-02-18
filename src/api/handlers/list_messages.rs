use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::Value;
use uuid::Uuid;

pub async fn list_messages(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(conversation_id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    Ok(Json(
        state
            .services
            .chat_service
            .get_messages_list(user_id, conversation_id)
            .await?,
    ))
}
