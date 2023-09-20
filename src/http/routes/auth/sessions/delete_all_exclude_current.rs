use std::sync::Arc;

use axum::{http::StatusCode, Extension};

use crate::{error::AppResult, jwt::Claims, service::SessionService};

pub async fn handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<StatusCode> {
    session_service
        .delete_all_sessions_exclude_current(&claims.sub, &claims.jti)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
