use async_trait::async_trait;
use futures_util::TryStreamExt;
use mongodb::{
    Client, Collection, Database,
    bson::{doc, oid::ObjectId},
};

use crate::{
    db::{AsyncStorage, errors::StorageError},
    model::task::{Task, TaskDocument},
};

#[derive(Debug, Clone)]
pub struct MongodbStorage {
    collection: Collection<TaskDocument>,
}

impl MongodbStorage {
    pub async fn new(
        client: Client,
        database: &str,
        collection: &str,
    ) -> Result<Self, StorageError> {
        let db: Database = client.database(database);
        let collection: Collection<TaskDocument> = db.collection(collection);

        let index_model = mongodb::IndexModel::builder()
            .keys(doc! { "task_id": 1 })
            .build();
        collection.create_index(index_model).await?;

        Ok(Self { collection })
    }

    pub fn convert_id(&self, id: String) -> Result<ObjectId, StorageError> {
        match ObjectId::parse_str(&id) {
            Ok(oid) => Ok(oid),
            Err(_) => Err(StorageError::InvalidInput {
                msg: format!("Invalid ObjectId format: {id}"),
            }),
        }
    }
}

#[async_trait]
impl AsyncStorage for MongodbStorage {
    async fn create(&self, value: String) -> Result<Task, StorageError> {
        let mongo_task = TaskDocument {
            id: None,
            value,
            is_ready: false,
        };

        let insert_result = self.collection.insert_one(&mongo_task).await?;

        let filter = doc! { "_id": &insert_result.inserted_id };
        let created_doc = self.collection.find_one(filter).await?;
        let task_doc = created_doc.ok_or(StorageError::UnknownError(
            "Created task not found".to_string(),
        ))?;

        Ok(Task::from(task_doc))
    }

    async fn read(&self, id: String) -> Result<Task, StorageError> {
        let object_id = self.convert_id(id)?;
        let filter = doc! { "_id": &object_id };

        match self.collection.find_one(filter).await? {
            Some(task_doc) => Ok(Task::from(task_doc)),
            None => Err(StorageError::RecordNotFound {
                id: object_id.to_string(),
            }),
        }
    }

    async fn update(&self, id: String, value: String) -> Result<Task, StorageError> {
        let object_id = self.convert_id(id)?;
        let filter = doc! { "_id": &object_id };

        let update = doc! {
            "$set": {
                "value": &value,
            }
        };

        match self.collection.find_one_and_update(filter, update).await? {
            Some(updated_doc) => Ok(Task::from(updated_doc)),
            None => Err(StorageError::RecordNotFound {
                id: object_id.to_string(),
            }),
        }
    }

    async fn delete(&self, id: String) -> Result<(), StorageError> {
        let object_id = self.convert_id(id)?;
        let filter = doc! { "_id": &object_id };

        let result = self.collection.delete_one(filter).await?;

        if result.deleted_count == 0 {
            Err(StorageError::RecordNotFound {
                id: object_id.to_string(),
            })
        } else {
            Ok(())
        }
    }

    async fn list(&self) -> Result<Vec<Task>, StorageError> {
        let filter = doc! {};

        let mut cursor = self.collection.find(filter).await?;
        let mut tasks = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            tasks.push(Task::from(doc));
        }

        Ok(tasks)
    }

    async fn mark_ready_or_not(&self, id: String, is_ready: bool) -> Result<Task, StorageError> {
        let object_id = self.convert_id(id)?;
        let filter = doc! { "_id": &object_id };

        let update = doc! {
            "$set": {
                "is_ready": is_ready,
            }
        };

        match self.collection.find_one_and_update(filter, update).await? {
            Some(updated_doc) => Ok(Task::from(updated_doc)),
            None => Err(StorageError::RecordNotFound {
                id: object_id.to_string(),
            }),
        }
    }
}
