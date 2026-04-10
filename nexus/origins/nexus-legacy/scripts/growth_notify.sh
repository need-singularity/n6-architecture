#!/usr/bin/env bash
set -euo pipefail

# NEXUS-6 Growth Notification
# ============================
# Usage: ./growth_notify.sh "message" [--level info|warn|error|success]
#
# Notification methods (uses whatever is available):
#   1. macOS notification center (osascript)
#   2. Terminal bell
#   3. Log file (~/.nexus/notifications.log)
#   4. Touch file (~/.nexus/last-growth-TIMESTAMP)

NEXUS_HOME="$HOME/.nexus"
NOTIFY_LOG="$NEXUS_HOME/notifications.log"

MESSAGE="${1:-NEXUS-6 growth event}"
shift || true
LEVEL="info"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --level) LEVEL="${2:-info}"; shift 2 ;;
        *) shift ;;
    esac
done

mkdir -p "$NEXUS_HOME"

TIMESTAMP=$(date +%Y-%m-%dT%H:%M:%S)

# ── Map level to display properties ──────────────────────────────────
case "$LEVEL" in
    success) ICON="[OK]";  TITLE="NEXUS-6 Growth" ;;
    warn)    ICON="[!!]";  TITLE="NEXUS-6 Warning" ;;
    error)   ICON="[XX]";  TITLE="NEXUS-6 Error" ;;
    *)       ICON="[--]";  TITLE="NEXUS-6" ;;
esac

# ── 1. Log file (always) ─────────────────────────────────────────────
echo "$TIMESTAMP $ICON $MESSAGE" >> "$NOTIFY_LOG"

# ── 2. macOS notification center ─────────────────────────────────────
if command -v osascript >/dev/null 2>&1; then
    # Only notify for success and error (avoid spam for info)
    if [[ "$LEVEL" == "success" || "$LEVEL" == "error" || "$LEVEL" == "warn" ]]; then
        osascript -e "display notification \"$MESSAGE\" with title \"$TITLE\"" 2>/dev/null || true
    fi
fi

# ── 3. Terminal bell (only for errors) ────────────────────────────────
if [[ "$LEVEL" == "error" ]]; then
    echo -e "\a" 2>/dev/null || true
fi

# ── 4. Touch file (for external tools to detect) ─────────────────────
touch "$NEXUS_HOME/last-growth-$(date +%s)" 2>/dev/null || true

# Cleanup old touch files (keep last 100)
ls -t "$NEXUS_HOME"/last-growth-* 2>/dev/null | tail -n +101 | xargs rm -f 2>/dev/null || true
