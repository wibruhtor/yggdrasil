use axum::{routing, Router};

mod get;
mod update;

pub fn routes() -> Router {
    Router::new()
        .route("/ban-words", routing::get(get::handler))
        .route("/ban-words", routing::put(update::handler))
}
