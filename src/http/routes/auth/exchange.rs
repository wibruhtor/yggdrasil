use std::sync::Arc;

use axum::{headers::UserAgent, http::HeaderMap, Extension, Json, TypedHeader};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    error::{AppResult, ValidationErrorsWrapper},
    service::AuthService,
};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    headers: HeaderMap,
    Json(request): Json<ExchangeRequest>,
) -> AppResult<Json<ExchangeResponse>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let ip: &str = headers.get("real-ip").unwrap().to_str().unwrap();

    let (access_token, refresh_token) = auth_service
        .exchange_code(&request.code, user_agent.as_str(), ip)
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
pub struct ExchangeResponse {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
}
