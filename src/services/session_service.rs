use sha2::Digest;
use uuid::Uuid;

use crate::{errors::AppResult, repositories::session_repository::SessionRepository};

#[derive(Debug, Clone)]
pub struct SessionService {
    repo: SessionRepository,
    user_repo: UserRepository,
}

use crate::domains::{Email, SessionInfo, UserPassword};
use crate::errors::AppError;
use crate::repositories::user_repository::UserRepository;
use base64::{Engine as _, engine::general_purpose};
use chrono::{Duration, Utc};
use reqwest::StatusCode;

pub fn generate_token() -> String {
    let bytes: [u8; 32] = rand::random();
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

impl SessionService {
    pub fn new(repo: SessionRepository, user_repo: UserRepository) -> Self {
        Self { repo, user_repo }
    }

    pub async fn authenticate(&self, token: &str) -> AppResult<Uuid> {
        let token_hash = hex::encode(sha2::Sha256::digest(token));
        self.repo
            .find_active_user_id_by_token_hash(token_hash)
            .await
    }

    pub async fn authenticate_user(
        &self,
        email: Email,
        password: UserPassword,
    ) -> AppResult<String> {
        let user = self.user_repo.get_user_by_email(email).await?;
        if !bcrypt::verify(password.as_ref(), user.password_hash())? {
            return Err(AppError(StatusCode::BAD_REQUEST, "邮箱或密码错误".into()));
        }

        let token = generate_token();
        let token_hash = hex::encode(sha2::Sha256::digest(token.clone()));

        let expires_at = Utc::now() + Duration::days(30);

        self.repo
            .insert_user_id_and_token_hash(user.id(), token_hash, expires_at)
            .await?;

        Ok(token)
    }
    
    pub async fn get_session_info_list(&self) -> AppResult<Vec<SessionInfo>> {
        self.repo.fetch_session_infos().await
    }
    
    pub async fn delete_session_by_id(&self, session_id: Uuid) -> AppResult<()> {
        self.repo.delete_session_by_id(session_id).await
    }

    pub async fn invalidate_session(&self, token: &str) -> AppResult<()> {
        let token_hash = hex::encode(sha2::Sha256::digest(token));
        self.repo.delete_session_by_token_hash(token_hash).await
    }
}
