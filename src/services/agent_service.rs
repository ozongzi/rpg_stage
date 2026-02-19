use crate::domains::MetaBrief;
use crate::domains::{AgentState, MetaAgent};
use crate::errors::AppResult;
use crate::repositories::agent_metadata_repository::AgentMetadataRepository;
use crate::repositories::agent_repository::AgentRepository;
use crate::repositories::user_repository::UserRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct AgentService {
    repo: AgentRepository,
    meta_repo: AgentMetadataRepository,
    user_repo: UserRepository,
}

impl AgentService {
    pub fn new(
        repo: AgentRepository,
        meta_repo: AgentMetadataRepository,
        user_repo: UserRepository,
    ) -> Self {
        AgentService {
            repo,
            meta_repo,
            user_repo,
        }
    }

    pub async fn create_returning_id(
        &self,
        user_id: Uuid,
        agent_metadata_id: Uuid,
    ) -> AppResult<Uuid> {
        let meta = self.meta_repo.get_metadata_by_id(agent_metadata_id).await?;
        let agent_metadata_id = self.repo.insert_agent_with_metadata(user_id, meta).await?;
        Ok(agent_metadata_id)
    }

    pub async fn get_agent_states_list(&self, user_id: Uuid) -> AppResult<Vec<AgentState>> {
        self.repo.fetch_agent_state_list_by_user_id(user_id).await
    }

    pub async fn get_agent_meta_list(&self) -> AppResult<Vec<MetaBrief>> {
        self.meta_repo.fetch_agent_meta_list().await
    }

    pub async fn new_agent_meta(&self, meta: &MetaAgent) -> AppResult<Uuid> {
        self.meta_repo.insert_metadata(meta).await
    }

    pub async fn get_agent_state(&self, user_id: Uuid, agent_id: Uuid) -> AppResult<AgentState> {
        self.repo
            .fetch_agent_state_by_user_id_and_agent_id(user_id, agent_id)
            .await
    }

    pub async fn delete_agent_by_id(&self, user_id: Uuid, agent_id: Uuid) -> AppResult<()> {
        if !self.user_repo.is_admin(user_id).await? {
            self.repo
                .assert_agent_belongs_to_user(agent_id, user_id)
                .await?;
        }

        self.repo.delete_agent_by_id(agent_id).await?;
        Ok(())
    }
}
