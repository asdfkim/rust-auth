use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub uuid: Uuid,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub uuid: Uuid,
    pub created_at: u64,
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
    pub created_at: u64,
    pub expires_in: u64,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_code: String,
}
