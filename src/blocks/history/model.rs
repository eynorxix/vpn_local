use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub ip: Option<String>,
    pub config_name: String,
    pub config_path: String,
    pub used_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}
