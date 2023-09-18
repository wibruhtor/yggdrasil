use std::sync::Arc;

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{error::AppResult, service::AuthService};

pub async fn handler(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Json(request): Json<RefreshRequest>,
) -> AppResult<Json<RefreshResponse>> {
    let (access_token, refresh_token) = auth_service.refresh_token(&request.token).await?;

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
