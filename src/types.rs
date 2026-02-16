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
