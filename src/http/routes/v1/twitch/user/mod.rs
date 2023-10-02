use axum::{routing, Router};

mod user_info;

pub fn routes() -> Router {
    Router::new().route("/info", routing::get(user_info::handler))
}
