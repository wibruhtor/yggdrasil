use std::sync::Arc;

use sqlx::{Pool, Postgres};

#[allow(dead_code)]
pub struct BanWordDao {
    pool: Arc<Box<Pool<Postgres>>>,
}

#[allow(dead_code)]
impl BanWordDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>) -> Arc<Self> {
        Arc::new(BanWordDao { pool })
    }
}
