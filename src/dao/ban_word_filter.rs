use std::sync::Arc;

use sqlx::{Pool, Postgres};

#[allow(dead_code)]
pub struct BanWordFilterDao {
    pool: Arc<Box<Pool<Postgres>>>,
}

#[allow(dead_code)]
impl BanWordFilterDao {
    pub fn new(pool: Arc<Box<Pool<Postgres>>>) -> Arc<Self> {
        Arc::new(BanWordFilterDao { pool })
    }
}
