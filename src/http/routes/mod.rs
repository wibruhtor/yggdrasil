use axum::Router;

mod index;

pub fn get() -> Router {
    Router::new().merge(index::routes())
}
