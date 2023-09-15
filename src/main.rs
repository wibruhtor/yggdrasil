use anyhow::Result;
use sqlx::PgPool;

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

    let pool = PgPool::connect(&config.database.postgres_url).await?;

    sqlx::migrate!().run(&pool).await?;

    http::server::run(config).await;

    Ok(())
}
