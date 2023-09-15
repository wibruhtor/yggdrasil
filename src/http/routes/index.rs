use axum::{routing::get, Router};

use crate::error::AppResult;

pub fn routes() -> Router {
    Router::new().route("/", get(index_route))
}

async fn index_route() -> AppResult<&'static str> {
    Ok("Hello, world!")
}
