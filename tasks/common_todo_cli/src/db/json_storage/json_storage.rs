use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    db::{AsyncStorage, storage::StorageError},
    model::task::Task,
};

const FIRST_ITEM: i32 = 1;

#[derive(Serialize, Deserialize)]
struct InnerDB {
    tasks: Vec<Task>,
    next_id: i32,
}

pub struct JsonStorage {
    file_path: PathBuf,
}

impl JsonStorage {
    pub async fn new(file_path: String) -> Result<Self, StorageError> {
        let path = PathBuf::from(&file_path);

        if !tokio::fs::try_exists(&path).await? {
            let inner_db = InnerDB {
                tasks: vec![],
                next_id: FIRST_ITEM,
            };
            let data = serde_json::to_string_pretty(&inner_db)?;
            tokio::fs::write(path.clone(), data).await?;
        }

        Ok(Self { file_path: path })
    }

    async fn load_inner_db(&self) -> Result<InnerDB, StorageError> {
        let path = self.file_path.clone();
        let content = tokio::fs::read_to_string(path).await?;
        let data: InnerDB = serde_json::from_str(&content)?;
        Ok(data)
    }

    async fn save_inner_db(&self, inner_db: &InnerDB) -> Result<(), StorageError> {
        let data = serde_json::to_string_pretty(inner_db)?;
        tokio::fs::write(self.file_path.clone(), data).await?;
        Ok(())
    }
}

#[async_trait]
impl AsyncStorage for JsonStorage {
    async fn create(&self, value: String) -> Result<Task, StorageError> {
        let mut inner_db = self.load_inner_db().await?;
        let id = inner_db.next_id;

        let task = Task::new(id, value);
        inner_db.tasks.push(task.clone());
        inner_db.next_id += 1;

        self.save_inner_db(&inner_db).await?;

        Ok(task)
    }

    async fn read(&self, id: i32) -> Result<Task, StorageError> {
        let inner_db = self.load_inner_db().await?;

        let task = inner_db
            .tasks
            .iter()
            .find(|task| task.id == id)
            .cloned()
            .ok_or(StorageError::RecordNotFound { id })?;

        Ok(task)
    }

    async fn update(&self, id: i32, value: String) -> Result<Task, StorageError> {
        let mut inner_db = self.load_inner_db().await?;

        let task = inner_db
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or(StorageError::RecordNotFound { id })?;
        task.value = value;

        let result_task = task.clone();

        self.save_inner_db(&inner_db).await?;
        Ok(result_task)
    }

    async fn delete(&self, id: i32) -> Result<(), StorageError> {
        let mut inner_db = self.load_inner_db().await?;

        let task_index = inner_db
            .tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or(StorageError::RecordNotFound { id })?;

        inner_db.tasks.remove(task_index);

        self.save_inner_db(&inner_db).await?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Task>, StorageError> {
        let inner_db = self.load_inner_db().await?;
        let tasks = inner_db.tasks;
        Ok(tasks)
    }

    async fn mark_ready_or_not(&self, id: i32, is_ready: bool) -> Result<Task, StorageError> {
        let mut inner_db = self.load_inner_db().await?;

        let task = inner_db
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or(StorageError::RecordNotFound { id })?;
        task.is_ready = is_ready;

        let result_task = task.clone();

        self.save_inner_db(&inner_db).await?;
        Ok(result_task)
    }
}
