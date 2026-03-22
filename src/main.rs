use auth_server::config::Config;
use auth_server::model::AppState;
use auth_server::{database, server};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::from_env());
    let pool = database::create_pool(&config.database_url)
        .await
        .expect("failed to connect database");

    let addr = config.server_addr.clone();
    let state = AppState { pool, config };

    server::run(&addr, state).await;
}
