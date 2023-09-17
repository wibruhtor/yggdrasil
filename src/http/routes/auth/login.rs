use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use crate::http::app::AppState;

pub async fn handler(Extension(app_state): Extension<Arc<AppState>>) -> Json<LoginResponse> {
    let url = app_state.auth_service.authorize_url();

    Json(LoginResponse { url })
}

#[derive(Serialize)]
pub struct LoginResponse {
    url: String,
}
