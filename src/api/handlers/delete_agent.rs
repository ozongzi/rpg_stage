use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn delete_agent(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let id = state
        .services
        .agent_service
        .delete_agent_by_id(user_id, id)
        .await?;
    Ok(Json(json!({ "agent_id": id })))
}
