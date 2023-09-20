use std::sync::Arc;

use axum::Extension;

use crate::{error::AppResult, jwt::Claims};

pub async fn handler(Extension(claims): Extension<Arc<Claims>>) -> AppResult {
    todo!()
}
