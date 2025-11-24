use async_trait::async_trait;

use crate::model::task::Task;

#[async_trait]
pub trait AsyncStorage {
    async fn create(&self, value: String) -> Result<Task, StorageError>;
    async fn list(&self) -> Result<Vec<Task>, StorageError>;
    async fn read(&self, id: u32) -> Result<Task, StorageError>;
    async fn delete(&self, id: u32) -> Result<(), StorageError>;
    async fn update(&self, id: u32, value: String) -> Result<Task, StorageError>;
    async fn mark_ready_or_not(&self, id: u32, is_ready: bool) -> Result<Task, StorageError>;
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Record not found: {id}")]
    RecordNotFound { id: u32 },
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("PostgreSQL error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
