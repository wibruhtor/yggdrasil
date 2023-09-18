use std::{net::SocketAddr, time::Instant};

use axum::{
    extract::{ConnectInfo, MatchedPath},
    headers::UserAgent,
    http::Request,
    middleware::Next,
    response::Response,
    Extension, TypedHeader,
};

use crate::error::AppResult;

use super::request_id::REQUEST_ID_HEADER_NAME;

pub async fn logger_middleware<B>(
    Extension(path): Extension<MatchedPath>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(socket): ConnectInfo<SocketAddr>,
    request: Request<B>,
    next: Next<B>,
) -> AppResult<Response> {
    let now = Instant::now();

    let user_agent = user_agent.as_str();
    let path = path.as_str();

    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER_NAME)
        .map(|h| h.to_str());
    let request_id = match request_id {
        Some(v) => v.unwrap_or_default(),
        None => "",
    };

    let method = request.method().as_str();

    let span = tracing::info_span!(
        "endpoint",
        request_id,
        method,
        path,
        ip = socket.ip().to_string()
    );
    let _span = span.enter();

    let response = next.run(request).await;

    let latency = now.elapsed().as_micros();
    let status_code = response.status().as_u16();

    if status_code >= 400 && status_code < 500 {
        tracing::warn!(status_code, latency, user_agent, "log");
    } else if status_code >= 500 && status_code < 600 {
        tracing::error!(status_code, latency, user_agent, "log");
    } else {
        tracing::info!(status_code, latency, user_agent, "log");
    }

    Ok(response)
}
