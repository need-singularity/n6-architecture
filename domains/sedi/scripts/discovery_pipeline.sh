#!/usr/bin/env bash
# SEDI Discovery Pipeline — chains discovery tools into a single run
# Usage: ./discovery_pipeline.sh [--full | --quick | --grade-only | --cross]
#
# Modes:
#   --quick      : Discovery Engine only (fastest, <5s)
#   --full       : Engine + Miner + Grade + Sync (complete pipeline)
#   --grade-only : Grade hypotheses + Sync to atlas (no discovery)
#   --cross      : Cross-tool feedback pipeline (all 4 tools + report)

set -euo pipefail

SEDI_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DATA_DIR="$SEDI_ROOT/data"
TOOLS_DIR="$SEDI_ROOT/tools"
SCRIPTS_DIR="$SEDI_ROOT/scripts"

# Ensure data directories exist
mkdir -p "$DATA_DIR/discoveries" "$DATA_DIR/pipeline"

# Colors for output
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; BLUE='\033[0;34m'; NC='\033[0m'

MODE="${1:---full}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT="$DATA_DIR/pipeline/run_${TIMESTAMP}.md"

# ─── Cross-tool mode: delegate to cross_pipeline.py ───
if [[ "$MODE" == "--cross" ]]; then
    echo -e "${BLUE}═══════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  SEDI Cross-Tool Pipeline${NC}"
    echo -e "${BLUE}  $(date)${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════${NC}"
    shift  # remove --cross from args
    exec python3 "$SCRIPTS_DIR/cross_pipeline.py" "$@"
fi

echo -e "${BLUE}═══════════════════════════════════════════════${NC}"
echo -e "${BLUE}  SEDI Discovery Pipeline — ${MODE}${NC}"
echo -e "${BLUE}  $(date)${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════${NC}"

# Initialize report
cat > "$REPORT" << EOF
# SEDI Discovery Pipeline Report
- **Date**: $(date)
- **Mode**: ${MODE}
- **Run ID**: ${TIMESTAMP}

EOF

TOTAL_START=$(date +%s)

# ─── Phase 1: Build Rust Tools ───
echo -e "\n${YELLOW}[1/5] Building Rust tools...${NC}"
PHASE1_START=$(date +%s)

if [[ "$MODE" != "--grade-only" ]]; then
    (cd "$TOOLS_DIR/discovery-engine" && ~/.cargo/bin/cargo build --release 2>&1 | tail -1) || true
    if [[ "$MODE" == "--full" ]]; then
        (cd "$TOOLS_DIR/formula-miner" && ~/.cargo/bin/cargo build --release 2>&1 | tail -1) || true
    fi
    echo -e "${GREEN}  ✓ Rust tools built${NC}"
else
    echo -e "${GREEN}  ✓ Skipped (grade-only mode)${NC}"
fi

PHASE1_END=$(date +%s)
echo "## Phase 1: Build ($((PHASE1_END - PHASE1_START))s)" >> "$REPORT"

# ─── Phase 2: Discovery Engine ───
if [[ "$MODE" != "--grade-only" ]]; then
    echo -e "\n${YELLOW}[2/5] Running Discovery Engine...${NC}"
    PHASE2_START=$(date +%s)

    ENGINE_OUT="$DATA_DIR/discoveries/engine_${TIMESTAMP}.json"
    if "$TOOLS_DIR/discovery-engine/target/release/sedi-discovery-engine" --json > "$ENGINE_OUT" 2>/dev/null; then
        ENGINE_COUNT=$(grep -c '"operator"' "$ENGINE_OUT" 2>/dev/null || echo "?")
        echo -e "${GREEN}  ✓ ${ENGINE_COUNT} discoveries found${NC}"
    elif "$TOOLS_DIR/discovery-engine/target/release/sedi-discovery-engine" > "$DATA_DIR/discoveries/engine_${TIMESTAMP}.txt" 2>/dev/null; then
        ENGINE_COUNT=$(wc -l < "$DATA_DIR/discoveries/engine_${TIMESTAMP}.txt" | tr -d ' ')
        echo -e "${GREEN}  ✓ Engine output: ${ENGINE_COUNT} lines${NC}"
    else
        ENGINE_COUNT="0"
        echo -e "${RED}  ⚠ Discovery engine failed${NC}"
    fi

    PHASE2_END=$(date +%s)
    echo "## Phase 2: Discovery Engine ($((PHASE2_END - PHASE2_START))s)" >> "$REPORT"
    echo "- Discoveries: ${ENGINE_COUNT}" >> "$REPORT"
else
    echo -e "\n${YELLOW}[2/5] Discovery Engine — skipped (grade-only)${NC}"
fi

# ─── Phase 3: Formula Miner ───
if [[ "$MODE" == "--full" ]]; then
    echo -e "\n${YELLOW}[3/5] Running Formula Miner (genetic search)...${NC}"
    PHASE3_START=$(date +%s)

    MINER_OUT="$DATA_DIR/discoveries/miner_${TIMESTAMP}.txt"
    if "$TOOLS_DIR/formula-miner/target/release/sedi-formula-miner" > "$MINER_OUT" 2>/dev/null; then
        MINER_COUNT=$(grep -c "^[0-9]" "$MINER_OUT" 2>/dev/null || echo "?")
        EXACT_COUNT=$(grep -c "EXACT" "$MINER_OUT" 2>/dev/null || echo "0")
        echo -e "${GREEN}  ✓ ${MINER_COUNT} formulas, ${EXACT_COUNT} exact matches${NC}"
    else
        MINER_COUNT="0"
        EXACT_COUNT="0"
        echo -e "${RED}  ⚠ Formula miner failed${NC}"
    fi

    PHASE3_END=$(date +%s)
    echo "## Phase 3: Formula Miner ($((PHASE3_END - PHASE3_START))s)" >> "$REPORT"
    echo "- Formulas: ${MINER_COUNT}, Exact: ${EXACT_COUNT}" >> "$REPORT"
else
    echo -e "\n${YELLOW}[3/5] Formula Miner — skipped${NC}"
fi

# ─── Phase 4: Grade Hypotheses ───
echo -e "\n${YELLOW}[4/5] Grading hypotheses (Bayesian tiers)...${NC}"
PHASE4_START=$(date +%s)

if python3 "$SCRIPTS_DIR/auto_grade_n6.py" 2>/dev/null; then
    if [[ -f "$DATA_DIR/sedi-grades.json" ]]; then
        TIER_A=$(python3 -c "
import json; d=json.load(open('$DATA_DIR/sedi-grades.json'))
td = d.get('tier_distribution', {})
if td:
    print(td.get('A', 0))
else:
    print(sum(1 for h in d.get('hypotheses',d.get('grades',[])) if h.get('tier','')=='A'))
" 2>/dev/null || echo "?")
        TIER_B=$(python3 -c "
import json; d=json.load(open('$DATA_DIR/sedi-grades.json'))
td = d.get('tier_distribution', {})
if td:
    print(td.get('B', 0))
else:
    print(sum(1 for h in d.get('hypotheses',d.get('grades',[])) if h.get('tier','')=='B'))
" 2>/dev/null || echo "?")
        echo -e "${GREEN}  ✓ Graded: ${TIER_A} Tier A, ${TIER_B} Tier B${NC}"
    else
        echo -e "${GREEN}  ✓ Grading complete${NC}"
    fi
else
    echo -e "${RED}  ⚠ auto_grade_n6.py had issues${NC}"
fi

PHASE4_END=$(date +%s)
echo "## Phase 4: Grading ($((PHASE4_END - PHASE4_START))s)" >> "$REPORT"

# ─── Phase 5: Sync to Atlas ───
if [[ "$MODE" == "--full" ]]; then
    echo -e "\n${YELLOW}[5/5] Syncing to TECS-L atlas...${NC}"
    PHASE5_START=$(date +%s)

    if python3 "$SCRIPTS_DIR/sync_to_atlas.py" 2>/dev/null; then
        echo -e "${GREEN}  ✓ Atlas sync complete${NC}"
    else
        echo -e "${RED}  ⚠ sync_to_atlas.py had issues${NC}"
    fi

    PHASE5_END=$(date +%s)
    echo "## Phase 5: Atlas Sync ($((PHASE5_END - PHASE5_START))s)" >> "$REPORT"
else
    echo -e "\n${YELLOW}[5/5] Atlas Sync — skipped${NC}"
fi

# ─── Summary ───
TOTAL_END=$(date +%s)
TOTAL_TIME=$((TOTAL_END - TOTAL_START))

echo -e "\n${BLUE}═══════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Pipeline complete in ${TOTAL_TIME}s${NC}"
echo -e "${BLUE}  Report: ${REPORT}${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════${NC}"

echo -e "\n---\n**Total time**: ${TOTAL_TIME}s" >> "$REPORT"
