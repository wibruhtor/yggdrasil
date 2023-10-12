use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::extract::MatchedPath;
use axum::http;
use axum_tracing_opentelemetry::middleware::Filter;
use http::{Request, Response};
use pin_project::pin_project;
use tower::Layer;
use tower_service::Service;
use tracing::field::Empty;
use tracing::Span;
use tracing_opentelemetry_instrumentation_sdk::http::{
    http_flavor, http_host, http_method, url_scheme, user_agent,
};
use tracing_opentelemetry_instrumentation_sdk::{http as otel_http, TRACING_TARGET};

#[derive(Debug, Clone, Default)]
pub struct TracingLayer {
    filter: Option<Filter>,
}

#[allow(dead_code)]
impl TracingLayer {
    #[must_use]
    pub fn filter(self, filter: Filter) -> Self {
        TracingLayer {
            filter: Some(filter),
        }
    }
}

impl<S> Layer<S> for TracingLayer {
    /// The wrapped service
    type Service = TracingService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        TracingService {
            inner,
            filter: self.filter,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TracingService<S> {
    inner: S,
    filter: Option<Filter>,
}

impl<S, B, B2> Service<Request<B>> for TracingService<S>
where
    S: Service<Request<B>, Response = Response<B2>> + Clone + Send + 'static,
    S::Error: Error + 'static, //fmt::Display + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        use tracing_opentelemetry::OpenTelemetrySpanExt;
        let req = req;
        let span = if self.filter.map_or(true, |f| f(req.uri().path())) {
            let span = make_span_from_request(&req);
            let route = http_route(&req);
            let method = http_method(req.method());
            let client_ip = client_ip(&req);
            span.record("http.route", route);
            span.record("otel.name", format!("{method} {route}").trim());
            span.record("client.address", client_ip);
            span.set_parent(otel_http::extract_context(req.headers()));
            span
        } else {
            Span::none()
        };
        let future = {
            let _ = span.enter();
            self.inner.call(req)
        };

        ResponseFuture {
            inner: future,
            span,
        }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    inner: F,
    span: Span,
}

impl<Fut, ResBody, E> Future for ResponseFuture<Fut>
where
    Fut: Future<Output = Result<Response<ResBody>, E>>,
    E: Error + 'static,
{
    type Output = Result<Response<ResBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _guard = this.span.enter();
        let result = futures_util::ready!(this.inner.poll(cx));
        otel_http::http_server::update_span_from_response_or_error(this.span, &result);
        Poll::Ready(result)
    }
}

#[inline]
fn http_route<B>(req: &Request<B>) -> &str {
    req.extensions()
        .get::<MatchedPath>()
        .map_or_else(|| "", |mp| mp.as_str())
}

#[inline]
fn client_ip<B>(req: &Request<B>) -> String {
    let header = req.headers().get("x-forwarded-for");
    if header.is_none() {
        return "".to_string();
    }
    let value = header.unwrap().to_str();
    if value.is_err() {
        return "".to_string();
    }
    let ips: Vec<&str> = value.unwrap().split(", ").collect();

    ips.first().unwrap_or(&"").to_string()
}

pub fn make_span_from_request<B>(req: &Request<B>) -> Span {
    let http_method = http_method(req.method());
    tracing::trace_span!(
        target: TRACING_TARGET,
        "HTTP request",
        http.request.method = %http_method,
        http.route = Empty, // to set by router of "webframework" after
        network.protocol.version = %http_flavor(req.version()),
        server.address = http_host(req),
        // server.port = req.uri().port(),
        http.client.address = Empty, //%$request.connection_info().realip_remote_addr().unwrap_or(""),
        user_agent.original = user_agent(req),
        http.response.status_code = Empty, // to set on response
        url.path = req.uri().path(),
        url.query = req.uri().query(),
        url.scheme = url_scheme(req.uri()),
        otel.name = %http_method, // to set by router of "webframework" after
        otel.kind = ?opentelemetry_api::trace::SpanKind::Server,
        otel.status_code = Empty, // to set on response
        trace_id = Empty, // to set on response
        request_id = Empty, // to set
        exception.message = Empty, // to set on response
        client.address = Empty, // to set
        "span.type" = "web", // non-official open-telemetry key, only supported by Datadog
    )
}
