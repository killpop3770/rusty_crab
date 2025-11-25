use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonStorageConfig {
    file_path: String,
}
