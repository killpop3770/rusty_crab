use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Record not found: {id}")]
    RecordNotFound { id: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(String),

    #[error("PostgreSQL error: {0}")]
    SqlxError(String),

    #[error("MongoDB error: {0}")]
    MongodbError(String),

    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Storage choice error")]
    StorageChoiceError(),

    #[error("Config load error: {0}")]
    ConfigLoadError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),

    #[error("Invalid input error: {msg}")]
    InvalidInput { msg: String },
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::JsonError(err.to_string())
    }
}

#[cfg(feature = "postgres")]
impl From<sqlx::Error> for StorageError {
    fn from(err: sqlx::Error) -> Self {
        StorageError::SqlxError(err.to_string())
    }
}

#[cfg(feature = "mongodb")]
impl From<mongodb::error::Error> for StorageError {
    fn from(err: mongodb::error::Error) -> Self {
        StorageError::MongodbError(err.to_string())
    }
}
