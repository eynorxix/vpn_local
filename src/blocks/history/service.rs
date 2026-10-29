use crate::blocks::history::{
    model::{History, HistoryEntry},
    repo::HistoryRepo,
};
use crate::core::time::{add_days, is_expired};
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct HistoryService {
    repo: HistoryRepo,
}

impl HistoryService {
    pub fn new(repo: HistoryRepo) -> Self {
        Self { repo }
    }

    pub fn load(&self) -> Result<History> {
        self.repo.load()
    }

    pub fn save(&self, h: &History) -> Result<()> {
        self.repo.save(h)
    }

    pub fn is_blocked(&self, config_name: &str, history: &History) -> bool {
        history
            .entries
            .iter()
            .any(|e| e.config_name == config_name && !is_expired(e.expires_at))
    }

    pub fn record(
        &self,
        history: &mut History,
        ip: Option<String>,
        config_name: String,
        config_path: String,
        _session_id: Option<String>,
    ) {
        let entry = HistoryEntry {
            id: Uuid::new_v4().to_string(),
            ip,
            config_name,
            config_path,
            used_at: Utc::now(),
            expires_at: add_days(3),
            session_id: None,
        };
        history.entries.push(entry);
    }

    pub fn list_recent(&self, history: &History) -> Vec<HistoryEntry> {
        let mut v: Vec<HistoryEntry> = history.entries.clone();
        v.sort_by(|a, b| b.used_at.cmp(&a.used_at));
        v
    }
}
