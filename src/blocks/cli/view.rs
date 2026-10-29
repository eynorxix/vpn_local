use crate::blocks::history::model::HistoryEntry;
use crate::core::time::{hours_left, is_expired};

pub fn print_status(
    ip: Option<String>,
    traffic: String,
    active: bool,
    cfg: Option<String>,
    proton_status: String,
    history: &[HistoryEntry],
) {
    println!("=== VPNLOCAL STATUS ===");
    println!("ACTIVO: {}", if active { "SI" } else { "NO" });
    println!("PROTONVPN: {}", proton_status);
    if let Some(c) = cfg {
        println!("CONFIG: {}", c);
    }
    println!(
        "IP PUBLICA: {}",
        ip.unwrap_or_else(|| "DESCONOCIDO".to_string())
    );
    println!("TRAFICO: {}", traffic);
    println!();
    println!("=== HISTORIAL USADO ===");
    if history.is_empty() {
        println!("(vacío)");
        return;
    }
    for (i, e) in history.iter().take(15).enumerate() {
        let tag = if is_expired(e.expires_at) {
            "LIBRE DE NUEVO"
        } else {
            "YA USADA - CAMBIA DE VPN"
        };
        let h = hours_left(e.expires_at);
        println!(
            "{}. [{}] {} | ip:{:?} | usado:{} | vence en ~{}h",
            i + 1,
            tag,
            e.config_name,
            e.ip,
            e.used_at.format("%Y-%m-%d %H:%M:%S UTC"),
            h
        );
    }
}
