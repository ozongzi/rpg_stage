use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MetaAgent {
    pub name: String,
    pub description: String,
    pub character_design: String,
    pub response_requirement: String,
    pub character_emotion_split: String,
    pub model: String,
}
