pub mod errors;

use crate::{config::app_config::AppConfig, db::errors::StorageError, model::task::Task};
use async_trait::async_trait;

#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "json")]
pub use json::{JsonStorage, create_json_storage};

#[cfg(feature = "mongodb")]
pub mod mongodb;
#[cfg(feature = "mongodb")]
pub use mongodb::{MongodbStorage, create_mongodb_storage};

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "postgres")]
pub use postgres::{PostgresStorage, create_postgres_storage};

#[cfg(feature = "postgres")]
pub type StorageImpl = PostgresStorage;

#[cfg(feature = "mongodb")]
pub type StorageImpl = MongodbStorage;

#[cfg(feature = "json")]
pub type StorageImpl = JsonStorage;

#[async_trait]
pub trait AsyncStorage {
    async fn create(&self, value: String) -> Result<Task, StorageError>;
    async fn list(&self) -> Result<Vec<Task>, StorageError>;
    async fn read(&self, id: String) -> Result<Task, StorageError>;
    async fn delete(&self, id: String) -> Result<(), StorageError>;
    async fn update(&self, id: String, value: String) -> Result<Task, StorageError>;
    async fn mark_ready_or_not(&self, id: String, is_ready: bool) -> Result<Task, StorageError>;
}

pub async fn create_storage(config: AppConfig) -> Result<StorageImpl, StorageError> {
    #[cfg(feature = "postgres")]
    return create_postgres_storage(config).await;

    #[cfg(feature = "mongodb")]
    return create_mongodb_storage(config).await;

    #[cfg(feature = "json")]
    return create_json_storage(config).await;

    #[cfg(not(any(feature = "json", feature = "postgres", feature = "mongodb")))]
    compile_error!("At least one storage feature must be enabled");
}
