use crate::api::extractors::auth_user::AuthUserAdmin;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::extract::{Path, State};
use axum::{Form, Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserForm {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

pub async fn update_user(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
    Path(id): Path<Uuid>,
    Form(form): Form<UpdateUserForm>,
) -> AppResult<Json<Value>> {
    Ok(Json(json!(
        state
            .services
            .user_service
            .update_user(
                id,
                form.name.map(|x| x.parse()).transpose()?,
                form.email.map(|x| x.parse()).transpose()?,
                form.password.map(|x| x.parse()).transpose()?
            )
            .await?
    )))
}
