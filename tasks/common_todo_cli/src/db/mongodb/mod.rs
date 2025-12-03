pub mod config;
pub mod mongodb_storage;
use crate::config::app_config::AppConfig;
use crate::db::errors::StorageError;
use crate::model::task::Task;
use ::mongodb::Client;
use mongodb::bson::oid::ObjectId;

pub use mongodb_storage::MongodbStorage;
use serde::{Deserialize, Serialize};

pub type StorageImpl = MongodbStorage;

pub async fn create_mongodb_storage(config: AppConfig) -> Result<StorageImpl, StorageError> {
    let mongodb_config = config.mongodb.ok_or(StorageError::ConfigLoadError(
        "Can not load Mongodb config file".to_string(),
    ))?;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub value: String,
    pub is_ready: bool,
}

impl From<TaskDocument> for Task {
    fn from(mongo_task: TaskDocument) -> Self {
        Self {
            id: mongo_task.id.unwrap().to_string(),
            value: mongo_task.value,
            is_ready: mongo_task.is_ready,
        }
    }
}
