use crate::domains::ChatMessage;
use crate::errors::AppResult;
use crate::infrastructures::deepseek_client::DeepseekClient;
use crate::repositories::agent_repository::AgentRepository;
use crate::repositories::message_repository::MessageRepository;
use crate::repositories::user_repository::UserRepository;
use axum::Json;
use ds_api::{Message, Role};
use serde_json::{Value, json};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatService {
    pub deepseek_client: DeepseekClient,
    pub user_repository: UserRepository,
    pub agent_repository: AgentRepository,
    pub message_repository: MessageRepository,
}

impl ChatService {
    pub fn new(
        deepseek_client: DeepseekClient,
        user_repository: UserRepository,
        agent_repository: AgentRepository,
        message_repository: MessageRepository,
    ) -> ChatService {
        Self {
            deepseek_client,
            user_repository,
            agent_repository,
            message_repository,
        }
    }

    pub async fn get_messages_list(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> AppResult<Value> {
        let is_vip = self.user_repository.is_vip(user_id).await?;

        let mut tx = self.message_repository.begin().await?;

        let agent_id = self
            .message_repository
            .get_agent_id_with_conversation_id_and_user_id(&mut tx, conversation_id, user_id)
            .await?;

        let agent = self
            .agent_repository
            .get_agent_with_agent_id_and_user_id(agent_id, user_id)
            .await?;

        let messages = self
            .message_repository
            .list_chat_messages(&mut tx, conversation_id)
            .await?;

        Ok(self
            .deepseek_client
            .get_chat_history_via_chat_messages(messages)
            .await?)
    }

    pub async fn chat(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        content: String,
    ) -> AppResult<Json<Value>> {
        let is_vip = self.user_repository.is_vip(user_id).await?;

        let mut tx = self.message_repository.begin().await?;

        let agent_id = self
            .message_repository
            .get_agent_id_with_conversation_id_and_user_id(&mut tx, conversation_id, user_id)
            .await?;

        let agent = self
            .agent_repository
            .get_agent_with_agent_id_and_user_id(agent_id, user_id)
            .await?;

        self.message_repository
            .insert_message(
                &mut tx,
                conversation_id,
                &ChatMessage::new(Role::User, content.clone()),
            )
            .await?;

        let messages = self
            .message_repository
            .list_chat_messages(&mut tx, conversation_id)
            .await?;

        let memories = self.agent_repository.get_memories(agent_id).await?;

        let (response, message) = self
            .deepseek_client
            .chat(agent.clone(), "unused".to_string(), messages, memories)
            .await?;

        self.message_repository
            .insert_message(&mut tx, conversation_id, &message)
            .await?;

        if let Some(memory) = response.new_memory {
            self.agent_repository
                .insert_memory(agent_id, &memory)
                .await?;
        }

        let mut js = json!({
            "content": response.response,
            "name": agent.name,
            "emotion": response.current_emotion,
            "favorability": response.new_favorability,
        });

        if is_vip {
            js["mind"] = json!(response.mind);
        }

        tx.commit().await?;

        Ok(Json(js))
    }
}
