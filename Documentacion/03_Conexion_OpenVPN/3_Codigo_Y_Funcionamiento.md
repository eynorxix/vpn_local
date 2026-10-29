# 3. Codigo Y Funcionamiento

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Conexion OpenVPN
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Implementación

**Archivo:** `src/blocks/engine/openvpn.rs`

### `up(conf_path: &Path) -> Result<String>`

```rust
let out = Command::new("sudo")
    .arg("-n")
    .arg("openvpn")
    .arg("--config")
    .arg(conf_path)
    .arg("--daemon")
    .arg("vpnlocal")
    .arg("--verb")
    .arg("3")
    .arg("--writepid")
    .arg("/tmp/vpnlocal/openvpn.pid")
    .output()?;
```

- Verifica `openvpn` con `which`
- Crea `/run/openvpn` y `/tmp/vpnlocal`
- Lanza en daemon con nombre `vpnlocal`
- Si falla con "password required", da instrucciones para configurar sudoers
- Retorna mensaje "Conectando con OpenVPN..."

### `down(_conf_path: &Path) -> Result<String>`

```rust
let _ = Command::new("sudo").arg("-n").arg("sh").arg("-c").arg("kill $(cat /tmp/vpnlocal/openvpn.pid 2>/dev/null) 2>/dev/null").output();
let _ = Command::new("sudo").arg("-n").arg("pkill").arg("-f").arg("openvpn.*vpnlocal").output();
let _ = Command::new("sudo").arg("-n").arg("pkill").arg("-f").arg("openvpn.*vpngate").output();
```

---

## Comprobación de funcionamiento

```bash
# Crear regla sudoers (hecho)
echo "eynor ALL=(ALL) NOPASSWD: /usr/bin/openvpn" | sudo tee /etc/sudoers.d/vpnlocal >/dev/null
sudo chmod 440 /etc/sudoers.d/vpnlocal
sudo visudo -cf /etc/sudoers.d/vpnlocal

# Probar
sudo -n /usr/bin/openvpn --version | head -1
vpnlocal activate  # Conecta aleatoriamente a VPNGate
vpnlocal status
vpnlocal disconnect
```

---

## Notas y pendientes

- Funciona correctamente con los `.ovpn` de VPNGate
- En algunos casos el daemon puede tardar un poco en establecer túnel
