# RETIRED: `promote_v3.py`

Per raw 9 hexa-only mandate, this Python file has been retired.

## Classification

One-shot promotion runner (v3-fixpoint analyzer) — already executed; output committed.

## What it did

`atlas.n6 [10] -> [10*] promotion analyzer (dry-run, v3 fixpoint)`.

Iterated to fixpoint over `atlas/atlas.n6`'s `[10]`-grade entries, classifying
each as PROMOTE / HOLD by tier (T1 verify script, T2 expr+continuation,
T4 breakthrough, T5 definitional header-expr; H1 cross-doc dep, H2 unverified
atlas dep, H3..H5 axiom/expr-only/etc.). Emitted a JSON dry-run report with
per-reason counts.

## When it was run

Baseline `2026-04-25` (the cycle in which `state/atlas_promotion_plan_20260425.md`
closed).

## Where the output lives

- `state/atlas_promotion_20260425/promote_v3_dryrun.json` (committed dry-run JSON)
- `state/atlas_promotion_plan_20260425.md` (lists this analyzer as v3 final)
- `state/atlas_promotion_plan_20260425.json` (machine-readable plan)
- `reports/sessions/omega-audit-constraint-write-barrier-2026-04-25.md`
  (audit ledger entry)

## Why retire instead of port

- Single-cycle dry-run runner (no recurring callers).
- Output already committed to git; the dry-run JSON is the durable artifact.
- The promotion plan it fed is already closed.
- No CI / hook / `.own` decl invokes this file.
- The v3 fixpoint algorithm is already documented in
  `state/atlas_promotion_plan_20260425.md`; if a future cycle needs the same
  classification, a fresh `.hexa` analyzer can be written against the current
  atlas.

## Recovery

`git show <pre-retire-sha>:state/atlas_promotion_20260425/promote_v3.py`
recovers the verbatim source if needed for archaeology.

## Retired in

Commit: cycle 30 — raw 9 hexa-only mandate, second wave.
