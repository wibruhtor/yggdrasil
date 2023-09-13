use std::time::Duration;

use axum::{extract::MatchedPath, http::Request, Router};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Span;

use crate::http::routes;

pub fn new() -> Router {
    let middleware = tower::ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                )
            })
            .on_response(
                DefaultOnResponse::new()
                    .latency_unit(tower_http::LatencyUnit::Millis)
                    .level(tracing::Level::INFO),
            )
            .on_failure(|_: ServerErrorsFailureClass, _: Duration, _: &Span| {}),
    );

    Router::new().merge(routes::get()).layer(middleware)
}
