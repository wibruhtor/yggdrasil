use std::sync::Arc;

use axum::{Extension, Json};

use service::TwitchService;
use types::error::AppResult;
use types::twitch;

pub async fn handler(
    Extension(twitch_service): Extension<Arc<TwitchService>>,
) -> AppResult<Json<Vec<twitch::Badge>>> {
    let emotes = twitch_service.get_global_badges().await?;

    Ok(Json(emotes))
}
