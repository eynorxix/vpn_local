use crate::blocks::history::model::History;
use crate::core::{config::Paths, fs};
use anyhow::Result;

#[derive(Clone)]
pub struct HistoryRepo {
    paths: Paths,
}

impl HistoryRepo {
    pub fn new(paths: Paths) -> Self {
        Self { paths }
    }

    pub fn load(&self) -> Result<History> {
        if !self.paths.history.exists() {
            return Ok(History::default());
        }
        Ok(fs::read_json(&self.paths.history)?)
    }

    pub fn save(&self, history: &History) -> Result<()> {
        fs::write_json(&self.paths.history, history)
    }
}
