use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use crate::domains::User;
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
        info!("Getting user by email {:?}", email);
        let result = sqlx::query!(
            "SELECT id, name, email, password_hash FROM users WHERE email = $1",
            email.as_ref()
        )
        .fetch_one(&self.pool)
        .await?;
        info!("result = {:?}", result);

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

    pub async fn get_user_without_password_hash(&self, id: Uuid) -> AppResult<User> {
        let result = sqlx::query!(r#"SELECT name, email FROM users WHERE id = $1"#, id)
            .fetch_one(&self.pool)
            .await?;

        Ok(User::new(
            id,
            result.name.parse()?,
            result.email.parse()?,
            "已隐藏".to_string(),
        ))
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

    pub async fn get_user_by_id(&self, user_id: Uuid) -> AppResult<User> {
        let record = sqlx::query!(
            "SELECT id, name, email, password_hash FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User::new(
            record.id,
            record.name.parse()?,
            record.email.parse()?,
            record.password_hash,
        ))
    }

    pub async fn update_user_by_id(&self, user_id: Uuid, user: User) -> AppResult<()> {
        sqlx::query!(
            "update users set name = $1, email = $2, password_hash = $3 where id = $4",
            user.name().as_ref(),
            user.email().as_ref(),
            user.password_hash(),
            user_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_user_by_id(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query!("delete from users where id = $1", user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
