#!/usr/bin/env bash
set -euo pipefail

CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"
# ═══════════════════════════════════════════════════════════════════════
# NEXUS-6 Architecture Growth
# Usage: ./grow_architecture.sh [--dry-run] [--max-actions N]
#
# Discovers architectural gaps and uses Claude Code CLI to fix them.
# This is the "brain" of NEXUS-6's self-evolution.
#
# Actions it can take:
#   - Implement stub modules (< 50 lines)
#   - Add cross-module integration between disconnected modules
#   - Create new modules for missing capabilities
#   - Refactor bottleneck hub modules
#   - Add integration tests spanning multiple modules
#   - Upgrade module maturity (Basic -> Mature)
# ═══════════════════════════════════════════════════════════════════════

NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SCRIPT_DIR="$NEXUS_ROOT/scripts"
LOG_FILE="$SCRIPT_DIR/growth_log.jsonl"
REPO_ROOT="$(cd "$NEXUS_ROOT/../.." && pwd)"

cd "$NEXUS_ROOT"

# --- Parse arguments ---
DRY_RUN=false
MAX_ACTIONS=1

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)      DRY_RUN=true; shift ;;
        --max-actions)  MAX_ACTIONS="$2"; shift 2 ;;
        -h|--help)
            echo "NEXUS-6 Architecture Growth"
            echo ""
            echo "Usage: $0 [--dry-run] [--max-actions N]"
            echo ""
            echo "Options:"
            echo "  --dry-run        Analyze only, no code generation"
            echo "  --max-actions N  Max architectural changes per run (default: 1)"
            exit 0
            ;;
        *) echo "Unknown flag: $1"; exit 1 ;;
    esac
done

# --- n=6 constants ---
N=6
SIGMA=12
PHI=2
TAU=4
SOPFR=5

# --- Utility functions ---

log_info()  { echo "[$(date +%H:%M:%S)] INFO:  $*"; }
log_warn()  { echo "[$(date +%H:%M:%S)] WARN:  $*"; }
log_error() { echo "[$(date +%H:%M:%S)] ERROR: $*"; }

timestamp_iso() { date -u +"%Y-%m-%dT%H:%M:%SZ"; }

# --- Priority architectural growth targets (in order) ---
# Format: "type:target" where type is stub|integration|capability|refactor|tests|upgrade
ARCH_PRIORITIES=(
    "stub:cross_intel"                      # Minimal stub, needs full implementation
    "stub:statistics"                       # Needs statistical analysis functions
    "stub:dream"                            # Dream mode barely implemented
    "stub:consciousness_bridge"             # Consciousness bridge is a stub
    "stub:time_travel"                      # Time travel snapshots minimal
    "stub:versioning"                       # Versioning barely there
    "integration:calibration+telescope"     # Calibration should auto-calibrate telescope lenses
    "integration:growth+ouroboros"          # Growth should feed into evolution engine
    "integration:simulation+experiment"     # Simulation should drive experiment design
    "integration:reward+genetic_prog"       # Reward should guide genetic evolution
    "integration:feedback+growth"           # Feedback should inform growth decisions
    "integration:self_improve+growth"       # Self-improvement ties into growth
    "integration:statistics+experiment"     # Statistics should analyze experiment results
    "integration:event+alert"              # Events should trigger alerts
    "integration:dream+consciousness_bridge"  # Dream feeds consciousness
    "integration:versioning+history"       # Versioning tracks history
    "integration:scheduler+autonomous"     # Scheduler drives autonomous mode
    "integration:sandbox+red_team"         # Sandbox for red-team isolation
    "integration:nlp+cli"                  # NLP enhances CLI
    "integration:multi_agent+distributed"  # Multi-agent uses distributed infra
    "capability:regression_detection"       # Missing: detect performance regressions
    "capability:cross_module_testing"       # Missing: integration test framework
    "capability:code_generation"            # Missing: auto code generation
    "capability:api_documentation"          # Missing: auto API docs
    "capability:architecture_visualization" # Missing: ASCII/DOT arch diagrams
    "capability:web_dashboard"              # Missing: real-time growth dashboard
    "tests:feedback"                        # Under-tested
    "tests:reward"                          # Under-tested
    "tests:genetic_prog"                    # Under-tested
    "tests:scheduler"                       # Under-tested
    "refactor:telescope"                    # Hub module, too many connections
)

# ═══════════════════════════════════════════════════════════════════════
# Step 1: DISCOVER — Analyze current architecture
# ═══════════════════════════════════════════════════════════════════════

discover_gaps() {
    log_info "Step 1: Discovering architectural gaps..."

    local stub_modules=()
    local untested_modules=()
    local orphan_modules=()
    local total_modules=0
    local total_tests=0
    local total_lines=0

    for mod_dir in src/*/; do
        local mod_name
        mod_name=$(basename "$mod_dir")
        total_modules=$((total_modules + 1))

        # Count lines
        local lines
        lines=$(find "$mod_dir" -name "*.rs" -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
        total_lines=$((total_lines + lines))

        # Count tests
        local tests
        tests=$(grep -r "#\[test\]" "$mod_dir" 2>/dev/null | wc -l | tr -d ' ')
        total_tests=$((total_tests + tests))

        # Stub detection: < 50 lines total
        if [[ "$lines" -lt 50 ]]; then
            stub_modules+=("$mod_name:${lines}L:${tests}T")
        fi

        # Under-tested: has code (> 50 lines) but < SOPFR tests
        if [[ "$lines" -gt 50 ]] && [[ "$tests" -lt "$SOPFR" ]]; then
            untested_modules+=("$mod_name:${lines}L:${tests}T")
        fi

        # Orphan detection: check if any other module imports this one
        local imported_by
        imported_by=$(grep -rl "crate::${mod_name}" src/ 2>/dev/null | grep -v "src/${mod_name}/" | wc -l | tr -d ' ')
        local imports
        imports=$(grep -r "crate::" "src/${mod_name}/" 2>/dev/null | grep -v "crate::${mod_name}" | wc -l | tr -d ' ')

        if [[ "$imported_by" -eq 0 ]] && [[ "$imports" -eq 0 ]] && [[ "$mod_name" != "cli" ]] && [[ "$mod_name" != "growth" ]]; then
            orphan_modules+=("$mod_name")
        fi
    done

    echo ""
    echo "  ┌──────────────────────────────────────────────────┐"
    echo "  │       Architecture Discovery Results             │"
    echo "  ├──────────────────────────────────────────────────┤"
    printf "  │  Total modules:      %4d                        │\n" "$total_modules"
    printf "  │  Total lines:        %6d                      │\n" "$total_lines"
    printf "  │  Total tests:        %4d                        │\n" "$total_tests"
    printf "  │  Stub modules:       %4d                        │\n" "${#stub_modules[@]}"
    printf "  │  Under-tested:       %4d                        │\n" "${#untested_modules[@]}"
    printf "  │  Orphan modules:     %4d                        │\n" "${#orphan_modules[@]}"
    echo "  └──────────────────────────────────────────────────┘"

    if [[ ${#stub_modules[@]} -gt 0 ]]; then
        echo ""
        echo "  Stub modules (< 50 lines):"
        for s in "${stub_modules[@]}"; do
            echo "    - $s"
        done
    fi

    if [[ ${#orphan_modules[@]} -gt 0 ]]; then
        echo ""
        echo "  Orphan modules (no connections):"
        for o in "${orphan_modules[@]}"; do
            echo "    - $o"
        done
    fi

    # Export for later steps
    DISCOVERED_STUBS=("${stub_modules[@]}")
    DISCOVERED_UNTESTED=("${untested_modules[@]}")
    DISCOVERED_ORPHANS=("${orphan_modules[@]}")
    TOTAL_MODULES=$total_modules
    TOTAL_LINES=$total_lines
    TOTAL_TESTS=$total_tests
}

# ═══════════════════════════════════════════════════════════════════════
# Step 2: PRIORITIZE — Pick the best action from priority list
# ═══════════════════════════════════════════════════════════════════════

select_architecture_action() {
    log_info "Step 2: Selecting architecture action..."

    # Walk the priority list and find the first actionable item
    for priority in "${ARCH_PRIORITIES[@]}"; do
        local ptype="${priority%%:*}"
        local ptarget="${priority#*:}"

        case "$ptype" in
            stub)
                # Check if this module is actually still a stub
                if [[ -d "src/$ptarget" ]]; then
                    local lines
                    lines=$(find "src/$ptarget" -name "*.rs" -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
                    if [[ "$lines" -lt 50 ]]; then
                        SELECTED_ACTION="stub"
                        SELECTED_TARGET="$ptarget"
                        SELECTED_LINES="$lines"
                        log_info "  Selected: implement stub '$ptarget' ($lines lines)"
                        return 0
                    fi
                fi
                ;;
            integration)
                # Parse "moduleA+moduleB"
                local mod_a="${ptarget%%+*}"
                local mod_b="${ptarget#*+}"
                # Check if integration already exists
                local has_link
                has_link=$(grep -r "crate::${mod_b}" "src/${mod_a}/" 2>/dev/null | wc -l | tr -d ' ')
                local has_reverse
                has_reverse=$(grep -r "crate::${mod_a}" "src/${mod_b}/" 2>/dev/null | wc -l | tr -d ' ')
                if [[ "$has_link" -eq 0 ]] && [[ "$has_reverse" -eq 0 ]]; then
                    SELECTED_ACTION="integration"
                    SELECTED_TARGET="$ptarget"
                    SELECTED_MOD_A="$mod_a"
                    SELECTED_MOD_B="$mod_b"
                    log_info "  Selected: integrate '$mod_a' <-> '$mod_b'"
                    return 0
                fi
                ;;
            capability)
                # Capabilities are always actionable (we'd need to check if file exists)
                SELECTED_ACTION="capability"
                SELECTED_TARGET="$ptarget"
                log_info "  Selected: add capability '$ptarget'"
                return 0
                ;;
            tests)
                local tests
                tests=$(grep -r "#\[test\]" "src/$ptarget/" 2>/dev/null | wc -l | tr -d ' ')
                if [[ "$tests" -lt "$SOPFR" ]]; then
                    SELECTED_ACTION="tests"
                    SELECTED_TARGET="$ptarget"
                    SELECTED_TESTS="$tests"
                    log_info "  Selected: add tests to '$ptarget' (currently $tests)"
                    return 0
                fi
                ;;
            refactor)
                # Check connection count
                local conn_count
                conn_count=$(grep -rl "crate::${ptarget}" src/ 2>/dev/null | wc -l | tr -d ' ')
                if [[ "$conn_count" -ge "$SIGMA" ]]; then
                    SELECTED_ACTION="refactor"
                    SELECTED_TARGET="$ptarget"
                    log_info "  Selected: refactor bottleneck '$ptarget' ($conn_count connections)"
                    return 0
                fi
                ;;
        esac
    done

    # Fallback: find the first stub from discovery
    if [[ ${#DISCOVERED_STUBS[@]} -gt 0 ]]; then
        local first="${DISCOVERED_STUBS[0]}"
        SELECTED_ACTION="stub"
        SELECTED_TARGET="${first%%:*}"
        SELECTED_LINES="${first#*:}"
        SELECTED_LINES="${SELECTED_LINES%%L*}"
        log_info "  Fallback: implement stub '${SELECTED_TARGET}'"
        return 0
    fi

    # Nothing to do
    SELECTED_ACTION="none"
    log_info "  No architectural gaps found. System is healthy."
    return 1
}

# ═══════════════════════════════════════════════════════════════════════
# Step 3: GENERATE — Use Claude Code CLI to implement the fix
# ═══════════════════════════════════════════════════════════════════════

generate_fix() {
    log_info "Step 3: Generating fix via Claude Code CLI..."

    local prompt=""

    case "$SELECTED_ACTION" in
        stub)
            prompt="In the NEXUS-6 project at $NEXUS_ROOT, fully implement the '${SELECTED_TARGET}' module. \
Currently it has only ~${SELECTED_LINES:-20} lines and is a stub. \
Read the existing files in src/${SELECTED_TARGET}/ to understand the current state. \
It should provide meaningful functionality as described by its name. \
Add at least $SOPFR unit tests in a #[cfg(test)] mod tests block. \
Follow the existing code style: n=6 constants (N=6, SIGMA=12, PHI=2, TAU=4, J2=24, SOPFR=5), \
doc comments on all public items, mod.rs re-exports. \
Do NOT break any existing public API. \
After changes, run: cd $NEXUS_ROOT && ~/.cargo/bin/cargo check && ~/.cargo/bin/cargo test"
            ;;

        integration)
            local desc_a desc_b
            desc_a=$(describe_module "$SELECTED_MOD_A")
            desc_b=$(describe_module "$SELECTED_MOD_B")
            prompt="In the NEXUS-6 project at $NEXUS_ROOT, add integration between '${SELECTED_MOD_A}' and '${SELECTED_MOD_B}'. \
Module '${SELECTED_MOD_A}': ${desc_a}. \
Module '${SELECTED_MOD_B}': ${desc_b}. \
Read the source files in src/${SELECTED_MOD_A}/ and src/${SELECTED_MOD_B}/ first. \
Create bridge functions that allow '${SELECTED_MOD_A}' to leverage '${SELECTED_MOD_B}' capabilities (or vice versa). \
Add at least $PHI integration tests. Update mod.rs re-exports as needed. \
After changes, run: cd $NEXUS_ROOT && ~/.cargo/bin/cargo check && ~/.cargo/bin/cargo test"
            ;;

        capability)
            prompt="In the NEXUS-6 project at $NEXUS_ROOT, create or extend a module to provide the \
'${SELECTED_TARGET}' capability. This capability is currently missing from NEXUS-6. \
If the capability maps to an existing module, extend it. Otherwise create a new sub-module. \
Implement at least $SOPFR public functions and $SOPFR unit tests. \
Follow the existing code style: n=6 constants, doc comments, mod.rs re-exports. \
After changes, run: cd $NEXUS_ROOT && ~/.cargo/bin/cargo check && ~/.cargo/bin/cargo test"
            ;;

        tests)
            prompt="In the NEXUS-6 project at $NEXUS_ROOT, the '${SELECTED_TARGET}' module has only \
${SELECTED_TESTS:-0} tests. Add at least $((SOPFR - ${SELECTED_TESTS:-0})) more unit tests \
to bring it to at least $SOPFR total. Read the source in src/${SELECTED_TARGET}/ first. \
Cover edge cases, error paths, and integration with modules it uses. \
Use #[test] functions in a mod tests block. \
After changes, run: cd $NEXUS_ROOT && ~/.cargo/bin/cargo check && ~/.cargo/bin/cargo test"
            ;;

        refactor)
            prompt="In the NEXUS-6 project at $NEXUS_ROOT, the '${SELECTED_TARGET}' module is a bottleneck \
with too many connections (threshold: sigma=$SIGMA). Refactor it: \
1. Read src/${SELECTED_TARGET}/ to identify distinct responsibilities. \
2. Extract sub-modules for each responsibility. \
3. Create a facade mod.rs that re-exports the split pieces. \
4. Ensure all existing public API items remain accessible. \
5. Add tests to verify the refactoring preserves behaviour. \
After changes, run: cd $NEXUS_ROOT && ~/.cargo/bin/cargo check && ~/.cargo/bin/cargo test"
            ;;

        *)
            log_warn "No action to generate."
            return 1
            ;;
    esac

    if $DRY_RUN; then
        echo ""
        echo "  [DRY-RUN] Would execute Claude CLI with prompt:"
        echo "  ────────────────────────────────────────────────"
        echo "  $prompt" | fold -w 80 -s | sed 's/^/  │ /'
        echo "  ────────────────────────────────────────────────"
        return 0
    fi

    # Execute Claude Code CLI
    log_info "  Invoking Claude Code CLI..."
    if $CLAUDE_CLI -p "$prompt" --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null; then
        log_info "  Claude Code CLI completed successfully."
        return 0
    else
        log_error "  Claude Code CLI failed."
        return 1
    fi
}

describe_module() {
    case "$1" in
        calibration)  echo "Lens calibration using synthetic and real datasets" ;;
        telescope)    echo "Core telescope with 22+ lenses, registry, consensus" ;;
        growth)       echo "Auto-growth system — metrics, benchmarks, tracking" ;;
        ouroboros)    echo "OUROBOROS evolution engine — self-improvement loop" ;;
        simulation)   echo "Simulation engine for what-if experiments" ;;
        experiment)   echo "Experiment runner with design and result tracking" ;;
        reward)       echo "Reward signal computation for RL-guided evolution" ;;
        genetic_prog) echo "Genetic programming for evolving scan strategies" ;;
        feedback)     echo "Feedback loop collection and processing" ;;
        self_improve) echo "Self-improvement analysis and suggestion" ;;
        statistics)   echo "Statistical analysis and hypothesis testing" ;;
        event)        echo "Event bus for inter-module communication" ;;
        alert)        echo "Real-time alerting for anomalies" ;;
        dream)        echo "Background pattern discovery during idle" ;;
        consciousness_bridge) echo "Bridge to consciousness layer" ;;
        versioning)   echo "Semantic versioning and compatibility tracking" ;;
        history)      echo "Versioned history tracking" ;;
        scheduler)    echo "Task scheduling and queue management" ;;
        autonomous)   echo "Autonomous operation mode" ;;
        sandbox)      echo "Sandboxed execution environment" ;;
        red_team)     echo "Red-team adversarial testing" ;;
        nlp)          echo "Natural language query interface" ;;
        cli)          echo "Command-line interface" ;;
        multi_agent)  echo "Multi-agent coordination" ;;
        distributed)  echo "Distributed scan execution" ;;
        *)            echo "NEXUS-6 module" ;;
    esac
}

# ═══════════════════════════════════════════════════════════════════════
# Step 4: VALIDATE — cargo check + cargo test
# ═══════════════════════════════════════════════════════════════════════

validate_changes() {
    if $DRY_RUN; then
        log_info "Step 4: [DRY-RUN] Skipping validation."
        return 0
    fi

    log_info "Step 4: Validating changes..."

    # cargo check
    if ! ~/.cargo/bin/cargo check 2>&1 | tail -5; then
        log_error "  cargo check failed."
        return 1
    fi
    log_info "  cargo check passed."

    # cargo test
    local test_output
    test_output=$(~/.cargo/bin/cargo test 2>&1) || true
    if echo "$test_output" | grep -q "FAILED"; then
        log_error "  cargo test failed."
        echo "$test_output" | grep "FAILED" | head -5
        return 1
    fi

    local passed
    passed=$(echo "$test_output" | grep "test result:" | grep -oP '\d+ passed' | head -1 || echo "? passed")
    log_info "  cargo test passed ($passed)."
    return 0
}

# ═══════════════════════════════════════════════════════════════════════
# Step 5: COMMIT or ROLLBACK
# ═══════════════════════════════════════════════════════════════════════

commit_or_rollback() {
    local success="$1"

    if $DRY_RUN; then
        log_info "Step 5: [DRY-RUN] Skipping commit."
        return 0
    fi

    if [[ "$success" == "true" ]]; then
        log_info "Step 5: Committing changes..."
        local commit_msg="growth(nexus): arch-${SELECTED_ACTION} ${SELECTED_TARGET}"
        (cd "$REPO_ROOT" && git add tools/nexus/src/ && git commit -m "$commit_msg") 2>/dev/null || {
            log_warn "  Nothing to commit or commit failed."
        }
    else
        log_warn "Step 5: Rolling back changes..."
        (cd "$REPO_ROOT" && git checkout -- tools/nexus/src/) 2>/dev/null || true
        log_info "  Changes reverted."
    fi
}

# ═══════════════════════════════════════════════════════════════════════
# Step 6: LOG — Append to growth_log.jsonl
# ═══════════════════════════════════════════════════════════════════════

log_result() {
    local success="$1"

    log_info "Step 6: Logging result..."

    local after_lines
    after_lines=$(find src/ -name "*.rs" -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
    local after_tests
    after_tests=$(grep -r "#\[test\]" src/ 2>/dev/null | wc -l | tr -d ' ')

    local entry
    entry=$(cat <<JSONEOF
{"timestamp":"$(timestamp_iso)","action_type":"architecture","action":"${SELECTED_ACTION}","target":"${SELECTED_TARGET}","success":${success},"before_lines":${TOTAL_LINES:-0},"after_lines":${after_lines},"before_tests":${TOTAL_TESTS:-0},"after_tests":${after_tests},"modules":${TOTAL_MODULES:-0}}
JSONEOF
    )

    echo "$entry" >> "$LOG_FILE"
    log_info "  Logged to $LOG_FILE"
}

# ═══════════════════════════════════════════════════════════════════════
# Main execution
# ═══════════════════════════════════════════════════════════════════════

echo ""
echo "  ╔══════════════════════════════════════════════════╗"
echo "  ║   NEXUS-6 Architecture Growth                    ║"
echo "  ║   Dry-run: $DRY_RUN  Max-actions: $MAX_ACTIONS              ║"
echo "  ╚══════════════════════════════════════════════════╝"
echo ""

actions_done=0

for i in $(seq 1 "$MAX_ACTIONS"); do
    echo ""
    echo "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Architecture Action $i / $MAX_ACTIONS"
    echo "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Discover
    DISCOVERED_STUBS=()
    DISCOVERED_UNTESTED=()
    DISCOVERED_ORPHANS=()
    TOTAL_MODULES=0
    TOTAL_LINES=0
    TOTAL_TESTS=0
    discover_gaps

    # Prioritize
    SELECTED_ACTION=""
    SELECTED_TARGET=""
    SELECTED_LINES=""
    SELECTED_TESTS=""
    SELECTED_MOD_A=""
    SELECTED_MOD_B=""

    if ! select_architecture_action; then
        log_info "No more architectural gaps to fix."
        break
    fi

    # Generate
    gen_ok=true
    if ! generate_fix; then
        gen_ok=false
    fi

    # Validate
    validate_ok=true
    if $gen_ok && ! $DRY_RUN; then
        if ! validate_changes; then
            validate_ok=false
        fi
    fi

    # Commit or rollback
    if $gen_ok && $validate_ok; then
        commit_or_rollback "true"
        log_result "true"
        actions_done=$((actions_done + 1))
    elif ! $DRY_RUN; then
        commit_or_rollback "false"
        log_result "false"
    else
        log_result "true"
        actions_done=$((actions_done + 1))
    fi
done

echo ""
echo "  ╔══════════════════════════════════════════════════╗"
echo "  ║   Architecture Growth Complete                   ║"
echo "  ║   Actions: $actions_done / $MAX_ACTIONS                              ║"
echo "  ╚══════════════════════════════════════════════════╝"
echo ""
