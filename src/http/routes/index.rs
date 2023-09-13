use anyhow::bail;
use axum::routing::get;
use axum::Router;

use crate::error::AppResult;

pub fn routes() -> Router {
    Router::new().route("/", get(index_route))
}

async fn index_route() -> AppResult {
    test()?;
    Ok(())
}

fn test() -> Result<(), anyhow::Error> {
    bail!("some cringe")
}
