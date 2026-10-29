# 1. Uso En El Proyecto

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Arquitectura Y Flujo General
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Técnica / Método

**Arquitectura modular en Rust + CLI con `clap` + integración con `openvpn`/`wg-quick`.**  
El binario `vpnlocal` orquesta la carga de configuraciones VPN, selección aleatoria con historial/cooldown, establecimiento de la conexión y verificación de conectividad.

---

## ¿Para qué sirve?

Permite conectar a servidores VPN de forma automatizada y aleatoria. En la configuración actual, **usa archivos `.ovpn` reales de VPNGate** ubicados en `vpns/vpngate-configs/` sin necesidad de generar configs ficticias.

---

## Comandos disponibles

| Comando | Acción |
|---|---|
| `vpnlocal connect` / `activate` | Conecta usando una config aleatoria |
| `vpnlocal disconnect` / `stop` | Desconecta la sesión activa |
| `vpnlocal status` | Muestra estado, IP pública, tráfico e historial |

---

## Notas y pendientes

- El proyecto ahora prioriza OpenVPN con archivos `.ovpn` reales.
