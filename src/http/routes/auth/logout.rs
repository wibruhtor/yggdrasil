use std::sync::Arc;

use axum::{http::StatusCode, Extension};

use crate::{error::AppResult, jwt::Claims, service::AuthService};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<StatusCode> {
    auth_service.revoke_token(&claims.jti).await?;

    Ok(StatusCode::NO_CONTENT)
}
