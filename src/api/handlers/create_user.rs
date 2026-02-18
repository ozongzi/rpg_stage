use crate::{
    api::extractors::auth_user::AuthUserAdmin,
    app_state::AppState,
    errors::{AppError, AppResult},
    services::user_service::CreateUserInput,
};
use axum::{Form, Json, extract::State};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct AddUserRequest {
    name: String,
    email: String,
    password: String,
}

impl TryFrom<AddUserRequest> for CreateUserInput {
    type Error = AppError;
    fn try_from(value: AddUserRequest) -> Result<Self, Self::Error> {
        let name = value.name.parse()?;
        let email = value.email.parse()?;
        let password = value.password.parse()?;
        Ok(Self {
            name,
            email,
            password,
        })
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
    Form(request): Form<AddUserRequest>,
) -> AppResult<Json<Value>> {
    let user_id = state
        .services
        .user_service
        .create_user(request.try_into()?)
        .await?;

    Ok(Json(json!({ "user_id": user_id })))
}
