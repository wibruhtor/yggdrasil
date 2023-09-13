use crate::http::router;
use anyhow::Result;

mod config;
mod http;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new()?;

    router::run(config).await;

    Ok(())
}
