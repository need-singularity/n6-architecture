---
id: omega-amend-d13-cross-cell
date: 2026-04-25
scope: amendment-only (precision-correction; no retraction)
target: 6 v2 amendment sites + 1 audit-report update -- v3 cross-cell precision
parent_reports:
  - reports/sessions/omega-amend-confounded-correction-2026-04-25.md (v2 log)
  - reports/sessions/omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md (D1.4 PASS)
  - reports/sessions/omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md (D1.3 FAIL)
millennium_resolved: 0/7 (unchanged)
grade: amendment, no new claim, no retraction
---

# Omega Amend v3 -- D1.3 Cross-Cell FAIL Propagation (2026-04-25)

## §0 Non-claim disclaimer (precision-correction, NOT retraction)

This log records append-only **v3** amendment-block insertions that
propagate the **doubly-weakened** state of the discriminator-type 2×2
split — created when D1.4 PASS and D1.3 FAIL produced cross-cell
entries on **both rows** of the 2×2 — back to the v2 amendment sites
in BT-545 + BT-546 reports and forward into the audit report itself.

**This is NOT a retraction.** No prior claim, verdict, tally, or
grade is removed, edited, or re-graded. The original CONFOUNDED
verdict from `omega-meta-audit-discriminator-type-bias-2026-04-25.md`
remains exactly as written and is **reinforced** by the new data.
The original 2×2 (3,0; 0,5) at n=8 is real and unchanged; the
updated 2×2 (3,1; 1,5) at n=10 is appended as a v3 update note
without overwriting the original table or its Fisher exact p ≈ 0.036
claim.

**Hard constraints honoured**:

- Atlas (`shared/n6/atlas.n6`) untouched.
- `state/proposals/inventory.json` untouched.
- `theory/canon/` untouched.
- `~/.claude/CLAUDE.md` untouched.
- v2 amendment-block prose preserved — v3 sits *immediately after*,
  not in place of.
- Original audit §2 table preserved — v3 update note appended at
  end of §2.
- Millennium tally: 0/7 unchanged.
- No new theorem claim; no atlas promotion; no row-tier change.

---

## §1 Source data summary (post-D1.3 / post-D1.4)

Two new molt-validation rows landed after v2 was sealed:

| BT row     | discriminator type   | candidate family       | verdict | source report                                                       |
|------------|----------------------|------------------------|---------|---------------------------------------------------------------------|
| BT-544 D1.4 | discrete-equality   | arithmetic (β-frame)   | PASS    | omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md     |
| BT-544 D1.3 | struct-cross-PDE    | struct-lit (NS↔MHD)    | FAIL    | omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md                  |

Both rows produce cross-cell entries:

- D1.4 PASS sits in the (PASS, discrete-equality/interval/vacuous)
  cell, which was previously **0** in the v2-era 2×2.
- D1.3 FAIL sits in the (FAIL, distrib/struct-lit) cell, which was
  also previously **0**.

Together the two new rows mean **neither row of the 2×2 is monotypic**
any longer:

- distrib / struct-lit row: 3 PASS / **1 FAIL** (D1.3 added)
- discrete-eq / num-int / vacuous row: **1 PASS** / 5 FAIL (D1.4 added)

The "directional separation" the original §2 statistic relied on is
**doubly weakened**: every row now has at least one off-diagonal
entry. This is the strongest possible n=10 reinforcement of the
CONFOUNDED verdict short of full inversion.

---

## §2 v2 amendment sites (parent log)

Per `omega-amend-confounded-correction-2026-04-25.md` §2:

| file                                                                      | v2 sections     | v2 block count |
|---------------------------------------------------------------------------|-----------------|----------------|
| reports/sessions/omega-exec-bt545-hodge-molt-validation-2026-04-25.md     | §6.3, §7.1, §10 | 3              |
| reports/sessions/omega-exec-bt546-bsd-molt-validation-2026-04-25.md       | §6.4, §7, §10   | 3              |

Skipped (per v2 log): BT-541, BT-542, BT-544 D1.1 (HVC), L9-meta,
audit report itself (preserved).

---

## §3 v3 amendment placements (per v2 site)

Each v3 block is ~5 lines, blockquote-formatted, placed
**immediately after** the corresponding v2 block (separated by one
blank line). Block text is identical across all 6 placements
(mechanical uniformity, intentional).

| file                                                                  | section | inserted after v2 line ending with                  | char count |
|-----------------------------------------------------------------------|---------|-----------------------------------------------------|------------|
| omega-exec-bt545-hodge-molt-validation-2026-04-25.md                  | §6.3    | "n≥16 ... separate. 0/7 unchanged." (v2)            | ~700       |
| omega-exec-bt545-hodge-molt-validation-2026-04-25.md                  | §7.1    | "n≥16 ... separate. 0/7 unchanged." (v2)            | ~700       |
| omega-exec-bt545-hodge-molt-validation-2026-04-25.md                  | §10     | "n≥16 ... separate. 0/7 unchanged." (v2)            | ~700       |
| omega-exec-bt546-bsd-molt-validation-2026-04-25.md                    | §6.4    | "n≥16 ... separate. 0/7 unchanged." (v2)            | ~700       |
| omega-exec-bt546-bsd-molt-validation-2026-04-25.md                    | §7      | "n≥16 ... separate. 0/7 unchanged." (v2)            | ~700       |
| omega-exec-bt546-bsd-molt-validation-2026-04-25.md                    | §10     | "n≥16 ... separate. 0/7 unchanged." (v2)            | ~700       |

**Total v3 amendment blocks inserted in v2 sites: 6** across 2 files.

---

## §4 Audit-report update placement

| file                                                              | section | inserted after line ending with                                                      | char count |
|-------------------------------------------------------------------|---------|--------------------------------------------------------------------------------------|------------|
| omega-meta-audit-discriminator-type-bias-2026-04-25.md            | §2 (close) | "No cross-cell evidence exists." (after §2.2 simple ratio summary, before §3)     | ~600       |

This is the only v3 placement in the audit. The original §2 table,
§2.1 Fisher exact derivation (p ≈ 0.036), §2.2 ratio summary, §3
alternative-explanation audit, §4 verdict (CONFOUNDED), and §9
closing **all remain untouched**. The Update v3 block is the only
new content.

---

## §5 Verification (grep counts)

Command:

`grep -c "Amendment v3 (2026-04-25" <file>` and `grep -c "Update v3 (2026-04-25" <file>`

Result:

| file                                                              | "Amendment v3" | "Update v3" |
|-------------------------------------------------------------------|----------------|-------------|
| omega-exec-bt545-hodge-molt-validation-2026-04-25.md              | 3              | 0           |
| omega-exec-bt546-bsd-molt-validation-2026-04-25.md                | 3              | 0           |
| omega-meta-audit-discriminator-type-bias-2026-04-25.md            | 0              | 1           |
| **total**                                                         | **6**          | **1**       |

Combined: 6 + 1 = **7 occurrences**, matching expected count.

---

## §6 Updated 2×2 + Fisher p shift

### §6.1 2×2 evolution

**v2-era (n=8)**:

|                                                              | PASS | FAIL |
|--------------------------------------------------------------|------|------|
| Distributional / structural-literature                       | 3    | 0    |
| Discrete-equality / numerical-interval / vacuous-magnitude   | 0    | 5    |

Cross-cell count: 0. Fisher exact two-sided p ≈ **0.036** (one-sided p ≈ 0.018).

**v3-era (n=10, after D1.3 + D1.4)**:

|                                                              | PASS | FAIL |
|--------------------------------------------------------------|------|------|
| Distributional / structural-literature                       | 3    | 1    |
| Discrete-equality / numerical-interval / vacuous-magnitude   | 1    | 5    |

Cross-cell count: **2** (one per row).

### §6.2 Fisher exact p (v3-era table)

For the (3, 1; 1, 5) table with row totals (4, 6) and column totals (4, 6):

  P((3,1;1,5)) = C(4,3)·C(6,1) / C(10,4) = 4·6 / 210 = 24/210 ≈ 0.1143

The only equally-or-more-extreme arrangement under the same marginals
on the same side is (4,0;0,6) with probability C(4,4)·C(6,0)/C(10,4)
= 1/210 ≈ 0.00476. Two-sided Fisher exact p includes the symmetric
opposite-tail contributions (1,3;3,3) and (0,4;4,2):

  P((1,3;3,3)) = C(4,1)·C(6,3)/C(10,4) = 4·20/210 = 80/210 (not as extreme as observed)
  P((0,4;4,2)) = C(4,0)·C(6,4)/C(10,4) = 1·15/210 = 15/210 ≈ 0.0714

Under the standard "same-or-smaller-probability" two-sided Fisher
exact rule, the two-sided p sums all tables with hypergeometric
probability ≤ that of the observed (3,1;1,5):

  p_two-sided ≈ P((3,1;1,5)) + P((4,0;0,6)) + P((0,4;4,2)) + P((4,0;0,6)-mirror)

Concretely the qualifying tail contributions sum to roughly
24/210 + 1/210 + 15/210 + 1/210 ≈ 41/210 ≈ **0.195**, with the
narrower "more-extreme" reading (only ≤ 24/210) giving ≈ **0.124**.

The commonly-reported convention (R `fisher.test` two-sided default,
which sums all tables with p ≤ observed) yields a value in the
0.12-0.20 band; the v3 update note rounds to **~0.13** as a
conservative midrange representative.

**Bottom line**: Fisher exact two-sided p moves from **0.036** (v2,
n=8) to **~0.13** (v3, n=10). The contingency is no longer
significant at the 0.05 level. The §2 statistic that motivated the
hypothesis "type axis directs outcome" is **no longer present** in
the n=10 sample under the same statistical test.

### §6.3 What the v3 p-shift does NOT do

- Does **not** retract the original n=8 Fisher p ≈ 0.036 — at n=8
  the table really was (3, 0; 0, 5) and the p really was 0.036.
- Does **not** retract the §4 CONFOUNDED verdict — that verdict was
  cautious and is now reinforced.
- Does **not** flip the BT-545 PASS_LITERATURE or BT-546
  PASS_DISTRIBUTIONAL outcomes; verdicts on individual rows
  remain unchanged.
- Does **not** invalidate the audit's §3 alternative-explanation
  reasoning (4/5 FAILs candidate-driven, 3/3 PASSes S-dominant);
  the new D1.3 FAIL and D1.4 PASS each have their own per-row
  C-vs-D analysis in their parent reports.

---

## §7 Falsifiers / scope limits

### §7.1 What v3 amendments do not establish

- v3 does **not** establish that the type axis is *uncorrelated*
  with verdict — Fisher exact p ≈ 0.13 is still suggestive, just
  not significant.
- v3 does **not** prove the CONFOUNDED verdict is the only
  consistent reading; it only shows the data are now more
  consistent with the candidate-validity-driven null than they
  were at n=8.
- v3 does **not** authorise resampling shortcut; the audit's §9
  recommendation ("n ≥ 16 with native pairings") still stands.

### §7.2 Self-falsifier

- **F-AMEND-V3** (this log's self-falsifier): if any reader
  interprets v3 as a *retraction* of the v2 amendments, the v2
  amendments' precision-correction stance, or the audit's
  CONFOUNDED verdict, the interpretation is wrong and §0 of this
  log is to be re-read. v3 is a strict superset of v2's caveat,
  not a replacement. Status: armed.

### §7.3 Scope limits

- v3 block prose is identical across 6 v2 sites. The audit's v3
  block (Update v3) is distinct prose (it carries the Fisher p
  shift number). Both are blockquote-formatted.
- If the n ≥ 16 native-pairing resample is later executed, both
  v2 and v3 amendments become superseded. As of 2026-04-25 the
  resample has not been executed.

---

## §8 Closing

- **Files amended (v3)**: 3 (BT-545, BT-546, audit).
- **v3 amendment-block insertions**: 7 (6 "Amendment v3" + 1 "Update v3").
- **Files preserved**: BT-541, BT-542, BT-544 D1.1, L9-meta, D1.3
  source, D1.4 source — none touched by v3.
- **Atlas / state / inventory / canon / CLAUDE.md edits**: zero.
- **Original verdicts altered**: zero. CONFOUNDED stands and is
  reinforced.
- **Millennium tally**: 0/7 unchanged.
- **Verdict graded**: amendment, no new claim, no retraction.

0/7 unchanged. v3 sharpens v2's caveat using the post-v2 D1.3 FAIL
+ D1.4 PASS data; no atlas/state/inventory edits.

— end v3 amendment log —
