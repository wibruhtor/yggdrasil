use anyhow::Result;

mod config;
mod http;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new()?;

    http::server::run(config).await;

    Ok(())
}
