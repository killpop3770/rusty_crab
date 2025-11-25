use config::{Config, File};
use serde::Deserialize;

use crate::db::{json_storage::config::JsonStorageConfig, postgres::config::PostgresStorageConfig};

#[derive(Deserialize)]
pub struct AppConfig {
    pub json_storage: Option<JsonStorageConfig>,
    pub postgres: Option<PostgresStorageConfig>,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let env = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // // Базовая конфигурация
            // .add_source(File::with_name("config/default"))
            // Конфигурация для окружения
            .add_source(File::with_name(&format!("config/{env}")).required(false))
            .build()?;

        config.try_deserialize()
    }
}
