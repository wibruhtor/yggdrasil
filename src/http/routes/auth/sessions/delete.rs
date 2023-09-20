use std::sync::Arc;

use axum::{extract::Path, Extension};
use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppResult, jwt::Claims, service::SessionService};

pub async fn handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<DeletePathParams>,
) -> AppResult<StatusCode> {
    session_service
        .delete(&claims.sub, &path_params.session_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct DeletePathParams {
    session_id: Uuid,
}
