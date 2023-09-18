use std::sync::Arc;

use axum::Extension;
use reqwest::StatusCode;

use crate::{
    error::{AppError, AppResult},
    jwt::Claims,
    service::AuthService,
};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<StatusCode> {
    let token_type = claims
        .token_type()
        .ok_or(AppError::new(StatusCode::FORBIDDEN).message("invalid token".to_string()))?;
    auth_service.delete_token(&claims.jti, token_type).await?;

    Ok(StatusCode::NO_CONTENT)
}
