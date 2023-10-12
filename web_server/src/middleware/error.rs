use axum::body::HttpBody;
use axum::http::header::CONTENT_TYPE;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

use types::error::{AppError, AppResult};

pub async fn error_middleware<B>(request: Request<B>, next: Next<B>) -> AppResult<Response> {
    let mut response = next.run(request).await;
    let content_type_header = response.headers().get(CONTENT_TYPE);
    if content_type_header.is_none() {
        return Ok(response);
    }
    let content_type = content_type_header.unwrap().to_str();

    if content_type.is_err() {
        return Ok(response);
    }
    let content_type = content_type.unwrap();

    if !content_type.contains("text/plain") {
        return Ok(response);
    }

    let body = response.body_mut().data().await;
    if body.is_none() {
        return Ok(response);
    }
    let body = body.unwrap();

    if body.is_err() {
        return Ok(response);
    }
    let body = body.unwrap().to_vec();

    let data = String::from_utf8(body);
    if data.is_err() {
        return Ok(response);
    }
    let data = data.unwrap();

    Err(AppError::new(response.status()).message(&data))
}
