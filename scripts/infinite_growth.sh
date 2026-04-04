#!/usr/bin/env bash
set -euo pipefail
GROWTH_NAME="n6-architecture"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOMAIN="architecture"
MAX_CYCLES=${MAX_CYCLES:-${1:-999}}
INTERVAL=${INTERVAL:-${2:-1800}}

# 공통 라이브러리
COMMON="$HOME/Dev/nexus6/scripts/lib/growth_common.sh"
source "$COMMON"

# 프로젝트별 phases
domain_phases() {
    local cycle="$1" load="$2"

    # 1. 아키텍처 문서 수
    log_info "Phase: 아키텍처 문서 스캔"
    local docs_dir="$PROJECT_ROOT/docs"
    if [ -d "$docs_dir" ]; then
        local md_count
        md_count=$(find "$docs_dir" -name '*.md' 2>/dev/null | wc -l | tr -d ' ')
        log_info "  docs/ Markdown: ${md_count}개"
        write_growth_bus "arch_docs" "ok" "count=${md_count}"
    else
        log_info "  docs/ 디렉토리 없음"
        write_growth_bus "arch_docs" "skip" "no_docs_dir"
    fi

    # 2. nexus6 tools 빌드
    log_info "Phase: nexus6 tools 빌드"
    local tools_dir="$PROJECT_ROOT/tools/nexus6"
    if [ -d "$tools_dir" ]; then
        local tools_cargo="$tools_dir/Cargo.toml"
        if [ -f "$tools_cargo" ]; then
            if cargo check --manifest-path "$tools_cargo" 2>/dev/null; then
                log_info "  cargo check tools/nexus6: OK"
                write_growth_bus "tools_build" "ok" ""
            else
                log_warn "  cargo check tools/nexus6: FAIL"
                write_growth_bus "tools_build" "fail" "cargo_check_error"
            fi
        else
            log_info "  tools/nexus6/Cargo.toml 없음"
            write_growth_bus "tools_build" "skip" "no_cargo_toml"
        fi
    else
        log_info "  tools/nexus6 디렉토리 없음 — skip"
        write_growth_bus "tools_build" "skip" "no_tools_dir"
    fi

    # 3. DSE 상태
    log_info "Phase: DSE 상태"
    local dse_dir="$PROJECT_ROOT/dse"
    if [ -d "$dse_dir" ]; then
        local dse_count
        dse_count=$(find "$dse_dir" -type f 2>/dev/null | wc -l | tr -d ' ')
        log_info "  dse/ 파일: ${dse_count}개"
        write_growth_bus "dse_status" "ok" "count=${dse_count}"
    else
        log_info "  dse/ 디렉토리 없음"
        write_growth_bus "dse_status" "skip" "no_dse_dir"
    fi

    # 4. 성장 스캔
    log_info "Phase: 성장 스캔"
    local scan_script="$PROJECT_ROOT/.growth/scan.py"
    if [ -f "$scan_script" ]; then
        python3 "$scan_script" 2>/dev/null | tail -5 | while IFS= read -r line; do
            log_info "  $line"
        done
        write_growth_bus "growth_scan" "ok" ""
    else
        log_info "  .growth/scan.py 없음 — skip"
        write_growth_bus "growth_scan" "skip" "no_script"
    fi
}

run_growth_loop "domain_phases"
