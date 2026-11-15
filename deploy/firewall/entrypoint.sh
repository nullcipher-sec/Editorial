#!/bin/sh
set -e
echo "[FW-E] Cargando reglas nftables..."
nft -f /etc/nftables/rules.nft
echo "[FW-E] Reglas cargadas. Política por defecto: DROP."
exec tail -f /dev/null
