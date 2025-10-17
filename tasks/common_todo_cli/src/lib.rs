use crate::{
    db::{Storage, storage::StorageError},
    model::task::Task,
};

pub mod db;
pub mod model;

#[derive(Default)]
pub struct CommonTodo<S>
where
    S: Storage,
{
    storage: S,
}

impl<S> CommonTodo<S>
where
    S: Storage,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn create(&self, value: String) -> Result<Task, StorageError> {
        self.storage.create(value)
    }

    pub fn list(&self) -> Result<Vec<Task>, StorageError> {
        self.storage.list()
    }

    pub fn read(&self, id: u32) -> Result<Task, StorageError> {
        self.storage.read(id)
    }

    pub fn delete(&self, id: u32) -> Result<(), StorageError> {
        self.storage.delete(id)
    }

    pub fn update(&self, id: u32, value: String) -> Result<Task, StorageError> {
        self.storage.update(id, value)
    }

    pub fn mark_ready_or_not(&self, id: u32, is_ready: bool) -> Result<Task, StorageError> {
        self.storage.mark_ready_or_not(id, is_ready)
    }
}

pub fn ready_or_not(is_ready: bool) -> String {
    match is_ready {
        true => String::from("ready"),
        false => String::from("not ready"),
    }
}
