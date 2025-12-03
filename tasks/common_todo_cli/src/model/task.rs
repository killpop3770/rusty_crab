use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub value: String,
    pub is_ready: bool,
}

impl Task {
    pub fn new(id: String, value: String) -> Self {
        Self {
            id,
            value,
            is_ready: false,
        }
    }
}
