use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostgresStorageConfig {
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub table: String,
}
