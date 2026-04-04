#!/usr/bin/env bash
set -euo pipefail

# ═══════════════════════════════════════════════════════════════
# N6-ARCHITECTURE Unified Infinite Growth Engine v3
# ═══════════════════════════════════════════════════════════════
# Single entry point for ALL growth — replaces separate scripts.
# Sources growth_common.sh for shared infrastructure.
#
# Usage: ./scripts/infinite_growth.sh [--interval MIN] [--max-cycles N]
#        Default: 3 min interval, 999 cycles
#
# Architecture:
#   Phase  1-12 (σ=12):  Base monitoring (DSE, BT, techniques, atlas...)
#   Phase 13-15:          NEXUS-6 dimension growth (weakest auto-pick)
#   Phase 16-18:          Auto domain explore + auto doc update + paper loop
#   Phase 19-21:          Cross-validation + growth bus sync + blowup
#   Phase 22-24 (J₂=24): Deep analysis + evolution + final commit
#
# Safety:
#   - Singleton: refuses to start if already running
#   - Resource monitoring: CPU/MEM/DISK per cycle, auto-throttle
#   - Size expansion: phases grow with cycle count (12→15→18→24)
#   - Stability: auto-pause on high load, skip heavy on low memory

# ── Source shared library ───────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/lib/growth_common.sh"

# ── Local paths (derived from common) ──────────────────────────
N6_ROOT="$PROJECT_ROOT"
DOCS="$DOCS_DIR"
TECHNIQUES="$N6_ROOT/techniques"
EXPERIMENTS="$N6_ROOT/experiments"
GROWTH_STATE="$GROWTH_DIR/growth_state.json"
GROWTH_SCAN="$GROWTH_DIR/scan.py"
DSE_MAP="$DOCS/dse-map.toml"
BT_FILE="$DOCS/breakthrough-theorems.md"
TP_FILE="$DOCS/testable-predictions.md"
ATLAS_FILE="$DOCS/atlas-constants.md"
PIDFILE="/tmp/n6_infinite_growth.pid"

# ── Parse arguments ─────────────────────────────────────────────
INTERVAL_MIN=3
MAX_CYCLES=999
CYCLE=0

while [ $# -gt 0 ]; do
    case "$1" in
        --interval)    INTERVAL_MIN="$2"; shift 2 ;;
        --max-cycles)  MAX_CYCLES="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [--interval MIN] [--max-cycles N]"
            echo "  Unified growth engine: σ=12→J₂=24 phases"
            echo "  Singleton + resource monitor + auto-expand"
            exit 0 ;;
        *) shift ;;
    esac
done

# ── Singleton (from common lib) ────────────────────────────────
singleton_acquire "$PIDFILE"

# ── Banner ──────────────────────────────────────────────────────
cat <<BANNER

  ╔═════════════════════════════════════════════════════════════╗
  ║  N6-ARCH UNIFIED INFINITE GROWTH ENGINE v3                 ║
  ║  PID: $$  Interval: ${INTERVAL_MIN}m  Max: $MAX_CYCLES cycles             ║
  ║  σ=12 base → J₂=24 full expansion                         ║
  ║  Singleton + ResourceMonitor + PaperLoop + AutoDomain      ║
  ╚═════════════════════════════════════════════════════════════╝

BANNER

# ═══════════════════════════════════════════════════════════════
# MAIN LOOP
# ═══════════════════════════════════════════════════════════════
ORIGINAL_INTERVAL=$INTERVAL_MIN

while [ "$CYCLE" -lt "$MAX_CYCLES" ]; do
    CYCLE=$((CYCLE + 1))
    START=$(date +%s)

    # ── Resource check ──────────────────────────────────────────
    RESOURCE_STATUS=$(check_resources)
    ACTIVE_PHASES=$(get_phase_count "$CYCLE")

    echo ""
    echo "╔═════════════════════════════════════════════════════════════╗"
    printf "║  CYCLE %d / %d — %s\n" "$CYCLE" "$MAX_CYCLES" "$(date '+%Y-%m-%d %H:%M:%S')"
    printf "║  Phases: %d  Resource: %-8s  Interval: %dm\n" "$ACTIVE_PHASES" "$RESOURCE_STATUS" "$INTERVAL_MIN"
    echo "╚═════════════════════════════════════════════════════════════╝"
    print_resources
    echo ""

    # Resource-based decisions
    if [ "$RESOURCE_STATUS" = "STOP" ]; then
        log_error "DISK < ${DISK_MIN_FREE_GB}GB — growth halted for safety"
        break
    fi
    SKIP_HEAVY=false
    if [ "$RESOURCE_STATUS" = "THROTTLE" ]; then
        log_warn "CPU > ${CPU_THROTTLE_PCT}% — doubling interval"
        INTERVAL_MIN=$((INTERVAL_MIN * 2))
    elif [ "$RESOURCE_STATUS" = "OK" ]; then
        # Reset throttle when resources recover
        INTERVAL_MIN=$ORIGINAL_INTERVAL
    fi
    if [ "$RESOURCE_STATUS" = "LIGHT" ]; then
        log_warn "MEM < ${MEM_MIN_FREE_MB}MB — skipping heavy phases"
        SKIP_HEAVY=true
    fi

    # ═══════════════════════════════════════════════════════════
    # TIER 1: Base Monitoring (Phase 1-12, always active)
    # ═══════════════════════════════════════════════════════════

    # === Phase 1: DSE Domain Scan ===
    run_phase 1 "$ACTIVE_PHASES" "DSE Domain Scan" "
        if [ -f '$DSE_MAP' ]; then
            DONE=\$(count_pattern '$DSE_MAP' 'dse.*=.*done')
            WIP=\$(count_pattern '$DSE_MAP' 'dse.*=.*wip')
            NONE=\$(count_pattern '$DSE_MAP' 'dse.*=.*none')
            TOTAL=\$(grep -c '^\[' '$DSE_MAP' 2>/dev/null || echo 0)
            TOTAL=\$((TOTAL - 1))
            echo \"  DSE: done=\$DONE wip=\$WIP none=\$NONE total=\$TOTAL\"
        else
            echo '  dse-map.toml not found'
        fi
    " 5

    # === Phase 2: BT Verification Status ===
    run_phase 2 "$ACTIVE_PHASES" "BT Verification" "
        if [ -f '$BT_FILE' ]; then
            BT_COUNT=\$(grep -c 'BT-[0-9]' '$BT_FILE' 2>/dev/null || echo 0)
            echo \"  BTs: \$BT_COUNT entries\"
        else
            echo '  breakthrough-theorems.md not found'
        fi
    " 3

    # === Phase 3: Technique Health ===
    if $SKIP_HEAVY; then
        echo "[$(log_ts)] Phase 3/$ACTIVE_PHASES: Technique Health — SKIPPED (low memory)"
    else
        run_phase 3 "$ACTIVE_PHASES" "Technique Health" "
            TECH_OK=0; TECH_FAIL=0; TECH_TOTAL=0
            for f in '$TECHNIQUES'/*.py; do
                [ -f \"\$f\" ] || continue
                TECH_TOTAL=\$((TECH_TOTAL + 1))
                basename_f=\$(basename \"\$f\" .py)
                if python3 -c \"import importlib.util; s=importlib.util.spec_from_file_location('\$basename_f','\$f'); m=importlib.util.module_from_spec(s)\" 2>/dev/null; then
                    TECH_OK=\$((TECH_OK + 1))
                else
                    TECH_FAIL=\$((TECH_FAIL + 1))
                    echo \"  FAIL: \$basename_f\"
                fi
            done
            echo \"  Techniques: \$TECH_OK/\$TECH_TOTAL OK\"
        " 8
    fi

    # === Phase 4: NEXUS-6 Telescope Status ===
    run_phase 4 "$ACTIVE_PHASES" "NEXUS-6 Telescope" "
        N6_BIN=\"\$HOME/Dev/nexus6\"
        if [ -d \"\$N6_BIN\" ]; then
            LENS_COUNT=\$(find \"\$N6_BIN/src/telescope\" -name '*.rs' 2>/dev/null | wc -l | tr -d ' ')
            echo \"  Lens files: \$LENS_COUNT\"
            [ -f \"\$HOME/.nexus6/lens_invariant_cores.json\" ] && echo '  Cores: present' || echo '  Cores: none'
            [ -f \"\$HOME/.nexus6/lens_elite.json\" ] && echo '  Elite: present' || echo '  Elite: none'
        else
            echo '  NEXUS-6 not found'
        fi
    " 5

    # === Phase 5: Testable Predictions ===
    run_phase 5 "$ACTIVE_PHASES" "Testable Predictions" "
        if [ -f '$TP_FILE' ]; then
            TP_TOTAL=\$(grep -c '|' '$TP_FILE' 2>/dev/null || echo 0)
            TIER1=\$(grep -c 'Tier 1\|tier 1' '$TP_FILE' 2>/dev/null || echo 0)
            TIER2=\$(grep -c 'Tier 2\|tier 2' '$TP_FILE' 2>/dev/null || echo 0)
            echo \"  Lines: \$TP_TOTAL, Tier1: \$TIER1, Tier2: \$TIER2\"
        fi
    " 4

    # === Phase 6: Growth Scan ===
    if $SKIP_HEAVY; then
        echo "[$(log_ts)] Phase 6/$ACTIVE_PHASES: Growth Scan — SKIPPED (low memory)"
    else
        run_phase 6 "$ACTIVE_PHASES" "Growth Scan" "
            if [ -f '$GROWTH_SCAN' ]; then
                python3 '$GROWTH_SCAN' 2>/dev/null || echo '  scan.py failed'
            else
                echo '  .growth/scan.py not found'
            fi
        " 10
    fi

    # === Phase 7: Cross-DSE Opportunities ===
    run_phase 7 "$ACTIVE_PHASES" "Cross-DSE" "
        if [ -f '$DSE_MAP' ]; then
            CROSS=\$(grep -c 'cross_dse' '$DSE_MAP' 2>/dev/null || echo 0)
            DONE=\$(count_pattern '$DSE_MAP' 'dse.*=.*done')
            GAP=\$((DONE > CROSS ? DONE - CROSS : 0))
            echo \"  Done: \$DONE, Cross-linked: \$CROSS, Gap: \$GAP\"
        fi
    " 4

    # === Phase 8: Atlas Sync ===
    run_phase 8 "$ACTIVE_PHASES" "Atlas Sync" "
        if [ -f '$ATLAS_FILE' ]; then
            ENTRIES=\$(grep -c '^|' '$ATLAS_FILE' 2>/dev/null || echo 0)
            echo \"  Atlas: ~\$ENTRIES entries\"
        fi
        SCANNER=\"\$HOME/Dev/TECS-L/.shared/scan_math_atlas.py\"
        [ -f \"\$SCANNER\" ] && python3 \"\$SCANNER\" --summary 2>/dev/null | tail -3 || true
    " 6

    # === Phase 9: Growth Bus Sync ===
    run_phase 9 "$ACTIVE_PHASES" "Growth Bus" "
        sync_growth_bus
    " 4

    # === Phase 10: Documentation Completeness ===
    run_phase 10 "$ACTIVE_PHASES" "Doc Completeness" "
        DOMAINS=0; HYP=0; GOAL=0; VERIFY=0; EXTREME=0
        for d in '$DOCS'/*/; do
            [ -d \"\$d\" ] || continue
            DOMAINS=\$((DOMAINS + 1))
            [ -f \"\${d}hypotheses.md\" ] && HYP=\$((HYP + 1))
            [ -f \"\${d}goal.md\" ] && GOAL=\$((GOAL + 1))
            [ -f \"\${d}verification.md\" ] && VERIFY=\$((VERIFY + 1))
            [ -f \"\${d}extreme-hypotheses.md\" ] && EXTREME=\$((EXTREME + 1))
        done
        echo \"  \$DOMAINS domains: hyp=\$HYP goal=\$GOAL verify=\$VERIFY extreme=\$EXTREME\"
    " 4

    # === Phase 11: Growth Tick ===
    run_phase 11 "$ACTIVE_PHASES" "Growth Tick" "
        update_growth_state $CYCLE
    " 3

    # === Phase 12: Auto-Commit (Tier 1) ===
    run_phase 12 "$ACTIVE_PHASES" "Auto-Commit" "
        growth_commit '.growth/' \"growth(n6-arch): cycle $CYCLE tier-1 monitoring\"
    " 3

    # ═══════════════════════════════════════════════════════════
    # TIER 2: NEXUS-6 Dimension Growth (Phase 13-15, cycle ≥7)
    # ═══════════════════════════════════════════════════════════
    if [ "$ACTIVE_PHASES" -ge 15 ]; then
        echo "  ── TIER 2: NEXUS-6 Dimension Growth ──"

        # === Phase 13: Weakest Dimension Growth ===
        if $SKIP_HEAVY; then
            echo "[$(log_ts)] Phase 13/$ACTIVE_PHASES: Dimension Growth — SKIPPED (low memory)"
        else
            run_phase 13 "$ACTIVE_PHASES" "NEXUS-6 Weakest Dimension" "
                run_nexus6_grow_dimension
            " 20
        fi

        # === Phase 14: NEXUS-6 Build Verify ===
        run_phase 14 "$ACTIVE_PHASES" "NEXUS-6 Build Check" "
            cd '$N6_ROOT/tools/nexus6'
            if ~/.cargo/bin/cargo check 2>&1 | tail -3; then
                echo '  cargo check: OK'
            else
                echo '  cargo check: FAIL — reverting'
                cd '$N6_ROOT' && git checkout -- tools/nexus6/src/ 2>/dev/null || true
            fi
        " 5

        # === Phase 15: Commit Dimension Growth ===
        run_phase 15 "$ACTIVE_PHASES" "Commit Growth" "
            growth_commit 'tools/nexus6/src/' \"growth(n6-arch): cycle $CYCLE nexus6-dimension\"
        " 3
    fi

    # ═══════════════════════════════════════════════════════════
    # TIER 3: Auto Domain + Doc Update + Paper (Phase 16-18, cycle ≥13)
    # ═══════════════════════════════════════════════════════════
    if [ "$ACTIVE_PHASES" -ge 18 ]; then
        echo "  ── TIER 3: Auto Domain + Doc + Paper ──"

        # === Phase 16: Auto Domain Explorer ===
        run_phase 16 "$ACTIVE_PHASES" "Auto Domain Explorer" "
            run_auto_domain_explore
        " 8

        # === Phase 17: Auto Document Update ===
        run_phase 17 "$ACTIVE_PHASES" "Auto Doc Update" "
            run_auto_doc_update
        " 10

        # === Phase 18: Paper Publish Loop ===
        run_phase 18 "$ACTIVE_PHASES" "Paper Publish Loop" "
            run_paper_loop
        " 8
    fi

    # ═══════════════════════════════════════════════════════════
    # TIER 4: Deep Analysis + Evolution (Phase 19-24, cycle ≥25)
    # ═══════════════════════════════════════════════════════════
    if [ "$ACTIVE_PHASES" -ge 24 ]; then
        echo "  ── TIER 4: Deep Analysis + Evolution ──"

        # === Phase 19: Cross-Resonance Deep Scan ===
        run_phase 19 "$ACTIVE_PHASES" "Cross-Resonance Deep" "
            if ls '$DOCS'/cross-domain-resonance*.md >/dev/null 2>&1; then
                RESONANCE=\$(grep -ciE 'resonance|bridge|cross.domain' '$DOCS'/cross-domain-resonance*.md 2>/dev/null || echo 0)
                echo \"  Cross-resonance entries: \$RESONANCE\"
            fi
        " 4

        # === Phase 20: Blowup Engine ===
        if $SKIP_HEAVY; then
            echo "[$(log_ts)] Phase 20/$ACTIVE_PHASES: Blowup — SKIPPED (low memory)"
        else
            run_phase 20 "$ACTIVE_PHASES" "Blowup Engine" "
                if [ -f '$N6_ROOT/tools/nexus6/scripts/growth_infinite_lens.py' ]; then
                    python3 '$N6_ROOT/tools/nexus6/scripts/growth_infinite_lens.py' 리포트 2>/dev/null | tail -10 || echo '  Blowup report: unavailable'
                fi
            " 10
        fi

        # === Phase 21: NEXUS-6 Health Dashboard ===
        run_phase 21 "$ACTIVE_PHASES" "NEXUS-6 Dashboard" "
            if [ -f '$NEXUS6_SCRIPTS/growth_dashboard.sh' ]; then
                bash '$NEXUS6_SCRIPTS/growth_dashboard.sh' 2>/dev/null | tail -20 || echo '  Dashboard: unavailable'
            fi
        " 20

        # === Phase 22: Evolution Candidates ===
        run_phase 22 "$ACTIVE_PHASES" "Evolution Candidates" "
            # Check for domains that could evolve (high BT count + DSE done)
            if [ -f '$BT_FILE' ] && [ -f '$DSE_MAP' ]; then
                DONE=\$(count_pattern '$DSE_MAP' 'dse.*=.*done')
                BTS=\$(grep -oE 'BT-[0-9]+' '$BT_FILE' 2>/dev/null | sort -u | wc -l | tr -d ' ')
                echo \"  Evolution pool: \$DONE DSE-done domains, \$BTS unique BTs\"
                # Check for Mk evolution directories
                MK_COUNT=\$(find '$DOCS' -name 'mk-*-*.md' 2>/dev/null | wc -l | tr -d ' ')
                echo \"  Mk documents: \$MK_COUNT\"
            fi
        " 5

        # === Phase 23: Full Sync Sweep ===
        run_phase 23 "$ACTIVE_PHASES" "Full Sync Sweep" "
            # Sync NEXUS-6 lenses
            [ -f '$N6_ROOT/.shared/sync-nexus6-lenses.sh' ] && bash '$N6_ROOT/.shared/sync-nexus6-lenses.sh' 2>/dev/null | tail -3 || true
            # Sync calculators
            [ -f '$N6_ROOT/.shared/sync-calculators.sh' ] && bash '$N6_ROOT/.shared/sync-calculators.sh' 2>/dev/null | tail -3 || true
            echo '  Sync sweep complete'
        " 6

        # === Phase 24: Final Commit (J₂=24) ===
        run_phase 24 "$ACTIVE_PHASES" "Final Commit (J₂=24)" "
            growth_commit '.' \"growth(n6-arch): cycle $CYCLE J₂=24 full expansion\"
        " 3
    fi

    # ═══════════════════════════════════════════════════════════
    # CYCLE SUMMARY
    # ═══════════════════════════════════════════════════════════
    END=$(date +%s)
    DUR=$((END - START))
    GROWTH_NOW=$(python3 -c "import json; print(json.load(open('$GROWTH_STATE')).get('total_growth', 0))" 2>/dev/null || echo "?")
    SCAN_NOW=$(python3 -c "import json; print(json.load(open('$GROWTH_STATE')).get('scan_count', 0))" 2>/dev/null || echo "?")

    echo ""
    echo "╔═════════════════════════════════════════════════════════════╗"
    printf "║  Cycle %-4s  %3ds  Phases: %-2s  Growth: %-6s  Scans: %-4s ║\n" \
        "$CYCLE" "$DUR" "$ACTIVE_PHASES" "$GROWTH_NOW" "$SCAN_NOW"
    printf "║  Resource: %-8s  CPU: %d%%  MEM: %dMB  DISK: %dGB       ║\n" \
        "$RESOURCE_STATUS" "${_RES_CPU:-0}" "${_RES_MEM:-0}" "${_RES_DISK:-0}"
    echo "╚═════════════════════════════════════════════════════════════╝"
    echo ""

    # ── Interruptible sleep ─────────────────────────────────────
    if [ "$CYCLE" -lt "$MAX_CYCLES" ]; then
        log_info "Sleeping ${INTERVAL_MIN}m..."
        sleep $((INTERVAL_MIN * 60)) &
        wait $! 2>/dev/null || true
    fi
done

echo ""
echo "Infinite growth complete: $CYCLE cycles"
