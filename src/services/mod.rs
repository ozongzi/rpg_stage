mod chat_service;
pub mod session_service;
pub mod user_service;

use crate::repositories::agent_repository::AgentRepository;
use crate::repositories::message_repository::MessageRepository;
use crate::repositories::session_repository::SessionRepository;
use crate::repositories::user_repository::UserRepository;
use crate::services::chat_service::ChatService;
use session_service::SessionService;
use sqlx::PgPool;
use user_service::UserService;
use crate::infrastructures::deepseek_client::DeepseekClient;

#[derive(Clone)]
pub struct Services {
    pub user_service: UserService,
    pub session_service: SessionService,
    pub chat_service: ChatService,
}

impl Services {
    pub fn install(pool: &PgPool, deepseek_token: String) -> Self {
        let user_repository = UserRepository::new(pool.clone());
        let session_repository = SessionRepository::new(pool.clone());
        let message_repository = MessageRepository::new(pool.clone());
        let agent_repository = AgentRepository::new(pool.clone());
        
        let deepseek_client = DeepseekClient::new(deepseek_token, reqwest::Client::new());

        let user_service = UserService::new(user_repository.clone());
        let session_service = SessionService::new(session_repository, user_repository.clone());
        let chat_service = ChatService::new(
            deepseek_client,
            user_repository.clone(),
            agent_repository,
            message_repository,
        );

        Self {
            user_service,
            session_service,
            chat_service,
        }
    }
}
