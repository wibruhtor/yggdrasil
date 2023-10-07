use config::Config;
use dao::Database;
use types::error::AppResult;

#[tokio::main]
async fn main() -> AppResult {
    let config = Config::load()?;

    let database = Database::new(config.database_config().postgres_url()).await?;

    println!("{:?}", database.postgres());

    Ok(())
}
