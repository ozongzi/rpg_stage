use axum::http::StatusCode;
use uuid::Uuid;

use crate::domains::User;
use crate::{
    domains::{Email, UserName, UserPassword},
    errors::{AppError, AppResult},
    repositories::user_repository::UserRepository,
};

pub struct CreateUserInput {
    pub name: UserName,
    pub email: Email,
    pub password: UserPassword,
}

#[derive(Debug, Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, request: CreateUserInput) -> AppResult<Uuid> {
        let password_hash = bcrypt::hash(request.password.as_ref(), bcrypt::DEFAULT_COST)
            .map_err(|_| AppError(StatusCode::INTERNAL_SERVER_ERROR, "密码哈希失败".into()))?;

        self.repo
            .insert_user(request.name, request.email, password_hash)
            .await
    }

    pub async fn assert_admin(&self, user_id: Uuid) -> AppResult<()> {
        let is_admin = self.repo.is_admin(user_id).await?;

        if !is_admin {
            return Err(AppError(StatusCode::FORBIDDEN, serde_json::Value::Null));
        }

        Ok(())
    }

    pub async fn is_vip(&self, user_id: Uuid) -> AppResult<bool> {
        self.repo.is_vip(user_id).await
    }

    pub async fn list_users(&self) -> AppResult<Vec<User>> {
        self.repo.get_user_list_without_password_hash().await
    }
}
