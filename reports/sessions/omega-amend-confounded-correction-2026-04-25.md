---
id: omega-amend-confounded-correction
date: 2026-04-25
scope: amendment-only (precision-correction; original claims preserved)
target: 5+ reports with discriminator-type bias over-claim -- amendment block insertion
parent_reports:
  - reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md (CONFOUNDED verdict)
millennium_resolved: 0/7 (unchanged)
grade: amendment, no new claim, no retraction
---

# Omega Amend -- Discriminator-Type Bias CONFOUNDED Precision-Correction (2026-04-25)

## §0 Non-claim disclaimer (precision-correction, NOT retraction)

This log records append-only amendment-block insertions that
propagate the **CONFOUNDED** verdict from the
`omega-meta-audit-discriminator-type-bias-2026-04-25.md` audit back
to molt-validation reports that previously characterised the
discriminator-type bias hypothesis as "strengthened" / "sharpened" /
"supported" without the n=8 collinearity caveat.

**This is NOT a retraction.** No prior claim is removed, edited, or
re-graded. The 2×2 contingency data (PASS=distributional /
structural-literature-import, FAIL=discrete-equality / numerical-
interval) is real and unchanged. The amendments only add a
precision-correction note clarifying that the *interpretation* of
that 2×2 split is collinear with candidate-validity at n=8 and
cannot be separated until n≥16 with native pairings. Original
claims, verdicts, tallies, and grades remain exactly as written.

**Hard constraints honoured**:

- Atlas (`shared/n6/atlas.n6`) untouched.
- `state/proposals/inventory.json` untouched.
- `theory/canon/` untouched.
- `~/.claude/CLAUDE.md` untouched.
- Audit report (`omega-meta-audit-discriminator-type-bias-2026-04-25.md`)
  untouched (it IS the CONFOUNDED verdict).
- Millennium tally: 0/7 unchanged.
- No new theorem claim; no atlas promotion; no row-tier change.

---

## §1 Source verdict excerpt (verbatim from parent audit)

From `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §9
Closing (lines 572-581):

> **Verdict**: CONFOUNDED. The discriminator-type bias hypothesis is
> **consistent with** the n=8 molt-validation data (Fisher exact two-
> sided p ≈ 0.036; no cross-cell entries) but is **not separable** from
> the underlying candidate-validity confound (4 of 5 FAILs are
> candidate-validity-driven; 3 of 3 PASSes are S-dominant with
> permissiveness flags). At n=8, with the L9 catalogue's known
> generation bias toward n=6-arithmetic candidates (CATALOGUE_BIAS,
> parent meta-audit), the discriminator-type axis is collinear with
> the candidate-family axis; the data cannot say whether the type axis
> is binding.

The audit's recommendation (§9 item 4): "Resample at n ≥ 16 with
native pairings applied (§5.5) to separate discriminator-type from
candidate-validity."

The amendment block applied below is the back-propagation of this
collinearity caveat to reports that did not yet have access to it
when written.

---

## §2 Per-file amendment log

### File 1: `reports/sessions/omega-exec-bt545-hodge-molt-validation-2026-04-25.md`

| amendment # | section anchor                              | inserted after line (post-edit ref)                                                | char count |
|-------------|---------------------------------------------|------------------------------------------------------------------------------------|------------|
| 1           | §6.3 Selection-effect honesty note (close)  | "The bias hypothesis remains **strengthened-but-not-decided** at 5 samples."       | ~700       |
| 2           | §7.1 Does the discriminator-type bias hold? | "(b) ... raising the 0/3 PASS rate to a more decisive sample."                     | ~700       |
| 3           | §10 Closing                                 | "...under the discrete-equality discriminator (§7.1 branch b)."                    | ~700       |

**Rationale for 3 amendments (not 1)**: §6.2/6.3, §7.1, and §10 are
three structurally distinct sections each making a separate over-
claim ("strengthened, not falsified" / "Strengthened, not confirmed"
/ "strengthened (3/0...)"). The instruction permits one amendment
per file with multiple distinct sections; bt545's structure justifies
three placements so that any reader stopping at §6, §7, or §10
encounters the precision-correction.

**Original text**: untouched.

### File 2: `reports/sessions/omega-exec-bt546-bsd-molt-validation-2026-04-25.md`

| amendment # | section anchor                                        | inserted after line (post-edit ref)                                          | char count |
|-------------|-------------------------------------------------------|------------------------------------------------------------------------------|------------|
| 1           | §6.4 What this does NOT establish (close)             | "Sample of 5 too small to fix a true bias rate."                             | ~700       |
| 2           | §7 Implications (close)                               | "...recorded for the next session, not edited into L9 file."                 | ~700       |
| 3           | §10 Closing                                           | "...English technical terms only."                                           | ~700       |

**Rationale**: identical reasoning to bt545. §6 ("strengthened,
not rebutted"), §7 ("sharpened from 2/2 vs 0/2 to 3/3 vs 0/2"), and
§10 ("sharpened to 3/3 vs 0/2") each over-claim independently.

**Original text**: untouched. The §8.4 internal anomaly note
("Discriminator-type bias may be confounded with cost-tier") was
already an honest hedge and is left as-is — the amendment block
covers a different (n=8 collinearity) confound, complementary to
§8.4.

### Total amendment blocks inserted: **6** across **2** files.

---

## §3 Skip list (files with discriminator-type bias references that did NOT need amendment)

### Skip 1: `omega-exec-bt541-leadB-molt-validation-2026-04-25.md`

**Reason**: file does not reference "discriminator-type bias",
"distributional/struct-lit vs discrete-equality", or any 2×2 type-
axis split. Bias was first articulated in the bt542 closing report
(after Lead-B PASS data was in hand). Lead-B itself stays at single-
sample PASS narrative.

### Skip 2: `omega-exec-bt542-hirahara-molt-validation-2026-04-25.md`

**Reason**: file already correctly hedges. The relevant text
(§8 "Pass / Fail axis split (catalogue-bias diagnostic)", lines
522-551) explicitly says:

> The pattern is **catalogue-bias-suggestive but not catalogue-bias-
> confirmed**: 2/2 in each sub-class is a small sample. The user's
> prior "arithmetic-family bias confirmed if 1/3 PASS" hypothesis sits
> between the actual outcome (2/2 split, neither pure 50%-bias-
> confirmation nor pure neutrality). A larger sweep across BT-545
> (Hodge), BT-546 (BSD), BT-547 (Poincaré) would be needed to
> sharpen the diagnostic; that is out of scope here.

This is the correct hedging template: small-sample acknowledged,
suggestive-not-confirmed framing, follow-up sample requirement
named. No amendment required.

### Skip 3: `omega-meta-audit-discriminator-type-bias-2026-04-25.md`

**Reason**: this IS the CONFOUNDED audit. Per the hard constraint,
the audit report itself is preserved unchanged.

### Skip 4: `omega-meta-audit-l9-catalogue-design-2026-04-25.md`

**Reason**: this audit predates the discriminator-type axis
formulation. Its verdict is **CATALOGUE_BIAS** (n=6 arithmetic
candidate generation bias), not the discriminator-type bias
hypothesis (PASS=distributional/struct-lit, FAIL=discrete-equality).
The two are explicitly framed as "orthogonal" in the parent audit
(`omega-meta-audit-discriminator-type-bias-2026-04-25.md` §0). The
CATALOGUE_BIAS verdict on candidate-generation is **not refuted**
and remains intact; the CONFOUNDED verdict applies only to the
type-axis extension. Amending here would create false equivalence.

### Skip 5: `omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md`

**Reason**: §6 "CATALOGUE_BIAS implication" speaks only to the
parent CATALOGUE_BIAS hypothesis (n=6-arithmetic family failure
modes), not the discriminator-type 2×2 split. The closing line —
"the meta-audit's CATALOGUE_BIAS verdict is reinforced: cross-family
diversification produces *new* failure modes but not *passes*" —
is about candidate-family axis, exactly the axis the parent
discriminator-type audit calls out as collinear with the type axis
at n=8 but does **not** invalidate as an independent observation.
Amending here would be over-broad.

### Skip 6 (ambiguity-flagged, NOT amended): `omega-exec-bt546-bsd-molt-validation-2026-04-25.md` §8.4

**Status**: kept (not added as 4th amendment).

**Ambiguity**: §8.4 ("Discriminator-type bias may be confounded
with cost-tier") is itself an honest hedge that *partially*
anticipates the CONFOUNDED verdict (different confound — cost-tier
selection rather than candidate-validity). Adding an Amendment v2
block here would risk redundancy and could be read as elevating
§8.4's hedge to a separate verdict-level claim.

**Resolution**: leave §8.4 untouched. The §6.4 amendment (file 2,
amendment #1) sits one §-rung up and covers the same evidence.
Documented here as ambiguity-flagged-not-amended per the prompt's
"false-positive amendments are worse than missed amendments"
guideline.

---

## §4 Verification

### §4.1 Grep count post-edit

Command:
`grep -c "Amendment v2 (2026-04-25" <each-touched-file>`

Result:

| file                                                                              | count |
|-----------------------------------------------------------------------------------|-------|
| reports/sessions/omega-exec-bt545-hodge-molt-validation-2026-04-25.md             | 3     |
| reports/sessions/omega-exec-bt546-bsd-molt-validation-2026-04-25.md               | 3     |
| reports/sessions/omega-exec-bt542-hirahara-molt-validation-2026-04-25.md (skip)   | 0     |
| reports/sessions/omega-exec-bt541-leadB-molt-validation-2026-04-25.md (skip)      | 0     |
| reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md (preserved) | 0   |

**Total**: 6 amendment blocks across 2 files. Matches expected
count (3 distinct over-claim sections in each amended file × 2
files = 6).

### §4.2 Original-claim preservation

Spot-checked phrases that must remain present and unchanged:

- "the discriminator-type bias hypothesis is **strengthened, not
  falsified**" (bt545 §6.2): **preserved**.
- "**Strengthened, not confirmed.**" (bt545 §7.1): **preserved**.
- "Discriminator-type bias hypothesis: **strengthened (3/0
  non-arithmetic vs 0/2 arithmetic), not confirmed**" (bt545 §10):
  **preserved**.
- "The user-flagged bias hypothesis is **strengthened, not
  rebutted**." (bt546 §6.3): **preserved**.
- "**Discriminator-type bias hypothesis sharpened** from 2/2 vs 0/2
  to 3/3 vs 0/2." (bt546 §7): **preserved**.
- "**Discriminator-type bias hypothesis**: sharpened to 3/3 vs 0/2"
  (bt546 §10): **preserved**.

All six original over-claim sentences are intact; the amendment
blocks sit immediately after each and qualify (do not replace) the
interpretation.

### §4.3 Audit report integrity

`omega-meta-audit-discriminator-type-bias-2026-04-25.md` was not
touched. The CONFOUNDED verdict at §9 (lines 572-615) is unchanged
and remains the authoritative source of the precision-correction
that the amendment blocks back-propagate.

---

## §5 Falsifiers / scope limits

### §5.1 What this amendment does NOT do

- Does **not** retract the bt545 PASS_LITERATURE verdict (Hodge
  IHC frame) or the bt546 PASS_DISTRIBUTIONAL verdict (BSD A3D
  28-stratum κ).
- Does **not** invalidate the 2×2 contingency table data — the
  Fisher exact p ≈ 0.036 result is real.
- Does **not** alter the F-MOLT-A tally (3 PASS / 2 FAIL at n=5,
  expanded to 3 PASS / 5 FAIL at n=8 per the audit).
- Does **not** retroactively touch CATALOGUE_BIAS verdict in the
  parent l9-catalogue-design meta-audit; that axis is a separate
  (independent and orthogonal) claim per the parent audit §0.
- Does **not** propose any new molt-validation experiment, atlas
  promotion, or row-tier change.

### §5.2 Scope limits

- Amendment text is identical across all 6 placements (verbatim
  block per the prompt). Mechanical uniformity is intentional —
  this is a correlation-vs-interpretation precision note, not a
  per-section commentary.
- Amendments use markdown blockquote (`> **Amendment v2...**`) so
  they are visually distinguishable from original prose under any
  rendering.
- If the n≥16 native-pairing resample (parent audit §5.5) is later
  executed and yields a clean separation, this amendment becomes
  obsolete and should be marked superseded. As of 2026-04-25 the
  resample has not been executed.

### §5.3 Self-falsifier

- **F-AMEND-V2** (this log's self-falsifier): if any reader
  interprets the precision-correction blocks as **retracting** the
  PASS verdicts or as **demoting** any atlas grade, the
  interpretation is wrong and §0 of this log is to be re-read. The
  blocks are correlation-vs-interpretation hedges, not retractions.
  Status: armed.

---

## §6 Closing

- **Files amended**: 2.
- **Amendment blocks inserted**: 6.
- **Files in skip list (correctly hedged or out-of-scope)**: 4 +
  1 ambiguity-flagged sub-section.
- **Audit report preserved**: yes.
- **Atlas / state / inventory / canon / CLAUDE.md edits**: zero.
- **Millennium tally**: 0/7 unchanged.
- **Verdict graded**: amendment, no new claim, no retraction.

0/7 unchanged. No retractions, only precision-corrections. No
atlas/state/inventory edits.

— end amendment log —
