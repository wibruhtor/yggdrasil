use anyhow::Result;

mod config;
mod web_server;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new()?;

    web_server::run(config).await;

    Ok(())
}
