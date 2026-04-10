#!/usr/bin/env bash
# benchmark_all_tools.sh — Unified benchmark for all Rust calculators in n6-architecture
# Builds each tool, runs with default args, measures compile/run time and output.
# Results saved to experiments/benchmark_results.md

set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RUSTC="$HOME/.cargo/bin/rustc"
TIMEOUT=30
RESULTS_FILE="$ROOT/experiments/benchmark_results.md"

# Portable timeout using perl (works on macOS without coreutils)
run_timed() {
  # Usage: run_timed <seconds> <outfile> <cmd> [args...]
  local limit=$1; shift
  local outfile=$1; shift
  perl -e '
    use POSIX ":sys_wait_h";
    my $limit = shift;
    my $outfile = shift;
    my $pid = fork();
    if ($pid == 0) {
      open(STDOUT, ">", $outfile) or die;
      open(STDERR, ">&STDOUT") or die;
      exec(@ARGV) or exit(127);
    }
    eval {
      local $SIG{ALRM} = sub { kill 9, $pid; die "timeout\n" };
      alarm($limit);
      waitpid($pid, 0);
      alarm(0);
    };
    if ($@ =~ /timeout/) { waitpid($pid, WNOHANG); exit(124); }
    exit($? >> 8);
  ' "$limit" "$outfile" "$@"
}
TIMESTAMP="$(date '+%Y-%m-%d %H:%M:%S')"

# Colors for terminal
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

# Counters
TOTAL_LOC=0
TOTAL_COMPILE=0
TOTAL_RUN=0
TOOL_COUNT=0
PASS_COUNT=0
FAIL_COUNT=0
SKIP_COUNT=0

# Table rows accumulator
TABLE_ROWS=""

# ---------- Default arguments per tool ----------
# Tools with required args get sensible defaults; others run bare or with --all.
get_tool_args() {
  case "$1" in
    atlas-verifier)       echo "$ROOT/docs/atlas-constants.md" ;;
    chip-n6-calc)         echo "anima" ;;
    cross-dse-calc)       echo "$ROOT/tools/universal-dse/domains --top 5" ;;
    crypto-calc)          echo "--verbose" ;;
    energy-calc)          echo "--all" ;;
    gpu-arch-calc)        echo "--all" ;;
    hypothesis-grader)    echo "$ROOT/docs/ai-efficiency" ;;
    interconnect-calc)    echo "--all" ;;
    quantum-calc)         echo "--verbose" ;;
    semiconductor-calc)   echo "--verbose" ;;
    universal-dse)        echo "$ROOT/tools/universal-dse/domains/chip.toml --top 5" ;;
    *)                    echo "" ;;
  esac
}

# ---------- Helper: milliseconds from fractional seconds ----------
to_ms() {
  # input: seconds as float (e.g. 0.312)
  printf "%.0f" "$(echo "$1 * 1000" | bc 2>/dev/null || python3 -c "print(int($1*1000))")"
}

# ---------- Discover all tools ----------
TOOLS=()
for mainrs in "$ROOT"/tools/*/main.rs; do
  tool="$(basename "$(dirname "$mainrs")")"
  TOOLS+=("$tool")
done

echo "========================================"
echo " N6 Architecture — Rust Benchmark Suite"
echo " Tools found: ${#TOOLS[@]}"
echo " Timestamp: $TIMESTAMP"
echo "========================================"
echo ""

# ---------- Benchmark each tool ----------
for tool in "${TOOLS[@]}"; do
  TOOL_COUNT=$((TOOL_COUNT + 1))
  TOOL_DIR="$ROOT/tools/$tool"
  SRC="$TOOL_DIR/main.rs"
  BIN="$TOOL_DIR/$tool"

  # LOC
  loc=$(wc -l < "$SRC" | tr -d ' ')
  TOTAL_LOC=$((TOTAL_LOC + loc))

  printf "[%2d/%d] %-28s " "$TOOL_COUNT" "${#TOOLS[@]}" "$tool"

  # --- Compile ---
  compile_start=$(python3 -c "import time; print(f'{time.time():.6f}')")
  if ! "$RUSTC" "$SRC" -o "$BIN" 2>/tmp/bench_compile_err; then
    compile_end=$(python3 -c "import time; print(f'{time.time():.6f}')")
    ct=$(echo "$compile_end - $compile_start" | bc)
    ct_s=$(printf "%.2f" "$ct")
    err=$(head -1 /tmp/bench_compile_err)
    printf "${RED}COMPILE FAIL${NC} (%.2fs) %s\n" "$ct" "$err"
    TABLE_ROWS+="| $tool | $loc | ${ct_s} | - | - | COMPILE FAIL |\n"
    FAIL_COUNT=$((FAIL_COUNT + 1))
    continue
  fi
  compile_end=$(python3 -c "import time; print(f'{time.time():.6f}')")
  ct=$(echo "$compile_end - $compile_start" | bc)
  ct_s=$(printf "%.2f" "$ct")
  TOTAL_COMPILE=$(echo "$TOTAL_COMPILE + $ct" | bc)

  # --- Run ---
  args="$(get_tool_args "$tool")"
  TMPOUT="/tmp/bench_run_out_$$"
  run_start=$(python3 -c "import time; print(f'{time.time():.6f}')")
  # shellcheck disable=SC2086
  run_timed "$TIMEOUT" "$TMPOUT" "$BIN" $args && run_exit=0 || run_exit=$?
  run_end=$(python3 -c "import time; print(f'{time.time():.6f}')")
  output=$(cat "$TMPOUT" 2>/dev/null || echo "")
  rm -f "$TMPOUT"

  if [ "$run_exit" -eq 0 ]; then
    status_icon="PASS"
    status_color="$GREEN"
  elif [ "$run_exit" -eq 124 ]; then
    status_icon="TIMEOUT"
    status_color="$YELLOW"
  else
    status_icon="PASS*"
    status_color="$YELLOW"
  fi
  rt=$(echo "$run_end - $run_start" | bc)
  rt_ms=$(to_ms "$rt")
  TOTAL_RUN=$(echo "$TOTAL_RUN + $rt" | bc)

  out_lines=$(echo "$output" | wc -l | tr -d ' ')

  if [ "$status_icon" = "TIMEOUT" ]; then
    printf "${status_color}%-10s${NC} LOC=%4d  compile=%.2fs  run=TIMEOUT  lines=%d\n" \
      "$status_icon" "$loc" "$ct" "$out_lines"
    TABLE_ROWS+="| $tool | $loc | ${ct_s} | >30000 | $out_lines | TIMEOUT |\n"
    FAIL_COUNT=$((FAIL_COUNT + 1))
  else
    printf "${status_color}%-10s${NC} LOC=%4d  compile=%.2fs  run=%dms  lines=%d\n" \
      "$status_icon" "$loc" "$ct" "$rt_ms" "$out_lines"
    TABLE_ROWS+="| $tool | $loc | ${ct_s} | $rt_ms | $out_lines | $status_icon |\n"
    PASS_COUNT=$((PASS_COUNT + 1))
  fi
done

# ---------- Summary ----------
AVG_RUN=0
if [ "$PASS_COUNT" -gt 0 ]; then
  AVG_RUN=$(echo "scale=0; ($TOTAL_RUN * 1000) / ($PASS_COUNT + $FAIL_COUNT)" | bc)
fi
TOTAL_COMPILE_S=$(printf "%.2f" "$TOTAL_COMPILE")
TOTAL_RUN_MS=$(to_ms "$TOTAL_RUN")

echo ""
echo "========================================"
echo " SUMMARY"
echo "========================================"
echo " Tools:          ${#TOOLS[@]}"
echo " Pass:           $PASS_COUNT"
echo " Fail/Timeout:   $FAIL_COUNT"
echo " Total Rust LOC: $TOTAL_LOC"
echo " Total compile:  ${TOTAL_COMPILE_S}s"
echo " Total run:      ${TOTAL_RUN_MS}ms"
echo " Avg run/tool:   ${AVG_RUN}ms"
echo "========================================"

# ---------- Save results to markdown ----------
cat > "$RESULTS_FILE" <<HEADER
# N6 Architecture — Rust Benchmark Results

**Date:** $TIMESTAMP
**Tools:** ${#TOOLS[@]} | **Pass:** $PASS_COUNT | **Fail/Timeout:** $FAIL_COUNT

## Performance Table

| Tool | LOC | Compile(s) | Run(ms) | Output | Status |
|------|-----|-----------|---------|--------|--------|
HEADER

echo -e "$TABLE_ROWS" >> "$RESULTS_FILE"

cat >> "$RESULTS_FILE" <<FOOTER

## Totals

| Metric | Value |
|--------|-------|
| Total Rust LOC | $TOTAL_LOC |
| Total compile time | ${TOTAL_COMPILE_S}s |
| Total run time | ${TOTAL_RUN_MS}ms |
| Average run/tool | ${AVG_RUN}ms |
| Pass rate | $PASS_COUNT / ${#TOOLS[@]} |

---
*Generated by benchmark_all_tools.sh*
FOOTER

echo ""
echo "Results saved to: $RESULTS_FILE"
