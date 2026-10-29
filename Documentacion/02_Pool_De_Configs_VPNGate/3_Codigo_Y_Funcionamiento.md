# 3. Codigo Y Funcionamiento

**Proyecto:** vpnlocal (9_vpn_Libre)
**Sistema:** Pool De Configs VPNGate
**Fecha de documentación:** 2026-10-25
**Autor:** opencode
**Ruta raíz:** /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre

---

## Función `PoolLoader::load()` (modificada)

Archivo: `src/blocks/pool/loader.rs`

```rust
// Priorizar siempre vpngate-configs (.ovpn) reales
let candidates = vec![
    PathBuf::from("/home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/vpns/vpngate-configs"),
    PathBuf::from("/home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/vpns"),
];
// ... cargar .ovpn
// También añadir ovpn del pool si existen (sin duplicados)
```

---

## Función `Selector::pick()` (modificada)

Archivo: `src/blocks/pool/selector.rs`

Si todos los candidatos están bloqueados por historial/cooldown, ahora vuelve a usar todas las configs del pool en lugar de fallar. Esto permite seguir conectando con los `.ovpn` reales aunque se repitan tras cooldown.

---

## Comprobación

```bash
ls /home/eynor/Documents/2_Proyect_Private/9_vpn_Libre/vpns/vpngate-configs/*.ovpn | wc -l
```

Hay múltiples archivos `.ovpn` reales disponibles para selección aleatoria.

---

## Notas y pendientes

- El nombre de cada config viene del `file_stem()` del archivo `.ovpn`
