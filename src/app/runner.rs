use crate::blocks::{
    cli::{
        commands::{Cli, Commands},
        view,
    },
    engine::{
        openvpn,
        session::{Session, SessionStore},
    },
    history::{repo::HistoryRepo, service::HistoryService},
    net::{
        ip::{get_public_ip, wait_for_public_ip_change},
        traffic::{get_traffic, human},
    },
    pool::{loader::PoolLoader, selector::Selector},
};
use crate::core::config::Paths;
use anyhow::{bail, Context, Result};
use clap::Parser;
use std::{path::Path, process::Command, time::Duration};

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let paths = Paths::new()?;
    let history_repo = HistoryRepo::new(paths.clone());
    let history_service = HistoryService::new(history_repo.clone());
    let session_store = SessionStore::new(paths.clone());
    let mut session = session_store.load()?;

    match cli.command {
        Commands::Connect | Commands::Activate => {
            if session.active && session.provider.as_deref() == Some("VPNLocal") {
                let socket = session.management_socket.as_deref().map(Path::new);
                if socket.is_some_and(openvpn::is_running) {
                    bail!(
                        "VPNLocal ya está activa ({}). Usa 'vpnlocal disconnect' antes de conectar otra.",
                        session.config_name.as_deref().unwrap_or("desconocida")
                    );
                }
                session_store.clear()?;
                session = Session::default();
            }

            if session.active && session.provider.as_deref() == Some("ProtonVPN") {
                session_store.clear()?;
            }

            let pool = PoolLoader::new(paths.clone()).load()?;
            if pool.is_empty() {
                bail!("Sin configs disponibles. Asegúrate de tener archivos .ovpn en ~/.config/vpnlocal/pool/ o en el directorio vpngate-configs.");
            }
            let previous_ip =
                get_public_ip().context("No se pudo obtener la IP pública antes de conectar")?;
            let chosen = Selector::new(&history_service).pick(&pool, &history_repo.load()?)?;
            let socket = paths.base.join(format!("{}.sock", uuid::Uuid::new_v4()));
            println!("[VPNLOCAL] Conectando -> {}", chosen.name);
            openvpn::up(&chosen.path, &socket)?;

            let mut session = Session {
                active: false,
                config_name: Some(chosen.name.clone()),
                config_path: Some(chosen.path.to_string_lossy().to_string()),
                started_at: Some(chrono::Utc::now().to_rfc3339()),
                management_socket: Some(socket.to_string_lossy().to_string()),
                provider: Some("VPNLocal".to_string()),
                original_ip: Some(previous_ip.clone()),
                public_ip: None,
            };
            session_store.save(&session)?;
            println!("[VPNLOCAL] Verificando el túnel (máximo 30 segundos)...");
            let ip = match wait_for_public_ip_change(&previous_ip, Duration::from_secs(30)) {
                Ok(ip) => ip,
                Err(error) => {
                    if let Err(stop_error) = openvpn::down(&chosen.path, Some(&socket)) {
                        bail!(
                            "No se verificó el túnel: {error:#}. También falló la desconexión: {stop_error:#}. La sesión queda registrada."
                        );
                    }
                    session_store.clear()?;
                    bail!("VPNLocal no pudo verificar una IP distinta: {error:#}. Se desconectó.");
                }
            };
            session.active = true;
            session.public_ip = Some(ip.clone());
            session_store.save(&session)?;

            let mut history = history_repo.load()?;
            history_service.record(
                &mut history,
                Some(ip.clone()),
                chosen.name.clone(),
                chosen.path.to_string_lossy().to_string(),
                None,
            );
            history_repo.save(&history)?;
            println!("[VPNLOCAL] CONECTADA | IP: {ip}");
            Ok(())
        }
        Commands::Disconnect | Commands::Stop => {
            if session.provider.as_deref() == Some("ProtonVPN") {
                session_store.clear()?;
            } else if session.active {
                if let (Some(config), Some(socket)) = (
                    session.config_path.as_deref(),
                    session.management_socket.as_deref(),
                ) {
                    openvpn::down(Path::new(config), Some(Path::new(socket)))?;
                }
                session_store.clear()?;
            } else {
                bail!("No hay sesión VPNLocal activa. ProtonVPN no fue modificado.");
            }
            println!("[VPNLOCAL] Desconectada; ProtonVPN no fue modificado");
            Ok(())
        }
        Commands::Status => {
            session = session_store.load()?;
            if session.provider.as_deref() == Some("ProtonVPN") {
                session_store.clear()?;
                session = Session::default();
            }
            if session.active
                && session.provider.as_deref() == Some("VPNLocal")
                && session
                    .management_socket
                    .as_deref()
                    .is_some_and(|socket| !openvpn::is_running(Path::new(socket)))
            {
                session_store.clear()?;
                session = Session::default();
            }
            let ip = get_public_ip().ok();
            let traffic = get_traffic().unwrap_or_default();
            let traffic = if traffic.is_empty() {
                "N/A".to_string()
            } else {
                traffic
                    .iter()
                    .map(|item| {
                        format!(
                            "{} RX:{} TX:{}",
                            item.iface,
                            human(item.rx_bytes),
                            human(item.tx_bytes)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" | ")
            };
            let history = history_service.list_recent(&history_repo.load()?);
            view::print_status(
                ip,
                traffic,
                session.active && session.provider.as_deref() == Some("VPNLocal"),
                session.config_name,
                proton_status(),
                &history,
            );
            Ok(())
        }
    }
}

fn proton_status() -> String {
    match Command::new("protonvpn").arg("status").output() {
        Ok(output) => {
            let message = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            if output.status.success() && message.contains("Status: Connected") {
                "CONECTADA".to_string()
            } else if output.status.success() {
                "DESCONECTADA".to_string()
            } else {
                "DESCONOCIDO".to_string()
            }
        }
        Err(_) => "NO DISPONIBLE".to_string(),
    }
}
