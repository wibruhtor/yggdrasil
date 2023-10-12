use axum::middleware::from_fn;
use axum::{routing, Router};

use crate::middleware::auth_middleware;

mod all;
mod create;
mod delete;
mod one;
mod update;

pub fn routes() -> Router {
    Router::new()
        .route("/", routing::get(all::handler))
        .route("/", routing::post(create::handler))
        .route("/:chat_settings_id", routing::put(update::handler))
        .route("/:chat_settings_id", routing::delete(delete::handler))
        .layer(from_fn(auth_middleware))
        .route("/:chat_settings_id", routing::get(one::handler))
}
