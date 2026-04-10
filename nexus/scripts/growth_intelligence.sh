#!/usr/bin/env bash
set -euo pipefail

# NEXUS-6 Growth Intelligence — Adaptive Strategy
# =================================================
# Learns from past growth patterns in growth_log.jsonl.
# Called by the daemon before picking the next growth action.
#
# Outputs JSON: {"recommended_dimension": "...", "confidence": 0.85,
#                "reason": "...", "skip_list": [...]}
#
# Strategy:
#   - If a dimension failed 3x in a row -> cooldown for 6 cycles
#   - High success rate -> boost weight
#   - Track per-dimension success rates -> focus on high-ROI
#   - If Claude CLI is slow -> prefer non-Claude dimensions

CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
LOG_FILE="$SCRIPT_DIR/growth_log.jsonl"

# If no log file, output neutral recommendation
if [[ ! -f "$LOG_FILE" ]] || [[ ! -s "$LOG_FILE" ]]; then
    echo '{"recommended_dimension":"","confidence":0.0,"reason":"no history","skip_list":[]}'
    exit 0
fi

python3 << 'PYEOF'
import json, os, sys
from datetime import datetime, timedelta, timezone
from collections import defaultdict

log_file = os.environ.get("LOG_FILE", "growth_log.jsonl")

# Parse all entries
entries = []
if os.path.exists(log_file):
    with open(log_file) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                entries.append(json.loads(line))
            except json.JSONDecodeError:
                continue

if not entries:
    print(json.dumps({
        "recommended_dimension": "",
        "confidence": 0.0,
        "reason": "no history",
        "skip_list": []
    }))
    sys.exit(0)

# ── Analyze per-dimension stats ───────────────────────────────────────

all_dims = [
    "performance", "architecture", "lenses", "modules", "tests",
    "hypotheses", "dse", "experiments", "calculators", "cross_resonance",
    "knowledge_graph", "red_team", "atlas", "documentation", "integration"
]

# Targets (same as daemon)
targets = {
    "performance": 10000, "architecture": 100, "lenses": 200, "modules": 4.0,
    "tests": 1000, "hypotheses": 150, "dse": 322, "experiments": 50,
    "calculators": 50, "cross_resonance": 100, "knowledge_graph": 500,
    "red_team": 100, "atlas": 2000, "documentation": 90, "integration": 50,
}

# Impact weights (same as daemon)
weights = {
    "tests": 0.12, "lenses": 0.10, "architecture": 0.10,
    "performance": 0.08, "hypotheses": 0.08, "integration": 0.08,
    "dse": 0.06, "knowledge_graph": 0.06, "red_team": 0.06,
    "cross_resonance": 0.05, "atlas": 0.05, "experiments": 0.05,
    "calculators": 0.04, "modules": 0.04, "documentation": 0.03,
}

# Per-dimension success/fail counts and streaks
dim_total = defaultdict(int)
dim_success = defaultdict(int)
dim_fail_streak = defaultdict(int)  # current consecutive failures
dim_last_cycle = defaultdict(int)   # last cycle number this dim was tried

# Track from entries
for i, e in enumerate(entries):
    d = e.get("dimension", "")
    if d not in all_dims:
        continue
    dim_total[d] += 1
    if e.get("success", False):
        dim_success[d] += 1
        dim_fail_streak[d] = 0
    else:
        dim_fail_streak[d] += 1
    dim_last_cycle[d] = i

total_entries = len(entries)

# ── Cooldown: dims with 3+ consecutive failures ──────────────────────

COOLDOWN_THRESHOLD = 3   # failures in a row
COOLDOWN_CYCLES = 6      # n=6 cooldown

skip_list = []
for d in all_dims:
    if dim_fail_streak[d] >= COOLDOWN_THRESHOLD:
        # Check if enough cycles have passed since last attempt
        cycles_since = total_entries - dim_last_cycle.get(d, 0)
        if cycles_since < COOLDOWN_CYCLES:
            skip_list.append(d)

# ── Compute smart scores ─────────────────────────────────────────────

# Get latest metrics from last entry
latest = entries[-1] if entries else {}

scores = {}
for d in all_dims:
    if d in skip_list:
        continue

    # Gap score (how far from target)
    current = float(latest.get(d, 0))
    target = targets.get(d, 100)
    gap = max(0.0, 1.0 - current / max(target, 0.001))

    # Weight
    w = weights.get(d, 0.05)

    # Success rate bonus (prefer dims that actually succeed)
    total = dim_total.get(d, 0)
    succ = dim_success.get(d, 0)
    if total > 0:
        success_rate = succ / total
    else:
        success_rate = 0.5  # unknown = neutral

    # Recency penalty (if tried very recently, slight penalty)
    cycles_since = total_entries - dim_last_cycle.get(d, -10)
    recency_factor = min(1.0, cycles_since / 3.0)  # full weight after 3 cycles

    # Combined score: gap * weight * success_boost * recency
    # Success rate acts as a multiplier (range 0.3 to 1.5)
    success_boost = 0.3 + success_rate * 1.2
    score = gap * w * success_boost * recency_factor

    scores[d] = {
        "score": score,
        "gap": gap,
        "success_rate": success_rate,
        "total_attempts": total,
        "recency": cycles_since,
    }

# ── Pick best dimension ──────────────────────────────────────────────

if not scores:
    # Everything is in cooldown, reset and pick by gap alone
    skip_list = []
    for d in all_dims:
        current = float(latest.get(d, 0))
        target = targets.get(d, 100)
        gap = max(0.0, 1.0 - current / max(target, 0.001))
        scores[d] = {"score": gap * weights.get(d, 0.05), "gap": gap,
                      "success_rate": 0.5, "total_attempts": 0, "recency": 999}

best_dim = max(scores, key=lambda d: scores[d]["score"])
best_info = scores[best_dim]

# Confidence: based on how much data we have + how clear the winner is
sorted_scores = sorted(scores.values(), key=lambda x: -x["score"])
if len(sorted_scores) >= 2 and sorted_scores[0]["score"] > 0:
    margin = (sorted_scores[0]["score"] - sorted_scores[1]["score"]) / sorted_scores[0]["score"]
else:
    margin = 0.0

data_confidence = min(1.0, total_entries / 20.0)  # max confidence after 20 entries
confidence = 0.3 + 0.4 * margin + 0.3 * data_confidence
confidence = round(min(1.0, confidence), 2)

# Reason
if best_info["gap"] > 0.9:
    reason = f"critical gap ({best_info['gap']:.0%} from target)"
elif best_info["success_rate"] > 0.7 and best_info["total_attempts"] >= 3:
    reason = f"high ROI (success rate {best_info['success_rate']:.0%})"
elif best_info["recency"] > 5:
    reason = f"not attempted in {best_info['recency']} cycles"
else:
    reason = f"best weighted score ({best_info['score']:.3f})"

result = {
    "recommended_dimension": best_dim,
    "confidence": confidence,
    "reason": reason,
    "skip_list": skip_list,
    "stats": {
        d: {
            "score": round(scores[d]["score"], 4),
            "success_rate": round(scores[d]["success_rate"], 2),
            "attempts": scores[d]["total_attempts"],
        }
        for d in sorted(scores, key=lambda x: -scores[x]["score"])[:5]
    }
}

print(json.dumps(result))
PYEOF
