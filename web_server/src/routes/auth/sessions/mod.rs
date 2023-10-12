use axum::middleware::from_fn;
use axum::{routing, Router};

use crate::middleware::auth_middleware;

mod all;
mod delete;
mod delete_all_exclude_current;

pub fn routes() -> Router {
    Router::new()
        .route("/", routing::get(all::handler))
        .route("/", routing::delete(delete_all_exclude_current::handler))
        .route("/:session_id", routing::delete(delete::handler))
        .layer(from_fn(auth_middleware))
}
