#!/usr/bin/env bash
# NEXUS-6 Growth Health Check — detects if daemon is running, starts if not
# Usage: ./health_check.sh [--quiet] [--start-if-dead] [--status-only]
#
# Exit codes:
#   0 = daemon is running
#   1 = daemon is NOT running
#   2 = daemon was dead but auto-started
set -uo pipefail

NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
NEXUS_SCRIPTS="$NEXUS_ROOT/scripts"
NEXUS_STATE="$HOME/.nexus"
DAEMON_SCRIPT="$NEXUS_SCRIPTS/nexus_growth_daemon.sh"
GROWTH_LOG="$NEXUS_STATE/growth_log.jsonl"
SCRIPT_GROWTH_LOG="$NEXUS_SCRIPTS/growth_log.jsonl"
DAEMON_PID_FILE="$NEXUS_STATE/daemon.pid"
LAUNCHD_LABEL="com.nexus.growth"

# ── Parse arguments ──────────────────────────────────────────────────
QUIET=false
START_IF_DEAD=false
STATUS_ONLY=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --quiet)         QUIET=true; shift ;;
        --start-if-dead) START_IF_DEAD=true; shift ;;
        --status-only)   STATUS_ONLY=true; shift ;;
        -h|--help)
            echo "NEXUS-6 Growth Health Check"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --quiet          Suppress output"
            echo "  --start-if-dead  Auto-start daemon if not running"
            echo "  --status-only    Print status and exit"
            echo ""
            echo "Exit codes:"
            echo "  0 = daemon is running"
            echo "  1 = daemon is NOT running"
            echo "  2 = daemon was dead but auto-started"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# ── Ensure state directory ───────────────────────────────────────────
mkdir -p "$NEXUS_STATE/reports"

# ── Check functions ──────────────────────────────────────────────────

check_launchd() {
    if launchctl list 2>/dev/null | grep -q "$LAUNCHD_LABEL"; then
        echo "loaded ($LAUNCHD_LABEL)"
        return 0
    fi
    echo "not loaded"
    return 1
}

check_cron() {
    local count
    count=$(crontab -l 2>/dev/null | grep -c "nexus" || true)
    if [[ "$count" -gt 0 ]]; then
        echo "$count jobs installed"
        return 0
    fi
    echo "no jobs"
    return 1
}

check_process() {
    # Check PID file first
    if [[ -f "$DAEMON_PID_FILE" ]]; then
        local pid
        pid=$(cat "$DAEMON_PID_FILE" 2>/dev/null)
        if [[ -n "$pid" ]] && kill -0 "$pid" 2>/dev/null; then
            echo "PID $pid"
            return 0
        fi
    fi
    # Fallback: search for process
    local pid
    pid=$(pgrep -f "nexus_growth_daemon" 2>/dev/null | head -1 || true)
    if [[ -n "$pid" ]]; then
        echo "PID $pid"
        return 0
    fi
    echo "not running"
    return 1
}

check_last_growth() {
    local log_file=""
    # Check both possible log locations
    for f in "$GROWTH_LOG" "$SCRIPT_GROWTH_LOG"; do
        if [[ -f "$f" ]]; then
            log_file="$f"
            break
        fi
    done

    if [[ -z "$log_file" ]]; then
        echo "no log file"
        return 1
    fi

    local last_mod now age_min
    last_mod=$(stat -f %m "$log_file" 2>/dev/null || echo "0")
    now=$(date +%s)
    age_min=$(( (now - last_mod) / 60 ))

    if [[ $age_min -lt 60 ]]; then
        echo "${age_min} min ago"
        return 0
    fi
    echo "${age_min} min ago (stale)"
    return 1
}

check_growth_log_entries() {
    local log_file=""
    for f in "$GROWTH_LOG" "$SCRIPT_GROWTH_LOG"; do
        if [[ -f "$f" ]]; then
            log_file="$f"
            break
        fi
    done

    if [[ -z "$log_file" ]]; then
        echo "no entries"
        return 1
    fi

    local count
    count=$(wc -l < "$log_file" 2>/dev/null | tr -d ' ')
    if [[ "$count" -gt 0 ]]; then
        echo "$count entries"
        return 0
    fi
    echo "empty"
    return 1
}

# ── Run all checks ───────────────────────────────────────────────────
launchd_status=$(check_launchd)
launchd_ok=$?

cron_status=$(check_cron)
cron_ok=$?

process_status=$(check_process)
process_ok=$?

growth_status=$(check_last_growth)
growth_ok=$?

log_status=$(check_growth_log_entries)
log_ok=$?

# ── Determine overall health ────────────────────────────────────────
# Healthy if: process is running OR recent activity exists
is_healthy=false
if [[ $process_ok -eq 0 ]] || [[ $growth_ok -eq 0 ]]; then
    is_healthy=true
fi

# ── Output ───────────────────────────────────────────────────────────
status_icon() {
    if [[ $1 -eq 0 ]]; then echo "OK"; else echo "--"; fi
}

if ! $QUIET; then
    echo ""
    echo "NEXUS-6 Growth System Health Check"
    echo "-----------------------------------"
    printf "  launchd:     %s %s\n" "$(status_icon $launchd_ok)" "$launchd_status"
    printf "  cron:        %s %s\n" "$(status_icon $cron_ok)" "$cron_status"
    printf "  process:     %s %s\n" "$(status_icon $process_ok)" "$process_status"
    printf "  last growth: %s %s\n" "$(status_icon $growth_ok)" "$growth_status"
    printf "  growth log:  %s %s\n" "$(status_icon $log_ok)" "$log_status"
    echo "-----------------------------------"

    if $is_healthy; then
        echo "Status: HEALTHY"
    else
        echo "Status: NOT RUNNING"
    fi
    echo ""
fi

if $STATUS_ONLY; then
    if $is_healthy; then exit 0; else exit 1; fi
fi

# ── Auto-start if requested ─────────────────────────────────────────
if $START_IF_DEAD && ! $is_healthy; then
    if ! $QUIET; then
        echo "Attempting to start NEXUS-6 growth daemon..."
    fi

    started=false

    # Method 1: Try launchd
    plist_path="$HOME/Library/LaunchAgents/$LAUNCHD_LABEL.plist"
    if [[ -f "$plist_path" ]] && [[ $launchd_ok -ne 0 ]]; then
        if launchctl load "$plist_path" 2>/dev/null; then
            if ! $QUIET; then echo "  -> Loaded via launchd"; fi
            started=true
        fi
    fi

    # Method 2: Try cron (install if missing)
    if ! $started && [[ $cron_ok -ne 0 ]]; then
        # Check if install script exists
        if [[ -f "$NEXUS_SCRIPTS/install_autonomous.sh" ]]; then
            if ! $QUIET; then echo "  -> install_autonomous.sh available (run manually to set up cron)"; fi
        fi
    fi

    # Method 3: Direct background start (always works)
    if ! $started; then
        if [[ -f "$DAEMON_SCRIPT" ]]; then
            nohup bash "$DAEMON_SCRIPT" \
                --max-cycles 999 --interval 30 --skip-commit \
                >> "$NEXUS_STATE/growth.log" 2>&1 &
            local_pid=$!
            echo "$local_pid" > "$DAEMON_PID_FILE"
            if ! $QUIET; then
                echo "  -> Started daemon directly (PID $local_pid)"
            fi
            started=true
        else
            if ! $QUIET; then
                echo "  -> ERROR: Daemon script not found: $DAEMON_SCRIPT"
            fi
        fi
    fi

    if $started; then
        # Log the auto-start event
        echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] health_check auto-started daemon" \
            >> "$NEXUS_STATE/notifications.log"
        exit 2
    else
        exit 1
    fi
fi

# ── Exit code ────────────────────────────────────────────────────────
if $is_healthy; then
    exit 0
else
    exit 1
fi
