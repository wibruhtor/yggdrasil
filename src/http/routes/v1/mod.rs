use axum::{middleware, Router};

use crate::http::middleware::auth_middleware;

mod ban_word_filters;

pub fn routes() -> Router {
    Router::new()
        .nest("/ban-word-filters", ban_word_filters::routes())
        .layer(middleware::from_fn(auth_middleware))
}
