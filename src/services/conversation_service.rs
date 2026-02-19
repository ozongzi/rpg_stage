use crate::domains::Conversation;
use crate::errors::AppResult;
use crate::repositories::agent_repository::AgentRepository;
use crate::repositories::conversation_repository::ConversationRepository;
use crate::repositories::user_repository::UserRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct ConversationService {
    repo: ConversationRepository,
    agent_repo: AgentRepository,
    user_repo: UserRepository,
}

impl ConversationService {
    pub fn new(
        repo: ConversationRepository,
        agent_repo: AgentRepository,
        user_repo: UserRepository,
    ) -> Self {
        Self {
            repo,
            agent_repo,
            user_repo,
        }
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

    pub async fn delete_conversation_by_user_id_and_agent_id_and_conversation_id(
        &self,
        user_id: Uuid,
        agent_id: Uuid,
        conversation_id: Uuid,
    ) -> AppResult<()> {
        if !self.user_repo.is_admin(user_id).await? {
            self.repo
                .assert_conversation_belongs_to_agent_id_and_user_id(conversation_id, agent_id, user_id)
                .await?;
        }

        self.repo
            .delete_conversation_by_conversation_id(conversation_id)
            .await
    }

    pub async fn get_conversation_by_conversation_id_and_agent_id_and_user_id(
        &self,
        conversation_id: Uuid,
        agent_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Conversation> {
        if !self.user_repo.is_admin(user_id).await? {
            self.repo
                .assert_conversation_belongs_to_agent_id_and_user_id(conversation_id, agent_id, user_id)
                .await?;
        }
        
        self.repo.get_conversation(conversation_id).await
    }
}
