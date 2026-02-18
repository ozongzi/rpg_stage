use crate::app_state::AppState;
use crate::errors::AppResult;
use axum::extract::State;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;

pub async fn logout(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> AppResult<()> {
    state
        .services
        .session_service
        .invalidate_session(bearer.token())
        .await
}
