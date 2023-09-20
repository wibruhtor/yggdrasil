use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{domain::BanWord, error::AppResult, jwt::Claims};

pub async fn handler(
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<UpdateBanWordsPathParams>,
    Json(request): Json<UpdateBanWordsRequest>,
) -> AppResult<Json<UpdateBanWordsResponse>> {
    todo!()
}

#[derive(Deserialize)]
pub struct UpdateBanWordsPathParams {
    ban_word_filter_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateBanWordsRequest {
    ban_words: Vec<BanWord>,
}

#[derive(Serialize)]
pub struct UpdateBanWordsResponse {
    ban_words: Vec<BanWord>,
}
