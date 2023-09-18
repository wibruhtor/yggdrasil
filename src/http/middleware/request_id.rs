use axum::{http::Request, middleware::Next, response::Response};
use uuid::Uuid;

use crate::error::AppResult;

pub const REQUEST_ID_HEADER_NAME: &str = "request-id";

pub async fn request_id_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> AppResult<Response> {
    let request_id = Uuid::new_v4();

    request.headers_mut().insert(
        REQUEST_ID_HEADER_NAME,
        request_id.to_string().parse().unwrap(),
    );

    let mut response = next.run(request).await;

    response.headers_mut().insert(
        REQUEST_ID_HEADER_NAME,
        request_id.to_string().parse().unwrap(),
    );

    Ok(response)
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestId(pub Uuid);
