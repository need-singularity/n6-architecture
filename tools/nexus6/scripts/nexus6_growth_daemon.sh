#!/usr/bin/env bash
set -euo pipefail

CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"
# NEXUS-6 Universal Growth Daemon
# ================================
# Master coordinator that cycles through ALL 15 growth dimensions,
# picking the weakest each cycle and executing targeted growth actions.
#
# Usage: ./nexus6_growth_daemon.sh [--interval MIN] [--max-cycles N] [--dimension DIM] [--dry-run]
#
# Dimensions: performance, architecture, lenses, modules, tests, hypotheses,
#             dse, experiments, calculators, cross_resonance, knowledge_graph,
#             red_team, atlas, documentation, integration
#
# Safety brakes:
#   - Stop if tests fail 3 consecutive times
#   - Stop if --max-cycles reached
#   - Graceful shutdown on SIGTERM/SIGINT

NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SCRIPT_DIR="$NEXUS_ROOT/scripts"
LOG_FILE="$SCRIPT_DIR/growth_log.jsonl"
REPO_ROOT="$(cd "$NEXUS_ROOT/../.." && pwd)"

cd "$NEXUS_ROOT"

# ── Parse arguments ──────────────────────────────────────────────────
INTERVAL_MIN=5
MAX_CYCLES=6          # n=6 default
FORCE_DIM=""
DRY_RUN=false
SKIP_COMMIT=false
AUTO_PUSH=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --interval)    INTERVAL_MIN="$2"; shift 2 ;;
        --max-cycles)  MAX_CYCLES="$2"; shift 2 ;;
        --dimension)   FORCE_DIM="$2"; shift 2 ;;
        --dry-run)     DRY_RUN=true; shift ;;
        --skip-commit) SKIP_COMMIT=true; shift ;;
        --auto-push)   AUTO_PUSH=true; shift ;;
        -h|--help)
            echo "NEXUS-6 Universal Growth Daemon"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --interval MIN     Sleep between cycles (default: 5)"
            echo "  --max-cycles N     Maximum cycles to run (default: 6)"
            echo "  --dimension DIM    Force a specific dimension each cycle"
            echo "  --dry-run          Analyze only, no code generation or commits"
            echo "  --skip-commit      Generate code but don't commit"
            echo "  --auto-push        Auto-push to origin after successful growth"
            echo ""
            echo "Dimensions:"
            echo "  performance, architecture, lenses, modules, tests, hypotheses,"
            echo "  dse, experiments, calculators, cross_resonance, knowledge_graph,"
            echo "  red_team, atlas, documentation, integration"
            exit 0
            ;;
        *) echo "Unknown flag: $1"; exit 1 ;;
    esac
done

# ── Utility functions ────────────────────────────────────────────────
log_info()  { echo "[$(date +%H:%M:%S)] INFO:  $*"; }
log_warn()  { echo "[$(date +%H:%M:%S)] WARN:  $*"; }
log_error() { echo "[$(date +%H:%M:%S)] ERROR: $*"; }

# Consecutive failure counter for safety brake
CONSECUTIVE_FAILURES=0
MAX_CONSECUTIVE_FAILURES=3

# Graceful shutdown
SHUTDOWN=false
trap 'log_info "SIGTERM/SIGINT received, shutting down after current cycle..."; SHUTDOWN=true' SIGTERM SIGINT

# ── Dimension measurement ────────────────────────────────────────────
#
# Collects metrics for ALL 15 dimensions and outputs a JSON object.
# Sources: measure.sh + file counting + pattern matching.

measure_all_dimensions() {
    local base_metrics
    base_metrics=$(bash "$SCRIPT_DIR/measure.sh" 2>/dev/null || echo '{}')

    # Extract base values
    local tests_pass warnings lenses_registered modules code_lines test_fns
    tests_pass=$(echo "$base_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('tests_pass',0))" 2>/dev/null || echo "0")
    warnings=$(echo "$base_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('warnings',0))" 2>/dev/null || echo "0")
    lenses_registered=$(echo "$base_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('lenses_registered',0))" 2>/dev/null || echo "0")
    modules=$(echo "$base_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('modules',0))" 2>/dev/null || echo "0")
    code_lines=$(echo "$base_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('code_lines',0))" 2>/dev/null || echo "0")
    test_fns=$(echo "$base_metrics" | python3 -c "import sys,json; print(json.load(sys.stdin).get('test_fns',0))" 2>/dev/null || echo "0")

    # Performance: composite score = tests_pass * (code_lines / 100) / warnings_penalty
    local warn_penalty=1
    if [[ "$warnings" -gt 0 ]]; then warn_penalty=$((1 + warnings / 10)); fi
    local performance=$(( tests_pass * (code_lines / 100) / warn_penalty ))

    # Architecture: completeness % (modules with tests / total modules)
    local mods_with_tests
    mods_with_tests=$(find src -mindepth 1 -maxdepth 1 -type d -exec sh -c 'grep -rl "#\[test\]" "$1" >/dev/null 2>&1 && echo 1' _ {} \; | wc -l | tr -d ' ')
    local arch_pct=0
    if [[ "$modules" -gt 0 ]]; then
        arch_pct=$((mods_with_tests * 100 / modules))
    fi

    # Lenses: count implemented lenses (individual files + bulk impl blocks)
    local lenses_files
    lenses_files=$(find src/telescope/lenses -name "*_lens.rs" 2>/dev/null | wc -l | tr -d ' ')
    local lenses_bulk
    lenses_bulk=$(grep -rh "impl.*Lens" src/telescope/ 2>/dev/null | grep -v "test\|mod.rs" | wc -l | tr -d ' ')
    lenses_bulk=${lenses_bulk:-0}
    local lenses_impl=$((lenses_files + lenses_bulk))

    # Modules: maturity score (0-5 scale, based on test density)
    local maturity="3.0"
    if [[ "$test_fns" -gt 0 && "$modules" -gt 0 ]]; then
        local tests_per_mod=$((test_fns / modules))
        if [[ "$tests_per_mod" -ge 10 ]]; then
            maturity="4.5"
        elif [[ "$tests_per_mod" -ge 5 ]]; then
            maturity="3.5"
        elif [[ "$tests_per_mod" -ge 2 ]]; then
            maturity="2.5"
        else
            maturity="1.5"
        fi
    fi

    # Hypotheses: count unique BT entries (BT-N pattern)
    local bt_count=0
    if [[ -f "$REPO_ROOT/docs/breakthrough-theorems.md" ]]; then
        bt_count=$(grep -oE "BT-[0-9]+" "$REPO_ROOT/docs/breakthrough-theorems.md" 2>/dev/null | sort -u | wc -l | tr -d ' ')
        bt_count=${bt_count:-0}
    fi

    # DSE: count explored TOML domains
    local dse_count=0
    if [[ -d "$REPO_ROOT/tools/universal-dse/domains" ]]; then
        dse_count=$(find "$REPO_ROOT/tools/universal-dse/domains" -name "*.toml" 2>/dev/null | wc -l | tr -d ' ')
    fi

    # Experiments: count experiment files
    local exp_count=0
    if [[ -d "$REPO_ROOT/experiments" ]]; then
        exp_count=$(find "$REPO_ROOT/experiments" -name "experiment_*.py" -o -name "verify_*.py" 2>/dev/null | wc -l | tr -d ' ')
    fi

    # Calculators: count Rust/Python tools
    local calc_count=0
    if [[ -d "$REPO_ROOT/tools" ]]; then
        calc_count=$(find "$REPO_ROOT/tools" -maxdepth 1 -type d | wc -l | tr -d ' ')
        calc_count=$((calc_count - 1))  # subtract the tools/ dir itself
    fi
    local shared_calc=0
    if [[ -d "$REPO_ROOT/.shared/calc" ]]; then
        shared_calc=$(find "$REPO_ROOT/.shared/calc" -name "*.py" -o -name "*.rs" 2>/dev/null | wc -l | tr -d ' ')
    fi
    calc_count=$((calc_count + shared_calc))

    # CrossResonance: count documented cross-domain connections
    local resonance_count=0
    if ls "$REPO_ROOT"/docs/cross-domain-resonance*.md >/dev/null 2>&1; then
        resonance_count=$(grep -ciE "resonance|bridge|cross.domain|shared.*constant|BT-[0-9]+" "$REPO_ROOT"/docs/cross-domain-resonance*.md 2>/dev/null || echo "0")
    fi
    # Also count BTs that span 3+ domains (from breakthrough-theorems.md)
    if [[ -f "$REPO_ROOT/docs/breakthrough-theorems.md" ]]; then
        local cross_bts
        cross_bts=$(grep -c "도메인\|domain" "$REPO_ROOT/docs/breakthrough-theorems.md" 2>/dev/null || echo "0")
        resonance_count=$((resonance_count + cross_bts))
    fi

    # KnowledgeGraph: count graph nodes (BtEntry + DiscoveryNode structs)
    local graph_nodes=0
    if [[ -d "src/graph" ]]; then
        local bt_nodes
        bt_nodes=$(grep -rh "BtEntry {" src/graph/ 2>/dev/null | wc -l | tr -d ' ')
        bt_nodes=${bt_nodes:-0}
        local disc_nodes
        disc_nodes=$(grep -rhE "DiscoveryNode|GraphNode|add_node" src/graph/ 2>/dev/null | wc -l | tr -d ' ')
        disc_nodes=${disc_nodes:-0}
        graph_nodes=$((bt_nodes + disc_nodes))
    fi

    # RedTeam: count adversarial/challenge/test functions
    local red_team_count=0
    if [[ -d "src/red_team" ]]; then
        red_team_count=$(grep -rhE "fn.*(challenge|adversar|falsif|attack|probe|stress|counter|test_)" src/red_team/ 2>/dev/null | wc -l | tr -d ' ')
        red_team_count=${red_team_count:-0}
    fi

    # Atlas: count constants
    local atlas_count=0
    if [[ -f "$REPO_ROOT/docs/atlas-constants.md" ]]; then
        atlas_count=$(grep -c "^|" "$REPO_ROOT/docs/atlas-constants.md" 2>/dev/null || echo "0")
    fi

    # Documentation: coverage estimate (modules with doc comments / total)
    local doc_pct=0
    local mods_with_docs
    mods_with_docs=$(find src -maxdepth 2 -name "mod.rs" -exec grep -l "^//!" {} \; 2>/dev/null | wc -l | tr -d ' ')
    if [[ "$modules" -gt 0 ]]; then
        doc_pct=$((mods_with_docs * 100 / modules))
    fi

    # Integration: count integration tests
    local integration_tests=0
    if [[ -d "tests" ]]; then
        integration_tests=$(grep -r "#\[test\]" tests/ 2>/dev/null | wc -l | tr -d ' ')
    fi
    # Also count cross-module tests in src
    local cross_mod_tests
    cross_mod_tests=$(grep -rl "use crate::" src/ 2>/dev/null | xargs grep -l "#\[cfg(test)\]" 2>/dev/null | wc -l | tr -d '[:space:]')
    cross_mod_tests=${cross_mod_tests:-0}
    integration_tests=$((integration_tests + cross_mod_tests))

    # Output JSON
    cat <<EOF
{"timestamp":"$(date -u +%Y-%m-%dT%H:%M:%SZ)","performance":${performance},"architecture":${arch_pct},"lenses":${lenses_impl},"modules":${maturity},"tests":${test_fns},"hypotheses":${bt_count},"dse":${dse_count},"experiments":${exp_count},"calculators":${calc_count},"cross_resonance":${resonance_count},"knowledge_graph":${graph_nodes},"red_team":${red_team_count},"atlas":${atlas_count},"documentation":${doc_pct},"integration":${integration_tests},"warnings":${warnings},"code_lines":${code_lines}}
EOF
}

# ── Weakest dimension picker ─────────────────────────────────────────
#
# Reads the JSON metrics and picks the dimension furthest from target.
# Targets: performance=10000, architecture=100, lenses=200, modules=4.0,
#   tests=1000, hypotheses=150, dse=322, experiments=50, calculators=50,
#   cross_resonance=100, knowledge_graph=500, red_team=100, atlas=2000,
#   documentation=90, integration=50

pick_weakest_dimension() {
    local metrics_json="$1"
    local skip_dims="${2:-}"
    python3 -c "
import sys, json

m = json.loads('''$metrics_json''')
skip = set('$skip_dims'.split()) if '$skip_dims'.strip() else set()

targets = {
    'performance':     10000,
    'architecture':      100,
    'lenses':            200,
    'modules':           4.0,
    'tests':            1000,
    'hypotheses':        150,
    'dse':               322,
    'experiments':        50,
    'calculators':        50,
    'cross_resonance':   100,
    'knowledge_graph':   500,
    'red_team':          100,
    'atlas':            2000,
    'documentation':      90,
    'integration':        50,
}

# Impact weights (same as registry.rs)
weights = {
    'tests':           0.12,
    'lenses':          0.10,
    'architecture':    0.10,
    'performance':     0.08,
    'hypotheses':      0.08,
    'integration':     0.08,
    'dse':             0.06,
    'knowledge_graph': 0.06,
    'red_team':        0.06,
    'cross_resonance': 0.05,
    'atlas':           0.05,
    'experiments':      0.05,
    'calculators':     0.04,
    'modules':         0.04,
    'documentation':   0.03,
}

best_dim = 'tests'
best_score = -1.0

for dim, target in targets.items():
    if dim in skip:
        continue
    current = float(m.get(dim, 0))
    if target > 0:
        gap = max(0.0, 1.0 - current / target)
    else:
        gap = 0.0
    score = gap * weights.get(dim, 0.05)
    if score > best_score:
        best_score = score
        best_dim = dim

print(best_dim)
" 2>/dev/null
}

# ── Growth action functions ──────────────────────────────────────────

grow_performance() {
    log_info "  Action: Performance optimization via auto_grow.sh"
    bash "$SCRIPT_DIR/auto_grow.sh" --cycles 1 --skip-commit 2>/dev/null || return 1
}

grow_architecture() {
    log_info "  Action: Architecture gap analysis + fix via grow_architecture.sh"
    bash "$SCRIPT_DIR/grow_architecture.sh" --max-actions 1 2>/dev/null || return 1
}

grow_lenses() {
    log_info "  Action: Implement new lenses (batch of 6)"
    bash "$SCRIPT_DIR/grow_lenses.sh" --batch 6 2>/dev/null || \
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/tools/nexus6/, implement 6 new telescope lenses. Check src/telescope/lenses/ for existing lenses and src/telescope/registry.rs for registered but unimplemented lenses. For each new lens: create the .rs file implementing the Lens trait with scan() method, add n=6 constants, add 2+ tests. Register all new lenses." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_modules() {
    log_info "  Action: Upgrade module maturity"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/tools/nexus6/, find the module with lowest test-to-code ratio (fewest tests per line of code). Add at least 8 (sigma-tau=8) meaningful tests to it. Each test should verify a specific behavior. Use n=6 constants in test data. Run cargo test to verify." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_tests() {
    log_info "  Action: Add tests to under-tested modules"
    bash "$SCRIPT_DIR/grow_tests.sh" --auto-pick 2>/dev/null || \
    bash "$SCRIPT_DIR/grow_tests.sh" 2>/dev/null || return 1
}

grow_hypotheses() {
    log_info "  Action: Generate new breakthrough theorems"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/, analyze docs/breakthrough-theorems.md and docs/atlas-constants.md. Find patterns that could lead to new breakthrough theorems (BT-128+). Generate 3 new BT candidates with n=6 expressions, cross-domain validation, and EXACT/CLOSE grades. Write them to docs/breakthrough-theorems.md." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_dse() {
    log_info "  Action: Explore unexplored DSE domain"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/, check docs/dse-map.toml for unexplored DSE domains. Pick one unexplored domain, create its TOML definition in tools/universal-dse/domains/, and run the DSE exploration. Record results in docs/dse-map.toml." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_experiments() {
    log_info "  Action: Create new experiment"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/, look at experiments/ and find techniques/ that don't have corresponding experiments yet. Create a new experiment file that tests an untested technique. Run it and record results." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_calculators() {
    log_info "  Action: Create new calculator"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/, check .shared/calc/ and tools/ for coverage gaps. Identify a domain that needs a calculator (e.g., a BT that lacks numerical verification). Create a Rust calculator in tools/ following CALCULATOR_RULES.md." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_cross_resonance() {
    log_info "  Action: Discover cross-domain resonances"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/, read docs/cross-domain-resonance-2026-03-31.md and docs/atlas-constants.md. Find NEW cross-domain resonances where the same n=6 constant appears in 3+ domains but isn't documented yet. Add discoveries to the resonance file." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_knowledge_graph() {
    log_info "  Action: Expand knowledge graph"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/tools/nexus6/, enhance src/graph/ by adding new discovery nodes from recent BTs (BT-118+) and experiments. Connect them via cross-domain edges. Add tests for new nodes." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_red_team() {
    log_info "  Action: Add adversarial challenges"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/tools/nexus6/, enhance src/red_team/ by adding new adversarial challenges. Pick 3 BTs from docs/breakthrough-theorems.md and generate devil's advocate challenges. Implement challenge functions with tests." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_atlas() {
    log_info "  Action: Expand math atlas"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/, scan docs/ for n=6 constants that aren't in docs/atlas-constants.md yet. Add newly discovered constants with their BT references and domain tags. Then run: python3 .shared/scan_math_atlas.py --save --summary" \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_documentation() {
    log_info "  Action: Improve documentation"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/, find modules or features that lack documentation. Pick the most important undocumented feature and add clear documentation (README sections, inline comments, or doc files). Focus on tools/nexus6/ which is the most active." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

grow_integration() {
    log_info "  Action: Add integration tests"
    $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/tools/nexus6/, find two modules that should work together but have no integration tests. Create integration tests that verify cross-module functionality. Add them to a tests/ directory or as #[cfg(test)] in relevant modules." \
        --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || return 1
}

# ── Execute growth action for a dimension ────────────────────────────

execute_dimension_growth() {
    local dimension="$1"

    case "$dimension" in
        performance)     grow_performance ;;
        architecture)    grow_architecture ;;
        lenses)          grow_lenses ;;
        modules)         grow_modules ;;
        tests)           grow_tests ;;
        hypotheses)      grow_hypotheses ;;
        dse)             grow_dse ;;
        experiments)     grow_experiments ;;
        calculators)     grow_calculators ;;
        cross_resonance) grow_cross_resonance ;;
        knowledge_graph) grow_knowledge_graph ;;
        red_team)        grow_red_team ;;
        atlas)           grow_atlas ;;
        documentation)   grow_documentation ;;
        integration)     grow_integration ;;
        *)
            log_warn "Unknown dimension: $dimension"
            return 1
            ;;
    esac
}

# ── Dashboard printer ────────────────────────────────────────────────

print_dashboard() {
    local metrics_json="$1"
    local cycle="$2"
    local dimension="$3"

    python3 -c "
import sys, json

m = json.loads('''$metrics_json''')
cycle = $cycle
dim = '$dimension'

targets = {
    'performance':     10000,
    'architecture':      100,
    'lenses':            200,
    'modules':           4.0,
    'tests':            1000,
    'hypotheses':        150,
    'dse':               322,
    'experiments':        50,
    'calculators':        50,
    'cross_resonance':   100,
    'knowledge_graph':   500,
    'red_team':          100,
    'atlas':            2000,
    'documentation':      90,
    'integration':        50,
}

print()
print('+' + '-'*65 + '+')
print(f'| NEXUS-6 Growth Dashboard -- Cycle {cycle:<6}                         |')
print('+--------------+---------+---------+--------------+----------------+')
print('| Dimension    |Current  |Target   |Progress      | Health         |')
print('+--------------+---------+---------+--------------+----------------+')

dims_sorted = sorted(targets.keys(), key=lambda d: -(1.0 - min(float(m.get(d, 0)) / max(targets[d], 0.001), 1.0)))

for d in dims_sorted:
    cur = float(m.get(d, 0))
    tgt = targets[d]
    pct = min(cur / max(tgt, 0.001), 1.0) * 100
    filled = int(pct / 100 * 12)
    bar = '#' * filled + '.' * (12 - filled)

    if pct < 25:
        health = '[!!] Critical'
    elif pct < 50:
        health = '[ =] Stagnant'
    elif pct >= 90:
        health = '[++] Thriving'
    else:
        health = '[ +] OnTrack '

    cur_s = f'{cur:.0f}' if cur >= 10 else f'{cur:.1f}'
    tgt_s = f'{tgt:.0f}' if tgt >= 10 else f'{tgt:.1f}'
    marker = ' <<' if d == dim else '   '

    print(f'| {d:<12} | {cur_s:>7} | {tgt_s:>7} | {bar} {pct:>3.0f}% | {health}{marker} |')

print('+--------------+---------+---------+--------------+----------------+')

total_pct = sum(min(float(m.get(d, 0)) / max(targets[d], 0.001), 1.0) for d in targets) / len(targets) * 100
critical = sum(1 for d in targets if float(m.get(d, 0)) / max(targets[d], 0.001) < 0.25)
print(f'| Overall: {total_pct:>5.1f}% | Critical: {critical} | Focus: {dim:<20}  |')
print('+' + '-'*65 + '+')
print()
" 2>/dev/null
}

# ═══════════════════════════════════════════════════════════════════════
# Main Loop
# ═══════════════════════════════════════════════════════════════════════

echo ""
echo "  +============================================+"
echo "  |   NEXUS-6 Universal Growth Daemon          |"
echo "  |   Max cycles: $MAX_CYCLES  Interval: ${INTERVAL_MIN}m          |"
echo "  |   Dimensions: 15 (all tracked)             |"
if [[ -n "$FORCE_DIM" ]]; then
echo "  |   Forced dimension: $FORCE_DIM                  |"
fi
echo "  +============================================+"
echo ""

# PID file management
NEXUS_STATE="$HOME/.nexus6"
mkdir -p "$NEXUS_STATE"
echo $$ > "$NEXUS_STATE/daemon.pid"
trap 'rm -f "$NEXUS_STATE/daemon.pid"' EXIT

for cycle in $(seq 1 "$MAX_CYCLES"); do
    if $SHUTDOWN; then
        log_info "Shutdown requested. Exiting."
        break
    fi

    echo ""
    echo "  ========================================"
    echo "  Cycle $cycle / $MAX_CYCLES"
    echo "  ========================================"
    echo ""

    # ── Step 1: Measure all dimensions ────────────────────────────────
    log_info "Step 1/5: Measuring all 15 dimensions..."
    metrics_json=$(measure_all_dimensions)
    log_info "  Metrics collected."

    # ── Step 2: Pick weakest dimension ────────────────────────────────
    if [[ -n "$FORCE_DIM" ]]; then
        dimension="$FORCE_DIM"
        log_info "Step 2/5: Forced dimension: $dimension"
    else
        dimension=$(pick_weakest_dimension "$metrics_json")
        log_info "Step 2/5: Weakest dimension: $dimension"
    fi

    # ── Consult growth intelligence ─────────────────────────────────
    if [[ -z "$FORCE_DIM" ]] && [[ -f "$SCRIPT_DIR/growth_intelligence.sh" ]]; then
        intel_json=$(bash "$SCRIPT_DIR/growth_intelligence.sh" 2>/dev/null || echo '{}')
        intel_dim=$(echo "$intel_json" | python3 -c "import sys,json; print(json.load(sys.stdin).get('recommended_dimension',''))" 2>/dev/null || echo "")
        skip_list=$(echo "$intel_json" | python3 -c "import sys,json; print(' '.join(json.load(sys.stdin).get('skip_list',[])))" 2>/dev/null || echo "")

        # If intelligence recommends a different dimension with high confidence
        if [[ -n "$intel_dim" ]] && [[ "$intel_dim" != "$dimension" ]]; then
            intel_conf=$(echo "$intel_json" | python3 -c "import sys,json; print(json.load(sys.stdin).get('confidence',0))" 2>/dev/null || echo "0")
            if python3 -c "exit(0 if float('$intel_conf') > 0.7 else 1)" 2>/dev/null; then
                log_info "  Intelligence override: $dimension -> $intel_dim (confidence: $intel_conf)"
                dimension="$intel_dim"
            fi
        fi

        # Skip blacklisted dimensions (in cooldown)
        if echo " $skip_list " | grep -q " $dimension "; then
            log_info "  Intelligence: $dimension is in cooldown. Picking next."
            dimension=$(pick_weakest_dimension "$metrics_json" "$skip_list")
            log_info "  New target: $dimension"
        fi
    fi

    # Print dashboard before action
    print_dashboard "$metrics_json" "$cycle" "$dimension"

    if $DRY_RUN; then
        log_info "[DRY-RUN] Would execute growth for: $dimension"
        echo "$metrics_json" | python3 -c "
import sys, json
d = json.load(sys.stdin)
d['cycle'] = $cycle
d['dimension'] = '$dimension'
d['action'] = 'dry-run'
d['success'] = True
print(json.dumps(d))
" >> "$LOG_FILE" 2>/dev/null || true
        continue
    fi

    # ── Step 3: Execute growth action ─────────────────────────────────
    log_info "Step 3/5: Executing growth for '$dimension'..."
    gen_ok=true
    if ! execute_dimension_growth "$dimension"; then
        log_warn "Growth action failed for $dimension."
        gen_ok=false
    fi

    # ── Step 4: Validate ──────────────────────────────────────────────
    log_info "Step 4/5: Validating changes..."
    validate_ok=true

    if $gen_ok; then
        # cargo check
        if ! ~/.cargo/bin/cargo check 2>&1 | tail -3; then
            log_error "cargo check failed."
            validate_ok=false
        fi

        # cargo test (only if check passed)
        if $validate_ok; then
            test_output=$(~/.cargo/bin/cargo test 2>&1) || true
            if echo "$test_output" | grep -q "FAILED"; then
                log_error "cargo test failed."
                validate_ok=false
            fi
        fi
    else
        validate_ok=false
    fi

    # Revert on failure
    if ! $validate_ok; then
        log_warn "Reverting changes..."
        (cd "$REPO_ROOT" && git checkout -- tools/nexus6/src/) 2>/dev/null || true
        CONSECUTIVE_FAILURES=$((CONSECUTIVE_FAILURES + 1))
        log_warn "Consecutive failures: $CONSECUTIVE_FAILURES / $MAX_CONSECUTIVE_FAILURES"

        # Record failure in troubleshoot.json
        bash "$SCRIPT_DIR/troubleshoot_update.sh" --record-failure "$dimension" "Cycle $cycle: validation failed after growth action" 2>/dev/null || true
        # Notify failure
        bash "$SCRIPT_DIR/growth_notify.sh" "Growth cycle $cycle: $dimension FAILED ($CONSECUTIVE_FAILURES/$MAX_CONSECUTIVE_FAILURES)" --level warn 2>/dev/null || true

        if [[ "$CONSECUTIVE_FAILURES" -ge "$MAX_CONSECUTIVE_FAILURES" ]]; then
            log_error "Safety brake: $MAX_CONSECUTIVE_FAILURES consecutive failures. Stopping."
            bash "$SCRIPT_DIR/growth_notify.sh" "Safety brake triggered after $MAX_CONSECUTIVE_FAILURES failures" --level error 2>/dev/null || true
            break
        fi
    else
        CONSECUTIVE_FAILURES=0
        # Record success in troubleshoot.json
        bash "$SCRIPT_DIR/troubleshoot_update.sh" --record-success 2>/dev/null || true
        # Notify success
        bash "$SCRIPT_DIR/growth_notify.sh" "Growth cycle $cycle: $dimension succeeded" --level success 2>/dev/null || true
    fi

    # ── Step 5: Commit + Log ──────────────────────────────────────────
    if $validate_ok && ! $SKIP_COMMIT; then
        log_info "Step 5/5: Committing + logging..."
        commit_msg="growth(nexus6): ${dimension} -- cycle ${cycle}"
        (cd "$REPO_ROOT" && git add tools/nexus6/src/ && git commit -m "$commit_msg") 2>/dev/null || {
            log_warn "Nothing to commit or commit failed."
        }

        # Auto-push if enabled
        if $AUTO_PUSH; then
            (cd "$REPO_ROOT" && git push origin main 2>/dev/null) && {
                log_info "  Auto-pushed to origin."
            } || {
                log_warn "  Auto-push failed (will retry next cycle)."
            }
        fi
    elif $validate_ok; then
        log_info "Step 5/5: Logging (commit skipped)..."
    else
        log_info "Step 5/5: Logging (validation failed)..."
    fi

    # Collect after-metrics and log
    after_metrics=$(measure_all_dimensions)
    echo "$after_metrics" | python3 -c "
import sys, json
d = json.load(sys.stdin)
d['cycle'] = $cycle
d['dimension'] = '$dimension'
d['action'] = '$dimension'
d['success'] = $( $validate_ok && echo "true" || echo "false" )
print(json.dumps(d))
" >> "$LOG_FILE" 2>/dev/null || true

    # Print after-dashboard
    if $validate_ok; then
        print_dashboard "$after_metrics" "$cycle" "$dimension"
    fi

    # Sleep between cycles (skip on last)
    if [[ "$cycle" -lt "$MAX_CYCLES" ]] && [[ "$INTERVAL_MIN" -gt 0 ]] && ! $SHUTDOWN; then
        log_info "Sleeping ${INTERVAL_MIN}m before next cycle..."
        sleep "${INTERVAL_MIN}m" &
        wait $! 2>/dev/null || true  # interruptible sleep
    fi
done

# ── Final report ──────────────────────────────────────────────────────
echo ""
echo "  +============================================+"
echo "  |   Growth Complete: $MAX_CYCLES cycle(s)              |"
echo "  |   Failures: $CONSECUTIVE_FAILURES                              |"
echo "  +============================================+"
echo ""

if [[ -f "$LOG_FILE" ]]; then
    bash "$SCRIPT_DIR/growth_dashboard.sh" --last "$MAX_CYCLES" 2>/dev/null || \
    bash "$SCRIPT_DIR/growth_report.sh" --last "$MAX_CYCLES" 2>/dev/null || true
fi
