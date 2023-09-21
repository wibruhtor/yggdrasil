use axum::{middleware, routing, Router};

use crate::http::middleware::auth_middleware;

mod all;
mod ban_words;
mod create;
mod delete;
mod one;
mod update;

pub fn routes() -> Router {
    Router::new()
        .route("/", routing::get(all::handler))
        .route("/", routing::post(create::handler))
        .route("/:ban_word_filter_id", routing::get(one::handler))
        .route("/:ban_word_filter_id", routing::put(update::handler))
        .route("/:ban_word_filter_id", routing::delete(delete::handler))
        .layer(middleware::from_fn(auth_middleware))
        .nest("/:ban_word_filter_id/ban-words", ban_words::routes())
}
