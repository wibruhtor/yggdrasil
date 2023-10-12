use std::sync::Arc;

use axum::{headers::UserAgent, Extension, Json, TypedHeader};
use axum_client_ip::LeftmostXForwardedFor;
use serde::{Deserialize, Serialize};
use validator::Validate;

use service::AuthService;
use types::error::{AppResult, ValidationErrorsWrapper};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ip: LeftmostXForwardedFor,
    Json(request): Json<ExchangeRequest>,
) -> AppResult<Json<ExchangeResponse>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let ip = ip.0.to_string();

    let (access_token, refresh_token) = auth_service
        .exchange_code(&request.code, user_agent.as_str(), &ip)
        .await?;

    Ok(Json(ExchangeResponse {
        access_token,
        refresh_token,
    }))
}

#[derive(Deserialize, Validate)]
pub struct ExchangeRequest {
    #[validate(length(min = 1))]
    code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeResponse {
    access_token: String,
    refresh_token: String,
}
