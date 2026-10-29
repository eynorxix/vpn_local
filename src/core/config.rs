use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Paths {
    pub base: PathBuf,
    pub pool: PathBuf,
    pub history: PathBuf,
    pub session: PathBuf,
    pub log: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().expect("No se encontró home dir");
        let base = home.join(".config").join("vpnlocal");
        let pool = base.join("pool");
        let history = base.join("history.json");
        let session = base.join("session.json");
        let log = base.join("vpnlocal.log");

        fs::create_dir_all(&base)?;
        fs::create_dir_all(&pool)?;

        Ok(Self {
            base,
            pool,
            history,
            session,
            log,
        })
    }
}
