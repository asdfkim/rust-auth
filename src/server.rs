use crate::routes;
use axum::Router;
use std::io;
use tokio::net::TcpListener;

pub async fn run(addr: &str) -> io::Result<()> {
    let app = Router::new()
        .nest("/test", routes::ping::router())
        .nest("/auth", routes::auth::router());

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
