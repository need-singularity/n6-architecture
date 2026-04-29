# RETIRED: `promote_n_hypo.py`

Per raw 9 hexa-only mandate, this Python file has been retired.

## Classification

One-shot promotion runner — already executed; output committed.

## What it did

`atlas.n6 [N?] hypothesis -> [N!] promotion analyzer (dry-run)`.

Walked `atlas/atlas.n6`, classified `[N?]`-grade entries by promotion
eligibility (verify-script presence OR breakthrough marker), and emitted a
JSON dry-run report.

## When it was run

Baseline `2026-04-25` (the cycle in which `state/atlas_promotion_plan_20260425.md`
closed).

## Where the output lives

- `state/atlas_promotion_20260425/nhypo_dryrun.json` (committed dry-run JSON)
- `state/atlas_promotion_20260425.md` (closure note)
- `state/atlas_promotion_plan_20260425.md` § "hypothesis analyzer"

## Why retire instead of port

- Single-cycle dry-run runner (no recurring callers).
- Output already committed to git; the JSON is the durable artifact.
- The promotion plan (`state/atlas_promotion_plan_20260425.md`) is closed.
- No active CI / hook / `.own` decl invokes this file.
- Re-running would require regenerating the same `nhypo_dryrun.json`
  (which has been preserved verbatim under git).

## Recovery

If a future cycle needs an N-hypothesis promotion analyzer, write a fresh
`.hexa` runner against the current atlas. Historical reproducibility is
provided by `git show <pre-retire-sha>:state/atlas_promotion_20260425/promote_n_hypo.py`.

## Retired in

Commit: cycle 30 — raw 9 hexa-only mandate, second wave.
