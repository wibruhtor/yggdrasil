use axum::http::StatusCode;
use axum::Router;

use types::error::AppError;

mod auth;
mod v1;

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/v1", v1::routes())
        .fallback(handler_404)
}

async fn handler_404() -> AppError {
    AppError::new(StatusCode::NOT_FOUND).message("not found")
}
