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
