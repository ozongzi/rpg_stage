mod add_user;
mod auth;
mod chat;
mod delete_user;
mod health_check;
mod user_list;

use add_user::add_user;
use auth::auth;
use chat::chat;
use delete_user::delete_user;
use health_check::health_check;
use user_list::user_list;

use axum::Router;
use axum::routing::{delete, get, post};
use sqlx::PgPool;

use crate::app_state::AppState;
use crate::configuration::get_configuration;
pub async fn create_app() -> Router {
    let configuration = get_configuration().unwrap();

    let db = PgPool::connect(&configuration.database_url).await.unwrap();
    let admin_email = configuration.admin_email;
    let deepseek_token = configuration.deepseek_token;

    Router::new()
        .route("/api/health_check", get(health_check))
        .route("/api/auth", post(auth))
        .route("/api/user/add", post(add_user))
        .route("/api/user/list", get(user_list))
        .route("/api/user", delete(delete_user))
        .route("/api/chat", post(chat))
        .with_state(AppState {
            db,
            admin_email,
            deepseek_token,
        })
}
