use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::extract::State;
use axum::{Form, Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMeForm {
    old_password: String,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

pub async fn update_me(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Form(form): Form<UpdateMeForm>,
) -> AppResult<Json<Value>> {
    Ok(Json(json!(
        state
            .services
            .user_service
            .update_user_self(
                user_id,
                form.old_password.parse()?,
                form.name.map(|x| x.parse()).transpose()?,
                form.email.map(|x| x.parse()).transpose()?,
                form.password.map(|x| x.parse()).transpose()?
            )
            .await?
    )))
}
