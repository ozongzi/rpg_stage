mod agent_service;
mod chat_service;
mod conversation_service;
pub mod session_service;
pub mod user_service;

use crate::infrastructures::deepseek_client::DeepseekClient;
use crate::repositories::agent_metadata_repository::AgentMetadataRepository;
use crate::repositories::agent_repository::AgentRepository;
use crate::repositories::conversation_repository::ConversationRepository;
use crate::repositories::message_repository::MessageRepository;
use crate::repositories::session_repository::SessionRepository;
use crate::repositories::user_repository::UserRepository;
use crate::services::agent_service::AgentService;
use crate::services::chat_service::ChatService;
use crate::services::conversation_service::ConversationService;
use session_service::SessionService;
use sqlx::PgPool;
use user_service::UserService;

#[derive(Clone)]
pub struct Services {
    pub user_service: UserService,
    pub session_service: SessionService,
    pub chat_service: ChatService,
    pub agent_service: AgentService,
    pub conversation_service: ConversationService,
}

impl Services {
    pub fn install(pool: &PgPool, deepseek_token: String) -> Self {
        let user_repository = UserRepository::new(pool.clone());
        let session_repository = SessionRepository::new(pool.clone());
        let message_repository = MessageRepository::new(pool.clone());
        let agent_repository = AgentRepository::new(pool.clone());
        let agent_metadata_repository = AgentMetadataRepository::new(pool.clone());
        let conversation_repository = ConversationRepository::new(pool.clone());

        let deepseek_client = DeepseekClient::new(deepseek_token, reqwest::Client::new());

        let user_service = UserService::new(user_repository.clone());
        let session_service = SessionService::new(session_repository, user_repository.clone());
        let chat_service = ChatService::new(
            deepseek_client,
            user_repository.clone(),
            agent_repository.clone(),
            message_repository,
        );
        let agent_service =
            AgentService::new(agent_repository.clone(), agent_metadata_repository.clone());
        let conversation_service =
            ConversationService::new(conversation_repository.clone(), agent_repository.clone());

        Self {
            user_service,
            session_service,
            chat_service,
            agent_service,
            conversation_service,
        }
    }
}
