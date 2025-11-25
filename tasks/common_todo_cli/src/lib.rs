use sqlx::postgres::PgPoolOptions;

use crate::{
    db::{
        AsyncStorage, config::AppConfig, postgres::postgres_storage::PostgresStorage,
        storage::StorageError,
    },
    model::task::Task,
};

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

    pub async fn read(&self, id: i32) -> Result<Task, StorageError> {
        self.storage.read(id).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), StorageError> {
        self.storage.delete(id).await
    }

    pub async fn update(&self, id: i32, value: String) -> Result<Task, StorageError> {
        self.storage.update(id, value).await
    }

    pub async fn mark_ready_or_not(&self, id: i32, is_ready: bool) -> Result<Task, StorageError> {
        self.storage.mark_ready_or_not(id, is_ready).await
    }
}

pub fn ready_or_not(is_ready: bool) -> String {
    match is_ready {
        true => String::from("ready"),
        false => String::from("not ready"),
    }
}

pub enum StorageType {
    Json,
    Postgres,
    Mongodb,
}

pub struct StorageFactory;

impl StorageFactory {
    pub async fn create(
        storage_type: StorageType,
        config: AppConfig,
    ) -> Result<impl AsyncStorage, StorageError> {
        match storage_type {
            StorageType::Json => Ok(create_json_storage(config).await?),
            StorageType::Postgres => Ok(create_postgres(config).await?),
            StorageType::Mongodb => Ok(create_mongodb(config).await?),
        }
    }
}

async fn create_postgres(config: AppConfig) -> Result<PostgresStorage, StorageError> {
    let pg_config = config.postgres.expect("Failed to load PostgreSQL config");
    let db_url = format!(
        "postgresql://{}:{}@{}/{}",
        pg_config.username, pg_config.password, pg_config.address, pg_config.database
    );
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL");
    let storage = PostgresStorage::new(pool, pg_config.table);
    Ok(storage)
}

async fn create_json_storage(config: AppConfig) -> Result<PostgresStorage, StorageError> {
    todo!()
}

async fn create_mongodb(config: AppConfig) -> Result<PostgresStorage, StorageError> {
    todo!()
}
