use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::BanWordFilter,
    error::{AppResult, ValidationErrorsWrapper},
    jwt::Claims,
    service::BanWordService,
};

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<UpdateBanWordFilterPathParams>,
    Json(request): Json<UpdateBanWordFilterRequest>,
) -> AppResult<Json<BanWordFilter>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let filter = ban_word_service
        .update_filter(&claims.sub, &path_params.ban_word_filter_id, &request.name)
        .await?;

    Ok(Json(filter))
}

#[derive(Deserialize)]
pub struct UpdateBanWordFilterPathParams {
    ban_word_filter_id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct UpdateBanWordFilterRequest {
    #[validate(length(min = 2, max = 32))]
    name: String,
}
