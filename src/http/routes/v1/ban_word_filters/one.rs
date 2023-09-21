use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{domain::BanWordFilter, error::AppResult, jwt::Claims, service::BanWordService};

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<GetBanWordFilterPathParams>,
) -> AppResult<Json<GetBanWordFilterResponse>> {
    let (filter, ban_words) = ban_word_service
        .get_filter(&claims.sub, &path_params.ban_word_filter_id)
        .await?;

    Ok(Json(GetBanWordFilterResponse { filter, ban_words }))
}

#[derive(Deserialize)]
pub struct GetBanWordFilterPathParams {
    ban_word_filter_id: Uuid,
}

#[derive(Serialize)]
pub struct GetBanWordFilterResponse {
    filter: BanWordFilter,
    #[serde(rename = "banWords")]
    ban_words: Vec<String>,
}
