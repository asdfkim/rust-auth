mod error;
mod model;
mod routes;
mod utils;

pub mod server {
    use crate::{database, routes};
    use axum::Router;
    use std::io;
    use tokio::net::TcpListener;

    pub async fn run(addr: &str) -> io::Result<()> {
        // Todo : 데이터베이스 주소 하드코딩하면 안됨.
        let pool = database::create_pool("sqlite://dev.db?mode=rwc")
            .await
            .expect("failed to connect database");

        database::create_tables(&pool)
            .await
            .expect("failed to create tables");

        let app = Router::new()
            .nest("/auth", routes::auth::router())
            .with_state(pool);

        let listener = TcpListener::bind(addr).await.expect("failed to bind port");

        axum::serve(listener, app).await?;

        Ok(())
    }
}

pub mod database {
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
}
