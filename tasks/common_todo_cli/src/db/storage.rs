use crate::model::task::Task;

pub trait Storage: Send + Sync {
    fn create(&self, value: String) -> Result<Task, StorageError>;
    fn list(&self) -> Result<Vec<Task>, StorageError>;
    fn read(&self, id: u32) -> Result<Task, StorageError>;
    fn delete(&self, id: u32) -> Result<(), StorageError>;
    fn update(&self, id: u32, value: String) -> Result<Task, StorageError>;
    fn mark_ready_or_not(&self, id: u32, is_ready: bool) -> Result<Task, StorageError>;
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Record not found: {id}")]
    RecordNotFound { id: u32 },
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
