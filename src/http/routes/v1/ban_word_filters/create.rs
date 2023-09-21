use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
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
    Json(request): Json<CreateBanWordFilterRequest>,
) -> AppResult<Json<BanWordFilter>> {
    request
        .validate()
        .map_err(|e| ValidationErrorsWrapper::from(e))?;

    let filter = ban_word_service
        .create_filter(&claims.sub, &request.name)
        .await?;

    Ok(Json(filter))
}

#[derive(Deserialize, Validate)]
pub struct CreateBanWordFilterRequest {
    #[validate(length(min = 2, max = 32))]
    name: String,
}
