# RETIRED: `h_meta_analysis_20260415.py`

Per raw 9 hexa-only mandate, this Python file has been retired.

## Classification

One-shot meta-analysis runner — already executed; output committed.

## What it did

Group-P meta-analysis (H1~H5) over `atlas.signals.n6` + signal staging:

- H1 — signal ROI by grade (length / complexity / witness / resonance / cross-repo).
- H2 — PageRank over (cross_repo + predicts) edges; top-10 hub signals.
- H3 — next-session signal-count prediction (last-7 avg + weighted-avg
  + 14-day linear-regression slope).
- H4 — depletion-date forecast (zero-rate extrapolation from LR slope;
  half-depletion sub-forecast).
- H5 — temporal-causal graph (cross_repo / predicts edges with strict
  time ordering); top hubs in/out.

## When it was run

`2026-04-15` (Group P session — banner `Group P -- H1~H5`).

## Where the output lives

- `reports/group-P/h_meta_analysis_result.json` — full JSON result (committed).

## Why retire instead of port

- Single-session meta-analysis (Group P, 2026-04-15); output already
  committed verbatim to git.
- Hard-coded paths point to `/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6`
  and `/Users/ghost/Dev/nexus/shared/n6/staging/...`, which are *external*
  to this repo (`Dev/nexus/...`, not `core/n6-architecture/`). The script
  is therefore not portable to a fresh checkout in any case.
- No `.own` decl, CI hook, or downstream tool invokes this file.
- Algorithms are pure-stdlib (defaultdict / Counter / regex / iterative
  PageRank / linear regression / cumulative-rate decay): a future port
  would be a `.hexa` frontend plus `python3 -c` for any irreducible
  numerical step (none required here — all steps are integer / float
  arithmetic, no scipy.stats).
- Retire avoids the porting cost for a one-shot analysis whose JSON
  output is the durable artifact.

## Recovery

`git show <pre-retire-sha>:reports/group-P/h_meta_analysis_20260415.py`
recovers the verbatim source. The result JSON
(`h_meta_analysis_result.json`) is the canonical reference for all H1~H5
findings cited in subsequent group-P notes.

## Retired in

Commit: cycle 30 — raw 9 hexa-only mandate, second wave.
