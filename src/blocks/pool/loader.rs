use crate::blocks::pool::model::VpnConfig;
use crate::core::config::Paths;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct PoolLoader {
    paths: Paths,
}

impl PoolLoader {
    pub fn new(paths: Paths) -> Self {
        Self { paths }
    }

    pub fn load(&self) -> Result<Vec<VpnConfig>> {
        let mut out: Vec<VpnConfig> = Vec::new();
        // Priorizar siempre vpngate-configs (.ovpn) reales
        let candidates = vec![
            PathBuf::from(
                "/home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/vpns/vpngate-configs",
            ),
            PathBuf::from("/home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/vpns"),
        ];
        for dir in candidates {
            if dir.exists() {
                for entry in fs::read_dir(&dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("ovpn") {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            out.push(VpnConfig {
                                name: stem.to_string(),
                                path: path.clone(),
                            });
                        }
                    }
                }
                if !out.is_empty() {
                    break;
                }
            }
        }
        // También añadir ovpn del pool si existen
        if self.paths.pool.exists() {
            for entry in fs::read_dir(&self.paths.pool)? {
                let entry = entry?;
                let path = entry.path();
                let ext = path.extension().and_then(|e| e.to_str());
                if ext == Some("ovpn") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        // evitar duplicados por nombre
                        if !out.iter().any(|c| c.name == stem) {
                            out.push(VpnConfig {
                                name: stem.to_string(),
                                path: path.clone(),
                            });
                        }
                    }
                }
            }
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }
}
