# CLAUDE.md — n6-architecture agent / contributor guide

> Short English-only guide for Claude Code agents and human contributors
> working inside this repository. The SSOT for governance rules lives in
> the root `.own` file; this document only restates the minimum that an
> agent needs to act safely.

## Project overview

`n6-architecture` is an AI-native arithmetic design framework centred on
the `n = 6` uniqueness theorem (sigma(n) * phi(n) = n * tau(n) iff n = 6).
The repository contains domain catalogues, theory snapshots, papers,
experiment plans, and the enforcement tooling that keeps all of this
consistent.

## Key directories

- `domains/` — per-axis domain catalogue; one `<domain>.md` body doc per
  directory, registered in each axis `_index.json` (own#3, own#16).
- `theory/` — permanent theory; timestamped snapshots belong in `reports/`
  instead (own#5).
- `papers/` — paper drafts; every paper must inline a ` ```python ` verify
  block (own#6).
- `experiments/` — experiment plans and reports; each must declare its
  MISS (failure) criterion before data collection (own#12).
- `reports/` — dated snapshot outputs and machine-written audit JSON.
- `tool/` — linters, verifiers, and auditors (see `tool/own_doc_lint.py`).
- `scripts/` — pipeline scripts, including the OUROBOROS detector
  (`scripts/monotone/ouroboros_detector_v2.py`).
- `atlas/` and `n6shared/` — canonical atlas.n6 SSOT and shared registries.

## Governance reference

- Root `.own` is the declarative SSOT (21 rules, `own#1..own#21`).
- Doc-only subset (`own#1..own#12`, `own#16`) is enforced by
  `tool/own_doc_lint.py`, which writes `reports/n6_own_doc_lint.json`.

## How to run enforcement

```bash
python3 tool/own_doc_lint.py
```

The command exits 0 on zero violations and 1 otherwise, and refreshes the
JSON report. Run it before every commit that touches markdown, `.own`,
or `_index.json`.

## Style rules for agents

- Public documents (`README.md`, `CONTRIBUTING.md`, `CLAUDE.md`, domain
  bodies, papers) must be English primary (own#1).
- Do not claim to "solve" or otherwise resolve any BT-541..BT-547 Clay
  Millennium Problem; describe progress as candidate, conditional, or
  draft (own#11).
- One body doc per domain (`<domain>.md`), optional `CLAUDE.md`; no other
  top-level markdown inside a domain directory (own#3).
- Every new domain must be registered in the parent axis `_index.json`
  (own#16).
- Theory files under `theory/` must carry permanent titles (no dated
  suffix); put timestamped snapshots in `reports/` (own#5).
- Experiments must declare MISS / failure criteria up front (own#12).

## Quick sanity commands

- Atlas integrity: `python3 tool/n6_atlas_integrity.py`
- OUROBOROS monotone: `python3 scripts/monotone/ouroboros_detector_v2.py`
- Full doc lint: `python3 tool/own_doc_lint.py`

See `CONTRIBUTING.md` for the Honesty Charter and full contribution
workflow.
