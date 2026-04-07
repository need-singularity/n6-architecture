#!/usr/bin/env bash
set -euo pipefail
# NEXUS-6 Growth Dashboard
# =========================
# Shows current state of all 15 growth dimensions with progress bars,
# growth velocity (trend arrows), and critical dimension highlighting.
#
# Usage: ./growth_dashboard.sh [--live] [--last N]

NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SCRIPT_DIR="$NEXUS_ROOT/scripts"
LOG_FILE="$SCRIPT_DIR/growth_log.jsonl"
REPO_ROOT="$(cd "$NEXUS_ROOT/../.." && pwd)"

cd "$NEXUS_ROOT"

# Parse arguments
LIVE=false
LAST_N=10

while [[ $# -gt 0 ]]; do
    case "$1" in
        --live)  LIVE=true; shift ;;
        --last)  LAST_N="$2"; shift 2 ;;
        -h|--help)
            echo "NEXUS-6 Growth Dashboard"
            echo ""
            echo "Usage: $0 [--live] [--last N]"
            echo ""
            echo "Options:"
            echo "  --live     Continuously refresh dashboard (Ctrl+C to stop)"
            echo "  --last N   Show trends from last N log entries (default: 10)"
            exit 0
            ;;
        *) echo "Unknown flag: $1"; exit 1 ;;
    esac
done

# ── Targets (same as registry.rs) ────────────────────────────────────

render_dashboard() {
    python3 -c "
import json, sys, os

log_file = '$LOG_FILE'
last_n = $LAST_N

# Targets matching registry.rs
targets = {
    'performance':     10000,
    'architecture':      100,
    'lenses':            200,
    'modules':           4.0,
    'tests':            1000,
    'hypotheses':        150,
    'dse':               322,
    'experiments':        50,
    'calculators':        50,
    'cross_resonance':   100,
    'knowledge_graph':   500,
    'red_team':          100,
    'atlas':            2000,
    'documentation':      90,
    'integration':        50,
}

dim_labels = {
    'performance':     'Performance',
    'architecture':    'Architecture',
    'lenses':          'Lenses',
    'modules':         'Modules',
    'tests':           'Tests',
    'hypotheses':      'Hypotheses',
    'dse':             'DSE',
    'experiments':     'Experiments',
    'calculators':     'Calculators',
    'cross_resonance': 'CrossReson',
    'knowledge_graph': 'KnowledgeGr',
    'red_team':        'RedTeam',
    'atlas':           'Atlas',
    'documentation':   'Docs',
    'integration':     'Integration',
}

# Read log entries
entries = []
if os.path.exists(log_file):
    with open(log_file, 'r') as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    entries.append(json.loads(line))
                except json.JSONDecodeError:
                    pass

if not entries:
    print('No growth log data found.')
    print(f'Run nexus_growth_daemon.sh first to generate data at {log_file}')
    sys.exit(0)

# Use last N entries
recent = entries[-last_n:]
latest = recent[-1]

# Compute velocity (change from first to last in window)
first = recent[0] if len(recent) > 1 else latest
n_steps = max(len(recent) - 1, 1)

print()
print('+' + '='*67 + '+')
print('|' + ' '*15 + 'NEXUS-6 GROWTH DASHBOARD' + ' '*28 + '|')
cycle = latest.get('cycle', '?')
ts = latest.get('timestamp', '?')
print(f'|  Cycle: {cycle:<8}  Time: {ts:<30}         |')
print('+' + '='*67 + '+')
print('| Dimension    | Current | Target  | Progress       | Vel  | Health  |')
print('+--------------+---------+---------+----------------+------+---------+')

# Sort by gap (worst first)
dims_sorted = sorted(targets.keys(), key=lambda d: -(1.0 - min(float(latest.get(d, 0)) / max(targets[d], 0.001), 1.0)))

overall_pct_sum = 0
critical_count = 0
thriving_count = 0

for d in dims_sorted:
    cur = float(latest.get(d, 0))
    tgt = targets[d]
    pct = min(cur / max(tgt, 0.001), 1.0) * 100

    # Progress bar (12 chars)
    filled = int(pct / 100 * 12)
    bar = '#' * filled + '.' * (12 - filled)

    # Velocity
    prev_val = float(first.get(d, 0))
    vel = (cur - prev_val) / n_steps
    if vel > 1.0:
        vel_arrow = '^^'
    elif vel > 0.01:
        vel_arrow = '^ '
    elif vel < -0.01:
        vel_arrow = 'v '
    else:
        vel_arrow = '- '

    # Health
    if pct < 25:
        health = 'CRIT'
        critical_count += 1
    elif cur < prev_val:
        health = 'DOWN'
    elif pct >= 90:
        health = 'BEST'
        thriving_count += 1
    elif pct >= 50:
        health = 'OK  '
    else:
        health = 'LOW '

    overall_pct_sum += pct / 100

    cur_s = f'{cur:.0f}' if cur >= 10 else f'{cur:.1f}'
    tgt_s = f'{tgt:.0f}' if tgt >= 10 else f'{tgt:.1f}'
    label = dim_labels.get(d, d)

    print(f'| {label:<12} | {cur_s:>7} | {tgt_s:>7} | {bar} {pct:>3.0f}% | {vel_arrow:>3}  | {health:<7} |')

overall_pct = overall_pct_sum / len(targets) * 100
print('+--------------+---------+---------+----------------+------+---------+')
print(f'| Overall: {overall_pct:>5.1f}%  | Critical: {critical_count}  | Thriving: {thriving_count}  | Dims: {len(targets)}       |')
print('+' + '-'*67 + '+')

# Recent actions
print()
print('Recent actions:')
for e in recent[-6:]:  # last n=6
    c = e.get('cycle', '?')
    d = e.get('dimension', e.get('action', '?'))
    s = 'OK' if e.get('success', False) else 'FAIL'
    print(f'  Cycle {c}: {d} [{s}]')

print()
" 2>/dev/null
}

# ── Main ─────────────────────────────────────────────────────────────

if $LIVE; then
    trap 'echo ""; echo "Dashboard stopped."; exit 0' SIGINT SIGTERM
    while true; do
        clear 2>/dev/null || true
        render_dashboard
        echo "  [Live mode — refreshing every 30s, Ctrl+C to stop]"
        sleep 30
    done
else
    render_dashboard
fi
