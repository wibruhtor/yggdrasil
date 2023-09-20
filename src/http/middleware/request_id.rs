use axum::{http::Request, middleware::Next, response::Response};
use serde::{Deserialize, Serialize};
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
    request.extensions_mut().insert(RequestId(request_id));

    let mut response = next.run(request).await;

    response.headers_mut().insert(
        REQUEST_ID_HEADER_NAME,
        request_id.to_string().parse().unwrap(),
    );

    Ok(response)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestId(pub Uuid);
