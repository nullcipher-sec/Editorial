@"
#!/usr/bin/env bash
set -euo pipefail
cd "`$(dirname "`$0")/.."

echo "[dev-up] Verificando .env..."
if [ ! -f deploy/.env ]; then echo "ERROR: falta deploy/.env."; exit 1; fi

echo "[dev-up] Levantando stack..."
cd deploy
docker compose up -d --build
"@ | Out-File -FilePath scripts/dev-up.sh -Encoding utf8