use crate::domains::Conversation;
use crate::errors::AppResult;
use crate::repositories::agent_repository::AgentRepository;
use crate::repositories::conversation_repository::ConversationRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct ConversationService {
    repo: ConversationRepository,
    agent_repo: AgentRepository,
}

impl ConversationService {
    pub fn new(repo: ConversationRepository, agent_repo: AgentRepository) -> Self {
        Self { repo, agent_repo }
    }

    pub async fn new_conversation_with_user_id_and_agent_id(
        &self,
        user_id: Uuid,
        agent_id: Uuid,
    ) -> AppResult<Uuid> {
        // tracing::info!("--- agent_id = {}", agent_id);

        self.agent_repo
            .assert_agent_belongs_to_user(agent_id, user_id)
            .await?;

        let id = self.repo.insert_conversation(user_id, agent_id).await?;
        Ok(id)
    }

    pub async fn get_conversations_list(
        &self,
        agent_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Vec<Conversation>> {
        self.agent_repo
            .assert_agent_belongs_to_user(agent_id, user_id)
            .await?;

        self.repo
            .fetch_all_conversation_with_agent_id_and_user_id(agent_id, user_id)
            .await
    }
}
