#!/usr/bin/env bash
# NEXUS-6 Test Growth — Add Tests to Under-Tested Module
# Usage: ./grow_tests.sh [module_name]
#
# If no module_name given, auto-detects the module with fewest tests.
set -euo pipefail

CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"
NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$NEXUS_ROOT"

MODULE_NAME="${1:-}"

# Auto-detect under-tested module if none specified
if [[ -z "$MODULE_NAME" ]]; then
    echo "=== Auto-detecting under-tested module ==="
    echo ""

    # Find modules and count their #[test] functions
    best_module=""
    best_ratio=999999

    for mod_dir in src/*/; do
        mod=$(basename "$mod_dir")
        [[ "$mod" == "telescope" ]] && continue  # telescope has many sub-files, skip

        rs_count=$(find "$mod_dir" -name "*.rs" | wc -l | tr -d ' ')
        [[ "$rs_count" -eq 0 ]] && continue

        test_count=$(grep -r "#\[test\]" "$mod_dir" 2>/dev/null | wc -l | tr -d ' ')
        code_lines=$(find "$mod_dir" -name "*.rs" -exec cat {} + 2>/dev/null | wc -l | tr -d ' ')

        # Ratio: tests per 100 lines of code
        if [[ "$code_lines" -gt 50 ]]; then
            ratio=$((test_count * 100 / code_lines))
            if [[ "$ratio" -lt "$best_ratio" ]]; then
                best_ratio=$ratio
                best_module=$mod
            fi
            printf "  %-20s %3d tests / %5d lines (ratio: %d%%)\n" "$mod" "$test_count" "$code_lines" "$ratio"
        fi
    done

    echo ""
    if [[ -z "$best_module" ]]; then
        echo "No suitable module found."
        exit 0
    fi
    MODULE_NAME="$best_module"
    echo "Selected: $MODULE_NAME (lowest test ratio: ${best_ratio}%)"
fi

echo ""
echo "=== NEXUS-6 Test Growth: $MODULE_NAME ==="
echo ""

MOD_DIR="src/$MODULE_NAME"
if [[ ! -d "$MOD_DIR" ]]; then
    echo "ERROR: Module directory $MOD_DIR does not exist."
    exit 1
fi

# Count current tests
before_tests=$(grep -r "#\[test\]" "$MOD_DIR" 2>/dev/null | wc -l | tr -d ' ')
echo "Current test count: $before_tests"
echo ""

# List the files in the module for context
rs_files=$(find "$MOD_DIR" -name "*.rs" | sort)
file_list=""
for f in $rs_files; do
    file_list="$file_list  - $f"$'\n'
done

# Build Claude prompt
PROMPT="In the NEXUS-6 project at $NEXUS_ROOT, add comprehensive tests to the '$MODULE_NAME' module.

Module files:
$file_list

Current test count: $before_tests

Requirements:
1. Read each .rs file in src/$MODULE_NAME/
2. Add #[cfg(test)] mod tests { ... } blocks with meaningful test functions
3. Each public function should have at least one test
4. Test edge cases: empty input, single element, large values
5. All tests must pass with 'cargo test'
6. Do NOT modify non-test code — only add tests
7. Use only dependencies already in Cargo.toml (rayon, serde, serde_json, blake3)
8. Add at least 3 new test functions total"

echo "Generating tests with Claude Code CLI..."
echo ""

if $CLAUDE_CLI -p "$PROMPT" --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null; then
    echo ""
    echo "Claude generation completed."
else
    echo ""
    echo "ERROR: Claude Code CLI failed."
    exit 1
fi

# Verify
echo ""
echo "Verifying..."

check_ok=true
echo "  Running cargo check..."
if ! ~/.cargo/bin/cargo check 2>&1; then
    echo "  FAIL: cargo check failed."
    check_ok=false
fi

test_ok=true
if $check_ok; then
    echo "  Running cargo test..."
    if ! ~/.cargo/bin/cargo test 2>&1; then
        echo "  FAIL: cargo test failed."
        test_ok=false
    fi
fi

after_tests=$(grep -r "#\[test\]" "$MOD_DIR" 2>/dev/null | wc -l | tr -d ' ')
added=$((after_tests - before_tests))

echo ""
if $check_ok && $test_ok; then
    echo "SUCCESS: Added $added tests to $MODULE_NAME ($before_tests -> $after_tests)"
else
    echo "FAILED: New tests did not pass verification."
    echo "Reverting changes..."
    git checkout -- src/ 2>/dev/null || true
    echo "Changes reverted."
    exit 1
fi
