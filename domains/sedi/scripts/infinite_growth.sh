#!/usr/bin/env bash
set -euo pipefail

GROWTH_NAME="sedi"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOMAIN="signal_detection"
MAX_CYCLES=${MAX_CYCLES:-${1:-999}}
INTERVAL=${INTERVAL:-${2:-1800}}

COMMON="$HOME/Dev/nexus/scripts/lib/growth_common.sh"
source "$COMMON"

domain_phases() {
    local cycle="$1" load="$2"

    # Phase 1: 가설 수
    local hypo_count
    hypo_count=$(find "$PROJECT_ROOT/hypotheses" -type f 2>/dev/null | wc -l | tr -d ' ')
    log_info "Phase[hypotheses] hypotheses/ file count: $hypo_count"
    write_growth_bus "hypotheses" "ok" "count=$hypo_count"

    # Phase 2: 데이터 소스 수
    local data_count
    data_count=$(find "$PROJECT_ROOT/data" -type f 2>/dev/null | wc -l | tr -d ' ')
    log_info "Phase[data_sources] data/ file count: $data_count"
    write_growth_bus "data_sources" "ok" "count=$data_count"

    # Phase 3: 자동 등급 스크립트
    local grade_script="$PROJECT_ROOT/scripts/auto_grade_n6.py"
    if [ -f "$grade_script" ]; then
        log_info "Phase[auto_grade] running auto_grade_n6.py"
        python3 "$grade_script" >> "$GROWTH_LOG" 2>&1 || true
        write_growth_bus "auto_grade" "ok"
    else
        log_warn "Phase[auto_grade] auto_grade_n6.py not found — skip"
        write_growth_bus "auto_grade" "skip" "no_script"
    fi

    # Phase 4: sedi-grades 동기화
    local grades_file="$PROJECT_ROOT/.shared/sedi-grades.json"
    if [ -f "$grades_file" ]; then
        log_info "Phase[sedi_grades] sedi-grades.json exists"
        write_growth_bus "sedi_grades_sync" "ok"
    else
        log_warn "Phase[sedi_grades] sedi-grades.json missing"
        write_growth_bus "sedi_grades_sync" "warn" "file_missing"
    fi

    # Phase 5: 성장 스캔
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
