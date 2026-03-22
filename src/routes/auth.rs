use crate::database::Database;
use crate::error::AppError;
use crate::models::{RegisterRequest, RegisterResponse, TokenRequest, TokenResponse, User};
use crate::utils;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<Database>> {
    Router::new()
        .route("/register", post(register))
        .route("/token", post(token))
}

async fn register(
    State(db): State<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Todo : 비밀번호 해싱

    let user = User {
        uuid: payload.uuid,
        password: payload.password,
        created_at: utils::now_ms(),
    };

    db.create_user(&user).await?;

    let res = RegisterResponse {
        uuid: user.uuid,
        created_at: user.created_at,
    };

    Ok((StatusCode::CREATED, Json(res)))
}

async fn token(
    State(db): State<Arc<Database>>,
    Json(payload): Json<TokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = db.get_user(&payload.uuid).await?;

    // Todo : 비밀번호 해싱
    let hashed = payload.password;

    if user.password != hashed {
        return Err(AppError::InvalidCredentials);
    }

    // Todo : JWT 생성

    let res = TokenResponse {
        uuid: user.uuid,
        token: "token".to_string(),
        created_at: utils::now_ms(),
        expires_in: utils::now_ms() + 1000 * 30, // 30 sec
    };

    Ok(Json(res))
}
