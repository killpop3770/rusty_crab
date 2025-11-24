use crate::db::{AsyncStorage, storage::StorageError};
use crate::model::task::Task;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::Row;

pub struct PostgresStorage {
    pool: PgPool,
    table_name: String,
}

impl PostgresStorage {
    pub fn new(pool: PgPool, table_name: String) -> Self {
        Self { pool, table_name }
    }

    pub async fn init_table(&self) -> Result<(), StorageError> {
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id VARCHAR PRIMARY KEY,
                value TEXT,
                is_ready BOOLEAN
            )",
            self.table_name
        );

        sqlx::query(&query).execute(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl AsyncStorage for PostgresStorage {
    async fn create(&self, _value: String) -> Result<crate::model::task::Task, StorageError> {
        todo!()
    }

    async fn list(&self) -> Result<Vec<crate::model::task::Task>, StorageError> {
        let query = format!("SELECT id, value, is_ready FROM {}", self.table_name);
        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;

        let mut items = Vec::new();
        for row in rows {
            let task = Task {
                id: row.get("id"),
                value: row.get("value"),
                is_ready: row.get("is_ready"),
            };
            items.push(task);
        }

        Ok(items)
    }

    async fn read(&self, _id: u32) -> Result<crate::model::task::Task, StorageError> {
        todo!()
    }

    async fn delete(&self, _id: u32) -> Result<(), StorageError> {
        todo!()
    }

    async fn update(
        &self,
        _id: u32,
        _value: String,
    ) -> Result<crate::model::task::Task, StorageError> {
        todo!()
    }

    async fn mark_ready_or_not(
        &self,
        _id: u32,
        _is_ready: bool,
    ) -> Result<crate::model::task::Task, StorageError> {
        todo!()
    }
}
