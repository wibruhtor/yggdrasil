use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use crate::service::AuthService;

pub async fn handler(Extension(auth_service): Extension<Arc<AuthService>>) -> Json<LoginResponse> {
    let url = auth_service.authorize_url();

    Json(LoginResponse { url })
}

#[derive(Serialize)]
pub struct LoginResponse {
    url: String,
}
