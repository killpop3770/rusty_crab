use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub value: String,
    pub is_ready: bool,
}

impl Task {
    pub fn new(id: i32, value: String) -> Self {
        Self {
            id,
            value,
            is_ready: false,
        }
    }
}
