#!/usr/bin/env bash
# NEXUS-6 Performance Benchmark Suite
# ====================================
# Measures scan, evolve, verify, test, and build speeds.
# Outputs JSON results to ~/.nexus6/benchmark-results.json
#
# Usage: ./run_benchmarks.sh [--quick] [--no-build]
set -euo pipefail

NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$NEXUS_ROOT"

CARGO="$HOME/.cargo/bin/cargo"
BINARY="$NEXUS_ROOT/target/release/nexus6"
RESULTS_DIR="$HOME/.nexus6"
RESULTS_FILE="$RESULTS_DIR/benchmark-results.json"

mkdir -p "$RESULTS_DIR"

# ── Parse arguments ──────────────────────────────────────────────────
QUICK=false
NO_BUILD=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --quick)    QUICK=true; shift ;;
        --no-build) NO_BUILD=true; shift ;;
        -h|--help)
            echo "NEXUS-6 Performance Benchmark Suite"
            echo ""
            echo "Usage: $0 [--quick] [--no-build]"
            echo ""
            echo "Options:"
            echo "  --quick      Skip build/test benchmarks (scan+verify only)"
            echo "  --no-build   Skip build speed benchmark"
            echo ""
            exit 0
            ;;
        *) echo "Unknown flag: $1"; exit 1 ;;
    esac
done

# ── Utility ──────────────────────────────────────────────────────────

log_info()  { echo "[$(date +%H:%M:%S)] $*"; }

# Portable high-res timing (macOS + Linux)
# Returns milliseconds
time_cmd() {
    local start end elapsed_ms
    # Use perl for sub-second precision (works on macOS without gdate)
    start=$(perl -MTime::HiRes=time -e 'printf "%.3f", time()')
    eval "$@" > /dev/null 2>&1
    end=$(perl -MTime::HiRes=time -e 'printf "%.3f", time()')
    elapsed_ms=$(perl -e "printf '%.0f', ($end - $start) * 1000")
    echo "$elapsed_ms"
}

# ── Ensure binary exists ─────────────────────────────────────────────

if [[ ! -x "$BINARY" ]]; then
    log_info "Binary not found. Building release..."
    $CARGO build --release 2>&1 | tail -3
fi

if [[ ! -x "$BINARY" ]]; then
    echo "ERROR: Failed to build nexus6 binary."
    exit 1
fi

# ── Header ───────────────────────────────────────────────────────────

echo ""
echo "  ╔══════════════════════════════════════════╗"
echo "  ║   NEXUS-6 Performance Benchmark Suite    ║"
echo "  ╚══════════════════════════════════════════╝"
echo ""

TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# ── Benchmark 1: Lens Scan Speed ─────────────────────────────────────

log_info "Benchmark 1/5: Lens scan speed..."

DOMAINS=("consciousness" "topology" "causal" "gravity" "wave" "evolution")
scan_times=()
scan_json_items=""

for domain in "${DOMAINS[@]}"; do
    ms=$(time_cmd "$BINARY scan $domain --full")
    scan_times+=("$ms")
    log_info "  scan $domain --full: ${ms}ms"
    scan_json_items="${scan_json_items}\"${domain}\": ${ms}, "
done

# Calculate average
scan_total=0
for t in "${scan_times[@]}"; do scan_total=$((scan_total + t)); done
scan_avg=$((scan_total / ${#scan_times[@]}))
log_info "  Average scan time: ${scan_avg}ms"

# ── Benchmark 2: Evolve Speed ────────────────────────────────────────

log_info "Benchmark 2/5: Evolve speed..."

evolve_domains=("consciousness" "topology" "causal")
evolve_times=()
evolve_json_items=""

for domain in "${evolve_domains[@]}"; do
    ms=$(time_cmd "$BINARY evolve $domain --max-cycles 1")
    evolve_times+=("$ms")
    log_info "  evolve $domain --max-cycles 1: ${ms}ms"
    evolve_json_items="${evolve_json_items}\"${domain}\": ${ms}, "
done

evolve_total=0
for t in "${evolve_times[@]}"; do evolve_total=$((evolve_total + t)); done
evolve_avg=$((evolve_total / ${#evolve_times[@]}))
log_info "  Average evolve time: ${evolve_avg}ms"

# ── Benchmark 3: Verify Speed ────────────────────────────────────────

log_info "Benchmark 3/5: Verify speed (n=6 constants batch)..."

# Key n=6 constants: 6, 12, 24, 144, 288, 720, 1024, 4096
VERIFY_VALUES=(6 12 24 48 144 288 720 1024 4096 8192)
verify_times=()
verify_json_items=""

for val in "${VERIFY_VALUES[@]}"; do
    ms=$(time_cmd "$BINARY verify $val")
    verify_times+=("$ms")
    verify_json_items="${verify_json_items}\"v${val}\": ${ms}, "
done

verify_total=0
for t in "${verify_times[@]}"; do verify_total=$((verify_total + t)); done
verify_avg=$((verify_total / ${#verify_times[@]}))
verify_count=${#verify_times[@]}
# ops/sec = count / (total_ms / 1000)
if [[ "$verify_total" -gt 0 ]]; then
    verify_ops_sec=$(perl -e "printf '%.1f', $verify_count / ($verify_total / 1000.0)")
else
    verify_ops_sec="inf"
fi
log_info "  Verified ${verify_count} values in ${verify_total}ms (${verify_ops_sec} ops/sec)"

# ── Benchmark 4: Test Speed ──────────────────────────────────────────

test_ms="null"
test_count="null"
if ! $QUICK; then
    log_info "Benchmark 4/5: Test speed (cargo test)..."
    test_start=$(perl -MTime::HiRes=time -e 'printf "%.3f", time()')
    test_output=$($CARGO test 2>&1) || true
    test_end=$(perl -MTime::HiRes=time -e 'printf "%.3f", time()')
    test_ms=$(perl -e "printf '%.0f', ($test_end - $test_start) * 1000")

    test_count=$(echo "$test_output" | grep "test result:" | \
        sed 's/.*\([0-9][0-9]*\) passed.*/\1/' | \
        awk '{s+=$1} END {print s+0}') || test_count=0
    log_info "  cargo test: ${test_ms}ms (${test_count} tests passed)"
else
    log_info "Benchmark 4/5: Skipped (--quick)"
fi

# ── Benchmark 5: Build Speed ─────────────────────────────────────────

build_ms="null"
if ! $QUICK && ! $NO_BUILD; then
    log_info "Benchmark 5/5: Build speed (cargo build --release)..."
    # Clean first for consistent measurement
    $CARGO clean 2>/dev/null || true
    build_start=$(perl -MTime::HiRes=time -e 'printf "%.3f", time()')
    $CARGO build --release 2>&1 | tail -3
    build_end=$(perl -MTime::HiRes=time -e 'printf "%.3f", time()')
    build_ms=$(perl -e "printf '%.0f', ($build_end - $build_start) * 1000")
    log_info "  cargo build --release: ${build_ms}ms"
else
    log_info "Benchmark 5/5: Skipped (--quick or --no-build)"
fi

# ── Generate JSON ────────────────────────────────────────────────────

# Remove trailing comma+space from json items
scan_json_items="${scan_json_items%, }"
evolve_json_items="${evolve_json_items%, }"
verify_json_items="${verify_json_items%, }"

cat > "$RESULTS_FILE" <<EOF
{
  "timestamp": "${TIMESTAMP}",
  "version": "$(${BINARY} help 2>&1 | head -1 | tr -d '\n' || echo 'unknown')",
  "scan": {
    "times_ms": { ${scan_json_items} },
    "avg_ms": ${scan_avg},
    "domains_tested": ${#scan_times[@]}
  },
  "evolve": {
    "times_ms": { ${evolve_json_items} },
    "avg_ms": ${evolve_avg},
    "domains_tested": ${#evolve_times[@]}
  },
  "verify": {
    "times_ms": { ${verify_json_items} },
    "avg_ms": ${verify_avg},
    "total_ms": ${verify_total},
    "count": ${verify_count},
    "ops_per_sec": ${verify_ops_sec}
  },
  "test": {
    "time_ms": ${test_ms},
    "tests_passed": ${test_count}
  },
  "build": {
    "time_ms": ${build_ms}
  }
}
EOF

# ── Summary ──────────────────────────────────────────────────────────

echo ""
echo "  ╔══════════════════════════════════════════╗"
echo "  ║   Benchmark Results                      ║"
echo "  ╠══════════════════════════════════════════╣"
printf "  ║   Scan avg:    %6d ms                 ║\n" "$scan_avg"
printf "  ║   Evolve avg:  %6d ms                 ║\n" "$evolve_avg"
printf "  ║   Verify:      %6s ops/sec            ║\n" "$verify_ops_sec"
if [[ "$test_ms" != "null" ]]; then
printf "  ║   Tests:       %6d ms (%s passed)    ║\n" "$test_ms" "$test_count"
fi
if [[ "$build_ms" != "null" ]]; then
printf "  ║   Build:       %6d ms                 ║\n" "$build_ms"
fi
echo "  ╠══════════════════════════════════════════╣"
echo "  ║   Saved: ~/.nexus6/benchmark-results.json║"
echo "  ╚══════════════════════════════════════════╝"
echo ""
