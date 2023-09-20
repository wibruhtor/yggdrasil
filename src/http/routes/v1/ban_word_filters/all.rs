use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use crate::{domain::BanWordFilter, error::AppResult, jwt::Claims, service::BanWordService};

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<Json<GetAllBanWordFiltersResponse>> {
    let filters = ban_word_service.get_all_filters(&claims.sub).await?;

    Ok(Json(GetAllBanWordFiltersResponse { filters }))
}

#[derive(Serialize)]
pub struct GetAllBanWordFiltersResponse {
    filters: Vec<BanWordFilter>,
}
