use crate::error::AppError;
use crate::model::{
    AppState, RegisterRequest, RegisterResponse, TokenRequest, TokenResponse, User,
};
use crate::utils;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/token", post(token))
}

async fn register(
    State(AppState { pool, .. }): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let exists = sqlx::query("SELECT 1 FROM users WHERE uuid = ?")
        .bind(&payload.uuid)
        .fetch_optional(&pool)
        .await
        .map_err(|_| AppError::Internal)?
        .is_some();

    if exists {
        return Err(AppError::AlreadyExists);
    }

    // --- //

    let password = payload.password;
    let hashed = tokio::task::spawn_blocking(move || utils::hash::hash_password(&password))
        .await
        .map_err(|_| AppError::Internal)??;

    let user = User {
        uuid: payload.uuid,
        password: hashed,
    };

    // --- //

    sqlx::query("INSERT INTO users (uuid, password) VALUES (?, ?)")
        .bind(user.uuid)
        .bind(&user.password)
        .execute(&pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::AlreadyExists
            }
            _ => AppError::Internal,
        })?;

    // --- //

    let res = RegisterResponse { uuid: user.uuid };

    Ok((StatusCode::CREATED, Json(res)))
}

async fn token(
    State(AppState { pool, config }): State<AppState>,
    Json(payload): Json<TokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE uuid = ?")
        .bind(&payload.uuid)
        .fetch_optional(&pool)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::InvalidCredentials)?;

    // --- //

    let password = payload.password;
    let hash = user.password;

    let is_valid = tokio::task::spawn_blocking(move || {
        utils::hash::verify_password(&password, &hash)
    })
    .await
    .map_err(|_| AppError::Internal)??;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // --- //

    let created_at = utils::time::now_unix();
    let expires_at = created_at + 30;
    let token = utils::jwt::generate(&user.uuid, expires_at, &config.jwt_secret)?;

    let res = TokenResponse {
        uuid: user.uuid,
        token,
        created_at,
        expires_at,
    };

    Ok((StatusCode::OK, Json(res)))
}
