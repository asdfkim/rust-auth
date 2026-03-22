use crate::model::AppState;
use crate::{database, routes};
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;

pub async fn run(addr: &str, state: AppState) {
    database::create_tables(&state.pool)
        .await
        .expect("failed to create tables");

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let app = Router::new()
        .nest("/auth", routes::auth::router())
        .layer(GovernorLayer::new(governor_conf))
        .with_state(state);

    let listener = TcpListener::bind(addr).await.expect("failed to bind port");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("failed to serve");
}
