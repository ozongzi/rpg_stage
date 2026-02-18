use crate::errors::AppError;
use axum::http::StatusCode;
use std::str::FromStr;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]

pub struct Email(String);

impl FromStr for Email {
    type Err = AppError;
    fn from_str(email: &str) -> Result<Self, AppError> {
        if validator::ValidateEmail::validate_email(&email) {
            Ok(Self(email.to_string()))
        } else {
            Err(AppError(StatusCode::BAD_REQUEST, "邮箱格式不正确".into()))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
