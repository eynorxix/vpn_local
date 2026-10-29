# 2. Integracion

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Conexion OpenVPN
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Archivos implicados

| Archivo | Cambio/Propósito |
|---|---|
| `src/blocks/engine/openvpn.rs` | Nuevo módulo - `up()` / `down()` con OpenVPN |
| `src/blocks/engine/mod.rs` | Añadido `pub mod openvpn;` |
| `src/app/runner.rs` | Usa `openvpn` en connect/disconnect en lugar de `wireguard` |
| `Cargo.toml` | Sin cambios necesarios (usa std::process::Command) |

---

## Flujo de integración

1. `runner::run()` llama a `openvpn::up(&chosen.path)`
2. `openvpn` lanza proceso daemon con `sudo -n`
3. Tras sleep + verificación, registra en historial
4. `disconnect` llama a `openvpn::down()`

---

## Notas y pendientes

- PID guardado en `/tmp/vpnlocal/openvpn.pid`
- Limpieza con `kill` y `pkill` como respaldo
