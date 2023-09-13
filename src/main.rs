use std::error::Error;

mod config;

// #[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::new()?;

    println!("{:#?}", config);

    Ok(())
}
