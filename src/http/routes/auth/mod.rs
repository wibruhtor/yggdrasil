use std::sync::Arc;

use axum::{middleware, routing, Extension, Router};
use reqwest::StatusCode;

use crate::{
    error::AppResult, http::middleware::auth_middleware, jwt::Claims, service::AuthService,
};

mod authorize;
mod exchange;

pub fn routes() -> Router {
    Router::new()
        .route("/authorize", routing::get(authorize::handler))
        .route("/exchange", routing::post(exchange::handler))
        .route(
            "/logout",
            routing::delete(handler).layer(middleware::from_fn(auth_middleware)),
        )
}

async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<StatusCode> {
    auth_service.delete_token(&claims.jti).await?;

    Ok(StatusCode::NO_CONTENT)
}
