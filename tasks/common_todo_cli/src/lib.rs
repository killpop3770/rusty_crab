use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;

use crate::{
    db::{
        AsyncStorage, config::AppConfig, json_storage::json_storage::JsonStorage,
        postgres::postgres_storage::PostgresStorage, storage::StorageError,
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

pub enum StorageBackend {
    Json(JsonStorage),
    Postgres(PostgresStorage),
    Mongodb,
}

#[async_trait]
impl AsyncStorage for StorageBackend {
    async fn create(&self, value: String) -> Result<Task, StorageError> {
        match self {
            StorageBackend::Json(s) => s.create(value).await,
            StorageBackend::Postgres(s) => s.create(value).await,
            StorageBackend::Mongodb => todo!(),
        }
    }
    async fn list(&self) -> Result<Vec<Task>, StorageError> {
        match self {
            StorageBackend::Json(s) => s.list().await,
            StorageBackend::Postgres(s) => s.list().await,
            StorageBackend::Mongodb => todo!(),
        }
    }
    async fn read(&self, id: i32) -> Result<Task, StorageError> {
        match self {
            StorageBackend::Json(s) => s.read(id).await,
            StorageBackend::Postgres(s) => s.read(id).await,
            StorageBackend::Mongodb => todo!(),
        }
    }
    async fn delete(&self, id: i32) -> Result<(), StorageError> {
        match self {
            StorageBackend::Json(s) => s.delete(id).await,
            StorageBackend::Postgres(s) => s.delete(id).await,
            StorageBackend::Mongodb => todo!(),
        }
    }
    async fn update(&self, id: i32, value: String) -> Result<Task, StorageError> {
        match self {
            StorageBackend::Json(s) => s.update(id, value).await,
            StorageBackend::Postgres(s) => s.update(id, value).await,
            StorageBackend::Mongodb => todo!(),
        }
    }
    async fn mark_ready_or_not(&self, id: i32, is_ready: bool) -> Result<Task, StorageError> {
        match self {
            StorageBackend::Json(s) => s.mark_ready_or_not(id, is_ready).await,
            StorageBackend::Postgres(s) => s.mark_ready_or_not(id, is_ready).await,
            StorageBackend::Mongodb => todo!(),
        }
    }
}

pub struct StorageFactory;

impl StorageFactory {
    pub async fn create(
        storage_type: StorageType,
        config: AppConfig,
    ) -> Result<StorageBackend, StorageError> {
        match storage_type {
            StorageType::Json => {
                let storage = create_json_storage(config).await?;
                println!("StorageType::Json");
                Ok(StorageBackend::Json(storage))
            }
            StorageType::Postgres => {
                let storage = create_postgres_storage(config).await?;
                println!("StorageType::Postgres");
                Ok(StorageBackend::Postgres(storage))
            }
            StorageType::Mongodb => {
                let _ = create_mongodb_storage(config).await?;
                println!("StorageType::Mongodb");
                Ok(StorageBackend::Mongodb)
            }
        }
    }
}

async fn create_postgres_storage(config: AppConfig) -> Result<PostgresStorage, StorageError> {
    let pg_config = config.postgres.ok_or(StorageError::ConfigLoadError())?;
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

async fn create_json_storage(config: AppConfig) -> Result<JsonStorage, StorageError> {
    let json_config = config.json_storage.ok_or(StorageError::ConfigLoadError())?;
    let path = json_config.file_path;
    let storage = JsonStorage::new(path).await?;
    Ok(storage)
}

async fn create_mongodb_storage(_config: AppConfig) -> Result<PostgresStorage, StorageError> {
    todo!()
}
