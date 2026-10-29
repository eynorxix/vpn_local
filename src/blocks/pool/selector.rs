use crate::blocks::history::model::History;
use crate::blocks::{history::service::HistoryService, pool::model::VpnConfig};
use anyhow::{bail, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Selector<'a> {
    history: &'a HistoryService,
}

impl<'a> Selector<'a> {
    pub fn new(history: &'a HistoryService) -> Self {
        Self { history }
    }

    pub fn pick(&self, pool: &[VpnConfig], history: &History) -> Result<VpnConfig> {
        let mut candidates: Vec<VpnConfig> = pool
            .iter()
            .cloned()
            .filter(|c| !self.history.is_blocked(&c.name, history))
            .collect();
        if candidates.is_empty() {
            // Si todas están bloqueadas, usar cualquiera de todas formas
            candidates = pool.iter().cloned().collect();
            if candidates.is_empty() {
                bail!("No hay VPNs disponibles. Añade más configs a ~/.config/vpnlocal/pool/");
            }
        }
        let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos() as usize;
        let idx = seed % candidates.len();
        Ok(candidates[idx].clone())
    }
}
