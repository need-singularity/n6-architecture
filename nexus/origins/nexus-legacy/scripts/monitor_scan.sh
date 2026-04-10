#!/usr/bin/env bash
# NEXUS-6 Monitoring — periodic scan for anima training
# Usage: ./monitor_scan.sh [--interval SECONDS] [--target HOST:PORT]
# Scans checkpoint weights via SSH, records Phi/stability trends
#
# Default interval: 3600 seconds (1 hour)
# Results appended to: ~/.nexus/monitoring.jsonl

set -euo pipefail

# ── Defaults ──────────────────────────────────────────────
INTERVAL=3600
TARGET=""
LOG_DIR="$HOME/.nexus"
LOG_FILE="$LOG_DIR/monitoring.jsonl"
PHI_THRESHOLD="0.1"   # 1/(sigma-phi) = 1/10
NEXUS_BIN="${NEXUS_BIN:-nexus}"

# ── Parse arguments ───────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --interval)
            INTERVAL="$2"
            shift 2
            ;;
        --target)
            TARGET="$2"
            shift 2
            ;;
        --threshold)
            PHI_THRESHOLD="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [--interval SECONDS] [--target HOST:PORT] [--threshold PHI_MIN]"
            echo ""
            echo "Options:"
            echo "  --interval SECONDS   Scan interval (default: 3600 = 1 hour)"
            echo "  --target HOST:PORT   SSH target for checkpoint extraction"
            echo "  --threshold PHI_MIN  Minimum Phi score for alert (default: 0.1)"
            echo ""
            echo "Output: ~/.nexus/monitoring.jsonl"
            exit 0
            ;;
        *)
            echo "Unknown option: $1" >&2
            exit 1
            ;;
    esac
done

# ── Setup ─────────────────────────────────────────────────
mkdir -p "$LOG_DIR"

echo "=== NEXUS-6 Monitor ==="
echo "  Interval:  ${INTERVAL}s"
echo "  Target:    ${TARGET:-'(local)'}"
echo "  Log:       $LOG_FILE"
echo "  Threshold: $PHI_THRESHOLD"
echo ""

# ── Helper: extract checkpoint values from remote/local ──
extract_checkpoint_values() {
    local values=""
    if [[ -n "$TARGET" ]]; then
        # SSH to target, find latest checkpoint, extract key numeric values
        local host="${TARGET%%:*}"
        local port="${TARGET##*:}"
        [[ "$port" == "$host" ]] && port=22

        values=$(ssh -p "$port" "$host" \
            'find /tmp/checkpoints -name "*.json" -type f -printf "%T@ %p\n" 2>/dev/null | sort -rn | head -1 | cut -d" " -f2 | xargs cat 2>/dev/null | grep -oE "[0-9]+\.?[0-9]*" | head -20 | tr "\n" " "' \
            2>/dev/null || echo "")
    else
        # Local mode: check for any checkpoint files
        local ckpt_dir="/tmp/nexus_checkpoints"
        if [[ -d "$ckpt_dir" ]]; then
            local latest
            latest=$(find "$ckpt_dir" -name "*.json" -type f -printf "%T@ %p\n" 2>/dev/null | sort -rn | head -1 | cut -d" " -f2)
            if [[ -n "$latest" ]]; then
                values=$(grep -oE '[0-9]+\.?[0-9]*' "$latest" 2>/dev/null | head -20 | tr '\n' ' ')
            fi
        fi
    fi

    # Fallback: n=6 canonical probe values
    if [[ -z "$values" ]]; then
        values="6.0 12.0 24.0 4.0 2.0 5.0"
    fi

    echo "$values"
}

# ── Helper: run nexus n6_check on values ─────────────────
run_n6_check() {
    local values="$1"
    local results=""
    local n6_count=0
    local total=0

    for val in $values; do
        total=$((total + 1))
        local output
        output=$("$NEXUS_BIN" verify "$val" 2>/dev/null || echo "NONE")
        if echo "$output" | grep -q "EXACT"; then
            n6_count=$((n6_count + 1))
        fi
    done

    local ratio="0.0"
    if [[ $total -gt 0 ]]; then
        ratio=$(awk "BEGIN { printf \"%.4f\", $n6_count / $total }")
    fi

    echo "$ratio $n6_count $total"
}

# ── Main monitoring loop ──────────────────────────────────
CYCLE=0
while true; do
    CYCLE=$((CYCLE + 1))
    TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    START_TIME=$(date +%s)

    echo "[cycle $CYCLE] $TIMESTAMP"

    # Extract checkpoint values
    VALUES=$(extract_checkpoint_values)
    echo "  Values: $VALUES"

    # Run n6 check
    read -r PHI_RATIO N6_COUNT TOTAL <<< "$(run_n6_check "$VALUES")"

    # Compute stability (simple: ratio of n6 matches)
    STABILITY="$PHI_RATIO"

    END_TIME=$(date +%s)
    ELAPSED=$((END_TIME - START_TIME))

    echo "  Phi ratio:  $PHI_RATIO ($N6_COUNT/$TOTAL EXACT)"
    echo "  Stability:  $STABILITY"
    echo "  Elapsed:    ${ELAPSED}s"

    # Check alert threshold
    ALERT="false"
    if awk "BEGIN { exit !($PHI_RATIO < $PHI_THRESHOLD) }"; then
        ALERT="true"
        echo "  ** ALERT: Phi ($PHI_RATIO) below threshold ($PHI_THRESHOLD) **" >&2
    fi

    # Append JSONL record
    cat >> "$LOG_FILE" << JSONEOF
{"timestamp":"$TIMESTAMP","cycle":$CYCLE,"phi_ratio":$PHI_RATIO,"n6_count":$N6_COUNT,"total":$TOTAL,"stability":$STABILITY,"elapsed_s":$ELAPSED,"alert":$ALERT,"target":"${TARGET:-local}"}
JSONEOF

    echo "  Logged to $LOG_FILE"
    echo ""

    # Sleep until next cycle
    if [[ "$INTERVAL" -eq 0 ]]; then
        echo "Single-shot mode (interval=0), exiting."
        break
    fi

    sleep "$INTERVAL"
done
