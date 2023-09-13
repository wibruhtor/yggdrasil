use anyhow::bail;
use axum::routing::get;
use axum::Router;

use crate::error::AppResult;

pub fn routes() -> Router {
    Router::new().route("/", get(index_route))
}

async fn index_route() -> AppResult {
    tracing::info!("Start index route handler");
    let span = tracing::trace_span!("index route");
    let _span = span.enter();
    tracing::info!("Invoke test function");
    test()?;
    Ok(())
}

fn test() -> Result<(), anyhow::Error> {
    bail!("some cringe")
}
