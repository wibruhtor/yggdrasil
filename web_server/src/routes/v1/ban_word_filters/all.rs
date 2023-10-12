use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use service::BanWordService;
use types::domain::BanWordFilterInfo;
use types::error::AppResult;
use utils::jwt::Claims;

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<Json<GetAllBanWordFiltersResponse>> {
    let ban_word_filters = ban_word_service.get_all_filters(&claims.sub).await?;

    Ok(Json(GetAllBanWordFiltersResponse { ban_word_filters }))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllBanWordFiltersResponse {
    ban_word_filters: Vec<BanWordFilterInfo>,
}
