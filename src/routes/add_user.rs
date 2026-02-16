use crate::{
    app_state::AppState,
    auth_user::AuthUserAdmin,
    types::{Email, UserName},
};
use axum::{
    Form, Json,
    extract::State,
    http::{Response, StatusCode},
};
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;

#[derive(Deserialize)]
pub struct AddUserRequest {
    name: String,
    email: String,
    password: String,
}

pub async fn add_user(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
    Form(request): Form<AddUserRequest>,
) -> Result<Json<Value>, Response<String>> {
    let name: UserName = request.name.parse().map_err(|e| {
        tracing::info!("Failed to parse name: {}", e);
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("无效的用户名".to_string())
            .unwrap()
    })?;

    let email: Email = request.email.parse().map_err(|e| {
        tracing::info!("Failed to parse email: {}", e);
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("无效的邮箱地址".to_string())
            .unwrap()
    })?;

    let password_hash = bcrypt::hash(request.password, bcrypt::DEFAULT_COST).map_err(|e| {
        tracing::error!("Failed to hash password: {}", e);
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("密码哈希算法失败".to_string())
            .unwrap()
    })?;

    let user_id = sqlx::query!(
        "insert into users (name, email, password_hash) values ($1, $2, $3) returning id",
        name.as_ref(),
        email.as_ref(),
        password_hash
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to add user: {}", e);
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("添加用户失败(重复邮箱)".to_string())
            .unwrap()
    })?;

    Ok(Json(json!({
        "user_id": user_id.id
    })))
}
