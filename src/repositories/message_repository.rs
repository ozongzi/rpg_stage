#[allow(unused)]
#[derive(Clone, Debug)]
pub struct DbMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub role: String,
    pub content: Option<String>,
    pub name: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_calls: Option<serde_json::Value>,
    pub reasoning_content: Option<String>,
    pub message_index: i32,
    pub input_tokens: Option<i32>,
    pub output_tokens: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

use crate::domains::ChatMessage;
use crate::errors::{AppError, AppResult};
use axum::http::StatusCode;
use ds_api::Role;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

impl TryFrom<DbMessage> for ChatMessage {
    type Error = AppError;

    fn try_from(value: DbMessage) -> Result<Self, Self::Error> {
        Ok(ChatMessage {
            role: match value.role.as_str() {
                "user" => Role::User,
                "assistant" => Role::Assistant,
                "system" => Role::System,
                "tool" => Role::Tool,
                _ => return Err(AppError(StatusCode::BAD_REQUEST, "Invalid role".into())),
            },
            content: value.content,
            name: value.name,
            tool_call_id: value.tool_call_id,
            tool_calls: value.tool_calls,
            reasoning_content: value.reasoning_content,
        })
    }
}

#[derive(Clone)]
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn begin(&self) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
        self.pool.begin().await
    }

    pub async fn insert_message(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        conversation_id: Uuid,
        chat_message: &ChatMessage,
    ) -> Result<(), sqlx::Error> {
        let role = match chat_message.role {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
            Role::Tool => "tool",
        };

        sqlx::query!(
            r#"
            INSERT INTO messages (
                conversation_id,
                role,
                content,
                name,
                tool_call_id,
                tool_calls,
                reasoning_content,
                message_index
            )
            VALUES ($1,$2,$3,$4,$5,$6,$7,next_message_index($1))
            "#,
            conversation_id,
            role,
            chat_message.content,
            chat_message.name,
            chat_message.tool_call_id,
            chat_message.tool_calls,
            chat_message.reasoning_content
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn list_chat_messages(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        conversation_id: Uuid,
    ) -> AppResult<Vec<ChatMessage>> {
        let messages = sqlx::query_as!(
            DbMessage,
            r#"select * from messages where conversation_id = $1"#,
            conversation_id
        )
        .fetch_all(&mut **tx)
        .await?
        .into_iter()
        .map(ChatMessage::try_from)
        .collect::<Result<_, _>>()?;
        Ok(messages)
    }

    pub async fn get_agent_id_with_conversation_id_and_user_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        conversation_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Uuid> {
        let record = sqlx::query!(
            r#"select agent_id
                from conversations
                where id = $1
                and user_id = $2"#,
            conversation_id,
            user_id
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(record.agent_id)
    }
}
