use std::{net::SocketAddr, time::Instant};

use axum::{
    extract::{ConnectInfo, MatchedPath},
    headers::UserAgent,
    http::Request,
    middleware::Next,
    response::Response,
    TypedHeader,
};

use crate::error::AppResult;

use super::request_id::REQUEST_ID_HEADER_NAME;

pub async fn logger_middleware<B>(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(socket): ConnectInfo<SocketAddr>,
    request: Request<B>,
    next: Next<B>,
) -> AppResult<Response> {
    let now = Instant::now();

    let user_agent = user_agent.as_str();
    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str);
    let path = match matched_path {
        Some(path) => path,
        None => request.uri().path(),
    };

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
        user_agent,
        ip = socket.ip().to_string()
    );
    let _span = span.enter();

    let response = next.run(request).await;

    let latency = now.elapsed().as_micros();
    let status_code = response.status().as_u16();

    if (400..500).contains(&status_code) {
        tracing::warn!(status_code, latency, "log");
    } else if (500..600).contains(&status_code) {
        tracing::error!(status_code, latency, "log");
    } else {
        tracing::info!(status_code, latency, "log");
    }

    Ok(response)
}
