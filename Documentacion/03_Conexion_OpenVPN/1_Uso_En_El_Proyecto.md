# 1. Uso En El Proyecto

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Conexion OpenVPN
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Técnica / Método

**Conexión mediante `openvpn` en modo daemon con permisos vía `sudo` NOPASSWD.**  
Se ejecuta `openvpn` con el archivo `.ovpn` seleccionado, en background, guardando PID para poder desconectar limpiamente.

---

## ¿Para qué sirve?

Permite establecer la conexión VPN con los servidores reales de VPNGate usando los archivos `.ovpn` tal como vienen.

---

## Requisitos

- `openvpn` instalado (`/usr/bin/openvpn`)
- Regla en `/etc/sudoers.d/vpnlocal`: `eynor ALL=(ALL) NOPASSWD: /usr/bin/openvpn`
- Permisos correctos del archivo sudoers (0440)

---

## Notas y pendientes

- Tras conectar se espera ~8s y se verifica conectividad + IP pública
- Si falla la verificación, se desconecta automáticamente
