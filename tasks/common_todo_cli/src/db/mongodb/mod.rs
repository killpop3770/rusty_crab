pub mod config;
pub mod mongodb_storage;

use crate::config::app_config::AppConfig;
use crate::db::errors::StorageError;
use ::mongodb::Client;

pub use mongodb_storage::MongodbStorage;

pub type StorageImpl = MongodbStorage;

pub async fn create_mongodb_storage(config: AppConfig) -> Result<StorageImpl, StorageError> {
    let mongodb_config = config.mongodb.ok_or(StorageError::ConfigLoadError())?;

    let url = format!(
        "mongodb://{}:{}@{}:{}",
        mongodb_config.username,
        mongodb_config.password,
        mongodb_config.address,
        mongodb_config.port
    );

    let client = Client::with_uri_str(&url).await?;
    let storage =
        MongodbStorage::new(client, &mongodb_config.database, &mongodb_config.collection).await?;

    Ok(storage)
}
