use super::handlers::*;

use crate::app_state::AppState;
use crate::configuration::get_configuration;
use crate::services::Services;
use axum::Router;
use axum::routing::{delete, get, patch, post};
use sqlx::PgPool;
// use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

pub async fn create_app() -> Router {
    let configuration = get_configuration().unwrap();

    let db = PgPool::connect(&configuration.database_url).await.unwrap();

    let services = Services::install(&db, configuration.deepseek_token.clone());
    let app_state = AppState { services };

    Router::new()
        // 健康检查
        .route("/health", get(health_check))
        // ========== Auth ==========
        .route("/auth/session", post(login)) // 登录
        .route("/auth/session", delete(logout)) // 当前用户登出
        // ========== Users ==========
        .route("/users", post(create_user)) // 注册
        .route("/users", get(list_users)) // 管理员列出用户
        .route("/users/me", get(get_me)) // 当前用户信息
        .route("/users/me", patch(update_me)) // 修改自己
        .route("/users/{id}", get(get_user)) // 管理员查看用户
        .route("/users/{id}", patch(update_user)) // 管理员修改
        .route("/users/{id}", delete(delete_user)) // 管理员删除
        // =========== Metadata ============
        .route("/agent_metas", post(create_agent_meta)) // 管理员添加
        .route("/agent_metas", get(list_agent_meta)) // 普通用户权限列出
        // ========== Agents ================
        .route("/agents", post(create_agent))
        .route("/agents", get(list_agents))
        .route("/agents/{id}", get(get_agent))
        .route("/agents/{id}", delete(delete_agent))
        // ========== Conversations ==========
        .route(
            "/agents/{agent_id}/conversations",
            post(create_conversation),
        )
        .route("/agents/{agent_id}/conversations", get(list_conversations))
        .route(
            "/agents/{agent_id}/conversations/{id}",
            get(get_conversation),
        )
        .route(
            "/agents/{agent_id}/conversations/{id}",
            delete(delete_conversation),
        )
        // ========== Messages ==========
        .route("/conversations/{id}/messages", post(create_message))
        .route("/conversations/{id}/messages", get(list_messages))
        // ========== Admin ==========
        .route("/admin/sessions", get(list_sessions))
        .route("/admin/sessions/{id}", delete(force_logout))
        .fallback_service(
            ServeDir::new("client/dist")
                .not_found_service(ServeFile::new("client/dist/index.html")),
        )
        // .layer(
        //     CorsLayer::new()
        //         .allow_origin(Any)
        //         .allow_methods(Any)
        //         .allow_headers(Any),
        // )
        .with_state(app_state)
}
