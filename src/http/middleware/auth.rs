use std::sync::Arc;

use axum::{
    extract::TypedHeader,
    headers::authorization::{Authorization, Bearer},
    http::Request,
    middleware::Next,
    response::Response,
    Extension,
};

use crate::{error::AppResult, service::AuthService};

pub async fn auth_middleware<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Extension(auth_service): Extension<Arc<AuthService>>,
    mut request: Request<B>,
    next: Next<B>,
) -> AppResult<Response> {
    let token = auth.token();
    let claims = auth_service.validate_token(token).await?;
    request.extensions_mut().insert(Arc::new(claims));
    Ok(next.run(request).await)
}
