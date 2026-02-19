use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::State;
use serde_json::{Value, json};

pub async fn get_me(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> AppResult<Json<Value>> {
    Ok(Json(json!(
        state.services.user_service.get_user(user_id).await?
    )))
}
