use std::sync::Arc;

use axum::{Extension, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, AppResult},
    jwt::{Claims, TokenType},
    service::AuthService,
};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(request): Json<RefreshRequest>,
) -> AppResult<Json<RefreshResponse>> {
    let refresh_token_claims = auth_service.validate_token(&request.token).await?;
    let token_type = refresh_token_claims.token_type();
    if token_type.is_none() || token_type.unwrap() != TokenType::Refresh {
        return Err(
            AppError::new(StatusCode::BAD_REQUEST).message("invalid refresh token".to_string())
        );
    }

    let (access_token, refresh_token) = auth_service.refresh_token(&claims).await?;

    Ok(Json(RefreshResponse {
        access_token,
        refresh_token,
    }))
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    access_token: String,
    refresh_token: String,
}
