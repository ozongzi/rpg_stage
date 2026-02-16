use std::str::FromStr;
pub struct Email(String);

impl FromStr for Email {
    type Err = String;
    fn from_str(email: &str) -> Result<Self, String> {
        if validator::ValidateEmail::validate_email(&email) {
            Ok(Self(email.to_string()))
        } else {
            Err("邮箱格式不正确".to_string())
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct UserName(String);

impl FromStr for UserName {
    type Err = String;
    fn from_str(name: &str) -> Result<Self, String> {
        if name.chars().any(|c| "/\'\"<>&!@#^*();,".contains(c)) {
            Err("用户名包含非法字符".to_string())
        } else {
            Ok(Self(name.to_string()))
        }
    }
}

impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct DbMessage {
    pub id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub role: String,
    pub content: Option<String>,
    pub name: Option<String>,
    pub tool_call_id: Option<String>,
    pub tool_calls: Option<serde_json::Value>,
    pub reasoning_content: Option<String>,
    pub message_index: i32,
    pub input_tokens: Option<i32>,
    pub output_tokens: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

use ds_api::{Message, Role};

impl TryFrom<DbMessage> for Message {
    type Error = String;

    fn try_from(value: DbMessage) -> Result<Self, Self::Error> {
        Ok(Message {
            role: match value.role.as_str() {
                "user" => Role::User,
                "assistant" => Role::Assistant,
                "system" => Role::System,
                "tool" => Role::Tool,
                _ => return Err("Invalid role".to_string()),
            },
            content: value.content,
            name: value.name,
            tool_call_id: value.tool_call_id,
            tool_calls: match value.tool_calls {
                Some(tool_calls) => {
                    Some(serde_json::from_value(tool_calls).map_err(|e| e.to_string())?)
                }
                None => None,
            },
            reasoning_content: value.reasoning_content,
            prefix: None,
        })
    }
}
