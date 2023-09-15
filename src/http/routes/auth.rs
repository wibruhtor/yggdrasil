use std::sync::Arc;

use axum::{routing, Extension, Json, Router};
use serde::Serialize;

use crate::http::app::AppState;

pub fn routes() -> Router {
    Router::new().route("/login", routing::get(login))
}

#[derive(Serialize)]
struct LoginResponse {
    url: String,
}

async fn login(Extension(app_state): Extension<Arc<AppState>>) -> Json<LoginResponse> {
    let url = app_state
        .auth_service
        .authorize_url(&uuid::Uuid::new_v4().to_string());

    Json(LoginResponse { url })
}
