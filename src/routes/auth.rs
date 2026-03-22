use crate::models::{RegisterRequest, RegisterResponse, TokenRequest, TokenResponse};
use crate::utils;
use axum::routing::{get, post};
use axum::{Json, Router};

pub fn router() -> Router {
    Router::new()
        .route("/me", get(me))
        .route("/register", post(register))
        .route("/token", post(token))
}

async fn me() -> &'static str {
    "me"
}

async fn register(Json(payload): Json<RegisterRequest>) -> Json<RegisterResponse> {
    let res = RegisterResponse {
        uuid: payload.uuid,
        created_at: utils::now_ms(),
    };

    Json(res)
}

async fn token(Json(payload): Json<TokenRequest>) -> Json<TokenResponse> {
    let res = TokenResponse {
        uuid: payload.uuid,
        token: "token".to_string(),
        created_at: utils::now_ms(),
        expires_in: utils::now_ms(),
    };

    Json(res)
}
