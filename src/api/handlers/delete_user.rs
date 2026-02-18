use crate::{api::extractors::auth_user::AuthUserAdmin, app_state::AppState};
use axum::{
    Form,
    extract::State,
    http::{Response, StatusCode},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    email: Option<String>,
    id: Option<Uuid>,
}

pub async fn delete_user(
    State(state): State<AppState>,
    AuthUserAdmin { user_id }: AuthUserAdmin,
    Form(request): Form<DeleteUserRequest>,
) -> Result<String, Response<String>> {
    todo!()
    // if let Some(id) = request.id {
    //     // Delete user by id
    //     if id == user_id {
    //         return Err(Response::builder()
    //             .status(StatusCode::BAD_REQUEST)
    //             .body("Cannot delete yourself".to_string())
    //             .unwrap());
    //     }

    //     sqlx::query!("DELETE FROM users WHERE id = $1", id)
    //         .execute(&state.db)
    //         .await
    //         .map_err(|e| {
    //             Response::builder()
    //                 .status(StatusCode::BAD_REQUEST)
    //                 .body(e.to_string())
    //                 .unwrap()
    //         })?;
    // } else if let Some(email) = request.email {
    //     // Delete user by email
    //     if email == state.admin_email {
    //         return Err(Response::builder()
    //             .status(StatusCode::BAD_REQUEST)
    //             .body("Cannot delete admin".to_string())
    //             .unwrap());
    //     }

    //     sqlx::query!("DELETE FROM users WHERE email = $1", email)
    //         .execute(&state.db)
    //         .await
    //         .map_err(|e| {
    //             Response::builder()
    //                 .status(StatusCode::BAD_REQUEST)
    //                 .body(e.to_string())
    //                 .unwrap()
    //         })?;
    // } else {
    //     return Err(Response::builder()
    //         .status(StatusCode::BAD_REQUEST)
    //         .body("Missing email or ID".to_string())
    //         .unwrap());
    // }

    // Ok("deleted".to_string())
}
