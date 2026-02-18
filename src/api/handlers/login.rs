use std::time::Duration;

use axum::{
    Form,
    extract::State,
    http::{Response, StatusCode},
};
use chrono::Utc;
use serde::Deserialize;
use sha2::Digest;
use sqlx::query;
use uuid::Uuid;

use crate::errors::AppResult;
use crate::{app_state::AppState, domains::Email};

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> AppResult<String> {
    let email = form.email.parse()?;
    let password = form.password.parse()?;

    state
        .services
        .session_service
        .authenticate_user(email, password)
        .await
}
