#!/usr/bin/env bash
# BrainWire Infinite Growth Daemon — 10-Phase Loop
# bash 3.2 compatible (no associative arrays, no |& , no [[ -v ]])
#
# Phases:
#   1: Modality coverage       6: Growth scan
#   2: Safety protocol check   7: PureField bridge status
#   3: Parameter verification  8: NEXUS-6 sync
#   4: NEXUS-6 scan            9: Growth tick
#   5: BCI protocol health    10: Auto-commit
#
# Usage: ./infinite_growth.sh [--max-cycles N] [--interval SEC] [--dry-run]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# ── Shared infrastructure ──
source "$SCRIPT_DIR/lib/growth_common.sh"

BW_ROOT="$PROJECT_ROOT"
STATE_FILE="$GROWTH_DIR/growth_state.json"
GROWTH_BUS="$HOME/Dev/nexus/shared/growth_bus.jsonl"
SCAN_PY="$GROWTH_DIR/scan.py"
_LOG_FILE="$GROWTH_DIR/growth.log"
PIDFILE="/tmp/brainwire_growth.pid"

# ── Defaults ──
MAX_CYCLES=999
INTERVAL=300
DRY_RUN=false

# ── Parse args ──
while test $# -gt 0; do
    case "$1" in
        --max-cycles)  MAX_CYCLES="$2"; shift 2 ;;
        --interval)    INTERVAL="$2"; shift 2 ;;
        --dry-run)     DRY_RUN=true; shift ;;
        -h|--help)
            echo "BrainWire Infinite Growth Daemon"
            echo "Usage: $0 [--max-cycles N] [--interval SEC] [--dry-run]"
            exit 0
            ;;
        *) echo "Unknown: $1"; exit 1 ;;
    esac
done

# ── Setup ──
mkdir -p "$GROWTH_DIR"
mkdir -p "$(dirname "$GROWTH_BUS")" 2>/dev/null || true

# ── Singleton lock ──
singleton_acquire "$PIDFILE"

write_bus() {
    local phase_name="$1"
    local status="$2"
    local detail="$3"
    local ts
    ts="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    local entry="{\"ts\":\"$ts\",\"repo\":\"brainwire\",\"type\":\"growth_phase\",\"phase\":\"$phase_name\",\"status\":\"$status\",\"detail\":\"$detail\"}"
    echo "$entry" >> "$GROWTH_BUS" 2>/dev/null || true
}

# ── Graceful shutdown ──
SHUTDOWN=false
# Re-register trap (singleton_acquire sets its own)
trap 'log_info "Shutting down..."; SHUTDOWN=true; rm -f "$PIDFILE"' INT TERM

# ── Phase functions (repo-specific) ──

phase_1_modality_coverage() {
    log_info "Phase 1: Modality coverage"
    if test -f "$SCAN_PY"; then
        local output
        output="$(python3 "$SCAN_PY" --quiet 2>/dev/null)" || true
        local pct
        pct="$(echo "$output" | python3 -c "import sys,json; print(json.load(sys.stdin).get('modality_coverage_pct',0))" 2>/dev/null)" || pct="?"
        local missing
        missing="$(echo "$output" | python3 -c "import sys,json; m=json.load(sys.stdin).get('modalities_missing',[]); print(','.join(m) if m else 'none')" 2>/dev/null)" || missing="?"
        log_info "  Modality coverage: ${pct}%, missing: $missing"
        write_bus "modality_coverage" "ok" "pct=$pct,missing=$missing"
    else
        log_warn "  scan.py not found"
        write_bus "modality_coverage" "skip" "scan.py missing"
    fi
}

phase_2_safety_check() {
    log_info "Phase 2: Safety protocol check"
    local safety_count=0
    safety_count="$(find "$BW_ROOT" -name '*safety*' -type f 2>/dev/null | wc -l | tr -d ' ')" || safety_count=0

    # Check critical safety module
    local has_safety_py="false"
    if test -f "$BW_ROOT/brainwire/hardware/safety.py"; then
        has_safety_py="true"
    fi

    # Check for experiment protocol
    local has_protocol="false"
    if test -f "$BW_ROOT/docs/experiment-protocol.md"; then
        has_protocol="true"
    fi

    log_info "  Safety files: $safety_count, safety.py: $has_safety_py, protocol: $has_protocol"
    write_bus "safety_check" "ok" "files=$safety_count,safety_py=$has_safety_py,protocol=$has_protocol"
}

phase_3_parameter_verify() {
    log_info "Phase 3: Parameter verification"
    local config_count=0
    if test -f "$BW_ROOT/brainwire/hardware/configs.py"; then
        config_count="$(grep -c 'class\|= {' "$BW_ROOT/brainwire/hardware/configs.py" 2>/dev/null)" || config_count=0
    fi

    local profile_count=0
    if test -d "$BW_ROOT/brainwire/profiles"; then
        profile_count="$(find "$BW_ROOT/brainwire/profiles" -type f 2>/dev/null | wc -l | tr -d ' ')"
    fi

    local test_count=0
    if test -d "$BW_ROOT/tests"; then
        test_count="$(find "$BW_ROOT/tests" -name 'test_*.py' -type f 2>/dev/null | wc -l | tr -d ' ')"
    fi

    log_info "  Configs: $config_count, profiles: $profile_count, tests: $test_count"
    write_bus "parameter_verify" "ok" "configs=$config_count,profiles=$profile_count,tests=$test_count"
}

phase_4_nexus_scan() {
    log_info "Phase 4: NEXUS-6 scan"
    local n6_script="$HOME/Dev/nexus/scripts/n6.py"
    if test -f "$n6_script"; then
        python3 "$n6_script" scan --repo "$BW_ROOT" > /dev/null 2>&1 || true
        log_info "  NEXUS-6 scan complete"
        write_bus "nexus_scan" "ok" "completed"
    else
        local alt_n6
        for alt_n6 in \
            "$BW_ROOT/tools/nexus/scripts/n6.py" \
            "$HOME/Dev/n6-architecture/tools/nexus/scripts/n6.py"; do
            if test -f "$alt_n6"; then
                python3 "$alt_n6" scan > /dev/null 2>&1 || true
                log_info "  NEXUS-6 scan complete (alt)"
                write_bus "nexus_scan" "ok" "alt_path"
                return
            fi
        done
        log_warn "  NEXUS-6 n6.py not found"
        write_bus "nexus_scan" "skip" "not found"
    fi
}

phase_5_bci_protocol() {
    log_info "Phase 5: BCI protocol health"
    # Check core BCI modules
    local engine_files=0
    if test -d "$BW_ROOT/brainwire/engine"; then
        engine_files="$(find "$BW_ROOT/brainwire/engine" -name '*.py' -type f 2>/dev/null | wc -l | tr -d ' ')"
    fi

    local has_eeg="false"
    if test -f "$BW_ROOT/brainwire/eeg_feedback.py"; then
        has_eeg="true"
    fi

    local has_protocol="false"
    if test -f "$BW_ROOT/brainwire/protocol.py"; then
        has_protocol="true"
    fi

    log_info "  Engine files: $engine_files, EEG: $has_eeg, protocol: $has_protocol"
    write_bus "bci_protocol" "ok" "engine=$engine_files,eeg=$has_eeg,protocol=$has_protocol"
}

phase_6_growth_scan() {
    log_info "Phase 6: Growth scan (full)"
    if test -f "$SCAN_PY"; then
        local health
        health="$(python3 "$SCAN_PY" 2>/dev/null | python3 -c "import sys,json; print(json.load(sys.stdin).get('health_score',0))" 2>/dev/null)" || health="?"
        log_info "  Health score: $health"
        write_bus "growth_scan" "ok" "health=$health"
    else
        write_bus "growth_scan" "skip" "scan.py missing"
    fi
}

phase_7_purefield_bridge() {
    log_info "Phase 7: PureField bridge status"
    local bridge_files=0
    bridge_files="$(find "$BW_ROOT" -name '*purefield*' -o -name '*pure_field*' -o -name '*anima*bridge*' 2>/dev/null | wc -l | tr -d ' ')" || bridge_files=0

    local consciousness_files=0
    consciousness_files="$(find "$BW_ROOT" -name '*consciousness*' -type f 2>/dev/null | wc -l | tr -d ' ')" || consciousness_files=0

    log_info "  PureField bridge: $bridge_files files, consciousness: $consciousness_files files"
    write_bus "purefield_bridge" "ok" "bridge=$bridge_files,consciousness=$consciousness_files"
}

phase_8_nexus_sync() {
    log_info "Phase 8: NEXUS-6 sync"
    local sync_script="$HOME/Dev/nexus/sync/sync-all.sh"
    if test -f "$sync_script"; then
        bash "$sync_script" > /dev/null 2>&1 || true
        log_info "  NEXUS-6 sync complete"
        write_bus "nexus_sync" "ok" "synced"
    else
        if test -f "$BW_ROOT/.shared/sync-nexus-lenses.sh"; then
            bash "$BW_ROOT/.shared/sync-nexus-lenses.sh" > /dev/null 2>&1 || true
            log_info "  Lens sync complete"
            write_bus "nexus_sync" "ok" "lens_sync"
        else
            log_warn "  No sync script found"
            write_bus "nexus_sync" "skip" "no sync script"
        fi
    fi
}

phase_9_growth_tick() {
    log_info "Phase 9: Growth tick"
    python3 -c "
import json
from datetime import datetime, timezone
sf = '$STATE_FILE'
try:
    state = json.load(open(sf))
except: state = {}
hist = state.get('phase_history', [])
hist.append({
    'ts': datetime.now(timezone.utc).isoformat(),
    'cycle': state.get('cycle_count', 0),
    'health': state.get('last_health', 0)
})
if len(hist) > 200: hist = hist[-100:]
state['phase_history'] = hist
json.dump(state, open(sf, 'w'), indent=2)
" 2>/dev/null || true
    write_bus "growth_tick" "ok" "recorded"
}

phase_10_auto_commit() {
    log_info "Phase 10: Auto-commit"
    if "$DRY_RUN"; then
        log_info "  Dry-run: skipping commit"
        write_bus "auto_commit" "skip" "dry_run"
        return
    fi

    growth_commit ".growth/" "growth(brainwire): cycle $(date +%Y%m%d-%H%M) health update"
    write_bus "auto_commit" "ok" "committed"
}

# ── Main loop ──
log_info "=== BrainWire Infinite Growth Daemon ==="
log_info "Max cycles: $MAX_CYCLES, Interval: ${INTERVAL}s, Dry-run: $DRY_RUN"

cycle=0
while test "$cycle" -lt "$MAX_CYCLES"; do
    if "$SHUTDOWN"; then
        log_info "Shutdown requested, exiting"
        break
    fi

    cycle=$((cycle + 1))
    log_info "--- Cycle $cycle/$MAX_CYCLES ---"

    # Resource check before heavy phases
    local_res="$(check_resources)"
    print_resources
    if [ "$local_res" = "STOP" ]; then
        log_error "Resources critical (disk), skipping cycle"
        sleep 60
        continue
    fi
    if [ "$local_res" = "LIGHT" ]; then
        log_warn "Low memory, running lightweight phases only (1,6,9,10)"
        phase_1_modality_coverage
        phase_6_growth_scan
        phase_9_growth_tick
        phase_10_auto_commit
        update_growth_state "$cycle"
        sleep "$INTERVAL"
        continue
    fi

    # Repo-specific phases (1-7)
    phase_1_modality_coverage
    phase_2_safety_check
    phase_3_parameter_verify
    phase_4_nexus_scan
    phase_5_bci_protocol
    phase_6_growth_scan
    phase_7_purefield_bridge
    # Common phases (8-12): doc + domain + paper + sync + emergence
    phase_8_nexus_sync
    run_common_phases "brainwire" $cycle
    phase_9_growth_tick
    phase_10_auto_commit

    update_growth_state "$cycle"

    if test "$cycle" -lt "$MAX_CYCLES" && ! "$SHUTDOWN"; then
        log_info "Sleeping ${INTERVAL}s before next cycle..."
        sleep "$INTERVAL" &
        SLEEP_PID=$!
        trap 'log_info "Shutting down..."; SHUTDOWN=true; kill $SLEEP_PID 2>/dev/null || true; rm -f "$PIDFILE"' INT TERM
        wait "$SLEEP_PID" 2>/dev/null || true
    fi
done

log_info "=== BrainWire Growth Daemon finished ($cycle cycles) ==="
