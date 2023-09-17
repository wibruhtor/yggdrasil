use axum::{routing, Router};

mod exchange;
mod login;

pub fn routes() -> Router {
    Router::new()
        .route("/login", routing::get(login::handler))
        .route("/exchange", routing::post(exchange::handler))
}
