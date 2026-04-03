#!/usr/bin/env bash
set -euo pipefail

CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"
# ═══════════════════════════════════════════════════════════════════════
# NEXUS-6 Module Growth — Auto-upgrade modules using Claude Code CLI
# ═══════════════════════════════════════════════════════════════════════
#
# Usage:
#   ./grow_modules.sh                     # Report maturity of all modules
#   ./grow_modules.sh --target MODULE     # Upgrade a specific module
#   ./grow_modules.sh --upgrade-all-stubs # Upgrade all Stub/Empty modules
#   ./grow_modules.sh --upgrade-basics    # Upgrade all Basic modules
#   ./grow_modules.sh --dry-run           # Show plan without executing
#   ./grow_modules.sh --max N             # Limit upgrades to N modules
#
# n=6 constants thread through every threshold:
#   N=6, SIGMA=12, PHI=2, TAU=4, J2=24, SOPFR=5

NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC_DIR="$NEXUS_ROOT/src"
LOG_FILE="$NEXUS_ROOT/growth_log.jsonl"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# ── n=6 constants ────────────────────────────────────────────────────
N=6
SIGMA=12
PHI=2
TAU=4
J2=24
SOPFR=5
SIGMA_MINUS_TAU=8
SIGMA_MINUS_PHI=10

# ── Maturity thresholds ──────────────────────────────────────────────
# Empty:       0 lines
# Stub:        < 50 lines, < 2 tests
# Basic:       < 200 lines, < 5 tests
# Developing:  < 500 lines, 5-9 tests
# Mature:      500+ lines, 10+ tests
# Production:  1000+ lines, 20+ tests

# ── Module purpose descriptions ──────────────────────────────────────
# Module purpose descriptions (bash 3.2 compatible — no associative arrays)
module_purpose_for() {
  case "$1" in
    cross_intel) echo "Cross-project intelligence — find patterns across TECS-L repos" ;;
    statistics) echo "Statistical significance testing, effect sizes, reproducibility" ;;
    template) echo "Templates for experiments, lenses, and reports" ;;
    versioning) echo "Schema versioning and migration for data formats" ;;
    sandbox) echo "Isolated execution environment for untrusted experiments" ;;
    scheduler) echo "Task scheduling with priority queues and dependencies" ;;
    distributed) echo "Distributed scanning across multiple nodes" ;;
    dream) echo "Creative hypothesis generation through random exploration" ;;
    consciousness_bridge) echo "Bridge between NEXUS-6 analysis and Anima consciousness engine" ;;
    feedback) echo "User feedback collection and learning from corrections" ;;
    time_travel) echo "Snapshot and restore system state for reproducibility" ;;
    pipeline) echo "Multi-stage data processing pipelines" ;;
    event) echo "Event bus for inter-module communication" ;;
    api) echo "REST API for external tool integration" ;;
    nlp) echo "Natural language processing for hypothesis text analysis" ;;
    plugin) echo "Plugin system for third-party extensions" ;;
    multi_agent) echo "Multi-agent coordination for parallel discovery" ;;
    autonomous) echo "Autonomous agent that runs growth cycles independently" ;;
    ingest) echo "Data ingestion from various formats (JSON, CSV, text)" ;;
    knowledge) echo "Knowledge base for storing and querying discoveries" ;;
    publish) echo "Publishing results in various formats (markdown, LaTeX, BT)" ;;
    reward) echo "Reward system for scoring lens and agent performance" ;;
    genetic_prog) echo "Genetic programming for lens parameter evolution" ;;
    alert) echo "Real-time alerting on significant discoveries or regressions" ;;
    self_improve) echo "Self-analysis and automatic optimization" ;;
    red_team) echo "Adversarial testing of discoveries (devil's advocate)" ;;
    telescope) echo "Core scanning engine with 22+ lenses for pattern detection" ;;
    cli) echo "Command-line interface for NEXUS-6 discovery engine" ;;
    ouroboros) echo "Self-evolving discovery loop (OUROBOROS cycle)" ;;
    science) echo "Scientific method modules: hypothesis, experiment design" ;;
    auto_register) echo "Automatic lens registration and discovery" ;;
    simulation) echo "Monte Carlo and deterministic simulation of systems" ;;
    experiment) echo "Experiment execution, tracking, and result collection" ;;
    calibration) echo "Lens calibration against known datasets" ;;
    lens_forge) echo "Dynamic lens creation and parameter tuning" ;;
    growth) echo "Growth tracking, benchmarking, and planning" ;;
    history) echo "Execution history tracking and audit trail" ;;
    gpu) echo "GPU architecture analysis and compute resource management" ;;
    graph) echo "Graph data structures for dependency and relationship modeling" ;;
    encoder) echo "Data encoding/decoding for compact representation" ;;
    verifier) echo "Independent verification of discoveries and claims" ;;
    materials) echo "Material property analysis and n=6 pattern detection" ;;
    *) echo "General NEXUS-6 module" ;;
  esac
}

# ═══════════════════════════════════════════════════════════════════════
# Functions
# ═══════════════════════════════════════════════════════════════════════

classify_maturity() {
  local lines=$1 tests=$2
  if [ "$lines" -eq 0 ]; then
    echo "EMPTY"
  elif [ "$lines" -lt 50 ] && [ "$tests" -lt "$PHI" ]; then
    echo "STUB"
  elif [ "$lines" -lt 200 ] && [ "$tests" -lt "$SOPFR" ]; then
    echo "BASIC"
  elif [ "$lines" -lt 500 ] && [ "$tests" -lt "$SIGMA_MINUS_PHI" ]; then
    echo "DEVELOPING"
  elif [ "$lines" -ge 1000 ] && [ "$tests" -ge 20 ]; then
    echo "PRODUCTION"
  elif [ "$lines" -ge 500 ] && [ "$tests" -ge "$SIGMA_MINUS_PHI" ]; then
    echo "MATURE"
  elif [ "$lines" -ge 500 ]; then
    echo "DEVELOPING"
  else
    echo "BASIC"
  fi
}

maturity_stars() {
  case "$1" in
    EMPTY)      echo "          " ;;
    STUB)       echo "*         " ;;
    BASIC)      echo "**        " ;;
    DEVELOPING) echo "***       " ;;
    MATURE)     echo "****      " ;;
    PRODUCTION) echo "*****     " ;;
    *)          echo "?         " ;;
  esac
}

maturity_score() {
  case "$1" in
    EMPTY)      echo 0 ;;
    STUB)       echo 1 ;;
    BASIC)      echo 2 ;;
    DEVELOPING) echo 3 ;;
    MATURE)     echo 4 ;;
    PRODUCTION) echo 5 ;;
    *)          echo 0 ;;
  esac
}

next_maturity() {
  case "$1" in
    EMPTY)      echo "STUB" ;;
    STUB)       echo "BASIC" ;;
    BASIC)      echo "DEVELOPING" ;;
    DEVELOPING) echo "MATURE" ;;
    MATURE)     echo "PRODUCTION" ;;
    PRODUCTION) echo "PRODUCTION" ;;
    *)          echo "BASIC" ;;
  esac
}

# Scan all modules and populate arrays
scan_modules() {
  local dir
  MODULE_NAMES=()
  declare -gA MOD_FILES MOD_LINES MOD_TESTS MOD_MATURITY

  for dir in "$SRC_DIR"/*/; do
    [ -d "$dir" ] || continue
    local name
    name=$(basename "$dir")

    local files lines tests
    files=$(find "$dir" -name "*.rs" 2>/dev/null | wc -l | tr -d ' ')
    lines=$(find "$dir" -name "*.rs" -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
    tests=$(grep -r "fn test_" "$dir" 2>/dev/null | wc -l | tr -d ' ')

    local maturity
    maturity=$(classify_maturity "$lines" "$tests")

    MODULE_NAMES+=("$name")
    MOD_FILES[$name]=$files
    MOD_LINES[$name]=$lines
    MOD_TESTS[$name]=$tests
    MOD_MATURITY[$name]=$maturity
  done
}

# Print maturity report table
print_report() {
  echo ""
  echo "┌────────────────────────┬───────┬───────┬───────┬──────────────┐"
  echo "│ Module                 │ Files │ Lines │ Tests │ Maturity     │"
  echo "├────────────────────────┼───────┼───────┼───────┼──────────────┤"

  # Sort by maturity score descending, then by name
  local sorted
  sorted=$(for name in "${MODULE_NAMES[@]}"; do
    local score
    score=$(maturity_score "${MOD_MATURITY[$name]}")
    printf "%s %s %s %s %s %s\n" "$score" "$name" "${MOD_FILES[$name]}" "${MOD_LINES[$name]}" "${MOD_TESTS[$name]}" "${MOD_MATURITY[$name]}"
  done | sort -t' ' -k1,1rn -k2,2)

  local total_score=0 total_modules=0
  local count_empty=0 count_stub=0 count_basic=0 count_dev=0 count_mature=0 count_prod=0

  while IFS=' ' read -r score name files lines tests maturity; do
    local stars
    stars=$(maturity_stars "$maturity")
    printf "│ %-22s │ %5s │ %5s │ %5s │ %s │\n" "$name" "$files" "$lines" "$tests" "$stars"
    total_score=$((total_score + score))
    total_modules=$((total_modules + 1))
    case "$maturity" in
      EMPTY)      count_empty=$((count_empty + 1)) ;;
      STUB)       count_stub=$((count_stub + 1)) ;;
      BASIC)      count_basic=$((count_basic + 1)) ;;
      DEVELOPING) count_dev=$((count_dev + 1)) ;;
      MATURE)     count_mature=$((count_mature + 1)) ;;
      PRODUCTION) count_prod=$((count_prod + 1)) ;;
    esac
  done <<< "$sorted"

  echo "└────────────────────────┴───────┴───────┴───────┴──────────────┘"
  echo ""

  if [ "$total_modules" -gt 0 ]; then
    local mean
    mean=$(echo "scale=2; $total_score / $total_modules" | bc 2>/dev/null || echo "N/A")
    echo "Mean maturity: $mean/5.0 | Modules: $total_modules"
  fi
  echo "Distribution: PROD=$count_prod MATURE=$count_mature DEV=$count_dev BASIC=$count_basic STUB=$count_stub EMPTY=$count_empty"
  echo ""
}

# Generate Claude prompt for a module upgrade
generate_prompt() {
  local name=$1
  local current_maturity=$2
  local target_maturity=$3
  local current_lines=${MOD_LINES[$name]}
  local current_tests=${MOD_TESTS[$name]}
  local purpose
  purpose=$(module_purpose_for "$name")

  local target_lines target_tests
  case "$target_maturity" in
    STUB)
      target_lines=50
      target_tests=$PHI
      ;;
    BASIC)
      target_lines=200
      target_tests=$SOPFR
      ;;
    DEVELOPING)
      target_lines=500
      target_tests=$SIGMA_MINUS_PHI
      ;;
    MATURE)
      target_lines=500
      target_tests=$SIGMA_MINUS_PHI
      ;;
    PRODUCTION)
      target_lines=1000
      target_tests=20
      ;;
    *)
      target_lines=200
      target_tests=$SOPFR
      ;;
  esac

  local lines_needed=$((target_lines > current_lines ? target_lines - current_lines : 0))
  local tests_needed=$((target_tests > current_tests ? target_tests - current_tests : 0))

  cat <<PROMPT
Upgrade the '${name}' module in tools/nexus6/src/${name}/ from ${current_maturity} to ${target_maturity} maturity.

Module purpose: ${purpose}
Current state: ${MOD_FILES[$name]} files, ${current_lines} lines, ${current_tests} tests
Target: ${target_lines}+ lines, ${target_tests}+ tests (need +${lines_needed} lines, +${tests_needed} tests)

Required:
1. Implement substantial functionality matching the module purpose
2. Add at least ${tests_needed} new unit tests (total ${target_tests}+)
3. Use n=6 constants: N=6, SIGMA=12, PHI=2, TAU=4, J2=24, SOPFR=5
4. All constants must have comments explaining their n=6 origin
5. Include #[cfg(test)] mod tests block
6. No external crates (std only)
7. Ensure \`cd tools/nexus6 && cargo check\` passes with no errors

Project root: $(dirname "$NEXUS_ROOT")
Crate root: ${NEXUS_ROOT}
PROMPT
}

# Upgrade a single module
upgrade_module() {
  local name=$1
  local dry_run=${2:-false}

  local current_maturity="${MOD_MATURITY[$name]}"
  local target_maturity
  target_maturity=$(next_maturity "$current_maturity")

  if [ "$current_maturity" = "PRODUCTION" ]; then
    echo "  [SKIP] $name is already PRODUCTION maturity"
    return 0
  fi

  echo ""
  echo "  ┌─────────────────────────────────────────────────┐"
  echo "  │  Upgrading: $name"
  echo "  │  $current_maturity -> $target_maturity"
  echo "  │  Current: ${MOD_FILES[$name]} files, ${MOD_LINES[$name]} lines, ${MOD_TESTS[$name]} tests"
  echo "  └─────────────────────────────────────────────────┘"

  local prompt
  prompt=$(generate_prompt "$name" "$current_maturity" "$target_maturity")

  if [ "$dry_run" = "true" ]; then
    echo ""
    echo "  [DRY RUN] Would execute Claude CLI with prompt:"
    echo "  ---"
    echo "$prompt" | sed 's/^/  | /'
    echo "  ---"
    return 0
  fi

  # Save pre-upgrade state for rollback
  local pre_lines=${MOD_LINES[$name]}
  local pre_tests=${MOD_TESTS[$name]}

  echo "  [RUN] Invoking Claude CLI..."
  local claude_exit=0
  $CLAUDE_CLI -p "$prompt" --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>&1 | tail -20 || claude_exit=$?

  if [ "$claude_exit" -ne 0 ]; then
    echo "  [FAIL] Claude CLI exited with code $claude_exit"
    log_result "$name" "$current_maturity" "$target_maturity" "CLAUDE_FAIL" "$pre_lines" "$pre_tests" 0 0
    return 1
  fi

  # Validate: cargo check
  echo "  [CHECK] Running cargo check..."
  if ! (cd "$NEXUS_ROOT" && ~/.cargo/bin/cargo check 2>&1 | tail -5); then
    echo "  [FAIL] cargo check failed — reverting..."
    (cd "$NEXUS_ROOT" && git checkout -- "src/$name/" 2>/dev/null || true)
    log_result "$name" "$current_maturity" "$target_maturity" "CHECK_FAIL" "$pre_lines" "$pre_tests" 0 0
    return 1
  fi

  # Validate: cargo test for this module
  echo "  [TEST] Running cargo test..."
  if ! (cd "$NEXUS_ROOT" && ~/.cargo/bin/cargo test -- "$name" 2>&1 | tail -10); then
    echo "  [FAIL] cargo test failed — reverting..."
    (cd "$NEXUS_ROOT" && git checkout -- "src/$name/" 2>/dev/null || true)
    log_result "$name" "$current_maturity" "$target_maturity" "TEST_FAIL" "$pre_lines" "$pre_tests" 0 0
    return 1
  fi

  # Re-scan module to get new stats
  local new_lines new_tests
  new_lines=$(find "$SRC_DIR/$name" -name "*.rs" -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
  new_tests=$(grep -r "fn test_" "$SRC_DIR/$name" 2>/dev/null | wc -l | tr -d ' ')
  local lines_added=$((new_lines - pre_lines))
  local tests_added=$((new_tests - pre_tests))

  echo "  [OK] Upgrade complete: +${lines_added} lines, +${tests_added} tests"
  log_result "$name" "$current_maturity" "$target_maturity" "SUCCESS" "$pre_lines" "$pre_tests" "$lines_added" "$tests_added"
  return 0
}

# Log result to JSONL
log_result() {
  local name=$1 from=$2 to=$3 status=$4 pre_lines=$5 pre_tests=$6 lines_added=$7 tests_added=$8
  echo "{\"timestamp\":\"$TIMESTAMP\",\"module\":\"$name\",\"from\":\"$from\",\"to\":\"$to\",\"status\":\"$status\",\"pre_lines\":$pre_lines,\"pre_tests\":$pre_tests,\"lines_added\":$lines_added,\"tests_added\":$tests_added}" >> "$LOG_FILE"
}

# ═══════════════════════════════════════════════════════════════════════
# CLI argument parsing
# ═══════════════════════════════════════════════════════════════════════

TARGET_MODULE=""
UPGRADE_ALL_STUBS=false
UPGRADE_BASICS=false
DRY_RUN=false
MAX_UPGRADES=$SIGMA  # default: sigma=12

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      TARGET_MODULE="$2"
      shift 2
      ;;
    --upgrade-all-stubs)
      UPGRADE_ALL_STUBS=true
      shift
      ;;
    --upgrade-basics)
      UPGRADE_BASICS=true
      shift
      ;;
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    --max)
      MAX_UPGRADES="$2"
      shift 2
      ;;
    --help|-h)
      echo "NEXUS-6 Module Growth"
      echo ""
      echo "Usage:"
      echo "  $0                       Report maturity of all modules"
      echo "  $0 --target MODULE       Upgrade a specific module"
      echo "  $0 --upgrade-all-stubs   Upgrade all Stub/Empty modules"
      echo "  $0 --upgrade-basics      Upgrade all Basic modules"
      echo "  $0 --dry-run             Show plan without executing"
      echo "  $0 --max N               Limit upgrades to N modules (default: sigma=12)"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# ═══════════════════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════════════════

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  NEXUS-6 Module Growth Engine"
echo "  $(date -u +"%Y-%m-%d %H:%M:%S UTC")"
echo "═══════════════════════════════════════════════════════════════"

# Step 1: Scan all modules
echo ""
echo "[1/3] Scanning modules..."
scan_modules

# Step 2: Print maturity report
echo "[2/3] Maturity report:"
print_report

# Step 3: Execute upgrades if requested
if [ -n "$TARGET_MODULE" ]; then
  echo "[3/3] Upgrading target module: $TARGET_MODULE"
  if [[ -z "${MOD_MATURITY[$TARGET_MODULE]+x}" ]]; then
    echo "  [ERROR] Module '$TARGET_MODULE' not found in $SRC_DIR/"
    exit 1
  fi
  upgrade_module "$TARGET_MODULE" "$DRY_RUN"

elif [ "$UPGRADE_ALL_STUBS" = "true" ]; then
  echo "[3/3] Upgrading all Empty/Stub modules (max: $MAX_UPGRADES)..."
  upgraded=0
  for name in "${MODULE_NAMES[@]}"; do
    [ "$upgraded" -ge "$MAX_UPGRADES" ] && break
    local_mat="${MOD_MATURITY[$name]}"
    if [ "$local_mat" = "EMPTY" ] || [ "$local_mat" = "STUB" ]; then
      upgrade_module "$name" "$DRY_RUN" || true
      upgraded=$((upgraded + 1))
    fi
  done
  echo ""
  echo "Upgraded $upgraded modules."

elif [ "$UPGRADE_BASICS" = "true" ]; then
  echo "[3/3] Upgrading all Basic modules (max: $MAX_UPGRADES)..."
  upgraded=0
  for name in "${MODULE_NAMES[@]}"; do
    [ "$upgraded" -ge "$MAX_UPGRADES" ] && break
    local_mat="${MOD_MATURITY[$name]}"
    if [ "$local_mat" = "BASIC" ]; then
      upgrade_module "$name" "$DRY_RUN" || true
      upgraded=$((upgraded + 1))
    fi
  done
  echo ""
  echo "Upgraded $upgraded modules."

else
  echo "[3/3] No upgrade action requested. Use --target, --upgrade-all-stubs, or --upgrade-basics."
  echo ""
  echo "Suggested next steps:"

  # Find weakest modules and suggest
  count=0
  for name in "${MODULE_NAMES[@]}"; do
    [ "$count" -ge "$N" ] && break
    local_mat="${MOD_MATURITY[$name]}"
    if [ "$local_mat" = "EMPTY" ] || [ "$local_mat" = "STUB" ]; then
      local target
      target=$(next_maturity "$local_mat")
      echo "  $0 --target $name  # $local_mat -> $target"
      count=$((count + 1))
    fi
  done

  if [ "$count" -eq 0 ]; then
    for name in "${MODULE_NAMES[@]}"; do
      [ "$count" -ge "$N" ] && break
      local_mat="${MOD_MATURITY[$name]}"
      if [ "$local_mat" = "BASIC" ]; then
        echo "  $0 --target $name  # BASIC -> DEVELOPING"
        count=$((count + 1))
      fi
    done
  fi
fi

echo ""
echo "Done. Log: $LOG_FILE"
