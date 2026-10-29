# 2. Integracion

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Arquitectura Y Flujo General
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Estructura de módulos

```
src/
├── main.rs                -> Punto de entrada
├── app/
│   ├── mod.rs
│   └── runner.rs          -> Orquestación CLI (match Commands)
├── blocks/
│   ├── cli/               -> Clap CLI (commands.rs, view.rs)
│   ├── engine/            -> Conexión (openvpn.rs, wireguard.rs, session.rs)
│   ├── history/           -> Historial/cooldown
│   ├── net/               -> IP pública, tráfico, conectividad
│   └── pool/              -> Carga/selección de configs
└── core/
    └── config.rs          -> Paths (~/.config/vpnlocal/)
```

---

## Archivos clave implicados

| Archivo | Propósito |
|---|---|
| `src/main.rs` | Llama a `app::runner::run()` |
| `src/app/runner.rs` | Flujo completo connect/disconnect/status |
| `src/blocks/pool/loader.rs` | Carga configs desde `vpns/vpngate-configs/` o `~/.config/vpnlocal/pool/` |
| `src/blocks/pool/selector.rs` | Selección aleatoria con historial |
| `src/blocks/engine/openvpn.rs` | Levanta/para OpenVPN vía `sudo openvpn` |
| `src/core/config.rs` | Define rutas base/pool/history/session |

---

## Integración con sistema

- Usa `sudo` con regla NOPASSWD para `/usr/bin/openvpn` (`/etc/sudoers.d/vpnlocal`)
- Guarda sesión en `~/.config/vpnlocal/session.json`
- Guarda historial en `~/.config/vpnlocal/history.json`
- Verifica internet tras conexión (`wait_for_internet`) y IP pública

---

## Notas y pendientes

- `wireguard.rs` queda como legacy (no usado tras el cambio a OpenVPN)
