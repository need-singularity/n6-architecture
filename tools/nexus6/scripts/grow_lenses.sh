#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════
# NEXUS-6 Lens Growth — Batch auto-implement lenses using Claude CLI
# ═══════════════════════════════════════════════════════════════
#
# Usage: ./grow_lenses.sh [--batch N] [--category CATEGORY] [--dry-run]
#
# Default: implement 6 lenses per run (n=6)
#
# Options:
#   --batch N        Number of lenses to implement (default: 6)
#   --category CAT   Filter to a specific category (n6, physics, accel, cross, all)
#   --dry-run        Show what would be implemented without running Claude
#   --help           Show this help
#
# The script:
#   1. Lists all implemented lenses (files in src/telescope/lenses/)
#   2. Lists all metadata-only lenses (names in registry source files)
#   3. Picks next batch based on priority
#   4. For each lens: generate prompt -> run Claude -> cargo check -> cargo test
#   5. Reports success/failure count

set -euo pipefail

CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"
NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$NEXUS_ROOT"

# ─── Defaults ───────────────────────────────────────────────────────────────────
BATCH_SIZE=6  # n=6
CATEGORY="all"
DRY_RUN=false

# ─── Priority order: lens names to implement first ──────────────────────────────
LENS_PRIORITY=(
    # n6_lenses — high impact industry lenses
    "chip_architecture"
    "battery_chemistry"
    "solar_efficiency"
    "fusion_plasma"
    "transformer_anatomy"
    "moe_routing"
    # physics_deep — scientific rigor
    "lattice_field"
    "renormalization"
    "conformal_bootstrap"
    "topological_insulator"
    "bose_einstein"
    "feynman_path"
    # accel_a — practical ML
    "speculative_decode"
    "flash_attention_lens"
    "kernel_fusion"
    "batch_optimization"
    "gradient_checkpointing"
    "mixed_precision"
    # cross — cross-project synergy
    "cross_domain_resonance"
    "hypothesis_linker"
    "atlas_pattern"
    # n6_lenses continued — verification/quality
    "isomorphism"
    "emergence"
    "periodicity"
    "completeness"
    "surprise"
    "frustration"
    "inverse"
    "combinatorial"
    "extrapolation"
    # physics_deep continued
    "electromagnetic_wave"
    "maxwell_equation"
    "superconductor_lens"
    "heat_conduction"
    "faraday_induction"
    "magnetic_monopole"
    "convection_pattern"
    # more accel
    "sparse_topology"
    "quantization_lens"
    "pruning_landscape"
    "loss_curvature"
    "activation_atlas"
)

# ─── Category filters ──────────────────────────────────────────────────────────
# Category sources (bash 3.2 compatible — no associative arrays)
category_source_for() {
    case "$1" in
        n6)      echo "n6_lenses" ;;
        physics) echo "physics_deep_lenses" ;;
        accel)   echo "accel_lenses_a accel_lenses_b accel_lenses_c accel_lenses_d" ;;
        cross)   echo "cross_lenses" ;;
        tecs)    echo "tecs_lenses" ;;
        anima)   echo "anima_lenses" ;;
        sedi)    echo "sedi_lenses" ;;
        *)       echo "" ;;
    esac
}

# ─── Parse args ─────────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --batch)
            BATCH_SIZE="$2"
            shift 2
            ;;
        --category)
            CATEGORY="$2"
            shift 2
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help|-h)
            head -20 "$0" | tail -18
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# ─── Step 1: List implemented lenses ───────────────────────────────────────────
echo "═══════════════════════════════════════════════════════════════"
echo "  NEXUS-6 Lens Growth Engine — Batch Mode"
echo "  Batch size: $BATCH_SIZE   Category: $CATEGORY"
echo "═══════════════════════════════════════════════════════════════"
echo ""

echo "[1/5] Scanning implemented lenses..."
IMPLEMENTED=()
for f in src/telescope/lenses/*_lens.rs; do
    if [[ -f "$f" ]]; then
        # Extract lens name from filename (remove _lens.rs suffix and path prefix)
        basename_f=$(basename "$f")
        lens_name="${basename_f%_lens.rs}"
        # Verify it has a real scan() implementation (not just a stub)
        if grep -q "fn scan" "$f" 2>/dev/null; then
            IMPLEMENTED+=("$lens_name")
        fi
    fi
done
echo "  Found ${#IMPLEMENTED[@]} implemented lenses."

# ─── Step 2: List metadata-only lenses ─────────────────────────────────────────
echo "[2/5] Scanning metadata-only lenses..."
ALL_META_NAMES=()

# Extract lens names from metadata source files
extract_lens_names() {
    local file="$1"
    if [[ -f "$file" ]]; then
        grep 'name:.*"' "$file" | sed 's/.*name:[[:space:]]*"//' | sed 's/".*//' | sort -u
    fi
}

# Collect from all sources
for src_file in src/telescope/n6_lenses.rs \
                src/telescope/physics_deep_lenses.rs \
                src/telescope/accel_lenses_a.rs \
                src/telescope/accel_lenses_b.rs \
                src/telescope/accel_lenses_c.rs \
                src/telescope/accel_lenses_d.rs \
                src/telescope/tecs_lenses.rs \
                src/telescope/anima_lenses.rs \
                src/telescope/sedi_lenses.rs \
                src/telescope/cross_lenses.rs \
                src/telescope/quantum_lenses.rs; do
    if [[ -f "$src_file" ]]; then
        while IFS= read -r name; do
            ALL_META_NAMES+=("$name")
        done < <(extract_lens_names "$src_file")
    fi
done

# Filter out already-implemented lenses
UNIMPLEMENTED=()
for meta_name in "${ALL_META_NAMES[@]}"; do
    is_impl=false
    for impl_name in "${IMPLEMENTED[@]}"; do
        if [[ "$meta_name" == "$impl_name" ]]; then
            is_impl=true
            break
        fi
    done
    if ! $is_impl; then
        UNIMPLEMENTED+=("$meta_name")
    fi
done

echo "  Found ${#ALL_META_NAMES[@]} total metadata entries."
echo "  Unimplemented: ${#UNIMPLEMENTED[@]} lenses."
echo ""

# Implementation rate
if [[ ${#ALL_META_NAMES[@]} -gt 0 ]]; then
    RATE=$(echo "scale=1; ${#IMPLEMENTED[@]} * 100 / ${#ALL_META_NAMES[@]}" | bc 2>/dev/null || echo "?")
    echo "  Implementation rate: ${#IMPLEMENTED[@]}/${#ALL_META_NAMES[@]} ($RATE%)"
else
    echo "  Implementation rate: ${#IMPLEMENTED[@]}/? (no metadata found)"
fi
echo ""

# ─── Step 3: Pick next batch ───────────────────────────────────────────────────
echo "[3/5] Selecting next batch of $BATCH_SIZE lenses..."

BATCH=()
BATCH_COUNT=0

# First: add from priority list (in order)
for pname in "${LENS_PRIORITY[@]}"; do
    if [[ $BATCH_COUNT -ge $BATCH_SIZE ]]; then
        break
    fi
    # Check it's in unimplemented list
    for uname in "${UNIMPLEMENTED[@]}"; do
        if [[ "$pname" == "$uname" ]]; then
            # Category filter
            if [[ "$CATEGORY" != "all" ]]; then
                # Check if this lens is in the target category source
                found_in_cat=false
                for src_id in $(category_source_for "$CATEGORY"); do
                    src_path="src/telescope/${src_id}.rs"
                    if [[ -f "$src_path" ]] && grep -q "\"$pname\"" "$src_path" 2>/dev/null; then
                        found_in_cat=true
                        break
                    fi
                done
                if ! $found_in_cat; then
                    continue
                fi
            fi
            BATCH+=("$pname")
            BATCH_COUNT=$((BATCH_COUNT + 1))
            break
        fi
    done
done

# If batch not full, fill from unimplemented (in order they appear)
if [[ $BATCH_COUNT -lt $BATCH_SIZE ]]; then
    for uname in "${UNIMPLEMENTED[@]}"; do
        if [[ $BATCH_COUNT -ge $BATCH_SIZE ]]; then
            break
        fi
        # Skip if already in batch
        in_batch=false
        for bname in "${BATCH[@]}"; do
            if [[ "$uname" == "$bname" ]]; then
                in_batch=true
                break
            fi
        done
        if $in_batch; then
            continue
        fi
        BATCH+=("$uname")
        BATCH_COUNT=$((BATCH_COUNT + 1))
    done
fi

echo "  Selected batch:"
for i in "${!BATCH[@]}"; do
    echo "    $((i+1)). ${BATCH[$i]}"
done
echo ""

if [[ ${#BATCH[@]} -eq 0 ]]; then
    echo "  No lenses to implement. All done!"
    exit 0
fi

if $DRY_RUN; then
    echo "  [DRY RUN] Would implement ${#BATCH[@]} lenses. Exiting."
    exit 0
fi

# ─── Step 4: Implement each lens ───────────────────────────────────────────────
echo "[4/5] Implementing lenses..."
echo ""

SUCCESS_COUNT=0
FAIL_COUNT=0
RESULTS=()

for lens_name in "${BATCH[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Growing: $lens_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    SNAKE_NAME=$(echo "$lens_name" | tr '[:upper:]' '[:lower:]' | tr '-' '_')
    PASCAL_NAME=$(echo "$SNAKE_NAME" | sed -r 's/(^|_)([a-z])/\U\2/g' 2>/dev/null || \
                  echo "$SNAKE_NAME" | perl -pe 's/(^|_)(\w)/uc($2)/ge' 2>/dev/null || \
                  echo "$SNAKE_NAME")
    IMPL_FILE="src/telescope/lenses/${SNAKE_NAME}_lens.rs"

    # Skip if already exists with real implementation
    if [[ -f "$IMPL_FILE" ]] && grep -q "fn scan" "$IMPL_FILE" 2>/dev/null && ! grep -q "HashMap::new()" "$IMPL_FILE" 2>/dev/null; then
        echo "  Already implemented. Skipping."
        RESULTS+=("$lens_name: SKIPPED (already exists)")
        continue
    fi

    # Look up description from metadata files
    DESCRIPTION=""
    for src_file in src/telescope/n6_lenses.rs \
                    src/telescope/physics_deep_lenses.rs \
                    src/telescope/accel_lenses_a.rs \
                    src/telescope/accel_lenses_b.rs \
                    src/telescope/accel_lenses_c.rs \
                    src/telescope/accel_lenses_d.rs \
                    src/telescope/tecs_lenses.rs \
                    src/telescope/anima_lenses.rs \
                    src/telescope/sedi_lenses.rs \
                    src/telescope/cross_lenses.rs; do
        if [[ -f "$src_file" ]]; then
            # Find the name line, then look for the next description line
            DESCRIPTION=$(awk -v name="$lens_name" '
                $0 ~ "name:.*\""name"\"" { found=1; next }
                found && /description:/ {
                    gsub(/.*description:[[:space:]]*"/, "")
                    gsub(/".*/, "")
                    print
                    exit
                }
            ' "$src_file" 2>/dev/null || true)
            if [[ -n "$DESCRIPTION" ]]; then
                break
            fi
        fi
    done
    if [[ -z "$DESCRIPTION" ]]; then
        DESCRIPTION="Analyze $lens_name patterns in data"
    fi

    # Build Claude prompt
    PROMPT="In the NEXUS-6 project at $NEXUS_ROOT, implement the '$lens_name' telescope lens.

The Lens trait is defined in src/telescope/lens_trait.rs:
\`\`\`rust
pub type LensResult = HashMap<String, Vec<f64>>;

pub trait Lens: Send + Sync {
    fn name(&self) -> &str;
    fn category(&self) -> &str;
    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult;
}
\`\`\`

SharedData (src/telescope/shared_data.rs) contains pre-computed distance matrix.
Use shared.dist(i, j) for pairwise distances (panics if i==j).

Lens: \"$lens_name\"
Description: \"$DESCRIPTION\"

Requirements:
1. Create: src/telescope/lenses/${SNAKE_NAME}_lens.rs
2. The scan() method must do REAL mathematical analysis (not empty HashMap)
3. Include at least 2 #[cfg(test)] tests that verify non-empty scan() output
4. Add 'pub mod ${SNAKE_NAME}_lens;' and 'pub use ${SNAKE_NAME}_lens::${PASCAL_NAME}Lens;' to src/telescope/lenses/mod.rs
5. Use only std library (no external crates)
6. Data is row-major: point i, dimension d = data[i * d_count + d_idx]

Reference existing implementation: src/telescope/lenses/consciousness_lens.rs

Do NOT create stubs. Write real mathematical analysis logic."

    # Run Claude
    echo "  Running Claude CLI..."
    if $CLAUDE_CLI -p "$PROMPT" --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null; then
        echo "  Claude generation completed."
    else
        echo "  ERROR: Claude CLI failed."
        RESULTS+=("$lens_name: FAIL (Claude error)")
        FAIL_COUNT=$((FAIL_COUNT + 1))
        continue
    fi

    # Verify: cargo check
    echo "  Running cargo check..."
    if ~/.cargo/bin/cargo check 2>&1 | tail -5; then
        echo "  cargo check: OK"
    else
        echo "  cargo check: FAILED — reverting."
        git checkout -- "src/telescope/lenses/${SNAKE_NAME}_lens.rs" 2>/dev/null || true
        # Also revert mod.rs changes for this lens
        git checkout -- src/telescope/lenses/mod.rs 2>/dev/null || true
        RESULTS+=("$lens_name: FAIL (cargo check)")
        FAIL_COUNT=$((FAIL_COUNT + 1))
        continue
    fi

    # Verify: cargo test (only tests for this lens)
    echo "  Running cargo test..."
    if ~/.cargo/bin/cargo test "${SNAKE_NAME}" 2>&1 | tail -10; then
        echo "  cargo test: OK"
    else
        echo "  cargo test: FAILED — reverting."
        git checkout -- "src/telescope/lenses/${SNAKE_NAME}_lens.rs" 2>/dev/null || true
        git checkout -- src/telescope/lenses/mod.rs 2>/dev/null || true
        RESULTS+=("$lens_name: FAIL (cargo test)")
        FAIL_COUNT=$((FAIL_COUNT + 1))
        continue
    fi

    echo "  SUCCESS: $lens_name implemented."
    RESULTS+=("$lens_name: SUCCESS")
    SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    echo ""
done

# ─── Step 5: Report ────────────────────────────────────────────────────────────
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  NEXUS-6 Lens Growth Report"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "  Batch size:  $BATCH_SIZE"
echo "  Succeeded:   $SUCCESS_COUNT"
echo "  Failed:      $FAIL_COUNT"
echo ""
echo "  Results:"
for result in "${RESULTS[@]}"; do
    echo "    - $result"
done
echo ""

# Updated implementation count
NEW_IMPL_COUNT=$(find src/telescope/lenses/ -name "*_lens.rs" | wc -l | tr -d ' ')
echo "  Total implemented lenses: $NEW_IMPL_COUNT"
echo ""

# Progress bar
if [[ ${#ALL_META_NAMES[@]} -gt 0 ]]; then
    TOTAL=${#ALL_META_NAMES[@]}
    PCT=$((NEW_IMPL_COUNT * 100 / TOTAL))
    BAR_LEN=50
    FILLED=$((PCT * BAR_LEN / 100))
    EMPTY=$((BAR_LEN - FILLED))
    BAR=$(printf '%*s' "$FILLED" | tr ' ' '#')
    SPACE=$(printf '%*s' "$EMPTY" | tr ' ' '-')
    echo "  [$BAR$SPACE] $PCT% ($NEW_IMPL_COUNT/$TOTAL)"
fi
echo ""
echo "═══════════════════════════════════════════════════════════════"
