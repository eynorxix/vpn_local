use crate::core::{config::Paths, fs};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Session {
    pub active: bool,
    pub config_name: Option<String>,
    pub config_path: Option<String>,
    pub started_at: Option<String>,
    #[serde(default)]
    pub management_socket: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub original_ip: Option<String>,
    #[serde(default)]
    pub public_ip: Option<String>,
}

#[derive(Clone)]
pub struct SessionStore {
    paths: Paths,
}

impl SessionStore {
    pub fn new(paths: Paths) -> Self {
        Self { paths }
    }

    pub fn load(&self) -> Result<Session> {
        if !self.paths.session.exists() {
            return Ok(Session::default());
        }
        Ok(fs::read_json(&self.paths.session)?)
    }

    pub fn save(&self, s: &Session) -> Result<()> {
        fs::write_json(&self.paths.session, s)
    }

    pub fn clear(&self) -> Result<()> {
        self.save(&Session::default())
    }
}
