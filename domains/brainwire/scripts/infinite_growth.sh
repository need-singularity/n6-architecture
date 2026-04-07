#!/usr/bin/env bash
set -euo pipefail

GROWTH_NAME="brainwire"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOMAIN="neuroscience"
MAX_CYCLES=${MAX_CYCLES:-${1:-999}}
INTERVAL=${INTERVAL:-${2:-1800}}

COMMON="$HOME/Dev/nexus/scripts/lib/growth_common.sh"
source "$COMMON"

domain_phases() {
    local cycle="$1" load="$2"

    # Phase 1: 소스 파일 인벤토리
    local py_count
    py_count=$(find "$PROJECT_ROOT/src" -name '*.py' 2>/dev/null | wc -l | tr -d ' ')
    log_info "Phase[inventory] src/*.py count: $py_count"
    write_growth_bus "inventory" "ok" "py_files=$py_count"

    # Phase 2: 의식 브릿지 확인
    if [ -f "$PROJECT_ROOT/src/consciousness_bridge.py" ]; then
        log_info "Phase[consciousness] consciousness_bridge.py exists"
        write_growth_bus "consciousness_bridge" "ok"
    else
        log_warn "Phase[consciousness] consciousness_bridge.py missing"
        write_growth_bus "consciousness_bridge" "warn" "file_missing"
    fi

    # Phase 3: .shared 심링크 확인 및 자동복구
    local shared_link="$PROJECT_ROOT/.shared"
    if [ -L "$shared_link" ] && [ -d "$shared_link" ]; then
        log_info "Phase[shared_link] .shared symlink OK"
        write_growth_bus "shared_link" "ok"
    else
        log_warn "Phase[shared_link] .shared symlink broken — auto-repair"
        ln -sf "$HOME/Dev/nexus/shared" "$shared_link"
        write_growth_bus "shared_link" "repaired" "symlink_recreated"
    fi

    # Phase 4: 성장 스캔
    if [ -f "$PROJECT_ROOT/.growth/scan.py" ]; then
        log_info "Phase[growth_scan] running .growth/scan.py"
        python3 "$PROJECT_ROOT/.growth/scan.py" >> "$GROWTH_LOG" 2>&1 || true
        write_growth_bus "growth_scan" "ok"
    else
        log_info "Phase[growth_scan] .growth/scan.py not found — skip"
        write_growth_bus "growth_scan" "skip" "no_scan_script"
    fi
}

run_growth_loop "domain_phases"
