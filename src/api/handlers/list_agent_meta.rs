use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};

pub async fn list_agent_meta(
    State(state): State<AppState>,
    AuthUser { user_id: _ }: AuthUser,
) -> AppResult<Json<Value>> {
    let agent_meta = state.services.agent_service.get_agent_meta_list().await?;
    Ok(Json(json!(agent_meta)))
}
