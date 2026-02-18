use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};

pub async fn list_agents(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> AppResult<Json<Value>> {
    let agents = state
        .services
        .agent_service
        .get_agent_states_list(user_id)
        .await?;
    Ok(Json(json!(agents)))
}
