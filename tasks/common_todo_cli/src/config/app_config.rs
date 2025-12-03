use config::{Config, File};
use serde::Deserialize;

#[cfg(feature = "json")]
use crate::db::json::config::JsonStorageConfig;

#[cfg(feature = "mongodb")]
use crate::db::mongodb::config::MongodbStorageConfig;

#[cfg(feature = "postgres")]
use crate::db::postgres::config::PostgresStorageConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[cfg(feature = "json")]
    pub json_storage: Option<JsonStorageConfig>,

    #[cfg(feature = "mongodb")]
    pub mongodb: Option<MongodbStorageConfig>,

    #[cfg(feature = "postgres")]
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
