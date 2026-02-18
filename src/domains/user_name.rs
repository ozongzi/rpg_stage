use crate::errors::AppError;
use axum::http::StatusCode;
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize, Debug, Clone)]
pub struct UserName(String);

impl FromStr for UserName {
    type Err = AppError;
    fn from_str(name: &str) -> Result<Self, AppError> {
        if name.chars().any(|c| "/\'\"<>&!@#^*();,".contains(c)) {
            Err(AppError(
                StatusCode::BAD_REQUEST,
                "用户名包含非法字符".into(),
            ))
        } else if name.chars().count() > 100 {
            Err(AppError(StatusCode::BAD_REQUEST, "用户名太长".into()))
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
