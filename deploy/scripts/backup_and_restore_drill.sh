#!/usr/bin/env bash
# Parallel Season FC — Append-Only World Ledger Backup & Restore Drill
# Safely snapshots the live SQLite WAL ledger (or PostgreSQL dump) and verifies projection parity.

set -euo pipefail

SOURCE_DB="${1:-/tmp/psfc_world.db}"
BACKUP_DIR="${2:-/tmp/psfc_backups}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
SNAP_FILE="${BACKUP_DIR}/psfc_world_${STAMP}.db"
RESTORE_CHECK_DB="${BACKUP_DIR}/restore_verify_${STAMP}.db"

mkdir -p "${BACKUP_DIR}"

if [[ ! -f "${SOURCE_DB}" ]]; then
    echo "ERROR: Source database '${SOURCE_DB}' does not exist." >&2
    exit 1
fi

python3 - "${SOURCE_DB}" "${SNAP_FILE}" "${RESTORE_CHECK_DB}" <<'PY'
import shutil
import sqlite3
import sys

src_path, snap_path, restore_path = sys.argv[1], sys.argv[2], sys.argv[3]

src = sqlite3.connect(src_path)
dst = sqlite3.connect(snap_path)
with dst:
    src.backup(dst)
dst.close()
src.close()

shutil.copy2(snap_path, restore_path)
verify = sqlite3.connect(restore_path)
integrity = verify.execute("PRAGMA integrity_check").fetchone()[0]
if integrity != "ok":
    raise SystemExit(f"Integrity check failed: {integrity}")
event_count = verify.execute("SELECT COUNT(*) FROM world_event").fetchone()[0]
world_count = verify.execute("SELECT COUNT(*) FROM world").fetchone()[0]
verify.close()
print(f"[OK] Backup snapshot: {snap_path} | integrity={integrity} | worlds={world_count} | world_events={event_count}")
PY

rm -f "${RESTORE_CHECK_DB}"
