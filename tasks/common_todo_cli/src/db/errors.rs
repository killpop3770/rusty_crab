#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Record not found: {id}")]
    RecordNotFound { id: String },
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("PostgreSQL error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Storage choice error")]
    StorageChoiceError(),
    #[error("Config load error")]
    ConfigLoadError(),
    #[error("Unknown error occured by: {0}")]
    UnknownError(String),
    #[error("Invalid input error: {msg}")]
    InvalidInput { msg: String },
    #[error("Mongodb error: {0}")]
    MongodbError(#[from] mongodb::error::Error),
}
