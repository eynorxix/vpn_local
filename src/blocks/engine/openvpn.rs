use anyhow::{bail, Result};
use std::fs;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

pub fn has_openvpn() -> bool {
    Command::new("which")
        .arg("openvpn")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn up(conf_path: &Path, management_socket: &Path) -> Result<String> {
    if !has_openvpn() {
        bail!("openvpn no encontrado. Instala openvpn.");
    }
    fs::create_dir_all("/tmp/vpnlocal")?;
    let management_user = std::env::var("SUDO_USER").or_else(|_| std::env::var("USER"))?;

    let out = Command::new("sudo")
        .arg("-n")
        .arg("openvpn")
        .arg("--config")
        .arg(conf_path)
        .arg("--redirect-gateway")
        .arg("def1")
        .arg("--management")
        .arg(management_socket)
        .arg("unix")
        .arg("--management-client-user")
        .arg(management_user)
        .arg("--daemon")
        .arg("vpnlocal")
        .arg("--verb")
        .arg("3")
        .arg("--writepid")
        .arg("/tmp/vpnlocal/openvpn.pid")
        .output()?;
    let s = String::from_utf8_lossy(&out.stdout).to_string()
        + &String::from_utf8_lossy(&out.stderr).to_string();
    if !out.status.success() {
        if s.contains("a password is required") || s.contains("terminal is required") {
            bail!("Se requiere contraseña para openvpn.\n\nConfigura passwordless:\n  sudo visudo -f /etc/sudoers.d/vpnlocal\n\nAñade:\n  {} ALL=(ALL) NOPASSWD: /usr/bin/openvpn\n", std::env::var("USER").unwrap_or_default());
        }
        bail!("openvpn fallo:\n{}", s);
    }
    Ok("Conectando con OpenVPN...".to_string())
}

pub fn down(conf_path: &Path, management_socket: Option<&Path>) -> Result<String> {
    if !has_openvpn() {
        bail!("openvpn no encontrado.");
    }
    if let Some(socket) = management_socket {
        if socket.exists() {
            stop_via_management(socket)?;
            return Ok("Desconectado".to_string());
        }

        let deadline = Instant::now() + Duration::from_secs(3);
        while Instant::now() < deadline {
            if socket.exists() {
                stop_via_management(socket)?;
                return Ok("Desconectado".to_string());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    stop_legacy_process(conf_path)?;
    Ok("Desconectado".to_string())
}

pub fn is_running(management_socket: &Path) -> bool {
    UnixStream::connect(management_socket).is_ok()
}

fn stop_via_management(socket: &Path) -> Result<()> {
    let mut stream = UnixStream::connect(socket)?;
    stream.write_all(b"signal SIGTERM\n")?;

    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if UnixStream::connect(socket).is_err() {
            match fs::remove_file(socket) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    bail!("OpenVPN recibió la orden de desconexión, pero sigue activo.");
}

fn stop_legacy_process(conf_path: &Path) -> Result<()> {
    let pid_file = Path::new("/tmp/vpnlocal/openvpn.pid");
    if !pid_file.exists() {
        return Ok(());
    }
    let pid = fs::read_to_string(pid_file)?.trim().parse::<u32>()?;
    let cmdline = match fs::read(format!("/proc/{pid}/cmdline")) {
        Ok(cmdline) => cmdline,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::remove_file(pid_file)?;
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let cmdline = String::from_utf8_lossy(&cmdline);
    if !cmdline.contains(&conf_path.to_string_lossy().to_string()) {
        bail!("El PID guardado no corresponde de forma segura a esta sesión OpenVPN.");
    }

    let out = Command::new("sudo")
        .args(["-n", "/usr/bin/kill", &pid.to_string()])
        .output()?;
    if !out.status.success() {
        let message = String::from_utf8_lossy(&out.stderr);
        bail!(
            "No se pudo detener la sesión antigua (PID {pid}): {}. Autoriza /usr/bin/kill en sudoers o detenla manualmente.",
            message.trim()
        );
    }

    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if !Path::new(&format!("/proc/{pid}")).exists() {
            match fs::remove_file(pid_file) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    bail!("OpenVPN (PID {pid}) sigue activo después de la orden de desconexión.");
}
