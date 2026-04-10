#!/usr/bin/env bash
set -uo pipefail
CLAUDE_CLI="${CLAUDE_CLI:-/Users/ghost/.local/bin/claude}"

# NEXUS-6 Troubleshooting Auto-Updater
# Usage:
#   ./troubleshoot_update.sh --record-failure "dimension" "error_message"
#   ./troubleshoot_update.sh --record-solution "PS-007" "solution_text"
#   ./troubleshoot_update.sh --add-rule "rule_text" "reason"
#   ./troubleshoot_update.sh --check-rules   (validate current scripts against known rules)
#   ./troubleshoot_update.sh --auto-fix       (use Claude CLI to fix known patterns)
#   ./troubleshoot_update.sh --report         (print convergence report)

TROUBLESHOOT_FILE="$HOME/.nexus/troubleshoot.json"
NEXUS_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Ensure file exists
if [[ ! -f "$TROUBLESHOOT_FILE" ]]; then
    echo '{"absolute_rules":[],"problems_solved":[],"failure_patterns":[],"convergence_metrics":{"total_problems_encountered":0,"total_problems_solved":0,"total_absolute_rules":0,"unsolved_problems":0,"reliability_score":1.0,"consecutive_success_streak":0}}' > "$TROUBLESHOOT_FILE"
fi

record_failure() {
    local dimension="$1"
    local error_msg="$2"
    local timestamp
    timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

    python3 -c "
import json, sys

with open('$TROUBLESHOOT_FILE', 'r') as f:
    db = json.load(f)

# Increment counters
m = db.setdefault('convergence_metrics', {})
m['total_problems_encountered'] = m.get('total_problems_encountered', 0) + 1
m['unsolved_problems'] = m.get('unsolved_problems', 0) + 1
m['consecutive_success_streak'] = 0
m['last_failure'] = '$timestamp'

# Update reliability
total = m['total_problems_encountered']
solved = m.get('total_problems_solved', 0)
m['reliability_score'] = round(solved / max(total, 1), 4)

# Check if this matches a known pattern
error = '''$error_msg'''
patterns = db.setdefault('failure_patterns', [])
matched = False
for p in patterns:
    if any(kw in error.lower() for kw in p.get('description', '').lower().split()):
        p['frequency'] = p.get('frequency', 0) + 1
        matched = True
        break

if not matched:
    # New failure pattern
    db.setdefault('unresolved_failures', []).append({
        'dimension': '$dimension',
        'error': error[:500],
        'timestamp': '$timestamp',
        'status': 'unresolved'
    })

with open('$TROUBLESHOOT_FILE', 'w') as f:
    json.dump(db, f, indent=2, ensure_ascii=False)

print(f'Recorded failure in $dimension. Reliability: {m[\"reliability_score\"]*100:.1f}%')
" 2>/dev/null || echo "Failed to update troubleshoot.json"
}

record_success() {
    python3 -c "
import json
with open('$TROUBLESHOOT_FILE', 'r') as f:
    db = json.load(f)
m = db.setdefault('convergence_metrics', {})
m['consecutive_success_streak'] = m.get('consecutive_success_streak', 0) + 1
total = m.get('total_problems_encountered', 0)
solved = m.get('total_problems_solved', 0)
if total > 0:
    m['reliability_score'] = round(solved / total, 4)
with open('$TROUBLESHOOT_FILE', 'w') as f:
    json.dump(db, f, indent=2, ensure_ascii=False)
" 2>/dev/null
}

record_solution() {
    local problem_id="$1"
    local solution="$2"
    local timestamp
    timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

    python3 -c "
import json
with open('$TROUBLESHOOT_FILE', 'r') as f:
    db = json.load(f)

m = db.setdefault('convergence_metrics', {})
m['total_problems_solved'] = m.get('total_problems_solved', 0) + 1
m['unsolved_problems'] = max(0, m.get('unsolved_problems', 0) - 1)
total = m.get('total_problems_encountered', 0)
solved = m['total_problems_solved']
m['reliability_score'] = round(solved / max(total, 1), 4)

# Mark unresolved as solved
for uf in db.get('unresolved_failures', []):
    if uf.get('status') == 'unresolved':
        uf['status'] = 'solved'
        uf['solution'] = '''$solution'''[:500]
        uf['solved_at'] = '$timestamp'
        break

with open('$TROUBLESHOOT_FILE', 'w') as f:
    json.dump(db, f, indent=2, ensure_ascii=False)
print('Solution recorded. Reliability: ' + str(m['reliability_score']*100) + '%')
" 2>/dev/null
}

add_rule() {
    local rule="$1"
    local reason="$2"
    local timestamp
    timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

    python3 -c "
import json
with open('$TROUBLESHOOT_FILE', 'r') as f:
    db = json.load(f)

rules = db.setdefault('absolute_rules', [])
next_id = 'AR-' + str(len(rules) + 1).zfill(3)
rules.append({
    'id': next_id,
    'rule': '''$rule'''[:300],
    'reason': '''$reason'''[:300],
    'discovered': '$timestamp',
    'affected': []
})
db.setdefault('convergence_metrics', {})['total_absolute_rules'] = len(rules)

with open('$TROUBLESHOOT_FILE', 'w') as f:
    json.dump(db, f, indent=2, ensure_ascii=False)
print(f'Rule {next_id} added. Total rules: {len(rules)}')
" 2>/dev/null
}

check_rules() {
    echo ""
    echo "NEXUS-6 Rule Compliance Check"
    echo "============================="
    echo ""

    local violations=0

    # AR-001: No declare -A
    local hits
    hits=$(grep -r "declare -A" "$NEXUS_ROOT/scripts/"*.sh 2>/dev/null | grep -v "troubleshoot_update.sh" | grep -v "# AR-001" | wc -l | tr -d '[:space:]')
    if [[ "$hits" -gt 0 ]]; then
        echo "  [FAIL] AR-001: declare -A found in $hits files"
        violations=$((violations + 1))
    else
        echo "  [ OK ] AR-001: No declare -A (bash 3.2 compatible)"
    fi

    # AR-002: No grep -c with glob
    hits=$(grep -rn "grep -c.*\*\.rs" "$NEXUS_ROOT/scripts/"*.sh 2>/dev/null | wc -l | tr -d '[:space:]')
    if [[ "$hits" -gt 0 ]]; then
        echo "  [FAIL] AR-002: grep -c with glob found in $hits places"
        violations=$((violations + 1))
    else
        echo "  [ OK ] AR-002: No grep -c with glob patterns"
    fi

    # AR-003: CLAUDE_CLI variable
    for f in "$NEXUS_ROOT/scripts/"*.sh; do
        if grep -q "claude -p\|claude --" "$f" 2>/dev/null; then
            if ! grep -q "CLAUDE_CLI" "$f" 2>/dev/null; then
                echo "  [FAIL] AR-003: $(basename "$f") uses claude without CLAUDE_CLI"
                violations=$((violations + 1))
            fi
        fi
    done
    if [[ $violations -eq 0 ]] || ! grep -rq "claude -p" "$NEXUS_ROOT/scripts/"*.sh 2>/dev/null; then
        echo "  [ OK ] AR-003: All scripts use CLAUDE_CLI variable"
    fi

    # AR-005: grow_lenses vs grow_lens
    if grep -q "grow_lens\.sh.*--batch" "$NEXUS_ROOT/scripts/nexus_growth_daemon.sh" 2>/dev/null; then
        echo "  [FAIL] AR-005: Daemon calls grow_lens.sh with --batch (should be grow_lenses.sh)"
        violations=$((violations + 1))
    else
        echo "  [ OK ] AR-005: Correct script names (grow_lens vs grow_lenses)"
    fi

    # AR-008: All src/ dirs have lib.rs registration
    local unregistered=0
    for dir in "$NEXUS_ROOT/src"/*/; do
        local modname
        modname=$(basename "$dir")
        if ! grep -q "pub mod $modname" "$NEXUS_ROOT/src/lib.rs" 2>/dev/null; then
            echo "  [FAIL] AR-008: Module '$modname' not registered in lib.rs"
            unregistered=$((unregistered + 1))
        fi
    done
    if [[ $unregistered -eq 0 ]]; then
        echo "  [ OK ] AR-008: All modules registered in lib.rs"
    fi

    echo ""
    echo "Violations: $violations"
    if [[ $violations -eq 0 ]]; then
        echo "Status: ALL RULES PASS"
    else
        echo "Status: $violations VIOLATIONS FOUND"
    fi
    echo ""

    return $violations
}

auto_fix() {
    echo "NEXUS-6 Auto-Fix — Using Claude CLI to resolve known issues"
    echo ""

    # Read unresolved failures
    local unresolved
    unresolved=$(python3 -c "
import json
with open('$TROUBLESHOOT_FILE', 'r') as f:
    db = json.load(f)
uf = [x for x in db.get('unresolved_failures', []) if x.get('status') == 'unresolved']
for item in uf[:3]:
    print(item.get('dimension','?') + '|||' + item.get('error','?')[:200])
" 2>/dev/null)

    if [[ -z "$unresolved" ]]; then
        echo "No unresolved failures. System is clean."
        return 0
    fi

    echo "$unresolved" | while IFS='|||' read -r dim err; do
        echo "Fixing: $dim — $err"
        $CLAUDE_CLI -p "In /Users/ghost/Dev/n6-architecture/tools/nexus/, fix this error that occurred during growth of '$dim' dimension: $err. Check troubleshoot.json at ~/.nexus/troubleshoot.json for known patterns and absolute rules. After fixing, run cargo check && cargo test to verify." \
            --allowedTools Edit,Write,Read,Bash,Grep,Glob 2>/dev/null || true
    done
}

print_report() {
    python3 -c "
import json

with open('$TROUBLESHOOT_FILE', 'r') as f:
    db = json.load(f)

m = db.get('convergence_metrics', {})
rules = db.get('absolute_rules', [])
solved = db.get('problems_solved', [])
patterns = db.get('failure_patterns', [])
unresolved = [x for x in db.get('unresolved_failures', []) if x.get('status') == 'unresolved']

rel = m.get('reliability_score', 0) * 100
streak = m.get('consecutive_success_streak', 0)
total_p = m.get('total_problems_encountered', 0)
total_s = m.get('total_problems_solved', 0)

print()
print('+' + '='*60 + '+')
print('|  NEXUS-6 Troubleshooting Convergence Report' + ' '*15 + '|')
print('+' + '='*60 + '+')
print(f'|  Reliability:        {rel:6.1f}%' + ' '*(35-len(f'{rel:.1f}')) + '|')
print(f'|  Success streak:     {streak}' + ' '*(38-len(str(streak))) + '|')
print(f'|  Problems total:     {total_p}' + ' '*(38-len(str(total_p))) + '|')
print(f'|  Problems solved:    {total_s}' + ' '*(38-len(str(total_s))) + '|')
print(f'|  Unresolved:         {len(unresolved)}' + ' '*(38-len(str(len(unresolved)))) + '|')
print(f'|  Absolute rules:     {len(rules)}' + ' '*(38-len(str(len(rules)))) + '|')
print('+' + '-'*60 + '+')

if patterns:
    print('|  Failure Patterns:' + ' '*41 + '|')
    for p in sorted(patterns, key=lambda x: -x.get('frequency',0)):
        name = p.get('pattern','?')[:20]
        freq = p.get('frequency',0)
        print(f'|    {name:<20} freq={freq}' + ' '*(35-len(f'freq={freq}')) + '|')

if unresolved:
    print('+' + '-'*60 + '+')
    print('|  UNRESOLVED:' + ' '*47 + '|')
    for u in unresolved[:5]:
        dim = u.get('dimension','?')[:15]
        err = u.get('error','?')[:35]
        print(f'|    [{dim}] {err}' + ' '*max(1, 58-len(f'[{dim}] {err}')) + '|')

# Convergence bar
bar_len = 50
filled = int(rel / 100 * bar_len)
bar = '#' * filled + '.' * (bar_len - filled)
print('+' + '-'*60 + '+')
print(f'|  [{bar}] {rel:.0f}%' + ' '*max(1, 5-len(f'{rel:.0f}')) + '|')
print('+' + '='*60 + '+')

if rel >= 100:
    print('  >>> 100% CONVERGENCE ACHIEVED <<<')
elif rel >= 90:
    print('  >>> Near-perfect reliability <<<')
elif rel >= 70:
    print('  >>> Good reliability, improving <<<')
else:
    print('  >>> Needs attention <<<')
print()
" 2>/dev/null
}

# ── Main dispatch ────────────────────────────────────────────────────
case "${1:-}" in
    --record-failure)  record_failure "${2:-unknown}" "${3:-unknown error}" ;;
    --record-success)  record_success ;;
    --record-solution) record_solution "${2:-PS-???}" "${3:-}" ;;
    --add-rule)        add_rule "${2:-}" "${3:-}" ;;
    --check-rules)     check_rules ;;
    --auto-fix)        auto_fix ;;
    --report)          print_report ;;
    -h|--help)
        echo "NEXUS-6 Troubleshooting Auto-Updater"
        echo ""
        echo "  --record-failure DIM MSG    Record a growth failure"
        echo "  --record-success            Record a growth success"
        echo "  --record-solution ID SOL    Record a solution"
        echo "  --add-rule RULE REASON      Add absolute rule"
        echo "  --check-rules               Validate scripts against rules"
        echo "  --auto-fix                  Use Claude CLI to fix unresolved"
        echo "  --report                    Print convergence report"
        ;;
    *)
        print_report
        ;;
esac
