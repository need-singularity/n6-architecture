#!/usr/bin/env bash
# NEXUS-6 Metrics Collection
# Usage: ./measure.sh
#
# Outputs a single JSON line with current project metrics.
# Used standalone or called by auto_grow.sh.
set -euo pipefail

NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$NEXUS_ROOT"

timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# --- File counts ---
rust_files=$(find src -name "*.rs" | wc -l | tr -d ' ')
code_lines=$(find src -name "*.rs" -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')
modules=$(find src -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d ' ')

# --- Tests ---
tests_total=0
tests_pass=0
tests_fail=0
compile_ok=true

test_output=$( ~/.cargo/bin/cargo test 2>&1 ) || true

# Parse "test result: ok. X passed; Y failed; Z ignored"
if echo "$test_output" | grep -q "test result:"; then
    # Sum all test result lines (lib + bin + integration)
    tests_pass=$(echo "$test_output" | grep "test result:" | \
        sed 's/.*\([0-9][0-9]*\) passed.*/\1/' | \
        awk '{s+=$1} END {print s+0}')
    tests_fail=$(echo "$test_output" | grep "test result:" | \
        sed 's/.*; \([0-9][0-9]*\) failed.*/\1/' | \
        awk '{s+=$1} END {print s+0}')
    tests_total=$((tests_pass + tests_fail))
fi

# Check if compilation succeeded
if echo "$test_output" | grep -q "^error\["; then
    compile_ok=false
fi

# --- Warnings ---
check_output=$( ~/.cargo/bin/cargo check 2>&1 ) || true
warnings=$(echo "$check_output" | grep -c "^warning\[" 2>/dev/null || true)
warnings=${warnings:-0}
warnings=$(echo "$warnings" | tr -d '[:space:]')

# --- Lenses registered ---
lenses_registered=$(grep -r "LensEntry" src/telescope/*.rs 2>/dev/null | \
    grep -c "LensEntry {" || echo "0")

# --- Test function count (static) ---
test_fns=$(grep -r "#\[test\]" src/ tests/ 2>/dev/null | wc -l | tr -d ' ')

# --- Output JSON ---
cat <<EOF
{"timestamp":"${timestamp}","modules":${modules},"rust_files":${rust_files},"code_lines":${code_lines},"tests_total":${tests_total},"tests_pass":${tests_pass},"tests_fail":${tests_fail},"test_fns":${test_fns},"warnings":${warnings},"lenses_registered":${lenses_registered},"compile_ok":${compile_ok}}
EOF
