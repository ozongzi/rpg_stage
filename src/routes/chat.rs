use axum::{Json, extract::State, http::StatusCode};
use ds_api::Message;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::info;
use uuid::Uuid;

use crate::{app_state::AppState, auth_user::AuthUser, types::DbMessage};

#[derive(Deserialize)]
pub struct ChatMessage {
    agent_id: Uuid,
    conversation_id: Uuid,
    content: String,
}

#[axum::debug_handler]
pub async fn chat(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(chat_message): Json<ChatMessage>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // get user
    let user = sqlx::query!("select name, vip from users where id = $1", user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            info!("Failed to fetch user: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to fetch user",
                })),
            )
        })?;
    // get agent

    let agent = sqlx::query!(
        r#"select name, emotion, favorability,
        character_design, response_requirement, character_emotion_split,
        model, temperature, max_tokens
        from agents where id = $1 and user_id = $2"#,
        chat_message.agent_id,
        user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        info!("agent not found : {e}");
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "agent not found",
            })),
        )
    })?;

    // get conversation
    let title = sqlx::query!(
        "select title from conversations where id = $1 and agent_id = $2 and user_id = $3",
        chat_message.conversation_id,
        chat_message.agent_id,
        user_id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        info!("conversation not found : {e}");
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "conversation not found",
            })),
        )
    })?;

    // get messages
    let messages = sqlx::query_as!(
        DbMessage,
        r#"select * from messages where conversation_id = $1 order by message_index asc"#,
        chat_message.conversation_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        info!("Failed to fetch messages: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to fetch messages",
            })),
        )
    })?
    .into_iter()
    .map(Message::try_from)
    .collect::<Result<Vec<Message>, _>>()
    .map_err(|x| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": x,
            })),
        )
    })?;

    match crate::chat::chat(
        &state.deepseek_token,
        agent.emotion,
        agent.favorability,
        agent.character_design,
        agent.response_requirement,
        agent.character_emotion_split,
        agent.model,
        agent.temperature,
        agent.max_tokens,
        title.title.unwrap_or_default(),
        messages,
        chat_message.content.clone(),
    )
    .await
    {
        Ok((resp, message)) => {
            // save message
            sqlx::query!(
                r#"insert into messages (
                    conversation_id,
                    role,
                    content,
                    message_index
                )
                values (
                    $1,
                    $2,
                    $3,
                    next_message_index($1)
                )
                "#,
                chat_message.conversation_id,
                "user",
                chat_message.content
            )
            .execute(&state.db)
            .await
            .map_err(|e| {
                info!("Failed to save message: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Failed to save message",
                    })),
                )
            })?;

            sqlx::query!(
                r#"insert into messages (
                    conversation_id,
                    role,
                    content,
                    message_index
                )
                values (
                    $1,
                    $2,
                    $3,
                    next_message_index($1)
                )
                "#,
                chat_message.conversation_id,
                "assistant",
                message.content
            )
            .execute(&state.db)
            .await
            .map_err(|e| {
                info!("Failed to save message: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Failed to save message",
                    })),
                )
            })?;

            let mut js = json!({
                "content": resp.response,
                "name": agent.name,
                "emotion": resp.current_emotion,
                "favorability": resp.new_favorability,
            });

            if user.vip {
                js["mind"] = json!(resp.mind);
            }

            Ok(Json(js))
        }
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": e.to_string(),
            })),
        )),
    }
}
