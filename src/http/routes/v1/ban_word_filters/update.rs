use std::sync::Arc;

use axum::{extract::Path, Extension};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppResult, jwt::Claims};

pub async fn handler(
    Extension(claims): Extension<Arc<Claims>>,
    Path(path_params): Path<UpdateBanWordFilterPathParams>,
) -> AppResult {
    todo!()
}

#[derive(Deserialize)]
pub struct UpdateBanWordFilterPathParams {
    ban_word_filter_id: Uuid,
}
