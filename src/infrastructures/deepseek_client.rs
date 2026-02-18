use crate::domains::{ChatAgent, ChatMessage};
use crate::errors::{AppError, AppResult};
use axum::http::StatusCode;
use ds_api::{Message, Response as _, Role};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub new_favorability: i32,
    pub current_emotion: String,
    pub response: String,
    pub mind: String,
    pub new_memory: Option<String>,
}

fn message_to_chat_message(message: Message) -> ChatMessage {
    ChatMessage {
        role: message.role,
        content: message.content,
        name: message.name,
        tool_call_id: message.tool_call_id,
        tool_calls: message.tool_calls.map(|x| json!(x)),
        reasoning_content: message.reasoning_content,
    }
}

fn chat_message_to_message(message: ChatMessage) -> Message {
    Message {
        role: message.role,
        content: message.content,
        name: message.name,
        tool_call_id: message.tool_call_id,
        tool_calls: message
            .tool_calls
            .map(|x| serde_json::from_value(x).unwrap()),
        reasoning_content: message.reasoning_content,
        prefix: None,
    }
}

#[derive(Clone, Debug)]
pub struct DeepseekClient {
    token: String,
    client: Client,
}

impl DeepseekClient {
    pub fn new(deepseek_token: String, client: Client) -> Self {
        Self {
            token: deepseek_token,
            client,
        }
    }

    pub async fn get_chat_history_via_chat_messages(
        &self,
        chat_messages: Vec<ChatMessage>,
    ) -> AppResult<Value> {
        let mut history = vec![];
        for message in chat_messages {
            match message.role {
                Role::User => history.push(json!({
                "role": "user",
                "content": message.content,
                })),
                Role::Assistant => {
                    if let Ok(response) = serde_json::from_str::<Response>(
                        message
                            .content
                            .ok_or(AppError(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "message错误".into(),
                            ))?
                            .as_str(),
                    ) {
                        history.push(json!({
                        "role": "user",
                        "content": response.response,
                        }))
                    }
                }
                _ => {}
            }
        }
        Ok(json!(history))
    }
    pub async fn chat(
        &self,
        agent: ChatAgent,
        _title: String,
        messages: Vec<ChatMessage>,
        memories: Vec<String>,
    ) -> AppResult<(Response, ChatMessage)> {
        let model = match agent.model.as_str() {
            "deepseek-chat" => ds_api::Model::DeepseekChat,
            "deepseek-reasoner" => ds_api::Model::DeepseekReasoner,
            _ => return Err(AppError(StatusCode::BAD_REQUEST, "Invalid model".into())),
        };

        let mut messages = messages
            .into_iter()
            .map(chat_message_to_message)
            .collect::<Vec<_>>();

        let system_prompt = generate_system_prompt(
            agent.emotion,
            agent.favorability,
            agent.character_design,
            agent.response_requirement,
            agent.character_emotion_split,
            memories,
        );

        let mut c_messages = vec![Message::new(Role::System, &system_prompt)];

        c_messages.append(&mut messages);

        // c_messages.push(Message::new(Role::User, &content));

        let mut request = ds_api::Request::builder().messages(c_messages).model(model);

        if let Some(temperature) = agent.temperature {
            request = request.temperature(temperature as f32);
        }

        if let Some(max_tokens) = agent.max_tokens {
            request = request.max_tokens(max_tokens as u32);
        }

        let response = request
            .execute_client_nostreaming(&mut self.client.clone(), &self.token)
            .await
            .map_err(|e| AppError(StatusCode::BAD_REQUEST, e.to_string().into()))?;

        Ok((
            serde_json::from_str(response.content())
                .map_err(|e| AppError(StatusCode::BAD_REQUEST, e.to_string().into()))?,
            message_to_chat_message(response.choices[0].message.clone()),
        ))
    }
}

fn generate_system_prompt(
    emotion: String,
    favorability: i32,
    character_design: String,
    response_requirement: String,
    character_emotion_split: String,
    memories: Vec<String>,
) -> String {
    let lines = character_emotion_split
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().to_string())
        .collect::<Vec<_>>();

    let emotion_description = if let Some(x) = lines.iter().find(|x| {
        x.split(" ")
            .next()
            .unwrap()
            .split("..=")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| favorability >= w[0] && favorability <= w[1])
    }) {
        x.split(" ").nth(2).unwrap().to_string()
    } else {
        "".to_string()
    };

    let memories = memories
        .iter()
        .enumerate()
        .map(|(i, x)| format!("{}: {}", i, x.trim()))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{}\n你当前的情绪是:{}\n你当前的好感度是:{}\n{}\n相关记忆：{}, {}",
        character_design, emotion, favorability, emotion_description, memories ,response_requirement
    )
}
