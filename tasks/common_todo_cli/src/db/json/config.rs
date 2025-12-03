use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct JsonStorageConfig {
    pub file_path: String,
}
