# 3. Codigo Y Funcionamiento

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Arquitectura Y Flujo General
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Flujo completo (connect/activate)

1. **Parseo CLI** (`src/main.rs` -> `app::runner::run()`) con `clap`
2. **Inicialización**: `Paths`, `HistoryRepo/Service`, `SessionStore`, `PoolLoader`, `Selector`
3. **Carga pool** (`PoolLoader::load()`): busca `.ovpn` en `vpns/vpngate-configs/` (prioritario). Si hay `.ovpn` en `~/.config/vpnlocal/pool/` se añaden sin duplicados
4. **Selección** (`Selector::pick()`): filtra por cooldown/historial, si vacío usa todos, elige aleatorio por seed nano
5. **Conexión** (`openvpn::up()`): ejecuta `sudo -n openvpn --config <file> --daemon vpnlocal --writepid /tmp/vpnlocal/openvpn.pid`
6. **Espera**: `thread::sleep(Duration::from_secs(8))`
7. **Verificación**: `wait_for_internet(15)`. Si falla -> `openvpn::down()` y limpia sesión
8. **Registro**: guarda IP pública, nombre de config y path en historial
9. **Estado**: marca sesión activa

## Desconexión

1. Verifica sesión activa
2. `openvpn::down()`: mata por PID en `/tmp/vpnlocal/openvpn.pid`, pkill alternativo
3. Limpia `session.json`

## Estado

`vpnlocal status` muestra IP pública, tráfico por interfaz, sesión activa/config y últimos registros del historial.

---

## Archivos relevantes

| Ruta | Notas |
|---|---|
| `src/app/runner.rs:1-160` | Flujo principal |
| `src/blocks/pool/loader.rs:1-90` | Lógica de carga (prioriza vpngate-configs) |
| `src/blocks/engine/openvpn.rs:1-60` | Implementación OpenVPN actual |

---

## Notas y pendientes

- Actualmente usa `sudo -n openvpn`. Requiere regla NOPASSWD en sudoers para evitar prompt.
- Los `.ovpn` de VPNGate son funcionales con OpenVPN.
