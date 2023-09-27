use std::sync::Arc;

use axum::{Extension, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::{AppError, AppResult, ValidationErrorsWrapper},
    jwt::TokenType,
    service::AuthService,
};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Json(request): Json<RefreshRequest>,
) -> AppResult<Json<RefreshResponse>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let refresh_token_claims = auth_service.validate_token(&request.token).await?;
    if refresh_token_claims.typ != TokenType::Refresh {
        return Err(
            AppError::new(StatusCode::BAD_REQUEST).message("invalid refresh token".to_string())
        );
    }

    let (access_token, refresh_token) = auth_service.refresh_token(&refresh_token_claims).await?;

    Ok(Json(RefreshResponse {
        access_token,
        refresh_token,
    }))
}

#[derive(Deserialize, Validate)]
pub struct RefreshRequest {
    #[validate(length(min = 1))]
    token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
}
