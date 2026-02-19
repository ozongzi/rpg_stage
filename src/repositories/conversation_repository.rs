use crate::domains::Conversation;
use crate::errors::{AppError, AppResult};
use axum::http::StatusCode;
use sqlx::PgPool;

use uuid::Uuid;

#[derive(Clone)]
pub struct ConversationRepository {
    pool: PgPool,
}

impl ConversationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_conversation(&self, user_id: Uuid, agent_id: Uuid) -> AppResult<Uuid> {
        let record = sqlx::query!(
            "INSERT INTO conversations (user_id, agent_id) VALUES ($1, $2) returning id",
            user_id,
            agent_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }

    pub async fn fetch_all_conversation_with_agent_id_and_user_id(
        &self,
        agent_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Vec<Conversation>> {
        let records = sqlx::query_as!(
            Conversation,
            "SELECT id, title FROM conversations WHERE agent_id = $1 AND user_id = $2",
            agent_id,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(records)
    }

    pub async fn get_conversation(&self, conversation_id: Uuid) -> AppResult<Conversation> {
        let record = sqlx::query_as!(
            Conversation,
            "SELECT id, title FROM conversations WHERE id = $1",
            conversation_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn assert_conversation_belongs_to_agent_id_and_user_id(
        &self,
        conversation_id: Uuid,
        agent_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()> {
        let exists = sqlx::query_scalar!(
            r#"
        SELECT EXISTS (
            SELECT 1 FROM conversations
            WHERE id = $1
              AND agent_id = $2
              AND user_id = $3
        )
        "#,
            conversation_id,
            agent_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        if !exists.unwrap_or(false) {
            return Err(AppError(
                StatusCode::NOT_FOUND,
                "Conversation not found or access denied".into(),
            ));
        }

        Ok(())
    }

    pub async fn delete_conversation_by_conversation_id(
        &self,
        conversation_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query!("DELETE FROM conversations WHERE id = $1", conversation_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
