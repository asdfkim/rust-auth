use crate::database::Database;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn router() -> Router<Arc<Database>> {
    Router::new().route("/ping", get(pong))
}

async fn pong() -> &'static str {
    "pong"
}
