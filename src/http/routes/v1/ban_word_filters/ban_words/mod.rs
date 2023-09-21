use axum::{middleware, routing, Router};

use crate::http::middleware::auth_middleware;

mod get;
mod update;

pub fn routes() -> Router {
    Router::new().route("/", routing::get(get::handler)).route(
        "/",
        routing::put(update::handler).layer(middleware::from_fn(auth_middleware)),
    )
}
