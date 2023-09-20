use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;

use crate::{domain::BanWordFilter, error::AppResult, jwt::Claims, service::BanWordService};

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(request): Json<CreateBanWordFilterRequest>,
) -> AppResult<Json<BanWordFilter>> {
    let filter = ban_word_service
        .create_filter(&claims.sub, &request.name)
        .await?;

    Ok(Json(filter))
}

#[derive(Deserialize)]
pub struct CreateBanWordFilterRequest {
    name: String,
}
