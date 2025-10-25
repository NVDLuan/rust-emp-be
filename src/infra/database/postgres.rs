use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::time::Duration;
use crate::errors::{AppError, AppResult};

pub type DbPool = DatabaseConnection;

pub async fn establish_connection(database_url: String) -> AppResult<DbPool> {
    let mut opt = ConnectOptions::new(database_url);
    
    // Configure connection pool
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    Database::connect(opt)
        .await
        .map_err(|e| AppError::Database(e))
}
