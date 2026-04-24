# Phase 6 Handoff — own#1 legacy allowlist final push

**Date**: 2026-04-25
**Author**: Phase 5 close-out (automated)
**Status**: handoff — Phase 6 not yet started
**Predecessor**: `reports/sessions/hard-english-only-session-2026-04-24.md` Section 14
**Trigger**: user command (no auto-schedule)

---

## 1. Scope

At Phase 5 close-out, the own#1 legacy allowlist stands at **235 entries** (parity verified: `_meta.count == len(allowlist) == len(set(allowlist)) == 235`). Phase 6 must drive this to zero.

| Bucket                                                                         | Files | Notes                                                            |
| ------------------------------------------------------------------------------ | ----: | ---------------------------------------------------------------- |
| **Phase 5 deferrals** (11) — high-risk / apex-density / infeasible-scope       |    11 | B15 (4) + B8 (2) + B12 (5); detail in Section 2                  |
| **`domains/` remaining** (Phase 3 left 217 of 417 files unaddressed)           |   217 | Long-tail `domains/**/*.md`; mostly moderate density             |
| **Stale Phase-5 entries to remove without translation work**                   |     7 | `papers/` 7 files currently in allowlist but already CJK-high — overlap with deferrals plus a few integrated/atlas-constants survivors |
| **Total**                                                                      | **235** | target: 0 by 2026 Q4                                           |

Prefix breakdown of current allowlist (235 entries):

- `domains/` 217
- `papers/` 8
- `theory/` 7
- `reports/` 3

The 11 Phase-5 deferrals distribute across `papers/` (2), `theory/` (5), `reports/` (4). The remaining `papers/` (6) and `theory/` (2) entries that are NOT on the 11-deferral list are surplus survivors (`n6-66-techniques-integrated-paper.md`, `n6-advanced-packaging-integrated-paper.md`, `n6-hexa-chip-7dan-integrated-paper.md`, `n6-hexa-consciousness-integrated-paper.md`, `n6-hexa-starship-integrated-paper.md`, `theory/constants/atlas-constants.md`, etc.). These should either be translated in a Phase 6 mini-batch or confirmed as intentional deferrals and folded into Section 2.

## 2. Per-file detail — 11 Phase-5 deferrals

### 2.1 B15 deferrals (4 files) — scale / ASCII / rep-theory

| Path                                                                             | CJK    | Lines  | Reason                                                              |
| -------------------------------------------------------------------------------- | -----: | -----: | ------------------------------------------------------------------- |
| `theory/breakthroughs/breakthrough-theorems.md`                                  | 125,694| ~28,000| Scale infeasible for single-agent; rep-theory density               |
| `reports/sessions/specs/2026-04-02-kstar-300s-steady-state-design.md`            | 11,872 | —      | ASCII art column alignment; KSTAR plasma control terminology        |
| `reports/sessions/specs/2026-04-02-kstar-n6-tokamak-design.md`                   | 15,141 | —      | ASCII art; tokamak / confinement / divertor domain vocabulary        |
| `reports/sessions/specs/2026-04-02-ultimate-fusion-powerplant-design.md`         | 15,851 | —      | ASCII art; powerplant / tritium / blanket physics vocabulary         |

### 2.2 B8 deferrals (2 files) — specialized physics / math

| Path                                                                | CJK   | Reason                                                           |
| ------------------------------------------------------------------- | ----: | ---------------------------------------------------------------- |
| `papers/moonshine-barrier-honest-report-2026-04-15.md`              | 9,319 | Dense Moonshine / VOA / Monster-group math; mis-translation risk |
| `papers/n6-ultimate-superconductor-integrated-paper.md`             | 7,359 | Dense condensed-matter physics; terminology uncertainty          |

### 2.3 B12 deferrals (5 files) — apex density (stall-resistance)

| Path                                                                    | CJK   | Lines | Reason                                |
| ----------------------------------------------------------------------- | ----: | ----: | ------------------------------------- |
| `theory/roadmap-v2/_archive-phase-01-forced-3-axes.md`                  | 6,342 | 1,234 | Apex density per stall-resistance     |
| `theory/roadmap-v2/n6arch-axes/axis-r1-emergence.md`                    | 5,258 |   906 | Apex density                          |
| `theory/roadmap-v2/n6arch-axes/axis-r2-refinement.md`                   | 5,548 |   970 | Apex density                          |
| `theory/roadmap-v2/n6arch-axes/axis-r3-finalization.md`                 | 5,656 | 1,166 | Apex density                          |
| `theory/roadmap-v2/round-03-emergence-saturation.md`                    | 4,685 | 1,001 | Apex density                          |

## 3. Recommended strategy (from B15 agent analysis)

- **`theory/breakthroughs/breakthrough-theorems.md`** — split by natural BT-100 / BT-500 / BT-1000 breakpoints; run 2-3 staged single-file agents with scope cap (one BT-range each per agent). Do not attempt as one pass.
- **KSTAR / fusion-powerplant specs (3)** — per-box ASCII re-rendering; agent must preserve column widths. Recommend a pre-pass that extracts ASCII diagrams to placeholders, translates prose, then re-injects diagrams verbatim.
- **`moonshine-barrier`, `n6-ultimate-superconductor`** — bespoke per-file agent with glossary lookups against published Moonshine / superconductor references; checkpoint after every equation block.
- **B12 apex files (5)** — single-file agents each with a scope cap (one file per agent, no parallelism within this group); apply the 11.5 + 11.6 + 11.7 discipline from the main session log plus the Phase 5 Section 14.3 additions (stage-via-index-add, atomic tmp-rename, immediate per-file commit).

## 4. `domains/` remaining (217 files)

Scope delta: Phase 3 translated 200 of 417 `domains/**/*.md` files (priority subset). The remaining 217 long-tail files are on the allowlist.

Strategy:

- Expect most to be moderate density (CJK ~1-3k range), suitable for N = 10-14 parallel-agent pattern as used in Phase 3 / Phase 4.
- **Before re-running parallel translation at scale**, address the single-file JSON contention point observed in Phase 5 Section 14.3: either shard the allowlist by prefix (`own1_legacy_allowlist.domains.json`, etc.) or introduce a per-batch lockfile protocol. Without this, N = 14+ will repeat the sibling-stash-wipeout cascade.
- Group by `domains/<subdir>/` sibling clusters for coherent terminology within a batch.

## 5. Phase 6 target

- **Allowlist 235 -> 0** by 2026 Q4 (roadmap section 5 of main session log).
- On reaching 0, delete `tool/own1_legacy_allowlist.json` and flip own#1 to unconditional HARD for the full tree.

## 6. Trigger

Phase 6 is **not auto-scheduled**. User command initiates the next run. Entry criteria:

- Main is clean (`git status` empty on tracked files).
- `python3 tool/own_doc_lint.py --rule 1` exits 0 with 0 HARD violations.
- Allowlist parity holds (`_meta.count == len == unique`).
- CI is green on the current tip.

At Phase 5 close-out (2026-04-25, `origin/main` at `77e10b9f` plus the session-log commit), all four entry criteria are satisfied.

## 7. Safety notes

- Never use `git stash -u` during an active parallel window. Use stage-via-index-add (`git add`) and commit-or-discard, nothing in between.
- Never run external regex/dictionary mass-translate pipelines against `.md` files in the working tree during a parallel window (see session log Section 11.7 item `g`).
- Never force-push main. Never `--no-verify` unless a hook hangs.
- When a file exceeds the agent's translation-quality threshold, defer explicitly (leave on allowlist, document here) rather than ship a degraded translation.
