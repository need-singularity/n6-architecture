#!/usr/bin/env bash
set -euo pipefail

# NEXUS-6 Daily Growth Report
# ============================
# Generates a comprehensive daily summary from growth_log.jsonl.
# Reads last 24h of entries, computes deltas, and produces an ASCII report.
#
# Usage: ./growth_daily_report.sh [--days N] [--output FILE]
#
# Output: ~/.nexus6/reports/daily-YYYY-MM-DD.txt

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
NEXUS6_HOME="$HOME/.nexus6"
LOG_FILE="$SCRIPT_DIR/growth_log.jsonl"
DAYS=1
OUTPUT_DIR="$NEXUS6_HOME/reports"
OUTPUT_FILE=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --days)    DAYS="$2"; shift 2 ;;
        --output)  OUTPUT_FILE="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [--days N] [--output FILE]"
            exit 0
            ;;
        *) shift ;;
    esac
done

mkdir -p "$OUTPUT_DIR"

TODAY=$(date +%Y-%m-%d)
if [[ -z "$OUTPUT_FILE" ]]; then
    OUTPUT_FILE="$OUTPUT_DIR/daily-${TODAY}.txt"
fi

# ── Generate report via Python ────────────────────────────────────────

export LOG_FILE DAYS TODAY

python3 << 'PYEOF' > "$OUTPUT_FILE"
import json, sys, os
from datetime import datetime, timedelta, timezone

log_file = os.environ.get("LOG_FILE", "growth_log.jsonl")
days = int(os.environ.get("DAYS", "1"))
today = os.environ.get("TODAY", datetime.now().strftime("%Y-%m-%d"))

# Parse log entries
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

# Filter last N days
cutoff = datetime.now(timezone.utc) - timedelta(days=days)
recent = []
older = []
for e in entries:
    ts_str = e.get("timestamp", "")
    try:
        if ts_str.endswith("Z"):
            ts = datetime.fromisoformat(ts_str.replace("Z", "+00:00"))
        elif "+" in ts_str or ts_str.count("-") > 2:
            ts = datetime.fromisoformat(ts_str)
        else:
            ts = datetime.fromisoformat(ts_str).replace(tzinfo=timezone.utc)
        if ts >= cutoff:
            recent.append(e)
        else:
            older.append(e)
    except (ValueError, TypeError):
        recent.append(e)  # include if we can't parse timestamp

# Compute stats
total_cycles = len(recent)
successes = sum(1 for e in recent if e.get("success", False))
failures = total_cycles - successes
success_rate = (successes / total_cycles * 100) if total_cycles > 0 else 0

# Dimensions touched
dim_counts = {}
dim_successes = {}
for e in recent:
    d = e.get("dimension", "unknown")
    dim_counts[d] = dim_counts.get(d, 0) + 1
    if e.get("success", False):
        dim_successes[d] = dim_successes.get(d, 0) + 1

# Compute deltas (first entry vs last entry in window)
key_metrics = ["tests", "lenses", "code_lines", "hypotheses", "atlas",
               "dse", "experiments", "integration", "red_team", "knowledge_graph"]

first_vals = {}
last_vals = {}
if recent:
    for k in key_metrics:
        first_vals[k] = recent[0].get(k, 0)
        last_vals[k] = recent[-1].get(k, 0)
elif entries:
    for k in key_metrics:
        first_vals[k] = entries[-1].get(k, 0)
        last_vals[k] = entries[-1].get(k, 0)
else:
    for k in key_metrics:
        first_vals[k] = 0
        last_vals[k] = 0

# Week-over-week: count recent successful cycles as a proxy
this_week_cycles = sum(1 for e in entries[-100:] if e.get("success", False))

# Find stagnant dimensions (0 delta)
stagnant = []
for k in key_metrics:
    if last_vals[k] == first_vals[k] and last_vals[k] == 0:
        stagnant.append(k)

# Generate report
W = 62  # inner width

def bar(count, max_val):
    if max_val <= 0:
        return ""
    width = 20
    filled = int(count / max_val * width)
    return "\u2588" * filled + "\u2591" * (width - filled)

def delta_str(before, after):
    d = after - before
    sign = "+" if d >= 0 else ""
    if before > 0:
        pct = d / before * 100
        return f"{sign}{d} ({sign}{pct:.1f}%)"
    elif d > 0:
        return f"{sign}{d} (new)"
    else:
        return "0"

def fmt_num(n):
    if n >= 1000:
        return f"{n/1000:.0f}K"
    return str(n)

lines = []
lines.append("\u2554" + "\u2550" * W + "\u2557")
lines.append(f"\u2551  NEXUS-6 Daily Growth Report -- {today:<29}\u2551")
lines.append("\u2560" + "\u2550" * W + "\u2563")
lines.append(f"\u2551{' ' * W}\u2551")
lines.append(f"\u2551  Cycles today:    {total_cycles:<5} ({days * 24}h window){' ' * (W - 40)}\u2551")
lines.append(f"\u2551  Successes:       {successes:<5} ({success_rate:.1f}%){' ' * (W - 36)}\u2551")
lines.append(f"\u2551  Failures:        {failures:<5}{' ' * (W - 25)}\u2551")
lines.append(f"\u2551{' ' * W}\u2551")
lines.append(f"\u2551  Growth Summary:{' ' * (W - 18)}\u2551")

for k in ["tests", "lenses", "code_lines", "hypotheses", "atlas"]:
    label = k.replace("_", " ").title()
    b = first_vals[k]
    a = last_vals[k]
    ds = delta_str(b, a)
    row = f"    {label:<14} {fmt_num(b):>6} -> {fmt_num(a):>6}  ({ds})"
    padding = W - len(row) - 2
    if padding < 0:
        row = row[:W-2]
        padding = 0
    lines.append(f"\u2551{row}{' ' * padding}\u2551")

lines.append(f"\u2551{' ' * W}\u2551")
lines.append(f"\u2551  Dimensions touched:{' ' * (W - 22)}\u2551")

max_dim_count = max(dim_counts.values()) if dim_counts else 1
for d, c in sorted(dim_counts.items(), key=lambda x: -x[1]):
    b = bar(c, max_dim_count)
    succ = dim_successes.get(d, 0)
    row = f"    {d:<18} {b} {c:>3} cycles ({succ} ok)"
    padding = W - len(row) - 2
    if padding < 0:
        row = row[:W-2]
        padding = 0
    lines.append(f"\u2551{row}{' ' * padding}\u2551")

lines.append(f"\u2551{' ' * W}\u2551")

# Top achievements
achievements = []
for k in key_metrics:
    d = last_vals[k] - first_vals[k]
    if d > 0:
        label = k.replace("_", " ")
        achievements.append(f"  +{d} {label}")

if achievements:
    lines.append(f"\u2551  Top achievements:{' ' * (W - 20)}\u2551")
    for a in achievements[:6]:
        row = f"    \u2705 {a}"
        padding = W - len(row) - 2
        if padding < 0:
            row = row[:W-2]
            padding = 0
        lines.append(f"\u2551{row}{' ' * padding}\u2551")
else:
    lines.append(f"\u2551  No measurable growth in this window.{' ' * (W - 40)}\u2551")

lines.append(f"\u2551{' ' * W}\u2551")

# Issues
if stagnant:
    lines.append(f"\u2551  Issues:{' ' * (W - 10)}\u2551")
    for s in stagnant[:5]:
        row = f"    \u26a0\ufe0f  {s} still at 0 (needs attention)"
        padding = W - len(row) - 2
        if padding < 0:
            row = row[:W-2]
            padding = 0
        lines.append(f"\u2551{row}{' ' * padding}\u2551")
    lines.append(f"\u2551{' ' * W}\u2551")

lines.append("\u255a" + "\u2550" * W + "\u255d")

report = "\n".join(lines)
print(report)
PYEOF

# Also print to stdout
cat "$OUTPUT_FILE"

echo ""
echo "[+] Report saved to: $OUTPUT_FILE"
