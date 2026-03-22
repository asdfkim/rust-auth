use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("NOT_FOUND")]
    NotFound,

    #[error("UNAUTHORIZED")]
    Unauthorized,

    #[error("INVALID_CREDENTIALS")]
    InvalidCredentials,

    #[error("ALREADY_EXISTS")]
    AlreadyExists,

    #[error("INTERNAL_ERROR")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::AlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({ "error_code": code }));
        (status, body).into_response()
    }
}
