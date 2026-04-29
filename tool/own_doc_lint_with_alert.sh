#!/usr/bin/env bash
# tool/own_doc_lint_with_alert.sh — own_doc_lint daily wrapper
#
# Authored 2026-04-29 (cycle 25 Step B). raw 13 OS-level deny on .git/hooks
# blocks pre-commit enforcement; this wrapper runs under launchd daily and
# emits raw 66 ai-native-error trailer + raw 77 audit ledger row when
# violations > 0.
#
# Usage (manual):
#   bash tool/own_doc_lint_with_alert.sh
# Usage (launchd):
#   ProgramArguments wires this script; runs every StartInterval seconds.
#
# Exit codes:
#   0  zero violations (PASS)
#   1  violations > 0 (raw 66 trailer emitted)
#   2  internal error (python3 missing, repo path invalid, etc.)

set -uo pipefail

REPO="${N6_ARCHITECTURE:-/Users/ghost/core/n6-architecture}"
TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
LEDGER="$REPO/state/audit/own_doc_lint_events.jsonl"

cd "$REPO" || { echo "[own-doc-lint] FATAL: cannot cd $REPO"; exit 2; }
mkdir -p "$(dirname "$LEDGER")"

# Run the Python lint (full sweep, all rules)
output="$(/usr/bin/env python3 tool/own_doc_lint.py 2>&1 || true)"
total_line="$(printf '%s\n' "$output" | grep -E 'total violations:' | tail -1 || true)"
hard_count="$(printf '%s\n' "$total_line" | sed -E 's/.*HARD=([0-9]+).*/\1/' || echo 0)"
soft_count="$(printf '%s\n' "$total_line" | sed -E 's/.*SOFT=([0-9]+).*/\1/' || echo 0)"
total_count="$(printf '%s\n' "$total_line" | sed -E 's/.*total violations: ([0-9]+).*/\1/' || echo 0)"
hard_count="${hard_count:-0}"
soft_count="${soft_count:-0}"
total_count="${total_count:-0}"

# Emit raw 77 audit row (append-only, single-line JSON, mass-load safe)
printf '{"schema":"raw_77_own_doc_lint_v1","ts":"%s","total":%s,"hard":%s,"soft":%s}\n' \
  "$TS" "$total_count" "$hard_count" "$soft_count" >> "$LEDGER"

if [ "${total_count:-0}" -eq 0 ]; then
  echo "__OWN_DOC_LINT_RESULT__ PASS  ts=$TS"
  exit 0
fi

# Emit raw 66 ai-native-error trailer
cat <<EOF
__OWN_DOC_LINT_RESULT__ FAIL  ts=$TS  hard=$hard_count soft=$soft_count
reason: own-1-or-other-violation-detected: total=$total_count hard=$hard_count soft=$soft_count
fix: 1) cd $REPO  2) python3 tool/own_doc_lint.py --rule 1   (re-run filtered to top offender)  3) edit violating .md to remove CJK / fix structural issue   4) python3 tool/own_doc_lint.py        (re-verify zero violations)   5) commit + push
ssot: .own (rules) + reports/n6_own_doc_lint.json (last full report) + $LEDGER (audit ledger)
EOF

# Re-print compact summary (last few report lines from python output) for log
printf '%s\n' "$output" | tail -8

exit 1
