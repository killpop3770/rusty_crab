pub mod config;
pub mod json_storage;

use crate::config::app_config::AppConfig;
use crate::db::errors::StorageError;

pub use json_storage::JsonStorage;

pub type StorageImpl = JsonStorage;

pub async fn create_json_storage(config: AppConfig) -> Result<StorageImpl, StorageError> {
    let json_config = config.json_storage.ok_or(StorageError::ConfigLoadError(
        "Can not load JSON config file".to_string(),
    ))?;

    let storage = JsonStorage::new(json_config.file_path).await?;
    Ok(storage)
}
