use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MongodbStorageConfig {
    pub port: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub collection: String,
}
