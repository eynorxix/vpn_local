use anyhow::Result;
use std::fs;

#[derive(Debug, Clone)]
pub struct Traffic {
    pub iface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

pub fn get_traffic() -> Result<Vec<Traffic>> {
    let content = fs::read_to_string("/proc/net/dev")?;
    let mut out = Vec::new();
    for line in content.lines().skip(2) {
        if let Some((iface, rest)) = line.split_once(':') {
            let iface = iface.trim().to_string();
            let mut parts = rest.split_whitespace();
            let rx: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            for _ in 0..7 {
                let _ = parts.next();
            }
            let tx: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            if iface == "lo" {
                continue;
            }
            if rx == 0 && tx == 0 {
                continue;
            }
            out.push(Traffic {
                iface,
                rx_bytes: rx,
                tx_bytes: tx,
            });
        }
    }
    Ok(out)
}

pub fn human(b: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let v = b as f64;
    if v >= GB {
        format!("{:.2} GB", v / GB)
    } else if v >= MB {
        format!("{:.2} MB", v / MB)
    } else if v >= KB {
        format!("{:.2} KB", v / KB)
    } else {
        format!("{} B", b)
    }
}
