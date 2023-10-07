use std::sync::Arc;

use axum::http::StatusCode;
use sqlx::{PgPool, Pool, Postgres};

use types::error::{AppError, AppResult};

pub struct Database {
    postgres: Arc<Pool<Postgres>>,
}

impl Database {
    pub async fn new(postgres_url: &str) -> AppResult<Self> {
        let pool = PgPool::connect(postgres_url)
            .await
            .map_err(|e| Database::CONNECT_TO_POSTGRES_ERROR.clone().cause(e.into()))?;

        sqlx::migrate!("../migrations")
            .run(&pool)
            .await
            .map_err(|e| Database::MIGRATE_POSTGRES_ERROR.clone().cause(e.into()))?;

        Ok(Database {
            postgres: Arc::new(pool),
        })
    }

    pub fn postgres(&self) -> Arc<Pool<Postgres>> {
        Arc::clone(&self.postgres)
    }
}

macro_rules! database_errors {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $status:expr, $phrase:expr);
        )+
    ) => {
        impl Database {
        $(
            $(#[$docs])*
            pub const $name: AppError = AppError {
                status_code: $status,
                message: Some($phrase),
                cause: None,
                other: None
            };
        )+
        }
    }
}

database_errors! {
    (CONNECT_TO_POSTGRES_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail connect to postgres");
    (MIGRATE_POSTGRES_ERROR, StatusCode::INTERNAL_SERVER_ERROR, "fail migrate postgres");
}
