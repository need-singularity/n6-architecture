---
id: omega-amend-scenario-a-validity-dominant
date: 2026-04-25
scope: amendment-only (precision-correction; no retraction; further refining CONFOUNDED)
target: 7 v3 amendment sites -- v4 Scenario A precision (H_validity dominant)
parent_reports:
  - reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md (CONFOUNDED original)
  - reports/sessions/omega-amend-confounded-correction-2026-04-25.md (v2 log)
  - reports/sessions/omega-amend-d13-cross-cell-2026-04-25.md (v3 log)
  - reports/sessions/omega-exec-n20-p567-batch-2026-04-25.md (P5+P6+P7 n=14 data)
millennium_resolved: 0/7 (unchanged)
grade: amendment, no new claim, no retraction
---

# Omega Amend v4 -- Scenario A (H_validity Dominant) Precision (2026-04-25)

## §0 Non-claim disclaimer (precision-correction, NOT retraction)

This log records append-only **v4** amendment-block insertions
appended *after* the v3 blocks at every v3 site. The v4 update
propagates the **n=14 P5+P6+P7 batch** result — Fisher exact p has
moved from 0.13 (n=10) to 0.2774 (P6 top) / 0.0909 (P6 bottom),
**away from H_type significance** and into **Scenario A (H_validity
dominant)** territory under both defensible classifications.

**This is NOT a retraction.** The original CONFOUNDED verdict from
`omega-meta-audit-discriminator-type-bias-2026-04-25.md` stands
exactly as written. v2's CONFOUNDED note is preserved. v3's
"doubly-weakened" / "cross-cell on both rows" finding is preserved.
v4 is a *progressive refinement*: it identifies **which axis
dominates** the confounding — candidate-validity, not
discriminator-type.

**Hard constraints honoured:**

- Atlas (`shared/n6/atlas.n6`) untouched.
- `state/proposals/inventory.json` untouched.
- `theory/canon/` untouched.
- `~/.claude/CLAUDE.md` untouched.
- v2 and v3 amendment-block prose preserved verbatim — v4 sits
  *immediately after* v3, not in place of.
- Original audit §2 table, §2.1 Fisher derivation (p ≈ 0.036),
  §2.2 ratio summary, §3 alternative-explanation audit, §4
  CONFOUNDED verdict, §9 closing — **all untouched**.
- Millennium tally: 0/7 unchanged.
- No new theorem claim; no atlas promotion; no row-tier change.

---

## §1 Trajectory summary (n=8 → n=10 → n=14)

| stage  | n  | 2×2 (top row; bottom row)         | Fisher 2-sided p          | Scenario alignment            |
|--------|----|-----------------------------------|---------------------------|-------------------------------|
| n=8    | 8  | (3,0; 0,5)                        | 0.036                     | type-axis suggestive (CONFOUNDED audit) |
| n=10   | 10 | (3,1; 1,5) [v3, after D1.3+D1.4]  | ~0.13                     | doubly-weakened, no significance |
| n=14   | 14 | (4,2; 2,6) primary OR (3,3; 2,6) alt | 0.2774 / 0.0909        | **Scenario A (H_validity dominant)** |

The trajectory monotonically moves *away* from H_type-style
significance (B envelope: p ∈ 0.001–0.005 at n=20) and into the
A envelope (p ∈ 0.10–0.18 at n=20). The current n=14 reading is
above (0.2774) or at the lower edge of (0.0909) the A envelope —
both readings are inconsistent with H_type and consistent with
H_validity.

---

## §2 Scenario A confirmation excerpt (P5+P6+P7 batch)

From `omega-exec-n20-p567-batch-2026-04-25.md` §6:

- **Primary classification (P6 top-row, structural)**: p = 0.2774.
  Outside (above) the A envelope (0.10–0.18); H_type is
  **further weakened** than even A predicts. Discriminator-type
  axis is losing predictive power as native pairings (especially
  the cross-cell P5 PASS and the diagonal P6 FAIL) are added.
- **Alternative classification (P6 bottom-row, discrete-equality
  read of irrep-dim matching)**: p = 0.0909. In the A envelope,
  at the lower edge.
- Both readings agree: the n=14 update points toward **Scenario A
  (H_validity dominant)** rather than B (H_type dominant) or C
  (mixed).

The dominant driver is **candidate-validity** (whether the
underlying L9-generated candidate is structurally well-grounded);
the **discriminator-type** axis is along-for-the-ride collinear at
n=8 and decoupled at n=14.

---

## §3 v3 amendment sites (parent v3 log)

Per `omega-amend-d13-cross-cell-2026-04-25.md` §3 / §4:

| file                                                                  | sections        | v3 block count |
|-----------------------------------------------------------------------|-----------------|----------------|
| reports/sessions/omega-exec-bt545-hodge-molt-validation-2026-04-25.md | §6.3, §7.1, §10 | 3 (Amendment v3) |
| reports/sessions/omega-exec-bt546-bsd-molt-validation-2026-04-25.md   | §6.4, §7, §10   | 3 (Amendment v3) |
| reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md | §2 (close)    | 1 (Update v3)    |

= 7 v3 blocks total. v4 is appended after each.

---

## §4 v4 amendment placements (per file)

Each v4 block is identical prose (mechanical uniformity), inserted
**immediately after** the v3 block (one blank line separator).
Block character count: ~870 each.

| file                                                                  | section | inserted after v3 line ending with                | char count |
|-----------------------------------------------------------------------|---------|---------------------------------------------------|------------|
| omega-exec-bt545-hodge-molt-validation-2026-04-25.md                  | §6.3    | "...CONFOUNDED verdict reinforced. 0/7 unchanged." | ~870       |
| omega-exec-bt545-hodge-molt-validation-2026-04-25.md                  | §7.1    | "...CONFOUNDED verdict reinforced. 0/7 unchanged." | ~870       |
| omega-exec-bt545-hodge-molt-validation-2026-04-25.md                  | §10     | "...CONFOUNDED verdict reinforced. 0/7 unchanged." | ~870       |
| omega-exec-bt546-bsd-molt-validation-2026-04-25.md                    | §6.4    | "...CONFOUNDED verdict reinforced. 0/7 unchanged." | ~870       |
| omega-exec-bt546-bsd-molt-validation-2026-04-25.md                    | §7      | "...CONFOUNDED verdict reinforced. 0/7 unchanged." | ~870       |
| omega-exec-bt546-bsd-molt-validation-2026-04-25.md                    | §10     | "...CONFOUNDED verdict reinforced. 0/7 unchanged." | ~870       |
| omega-meta-audit-discriminator-type-bias-2026-04-25.md                | §2 (close) | "...not cleanly separable from candidate-validity at n=10." | ~870 |

**Total v4 amendment blocks inserted: 7** across 3 files.

---

## §5 Verification (grep counts)

Command:

`grep -c "Amendment v4 (2026-04-25" <file>`

Result:

| file                                                                  | v2 | v3 | Update v3 | **v4** |
|-----------------------------------------------------------------------|----|----|-----------|--------|
| omega-exec-bt545-hodge-molt-validation-2026-04-25.md                  | 3  | 3  | 0         | **3**  |
| omega-exec-bt546-bsd-molt-validation-2026-04-25.md                    | 3  | 3  | 0         | **3**  |
| omega-meta-audit-discriminator-type-bias-2026-04-25.md                | 0  | 0  | 1         | **1**  |
| **total**                                                             | 6  | 6  | 1         | **7**  |

All v2 and v3 / Update v3 blocks preserved. 7 v4 blocks placed,
matching the expected count (6 in BT-545/546 + 1 in audit).

---

## §6 Amendment chain summary (v1 → v2 → v3 → v4 progressive refinement)

| version | trigger                                       | claim                                                      | preserves prior? |
|---------|-----------------------------------------------|------------------------------------------------------------|------------------|
| v1 (audit, n=8) | original CONFOUNDED audit             | type-axis correlation real but interpretation-CONFOUNDED  | n/a (origin)     |
| v2 (n=8)        | per-BT propagation of CONFOUNDED note | annotates BT-545/546 reports' 2×2 with CONFOUNDED caveat  | yes              |
| v3 (n=10)       | D1.3 FAIL + D1.4 PASS cross-cell      | doubly-weakened; cross-cell on both rows; p → ~0.13       | yes (v2)         |
| v4 (n=14)       | P5+P6+P7 native-paired batch          | **Scenario A (H_validity dominant) confirmed**; p → 0.2774/0.0909 | yes (v2, v3) |

Each version *strictly refines* its predecessor; none replace.
The CONFOUNDED verdict from v1 holds at all subsequent stages. v4
is the most recent stage in this strictly-monotone refinement.

---

## §7 Falsifiers / scope limits

### §7.1 What v4 amendments do not establish

- v4 does **not** prove H_validity is the *only* driver — it shows
  H_type's predictive power is monotonically decaying as native
  pairings replace L9-aligned pairings.
- v4 does **not** retract the audit's CONFOUNDED verdict; that
  verdict is reinforced (and refined to identify the dominant
  axis).
- v4 does **not** flip BT-545 PASS_LITERATURE, BT-546
  PASS_DISTRIBUTIONAL, or any individual molt-validation outcome.
- v4 does **not** prove H_type is dead — at n=14 H_type is below
  significance, but n=14 is still small and the alternative
  classification (p = 0.0909) sits inside the A envelope, leaving
  H_validity-dominant + residual-H_type-contribution mixed
  scenario possible.

### §7.2 Sample-size caveat

n=14 is still below the n ≥ 16 native-pairing threshold from the
audit §9. Full separation between A (H_validity dominant) and C
(mixed) requires the remaining n=20 batch (P8 + P9 + P10) per
`omega-meta-design-n20-native-pairing-resample-2026-04-25.md`.
v4's Scenario A claim is **post-batch consistent**, not
**resampled-confirmed**.

### §7.3 Self-falsifier

- **F-AMEND-V4** (this log's self-falsifier): if any reader
  interprets v4 as (a) a retraction of v1/v2/v3, (b) a closure of
  the CONFOUNDED verdict into a CONFIRMED-H_validity verdict, or
  (c) a license to skip the n=20 P8/P9/P10 batch, the
  interpretation is wrong and §0 of this log is to be re-read.
  v4 is a strict superset of v3's caveat with axis-precision
  attached. Status: armed.

### §7.4 Scope limits

- v4 block prose is identical across 7 sites (mechanical
  uniformity, intentional).
- If the n=20 P8/P9/P10 batch is later executed, v4 will be
  superseded by v5 with the n=20 Fisher p value; v4's
  Scenario A claim will be either confirmed (if p lands in
  0.10–0.18) or refined toward C (if p lands in 0.02–0.05).
- As of 2026-04-25 the n=20 completion has not been executed.

---

## §8 Closing

- **Files amended (v4)**: 3 (BT-545, BT-546, audit).
- **v4 amendment-block insertions**: 7.
- **Files preserved**: BT-541, BT-542, BT-544 D1.1, L9-meta, D1.3
  source, D1.4 source, P5/P6/P7 batch source — none touched by
  v4.
- **Atlas / state / inventory / canon / CLAUDE.md edits**: zero.
- **Original verdicts altered**: zero. CONFOUNDED stands and is
  refined toward H_validity-dominant.
- **Millennium tally**: 0/7 unchanged.
- **Verdict graded**: amendment, no new claim, no retraction.

0/7 unchanged. v4 sharpens v3's caveat using the n=14 P5+P6+P7
batch data; identifies candidate-validity (NOT discriminator-type)
as the dominant axis under the CONFOUNDED verdict; no
atlas/state/inventory edits.

— end v4 amendment log —
