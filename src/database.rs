use crate::error::AppError;
use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub type DbPool = SqlitePool;
pub type DbPoolOptions = SqlitePoolOptions;

pub async fn create_pool(url: &str) -> Result<DbPool, AppError> {
    DbPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .map_err(|_e| AppError::Internal)
}

pub async fn create_tables(pool: &DbPool) -> Result<(), AppError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
                uuid     TEXT PRIMARY KEY NOT NULL,
                password TEXT             NOT NULL
            )",
    )
    .execute(pool)
    .await
    .map_err(|_e| AppError::Internal)?;

    Ok(())
}
