use sqlx::{PgPool, query_as};
use std::result;
use uuid::Uuid;

use crate::domains::{User, UserPassword};
use crate::{
    domains::{Email, UserName},
    errors::AppResult,
};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_user(
        &self,
        name: UserName,
        email: Email,
        password_hash: String,
    ) -> AppResult<Uuid> {
        let record = sqlx::query!(
            r#"insert into users (name, email, password_hash)
             values ($1, $2, $3)
             returning id"#,
            name.as_ref(),
            email.as_ref(),
            &password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }

    pub async fn get_user_by_email(&self, email: Email) -> AppResult<User> {
        let result = sqlx::query!(
            "SELECT id, name, email, password_hash FROM users WHERE email = $1",
            email.as_ref()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User::new(
            result.id,
            result.name.parse()?,
            result.email.parse()?,
            result.password_hash,
        ))
    }

    pub async fn get_user_list_without_password_hash(&self) -> AppResult<Vec<User>> {
        let results = sqlx::query!(r#"SELECT id, name, email FROM users"#)
            .fetch_all(&self.pool)
            .await?;

        let results = results
            .into_iter()
            .map(|x| {
                Ok(User::new(
                    x.id,
                    x.name.parse()?,
                    x.email.parse()?,
                    "已隐藏".to_string(),
                ))
            })
            .collect::<AppResult<Vec<_>>>()?;

        Ok(results)
    }

    pub async fn is_admin(&self, user_id: Uuid) -> AppResult<bool> {
        let exists = sqlx::query_scalar!(
            "select exists(
                select 1 from users
                where id = $1 and is_admin = true
            )",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }

    pub async fn is_vip(&self, user_id: Uuid) -> AppResult<bool> {
        let exists = sqlx::query_scalar!(
            "select exists(
                select 1 from users
                where id = $1 and vip = true
            )",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
