#!/usr/bin/env bash
# NEXUS-6 Growth History Report
# Usage: ./growth_report.sh [--last N]
#
# Reads growth_log.jsonl and displays a trend table.
set -euo pipefail

NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG_FILE="$NEXUS_ROOT/scripts/growth_log.jsonl"

last_n=20

while [[ $# -gt 0 ]]; do
    case "$1" in
        --last) last_n="$2"; shift 2 ;;
        *) echo "Unknown flag: $1"; exit 1 ;;
    esac
done

if [[ ! -f "$LOG_FILE" ]]; then
    echo "No growth log found at $LOG_FILE"
    echo "Run auto_grow.sh first to generate data."
    exit 0
fi

total_lines=$(wc -l < "$LOG_FILE" | tr -d ' ')
if [[ "$total_lines" -eq 0 ]]; then
    echo "Growth log is empty."
    exit 0
fi

echo ""
echo "  ┌──────────────────────────────────────────────────────────────────────┐"
echo "  │  NEXUS-6 Growth Report (last $last_n of $total_lines cycles)                      │"
echo "  ├───────┬────────┬─────────┬─────────┬────────┬────────────────────────┤"
echo "  │ Cycle │ Tests  │  Lines  │ Lenses  │ Warns  │ Action                 │"
echo "  ├───────┼────────┼─────────┼─────────┼────────┼────────────────────────┤"

tail -n "$last_n" "$LOG_FILE" | while IFS= read -r line; do
    cycle=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('cycle','-'))" 2>/dev/null || echo "-")
    tests=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('tests_pass','-'))" 2>/dev/null || echo "-")
    lines=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); v=d.get('code_lines',0); print(f'{v/1000:.1f}K') if v else print('-')" 2>/dev/null || echo "-")
    lenses=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('lenses_registered','-'))" 2>/dev/null || echo "-")
    warns=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('warnings','-'))" 2>/dev/null || echo "-")
    action=$(echo "$line" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('action','measure')[:22])" 2>/dev/null || echo "-")
    printf "  │ %5s │ %6s │ %7s │ %7s │ %6s │ %-22s │\n" "$cycle" "$tests" "$lines" "$lenses" "$warns" "$action"
done

echo "  └───────┴────────┴─────────┴─────────┴────────┴────────────────────────┘"

# Growth rate calculation
if [[ "$total_lines" -ge 2 ]]; then
    echo ""
    python3 -c "
import json, sys

lines = open('$LOG_FILE').readlines()
if len(lines) >= 2:
    first = json.loads(lines[0])
    last = json.loads(lines[-1])
    n = len(lines) - 1
    if n > 0:
        t0, t1 = first.get('tests_pass',0), last.get('tests_pass',0)
        l0, l1 = first.get('lenses_registered',0), last.get('lenses_registered',0)
        c0, c1 = first.get('code_lines',0), last.get('code_lines',0)
        def rate(a, b, steps):
            if a and steps: return ((b - a) / steps / a * 100) if a else 0
            return 0
        print(f'  Growth rates over {n} cycles:')
        print(f'    Tests:  {t0} -> {t1} ({rate(t0,t1,n):+.1f}%/cycle)')
        print(f'    Lenses: {l0} -> {l1} ({rate(l0,l1,n):+.1f}%/cycle)')
        print(f'    Code:   {c0} -> {c1} lines ({rate(c0,c1,n):+.1f}%/cycle)')
" 2>/dev/null || true
fi
echo ""
