# 1. Uso En El Proyecto

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Pool De Configs VPNGate
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Técnica / Método

**Uso directo de archivos `.ovpn` reales obtenidos de VPNGate (vpngate.net).**  
En lugar de generar configuraciones sintéticas, el proyecto carga y usa aleatoriamente los archivos `.ovpn` disponibles en `vpns/vpngate-configs/`.

---

## ¿Para qué sirve?

Garantiza que las conexiones usen servidores reales de VPNGate. Esto evita el problema anterior donde las configs generadas no funcionaban por falta de direcciones/credenciales válidas.

---

## Ubicación de los configs

| Ruta | Uso |
|---|---|
| `/home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/vpns/vpngate-configs/` | Fuente principal (archivos `.ovpn` reales) |
| `~/.config/vpnlocal/pool/` | Opcional - configs adicionales del usuario (`.ovpn` soportados) |

---

## Notas y pendientes

- Se movieron los antiguos `.conf` de WireGuard fuera del pool para forzar el uso de `.ovpn`.
