pub type AppResult<T> = Result<T, AppError>;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct AppError(pub StatusCode, pub Value);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, Json(self.1)).into_response()
    }
}

use serde_json::Value;
use sqlx::Error as SqlxError;

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        tracing::info!("{:?}", err);
        match err {
            SqlxError::Database(db_err) => {
                if db_err.constraint().is_some() {
                    return AppError(StatusCode::CONFLICT, "数据已存在".into());
                }
                AppError(StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".into())
            }
            SqlxError::RowNotFound => AppError(StatusCode::BAD_REQUEST, "数据不存在".into()),
            SqlxError::ColumnNotFound(_) => AppError(StatusCode::BAD_REQUEST, "数据不存在".into()),
            _ => AppError(StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".into()),
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        tracing::info!("{:?}", err);
        AppError(StatusCode::INTERNAL_SERVER_ERROR, "密码学错误".into())
    }
}