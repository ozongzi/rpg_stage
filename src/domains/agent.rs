use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatAgent {
    pub name: String,
    pub emotion: String,
    pub favorability: i32,
    pub character_design: String,
    pub response_requirement: String,
    pub character_emotion_split: String,
    pub model: String,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgentState {
    pub id: Uuid,
    pub name: String,
    pub emotion: String,
    pub favorability: i32,
}
