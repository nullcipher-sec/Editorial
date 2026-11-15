#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

echo "[dev-up] Verificando .env..."
if [ ! -f .env ]; then echo "ERROR: falta deploy/.env."; exit 1; fi

echo "[dev-up] Levantando stack..."
docker compose up -d --build
