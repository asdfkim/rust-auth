mod error;
mod models;
mod routes;
mod utils;

pub mod server {
    use crate::{database, routes};
    use axum::Router;
    use std::io;
    use std::sync::Arc;
    use tokio::net::TcpListener;

    pub async fn run(addr: &str) -> io::Result<()> {
        let db = database::Database::init().await.unwrap();

        let state = Arc::new(db);

        let app = Router::new()
            .nest("/test", routes::ping::router())
            .nest("/auth", routes::auth::router())
            .with_state(state);

        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

pub mod database {
    use crate::error::AppError;
    use crate::models::User;
    use anyhow::Result;
    use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
    use std::env;
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct Database {
        pub pool: SqlitePool,
    }

    impl Database {
        pub async fn init() -> Result<Self> {
            let url =
                env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./dev.db?mode=rwc".to_string());

            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await?;

            Self::create_tables(&pool).await?;

            Ok(Self { pool })
        }

        async fn create_tables(pool: &SqlitePool) -> Result<()> {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS users (
                    uuid TEXT PRIMARY KEY NOT NULL,
                    password TEXT NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
                );
                "#,
            )
            .execute(pool)
            .await?;

            Ok(())
        }

        pub async fn get_user(&self, uuid: &Uuid) -> Result<User, AppError> {
            let result = sqlx::query_as!(
                User,
                r#"
            SELECT uuid as "uuid: _", password, created_at as "created_at: _"
            FROM users
            WHERE uuid = ?
            "#,
                uuid
            )
            .fetch_optional(&self.pool)
            .await;

            match result {
                Ok(Some(user)) => Ok(user),
                Ok(None) => Err(AppError::NotFound),
                Err(e) => Err(AppError::DatabaseError(e)),
            }
        }

        pub async fn create_user(&self, user: &User) -> Result<User, AppError> {
            let result = sqlx::query_as!(
                User,
                r#"
                INSERT INTO users (uuid, password, created_at)
                VALUES (?, ?, ?)
                RETURNING uuid as "uuid: _", password, created_at as "created_at: _"
                "#,
                user.uuid,
                user.password,
                user.created_at
            )
            .fetch_one(&self.pool)
            .await;

            match result {
                Ok(row) => Ok(row),
                Err(e) => {
                    if let Some(db_err) = e.as_database_error() {
                        if db_err.code() == Some("2067".into()) {
                            // 2067 == 중복
                            return Err(AppError::AlreadyExists);
                        }
                    }
                    Err(AppError::DatabaseError(e))
                }
            }
        }
    }
}
