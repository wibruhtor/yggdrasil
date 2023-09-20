use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppResult, jwt::Claims, service::BanWordService};

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<DeleteBanWordFilterPathParams>,
) -> AppResult<StatusCode> {
    ban_word_service
        .delete_filter(&claims.sub, &path_params.ban_word_filter_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct DeleteBanWordFilterPathParams {
    ban_word_filter_id: Uuid,
}
