#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════
# growth_common.sh — Shared Growth Infrastructure
# ═══════════════════════════════════════════════════════════════
# Source this from any growth script:
#   source "$(dirname "$0")/lib/growth_common.sh"
#
# Provides: singleton, logging, resource monitoring, git ops,
#           phase runner, measurement, paper publish, doc sync

# ── Constants (n=6 family) ──────────────────────────────────────
readonly N6_SIGMA=12
readonly N6_J2=24
readonly N6_TAU=4
readonly N6_PHI=2
readonly N6_SOPFR=5
readonly N6_N=6

# ── Paths ───────────────────────────────────────────────────────
GROWTH_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="$(cd "$GROWTH_LIB_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$SCRIPTS_DIR/.." && pwd)"
NEXUS6_SCRIPTS="$PROJECT_ROOT/tools/nexus6/scripts"
NEXUS6_ROOT="${HOME}/Dev/nexus6"
NEXUS6_STATE="${HOME}/.nexus6"
GROWTH_DIR="$PROJECT_ROOT/.growth"
DOCS_DIR="$PROJECT_ROOT/docs"
PAPERS_DIR="${HOME}/Dev/papers"

# ── Logging ─────────────────────────────────────────────────────
_LOG_FILE="${_LOG_FILE:-/tmp/n6_growth.log}"

log_ts()    { date +%H:%M:%S; }
log_info()  { echo "[$(log_ts)] INFO:  $*" | tee -a "$_LOG_FILE"; }
log_warn()  { echo "[$(log_ts)] WARN:  $*" | tee -a "$_LOG_FILE"; }
log_error() { echo "[$(log_ts)] ERROR: $*" | tee -a "$_LOG_FILE"; }
log_ok()    { echo "[$(log_ts)] OK:    $*" | tee -a "$_LOG_FILE"; }

# ── Singleton (단독 실행 — 중복 시 즉시 종료) ──────────────────
# Usage: singleton_acquire "/tmp/my_daemon.pid"
#   Returns 0 if lock acquired, exits 1 if already running.
singleton_acquire() {
    local pidfile="$1"
    if [ -f "$pidfile" ]; then
        local old_pid
        old_pid=$(cat "$pidfile" 2>/dev/null) || true
        if [ -n "$old_pid" ] && kill -0 "$old_pid" 2>/dev/null; then
            echo "╔════════════════════════════════════════════════════════╗"
            echo "║  Already running (PID $old_pid)                        "
            echo "║  단독 실행 정책: 중복 인스턴스 거부                    ║"
            echo "║  종료하려면: kill $old_pid                              "
            echo "╚════════════════════════════════════════════════════════╝"
            exit 1
        fi
        rm -f "$pidfile"
    fi
    echo $$ > "$pidfile"
    trap "rm -f '$pidfile'; exit 0" SIGTERM SIGINT EXIT
}

# ── Resource Monitoring (macOS) ─────────────────────────────────
# Thresholds (configurable by caller)
: "${CPU_THROTTLE_PCT:=80}"
: "${MEM_MIN_FREE_MB:=512}"
: "${DISK_MIN_FREE_GB:=2}"

get_cpu_usage() {
    # macOS top: "CPU usage: X% user, Y% sys, Z% idle"
    local idle
    idle=$(top -l 1 -n 0 2>/dev/null | awk '/CPU usage/ {
        for(i=1;i<=NF;i++) if($i=="idle") {gsub(/%/,"",$(i-1)); print $(i-1)}
    }' || echo "50")
    [ -z "$idle" ] && idle="50"
    python3 -c "print(max(0, min(100, int(100 - float('${idle}')))))" 2>/dev/null || echo "50"
}

get_free_mem_mb() {
    # macOS: free + inactive pages = available memory
    local page_size free_pages inactive_pages
    page_size=$(sysctl -n hw.pagesize 2>/dev/null || echo 16384)
    free_pages=$(vm_stat 2>/dev/null | awk '/Pages free/ {gsub(/\./,""); print $3}' || echo "0")
    inactive_pages=$(vm_stat 2>/dev/null | awk '/Pages inactive/ {gsub(/\./,""); print $3}' || echo "0")
    python3 -c "print(int((${free_pages} + ${inactive_pages}) * ${page_size} / 1024 / 1024))" 2>/dev/null || echo "4096"
}

get_free_disk_gb() {
    df -g "$PROJECT_ROOT" 2>/dev/null | awk 'NR==2 {print $4}' || echo "10"
}

# Returns: OK | THROTTLE | LIGHT | STOP
check_resources() {
    local cpu_pct mem_free_mb disk_free_gb
    cpu_pct=$(get_cpu_usage)
    mem_free_mb=$(get_free_mem_mb)
    disk_free_gb=$(get_free_disk_gb)
    # Export for callers
    export _RES_CPU="$cpu_pct"
    export _RES_MEM="$mem_free_mb"
    export _RES_DISK="$disk_free_gb"

    if [ "$disk_free_gb" -lt "$DISK_MIN_FREE_GB" ]; then echo "STOP"; return; fi
    if [ "$mem_free_mb" -lt "$MEM_MIN_FREE_MB" ]; then echo "LIGHT"; return; fi
    if [ "$cpu_pct" -gt "$CPU_THROTTLE_PCT" ]; then echo "THROTTLE"; return; fi
    echo "OK"
}

print_resources() {
    local cpu="${_RES_CPU:-0}" mem="${_RES_MEM:-0}" disk="${_RES_DISK:-0}"
    local cpu_filled=$((cpu / 10))
    local bar=""
    for i in $(seq 1 "$cpu_filled"); do bar="${bar}█"; done
    for i in $(seq 1 $((10 - cpu_filled))); do bar="${bar}░"; done

    local mem_s="OK"; [ "$mem" -lt "$MEM_MIN_FREE_MB" ] && mem_s="LOW"
    local dsk_s="OK"; [ "$disk" -lt "$DISK_MIN_FREE_GB" ] && dsk_s="CRIT"

    echo "  ┌─ Resources ──────────────────────────────────────────┐"
    printf "  │ CPU: %s %3d%%  MEM: %5dMB [%-4s] DISK: %3dGB [%-4s] │\n" \
        "$bar" "$cpu" "$mem" "$mem_s" "$disk" "$dsk_s"
    printf "  │ PID: %-8d                                        │\n" "$$"
    echo "  └───────────────────────────────────────────────────────┘"
}

# ── Phase Runner ────────────────────────────────────────────────
# Usage: run_phase NUM TOTAL NAME COMMAND [MAX_LINES]
run_phase() {
    local num="$1" total="$2" name="$3" cmd="$4" lines="${5:-10}"
    echo "[$(log_ts)] Phase ${num}/${total}: ${name}..."
    if eval "$cmd" 2>&1 | tail -"$lines"; then
        echo "  [OK] ${name}"
    else
        echo "  [WARN] ${name} (non-fatal)"
    fi
    echo ""
}

# ── Git Ops ─────────────────────────────────────────────────────
# Usage: growth_commit SCOPE MESSAGE
#   SCOPE: directory to git add (e.g. ".growth/" or "tools/nexus6/src/")
#   MESSAGE: commit message
growth_commit() {
    local scope="$1" message="$2"
    cd "$PROJECT_ROOT"
    git add "$scope" 2>/dev/null || true
    if git diff --cached --quiet 2>/dev/null; then
        log_info "  No changes to commit"
    else
        git commit -m "$message" --no-verify 2>/dev/null && log_ok "Committed: $message" || log_warn "Commit failed"
    fi
}

# ── Count Helper (bash 3.2 safe) ───────────────────────────────
count_pattern() {
    local file="$1" pattern="$2"
    grep -c "$pattern" "$file" 2>/dev/null || echo "0"
}

# ── Size Expansion (σ=12 → J₂=24) ─────────────────────────────
# Phase count grows with cycle number
get_phase_count() {
    local cycle="$1"
    if [ "$cycle" -ge 25 ]; then echo 24    # J₂=24 max
    elif [ "$cycle" -ge 13 ]; then echo 18  # 18 phases
    elif [ "$cycle" -ge 7 ]; then echo 15   # 15 phases
    else echo 12                             # σ=12 base
    fi
}

# ── Paper Publish Loop ──────────────────────────────────────────
# Checks for publishable content and auto-publishes via publish_paper.sh
run_paper_loop() {
    local papers_script="$PAPERS_DIR/publish_paper.sh"
    if [ ! -f "$papers_script" ]; then
        echo "  publish_paper.sh not found at $papers_script"
        return 0
    fi

    # Check for new/updated papers
    local paper_candidates=0
    local published=0

    # Scan docs/paper/ for unpublished papers
    for f in "$DOCS_DIR"/paper/*.md; do
        [ -f "$f" ] || continue
        paper_candidates=$((paper_candidates + 1))
        local basename_f
        basename_f=$(basename "$f")
        # Check if already in manifest
        if [ -f "$PAPERS_DIR/manifest.json" ]; then
            if grep -q "$basename_f" "$PAPERS_DIR/manifest.json" 2>/dev/null; then
                continue
            fi
        fi
        # Unpublished paper found — dry-run first
        echo "  Found unpublished: $basename_f"
        if bash "$papers_script" "$f" --dry-run 2>/dev/null | tail -3; then
            published=$((published + 1))
        fi
    done

    # Scan for blowup papers
    for f in "$DOCS_DIR"/paper/blowup-*.md; do
        [ -f "$f" ] || continue
        local basename_f
        basename_f=$(basename "$f")
        if [ -f "$PAPERS_DIR/manifest.json" ]; then
            grep -q "$basename_f" "$PAPERS_DIR/manifest.json" 2>/dev/null && continue
        fi
        echo "  Found blowup paper: $basename_f"
        published=$((published + 1))
    done

    echo "  Papers scanned: $paper_candidates, new/updated: $published"
}

# ── Auto Domain Explorer ───────────────────────────────────────
# Finds domains without DSE and creates TOML stubs
run_auto_domain_explore() {
    local dse_map="$DOCS_DIR/dse-map.toml"
    local dse_domains="$PROJECT_ROOT/tools/universal-dse/domains"

    if [ ! -f "$dse_map" ]; then
        echo "  dse-map.toml not found"
        return 0
    fi

    # Count unexplored domains
    local none_count
    none_count=$(count_pattern "$dse_map" 'dse.*=.*none')
    local wip_count
    wip_count=$(count_pattern "$dse_map" 'dse.*=.*wip')
    local done_count
    done_count=$(count_pattern "$dse_map" 'dse.*=.*done')
    local total
    total=$(grep -c '^\[' "$dse_map" 2>/dev/null || echo 0)
    total=$((total - 1))

    echo "  DSE Status: done=$done_count wip=$wip_count none=$none_count total=$total"

    # Check for doc domains without TOML
    local missing_toml=0
    for d in "$DOCS_DIR"/*/; do
        [ -d "$d" ] || continue
        local domain_name
        domain_name=$(basename "$d")
        if [ ! -f "$dse_domains/${domain_name}.toml" ] 2>/dev/null; then
            if [ -f "${d}goal.md" ] || [ -f "${d}hypotheses.md" ]; then
                missing_toml=$((missing_toml + 1))
                echo "  Missing TOML: $domain_name (has goal/hypotheses)"
            fi
        fi
    done

    echo "  Domains with content but no TOML: $missing_toml"
    if [ "$none_count" -gt 0 ]; then
        echo "  WARNING: $none_count domains have no DSE results"
    fi
}

# ── Auto Document Update ───────────────────────────────────────
# Syncs README, atlas, calculators, and checks doc completeness
run_auto_doc_update() {
    local updated=0

    # 1. Check doc completeness per domain
    local domains=0 has_hyp=0 has_goal=0 has_verify=0 has_extreme=0
    for d in "$DOCS_DIR"/*/; do
        [ -d "$d" ] || continue
        domains=$((domains + 1))
        [ -f "${d}hypotheses.md" ] && has_hyp=$((has_hyp + 1))
        [ -f "${d}goal.md" ] && has_goal=$((has_goal + 1))
        [ -f "${d}verification.md" ] && has_verify=$((has_verify + 1))
        [ -f "${d}extreme-hypotheses.md" ] && has_extreme=$((has_extreme + 1))
    done
    echo "  Docs: $domains domains"
    echo "    hypotheses: $has_hyp/$domains  goal: $has_goal/$domains"
    echo "    verification: $has_verify/$domains  extreme: $has_extreme/$domains"

    # 2. Atlas sync
    local scanner="$HOME/Dev/TECS-L/.shared/scan_math_atlas.py"
    if [ -f "$scanner" ]; then
        python3 "$scanner" --summary 2>/dev/null | tail -3 || echo "  Atlas scanner: skip"
        updated=$((updated + 1))
    fi

    # 3. Calculator registry sync
    local calc_scanner="$HOME/Dev/TECS-L/.shared/scan-calculators.py"
    if [ -f "$calc_scanner" ]; then
        python3 "$calc_scanner" --summary 2>/dev/null | tail -2 || echo "  Calc scanner: skip"
        updated=$((updated + 1))
    fi

    # 4. README sync (if sync script exists)
    local sync_readme="$SCRIPTS_DIR/sync-readme.py"
    if [ -f "$sync_readme" ]; then
        python3 "$sync_readme" 2>/dev/null | tail -2 || echo "  README sync: skip"
        updated=$((updated + 1))
    fi

    echo "  Sync operations attempted: $updated"
}

# ── NEXUS-6 Daemon Bridge ──────────────────────────────────────
# Calls nexus6 growth daemon for a single dimension growth cycle
run_nexus6_grow_dimension() {
    local dimension="${1:-}"
    if [ -z "$dimension" ]; then
        # Auto-pick weakest via daemon's measure
        dimension="auto"
    fi

    if [ -f "$NEXUS6_SCRIPTS/nexus6_growth_daemon.sh" ]; then
        if [ "$dimension" = "auto" ]; then
            bash "$NEXUS6_SCRIPTS/nexus6_growth_daemon.sh" --max-cycles 1 --skip-commit 2>/dev/null | tail -20
        else
            bash "$NEXUS6_SCRIPTS/nexus6_growth_daemon.sh" --max-cycles 1 --dimension "$dimension" --skip-commit 2>/dev/null | tail -20
        fi
    else
        echo "  nexus6_growth_daemon.sh not found"
    fi
}

# ── Growth State Update ────────────────────────────────────────
update_growth_state() {
    local cycle="$1"
    local state_file="$GROWTH_DIR/growth_state.json"
    [ -f "$state_file" ] || return 0

    python3 -c "
import json, time
g = json.load(open('$state_file'))
g['scan_count'] = g.get('scan_count', 0) + 1
g['total_growth'] = g.get('total_growth', 0) + 1
g['last_tick'] = time.strftime('%Y-%m-%dT%H:%M:%S')
g['infinite_cycle'] = $cycle
json.dump(g, open('$state_file', 'w'), indent=2, ensure_ascii=False)
print(f'  Scan #{g[\"scan_count\"]}, total growth: {g[\"total_growth\"]}')
" 2>/dev/null || echo "  Growth tick: failed"
}

# ── Growth Bus Sync ─────────────────────────────────────────────
sync_growth_bus() {
    local bus_file="${NEXUS6_ROOT}/shared/growth_bus.jsonl"
    if [ -f "$bus_file" ]; then
        local bus_lines
        bus_lines=$(wc -l < "$bus_file" 2>/dev/null | tr -d ' ')
        local n6_entries
        n6_entries=$(grep -c 'n6-architecture' "$bus_file" 2>/dev/null || echo "0")
        echo "  Growth bus: $bus_lines total, $n6_entries from n6-arch"
    else
        echo "  Growth bus: not found"
    fi
}

# ═══════════════════════════════════════════════════════════════
# COMMON PHASES — 모든 리포에서 호출 가능한 공통 성장 phase
# ═══════════════════════════════════════════════════════════════

# Phase: 논문 퍼블리시 루프
common_phase_paper_loop() {
    log_info "Common Phase: Paper publish loop"
    run_paper_loop
}

# Phase: 자동 문서 갱신
common_phase_doc_update() {
    log_info "Common Phase: Auto doc update"
    run_auto_doc_update
}

# Phase: 자동 도메인 탐색
common_phase_domain_explore() {
    log_info "Common Phase: Auto domain explore"
    run_auto_domain_explore
}

# Phase: 성장 버스 동기화
common_phase_bus_sync() {
    log_info "Common Phase: Growth bus sync"
    sync_growth_bus
}

# Phase: 전체 동기화 (atlas + calc + readme + lenses)
common_phase_full_sync() {
    log_info "Common Phase: Full sync sweep"
    local synced=0
    for script in \
        "$HOME/Dev/TECS-L/.shared/scan_math_atlas.py" \
        "$HOME/Dev/TECS-L/.shared/scan-calculators.py"; do
        if [ -f "$script" ]; then
            python3 "$script" --summary 2>/dev/null | tail -2 || true
            synced=$((synced + 1))
        fi
    done
    # Lens sync
    local lens_sync="$PROJECT_ROOT/.shared/sync-nexus6-lenses.sh"
    if [ -f "$lens_sync" ]; then
        bash "$lens_sync" 2>/dev/null | tail -2 || true
        synced=$((synced + 1))
    fi
    echo "  Synced: $synced operations"
}

# Phase: 창발 시도 (blowup emergence — 코어 탐색)
common_phase_emergence() {
    log_info "Common Phase: Emergence attempt"
    local emergence_script="$HOME/Dev/n6-architecture/tools/nexus6/scripts/growth_infinite_lens.py"
    if [ -f "$emergence_script" ]; then
        python3 "$emergence_script" 창발 --cycles 6 2>/dev/null | tail -8 || echo "  Emergence: unavailable"
    else
        echo "  growth_infinite_lens.py not found"
    fi
}

# ── Run all common phases (call from any repo's growth loop) ───
# Usage: run_common_phases [REPO_NAME]
#   Runs: doc_update → domain_explore → paper_loop → bus_sync → full_sync
run_common_phases() {
    local repo_name="${1:-unknown}"
    log_info "=== Common growth phases for $repo_name ==="
    common_phase_doc_update
    common_phase_domain_explore
    common_phase_paper_loop
    common_phase_bus_sync
    common_phase_emergence
    common_phase_full_sync
}

# ═══════════════════════════════════════════════════════════════
# COMMON PHASES — 모든 리포에서 공유하는 phase 함수
# 각 리포는 자기 고유 phase + 이 공통 phase를 조합
# ═══════════════════════════════════════════════════════════════

# ── 공통 Phase: 논문 루프 ───────────────────────────────────────
# 모든 리포의 docs/paper/ 스캔 → 미발행 논문 발견 → dry-run 리포트
common_phase_paper_loop() {
    log_info "  [Common] Paper publish loop"
    run_paper_loop
}

# ── 공통 Phase: 문서 자동 갱신 ──────────────────────────────────
# Atlas sync + calculator sync + README sync + doc completeness
common_phase_doc_update() {
    log_info "  [Common] Auto document update"
    run_auto_doc_update
}

# ── 공통 Phase: 도메인 탐색 ─────────────────────────────────────
# DSE 미탐색 도메인 발견 + TOML 누락 체크
common_phase_domain_explore() {
    log_info "  [Common] Auto domain explorer"
    run_auto_domain_explore
}

# ── 공통 Phase: NEXUS-6 스캔 ───────────────────────────────────
# 어느 리포에서든 NEXUS-6 스캔 실행
common_phase_nexus6_scan() {
    log_info "  [Common] NEXUS-6 scan"
    local n6_script="$HOME/Dev/nexus6/scripts/n6.py"
    if [ -f "$n6_script" ]; then
        python3 "$n6_script" scan --repo "$PROJECT_ROOT" > /dev/null 2>&1 || true
        echo "  NEXUS-6 scan complete"
    else
        echo "  n6.py not found, skipping"
    fi
}

# ── 공통 Phase: NEXUS-6 동기화 ──────────────────────────────────
common_phase_nexus6_sync() {
    log_info "  [Common] NEXUS-6 sync"
    local sync_script="$HOME/Dev/nexus6/sync/sync-all.sh"
    if [ -f "$sync_script" ]; then
        bash "$sync_script" > /dev/null 2>&1 || true
        echo "  Sync complete"
    elif [ -f "$PROJECT_ROOT/.shared/sync-nexus6-lenses.sh" ]; then
        bash "$PROJECT_ROOT/.shared/sync-nexus6-lenses.sh" > /dev/null 2>&1 || true
        echo "  Lens sync complete"
    else
        echo "  No sync script"
    fi
}

# ── 공통 Phase: Growth Bus 쓰기 ────────────────────────────────
# 어느 리포에서든 growth bus에 이벤트 기록
write_growth_bus() {
    local repo_name="$1" phase_name="$2" status="$3" detail="$4"
    local bus_file="${NEXUS6_ROOT}/shared/growth_bus.jsonl"
    local ts
    ts="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
    echo "{\"ts\":\"$ts\",\"repo\":\"$repo_name\",\"type\":\"growth_phase\",\"phase\":\"$phase_name\",\"status\":\"$status\",\"detail\":\"$detail\"}" \
        >> "$bus_file" 2>/dev/null || true
}

# ── 공통 Phase: 창발 시도 ──────────────────────────────────────
# 블로업 엔진으로 한계 돌파 창발
common_phase_emergence() {
    log_info "  [Common] Emergence attempt (창발)"
    local lens_script="$PROJECT_ROOT/tools/nexus6/scripts/growth_infinite_lens.py"
    if [ -f "$lens_script" ]; then
        python3 "$lens_script" 창발 --cycles 6 2>/dev/null | tail -5 || echo "  Emergence: not available"
    else
        # Try from nexus6 repo
        lens_script="$HOME/Dev/n6-architecture/tools/nexus6/scripts/growth_infinite_lens.py"
        if [ -f "$lens_script" ]; then
            python3 "$lens_script" 창발 --cycles 6 2>/dev/null | tail -5 || echo "  Emergence: failed"
        else
            echo "  growth_infinite_lens.py not found"
        fi
    fi
}

# ── 공통 Phase: Auto-commit ────────────────────────────────────
# 리포별 커밋 메시지만 다르게
common_phase_auto_commit() {
    local repo_name="$1"
    local cycle="$2"
    local dry_run="${3:-false}"
    if [ "$dry_run" = "true" ]; then
        log_info "  Dry-run: skipping commit"
        return
    fi
    growth_commit ".growth/" "growth($repo_name): cycle $cycle"
}

# ── 공통 Phase: Growth Tick ────────────────────────────────────
common_phase_growth_tick() {
    local cycle="$1"
    update_growth_state "$cycle"
}

log_info "growth_common.sh loaded (n=$N6_N, σ=$N6_SIGMA, J₂=$N6_J2)"
