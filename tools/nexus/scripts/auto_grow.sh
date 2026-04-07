#!/usr/bin/env bash
# NEXUS-6 Auto-Growth System
# Usage: ./auto_grow.sh [--cycles N] [--dry-run] [--skip-commit]
#
# Each cycle:
#   1. Measure  — cargo test + benchmark + metrics
#   2. Analyze  — identify growth opportunities
#   3. Generate — use Claude Code CLI to create improvements
#   4. Validate — cargo check + cargo test
#   5. Commit   — auto-commit if tests pass
#   6. Report   — log growth metrics
set -euo pipefail

CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"
NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SCRIPT_DIR="$NEXUS_ROOT/scripts"
LOG_FILE="$SCRIPT_DIR/growth_log.jsonl"
REPO_ROOT="$(cd "$NEXUS_ROOT/../.." && pwd)"

cd "$NEXUS_ROOT"

# --- Parse arguments ---
CYCLES=1
DRY_RUN=false
SKIP_COMMIT=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --cycles)    CYCLES="$2"; shift 2 ;;
        --dry-run)   DRY_RUN=true; shift ;;
        --skip-commit) SKIP_COMMIT=true; shift ;;
        -h|--help)
            echo "NEXUS-6 Auto-Growth System"
            echo ""
            echo "Usage: $0 [--cycles N] [--dry-run] [--skip-commit]"
            echo ""
            echo "Options:"
            echo "  --cycles N       Number of growth cycles (default: 1)"
            echo "  --dry-run        Run analysis only, no code generation or commits"
            echo "  --skip-commit    Generate code but don't commit"
            echo ""
            exit 0
            ;;
        *) echo "Unknown flag: $1"; exit 1 ;;
    esac
done

# --- Utility functions ---

log_info()  { echo "[$(date +%H:%M:%S)] INFO:  $*"; }
log_warn()  { echo "[$(date +%H:%M:%S)] WARN:  $*"; }
log_error() { echo "[$(date +%H:%M:%S)] ERROR: $*"; }

collect_metrics() {
    # Returns JSON metrics on stdout
    bash "$SCRIPT_DIR/measure.sh" 2>/dev/null
}

count_cargo_warnings() {
    ~/.cargo/bin/cargo check 2>&1 | grep -c "^warning\[" || echo "0"
}

find_unimplemented_lenses() {
    # Find lenses in registry that have no scan() implementation
    # Look for LensEntry names that don't have corresponding _lens.rs with impl Lens
    local registry_names
    registry_names=$(grep -h "name:" src/telescope/*.rs 2>/dev/null | \
        grep -oP '"[^"]+_lens"' | tr -d '"' | sort -u || true)

    local impl_names
    impl_names=$(find src/telescope/lenses -name "*_lens.rs" -exec grep -l "impl Lens for" {} \; 2>/dev/null | \
        xargs -I{} basename {} .rs | sort -u || true)

    # Lenses that are in registry metadata files but lack impl
    comm -23 <(echo "$registry_names" | sort) <(echo "$impl_names" | sort) 2>/dev/null | head -5
}

find_modules_needing_tests() {
    # Find modules with lowest test-to-code ratio
    local worst_module=""
    local worst_ratio=999999

    for mod_dir in src/*/; do
        local mod
        mod=$(basename "$mod_dir")
        local code_lines
        code_lines=$(find "$mod_dir" -name "*.rs" -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
        [[ "$code_lines" -lt 50 ]] && continue

        local test_count
        test_count=$(grep -r "#\[test\]" "$mod_dir" 2>/dev/null | wc -l | tr -d ' ')
        local ratio=$((test_count * 100 / code_lines))

        if [[ "$ratio" -lt "$worst_ratio" ]]; then
            worst_ratio=$ratio
            worst_module=$mod
        fi
    done

    echo "$worst_module"
}

select_growth_action() {
    local warnings="$1"
    local tests_fail="$2"

    # Priority order:
    # 1. Fix compilation/test failures first
    if [[ "$tests_fail" -gt 0 ]]; then
        echo "fix_tests"
        return
    fi

    # 2. Fix warnings if many
    if [[ "$warnings" -gt 5 ]]; then
        echo "fix_warnings"
        return
    fi

    # 3. Implement unimplemented lens (if any metadata-only lenses exist)
    local unimpl
    unimpl=$(find_unimplemented_lenses | head -1)
    if [[ -n "$unimpl" ]]; then
        echo "impl_lens:$unimpl"
        return
    fi

    # 4. Add tests to under-tested modules
    local needs_tests
    needs_tests=$(find_modules_needing_tests)
    if [[ -n "$needs_tests" ]]; then
        echo "add_tests:$needs_tests"
        return
    fi

    # 5. Fix remaining warnings
    if [[ "$warnings" -gt 0 ]]; then
        echo "fix_warnings"
        return
    fi

    # 6. Default: add tests
    echo "add_tests:graph"
}

execute_growth_action() {
    local action="$1"
    local action_type="${action%%:*}"
    local action_target="${action#*:}"

    case "$action_type" in
        fix_tests)
            log_info "Action: Fix failing tests"
            local prompt="In the NEXUS-6 project at $NEXUS_ROOT, some tests are failing. \
Read the test output from 'cargo test', identify the failures, and fix them. \
Only fix test-related issues. Do not change core logic unless a bug is obvious. \
Run 'cargo test' to verify your fixes."
            $CLAUDE_CLI -p "$prompt" --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
            ;;

        fix_warnings)
            log_info "Action: Fix compiler warnings"
            local prompt="In the NEXUS-6 project at $NEXUS_ROOT, fix compiler warnings. \
Run 'cargo check 2>&1' to see warnings, then fix them (unused imports, dead code, etc.). \
Do NOT remove any public API — only clean up warnings. Run 'cargo check' to verify."
            $CLAUDE_CLI -p "$prompt" --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
            ;;

        impl_lens)
            log_info "Action: Implement lens '$action_target'"
            bash "$SCRIPT_DIR/grow_lens.sh" "$action_target" || return 1
            ;;

        add_tests)
            log_info "Action: Add tests to module '$action_target'"
            bash "$SCRIPT_DIR/grow_tests.sh" "$action_target" || return 1
            ;;

        *)
            log_warn "Unknown action: $action"
            return 1
            ;;
    esac
}

# --- Main loop ---

echo ""
echo "  ╔══════════════════════════════════════════╗"
echo "  ║   NEXUS-6 Auto-Growth System             ║"
echo "  ║   Cycles: $CYCLES  Dry-run: $DRY_RUN                ║"
echo "  ╚══════════════════════════════════════════╝"
echo ""

for cycle in $(seq 1 "$CYCLES"); do
    echo ""
    echo "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Cycle $cycle / $CYCLES"
    echo "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # --- Step 1: Measure ---
    log_info "Step 1/6: Measuring current state..."
    before_metrics=$(collect_metrics)
    before_tests=$(echo "$before_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tests_pass',0))" 2>/dev/null || echo "0")
    before_warnings=$(echo "$before_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('warnings',0))" 2>/dev/null || echo "0")
    before_lines=$(echo "$before_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('code_lines',0))" 2>/dev/null || echo "0")
    before_lenses=$(echo "$before_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('lenses_registered',0))" 2>/dev/null || echo "0")
    tests_fail=$(echo "$before_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tests_fail',0))" 2>/dev/null || echo "0")

    log_info "  Tests: $before_tests pass, $tests_fail fail | Warnings: $before_warnings | Lines: $before_lines | Lenses: $before_lenses"

    # --- Step 2: Analyze ---
    log_info "Step 2/6: Analyzing growth opportunities..."
    action=$(select_growth_action "$before_warnings" "$tests_fail")
    action_type="${action%%:*}"
    log_info "  Selected action: $action"

    if $DRY_RUN; then
        log_info "[DRY-RUN] Would execute: $action"
        log_info "[DRY-RUN] Skipping generation, validation, commit."

        # Log dry-run metrics
        echo "$before_metrics" | python3 -c "
import sys, json
d = json.load(sys.stdin)
d['cycle'] = $cycle
d['action'] = 'dry-run: $action_type'
d['success'] = True
print(json.dumps(d))
" >> "$LOG_FILE" 2>/dev/null || true

        continue
    fi

    # --- Step 3: Generate ---
    log_info "Step 3/6: Generating improvements..."
    gen_ok=true
    if ! execute_growth_action "$action"; then
        log_warn "Generation step failed. Skipping this cycle."
        gen_ok=false
    fi

    # --- Step 4: Validate ---
    log_info "Step 4/6: Validating changes..."
    validate_ok=true

    if $gen_ok; then
        if ! ~/.cargo/bin/cargo check 2>&1 | tail -5; then
            log_error "cargo check failed after generation."
            validate_ok=false
        fi

        if $validate_ok; then
            test_output=$(~/.cargo/bin/cargo test 2>&1) || true
            if echo "$test_output" | grep -q "FAILED"; then
                log_error "cargo test failed after generation."
                validate_ok=false
            fi
        fi
    else
        validate_ok=false
    fi

    # Revert on failure
    if ! $validate_ok; then
        log_warn "Reverting changes due to validation failure..."
        (cd "$REPO_ROOT" && git checkout -- tools/nexus/src/) 2>/dev/null || true
        log_info "Changes reverted."
    fi

    # --- Step 5: Commit ---
    if $validate_ok && ! $SKIP_COMMIT; then
        log_info "Step 5/6: Committing changes..."

        # Collect after metrics
        after_metrics=$(collect_metrics)
        after_tests=$(echo "$after_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tests_pass',0))" 2>/dev/null || echo "0")

        commit_msg="growth(nexus): ${action_type} — cycle ${cycle}, tests ${before_tests}->${after_tests}"

        (cd "$REPO_ROOT" && git add tools/nexus/src/ && git commit -m "$commit_msg") 2>/dev/null || {
            log_warn "Nothing to commit or commit failed."
        }
    elif $validate_ok && $SKIP_COMMIT; then
        log_info "Step 5/6: Skipping commit (--skip-commit)"
    else
        log_info "Step 5/6: Skipping commit (validation failed)"
    fi

    # --- Step 6: Report ---
    log_info "Step 6/6: Logging growth metrics..."

    after_metrics=$(collect_metrics)
    # Append cycle info to log
    echo "$after_metrics" | python3 -c "
import sys, json
d = json.load(sys.stdin)
d['cycle'] = $cycle
d['action'] = '$action_type'
d['success'] = $( $validate_ok && echo "true" || echo "false" )
d['before_tests'] = $before_tests
d['before_lines'] = $before_lines
print(json.dumps(d))
" >> "$LOG_FILE" 2>/dev/null || true

    if $validate_ok; then
        after_tests_final=$(echo "$after_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tests_pass',0))" 2>/dev/null || echo "0")
        after_lines_final=$(echo "$after_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('code_lines',0))" 2>/dev/null || echo "0")
        log_info "  Result: tests $before_tests -> $after_tests_final, lines $before_lines -> $after_lines_final"
    else
        log_info "  Result: cycle failed, no changes applied."
    fi
done

# --- Final report ---
echo ""
echo "  ╔══════════════════════════════════════════╗"
echo "  ║   Growth Complete: $CYCLES cycle(s)              ║"
echo "  ╚══════════════════════════════════════════╝"
echo ""

if [[ -f "$LOG_FILE" ]]; then
    bash "$SCRIPT_DIR/growth_report.sh" --last "$CYCLES"
fi
