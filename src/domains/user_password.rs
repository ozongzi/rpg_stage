use crate::errors::AppError;
use axum::http::StatusCode;
use std::str::FromStr;
#[derive(Debug)]
pub struct UserPassword(String);

impl FromStr for UserPassword {
    type Err = AppError;
    fn from_str(password: &str) -> Result<Self, AppError> {
        if password.len() > 20 {
            Err(AppError(StatusCode::BAD_REQUEST, "密码太长".into()))
        } else {
            Ok(Self(password.to_string()))
        }
    }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
