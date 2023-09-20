use axum::Router;

mod ban_word_filters;

pub fn routes() -> Router {
    Router::new().nest("/ban-word-filters", ban_word_filters::routes())
}
