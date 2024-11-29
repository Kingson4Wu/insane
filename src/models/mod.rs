use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthCheck {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}
