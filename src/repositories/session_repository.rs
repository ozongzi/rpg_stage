use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppResult;

#[derive(Debug, Clone)]
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_active_user_id_by_token_hash(&self, token_hash: String) -> AppResult<Uuid> {
        let record = sqlx::query!(
            r#"select user_id from sessions where token_hash = $1 and expires_at > now()"#,
            token_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.user_id)
    }

    pub async fn insert_user_id_and_token_hash(
        &self,
        user_id: Uuid,
        token_hash: String,
        expires_at: DateTime<Utc>,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"insert into sessions (user_id, token_hash, expires_at) values ($1, $2, $3)"#,
            user_id,
            token_hash,
            expires_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_session_by_token_hash(&self, token_hash: String) -> AppResult<()> {
        sqlx::query!(r#"delete from sessions where token_hash = $1"#, token_hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
