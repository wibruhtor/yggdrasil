use axum::{middleware, routing, Router};

use crate::http::middleware::auth_middleware;

mod all;
mod delete;
mod delete_all_exclude_current;

pub fn routes() -> Router {
    Router::new()
        .route("/", routing::get(all::handler))
        .route("/", routing::delete(delete_all_exclude_current::handler))
        .route("/:session_id", routing::delete(delete::handler))
        .layer(middleware::from_fn(auth_middleware))
}
