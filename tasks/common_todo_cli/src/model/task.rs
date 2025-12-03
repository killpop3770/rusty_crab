use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub value: String,
    pub is_ready: bool,
}

impl Task {
    pub fn new(id: String, value: String) -> Self {
        Self {
            id,
            value,
            is_ready: false,
        }
    }
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
