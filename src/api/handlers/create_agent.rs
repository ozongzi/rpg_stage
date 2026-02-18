use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::extract::State;
use axum::{Form, Json};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateAgentForm {
    pub agent_metadata_id: Uuid,
}

pub async fn create_agent(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Form(agent_form): Form<CreateAgentForm>,
) -> AppResult<Json<Value>> {
    let id = state
        .services
        .agent_service
        .create_returning_id(user_id, agent_form.agent_metadata_id)
        .await?;
    Ok(Json(json!({ "agent_id": id })))
}
