#!/usr/bin/env bash
set -e
echo "[vpnlocal] Compilando release..."
cargo build --release
BIN="$HOME/.local/bin"
mkdir -p "$BIN"
cp target/release/vpnlocal "$BIN/vpnlocal"
chmod +x "$BIN/vpnlocal"
echo "[vpnlocal] Instalado en: $BIN/vpnlocal"
echo "[vpnlocal] Asegúrate de tener $BIN en tu PATH"
echo ""
echo "[vpnlocal] Siguiente paso:"
echo "  vpnlocal connect    # conecta solo VPNLocal"
echo "  vpnlocal status"
echo "  vpnlocal disconnect # desconecta solo VPNLocal"
