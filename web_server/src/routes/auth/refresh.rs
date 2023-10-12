use std::sync::Arc;

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use service::AuthService;
use types::error::{AppResult, ValidationErrorsWrapper};
use utils::jwt::{JwtMaker, TokenType};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Json(request): Json<RefreshRequest>,
) -> AppResult<Json<RefreshResponse>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let refresh_token_claims = auth_service.validate_token(&request.token).await?;
    if refresh_token_claims.typ != TokenType::Refresh {
        return Err(JwtMaker::INVALID_TOKEN_ERROR);
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
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    access_token: String,
    refresh_token: String,
}
