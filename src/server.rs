use crate::model::AppState;
use crate::{database, routes};
use axum::Router;
use tokio::net::TcpListener;

pub async fn run(addr: &str, state: AppState) {
    database::create_tables(&state.pool)
        .await
        .expect("failed to create tables");

    let app = Router::new()
        .nest("/auth", routes::auth::router())
        .with_state(state);

    let listener = TcpListener::bind(addr).await.expect("failed to bind port");

    axum::serve(listener, app).await.expect("failed to serve");
}
