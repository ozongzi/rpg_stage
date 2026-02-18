use crate::domains::ChatAgent;
use crate::errors::AppResult;
use crate::repositories::session_repository::SessionRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AgentRepository {
    pool: PgPool,
}

impl AgentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_agent_with_agent_id_and_user_id(
        &self,
        agent_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<ChatAgent> {
        let agent = sqlx::query_as!(
            ChatAgent,
            r#"select name, emotion, favorability, character_design, response_requirement,
            character_emotion_split, model, temperature, max_tokens
            from agents where id = $1 and user_id = $2"#,
            agent_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(agent)
    }

    pub async fn insert_memory(&self, agent_id: Uuid, memory: &str) -> AppResult<()> {
        sqlx::query!(
            r#"insert into agent_memories (agent_id, content) values ($1, $2)"#,
            agent_id,
            memory,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    
    pub async fn get_memories(&self, agent_id: Uuid) -> AppResult<Vec<String>> {
        let memories = sqlx::query!(
            r#"select content from agent_memories where agent_id = $1"#,
            agent_id
        ).fetch_all(&self.pool).await?;
        Ok(memories.into_iter().map(|x| x.content).collect())
    }
}
