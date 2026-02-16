use crate::app_state::AppState;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use sha2::Digest;

use uuid::Uuid;

pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    tracing::info!("Failed to extract authorization header");
                    StatusCode::UNAUTHORIZED
                })?;
        let token = bearer.token();
        let token = Uuid::parse_str(token).map_err(|_| {
            tracing::info!("Failed to parse token as UUID");
            StatusCode::UNAUTHORIZED
        })?;

        let token_hash = hex::encode(sha2::Sha256::digest(token.as_bytes()));

        let record = sqlx::query!(
            "SELECT user_id FROM sessions
                WHERE token_hash = $1 AND expires_at > now()",
            token_hash
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let record = record.ok_or({
            tracing::info!("Failed to find session");
            StatusCode::UNAUTHORIZED
        })?;

        Ok(AuthUser {
            user_id: record.user_id,
        })
    }
}

pub struct AuthUserAdmin {
    #[allow(unused)]
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUserAdmin {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let AuthUser { user_id } = AuthUser::from_request_parts(parts, state).await?;
        let user = sqlx::query!("select email from users where id = $1", user_id)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        if user.email != state.admin_email {
            return Err(StatusCode::FORBIDDEN);
        }

        Ok(AuthUserAdmin { user_id })
    }
}
