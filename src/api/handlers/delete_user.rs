use crate::api::extractors::auth_user::AuthUser;
use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn delete_user(
    State(state): State<AppState>,
    AuthUser { user_id: _ }: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    state.services.user_service.delete_user_by_id(id).await?;
    Ok(())
}
