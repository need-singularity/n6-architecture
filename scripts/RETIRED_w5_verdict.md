# RETIRED — `scripts/w5_verdict.py`

**Retire date**: 2026-04-29 (cycle 30 wave 2)
**Per**: raw 9 hexa-only mandate + raw 102 ADD-new path (one-shot
verdict generator, external callers = 0, output committed)

## Original purpose

HEXA-WEAVE MVP W5 deterministic verdict generator for 6L1U dry-run
inference. Emitted a raw 77 schema-versioned audit ledger row to
`state/audit/w5_verdict_events.jsonl` after running deterministic
checks against the W5 inference output.

## Why retired

- **External callers = 0**: full repo grep for `w5_verdict` returns only
  the file itself + the audit ledger reference inside its own docstring.
- **One-shot semantic**: the script ran during HEXA-WEAVE MVP W5 cycle
  (commit `5fd60d4d` "feat(hexa-weave): cycle 15 5-agent fan-out … W5
  scripts"); the resulting verdict row landed in
  `state/audit/w5_verdict_events.jsonl` and is git-tracked.
- **Output committed**: the verdict ledger is preserved
  (`state/audit/w5_verdict_events.jsonl`).
- **No CI / .own / workflow reference**: zero downstream automation.

## Recovery

```bash
git show 5fd60d4d:scripts/w5_verdict.py > scripts/w5_verdict.py
```

The original script remains accessible via git history. If a future
HEXA-WEAVE wave (W6+) needs an analogous verdict generator, port the
deterministic check chain to hexa-native (raw 9 / raw 53
deterministic-verifier-manifest); reference impl pattern at
`tool/zenodo/verify_paper_block.hexa`.

## Cross-reference

- raw 9 hexa-only mandate (n6-architecture /.raw)
- raw 53 deterministic-verifier-manifest
- raw 91 honest C3 disclosure
- raw 102 ADD-new path (RETIRE option)
- HEXA-WEAVE MVP W5 cycle (commit `5fd60d4d`)
- audit ledger preserved: `state/audit/w5_verdict_events.jsonl`
