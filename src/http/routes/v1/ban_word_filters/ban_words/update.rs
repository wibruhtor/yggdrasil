use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppResult, jwt::Claims, service::BanWordService};

pub async fn handler(
    Extension(ban_word_service): Extension<Arc<BanWordService>>,
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<UpdateBanWordsPathParams>,
    Json(request): Json<UpdateBanWordsRequest>,
) -> AppResult<StatusCode> {
    ban_word_service
        .update_ban_words(
            &claims.sub,
            &path_params.ban_word_filter_id,
            &request.ban_words,
        )
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct UpdateBanWordsPathParams {
    ban_word_filter_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateBanWordsRequest {
    ban_words: Vec<String>,
}
