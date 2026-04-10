#!/usr/bin/env bash
# NEXUS-6 Single Lens Growth
# Usage: ./grow_lens.sh <lens_name>
#
# Implements one lens using Claude Code CLI.
# Steps:
#   1. Check if lens exists in registry (metadata)
#   2. Check if lens has Lens trait implementation
#   3. If not implemented, use Claude to implement it
#   4. Verify: cargo check + cargo test
#   5. Report success/failure
set -euo pipefail

CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"
NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$NEXUS_ROOT"

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <lens_name>"
    echo ""
    echo "Examples:"
    echo "  $0 consciousness"
    echo "  $0 gravity"
    echo "  $0 thermo"
    exit 1
fi

LENS_NAME="$1"
LENS_FILE_SNAKE=$(echo "$LENS_NAME" | tr '[:upper:]' '[:lower:]' | tr '-' '_')

echo "=== NEXUS-6 Lens Growth: $LENS_NAME ==="
echo ""

# Step 1: Check registry for metadata
echo "[1/5] Checking registry for lens metadata..."
if grep -rq "\"$LENS_NAME\"\|\"${LENS_FILE_SNAKE}\"" src/telescope/*.rs 2>/dev/null; then
    echo "  Found lens metadata in registry."
else
    echo "  WARNING: Lens '$LENS_NAME' not found in registry metadata."
    echo "  Claude will create both metadata and implementation."
fi

# Step 2: Check if Lens trait is already implemented
echo "[2/5] Checking for existing Lens trait implementation..."
impl_file="src/telescope/lenses/${LENS_FILE_SNAKE}_lens.rs"
has_impl=false
if [[ -f "$impl_file" ]]; then
    if grep -q "impl Lens for" "$impl_file" 2>/dev/null; then
        if grep -q "fn scan" "$impl_file" 2>/dev/null; then
            # Check if it's a stub (returns empty)
            if grep -q "HashMap::new()" "$impl_file" 2>/dev/null; then
                echo "  Found stub implementation at $impl_file — will upgrade."
            else
                echo "  Lens already has full implementation at $impl_file"
                echo "  Nothing to do. Exiting."
                exit 0
            fi
        fi
    fi
else
    echo "  No implementation file found. Will create $impl_file"
fi

# Step 3: Use Claude Code CLI to implement
echo "[3/5] Generating implementation with Claude Code CLI..."
echo ""

# Build the prompt
PROMPT="In the NEXUS-6 project at $NEXUS_ROOT, implement the '$LENS_NAME' telescope lens.

The Lens trait is defined in src/telescope/lens_trait.rs:
\`\`\`rust
pub type LensResult = HashMap<String, Vec<f64>>;

pub trait Lens: Send + Sync {
    fn name(&self) -> &str;
    fn category(&self) -> &str;
    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult;
}
\`\`\`

SharedData (src/telescope/shared_data.rs) contains pre-computed distance matrices.

Requirements:
1. Create or update: src/telescope/lenses/${LENS_FILE_SNAKE}_lens.rs
2. The scan() method must do REAL analysis (not return empty HashMap)
3. Include at least 2 #[test] functions that verify scan() output
4. Make sure it compiles with: cargo check
5. Register the struct in src/telescope/lenses/mod.rs if not already there
6. Use only dependencies already in Cargo.toml (rayon, serde, serde_json, blake3)

Do NOT create stub implementations. Write real scan logic appropriate for the '$LENS_NAME' domain."

# Run Claude non-interactively
if $CLAUDE_CLI -p "$PROMPT" --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null; then
    echo ""
    echo "  Claude generation completed."
else
    echo ""
    echo "  ERROR: Claude Code CLI failed. Skipping."
    exit 1
fi

# Step 4: Verify
echo "[4/5] Verifying implementation..."
echo ""

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

# Step 5: Report
echo ""
echo "[5/5] Result:"
if $check_ok && $test_ok; then
    echo "  SUCCESS: Lens '$LENS_NAME' implemented and verified."
    echo "  File: $impl_file"
else
    echo "  FAILED: Implementation did not pass verification."
    echo "  Reverting changes..."
    git checkout -- src/ 2>/dev/null || true
    echo "  Changes reverted."
    exit 1
fi
