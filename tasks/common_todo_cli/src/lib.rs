use crate::{
    db::{AsyncStorage, errors::StorageError},
    model::task::Task,
};

pub mod config;
pub mod db;
pub mod gui;
pub mod model;

#[derive(Default)]
pub struct CommonTodo<S>
where
    S: AsyncStorage,
{
    storage: S,
}

impl<S> CommonTodo<S>
where
    S: AsyncStorage,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub async fn create(&self, value: String) -> Result<Task, StorageError> {
        self.storage.create(value).await
    }

    pub async fn list(&self) -> Result<Vec<Task>, StorageError> {
        self.storage.list().await
    }

    pub async fn read(&self, id: String) -> Result<Task, StorageError> {
        self.storage.read(id).await
    }

    pub async fn delete(&self, id: String) -> Result<(), StorageError> {
        self.storage.delete(id).await
    }

    pub async fn update(&self, id: String, value: String) -> Result<Task, StorageError> {
        self.storage.update(id, value).await
    }

    pub async fn mark_ready_or_not(
        &self,
        id: String,
        is_ready: bool,
    ) -> Result<Task, StorageError> {
        self.storage.mark_ready_or_not(id, is_ready).await
    }
}

pub fn ready_or_not(is_ready: bool) -> String {
    match is_ready {
        true => String::from("ready"),
        false => String::from("not ready"),
    }
}
