use crate::domains::{AgentState, ChatAgent, MetaAgent};
use crate::errors::{AppError, AppResult};
use crate::repositories::session_repository::SessionRepository;
use axum::http::StatusCode;
use serde_json::json;
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

    pub async fn assert_agent_belongs_to_user(
        &self,
        agent_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<()> {
        let record = sqlx::query!(r#"SELECT user_id FROM agents WHERE id = $1"#, agent_id)
            .fetch_optional(&self.pool)
            .await?;

        match record {
            Some(r) if r.user_id == user_id => Ok(()),
            Some(_) => Err(AppError(
                StatusCode::FORBIDDEN,
                json!("该 Agent 不属于当前用户"),
            )),
            None => {
                tracing::debug!("不存在 agent_id = {}", agent_id);
                Err(AppError(StatusCode::NOT_FOUND, json!("Agent 不存在")))
            }
        }
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
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(memories.into_iter().map(|x| x.content).collect())
    }

    pub async fn insert_agent_with_metadata(
        &self,
        user_id: Uuid,
        agent_meta: MetaAgent,
    ) -> AppResult<Uuid> {
        let record = sqlx::query!(
            r#"insert into agents (user_id, name, emotion, favorability, character_design, response_requirement, character_emotion_split, model)
        values ($1, $2, $3, $4, $5, $6, $7, $8) returning id"#,
            user_id,
            agent_meta.name,
            "",
            0,
            agent_meta.character_design,
            agent_meta.response_requirement,
            agent_meta.character_emotion_split,
            agent_meta.model,
        ).fetch_one(&self.pool).await?;

        Ok(record.id)
    }

    pub async fn fetch_agent_state_list_by_user_id(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<AgentState>> {
        let records = sqlx::query_as!(
            AgentState,
            r#"select id, name, emotion, favorability from agents where user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }

    pub async fn fetch_agent_state_by_user_id_and_agent_id(
        &self,
        user_id: Uuid,
        agent_id: Uuid,
    ) -> AppResult<AgentState> {
        let record = sqlx::query_as!(
            AgentState,
            r#"select id, name, emotion, favorability from agents where user_id = $1 and id = $2"#,
            user_id,
            agent_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }
}
