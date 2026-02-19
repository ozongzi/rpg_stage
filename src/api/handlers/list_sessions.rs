use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use crate::api::extractors::auth_user::AuthUserAdmin;
use crate::app_state::AppState;
use crate::errors::AppResult;

pub async fn list_sessions(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
) -> AppResult<Json<Value>> {
    let sessions = state.services.session_service.get_session_info_list().await?;
    Ok(Json(json!(sessions)))
}