use axum::{routing, Router};

mod authorize;

pub fn routes() -> Router {
    Router::new().route("/authorize", routing::get(authorize::handler))
}
