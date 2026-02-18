use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::errors::AppResult;
use crate::{api::extractors::auth_user::AuthUser, app_state::AppState};

#[derive(Deserialize)]
pub struct ChatMessage {
    content: String,
}

pub async fn create_message(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(conversation_id): Path<Uuid>,
    Json(chat_message): Json<ChatMessage>,
) -> AppResult<Json<Value>> {
    Ok(state
        .services
        .chat_service
        .chat(
            user_id,
            conversation_id,
            chat_message.content,
        )
        .await?)
}
