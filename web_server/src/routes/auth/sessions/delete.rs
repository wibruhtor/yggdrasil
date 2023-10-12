use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension};
use serde::Deserialize;
use uuid::Uuid;

use service::SessionService;
use types::error::AppResult;
use utils::jwt::Claims;

pub async fn handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<DeleteSessionPathParams>,
) -> AppResult<StatusCode> {
    session_service
        .delete_session(&claims.sub, &path_params.session_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct DeleteSessionPathParams {
    session_id: Uuid,
}
