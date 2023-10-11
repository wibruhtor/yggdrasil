use axum::http::StatusCode;
use axum::Router;

use types::error::AppError;

mod auth;

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .fallback(handler_404)
}

async fn handler_404() -> AppError {
    AppError::new(StatusCode::NOT_FOUND).message("not found")
}
