use axum::routing::get;
use axum::{response::IntoResponse, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(index_route))
}

async fn index_route() -> impl IntoResponse {
    "Hello, World!"
}
