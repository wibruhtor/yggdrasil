use axum::{routing::get, Router};

pub fn new() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
