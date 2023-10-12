use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use service::BanWordService;
use types::domain::{BanWordFilter, UpdateBanWordFilter};
use types::error::{AppResult, ValidationErrorsWrapper};
use utils::jwt::Claims;

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<UpdateBanWordFilterPathParams>,
    Json(request): Json<UpdateBanWordFilter>,
) -> AppResult<Json<BanWordFilter>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let filter = ban_word_service
        .update_filter(&claims.sub, &path_params.ban_word_filter_id, &request)
        .await?;

    Ok(Json(filter))
}

#[derive(Deserialize)]
pub struct UpdateBanWordFilterPathParams {
    ban_word_filter_id: Uuid,
}
