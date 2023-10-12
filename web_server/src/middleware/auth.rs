use std::sync::Arc;

use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Extension, TypedHeader};

use service::AuthService;
use types::error::AppResult;
use utils::jwt::{JwtMaker, TokenType};

pub async fn auth_middleware<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Extension(auth_service): Extension<Arc<AuthService>>,
    mut request: Request<B>,
    next: Next<B>,
) -> AppResult<Response> {
    let token = auth.token();
    let claims = auth_service.validate_token(token).await?;
    if claims.typ != TokenType::Access {
        return Err(JwtMaker::INVALID_TOKEN_ERROR);
    }
    request.extensions_mut().insert(Arc::new(claims));
    Ok(next.run(request).await)
}
