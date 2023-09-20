use std::sync::Arc;

use axum::Extension;
use reqwest::StatusCode;

use crate::{error::AppResult, jwt::Claims, service::SessionService};

pub async fn handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<StatusCode> {
    session_service
        .delete_all_exclude_current(&claims.sub, &claims.jti)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
