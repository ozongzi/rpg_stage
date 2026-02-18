use crate::domains::MetaAgent;
use crate::errors::AppResult;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AgentMetadataRepository {
    pool: PgPool,
}

impl AgentMetadataRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_metadata_by_id(&self, id: Uuid) -> AppResult<MetaAgent> {
        let meta = sqlx::query_as!(
            MetaAgent,
            "SELECT name, description, character_design, response_requirement, character_emotion_split, model FROM agent_metadata WHERE id = $1",
            id
        ).fetch_one(&self.pool).await?;

        Ok(meta)
    }

    pub async fn insert_metadata(&self, meta: &MetaAgent) -> AppResult<Uuid> {
        let record = sqlx::query!(
            r#"insert into agent_metadata (name, description, character_design, response_requirement, character_emotion_split, model) values ($1, $2, $3, $4, $5, $6) returning id"#,
            meta.name, meta.description, meta.character_design, meta.response_requirement, meta.character_emotion_split, meta.model
        ).fetch_one(&self.pool).await?;

        Ok(record.id)
    }
}
