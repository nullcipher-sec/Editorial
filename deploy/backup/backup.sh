#!/usr/bin/env bash
set -euo pipefail

PGHOST="${PGHOST:-10.20.40.10}"
PGPORT="${PGPORT:-5432}"
PGUSER="${POSTGRES_USER:?falta POSTGRES_USER}"
PGPASSWORD="${POSTGRES_PASSWORD:?falta POSTGRES_PASSWORD}"
PGDATABASE="${POSTGRES_DB:?falta POSTGRES_DB}"

RECIPIENT_FILE="${AGE_RECIPIENT_FILE:-/secrets/age.pub}"
RETENTION="${BACKUP_RETENTION_DAYS:-30}"
DEST_DIR="${BACKUP_DIR:-/backups}"

if [ ! -f "$RECIPIENT_FILE" ]; then echo "[backup] ERROR: no existe $RECIPIENT_FILE" >&2; exit 1; fi
RECIPIENT=$(cat "$RECIPIENT_FILE")
if [ -z "$RECIPIENT" ]; then echo "[backup] ERROR: $RECIPIENT_FILE vacío" >&2; exit 1; fi

mkdir -p "$DEST_DIR"
TS=$(date +%Y%m%d-%H%M%S)
OUT="$DEST_DIR/editorial-${TS}.sql.age"

echo "[backup] pg_dump $PGDATABASE -> age..."
export PGPASSWORD
pg_dump -h "$PGHOST" -p "$PGPORT" -U "$PGUSER" -d "$PGDATABASE" --no-owner --no-acl | age -r "$RECIPIENT" -o "$OUT"

echo "[backup] Creado: $OUT ($(du -h "$OUT" | cut -f1))"
DELETED=$(find "$DEST_DIR" -name "editorial-*.sql.age" -mtime +"$RETENTION" -print -delete | wc -l)
echo "[backup] Retención: $RETENTION días, $DELETED archivo(s) borrado(s)."

if [ "${BACKUP_LOOP:-true}" = "true" ]; then echo "[backup] Durmiendo 24h..."; sleep 86400; exec "$0"; fi
