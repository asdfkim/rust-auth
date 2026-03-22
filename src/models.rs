use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::{FromRow, SqlitePool};

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub uuid: Uuid,
    pub password: String,
    pub created_at: i64,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub uuid: Uuid,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub uuid: Uuid,
    pub created_at: i64,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub uuid: Uuid,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub uuid: Uuid,
    pub token: String,
    pub created_at: i64,
    pub expires_in: i64,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_code: String,
}
