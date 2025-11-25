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
                id SERIAL PRIMARY KEY,
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
    async fn create(&self, value: String) -> Result<crate::model::task::Task, StorageError> {
        let query = format!(
            "INSERT INTO {} (value, is_ready) VALUES ($1, $2) RETURNING id, value, is_ready",
            self.table_name
        );
        let row = sqlx::query(&query)
            .bind(value)
            .bind(false)
            .fetch_one(&self.pool)
            .await?;
        let task = Task {
            id: row.get("id"),
            value: row.get("value"),
            is_ready: row.get("is_ready"),
        };

        Ok(task)
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

    async fn read(&self, id: i32) -> Result<crate::model::task::Task, StorageError> {
        let query = format!(
            "SELECT id, value, is_ready FROM {} WHERE id = $1",
            self.table_name
        );
        let row = sqlx::query(&query).bind(id).fetch_one(&self.pool).await?;

        let task = Task {
            id: row.get("id"),
            value: row.get("value"),
            is_ready: row.get("is_ready"),
        };

        Ok(task)
    }

    async fn delete(&self, id: i32) -> Result<(), StorageError> {
        let query = format!("DELETE FROM {} WHERE id = $1", self.table_name);
        let result = sqlx::query(&query).bind(id).execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::RecordNotFound { id });
        }

        Ok(())
    }

    async fn update(
        &self,
        id: i32,
        value: String,
    ) -> Result<crate::model::task::Task, StorageError> {
        let query = format!(
            "UPDATE {} SET value = $1 WHERE id = $2 RETURNING id, value, is_ready",
            self.table_name
        );
        let row = sqlx::query(&query)
            .bind(value)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(StorageError::RecordNotFound { id })?;

        let task = Task {
            id: row.get("id"),
            value: row.get("value"),
            is_ready: row.get("is_ready"),
        };

        Ok(task)
    }

    async fn mark_ready_or_not(
        &self,
        id: i32,
        is_ready: bool,
    ) -> Result<crate::model::task::Task, StorageError> {
        let query = format!(
            "UPDATE {} SET is_ready = $1 WHERE id = $2 RETURNING id, value, is_ready",
            self.table_name
        );
        let row = sqlx::query(&query)
            .bind(is_ready)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(StorageError::RecordNotFound { id })?;

        let task = Task {
            id: row.get("id"),
            value: row.get("value"),
            is_ready: row.get("is_ready"),
        };

        Ok(task)
    }
}
