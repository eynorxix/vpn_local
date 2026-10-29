# 2. Integracion

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Pool De Configs VPNGate
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Archivos implicados

| Archivo | Cambio |
|---|---|
| `src/blocks/pool/loader.rs` | Modificado para priorizar `vpns/vpngate-configs/` con `.ovpn`. Añade configs del pool sin duplicados. |
| `src/blocks/pool/selector.rs` | Modificado para usar todas las configs si todas están en cooldown (evita fallo total) |
| `src/app/runner.rs` | Actualizado para usar `openvpn` en lugar de `wireguard` |

---

## Lógica de carga

1. Buscar primero en `/home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/vpns/vpngate-configs/` archivos `.ovpn`
2. Si no encuentra, buscar en `/home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/vpns/`
3. Añadir archivos `.ovpn` desde `~/.config/vpnlocal/pool/` si existen (sin duplicados por nombre)
4. Ordenar por nombre

---

## Notas y pendientes

- Los archivos `.ovpn` de VPNGate incluyen configuración para OpenVPN (remote, proto, cipher, etc.)
