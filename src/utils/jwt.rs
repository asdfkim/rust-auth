use crate::error::AppError;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Todo : 이건 진짜 하드코딩 하면 안됨.
const SECRET: &str = "asdfkim";

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub uuid: String,
    pub exp: i64,
}

pub fn generate(uuid: &Uuid, expires_at: i64) -> Result<String, AppError> {
    let claims = Claims {
        uuid: uuid.to_string(),
        exp: expires_at,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .map_err(|_| AppError::Internal)
}

pub fn verify(token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}
