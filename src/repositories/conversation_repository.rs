use crate::domains::Conversation;
use crate::errors::AppResult;
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
}
