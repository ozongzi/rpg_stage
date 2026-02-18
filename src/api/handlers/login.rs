use axum::{Form, extract::State};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::errors::AppResult;

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
