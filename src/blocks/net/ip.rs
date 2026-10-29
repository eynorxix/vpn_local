use anyhow::Result;
use std::time::{Duration, Instant};

pub fn get_public_ip() -> Result<String> {
    let agents = [
        "https://api4.ipify.org",
        "https://ipv4.icanhazip.com",
        "https://v4.ident.me",
    ];
    for a in agents {
        if let Ok(resp) = ureq::get(a)
            .timeout(std::time::Duration::from_secs(5))
            .call()
        {
            if resp.status() == 200 {
                if let Ok(s) = resp.into_string() {
                    let ip = s.trim().to_string();
                    if !ip.is_empty() {
                        return Ok(ip);
                    }
                }
            }
        }
    }
    anyhow::bail!("No se pudo obtener IP pública")
}

pub fn wait_for_public_ip_change(previous: &str, timeout: Duration) -> Result<String> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if let Ok(ip) = get_public_ip() {
            if ip != previous {
                return Ok(ip);
            }
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    anyhow::bail!(
        "No se verificó una IP pública IPv4 distinta de {previous} en {} segundos",
        timeout.as_secs()
    )
}
