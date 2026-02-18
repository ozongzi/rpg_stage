use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn get_agent(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    let agent = state
        .services
        .agent_service
        .get_agent_state(user_id, id)
        .await?;
    Ok(Json(json!(agent)))
}
