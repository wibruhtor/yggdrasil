use std::{net::SocketAddr, sync::Arc};

use axum::{extract::ConnectInfo, headers::UserAgent, Extension, Json, TypedHeader};
use serde::{Deserialize, Serialize};

use crate::{error::AppResult, service::AuthService};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(socket): ConnectInfo<SocketAddr>,
    Json(request): Json<ExchangeRequest>,
) -> AppResult<Json<ExchangeResponse>> {
    let (access_token, refresh_token) = auth_service
        .exchange_code(
            &request.code,
            user_agent.as_str(),
            socket.ip().to_string().as_ref(),
        )
        .await?;

    Ok(Json(ExchangeResponse {
        access_token,
        refresh_token,
    }))
}

#[derive(Deserialize)]
pub struct ExchangeRequest {
    code: String,
}

#[derive(Debug, Serialize)]
pub struct ExchangeResponse {
    access_token: String,
    refresh_token: String,
}
