use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct Database {
    pub postgres_url: String,
}

impl Database {
    pub fn new() -> Self {
        Database {
            postgres_url: env::var("DATABASE_URL").expect("fail get DATABASE_URL"),
        }
    }
}
