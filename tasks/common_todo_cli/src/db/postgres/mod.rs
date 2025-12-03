pub mod config;
pub mod postgres_storage;

use crate::config::app_config::AppConfig;
use crate::db::errors::StorageError;
use sqlx::postgres::PgPoolOptions;

pub use postgres_storage::PostgresStorage;

pub type StorageImpl = PostgresStorage;

pub async fn create_postgres_storage(config: AppConfig) -> Result<StorageImpl, StorageError> {
    let pg_config = config.postgres.ok_or(StorageError::ConfigLoadError(
        "Can not load Postgres config file".to_string(),
    ))?;
    let db_url = format!(
        "postgresql://{}:{}@{}/{}",
        pg_config.username, pg_config.password, pg_config.address, pg_config.database
    );
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL");
    let storage = PostgresStorage::new(pool, pg_config.table);
    Ok(storage)
}
