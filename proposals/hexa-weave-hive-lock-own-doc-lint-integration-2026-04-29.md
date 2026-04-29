---
name: hexa-weave-hive-lock-own-doc-lint-integration
type: phased
phase: cycle-25-step-c
date: 2026-04-29
parent: hexa_weave_mvp_w5_mac_cpu_fallback_2026_04_28.md
status: forward-spec
category: phased
---

# HEXA-WEAVE — hive lock + own_doc_lint atomic integration (cycle 27+)

## §1 WHY

raw 13 (ai-tool-and-git-hook config ban) is OS-level enforced via SBPL +
LD_PRELOAD: any `.git/hooks/pre-commit`, `.githooks/`, `~/.claude/hooks/`,
or any other commit-time hook config is denied at write-time. This is a
deliberate hive-architecture decision — pre-commit hooks are bypassable
and fragment governance. Consequence: own 1 (English-only HARD) and
own 29 (README friendly toolkit HARD) cannot be enforced at git commit
time via the conventional hook path.

cycle 25 introduced two non-hook enforcement paths:

- **Step A (cycle 25)** — retroactive cleanup. 17 proposals had
  accumulated CJK during cycle 1-22 because no enforcement gate fired.
  Step A translated all 17 → 0 CJK; own 1 violation count 17 → 0.
- **Step B (cycle 25)** — daily detection. `launchd` plist
  `com.n6-architecture.own-doc-lint` runs `tool/own_doc_lint_with_alert.sh`
  every 86400s (and at load), emits raw 66 ai-native-error trailer +
  raw 77 ledger row on any HARD violation. detection lag up to 24 h.

This proposal (Step C, cycle 27+) closes the lag with **atomic
prevention** via the hive raw 99 governance dispatcher. Specifically:
when a user runs `hive lock` on n6-architecture's `.raw` / `.own` /
`.roadmap` / `.ext` SSOT files (the chflags uchg unlock-edit-relock
cycle), the relock step invokes own_doc_lint --strict before re-applying
chflags uchg. Any HARD violation aborts the relock; the SSOT remains
unlocked until the user fixes the violation and retries.

## §2 COMPARE

| Path | Latency | Bypass-resistance | Cost |
|------|---------|-------------------|------|
| pre-commit hook (raw 13 banned) | ms | low (--no-verify) | banned |
| Step B launchd daily | 24 h max | medium (off the box) | already deployed |
| Step C hive lock atomic (this) | ms (per relock) | high (hive raw 99 canonical) | cycle 27+ implementation |

raw 99 mandates `hive` as the only canonical entry for governance
mutation. Once hive lock integration lands, all SSOT changes route
through the same lint gate, regardless of the editor or human.

## §3 REQUIRES

- raw 99 production deployment of `hive lock` / `hive unlock` subcommands
- own_doc_lint --strict flag (returns non-zero on any violation; CHECKERS
  list already in `tool/own_doc_lint.py`)
- raw 66 ai-native-error trailer schema in lint output (Step B already
  emits)
- raw 77 audit ledger path `state/audit/own_doc_lint_events.jsonl`
  (Step B initialized)
- hive CLI ability to invoke a per-repo lint script (n6-architecture's
  `tool/own_doc_lint.py`) from the repo root

## §4 STRUCT

The atomic gate fits into the existing unlock-edit-relock flow as a
pre-relock check:

```
hive unlock <ssot-file>          # chflags nouchg
  user edits .raw / .own / etc.
hive lock <ssot-file>            # NEW: pre-relock lint
  ├── invoke: python3 <repo>/tool/own_doc_lint.py --strict
  ├── if exit != 0:
  │     emit raw 66 trailer
  │     append raw 77 ledger row (event=lock_blocked)
  │     ABORT — file remains unlocked
  └── if exit == 0:
        chflags uchg <file>      # original behavior
        append raw 77 ledger row (event=lock_ok)
```

The check is a single subprocess invocation; lint runtime is sub-second
on n6-architecture's current scope (~20 .md files in scope).

## §5 FLOW

User edit cycle, before vs after:

| Step | Today (cycle 25) | After Step C (cycle 27+) |
|------|------------------|--------------------------|
| 1 | unlock | unlock |
| 2 | edit | edit |
| 3 | relock (immediate) | relock with pre-lint |
| 4 | violation undetected for 24 h | violation blocks relock instantly |
| 5 | Step B daily fires | already prevented |

Detection lag drops from 24 h to ms.

## §6 EVOLVE

Three phases in sequence: Step A retroactive cleanup → Step B daily
detection → Step C atomic prevention. Each phase narrows the violation
window:

- pre-cycle 25: undetected, accumulating (17 violations had piled up)
- post Step A: zero baseline established
- post Step B: detected within 24 h
- post Step C: prevented at write time

## §7 VERIFY

Smoke tests once hive lock --pre-lint lands:

| Scenario | Expected |
|----------|----------|
| edit .raw with no CJK, then hive lock | exit 0, file relocked |
| edit .own adding CJK, then hive lock | exit 1, raw 66 trailer, file remains unlocked |
| edit README.md removing toolkit row, then hive lock | exit 1 (own 29), file remains unlocked |
| edit .raw legitimately, hive lock --skip-lint (with rationale) | exit 0 + audit ledger row event=lock_ok_skipped |

## §8 IDEAS

- extend pre-lint to other SSOT mutations: hive raw register / hive own
  register / hive roadmap edit
- auto-suggest fix for common own 1 violations (CJK → English
  candidate dictionary)
- aggregate ledger across all sister repos (raw 47 cross-repo trawl) to
  measure violation rate per repo

## §9 METRICS

- own 1 / own 29 violation rate at lock time (target: zero per
  rolling 30-day window)
- hive lock latency p50 / p99 (target: < 200 ms / < 1 s)
- false-positive rate of --strict gate (target: < 1 %, measured by
  user override count)

## §10 RISKS

| Risk | Likelihood | Mitigation |
|------|-----------|-----------|
| hive lock latency regression | medium | benchmark before / after; cap at 1 s p99 |
| false positive blocks legitimate edit | medium | --skip-lint flag with rationale logged to ledger |
| cross-repo dispatch (hive → n6-architecture) creates dependency cycle | low | hive invokes via subprocess only; no python import |
| own_doc_lint.py refactor breaks hive integration | low | API contract: exit code + JSON report path |

## §11 DEPENDENCIES

- raw 99 hive-cli-canonical-entry-point (production, cycle 27+)
- raw 13 ai-tool-and-git-hook config ban (already OS-level enforced)
- raw 66 ai-native-error-message (trailer schema, used by Step B
  wrapper)
- raw 77 execution-audit-append-only-ledger (audit row schema)
- raw 142 D2 try-and-revert (skip-lint with audit row preserves
  reversibility)
- own 1 doc-english-required (HARD)
- own 29 readme-friendly-toolkit-required (HARD)
- own 14 readme-sealed-required (re-seal after README edit; orthogonal
  but typically co-touched)

## §12 TIMELINE

- cycle 25 (today): forward-spec authored
- cycle 27+ (gated on hive raw 99 production): hive lock --pre-lint
  flag implemented in hive CLI
- cycle 28+: lint dispatch + raw 66 trailer integration
- cycle 29+: cross-repo extension (sister repos register their own
  lint scripts; hive lock invokes per repo)
- cycle 30+ (long): retire Step B launchd plist if Step C adoption
  is universal (defense in depth means keep both for now)

## §13 TOOLS

- `hive lock --pre-lint` (new flag, cycle 27+ implementation)
- `hive lock --skip-lint --reason "<text>"` (override path,
  audit-logged)
- `tool/own_doc_lint.py --strict` (existing in cycle 25, exit code
  already non-zero on violation)
- `tool/own_doc_lint_with_alert.sh` (Step B wrapper; Step C may reuse
  the trailer-emit logic without the launchd schedule)
- `state/audit/own_doc_lint_events.jsonl` (raw 77 ledger; both Step B
  and Step C append)

## §14 TEAM

Solo / AI-assisted. The hive CLI itself is owned by the user; this
proposal is a forward-spec only. raw 91 C3 honest disclosure: agent
does not modify hive repo source; cycle 27+ implementation is the
user's call.

## §15 REFERENCES

- `~/core/hive/.raw` raw 99 hive-cli-canonical-entry-point
- `~/core/hive/.raw` raw 13 ai-tool-and-git-hook config ban
- `~/core/hive/.raw` raw 66 ai-native-error-message
- `~/core/hive/.raw` raw 77 execution-audit-append-only-ledger
- `~/core/hive/.raw` raw 142 design-strategy-autonomous-loop-self
  (D2 try-and-revert)
- `~/core/n6-architecture/.own` own 1 doc-english-required
- `~/core/n6-architecture/.own` own 14 readme-sealed-required
- `~/core/n6-architecture/.own` own 29 readme-friendly-toolkit-required
- `~/core/n6-architecture/tool/own_doc_lint.py` Python lint dispatcher
- `~/core/n6-architecture/tool/own_doc_lint_with_alert.sh` Step B
  launchd wrapper
- `~/core/n6-architecture/config/launchd/com.n6-architecture.own-doc-lint.plist`
  Step B plist

## raw 71 falsifier preregistration

- F-CYCLE25-STEP-C-1: hive raw 99 production not deployed by 2026-08-29
  → Step C blocked indefinitely; mandate retired or scope reduced
  (deadline 2026-08-29)
- F-CYCLE25-STEP-C-2: hive lock --pre-lint adds latency > 1 s at p99 on
  n6-architecture .own (~36 KB) — implementation reverts to async
  Step B fallback (deadline 2026-12-29)
- F-CYCLE25-STEP-C-3: false-positive rate of --strict gate > 5 % over
  rolling 30 days (measured by --skip-lint count) — gate too strict,
  recalibrate or extend grandfather scope (deadline 2027-04-29)
- F-CYCLE25-STEP-C-4: hive integration creates cycle dependency
  (hive subprocess invocation requires n6-architecture python tool that
  itself imports hive code) — refactor to pure CLI subprocess
  contract, no python import (deadline 2027-04-29)

## raw 91 C3 honest disclosure

- forward-spec only; cycle 25 today does not modify any hive repo file
- cycle 27+ implementation timing depends on hive raw 99 maturity, which
  is the user's roadmap
- Step B launchd daily already covers the 24-h-lag detection case;
  Step C upgrades from detection to prevention
- raw 13 ai-tool-and-git-hook config ban is the architectural reason
  pre-commit hooks cannot be the path; hive raw 99 dispatcher is the
  canonical alternative
