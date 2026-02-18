use crate::api::extractors::auth_user::AuthUserAdmin;
use crate::app_state::AppState;
use crate::domains::MetaAgent;
use crate::errors::AppResult;
use axum::extract::State;
use axum::{Form, Json};
use serde_json::{Value, json};

pub type MetadataForm = MetaAgent;
pub async fn create_agent_meta(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
    Form(form): Form<MetadataForm>,
) -> AppResult<Json<Value>> {
    let id = state.services.agent_service.new_agent_meta(&form).await?;

    Ok(Json(json!({ "agent_meta_id": id })))
}
