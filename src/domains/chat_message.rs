pub use ds_api::Role;

#[derive(Default)]
pub struct ChatMessage {
    pub role: Role,
    pub content: Option<String>,
    pub name: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_calls: Option<serde_json::Value>,
    pub reasoning_content: Option<String>,
}

impl ChatMessage {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role,
            content: Some(content),
            ..Default::default()
        }
    }
}
