use crate::api::extractors::auth_user::AuthUserAdmin;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn force_logout(
    State(state): State<AppState>,
    AuthUserAdmin { user_id: _ }: AuthUserAdmin,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    state
        .services
        .session_service
        .delete_session_by_id(id)
        .await
}
