use anyhow::Result;

mod config;
mod error;
mod http;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::Config::new().expect("fail load config");

    tracing_subscriber::fmt()
        .json()
        .with_max_level(config.logging.level)
        .with_current_span(false)
        .try_init()
        .expect("fail init tracing subscriber");

    http::server::run(config).await;

    Ok(())
}
