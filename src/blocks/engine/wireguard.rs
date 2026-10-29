use anyhow::{bail, Result};
use std::path::Path;
use std::process::Command;

pub fn has_wg_quick() -> bool {
    Command::new("which")
        .arg("wg-quick")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn up(conf_path: &Path) -> Result<String> {
    if !has_wg_quick() {
        bail!("wg-quick no encontrado. Instala wireguard-tools.");
    }
    let out = Command::new("sudo")
        .arg("-n")
        .arg("wg-quick")
        .arg("up")
        .arg(conf_path)
        .output()?;
    let s = String::from_utf8_lossy(&out.stdout).to_string()
        + &String::from_utf8_lossy(&out.stderr).to_string();
    if !out.status.success() {
        if s.contains("a password is required") || s.contains("terminal is required") {
            bail!("Se requiere contraseña para wg-quick.\n\nConfigura passwordless:\n  sudo visudo -f /etc/sudoers.d/vpnlocal\n\nAñade:\n  {} ALL=(ALL) NOPASSWD: /usr/bin/wg-quick\n", std::env::var("USER").unwrap_or_default());
        }
        bail!("wg-quick up falló:\n{}", s);
    }
    Ok(s)
}

pub fn down(conf_path: &Path) -> Result<String> {
    if !has_wg_quick() {
        bail!("wg-quick no encontrado.");
    }
    let out = Command::new("sudo")
        .arg("-n")
        .arg("wg-quick")
        .arg("down")
        .arg(conf_path)
        .output()?;
    let s = String::from_utf8_lossy(&out.stdout).to_string()
        + &String::from_utf8_lossy(&out.stderr).to_string();
    if !out.status.success() {
        if s.contains("a password is required") || s.contains("terminal is required") {
            bail!("Se requiere contraseña para wg-quick.\nConfigura NOPASSWD para /usr/bin/wg-quick en sudoers.");
        }
        // si ya está down, no es fatal
    }
    Ok(s)
}
