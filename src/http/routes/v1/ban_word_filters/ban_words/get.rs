use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{domain::BanWord, error::AppResult, jwt::Claims};

pub async fn handler(
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<GetBanWordsPathParams>,
) -> AppResult<Json<GetBanWordsResponse>> {
    todo!()
}

#[derive(Deserialize)]
pub struct GetBanWordsPathParams {
    ban_word_filter_id: Uuid,
}

#[derive(Serialize)]
pub struct GetBanWordsResponse {
    ban_words: Vec<BanWord>,
}
