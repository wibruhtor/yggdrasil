use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppResult, jwt::Claims, service::BanWordService};

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<GetBanWordsPathParams>,
) -> AppResult<Json<GetBanWordsResponse>> {
    let ban_words = ban_word_service
        .get_ban_words(&claims.sub, &path_params.ban_word_filter_id)
        .await?;

    Ok(Json(GetBanWordsResponse { ban_words }))
}

#[derive(Deserialize)]
pub struct GetBanWordsPathParams {
    ban_word_filter_id: Uuid,
}

#[derive(Serialize)]
pub struct GetBanWordsResponse {
    ban_words: Vec<String>,
}
