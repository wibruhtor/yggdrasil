use axum::{routing, Router};

mod channel;
mod global;

pub fn routes() -> Router {
    Router::new()
        .route("/global", routing::get(global::handler))
        .route("/:channel_id", routing::get(channel::handler))
}
