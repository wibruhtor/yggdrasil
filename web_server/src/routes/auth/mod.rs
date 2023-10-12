use axum::middleware::from_fn;
use axum::{routing, Router};

use crate::middleware::auth_middleware;

mod authorize;
mod exchange;
mod logout;
mod refresh;
mod sessions;

pub fn routes() -> Router {
    Router::new()
        .route("/authorize", routing::get(authorize::handler))
        .route("/exchange", routing::post(exchange::handler))
        .route("/refresh", routing::post(refresh::handler))
        .route(
            "/logout",
            routing::delete(logout::handler).layer(from_fn(auth_middleware)),
        )
        .nest("/sessions", sessions::routes())
}
