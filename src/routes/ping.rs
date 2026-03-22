use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new().route("/ping", get(pong))
}

async fn pong() -> &'static str {
    "pong"
}
