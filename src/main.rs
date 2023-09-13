use anyhow::Result;

mod config;
mod error;
mod http;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new()?;

    http::server::run(config).await;

    Ok(())
}
