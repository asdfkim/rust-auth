use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("ALREADY_EXISTS")]
    AlreadyExists,

    #[error("DATABASE_ERROR")]
    DatabaseError(#[from] sqlx::Error),

    #[error("INTERNAL_ERROR")]
    Anyhow(#[from] anyhow::Error),

    #[error("NOT_FOUND")]
    NotFound,

    #[error("INVALID_CREDENTIALS")]
    InvalidCredentials,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            AppError::AlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::DatabaseError(_) | AppError::Anyhow(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = Json(json!({ "code": code }));
        (status, body).into_response()
    }
}
