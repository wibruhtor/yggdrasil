use axum::{middleware, routing, Router};

use crate::http::middleware::auth_middleware;

pub fn routes() -> Router {
    Router::new()
        .route("/", routing::get(|| async {}))
        .route("/", routing::post(|| async {}))
        .route("/:chat_settings_id", routing::put(|| async {}))
        .route("/:chat_settings_id", routing::delete(|| async {}))
        .layer(middleware::from_fn(auth_middleware))
        .route("/:chat_settings_id", routing::get(|| async {}))
}
