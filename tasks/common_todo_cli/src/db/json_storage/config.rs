use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonStorageConfig {
    pub file_path: String,
}
