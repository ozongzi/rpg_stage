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

    pub async fn list_users(&self) -> AppResult<Vec<User>> {
        self.repo.get_user_list_without_password_hash().await
    }

    pub async fn get_user(&self, id: Uuid) -> AppResult<User> {
        self.repo.get_user_without_password_hash(id).await
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        user_name: Option<UserName>,
        email: Option<Email>,
        new_password: Option<UserPassword>,
    ) -> AppResult<()> {
        let user = self.repo.get_user_by_id(id).await?;
        let new_password_hash = new_password
            .map(|p| bcrypt::hash(p.as_ref(), bcrypt::DEFAULT_COST))
            .transpose()?;
        self.repo
            .update_user_by_id(
                id,
                User::new(
                    id,
                    user_name.unwrap_or(user.name().clone()),
                    email.unwrap_or(user.email().clone()),
                    new_password_hash.unwrap_or(user.password_hash().to_string()),
                ),
            )
            .await
    }

    pub async fn update_user_self(
        &self,
        id: Uuid,
        old_password: UserPassword,
        user_name: Option<UserName>,
        email: Option<Email>,
        new_password: Option<UserPassword>,
    ) -> AppResult<()> {
        let user = self.repo.get_user_by_id(id).await?;
        if bcrypt::verify(old_password.as_ref(), user.password_hash())? {
            return Err(AppError(StatusCode::FORBIDDEN, "密码错误".into()));
        }
        let new_password_hash = new_password
            .map(|p| bcrypt::hash(p.as_ref(), bcrypt::DEFAULT_COST))
            .transpose()?;
        self.repo
            .update_user_by_id(
                id,
                User::new(
                    id,
                    user_name.unwrap_or(user.name().clone()),
                    email.unwrap_or(user.email().clone()),
                    new_password_hash.unwrap_or(user.password_hash().to_string()),
                ),
            )
            .await
    }
    
    pub async fn delete_user_by_id(&self, id: Uuid) -> AppResult<()> {
        self.repo.delete_user_by_id(id).await
    }
}
