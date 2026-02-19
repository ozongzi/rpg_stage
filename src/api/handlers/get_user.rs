use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::Json;
use axum::extract::{Path, State};
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn get_user(
    State(state): State<AppState>,
    AuthUser { user_id: _ }: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    Ok(Json(json!(state.services.user_service.get_user(id).await?)))
}
