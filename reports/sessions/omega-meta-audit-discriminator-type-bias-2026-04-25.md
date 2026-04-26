---
id: omega-meta-audit-discriminator-type-bias
date: 2026-04-25
scope: research-only meta-audit (NOT modifying L9 catalogue; sharpening bias diagnostic)
target: discriminator-type bias hypothesis -- precision audit on n>=9 samples
parent_reports:
  - reports/sessions/omega-meta-audit-l9-catalogue-design-2026-04-25.md (CATALOGUE_BIAS verdict)
  - reports/sessions/omega-exec-bt544-q1-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt543-p3-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt541-leadB-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-fallback-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt542-hirahara-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: meta-audit, no claim
---

# Omega Meta-Audit -- Discriminator-Type Bias Hypothesis (2026-04-25)

## §0 Non-claim disclaimer

This report **only** sharpens the discriminator-type bias diagnostic
that was first stated after BT-541 + BT-542 closed the F-MOLT-A 4-BT
sweep (n=4) and is now testable on the post-D1/D2/D3 expanded set
(n>=9). It does **NOT**:

- claim 3D NS regularity, RH, P vs NP, or any Millennium resolution;
- modify `shared/n6/atlas.n6`, `state/proposals/inventory.json`,
  `theory/canon/`, or any L9 / D1 / D2 / D3 source document;
- promote, demote, or otherwise touch atlas grades;
- supersede the prior CATALOGUE_BIAS verdict in the parent meta-audit
  (`omega-meta-audit-l9-catalogue-design-2026-04-25.md`); this audit is
  *orthogonal*, asking specifically about the **discriminator type**
  axis rather than the **candidate generation** axis;
- treat ACCEPTABLE verdicts (D2 R1, D2 R5) as molt-validation outcomes.
  They are **L_ω axiom-recast acceptability** results, scored on a
  separate criterion grid; they enter only as edge-case rows in §6.

Millennium tally: **0/7 unchanged**.

The hypothesis under test (verbatim, as supplied):

> **PASS family** = distributional / structural literature-import.
> **FAIL family** = discrete-equality / numerical-interval /
> vacuous-magnitude.

The verdict is one of {CONFIRMED, CONFOUNDED, REFUTED, MIXED}, and the
audit is read-only.

---

## §1 Sample table (canonical fields extracted from the 9 reports)

Each row reports the BT id, the frame-shift name, the molt verdict
*as recorded in the parent report*, the discriminator-type label
(restricted to the supplied label vocabulary), and a numerical /
qualitative pass-fail margin. **No counts or labels are fabricated**;
each cell is sourced from the parent report cited in the row.

| # | BT row              | Frame-shift                               | Verdict           | Discriminator type             | Margin / mechanism                                                                                                                                                       | Source                                              |
|---|---------------------|-------------------------------------------|-------------------|--------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------|
| 1 | BT-544 Q1           | KdV 6-soliton Gram-lattice                | FAIL              | discrete-equality              | rank=6 vs target 3 on both κ families; det/σ = 130.95 / 157.22, neither in ℤ; double-margin per §5 of Q1 report                                                          | omega-exec-bt544-q1-molt-validation                  |
| 2 | BT-544 Q5           | Sobolev/Besov mechanism seed              | FAIL              | discrete-equality              | 0 of 7 catalog rows non-trivial; 3 synthesis attempts all reduce to "arithmetic without analysis"; F-544-B fires (no concrete estimate producible)                       | omega-exec-bt544-fallback-molt-validation §3-§4      |
| 3 | BT-544 KPZ d=7      | KPZ d-lift to second perfect number       | FAIL              | discrete-equality              | 0 literature attestations of (χ_7, z_7)=(1/7,6/7); ansatz χ_d=1/d already fails at d=2 substrate (≈0.39 vs 1/2); F-544-C fires                                            | omega-exec-bt544-fallback-molt-validation §5-§6      |
| 4 | BT-544 D1.1         | HVC Bekenstein cap on enstrophy           | FAIL              | vacuous-magnitude              | cap=52.36 (Lamb-Oseen) / 13.09 (Taylor-Green); max Φ/cap = 0.000 / 0.004 ≪ 0.1 threshold; F-D1.1-B fires (cap decorative, primitive vacuous)                              | omega-exec-bt544-d1-1-hvc-molt-validation §5         |
| 5 | BT-543 P3           | A4-ratio-only frame                       | FAIL              | numerical-interval             | 3/3 outliers above [2.5, 3.5]: MP1999 R=3.93, BMW2008 R=3.89, FLAG2024 R=3.64; even 1σ-extreme readings stay outside; F-NP1-A4rev fires                                  | omega-exec-bt543-p3-molt-validation §5               |
| 6 | BT-541 Lead-B       | SLE_6 × GUE coupling                      | PASS              | distributional                 | KS p < 0.01 on every scale × both readings; min p=1.0e-3 (Cardy at n=1000), max-margin underflow; double-margin PASS                                                     | omega-exec-bt541-leadB-molt-validation §5            |
| 7 | BT-542 Hirahara     | non-naturalized MCSP / non-black-box      | PASS_LITERATURE   | structural-literature          | Hirahara 2018 FOCS + 2022 FOCS + OS-2017 + RR-1997 chain documented in v3-T5; non-natural step exists, primitive absent from GATE-BARRIER frame                          | omega-exec-bt542-hirahara-molt-validation §3-§5      |
| 8 | BT-544 D3.A         | 2.5D non-local-pressure axis A clean      | PASS_LITERATURE   | structural-literature          | (★) ‖v(t)‖_{H^s} ≤ C(t,ν,‖v_0‖_{H^s}) by Ladyzhenskaya 1969 + Calderón-Zygmund 1952 + BKM 1984 + Friedman 1964; 12-step sketch with classical inputs, no obstruction      | omega-exec-bt544-d3-A-axis-discriminator §4-§6       |
| 9 | BT-544 D2 R1        | Onsager-Hölder threshold α* recast        | ACCEPTABLE        | axiom-recast                   | Millennium-grade preserved; partial-result import dense (Onsager 1949 → CET 1994 → Isett 2018 → Buckmaster-Vicol 2019); α* a new invariant per F-544-B                   | omega-exec-bt544-d2-r1r5-acceptability §2            |
| 10| BT-544 D2 R5        | Uniform-in-Mach low-Mach recast           | ACCEPTABLE        | axiom-recast                   | Millennium-grade preserved; partial-result import dense (Klainerman-Majda 1982 → Schochet 1986 → Lions-Masmoudi 1998 → Feireisl-Novotný 2009); ε-uniformity invariant new | omega-exec-bt544-d2-r1r5-acceptability §3            |

**Total**: 10 rows. Of these, **8 are direct molt-validations** (rows
1-8); **2 are L_ω acceptability checks** (rows 9-10) on a separate
criterion grid.

For the 2×2 contingency, only the 8 molt-validation rows enter
(verdict ∈ {PASS, FAIL, PASS_LITERATURE}; PASS_LITERATURE is treated
as PASS for the contingency, per the parent reports' own framing of
PASS_LITERATURE as a real-molt outcome, not an inconclusive). The
ACCEPTABLE rows are addressed separately in §6.

---

## §2 2×2 contingency table + Fisher exact ratio

Rows 1-8 (the molt-validation set; n=8). Discriminator-type axis
**collapsed** to two cells per the hypothesis:

- Row "PASS-family of hypothesis": discriminator type ∈
  {distributional, structural-literature}.
- Row "FAIL-family of hypothesis": discriminator type ∈
  {discrete-equality, numerical-interval, vacuous-magnitude}.

|                                                              | PASS verdict | FAIL verdict |
|--------------------------------------------------------------|--------------|--------------|
| **Distributional / structural-literature**                   | 3            | 0            |
| **Discrete-equality / numerical-interval / vacuous-magnitude**| 0            | 5            |

Cell-by-cell derivation:

- (PASS, distrib/struct-lit) = 3: row 6 BT-541 Lead-B (distributional),
  row 7 BT-542 Hirahara (structural-literature), row 8 BT-544 D3.A
  (structural-literature).
- (FAIL, distrib/struct-lit) = 0.
- (PASS, discrete-eq/num-int/vacuous) = 0.
- (FAIL, discrete-eq/num-int/vacuous) = 5: rows 1, 2, 3 (discrete-
  equality), row 5 (numerical-interval), row 4 (vacuous-magnitude).

**Marginal sums**: 3 PASS + 5 FAIL = 8; 3 distrib/struct + 5
discrete/interval/vacuous = 8 (consistent).

### §2.1 Fisher exact two-sided p-value

For the 2×2 table

```
[3 0]
[0 5]
```

with row totals (3, 5) and column totals (3, 5), the hypergeometric
probability of observing the **exact** (3, 0; 0, 5) split is

  P((3,0;0,5)) = C(3,3)·C(5,0) / C(8,3) = 1·1 / 56 = 1/56 ≈ 0.0179

Under Fisher's exact test (two-sided), the only equally-or-more
extreme arrangement under the same marginals is the swapped (0,3;5,0)
(also probability 1/56), and both intermediate configurations
(2,1;1,4) and (1,2;2,3) have higher probability and are not "more
extreme". So:

  p_two-sided = 2 × 1/56 = **2/56 ≈ 0.0357**.

The one-sided p (testing "PASS concentrates in distrib/struct") is
**1/56 ≈ 0.0179**.

**Interpretation**: the contingency is at the extreme of the marginal
distribution, with two-sided p ≈ 0.036 (one-sided p ≈ 0.018). At
n=8 this is the **strongest possible** deviation from independence
under the row/column marginals observed.

### §2.2 Simple ratio summary

- PASS-rate within distrib/struct-literature row: 3/3 = **100%**.
- PASS-rate within discrete-eq/num-int/vacuous row: 0/5 = **0%**.
- Cross-cell entries (which would weaken the hypothesis): **0**.

The hypothesis is, on the n=8 molt-validation set, **maximally
concentrated** along the predicted axis. No cross-cell evidence
exists.

> **Update v3 (2026-04-25, post D1.3 + D1.4)**: Subsequent samples produced cross-cell entries in both rows. Updated 2×2: distrib/struct-lit row = 3 PASS / 1 FAIL (D1.3 FAIL added); discrete-equality/interval/vacuous row = 1 PASS / 5 FAIL (D1.4 PASS added). Fisher exact two-sided p moves from 0.036 to ~0.13. CONFOUNDED verdict stands and is reinforced — type axis is no longer cleanly separable from candidate-validity at n=10.

> **Amendment v4 (2026-04-25, post P5+P6+P7 n=14 batch)**: With 3 additional native-paired samples (P5 BT-541-Mertens-KS PASS_DISTRIBUTIONAL, P6 BT-546-stratum-irrep FAIL_NO_MATCH, P7 BT-545-IHC-dim PASS_DISCRETE_MATCH), the 2×2 is now (4,2;2,6) (P6 top) or (3,3;2,6) (P6 bottom). Fisher exact p moves from 0.13 (n=10) to 0.2774 / 0.0909 (n=14) — **away from H_type significance**. Per `omega-exec-n20-p567-batch-2026-04-25.md`, **Scenario A (H_validity dominant)** confirmed under both classifications. The CONFOUNDED verdict is reinforced AND refined: candidate-validity is the dominant driver, NOT discriminator-type. n=20 P8/P9/P10 needed for full separation. 0/7 unchanged.

---

## §3 Alternative-explanation audit

The §2 statistic is suggestive but not dispositive: the
discriminator-type label is **correlated** with the candidate's
underlying merit, and a confound between the two could mimic a
discriminator-type effect. This section interrogates each row to
separate (D) "the FAIL/PASS is caused by the discriminator type"
from (C) "the FAIL/PASS is caused by the candidate's underlying
validity / invalidity".

### §3.1 Per-FAIL audit

| # | BT row          | Was the FAIL discriminator-driven (D), or candidate-driven (C)?                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|---|-----------------|--------|
| 1 | BT-544 Q1       | **Mostly C, partly D**. The candidate (KdV phase-shift Gram-lattice) is *also* algebraically wrong: rank(M) = 6 ≠ 3 is a structural fact, not a threshold artifact. Even relaxing the integrality threshold (det/σ ∈ ℤ → "near-integer") does not flip the rank=6 finding. The discrete-equality discriminator did not cause the failure; it *recorded* a structural mismatch. (Note: a **distributional** reading — e.g. KS distance of M's spectrum vs a reference Wishart spectrum — would have produced a non-trivial p-value, but the hypothesis is "what the spec asked", not "what could have been asked".)|
| 2 | BT-544 Q5       | **C dominant**. The Q5 catalog walk found 0 category-(b) estimates; 3 synthesis attempts produced arithmetic exponents (5/12 etc.) without PDE backing. The fail is "no concrete estimate exists", which is candidate-validity. The discrete-equality framing (does an estimate satisfy a literal lattice-derived inequality) reflects the catalog being empty, not the discriminator being too strict.                                                                                                                                                                                                                                                                                                                          |
| 3 | BT-544 KPZ d=7  | **C dominant**. No literature attestation exists; the upstream ansatz already fails at d=2 substrate (numerical KPZ χ_2 ≈ 0.39 vs predicted 1/2). The fail is candidate-driven (no anchor); the discriminator merely records the absence.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| 4 | BT-544 D1.1     | **D-mixed-with-parameter-choice**. The HVC Bekenstein cap is satisfied trivially (Φ ≪ cap by 250×) but the cap formulation is *vacuous* under the chosen R, ν, T, MI_BINS=8. The D1.1 report (§7.2) explicitly registers a parameter-sensitivity follow-up that could rescue HVC to "non-vacuous in core-resolved regime". So the FAIL here is partly a **parameter-tuning failure** and partly a structural one: the 8-bin MI estimator is scale-invariant on multiplicatively-separable flows, hence cannot witness viscous decay (A8.2). The vacuous-magnitude discriminator is sensitive to this; a distributional discriminator (e.g. KS of histogram vs uniform) would still fail the *non-vacuous gap* requirement. **Mixed** — but tilts toward C because the cap is structurally tight on smooth flows. |
| 5 | BT-543 P3       | **C dominant, with a strictness sub-component**. All 3 lattice sources sit above [2.5, 3.5]; even 1σ extremes stay outside. The body-text heuristic 3.57 is itself outside the interval. The fail is empirical (the world's lattice numbers do not satisfy the predicted interval), not a threshold artifact. The strictness audit in the parent meta-audit (§3.2 of catalogue-design report) confirms: relaxing to ≥3 outliers from ≥2 would only flip P3 to NEAR, not PASS. Discriminator type plays a minor role; underlying lattice physics is the binding constraint.                                                                                                                                                          |

**FAIL-side summary**: 4 of 5 FAILs are dominantly C (candidate-
validity-driven). Only D1.1 is **mixed**, with a non-trivial D
(discriminator-type / parameter-choice) component. None of the FAILs
is *purely* D-driven.

This means the §2 statistic (3/3 vs 0/5) **overstates** the
discriminator-type effect: most of the FAILs would fail under any
reasonable discriminator because the candidates do not introduce a
real new primitive. The hypothesis "discriminator type drives PASS/FAIL"
is **partially confounded** with "candidate validity drives PASS/FAIL".

### §3.2 Per-PASS audit

| # | BT row              | Was the PASS supported by structural truth (S), or by a permissive discriminator (P)?                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
|---|---------------------|---|
| 6 | BT-541 Lead-B       | **S dominant**. KS test at N=50,000 with min p=1.0e-3 across 6 multi-scale × 2-reading cells is well above the threshold p<0.01 by an order of magnitude (double-margin per §5). Even Reading B (Cardy, the *closest* SLE_6 reading to GUE) is rejected at every scale. The κ=6 boundary-crossing distribution is structurally distinct from Wigner-Dyson β=2; that is a known fact about Smirnov 2001 + Cardy 1992 vs Montgomery-Odlyzko 1973. **Permissive worry**: KS at N=50k rejects *any* two non-identical distributions; the F-VAL-C falsifier in §10 of the Lead-B report flags this as "p<0.01 is too lenient on Reading B (D=0.07)". The PASS *survives* a tightening to effect-size criteria too (D=0.07 is small but real). Permissiveness exists at the spec level but does not flip the verdict.                                                       |
| 7 | BT-542 Hirahara     | **S dominant, P-flagged**. The Hirahara 2018 non-black-box reduction technique is FOCS 2018 best paper; OS-2017 + Hirahara 2022 + Razborov-Rudich 1997 chain is published, peer-reviewed, repo-registered (v3-T5 200-line digest). Structural truth is solid. **Permissive worry**: structural-literature-import is by design permissive — *any* new published primitive that lands in the right neighborhood satisfies the discriminator. F-VAL-B (§10 of Hirahara report) flags "if a technical reading shows non-black-box reductions DO satisfy a generalized RR predicate, the molt collapses to F-542-A". This is a non-trivial open technical question; the parent report concedes "PASS_LITERATURE, not PASS_CONSTRUCTION". The PASS rests on **literature consensus** that the chain is non-naturalized, which is the scope of structural-literature-import. |
| 8 | BT-544 D3.A         | **S dominant, with a known-result caveat**. The 2.5D ansatz reduces to 2D NS + linear advection-diffusion of a passive scalar, both globally regular by Ladyzhenskaya 1969 + standard parabolic theory. The PASS is truthful but **predictable from the literature** — the D3.A report itself reclassifies the result as "literature-settled" rather than "moderate difficulty" (§2.3 of D3.A report) and partially activates F-D3-META-D ("top-1 dispatch settled by literature import"). Structural-literature-import discriminator is permissive *here* in the sense that it is satisfied by any reduction to known-regular components; the PASS contains genuine structural content but is **not** a new theorem. |

**PASS-side summary**: All 3 PASSes are S-dominant (structural truth
exists); all 3 carry a permissiveness flag at the level of "the
discriminator is satisfied by literature import / large-N saturation".
The PASSes are honest but partially due to the discriminator's
admissive shape.

### §3.3 Net audit conclusion

The §2 cross-tabulation shows perfect concentration along the
predicted axis. The §3.1-§3.2 audit shows that **most of this
concentration is candidate-validity-driven, not discriminator-type-
driven**. Specifically:

- 4 of 5 FAILs are C-driven (would fail under any reasonable
  discriminator);
- 3 of 3 PASSes are S-driven (real structural content), with
  permissive-discriminator flags but no flip-the-verdict failure.

The discriminator-type effect is **real but not the dominant axis**.
The dominant axis is candidate-validity (whether the candidate
actually introduces a new primitive). The discriminator-type axis
correlates with the candidate-validity axis because the L9 catalogue's
generation process (per parent meta-audit CATALOGUE_BIAS verdict)
produced *arithmetic-family* candidates (which naturally call for
discrete-equality discriminators) that systematically lacked
analytic content.

---

## §4 Refined hypothesis verdict

**Verdict**: **CONFOUNDED**.

Justification:

1. The 2×2 contingency is at the extreme of its marginal (Fisher
   exact two-sided p ≈ 0.036) — a strong signal at n=8.
2. However, the §3 alternative-explanation audit shows that the
   correlation between discriminator-type and verdict is **not
   independent of** the correlation between candidate-validity and
   verdict.
3. Specifically, 4/5 FAILs would fail under any reasonable
   discriminator because the candidate has no real new primitive
   (Q1 rank-6, Q5 no-construction, KPZ d=7 no-anchor, P3 lattice-
   above-interval). The 5th (D1.1) is genuinely mixed.
4. All 3 PASSes have S-dominant structural content; the discriminator's
   permissive shape is a *flag* on each PASS but does not produce
   spurious passes — the underlying primitives (κ=6 ≠ Wigner;
   Hirahara non-black-box; 2.5D = 2D-NS + passive scalar) are
   structurally real.
5. Therefore the data are **consistent with both** hypotheses
   (discriminator-type drives outcome **vs** candidate-validity drives
   outcome) and **cannot distinguish** them at n=8 because the L9
   catalogue's generation systematically pairs discrete-equality
   discriminators with weak candidates and distributional/structural
   discriminators with stronger candidates.

The verdict CONFOUNDED is selected because (a) one cross-cell entry
would have *refuted* the hypothesis (none observed → not REFUTED);
(b) MIXED would imply the data sustain the hypothesis as one of
several active axes (the data sustain *some* effect but cannot say
how much is type-driven); (c) CONFIRMED would require the §3 audit
to rule out candidate-validity confound (which it does not).

This is also distinct from the prior `omega-meta-audit-l9-catalogue-
design-2026-04-25.md` verdict (CATALOGUE_BIAS): that audit is on the
**generation** axis (where do candidates come from?); this audit is
on the **discrimination** axis (what scoring rule was applied?).
Both verdicts are non-dispositive at the current sample size and are
mutually compatible — the same correlation structure underwrites
both.

### §4.1 Why not CONFIRMED

Confirmation would require demonstrating that holding candidate-
validity fixed, distributional/structural discriminators
systematically PASS where discrete-equality discriminators
systematically FAIL. The current sample does not have a single pair
of (same candidate, two different discriminators) measurements;
without such pairs, candidate-validity and discriminator-type are not
separable.

A possible CONFIRMED-establishing experiment: take the **same
candidate** (e.g. KdV 6-soliton phase-shift) and apply (a) the
discrete-equality discriminator (det/σ ∈ ℤ, rank=3) → FAIL (already
known), and (b) a distributional discriminator (KS of M's eigenvalue
spectrum vs a reference) → ? . If (b) yielded a robust PASS, the
discriminator-type axis would be isolated. This experiment is **not
performed in this audit** (read-only scope).

### §4.2 Why not REFUTED

A REFUTED verdict would require at least one cross-cell entry: either
a PASS with discrete-equality / numerical-interval / vacuous-magnitude
discriminator, or a FAIL with distributional / structural-literature
discriminator. **No such row exists in the n=8 set**. The hypothesis
as stated is consistent with every observation.

### §4.3 Why not MIXED

MIXED would imply discriminator-type matters but is one of several
active axes. The data are consistent with this reading too, but the
§3 audit has shifted weight away from "discriminator-type matters"
toward "candidate-validity matters and discriminator-type is collinear
with it". The honest characterization is CONFOUNDED, not MIXED.

### §4.4 Why not REFUTED-by-edge-cases

The ACCEPTABLE rows (D2 R1, D2 R5) are not direct molt-validations
and use a separate acceptability criterion grid (Millennium-grade
preservation + partial-result import + F-544-B novelty test). They
are addressed in §6 below; they do not produce cross-cell entries
under the hypothesis as stated, since "axiom-recast" is not in the
hypothesis's PASS-family list and they are not FAILs anyway.

---

## §5 Recommendations

### §5.1 For BT-545 / BT-546 catalogue extension (parallel session)

- **Default to distributional or structural-literature-import
  discriminators** for any new candidate where structural truth is
  expected to be the binding question. These two types have, in the
  n=8 sample, a 3/3 PASS rate (with S-dominant audits) and admit
  honest passes when the candidate is real.
- **Treat discrete-equality and numerical-interval discriminators as
  high-bar, candidate-stress-test discriminators**. They have, in
  the n=8 sample, a 0/5 PASS rate; the parent meta-audit's
  CATALOGUE_BIAS verdict suggests this rate reflects candidate-
  family weakness (n=6-arithmetic candidates) rather than
  discriminator over-strictness. Use them when the goal is to
  *falsify* a candidate quickly, not when the goal is to license one.
- **Vacuous-magnitude discriminators (cap-style) require parameter-
  sensitivity sweeps before being used as primary**. The D1.1 result
  is informative *negatively* (cap is decorative under the seed-
  frozen parameters) but the parameter-sensitivity question
  (cap-non-vacuous in any (R, ν, T) regime?) was deferred. A
  primary-use cap discriminator should bake in such a sweep.

### §5.2 Should the L9 catalogue admit only distributional / structural discriminators?

**No (with caveat)**. Restricting the catalogue to PASS-family
discriminators alone would (a) lose the FAIL-fast diagnostic power
of discrete-equality / numerical-interval discriminators (which
correctly *killed* arithmetic-family candidates on the first probe),
and (b) bias the catalogue toward licenses, since PASS-family
discriminators are by design more permissive (per §3.2). The catalogue
should retain *all* discriminator types but **rank them by candidate-
type-match**:

- arithmetic-family candidates → discrete-equality discriminator
  (will fail fast if the arithmetic doesn't close);
- coupling / distribution candidates → distributional discriminator;
- literature-anchored candidates → structural-literature-import
  discriminator;
- info-theoretic / cap candidates → vacuous-magnitude discriminator
  *with parameter-sensitivity sweep*;
- analytic candidates from open auxiliary problems → axiom-recast
  acceptability check (D2-style, not molt-validation).

This pairing avoids the CONFOUNDED axis: each candidate gets the
discriminator that interrogates its actual structure, and PASS/FAIL
becomes more cleanly attributable to candidate-validity rather than
to discriminator-type-mismatch.

### §5.3 Honest discrete-equality discriminators (if any)

A discrete-equality discriminator is **honest** when:

1. The target equality is derivable from the proposed primitive
   *before* the measurement is run (not retro-fitted from a
   numerical accident);
2. The discriminator's threshold (e.g. "rank = n/φ", "det/σ ∈ ℤ")
   has a non-trivial probability of failing on a generic input from
   the same family — i.e. the test is not tautological;
3. A failure of the equality has a clear interpretation (the
   primitive does not do the work it claims) rather than being
   numerical-noise rejection.

The Q1 KdV-Gram discriminator satisfies (1) and (3): the rank=3 and
det/σ ∈ ℤ targets are derivable from the n=6 lattice ahead of the
measurement, and the failure cleanly says "the KdV phase-shift Gram
does not have the predicted lattice structure". It also satisfies
(2): generic 6×6 symmetric real matrices have rank 6 and irrational
determinant. **Q1 is therefore an honest discrete-equality
discriminator** — it just happens to fire FAIL on a candidate with
no underlying primitive.

A **dishonest** discrete-equality discriminator would be one where
the equality is an arithmetic accident (e.g. "σ³ · sopfr = 8640
matches the lattice composite Π_NS = 8640") that has no derivational
status. The Q5 attempt-A "5/12 = sopfr/σ as Hölder exponent" is
close to this failure mode (5/12 has no PDE derivation; the equality
is post-hoc).

### §5.4 For future molt-validations

- **Pair candidates with native discriminators** (per §5.2). Avoid
  applying a discrete-equality discriminator to a candidate whose
  natural test is distributional, and vice versa.
- **Run cross-discriminator cross-checks on borderline cases**. If a
  candidate FAILs under one discriminator type, optionally re-test
  under a second type *of the same logical content* (not a relaxation
  of the threshold) before declaring the molt definitively dead.
  Example: Q1's spectrum could be re-tested as a KS-vs-reference
  distributional measurement, and if *that* also fails, the FAIL is
  doubly licensed and the discriminator-type confound is reduced.
- **Document discriminator-type explicitly** in each molt-validation
  report's front-matter, using the controlled vocabulary
  {distributional, structural-literature, sketch, discrete-equality,
  numerical-interval, vacuous-magnitude, axiom-recast, OTHER}. This
  enables future meta-audits at lower cost.

### §5.5 Resampling target for re-running this audit

The CONFOUNDED verdict is a function of n=8 plus the systematic
generation bias documented in the parent CATALOGUE_BIAS audit. A
resampling at n ≥ 16 *with the §5.2 pairing applied* would
re-test the hypothesis with the candidate-validity confound
reduced (each candidate gets its native discriminator, so type
no longer correlates with candidate weakness as strongly). If the
hypothesis is **truly** discriminator-type-driven, the next 8 rows
should produce more cross-cell entries (since native pairings
should equalize PASS rates across types). If **truly** candidate-
validity-driven, native pairings should still produce a strong
type-vs-verdict correlation but with *fewer* trivial FAILs.

---

## §6 Edge cases: ACCEPTABLE verdicts (D2 R1 and R5)

The ACCEPTABLE verdicts in `omega-exec-bt544-d2-r1r5-acceptability-
2026-04-25.md` are **not direct molt-validations**. They use a
separate criterion grid:

1. Millennium-grade preservation (does the recast trivialize or
   over-shoot?);
2. Partial-result import density (does the recast admit existing
   results unavailable to Clay-form NS?);
3. F-544-B novelty test (is the new invariant a relabeling?).

This is an **L_ω axiom-recast acceptability** check, not an L9
molt-validation. The discriminator-type label "axiom-recast" was
added to the controlled vocabulary precisely to mark this distinction.

How they fit the bias hypothesis:

- They are not PASS-or-FAIL on a frame-shift; they are
  ACCEPTABLE-or-not on an auxiliary research problem.
- Both R1 (Onsager-Hölder) and R5 (low-Mach) PASSed all three
  acceptability tests.
- The discriminator type ("axiom-recast") is closest in spirit to
  *structural-literature-import* — both rely on a chain of published
  partial results to license the claim. R1 cites Onsager 1949 →
  Constantin-E-Titi 1994 → Isett 2018 → Buckmaster-Vicol 2019; R5
  cites Klainerman-Majda 1982 → Schochet 1986 → Lions-Masmoudi 1998
  → Feireisl-Novotný 2009.
- If forced into the 2×2, both rows would land in the
  "distrib/struct-literature" row (PASS-family) and both would be
  ACCEPTABLE → the hypothesis's PASS-family prediction is
  **consistent with** but **not strictly tested by** the ACCEPTABLE
  rows (since ACCEPTABLE is not the same predicate as PASS).
- They are **not** cross-cell entries under any reading of the
  hypothesis.

**Caveat**: the acceptability check is at the L_ω apex (axiom-recast
of A2 / A6), not at the L9 molt level. ACCEPTABLE here means "this
is a worthwhile auxiliary research problem", not "this is a real
molt within the n=6 frame". The discriminator-type bias hypothesis
was stated for L9 molt-validations; extending it to L_ω
acceptability checks is not in-scope and would require a separate
hypothesis grid.

For completeness, if the hypothesis is informally re-stated as
"PASS-family ≈ distributional / structural-literature / axiom-recast
{which all share a partial-result-import character}" and "FAIL-family
≈ discrete-equality / numerical-interval / vacuous-magnitude {which
all share a binary-threshold character}", then the n=10 rendition
becomes:

|                                                                                          | PASS-equivalent | FAIL-equivalent |
|------------------------------------------------------------------------------------------|-----------------|-----------------|
| Distributional / structural-literature / axiom-recast (partial-result-import character)  | 5               | 0               |
| Discrete-equality / numerical-interval / vacuous-magnitude (binary-threshold character)  | 0               | 5               |

This 2×2 is even cleaner (Fisher exact two-sided p =
2·C(5,5)·C(5,0)/C(10,5) = 2/252 ≈ 0.0079), but the same CONFOUNDED
verdict applies for the same reasons (§3 audit unchanged: the
ACCEPTABLE rows reflect S-dominant structural truth, not permissive
discrimination).

---

## §7 Anti-list — hypothesis variants considered and rejected

Variants of the bias hypothesis that *could* have been tested
against the n=8/n=10 data but are not selected as the operative
formulation, with reasons.

| Variant                                                                                                                                                | Why rejected                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
|--------------------------------------------------------------------------------------------------------------------------------------------------------|-------|
| V1: "PASS family = anything continuous; FAIL family = anything discrete"                                                                               | Too coarse. KS test (PASS, BT-541) is on a continuous distribution, but the *test statistic* is a number; the discriminator is structural about the distribution, not "continuous in form". The user's vocabulary is more precise.                                                                                                                                                                                                                                                                                  |
| V2: "PASS family = high-N statistical; FAIL family = low-N algebraic"                                                                                  | Refuted in spirit by row 7 (Hirahara is N=0 / no statistics; PASS via literature). The "high-N" framing misses the structural-literature-import path.                                                                                                                                                                                                                                                                                                                                          |
| V3: "PASS family = literature-attested; FAIL family = repo-derivable"                                                                                  | Partially right but wrong polarity for BT-541 Lead-B (repo-derivable PASS via KS). The "literature-attested" sub-axis is one of two PASS-family ingredients, not the only one.                                                                                                                                                                                                                                                                                                                  |
| V4: "PASS family = expensive to compute; FAIL family = cheap to compute"                                                                               | Disconfirmed by row 7 (Hirahara ≈ 5 minutes of literature scan; PASS) and row 1 (KdV Gram ≈ 0.27 s; FAIL). Cost is uncorrelated with verdict.                                                                                                                                                                                                                                                                                                                                                  |
| V5: "PASS family = candidates with prior atlas grade ≥ [10]; FAIL family = candidates without"                                                          | Not testable from the data extracted; would require atlas-grade lookup per candidate. Possibly a useful sub-audit but orthogonal to discriminator-type.                                                                                                                                                                                                                                                                                                                                                |
| V6: "PASS family = NON-arithmetic discriminators; FAIL family = arithmetic-family discriminators"                                                       | Equivalent in n=8 outcome to the operative hypothesis (since {distributional, structural-literature} happen to be the non-arithmetic discriminators in the sample). However, the user's vocabulary is more precise (discriminator-type, not arithmetic/non-arithmetic), so the operative form is preferred. The CONFOUNDED verdict applies identically.                                                                                                                                                  |
| V7: "PASS family = those that PASSed; FAIL family = those that FAILed (tautology check)"                                                                | Tautological; the §3 alternative-explanation audit is exactly the test that distinguishes the operative hypothesis from this tautology.                                                                                                                                                                                                                                                                                                                                                          |

---

## §8 Falsifiers active for this audit

Audit-level falsifiers — conditions under which **this very meta-
audit's CONFOUNDED verdict** would be retracted.

- **F-AUDIT-D1** (label-fabrication): if any of the discriminator-type
  labels in §1 is shown to mis-classify the parent report's actual
  discriminator (e.g. if BT-541 Lead-B's KS test is re-classified as
  numerical-interval rather than distributional), the §2 contingency
  shifts. The labels in §1 are sourced from the parent reports'
  explicit framing ("KS p<0.01" → distributional; "rank=3 AND det/σ ∈
  ℤ" → discrete-equality; "Φ_holo < cap" → vacuous-magnitude;
  "literature-import" → structural-literature; "ratio ∈ [2.5, 3.5]"
  → numerical-interval; "axiom recast" → axiom-recast). Risk: low
  for rows 1, 5, 6, 7, 9, 10 (label is verbatim from parent); medium
  for rows 2, 3, 4, 8 (label is interpretation of the parent's
  failure-mode language). **Not active** under current readings.
- **F-AUDIT-D2** (verdict-list-violation): the verdict must be from
  {CONFIRMED, CONFOUNDED, REFUTED, MIXED}. CONFOUNDED is on the list.
  **Not active**.
- **F-AUDIT-D3** (cell-count-fabrication): the 2×2 cell counts in
  §2 must be derivable from the §1 row labels alone. Re-derivation
  per §2 yields exactly (3, 0; 0, 5). **Not active**.
- **F-AUDIT-D4** (catalogue-edit-side-effect): if executing this
  audit is shown to have edited any L9 catalogue file, atlas, state,
  or theory/canon, the read-only scope is violated. This audit is
  read-only and produces only this one new file. **Not active**.
- **F-AUDIT-D5** (ACCEPTABLE-mis-counted-as-PASS): the hypothesis as
  stated does not name "axiom-recast / ACCEPTABLE" in the PASS family.
  If a future reading reclassifies ACCEPTABLE as PASS (e.g. for
  consistency), the §6 edge-case treatment changes. The §2 main
  contingency excludes ACCEPTABLE rows precisely to avoid this risk.
  **Not active**.
- **F-AUDIT-D6** (CONFOUNDED-uniqueness): if a future audit
  demonstrates that one of the FAIL rows is **purely** discriminator-
  driven (e.g. a parameter-sensitivity sweep on D1.1 finds
  cap-non-vacuous in some (R, ν, T) regime, flipping D1.1 from FAIL
  to PASS), the audit's CONFOUNDED verdict shifts toward MIXED or
  CONFIRMED. The current §3 audit places D1.1 as **mixed-tilting-C**
  but allows for D component. The follow-up sweep is registered as
  open-task in §7.2 of the D1.1 report. **Not active in this audit
  but conditional on the parameter sweep.**
- **F-AUDIT-D7** (n-too-small): n=8 (or n=10 with edge-case
  inclusion) gives Fisher exact p ≈ 0.036 (or 0.008). At this n the
  CONFOUNDED verdict is the *honest* characterization. If a resampling
  at n ≥ 16 produces zero cross-cell entries with native pairings
  applied (per §5.5), the verdict should re-shift toward CONFIRMED.
  **Not active in this audit; conditional on resampling.**
- **F-AUDIT-D8** (parent-CATALOGUE_BIAS-confound): the parent meta-
  audit identified CATALOGUE_BIAS (generation-axis bias toward n=6-
  arithmetic candidates). If that bias is in fact the only operative
  axis, the discriminator-type signal here is fully derivative. The
  §3 audit makes this exact point; CONFOUNDED is the honest verdict
  *because* the two axes cannot be separated in the current data.
  **Not active as a falsifier; recorded as the structural reason for
  CONFOUNDED.**

None of F-AUDIT-D1..D8 fires under the current evidence. The
CONFOUNDED verdict is robust to the active falsifiers.

---

## §9 Closing

**Verdict**: CONFOUNDED. The discriminator-type bias hypothesis is
**consistent with** the n=8 molt-validation data (Fisher exact two-
sided p ≈ 0.036; no cross-cell entries) but is **not separable** from
the underlying candidate-validity confound (4 of 5 FAILs are
candidate-validity-driven; 3 of 3 PASSes are S-dominant with
permissiveness flags). At n=8, with the L9 catalogue's known
generation bias toward n=6-arithmetic candidates (CATALOGUE_BIAS,
parent meta-audit), the discriminator-type axis is collinear with
the candidate-family axis; the data cannot say whether the type axis
is binding.

**Recommendations (§5)**:
1. For BT-545 / BT-546 catalogue extension: **default to native
   discriminator pairing** per candidate type (§5.2), not to
   discriminator-type restriction.
2. **Do not** restrict the L9 catalogue to PASS-family discriminators
   alone (§5.2): the FAIL-family discriminators correctly killed the
   weak arithmetic-family candidates on first probe — that diagnostic
   power is real and useful.
3. **Discrete-equality discriminators are honest** when the target
   equality has derivational backing and the test is non-tautological
   (§5.3); Q1 satisfies these criteria even though it FAILed.
4. **Resample at n ≥ 16 with native pairings applied** (§5.5) to
   separate discriminator-type from candidate-validity. If the
   hypothesis is truly type-driven, native pairings should produce
   cross-cell entries; if truly candidate-validity-driven, native
   pairings should produce a similar (or even stronger) type-vs-
   verdict correlation but with fewer trivial FAILs.

**Edge cases (§6)**: ACCEPTABLE verdicts (D2 R1, R5) sit on a
separate L_ω acceptability grid, are not direct molt-validations,
and are **consistent with but not strictly testing** the hypothesis.
They produce no cross-cell entries.

**Atlas / state / inventory**: untouched. **Millennium tally**:
**0/7 unchanged**. No atlas grade modified, no canon edit, no
inventory change.

**No new theorem claimed.** This is a diagnostic on the
discriminator-type axis of the L9 calibration sweep + D1/D2/D3
extensions, scored against a fixed-vocabulary hypothesis using the
audit-level criteria the user supplied.

— end meta-audit —
