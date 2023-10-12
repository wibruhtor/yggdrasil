use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use service::BanWordService;
use types::domain::BanWordFilter;
use types::error::AppResult;

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Path(path_params): Path<GetBanWordFilterPathParams>,
) -> AppResult<Json<BanWordFilter>> {
    let ban_word_filter = ban_word_service
        .get_filter(&path_params.ban_word_filter_id)
        .await?;

    Ok(Json(ban_word_filter))
}

#[derive(Deserialize)]
pub struct GetBanWordFilterPathParams {
    ban_word_filter_id: Uuid,
}
