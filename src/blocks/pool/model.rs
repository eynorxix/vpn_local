#[derive(Debug, Clone)]
pub struct VpnConfig {
    pub name: String,
    pub path: std::path::PathBuf,
}
