use std::time::Duration;

use axum::{
    Form,
    extract::State,
    http::{Response, StatusCode},
};
use chrono::Utc;
use serde::Deserialize;
use sha2::Digest;
use sqlx::query;
use uuid::Uuid;

use crate::{app_state::AppState, types::Email};

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn auth(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<Response<String>, Response<String>> {
    let email: Email = form.email.parse().map_err(|x| {
        Response::builder()
            .status(StatusCode::UNPROCESSABLE_ENTITY)
            .body(x)
            .unwrap()
    })?;

    let user_password_hash = query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        email.as_ref()
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("用户邮箱或密码错误".to_string())
            .unwrap()
    })?;

    if user_password_hash.password_hash.is_none() {
        return Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("用户无密码，请用其他方式登陆".to_string())
            .unwrap());
    }

    let matched = bcrypt::verify(&form.password, &user_password_hash.password_hash.unwrap())
        .map_err(|x| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(x.to_string())
                .unwrap()
        })?;

    if !matched {
        return Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("用户邮箱或密码错误".to_string())
            .unwrap());
    }

    let token = Uuid::new_v4();
    // let token_hash = bcrypt::hash(token.to_string(), DEFAULT_COST).map_err(|x| {
    //     Response::builder()
    //         .status(StatusCode::INTERNAL_SERVER_ERROR)
    //         .body(x.to_string())
    //         .unwrap()
    // })?;
    //
    let token_hash = hex::encode(sha2::Sha256::digest(token));
    let user_id = user_password_hash.id;
    let expires_at = Utc::now() + Duration::from_hours(30 * 24);

    query!(
        "INSERT INTO sessions (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user_id,
        token_hash,
        expires_at
    )
    .execute(&state.db)
    .await
    .map_err(|x| {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(x.to_string())
            .unwrap()
    })?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(token.to_string())
        .unwrap())
}
