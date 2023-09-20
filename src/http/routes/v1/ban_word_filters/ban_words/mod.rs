use axum::{routing, Router};

mod get;
mod update;

pub fn routes() -> Router {
    Router::new()
        .route("/", routing::get(get::handler))
        .route("/", routing::put(update::handler))
}
