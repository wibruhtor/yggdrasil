use std::sync::Arc;

use axum::{Extension, Json};

use crate::{domain::TwitchBadge, error::AppResult, service::TwitchService};

pub async fn handler(
    Extension(twitch_service): Extension<Arc<TwitchService>>,
) -> AppResult<Json<Vec<TwitchBadge>>> {
    let emotes = twitch_service.get_global_badges().await?;

    Ok(Json(emotes))
}
