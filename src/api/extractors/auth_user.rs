use crate::{app_state::AppState, errors::AppError};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use uuid::Uuid;

pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    tracing::debug!("Failed to extract authorization header");
                    AppError(StatusCode::UNAUTHORIZED, "请提供token".into())
                })?;

        let user_id = state
            .services
            .session_service
            .authenticate(bearer.token())
            .await?;

        Ok(AuthUser { user_id })
    }
}

pub struct AuthUserAdmin {
    #[allow(unused)]
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUserAdmin {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let AuthUser { user_id } = AuthUser::from_request_parts(parts, state).await?;
        state.services.user_service.assert_admin(user_id).await?;
        Ok(AuthUserAdmin { user_id })
    }
}
