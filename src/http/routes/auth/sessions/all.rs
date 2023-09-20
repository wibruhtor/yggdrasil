use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;

use crate::{domain::Token, error::AppResult, jwt::Claims, service::SessionService};

pub async fn handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Extension(claims): Extension<Arc<Claims>>,
) -> AppResult<Json<AllSessionsResponse>> {
    let sessions = session_service.get_all(&claims.sub).await?;

    Ok(Json(AllSessionsResponse { sessions }))
}

#[derive(Serialize)]
pub struct AllSessionsResponse {
    sessions: Vec<Token>,
}
