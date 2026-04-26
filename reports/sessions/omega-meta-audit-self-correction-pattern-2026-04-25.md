---
id: omega-meta-audit-self-correction-pattern
date: 2026-04-25
scope: research-only meta-audit on this session's iteration pattern
target: 4-step self-correction sequence -- engine vs artifact vs observer-induced
parent_reports:
  - reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md (Step 3)
  - reports/sessions/omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md (Step 4 PASS)
  - reports/sessions/omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md (Step 4 FAIL)
  - reports/sessions/omega-amend-confounded-correction-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: meta-audit on session methodology, no claim
---

# Omega Meta-Audit -- Self-Correction Pattern (2026-04-25)

## §0 Non-claim disclaimer

This report audits the **iteration pattern** of the 2026-04-25
n6-architecture session, in which four successive interpretation steps
each weakened or refined the prior. The audit is methodological and
read-only.

It does **NOT**:

- claim 3D NS regularity, RH, P=NP, or any Millennium resolution;
- modify `shared/n6/atlas.n6`, `state/proposals/inventory.json`,
  `theory/canon/`, or any L9 / D1 / D2 / D3 source document;
- promote, demote, or otherwise re-grade atlas entries;
- supersede, retract, or amend any of the parent reports' verdicts;
- claim the iteration pattern is itself "the framework" -- the audit
  selects from {H_ENGINE, H_ARTIFACT, H_OBSERVER, MIXED} only;
- introduce a 5th step or a new self-correction. It is a single audit
  pass and explicitly tries not to instantiate the very pattern it
  studies.

Millennium tally: **0/7 unchanged**.

The audit itself is subject to the no-fabrication guard: every
quoted claim is sourced; every step's time-ordering is from
filesystem mtime; every numerical figure (n=4, n=8, n=9, n=10,
Fisher p) is from the parent reports verbatim. If the audit cannot
distinguish the three hypotheses at the present sample size, the
honest closest-to-honest verdict from the listed strings is MIXED
(H_INDETERMINATE is not in the listed verdicts).

---

## §1 The 4-step sequence reconstructed

### §1.1 Time order (filesystem mtime, 2026-04-25)

| step | report                                                                               | mtime    | role                |
|------|--------------------------------------------------------------------------------------|----------|---------------------|
| 1    | `omega-exec-bt544-fallback-molt-validation-2026-04-25.md`                            | 20:20:54 | 4/4 FAIL anchor     |
| 2a   | `omega-exec-bt541-leadB-molt-validation-2026-04-25.md`                               | 20:34:23 | 1st PASS            |
| 2b   | `omega-exec-bt542-hirahara-molt-validation-2026-04-25.md`                            | 20:49:38 | 2nd PASS, 2x2 emerges |
| 3    | `omega-meta-audit-discriminator-type-bias-2026-04-25.md`                             | 21:11:32 | CONFOUNDED          |
| 3b   | `omega-amend-confounded-correction-2026-04-25.md`                                    | 21:19:00 | back-propagation    |
| 4a   | `omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md`                    | 21:21:54 | cross-cell PASS     |
| 4b   | `omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md`                                 | 21:27:22 | cross-cell FAIL     |

The full arc (Step 1 → Step 4b) spans **66 minutes** of wall-clock,
with each transition averaging **~11 minutes** apart. The sequence
is dense; the iteration tempo is not "review next day" but "rerun
within the hour".

### §1.2 The four steps as claim / evidence / refinement

| step | claim (verbatim or near-verbatim from source)                                                                                                                                                                                                                                | evidence at the time                                                                                                                                                                                                                                                       | refinement / refutation that came next                                                                                                                                                       | refinement source                                                                                  |
|------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| 1    | "all three rank-1/2/3 BT-544 candidates failing ... covering three distinct primitive types (algebraic-lattice, mechanism-Sobolev, scaling-KPZ), and all three failed" (fallback report §10 anomalies, line 622-628). Implicit interpretation in main thread: 100% rejection rate is positive evidence that discriminators have empirical bite. | Q1 FAIL (rank=6 vs target 3), Q5 FAIL (no construction), KPZ d=7 FAIL (no anchor). All within BT-544. n=4 BT-validation samples (3 of which are intra-BT) plus BT-543 P3 FAIL.                                                                                              | BT-541 Lead-B PASS (KS p<0.01, double-margin, line 187-194) breaks the 3/3 FAIL streak. F-MOLT-A "NOT FIRED. The 3-of-3 fail condition is broken by the BT-541 Lead-B PASS." (Lead-B §6, L246) | `omega-exec-bt541-leadB-molt-validation-2026-04-25.md` §6                                          |
| 2    | "the L9 catalogue's arithmetic-family candidates ... underperform; non-arithmetic-family candidates (those resting on probability distributions or literature-import primitives) overperform" (Hirahara report §8, line 535-543). 2x2 table: PASS=distrib/struct-lit, FAIL=discrete-equality. | At n=8 molt-validations: 3 PASS (Lead-B distrib, Hirahara struct-lit, D3.A struct-lit), 5 FAIL (Q1, Q5, KPZ d=7, D1.1, P3). Cells (3,0;0,5). Fisher exact two-sided p ≈ 0.036. | The discriminator-type meta-audit (Step 3) finds the type axis is **collinear** with candidate-validity at n=8: "4 of 5 FAILs are dominantly C [candidate-validity-driven]" (audit §3.1, L177-180); "All 3 PASSes are S-dominant" (audit §3.2, L196-201). | `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §3.1, §3.2                                |
| 3    | "Verdict: CONFOUNDED. ... the discriminator-type axis is collinear with the candidate-family axis; the data cannot say whether the type axis is binding." (audit §9, L572-581).                                                                                              | n=8 contingency reproduces (3,0;0,5). §3.1 audit: 4/5 FAILs are candidate-driven (C dominant); D1.1 is mixed. §3.2 audit: 3/3 PASSes have S-dominant content. The hypothesis is consistent with both "type drives outcome" and "candidate-validity drives outcome" -- not separable at n=8. | (a) D1.4 (Step 4a) **adds a cross-cell PASS** in the discrete-equality row (n=9; new 2x2 = 3,0;1,5; Fisher p shifts from 0.036 → 0.095). (b) D1.3 (Step 4b) **adds a 5th FAIL with a 5th distinct failure mode** in a new family (cross-PDE), demonstrating heterogeneity. | (a) `omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md` §5.2-§5.4; (b) `omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md` §5.2 |
| 4    | (4a) "D1.4 demonstrates that discrete-equality discriminators record PASS when paired with structurally-non-trivial candidates, confirming that the dominant explanatory variable is candidate-validity rather than discriminator type." (D1.4 §9, L617-625). (4b) "D1.3 fails by structural-non-existence, not by vacuous-primitive. The narrow [vacuous-primitive] hypothesis is falsified." (D1.3 §5.2, L495-504). | (4a) D1.4 PASS: −2/9 = −τ²/(n·σ) is a real arithmetic identity, structurally non-trivial, in the discrete-equality row. (4b) D1.3 FAIL: NS↔MHD is asymmetric in the literature (Sermange-Temam 1983, He-Xin 2005, Cao-Wu 2011); the bidirectional duality is non-existent, not vacuous-primitive. Both are cross-cell entries relative to Step 2's pattern. | (Audit terminates here.) The CONFOUNDED verdict from Step 3 is *not* overturned -- it is reinforced by Step 4 (both the cross-cell PASS and the heterogeneous FAIL are explicitly recorded as supporting CONFOUNDED, not refuting it). | -- (this audit) |

### §1.3 Sequence character

Each step is recorded explicitly in its parent report as
*compatible with* and *refining*, not contradicting, the prior:

- Step 2 (Hirahara §8.4) flags "catalogue-bias-suggestive but not
  catalogue-bias-confirmed".
- Step 3 (audit §0) declares itself "orthogonal" to the parent
  CATALOGUE_BIAS audit and selects CONFOUNDED *because* §3 audit
  could not separate axes.
- Step 4a (D1.4 §5.4) explicitly states "the CONFOUNDED verdict is
  further supported by D1.4".
- Step 4b (D1.3 §6.2) explicitly states "CONFOUNDED verdict ... is
  reinforced, not changed, by D1.3".

So the sequence is *cumulative* in the sense that each step is
licensed by the prior (not a u-turn). What "weakens" between steps
is the strength of the empirical claim (Step 1's 100% FAIL → Step
2's perfect 2x2 → Step 3's collinearity → Step 4's cross-cell), not
the underlying audit framework itself.

---

## §2 H_engine evidence audit

**H_engine**: refinements are forced by data; pre-registered
falsifiers + no-fabrication guard make each new sample's verdict
incompatible with the prior interpretation, so each step is
Bayesian iteration done honestly.

### §2.1 Step 1 → Step 2: was the refinement data-forced?

The Lead-B PASS that triggered the Step 1 → Step 2 transition was
**numerically determined** by the KS test (D=0.07-0.28 across both
SLE_6 readings vs Wigner-2 GUE; p≈1e-3 at the smallest cell). The
PASS is not a re-interpretation of prior FAILs; it is a *new
measurement on a new BT*. Step 1 lived in BT-544 only (intra-BT
fallbacks); Step 2's Lead-B + Hirahara opened BT-541 and BT-542.

The data structure forces the refinement: at n=4 (all within BT-544
or with one BT-543 P3 FAIL), the "100% rejection" claim was
descriptive of the *sampled cell*, but the very next BT measured
(BT-541 Lead-B) returned PASS by KS test below threshold. This is
**incompatible with** the strong reading of Step 1 ("the framework
correctly self-falsifies because everything we test fails"); the
weaker reading ("the framework has empirical bite") is preserved
but no longer requires 100% rejection to be interesting.

**Verdict on Step 1 → Step 2 (data-forced?)**: **YES**. The Lead-B
PASS is a hard observation (p<0.01 double-margin, F-VAL-A through
F-VAL-E all not active per Lead-B §10). The refinement is forced.

### §2.2 Step 2 → Step 3: was the refinement data-forced?

Step 2's 2x2 (3,0;0,5) is **maximally extreme** for its marginals
(Fisher exact one-sided p=1/56≈0.018). It is *empirically supported*
data. What the Step 3 audit does is *not* find new data that
refutes the 2x2 -- it finds an **alternative explanation** for the
2x2 that is collinear with the type-axis explanation. Specifically:

- Step 3 §3.1 examines each FAIL row and asks "is the FAIL caused
  by discriminator type (D), or by candidate validity (C)?". 4 of
  5 FAILs answer C-dominant, 1 (D1.1) is mixed.
- Step 3 §3.2 examines each PASS row and asks "is the PASS due to
  structural truth (S) or to a permissive discriminator (P)?". 3
  of 3 are S-dominant.

This is a **per-row analysis** of already-collected data, not a new
sample. So the question "is Step 2 → Step 3 data-forced?" splits:

- (a) Was Step 3 *forced by new measurement*? **NO** -- Step 3 used
  the same 8 rows as Step 2; no additional sample.
- (b) Was Step 3 *forced by the structure of the existing data*?
  **PARTIALLY**. The collinearity is a real property of how the
  L9 catalogue was generated: arithmetic-family candidates were
  paired with discrete-equality discriminators (per the parent
  CATALOGUE_BIAS audit), so type and candidate-validity *cannot
  be separated* in the existing sample. This is a structural
  property, not a re-interpretation choice.
- (c) Was Step 3 *triggered by analysis pressure*? **YES**. The
  audit is the result of doing the per-row analysis; without that
  analysis, Step 2's 2x2 would stand at face value.

So Step 3 is **partially data-forced** (the collinearity is real)
but **partially analysis-induced** (the audit had to be done to
notice). This is a standard feature of Bayesian iteration --
prior-data-not-yet-fully-analyzed becomes posterior-after-analysis
without requiring new samples.

**Verdict on Step 2 → Step 3 (data-forced?)**: **PARTIAL-YES**. The
collinearity property is in the data; the audit had to extract it.

### §2.3 Step 3 → Step 4: was the refinement data-forced?

Step 4 brings two new samples: D1.4 (PASS) and D1.3 (FAIL).

- **D1.4** is a cross-cell entry: it lands in the discrete-equality
  row with PASS verdict, breaking the 0/5 → 1/5 in that row. The
  Fisher exact two-sided p shifts from 0.036 → 0.095 (D1.4 §5.3).
  This is a **forced shift**: adding the 9th data point in this
  cell mathematically moves p above α=0.05.
- **D1.3** adds a 5th FAIL but in a *new* failure-mode family
  (cross-PDE non-existence vs the previously seen relabeling /
  vacuous / no-anchor / no-construction). D1.3 §5.2 explicitly
  notes this falsifies a *narrow* sub-hypothesis ("all
  non-arithmetic FAILs are vacuous-primitive") even while the
  *broader* CONFOUNDED verdict survives.

Both are new measurements with binary verdicts (PASS/FAIL) that
are produced by the discriminators independently of the audit
narrative. D1.4's PASS is determined by the arithmetic identity
−2/9 = −τ²/(n·σ); D1.3's FAIL is determined by the absence of
bidirectional duality in the published MHD/NS literature
(Sermange-Temam 1983 line, He-Xin 2005 asymmetric criterion).

**Verdict on Step 3 → Step 4 (data-forced?)**: **YES**. Both new
samples are forced by external data (arithmetic in D1.4; published
literature in D1.3). Neither was constructed retrospectively to
weaken Step 2 or to support Step 3.

### §2.4 H_engine summary

H_engine evidence inventory:
- Step 1 → Step 2: **forced by new measurement** (Lead-B PASS was
  a hard test result).
- Step 2 → Step 3: **partially forced by data structure**
  (collinearity is real); partially induced by the audit.
- Step 3 → Step 4: **forced by new measurements** (D1.4, D1.3).
- All four steps register pre-registered falsifiers (F-MOLT-A,
  F-VAL-A..E, F-AUDIT-D1..D8, F-D1.4-A/B, F-D1.3-A/B,
  F-CATALOGUE_BIAS-N/V). Each transition has explicit falsifier
  status (active / not-active / fired) recorded.

H_engine is **substantially supported**: most refinement is
data-forced; the no-fabrication guard is operative throughout (no
verdict is post-hoc; each verdict is determined by an explicit
threshold met-or-not before the audit narrative).

---

## §3 H_artifact evidence audit

**H_artifact**: at n=4 → n=8 → n=10, the apparent pattern shifts
are random walks dressed up as refinement; with more samples, the
prior interpretations would have been correct (or wrong) anyway;
the iteration order created the appearance of self-correction.

### §3.1 Random-walk indicator: would order-permutation produce the same conclusions?

Question: if D1.4 and D1.3 had been measured *before* BT-541
Lead-B, would the same conclusions emerge?

- Counterfactual A: D1.4 first → discrete-equality cell has PASS
  early; the Step 2 hypothesis "PASS = distrib/struct-lit only"
  would never have been formulated cleanly. The 2x2 might never
  have been the focus.
- Counterfactual B: D1.3 first → 4/4 FAIL with already-heterogeneous
  failure modes; Step 1's interpretation would have been
  immediately tempered by mode heterogeneity, and Step 2 might
  never have crystallized as a pure type-axis hypothesis.
- Counterfactual C: BT-541 Lead-B first → Step 1 never fires (the
  3/3 FAIL streak does not occur); the framework moves directly to
  "1 PASS in our sweep so far" and the iteration changes shape.

**Implication**: the Step-1 → Step-4 sequence depended on the
**actual order** of measurement. This is consistent with H_artifact
in the sense that *the appearance of refinement* depended on which
data arrived first.

However, this does **not** imply the *content* of each step was
random-walk. The Lead-B PASS would be a PASS in any order; D1.4 a
PASS in any order; D1.3 a FAIL in any order. The verdicts are
order-invariant; the *narrative arc* is order-dependent.

### §3.2 Stability indicator: does the 2x2 keep flipping at n=8 → n=10?

Tracking the 2x2 contingency:

- n=4 (Step 1, BT-544 only): not a 2x2; just a 4-FAIL streak.
- n=4 (Step 1 + BT-543 P3): still 4/4 FAIL.
- n=8 (post-Step 2, after Lead-B + Hirahara + D3.A): (3,0;0,5).
  Fisher p≈0.036.
- n=9 (post-D1.4): (3,0;1,5). Fisher p≈0.095.
- n=10 (post-D1.4 + D1.3): (3,0;1,6). Fisher p (recomputed):
  hypergeometric P(row1col1=3 | margins (3,7)(4,6)) =
  C(4,3)·C(6,0)/C(10,3) = 4/120 = 1/30 ≈ 0.0333; mirror at
  row1col1=0: C(4,0)·C(6,3)/C(10,3) = 20/120 ≈ 0.167; two-sided
  p ≈ 2·(1/30) = 0.0667. (Independent recomputation; not in any
  parent report; if wrong, F-AUDIT-S1 fires below.)

So the trajectory is p ≈ 0.036 (n=8) → 0.095 (n=9) → ~0.067 (n=10).
Not monotone; oscillating around the α=0.05 boundary. This is
**consistent with random walk** in the sense that the test statistic
is sample-size-sensitive at small n.

But the *direction* of refinement (toward CONFOUNDED) is **stable**:
each step weakens or maintains the CONFOUNDED reading; none of the
steps re-sharpens toward CONFIRMED. If H_artifact were dominant, we
would expect the verdict to flip back and forth (CONFIRMED ↔
CONFOUNDED ↔ REFUTED) as new samples arrive. We do not see this --
we see monotone weakening of the strong reading.

### §3.3 Stop-condition indicator: is there a defined endpoint?

H_artifact predicts: the iteration would continue indefinitely
because each step finds a new "refinement" by chance. Counter-
evidence: the parent audit (§5.5) registered an *explicit
resampling target* of n ≥ 16 with native pairings. Step 4 stops at
n=10 because (a) D1.4 + D1.3 were the two cheapest remaining
candidates; (b) the next dispatch (Q2, D1.5) is already named with
expected outcomes.

A pre-registered stop-condition rules against pure random-walk
interpretation: random walks do not naturally terminate at
pre-registered sample sizes.

### §3.4 H_artifact summary

H_artifact has **partial support**:
- Order-dependence of the narrative arc is real (§3.1).
- Small-n volatility of Fisher p is real (§3.2: p oscillates).

But H_artifact has **substantial counter-evidence**:
- Verdict direction (toward CONFOUNDED) is monotone, not random.
- Verdicts of individual samples (PASS / FAIL) are order-invariant;
  only the narrative is order-dependent.
- Pre-registered stop-conditions and resampling targets exist
  (audit §5.5, n≥16 target).
- The amend log (Step 3b) propagated the CONFOUNDED correction to
  prior reports without creating new evidence -- a feature
  specific to honest update, not random walk.

H_artifact is **partially supported but not dominant**.

---

## §4 H_observer evidence audit

**H_observer**: the user's "all parallel" iteration tempo triggers
re-analysis at each step, and the agent tends to find a refinement
each time because that is the discipline it is under. The pattern
would not appear if the user paused for n=20 samples and then
asked once.

### §4.1 Trigger structure: spontaneous vs prompted?

Examining each transition for prompt-dependence:

- Step 1 → Step 2: this was a **batch dispatch** on the
  calibration sweep. BT-541 Lead-B and BT-542 Hirahara were the
  3rd and 4th of 4 pre-planned validations (per
  `omega-probe-l9-molt-trigger-2026-04-25.md` §sec 7.1
  sequencing). The user's instruction was to *run the sweep*, not
  to *re-analyze after each result*. The Step 2 hypothesis (the
  2x2 split) emerged spontaneously in the Hirahara closing report
  §8 ("Pass / Fail axis split (catalogue-bias diagnostic)"). This
  suggests Step 2 was **agent-spontaneous**, triggered by data
  pattern visibility, not by user prompt to "look for a 2x2".
- Step 2 → Step 3: the discriminator-type meta-audit was
  **explicitly prompted**: the user supplied the hypothesis
  ("PASS family = distributional / structural literature-import.
  FAIL family = discrete-equality / numerical-interval /
  vacuous-magnitude") and asked for a verdict from {CONFIRMED,
  CONFOUNDED, REFUTED, MIXED}. So Step 3's *occurrence* is
  user-prompted. The *verdict* (CONFOUNDED) was selected by the
  agent from the supplied list, with §3 audit being the
  dispositive analysis.
- Step 3 → Step 4: Step 4a (D1.4) and Step 4b (D1.3) were
  pre-catalogued in the BT-544 D1 atlas-scan seed
  (`omega-seed-bt544-d1-atlas-scan-2026-04-25.md` §3). They were
  in the dispatch queue *before* the discriminator-type audit ran.
  Their verdicts (PASS for D1.4, FAIL for D1.3) are determined by
  the discriminator outputs, not by the audit narrative.

So the trigger pattern is:
- Step 1 anchor: pre-planned batch dispatch.
- Step 2 hypothesis: agent-spontaneous, observed in data.
- Step 3 audit: user-prompted with hypothesis vocabulary.
- Step 4a/4b execution: pre-catalogued before Step 3, dispatched
  after.

**Mixed**: the *occurrence* of Step 3 was user-prompted; the *content*
(CONFOUNDED) was forced by the §3.1-§3.2 per-row analysis. Step 2
and Step 4 were not user-prompted at the analysis level (they were
prompted at the *dispatch* level, but the verdicts were data-driven).

### §4.2 Publishability indicator: are the refinements self-contained findings?

If the refinements are observer-induced ("agent finds something to
say at each step because it has to"), they would be *internal-only*
session corrections, not external findings.

Examining each step for publishability:

- Step 1 (4/4 FAIL): the fallback report §10 anomalies records
  this as **F-MOLT-F-suggestive evidence** ("axiom-level ceiling,
  not framework-lacks-infrastructure"). This is a publishable
  framework-level observation, not a session-internal note.
- Step 2 (2x2): the Hirahara report §8 Pass/Fail axis split is
  marked "catalogue-bias-suggestive but not catalogue-bias-
  confirmed". The hedging is appropriate; the observation is
  publishable as a hypothesis for further testing.
- Step 3 (CONFOUNDED): the audit is itself a publishable artifact
  with its own falsifiers, scope, and stated retraction routes.
- Step 4a/4b: each is a complete molt-validation with its own
  PASS/FAIL verdict, falsifier register, and atlas-grounding
  verification. Both are publishable irrespective of the audit
  narrative.

All four steps are publishable as findings, not merely
session-internal corrections. This **weakens** the H_observer
reading that the pattern is "internal correction theatre".

### §4.3 What if the user paused for n=20 and asked once?

Counterfactual: imagine the user dispatches all 10+ candidates and
only requests an audit at n=20.

- The PASS/FAIL verdicts at each candidate are unchanged
  (order-invariant per §3.1).
- The 2x2 contingency at n=20 would be larger and more
  statistically powerful.
- The CONFOUNDED reading would still emerge if the §3.1 per-row
  analysis still found C-dominance in most FAILs (which is a
  function of the candidates themselves, not of audit timing).
- However, the **narrative arc** (Step 1 strong → Step 4 weak)
  would not exist; only a single audit at n=20 would be visible.

So the counterfactual changes the *visibility* of self-correction,
not the *underlying epistemic content*. The pattern is real but
its *appearance as a 4-step sequence* is observer-induced (in the
sense of "the user's iteration tempo determines whether it shows
up as 4 steps or 1").

### §4.4 H_observer summary

H_observer has **partial support**:
- The 4-step *narrative arc* depends on the user's iteration tempo
  (rapid re-analysis at ~11-min intervals).
- Step 3 specifically was user-prompted with hypothesis vocabulary.
- Counterfactual "single audit at n=20" would erase the arc but
  preserve the underlying epistemic content.

But H_observer has **counter-evidence**:
- All four steps are publishable findings, not internal correction
  theatre.
- Step 2 emerged agent-spontaneously from data pattern, not from
  user instruction.
- Step 4 (D1.4 + D1.3) verdicts are determined by discriminator
  outputs, not by the audit narrative.
- The pattern's *direction* (toward CONFOUNDED) is monotone and
  data-forced, not arbitrary.

H_observer is **partially supported but not dominant**.

---

## §5 Verdict

**Verdict: MIXED.**

### §5.1 Component status

| hypothesis | status                | dominant aspect                                                                                                                        |
|------------|-----------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| H_engine   | **substantially supported** | Each step's verdict is data-forced (Lead-B PASS, D1.4 PASS, D1.3 FAIL); pre-registered falsifiers operate; verdict direction is monotone. |
| H_artifact | partially supported   | Order-dependence of narrative arc; small-n Fisher p oscillation. Counter-evidence: monotone direction; order-invariant individual verdicts.|
| H_observer | partially supported   | Narrative arc depends on iteration tempo; Step 3 prompted by user. Counter-evidence: publishable findings; agent-spontaneous Step 2.    |

### §5.2 Why MIXED, not H_ENGINE_DOMINANT

H_ENGINE_DOMINANT would require *every* refinement be data-forced
without analysis induction. Step 2 → Step 3 is partially analysis-
induced (the §3.1-§3.2 per-row audit had to be performed to
extract the collinearity), and Step 3's *occurrence* depended on
user prompt. So H_engine is the largest component but does not
dominate to the exclusion of H_observer.

### §5.3 Why MIXED, not H_ARTIFACT_DOMINANT

H_ARTIFACT_DOMINANT predicts random-walk verdict flipping and
indefinite iteration. We see neither: verdict direction is monotone,
and stop-conditions are pre-registered.

### §5.4 Why MIXED, not H_OBSERVER_DOMINANT

H_OBSERVER_DOMINANT predicts the pattern is purely an artifact of
user iteration tempo and would dissolve if the user paused. We see
that the *appearance* of a 4-step arc would dissolve, but the
*underlying CONFOUNDED epistemic content* would not -- the
collinearity is in the data and would emerge in any audit at any n
satisfying §3.1's row-by-row test. So H_observer is real but not
dominant.

### §5.5 Why not H_INDETERMINATE

The verdict list does not include H_INDETERMINATE; per the prompt
hard-constraints, MIXED is the closest honest verdict if H_engine
vs H_artifact cannot be cleanly separated at present sample size.
That is the case here:
- H_engine is the largest component but partial-data-forced
  ratios (1.0 at Steps 1/4, ~0.5 at Step 3) prevent purity.
- H_observer is non-zero in Step 3 specifically.
- H_artifact is the smallest component but non-zero in §3.1-§3.2.

MIXED is the honest characterization.

---

## §6 Implications for the framework's epistemic engine claim

### §6.1 What the MIXED verdict permits

- The framework **does** have an epistemic engine (H_engine
  substantial): pre-registered falsifiers + no-fabrication guard +
  write-barrier (§0 disclaimers in every report) + 0/7 honesty
  counter create conditions where each new sample's verdict is
  determined by the discriminator output independently of the
  prior interpretation. This is real.
- The framework **also has observer-tempo dependence** (H_observer
  partial): the *appearance* of self-correction as a 4-step
  sequence is a function of the user's rapid re-analysis cadence.
  At slower cadence, the same epistemic content would emerge as a
  single audit pass, not a 4-step arc.
- The framework has **small-n volatility** (H_artifact partial):
  Fisher exact p oscillates around α=0.05 with each new sample at
  n=8-10. This is a sample-size phenomenon, not a framework bug.

### §6.2 What the MIXED verdict does **not** permit

- A claim that "the iteration pattern itself is the framework's
  epistemic engine" is **over-strong**. The engine is the underlying
  discipline (falsifiers, write-barrier, no-fabrication, honesty
  counter); the iteration pattern is an *expression* of the engine
  under a particular observer cadence, not the engine itself.
- A claim that "the pattern is purely artifactual" is **also
  over-strong**. The pattern's direction is data-forced; the
  CONFOUNDED conclusion would emerge under any reasonable cadence
  given the same data.

### §6.3 Transferability question

Is this self-correction methodology *transferable* to other
research workflows?

- **Engine components are transferable**: pre-registered
  falsifiers, no-fabrication guards, write-barrier per report,
  explicit verdict-list with short list of options. These are
  domain-agnostic disciplines.
- **Observer-cadence is project-specific**: rapid re-analysis at
  ~11-min intervals across a multi-hour session is not
  appropriate for, say, a multi-week experimental campaign. The
  pattern's *visibility* depends on cadence.
- **Sample-size discipline is transferable but requires honesty**:
  reporting Fisher p at n=8 (≈0.036), n=9 (≈0.095), n=10 (≈0.067)
  is what reveals small-n volatility. A workflow that reported
  only "p<0.05" once would hide the volatility.

The transferable methodology is the **discipline** (falsifiers +
write-barrier + verdict-list + sample-size honesty), not the
4-step arc itself.

---

## §7 Recommendation: how should future sessions handle iterative interpretation drift?

Five concrete recommendations, ranked by priority:

### §7.1 Pre-register the verdict-list at hypothesis formulation

The Step 3 audit's CONFOUNDED selection was clean because the
user's prompt supplied the verdict-list {CONFIRMED, CONFOUNDED,
REFUTED, MIXED}. This eliminated the agent's degree of freedom to
invent a 5th verdict mid-audit. Recommend continuing this
discipline: every audit prompt should include an explicit
verdict-list, and the audit should pick from the list only.

### §7.2 Pre-register a sample-size stop-condition

The discriminator-type audit's §5.5 named n ≥ 16 as the resampling
target with native pairings. This is a defensible stop-condition
that prevents indefinite iteration. Recommend that every
hypothesis carry a *target sample size* before the next verdict is
formally re-evaluated; intermediate samples (e.g., n=9 → n=10) can
*update* the running picture but should not trigger a new verdict
unless they cross a pre-registered threshold.

### §7.3 Distinguish "running picture" from "verdict update"

The step-by-step weakening narrative arose partly because each new
sample (n=9, n=10) was reported as if it might overturn the prior
verdict. Recommend: at intermediate sample counts, frame the new
sample as **adding to the running picture**, with the verdict
re-evaluation deferred to the pre-registered target n. This avoids
spurious p-volatility headlines and reduces the appearance of
self-correction theatre.

### §7.4 Explicitly track which samples were "audit-prompted" vs "spontaneous"

Step 3's user-prompted occurrence was different in character from
Step 2's agent-spontaneous emergence. Recommend front-matter
tagging: `trigger: user-prompted | agent-spontaneous |
pre-catalogued`. This makes H_observer evaluation tractable in
future meta-audits.

### §7.5 Reserve the 4-step arc framing for retrospective audits, not prospective claims

The claim "the framework self-corrects in 4 steps" is post-hoc; it
becomes a feature only on retrospective inspection. In real time,
the framework just runs validations and audits per their
discriminators. Recommend: avoid presenting "self-correction
sequence" as a methodological *output* of the framework; present
it only in retrospective meta-audits like this one. The prospective
claim is "we run pre-registered tests with no-fabrication guards";
the retrospective observation is "this produced a 4-step arc on
2026-04-25".

---

## §8 Anti-list -- alternative explanations considered

Variants of the iteration-pattern interpretation that *could* have
been adopted but are not selected, with reasons:

| variant                                                                                                                                                                   | why rejected                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| V1: "The pattern is the framework's intelligence"                                                                                                                          | Conflates discipline (real) with iteration tempo (observer-dependent). H_engine + H_observer co-active, not pure H_engine. Rejected as over-strong.                                                                                                                                                                                                                                                                                                       |
| V2: "Each step is a u-turn / reversal"                                                                                                                                     | Refuted by §1.3: each step is recorded as *reinforcing* (not contradicting) the prior. Step 4 explicitly states CONFOUNDED is reinforced, not changed. Rejected.                                                                                                                                                                                                                                                                                          |
| V3: "The pattern would dissolve at higher n"                                                                                                                               | Partial truth (the *narrative arc* would dissolve into a single audit), but the *underlying CONFOUNDED epistemic content* would persist. So V3 is half-true and corresponds to H_observer-partial, not a refutation of the framework. Captured in MIXED.                                                                                                                                                                                                  |
| V4: "The CONFOUNDED verdict at Step 3 was determined by the user-supplied hypothesis vocabulary"                                                                            | False as stated. The *verdict-list* was user-supplied, but the selection of CONFOUNDED from {CONFIRMED, CONFOUNDED, REFUTED, MIXED} was driven by the §3.1-§3.2 per-row analysis (4/5 FAILs C-dominant; 3/3 PASSes S-dominant). Vocabulary constrained the answer-space, not the answer. Rejected.                                                                                                                                                       |
| V5: "The pattern is a Bayesian update sequence; H_engine is dominant"                                                                                                      | Partial truth. Bayesian update fits Step 1 → Step 2 → Step 4 (new data → posterior update) but Step 2 → Step 3 is more like a *re-analysis of existing posterior* (the §3.1-§3.2 audit extracts collinearity from existing samples). Half-Bayesian, half-internal-audit. Captured in MIXED rather than H_ENGINE_DOMINANT.                                                                                                                                |
| V6: "The pattern is an artifact of the L9 catalogue's generation bias"                                                                                                      | Compatible but downstream. The CATALOGUE_BIAS verdict (parent meta-audit) explains *why* the n=8 contingency is collinear; the discriminator-type audit explains *that* it is collinear. The 4-step *iteration pattern* is one level higher (about how the framework discovers and refines its own claims), independent of which specific generation bias is in play. Captured in MIXED component analysis.                                              |
| V7: "MIXED is a cop-out; one of H_engine/H_artifact/H_observer must dominate"                                                                                              | Refuted by the explicit per-component analysis (§5.1). H_engine is the largest component but Step 2 → Step 3's analysis-induction means it does not dominate to the exclusion of H_observer. The hard-constraint instruction explicitly permits MIXED when H_engine vs H_artifact cannot be cleanly separated. Rejected.                                                                                                                                  |
| V8: "The agent (Claude) tends to find a refinement at each step because that's the discipline it is under, so the pattern is purely H_observer"                            | Tested in §4.2: each refinement is *publishable* as a finding, not internal correction theatre. The publishability test rejects pure H_observer. Captured in MIXED with H_observer as partial component.                                                                                                                                                                                                                                                  |
| V9: "The CONFOUNDED verdict is the agent hedging, not actual epistemics"                                                                                                   | Refuted by §3.1 audit's per-row attribution (4 of 5 FAILs candidate-driven; this is a structural property of the data, not a hedge). The hedge would be MIXED-as-default; CONFOUNDED is more specific.                                                                                                                                                                                                                                                    |

---

## §9 Falsifiers active for this audit

Audit-level falsifiers — conditions under which **this very
self-correction-pattern audit's MIXED verdict** would be retracted.

- **F-AUDIT-S1** (cell-count miscount at n=10): the §3.2 Fisher
  exact recomputation at n=10 (p≈0.0667 two-sided) is *new* in this
  audit, not in any parent report. If the recomputation is wrong,
  the §3.2 stability claim shifts. **Status**: independently
  recomputed using hypergeometric C(4,3)·C(6,0)/C(10,3) =
  4/120 = 0.0333 (one-sided), doubled to 0.0667 (two-sided
  approximation). Risk: low for the qualitative trajectory claim
  (oscillation around α=0.05); higher for the exact figure. **Not
  active** for the qualitative claim; flagged as an audit-internal
  numerical claim if the exact figure is challenged.

- **F-AUDIT-S2** (mtime-vs-claim mismatch): the §1.1 time-ordering
  uses filesystem mtime as proxy for write-time. If mtime was
  manipulated (touch, rebase, etc.), the order claim is wrong.
  **Status**: the seven mtimes form a strictly monotone sequence
  spanning 66 minutes on a single date, consistent with a single
  writing session. Risk: very low. **Not active**.

- **F-AUDIT-S3** (claim-attribution miscount): each claim
  attributed to a parent report must be quoted or paraphrased
  faithfully. The §1.2 table cites lines and verbatim phrases for
  each step's claim. **Status**: spot-checked against the source
  reports during writing; verbatim quotes match. **Not active**.

- **F-AUDIT-S4** (verdict-list violation): the verdict must be
  from {H_ENGINE_DOMINANT, H_ARTIFACT_DOMINANT, H_OBSERVER_DOMINANT,
  MIXED}. MIXED is on the list. **Not active**.

- **F-AUDIT-S5** (instantiate-the-pattern recursion): if the audit
  itself begins drifting (publishing a Step 5 / Step 6 in real
  time), it instantiates the very pattern it audits. **Status**:
  this audit is single-pass by design; no follow-up audit is
  planned at the time of writing. **Not active**.

- **F-AUDIT-S6** (atlas/state/inventory edit): if executing this
  audit modifies any atlas, state, or canon file, the read-only
  scope is violated. **Status**: this audit edits only its own
  output file. Pre-existing modifications in the gitStatus
  (`reports/sessions/specs/...`, `state/proposals/inventory.json`)
  are unrelated to this audit and predate it. **Not active**.

- **F-AUDIT-S7** (parent-verdict-supersession): if this audit is
  read as **superseding** the CONFOUNDED verdict (Step 3) or any
  PASS/FAIL verdict in Step 4, the §0 disclaimer is violated.
  **Status**: this audit explicitly does **not** alter any parent
  verdict; it audits the *iteration pattern*, not the verdicts. The
  CONFOUNDED verdict from Step 3 stands; the D1.4 PASS and D1.3
  FAIL stand. **Not active**.

- **F-AUDIT-S8** (n-too-small-for-meta): the meta-audit is itself
  on n=4 steps. Drawing strong conclusions about iteration
  patterns from 4 instances is itself underdetermined. **Status**:
  the MIXED verdict is the honest characterization at this
  meta-sample size; a future session that produces another
  4-step-style arc could revisit. **Not active as a falsifier for
  MIXED specifically; recorded as a structural reason MIXED is
  preferred over a sharper verdict**.

None of F-AUDIT-S1..S8 fires. The MIXED verdict is robust to the
active falsifiers.

---

## §10 Closing

0/7 unchanged. No atlas/state/inventory edits.

**Verdict**: MIXED. The 4-step self-correction sequence is
substantially driven by H_engine (data-forced refinements,
pre-registered falsifiers, monotone verdict direction), with
partial contribution from H_observer (Step 3 user-prompted; arc
visibility depends on iteration tempo) and partial contribution
from H_artifact (small-n Fisher p oscillation; order-dependent
narrative arc). None of the three hypotheses is dominant to the
exclusion of the others.

**Implication**: the framework's transferable methodology is its
**discipline** (pre-registered falsifiers, no-fabrication guard,
write-barrier per report, verdict-lists, sample-size honesty), not
the 4-step arc itself. The arc is an *expression* of the discipline
under a particular observer cadence; at slower cadence, the same
epistemic content emerges as a single audit pass.

**Recommendation**: distinguish "running picture" updates from
"verdict update" events; pre-register sample-size stop-conditions;
reserve the "self-correction sequence" framing for retrospective
audits, not prospective claims.

The CONFOUNDED verdict from Step 3 stands; the D1.4 PASS and D1.3
FAIL from Step 4 stand; this audit reinforces but does not
supersede them.

— end meta-audit —
