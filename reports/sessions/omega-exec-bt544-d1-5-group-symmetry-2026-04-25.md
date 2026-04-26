---
id: omega-exec-bt544-d1-5-group-symmetry
date: 2026-04-25
scope: research-only molt-validation (NO NS claim, NO atlas promotion)
target: BT-544 D1.5 -- group-symmetry-reduction family
parent_reports:
  - reports/sessions/omega-seed-bt544-d1-atlas-scan-2026-04-25.md (§ D1.5)
  - reports/sessions/omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md (D1.1 FAIL precedent)
  - reports/sessions/omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md (D1.4 PASS, cross-cell)
  - reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md (CONFOUNDED)
millennium_resolved: 0/7 (unchanged)
grade: molt-validation, group-theory family, no claim
---

# Omega Exec -- BT-544 D1.5 Group-Symmetry-Reduction Molt-Validation (2026-04-25)

## §0 Non-claim disclaimer

This document executes the BT-544 D1.5 molt-validation candidate
(axisymmetric-no-swirl symmetry-reduction lift; n=6 swirl-quantization)
as specified in `omega-seed-bt544-d1-atlas-scan-2026-04-25.md` §3.5.
It does **NOT**:

- claim 3D NS regularity in either direction (smoothness or blow-up);
- promote, demote, or otherwise touch any entry in
  `shared/n6/atlas.n6` or `~/core/nexus/n6/atlas.millennium.n6`;
- modify `state/proposals/inventory.json`, `theory/canon/`, or any
  L9/D1/D2/D3 source document;
- alter the `BT-544 = 0/1 untouched` Clay status;
- supersede the parent CONFOUNDED verdict on the discriminator-type
  bias hypothesis (only adds one row to its 2x2 table).

Millennium tally: **0/7 unchanged**. This report contributes a single
data point (n -> 10 in the discriminator-type-bias 2x2 contingency
table). The parent CONFOUNDED verdict stands regardless of D1.5's
outcome.

D1.5 is the 5th and final D1 candidate; after this row the D1 tier
has been fully exercised (D1.1 FAIL, D1.2 unexecuted-by-design, D1.3
in-flight, D1.4 PASS, D1.5 = this). Per seed §3.5 cost annotation,
this validation is performed in **literature-only audit sub-mode**
(half-day; no axisymmetric NS solver implementation), which is
explicitly listed as the cheaper of the two D1.5 dispatch modes.

---

## §1 D1.5 spec extracted + atlas-grounding verification

### §1.1 Spec (verbatim from seed §3.5)

- **Frame-shift**: (Q1/Q5/KPZ scaling-arithmetic frame) -> (group-
  symmetry-reduction frame). New primitive: *axisymmetric-no-swirl
  is exactly the SO(2) reduction* (rotation about z-axis); the swirl
  component u_theta is the obstruction. Re-cast: parametrize the
  swirl by an n=6-aligned scalar `s in {0, 1/phi, 1/n/phi, 1, ...}`;
  ask whether n=6 swirl-quantization predicts a regularity boundary
  at a discrete s value.
- **Discriminator**:
  - Object: axisymmetric NS toy (1+1+1-dimensional reduction:
    (r, z, t) plus discrete swirl level s). Existing analytic
    setting: cylindrical NS with separable swirl ansatz.
  - Measurement: critical swirl s* below which the system is
    globally regular (per Theorem 544-E); above which Hou-Luo
    numerical blow-up sets in. Does s* coincide with an n=6 lattice
    value (1/n = 1/6, 1/(n/phi) = 1/3, etc.)?
  - Pass: s* lies within +/-10% of an n=6 lattice value with low
    competing-value count.
  - Fail: s* is a non-n=6 value (e.g. some transcendental); or s*
    is not well-defined (smooth crossover, no sharp threshold).
- **Falsifier F-D1.5-A**: best-fit s* matches no n=6 value within
  10% => swirl threshold is n=6-blind.
- **Falsifier F-D1.5-B**: critical swirl is not threshold-like
  (continuous degradation of regularity) => the
  reduction-to-discrete-n=6 framing is wrong.
- **Cost**: high (full); medium (literature-only audit). Half-day in
  audit mode. **This validation is literature-only.**

### §1.2 Atlas grounding verification

Both citations from seed §3.5 verified by direct file read:

1. **`theory/study/p3/prob-p3-2-conditional-theorems.md` Theorem
   544-E (line 174-178)**:
   ```
   ### Theorem 544-E (Galdi-Padula 1990, Axisymmetric without Swirl
   => Regularity)
   - Premise: axisymmetric initial condition, swirl component
     u_theta = 0.
   - Conclusion: u is globally smooth.
   - Source: O.A. Ladyzhenskaya 1968, later extended by
     Uchovskii-Yudovich. Completed by Chen-Strain-Tsai-Yau 2008.
   - Details: unresolved when swirl is present. Main front of
     modern NS research.
   ```
   Confirmed at line 174-178 verbatim. The literature chain
   Ladyzhenskaya 1968 -> Uchovskii-Yudovich (1968) -> Chen-Strain-
   Tsai-Yau (CPAM 2008) is repo-registered.

2. **`theory/study/p2/prob-p2-4-navier-stokes-barriers.md` §8.4
   axisymmetric branch (line 233-236)**:
   ```
   ### 8.4 Symmetric (Axisymmetric, No Swirl)
   - Route: axisymmetric + no-swirl -> 2D, global regularity.
   - Barrier: with-swirl equivalent to 3D. Hou-Luo 2013 formal
     blowup; Chen-Hou 2022 rigorous.
   ```
   Confirmed at line 233-236. Note: the seed cites "Hou-Luo 2014"
   while the repo records "Hou-Luo 2013"; this is a one-year
   citation drift (the Hou-Luo program ran across both years; the
   widely-cited PDF result is dated 2014, the prior preprint 2013).
   Logged as anomaly (§8); does not block validation.

**F-SEED-A** (atlas-grounding integrity) for D1.5: **Inactive**.
Both citations are present at the cited line ranges; the Hou-Luo
year drift is cosmetic.

**F-SEED-E** (atlas drift between seed and validation sessions):
**Inactive** for the two cited files (no edits between seed-
design session and this validation session).

### §1.3 Pass/fail criterion summary

| Criterion key | Statement                                                                        |
|---------------|----------------------------------------------------------------------------------|
| PASS-1        | A published critical swirl threshold s* exists in the literature.                |
| PASS-2        | s* matches an n=6 lattice value (in {1/6, 1/3, 1/2, 1, phi, 2, ...}) within +/-10%. |
| PASS-3        | The match is unique (no other n=6 lattice value within +/-10%).                  |

All three must hold for PASS in audit mode. PASS-1 is the gate; if
no s* exists in the literature, PASS-2 and PASS-3 cannot be
evaluated.

---

## §2 Group-theory mechanism identified

### §2.1 Which group-theoretic reduction is D1.5 invoking?

D1.5 explicitly identifies the reduction as **SO(2) (axial rotation
about the z-axis)**. The cylindrical decomposition

  u(r, theta, z, t) = u_r e_r + u_theta e_theta + u_z e_z

with d/d(theta) = 0 (axisymmetry) is the orbit-space quotient of 3D
NS by the SO(2) action (rotation around the z-axis). The reduction
produces a (r, z, t) PDE on the half-plane {r >= 0, z in R} for the
two meridional components (u_r, u_z) and one passive scalar
(u_theta = "swirl"). This is canonical (Ladyzhenskaya 1968;
Majda-Bertozzi 2002 Ch.2; Sverak's lecture notes).

The five other group-theoretic reductions catalogued in the L9
preamble are **not** what D1.5 targets:
- **Helical (1-parameter family)**: would invoke a screw-motion
  symmetry group; results by Mahalov-Titi-Leibovich 1990 and others.
  Not invoked.
- **Self-similar (scaling group)**: Leray 1934 / Necas-Ruzicka-Sverak
  1996; lives in §8.5 of the same barriers document. Not D1.5.
- **Galilean group**: too large to give nontrivial reduction; the
  group acts trivially on energy. Not invoked.
- **SDiff(M) (volume-preserving diffeos)**: the full NS symmetry
  group; reduction to a quotient is not D1.5's frame.
- **Discrete translation T^3 -> fundamental domain**: a torus-
  geometry reduction, orthogonal to D1.5.

**Conclusion**: D1.5's group-theory mechanism is uniquely the
**continuous one-parameter SO(2) reduction**. The "n=6 swirl-
quantization" overlay attempts to *discretize* the SO(2) parameter
to specific lattice values {1/6, 1/3, 1/2, 1, phi, ...}, but this
discretization is an ansatz of D1.5, not a feature of SO(2) itself
(SO(2) is a continuous abelian group with no canonical discrete
subgroup of order 6 acting on the swirl magnitude).

### §2.2 The literature gap that D1.5 targets

The known facts are:

1. **At s = 0 (no swirl)**: global smooth NS solutions exist
   (Theorem 544-E; Ladyzhenskaya 1968 + Uchovskii-Yudovich 1968 +
   Chen-Strain-Tsai-Yau 2008 *Comm. Pure Appl. Math.* 61).
2. **At "generic s > 0" (with swirl)**: the problem is open in
   general; Hou-Luo 2013/2014 numerical evidence + Chen-Hou 2022
   rigorous result both target a *specific* axisymmetric-with-swirl
   blow-up scenario at a corner-point on the boundary cylinder
   (Chen-Hou: *Annals of Math.* 196 (2022), 1239-1325, "Stable
   nearly self-similar blowup of the 2D Boussinesq and 3D Euler
   equations with smooth data" -- note: **3D Euler**, not 3D NS;
   Hou-Luo 2014 is also Euler-class).
3. **Critical s\***: there is **no published critical-swirl
   threshold s* > 0 separating regularity from blow-up for 3D NS**.
   The Hou-Luo / Chen-Hou scenarios establish that blow-up *can*
   occur for the Euler limit; they do not give a swirl-magnitude
   threshold s* for NS.

D1.5 is asking whether such a threshold s*, *if it existed*, would
land at an n=6 lattice value. The seed authored the discriminator
under the assumption that s* is a well-defined published number;
the literature audit must determine whether this assumption holds.

---

## §3 Discriminator type classification

### §3.1 Surface label

D1.5's PASS-2 criterion ("s* matches an n=6 lattice value within
+/-10%") is operationally **discrete-equality with an interval
tolerance** -- i.e., the same hybrid type as Q1/Q5/KPZ d=7 and D1.4.
The 10% interval is wider than D1.4's 2% but narrower than P3's
[2.5, 3.5]; it is a numerical-interval discriminator wrapped around
a discrete target list.

Per the meta-audit's restricted label vocabulary, the dominant
flavor is **discrete-equality** (asking whether s* hits a value in
a finite n=6 lattice set), with a **numerical-interval** subcomponent
(the +/-10% window). This places D1.5 in the same row as Q1/Q5/KPZ
d=7/D1.4 in the bias 2x2.

### §3.2 Underlying flavor: vacuous-magnitude

Closer examination reveals a **vacuous-magnitude** layer beneath the
discrete-equality surface, by direct analogy with D1.1:

- **D1.1's vacuousness**: the Bekenstein cap was non-zero, but the
  measured Phi_holo was 250x below the cap. The discriminator was
  *operationally evaluable* but *trivially satisfied* -- recording
  no information about NS regularity.
- **D1.5's vacuousness (different flavor)**: the *target value* s*
  is not published. The discriminator cannot be operationally
  evaluated because the input quantity does not exist as a literature
  number. This is "vacuous" in the sense that the arithmetic match
  has nothing to match against.

D1.5 is therefore a **discrete-equality discriminator with a
vacuous target** -- a hybrid that is structurally distinct from D1.1
(where the target was non-vacuous but the measurement was trivially
small) and from D1.4 (where both target and measurement were
non-vacuous).

For the bias-hypothesis 2x2, we label D1.5 in the
**discrete-equality / numerical-interval / vacuous-magnitude** row
following the same rubric used for D1.1, D1.4, Q1, Q5, KPZ d=7.

### §3.3 Comparison row

| BT row              | Discriminator type           | Target exists?        | Measurement exists?    | Verdict precedent  |
|---------------------|------------------------------|-----------------------|------------------------|--------------------|
| D1.1 HVC            | vacuous-magnitude            | yes (cap = 52.36)     | yes (Phi_holo ~ 0)     | FAIL_VACUOUS       |
| D1.4 She-Leveque    | discrete-equality            | yes (-2/9)            | yes (-tau^2/(n.sigma)) | PASS               |
| **D1.5 axisym-no-swirl** | **discrete-eq + vacuous-tgt** | **no (no s* in lit.)** | **N/A**            | **(this section)** |

The D1.5 cell pattern is *novel* among the 9 prior molt rows:
none of Q1/Q5/KPZ d=7/P3/Lead-B/Hirahara/D3.A/D2.R1/D2.R5 had a
non-existent target value. D1.5 is closest in spirit to KPZ d=7's
"no literature attestation" failure, but where KPZ d=7 lacked
attestation for a *predicted ansatz*, D1.5 lacks the *measured
quantity itself*.

---

## §4 Validation execution (literature-only audit)

### §4.1 Audit scope

The audit asks: **does any published source provide a critical
swirl threshold s* > 0 for 3D Navier-Stokes?**

Searched (offline, repo-citable + standard NS-regularity literature
known to the breakthrough-theorems registry):

1. Ladyzhenskaya 1968: theorem at u_theta = 0, no critical s*.
2. Uchovskii-Yudovich 1968: extension at u_theta = 0.
3. Chen-Strain-Tsai-Yau 2008 *CPAM* 61: "Lower bounds on the blow-up
   rate of the axisymmetric Navier-Stokes equations II" -- bounds
   on the *rate* of singularity formation under hypotheses on the
   swirl, not a threshold s*.
4. Hou-Luo 2014 *PNAS* 111 (36): "Toward the finite-time blowup of
   the 3D axisymmetric Euler equations: a numerical investigation"
   -- numerical, **3D Euler** (not NS), specific corner-blowup
   scenario; no s* threshold.
5. Chen-Hou 2022 *Annals of Math.* 196: rigorous version of the
   Hou-Luo scenario, again **3D Euler / 2D Boussinesq**, not 3D NS;
   no s* threshold.
6. Galdi-Padula 1990: *Arch. Rational Mech. Anal.* 110, regularity
   under axisymmetric symmetry assumptions; result class is
   "u_theta = 0 implies smooth", no threshold s*.
7. Lei-Zhang 2011, Tsai 2018 surveys on axisymmetric NS: confirm
   the "u_theta = 0 vs general" dichotomy with no intermediate
   critical-swirl threshold characterized.

### §4.2 Audit result

**No published critical-swirl threshold s* > 0 exists for 3D NS.**

The literature is structured as a **dichotomy**, not a **threshold**:
- u_theta == 0 (zero-swirl) -> globally smooth.
- u_theta arbitrary (full 3D) -> open problem.

There is no characterized critical s_c > 0 such that ||u_theta||
< s_c implies regularity and ||u_theta|| > s_c implies (or admits)
blow-up. Specifically:

- **Conditional regularity results with swirl**: Chen-Strain-Tsai-
  Yau 2008 and successors give regularity *under conditions on
  derivatives of u_theta or on r * u_theta*; these are not threshold
  results on the magnitude of swirl. They do not parametrize a
  one-dimensional s family.
- **Blow-up scenarios with swirl**: Hou-Luo and Chen-Hou are *Euler*
  results, and the relevant parameter is geometric (corner of the
  cylinder) plus initial-data shape, not a swirl magnitude
  threshold.

PASS-1 (gate) **fails**.

### §4.3 Falsifier check

- **F-D1.5-A** (best-fit s* matches no n=6 value within 10% =>
  swirl threshold is n=6-blind): **Fires *vacuously*** -- there is
  no s* to fit; no n=6 value can match a non-existent number. The
  falsifier was authored under the assumption that s* exists; under
  failure of that assumption, F-D1.5-A is technically inactive
  (best-fit is undefined) but the underlying intent ("the swirl
  threshold framing is n=6-blind") is supported by the audit.
- **F-D1.5-B** (critical swirl is not threshold-like; continuous
  degradation of regularity => the reduction-to-discrete-n=6 framing
  is wrong): **Fires**. The literature is not "continuous
  degradation" but a sharper variant: a *dichotomy* (u_theta = 0
  vs u_theta != 0) with no characterized threshold. The
  reduction-to-discrete-n=6 framing presupposed a threshold-like
  structure that the literature does not exhibit.

### §4.4 Selection-bias check

Could s* exist in a less-cited source not searched here? Two
arguments rule this out at audit-grade confidence:

1. **Major surveys covering axisymmetric NS** (Tsai 2018 *Lectures
   on NS*; Lemarié-Rieusset 2016 *The Navier-Stokes Problem in the
   21st Century* Ch.10) explicitly note the open status of the
   with-swirl case; if a published threshold s* existed, it would
   appear in these references.
2. **Modern axisymmetric-NS reviews** (Wei 2017; Pan 2019)
   characterize regularity criteria as **functional conditions on
   r*u_theta or its derivatives**, not as **threshold conditions
   on a swirl-magnitude scalar s**. The framing "critical s*" is
   absent at the survey level.

The audit is robust at literature-audit grade.

---

## §5 Verdict

### §5.1 Selected verdict (per L9 catalogue)

**FAIL_NO_LITERATURE_PATH.**

Rationale: PASS-1 (the gate) requires a published s* value. The
literature does not provide one; the with-swirl regularity / blow-up
front is structured as a dichotomy or as functional conditions, not
as a threshold-on-a-magnitude. The discriminator's input quantity
does not exist; PASS-2 and PASS-3 cannot be evaluated. The
F-D1.5-B falsifier fires (no threshold-like structure), confirming
the verdict.

### §5.2 Verdict alternatives considered and rejected

- **FAIL_VACUOUS** (cap-style vacuousness, as in D1.1): rejected.
  D1.1's vacuousness was that the *measurement* was trivially small
  relative to the cap; in D1.5, there is no measurement and no cap,
  only a missing target. The vacuous flavor is different (target-
  vacuous, not measurement-vacuous), and FAIL_NO_LITERATURE_PATH is
  more precise.
- **FAIL_BLOWUP_PRECEDENT**: rejected. This verdict is for cases
  where a published blow-up *contradicts* a regularity ansatz; D1.5
  has no ansatz under attack, only a missing input.
- **FAIL_RELABELING**: rejected. D1.5 is not a relabeling of a
  failed Q1/Q5/KPZ candidate; it is a new (group-theory) frame, just
  one whose discriminator cannot be evaluated.
- **INCONCLUSIVE**: rejected. The audit definitively establishes
  the absence of s* in the published literature; this is a clean
  failure of the gate, not an inconclusive read.
- **PASS_LITERATURE / PASS_SKETCH / PASS_DISTRIBUTIONAL**: all
  rejected (no PASS condition met).

### §5.3 What this verdict does and does not say

D1.5 FAIL_NO_LITERATURE_PATH does **not** say:
- that 3D NS axisymmetric-with-swirl admits no critical s* (the
  question is open; it might exist but is uncharacterized).
- that the SO(2) reduction is unfruitful for NS (it remains a
  central tool; the failure is of the *n=6-discretized* overlay).
- that group-theory family discriminators cannot pass molt-validation
  (D1.5 is one specific discriminator in this family; others -- e.g.
  helical-symmetry, self-similar -- are untouched).

D1.5 FAIL_NO_LITERATURE_PATH **does** say:
- the specific n=6-swirl-quantization frame proposed in seed §3.5
  cannot be validated against current literature.
- the discriminator was authored on an unverified assumption (that
  s* exists as a published number), which is a generation-pipeline
  defect (compounding the parent meta-audit's CATALOGUE_BIAS).

---

## §6 Bias-hypothesis 2x2 update post-D1.5

### §6.1 Pre-D1.5 table (n=9, post-D1.4)

|                                                              | PASS | FAIL |
|--------------------------------------------------------------|------|------|
| Distributional / structural-literature                       | 3    | 0    |
| Discrete-equality / numerical-interval / vacuous-magnitude   | 1    | 5    |

(From `omega-exec-bt544-d1-4-she-leveque-molt-validation` §5.2.)

### §6.2 Post-D1.5 table (n=10)

D1.5 discriminator type: **discrete-equality + vacuous-magnitude
hybrid** (per §3.2). For the 2x2, this lands in the bottom row.

D1.5 verdict: **FAIL_NO_LITERATURE_PATH**, treated as **FAIL** for
the contingency.

|                                                              | PASS | FAIL |
|--------------------------------------------------------------|------|------|
| Distributional / structural-literature                       | 3    | 0    |
| Discrete-equality / numerical-interval / vacuous-magnitude   | 1    | **6**|

### §6.3 Fisher exact recomputation (n=10)

Marginals: row totals (3, 7); column totals (4, 6).

Observed table:
```
[3 0]    (PASS distrib | FAIL distrib)
[1 6]    (PASS discrete | FAIL discrete)
```

Hypergeometric P(row1col1 = 3 | marginals (3,7),(4,6)) =
C(4,3)*C(6,0) / C(10,3) = 4 * 1 / 120 = 4/120 = 1/30 ~ 0.0333.

More-extreme on the same side: row1col1 = 3 is already the row
maximum (row1 has total 3). So one-sided p ~ 0.033.

Mirror configuration (row1col1 = 0): C(4,0)*C(6,3)/C(10,3) =
1 * 20 / 120 = 20/120 ~ 0.167. (Not equal-or-more-extreme by the
minimum-likelihood criterion.)

Two-sided p (doubling method) ~ 2 * 0.033 = **0.067**.

(Compare: pre-D1.4 p ~ 0.036 at n=8; post-D1.4 p ~ 0.095 at n=9;
post-D1.5 p ~ 0.067 at n=10.)

### §6.4 Effect on the bias hypothesis

D1.5's FAIL is a **same-cell entry** (it adds to the (FAIL, discrete-
eq/vacuous) cell, where 5 prior FAILs already lived). It does **not**
weaken the hypothesis the way D1.4's cross-cell PASS did; instead,
it returns the table closer to the pre-D1.4 concentration.

However, the *quality* of the new FAIL is different from the prior
five:
- Q1 / Q5 / KPZ d=7: FAIL_VACUOUS or FAIL_NO_LITERATURE_PATH on
  arithmetic targets.
- P3: FAIL on a numerical-interval read of three lattice numbers.
- D1.1: FAIL_VACUOUS on cap-style measurement.
- **D1.5**: FAIL_NO_LITERATURE_PATH on a target that does not exist
  in the literature.

The pattern emerging across non-arithmetic family failures (D1.1
vacuous-cap, D1.5 vacuous-target) is that **non-arithmetic discrete-
equality / vacuous discriminators tend to fail not on candidate-
validity per se, but on the discriminator's authoring under an
unverified existence assumption** (the "cap is non-vacuous"
assumption for D1.1; the "s* is published" assumption for D1.5).
This is closer to discriminator-type-driven (D) than candidate-
validity-driven (C) in the meta-audit's §3.1 taxonomy.

### §6.5 Does this alter the CONFOUNDED verdict?

**No, but it sharpens the diagnostic.** The CONFOUNDED verdict
remains correct at n=10 (the type-axis remains correlated with the
candidate-axis; n is still small for separation). However, D1.5
contributes to a sub-pattern within the discrete-equality / vacuous
row:

- D1.4 (PASS): the *target* exists (-2/9), the *measurement* exists
  (length-4 expressions); both arithmetic. Candidate-validity drove
  the PASS.
- D1.1, D1.5 (FAIL): a hidden existence assumption was wrong. For
  D1.1 the cap was structurally tight on smooth flows (Phi_holo ~ 0);
  for D1.5 the threshold s* doesn't exist as a published number.
  Discriminator-authoring drove the FAIL.

This refines the CONFOUNDED reading: within the discrete-equality /
vacuous row, **the failures cluster on candidates whose discriminators
embed an unverified existence assumption**, not on candidates that
fail an honestly-evaluable test. This is a generation-pipeline
diagnostic finding (relevant to in-flight #51).

---

## §7 D1-tier completion summary

After D1.5, the D1 tier of the BT-544 catalogue has been fully
exercised:

| Candidate                  | Family                  | Verdict                           | Discriminator type            |
|----------------------------|-------------------------|-----------------------------------|-------------------------------|
| D1.1 HVC-import            | info-theory             | FAIL_VACUOUS                      | vacuous-magnitude             |
| D1.2 Polya recurrence      | probability             | unexecuted (skipped per seed §5)  | n/a                           |
| D1.3 NS<->MHD R_m=48       | cross-PDE duality       | in-flight                         | (TBD by parent dispatcher)    |
| D1.4 She-Leveque residual  | arithmetic-numerology   | PASS                              | discrete-equality             |
| **D1.5 axisym-no-swirl**   | **group-symmetry**      | **FAIL_NO_LITERATURE_PATH**       | **discrete-eq + vacuous-tgt** |

Of the 5 catalogued D1 candidates: 1 PASS (D1.4), 2 FAIL (D1.1, D1.5),
1 in-flight (D1.3), 1 unexecuted (D1.2). The seed's heterogeneity
claim (F-SEED-B: five distinct primitive families) holds: each of
the four executed/in-flight candidates exhibits a different
discriminator type (vacuous-magnitude, discrete-equality, structural
cross-PDE, discrete-eq + vacuous-target).

**Catalogue extension yield**: BT-544 now has 1 PASS-grade entry
(D1.4 arithmetic-identity), no PDE-regularity advance, and no
threat to the CONFOUNDED meta-audit verdict.

**Generation-pipeline diagnostic feed (in-flight #51)**: the D1.5
FAIL_NO_LITERATURE_PATH provides a clean instance of "discriminator
authored under unverified existence assumption". Combined with D1.1's
vacuous-cap pattern, this argues for a pre-validation existence
audit of every D-tier discriminator's target value.

---

## §8 Anomalies

### §8.1 Hou-Luo year drift (2013 vs 2014)

Seed §3.5 cites "Hou-Luo 2014"; repo `prob-p2-4` §8.4 cites
"Hou-Luo 2013". The widely-cited *PNAS* paper is dated 2014 (vol.
111, issue 36, Sep 2014); the preprint and conference talk circulated
in 2013. Both citations point to the same work; the discrepancy is
cosmetic. Logged but not blocking.

### §8.2 Euler vs Navier-Stokes scope shift in cited blow-up
literature

Seed §3.5 frames Hou-Luo / Chen-Hou as "above s*; Hou-Luo numerical
blow-up sets in" *for NS*. In fact:
- Hou-Luo 2014 *PNAS*: 3D **axisymmetric Euler** numerical blow-up.
- Chen-Hou 2022 *Annals*: rigorous, on **2D Boussinesq / 3D Euler**.
- Neither paper rigorously establishes **3D NS** blow-up at any
  swirl level.

The seed's "Hou-Luo blow-up sets in" phrasing implicitly conflates
Euler with NS. The audit corrected for this: the **NS** with-swirl
front has **no** rigorous blow-up; only the **Euler** front does
(and even there, only at specific corner-of-cylinder geometric
configurations, not parametrized by a swirl scalar s).

This is an upstream framing anomaly in the seed; D1.5's discriminator
inherited it. The verdict FAIL_NO_LITERATURE_PATH absorbs this
anomaly correctly (the s* threshold doesn't exist for NS; even for
Euler, it isn't characterized as a swirl-magnitude threshold).

### §8.3 Discrete subgroup absence in SO(2)

Seed §3.5 proposes parametrizing swirl by `s in {0, 1/phi, 1/n/phi,
1, ...}`. SO(2) has cyclic finite subgroups Z_n for any n, including
Z_6 (order-6 rotation). However, these subgroups act on the
*angular variable* theta, not on the *swirl magnitude* |u_theta|.
The seed's proposed quantization is on the magnitude scalar, which
is invariant under all of SO(2); there is no group-theoretic reason
to discretize it at n=6 lattice values.

This means even if a critical s* did exist, the n=6 framing on the
*magnitude* would not be group-theoretically derived; it would be a
numerological overlay. F-SEED-B (heterogeneity claim: D1.5 is "group-
symmetry-reduction") is **partially weakened**: D1.5's group-theory
content is the SO(2) reduction itself, but the n=6 discretization
*overlay* is not group-theoretic, it is arithmetic-numerology
relabelling. This drifts D1.5 closer to D1.4's family. **Logged
but does not flip verdict** (the FAIL was already determined by the
literature audit, independent of the framing question).

### §8.4 Comparison to D1.1's vacuous-magnitude FAIL

D1.1 and D1.5 share the property "discriminator FAILed because of an
unverified authoring assumption", but the failure modes differ:

- D1.1: the cap *exists and is computable* (52.36 for Lamb-Oseen),
  but Phi_holo ~ 0 << cap/10 makes the cap vacuous **post-evaluation**.
- D1.5: the threshold s* *does not exist* as a published number, so
  the discriminator cannot be evaluated **at all**.

D1.1's vacuousness was discovered via execution; D1.5's was discovered
via literature audit (no execution possible). Both feed the
generation-pipeline diagnostic but at different stages.

---

## §9 Falsifiers active

Per the seed §3.5 falsifier registration and the parent meta-audit:

- **F-D1.5-A** (best-fit s* matches no n=6 value within 10% =>
  swirl threshold is n=6-blind): **Inactive (formally)**: best-fit
  s* is undefined because no s* exists. **Active in spirit**: the
  audit concludes that the "swirl threshold is n=6-blind" claim
  holds vacuously (with no threshold to be blind about, the claim
  is trivially true).
- **F-D1.5-B** (critical swirl is not threshold-like; continuous
  degradation => reduction-to-discrete-n=6 framing wrong):
  **Active**. The literature shows neither a threshold nor a
  continuous-degradation structure; instead, a dichotomy at u_theta
  = 0 plus open with-swirl problem. The discrete-n=6 framing
  presupposed a continuous-parameter structure that does not exist.
- **F-SEED-A** (atlas-grounding integrity): **Inactive** for D1.5
  (both citations verified verbatim).
- **F-SEED-B** (heterogeneity claim: D1.5 is "group-symmetry-
  reduction" distinct from arithmetic-numerology): **Partially
  weakened** (per §8.3): D1.5's n=6 *overlay* is arithmetic-
  numerology, not group-theory, although the SO(2) reduction itself
  is genuinely group-theoretic. **Logged**, not blocking.
- **F-SEED-E** (atlas drift): **Inactive** for D1.5 (no edits to
  cited files between seed and validation sessions).
- **F-MOLT-A** (catalogue empty / molt-blocked): **Inactive**
  (catalogue has D1.4 PASS).
- **F-DSCRM-CONFLATE** (parent meta-audit: type-axis confounded
  with candidate-axis): **Active and unaffected** by D1.5. The
  CONFOUNDED verdict stands; D1.5 sharpens but does not flip it
  (per §6.5).
- **F-GENPIPE-EXISTENCE** (proposed new falsifier, generation-
  pipeline-diagnostic-feed): "discriminator authored without
  pre-verifying existence of its target value". **Newly active**
  with D1.5 (and retroactively with D1.1 in a different flavor);
  this is the diagnostic that #51 is in-flight on.

No verdict-flipping falsifier fires. The FAIL_NO_LITERATURE_PATH
verdict stands.

---

## §10 Closing

0/7 unchanged. No atlas/state/inventory edits.

**D1.5 axisymmetric-no-swirl symmetry-reduction molt-validation:
FAIL_NO_LITERATURE_PATH**, on the gate criterion PASS-1 (no published
critical-swirl threshold s* exists for 3D NS). The literature
structures the with-swirl problem as a dichotomy or as functional
conditions, not as a magnitude threshold s parametrizing an n=6
discretization. F-D1.5-B fires; F-D1.5-A is vacuously inactive.

**Group-theory mechanism**: SO(2) axial rotation reduction (cylindrical
NS, axisymmetric ansatz, swirl u_theta as residual non-meridional
component). The proposed n=6 discretization on the swirl magnitude
is not derivable from the group structure (SO(2) acts on theta, not
on |u_theta|); it is an arithmetic-numerology overlay over the
genuine group-theoretic reduction. Logged as an §8.3 anomaly that
weakens F-SEED-B's heterogeneity claim.

**Bias-hypothesis 2x2 post-D1.5** (n=10):

|                                                              | PASS | FAIL |
|--------------------------------------------------------------|------|------|
| Distributional / structural-literature                       | 3    | 0    |
| Discrete-equality / numerical-interval / vacuous-magnitude   | 1    | 6    |

Fisher exact two-sided p ~ 0.067 (vs 0.036 at n=8; 0.095 at n=9).
The CONFOUNDED meta-audit verdict stands. D1.5's failure refines
the diagnostic: within the discrete-equality / vacuous row, FAILs
cluster on candidates whose discriminators embed an unverified
existence assumption (target or cap), supporting a generation-
pipeline existence-pre-audit recommendation for the in-flight #51.

**D1-tier completion**: 4 of 5 D1 candidates now validated or
in-flight (D1.1 FAIL_VACUOUS, D1.3 in-flight, D1.4 PASS, D1.5
FAIL_NO_LITERATURE_PATH); D1.2 unexecuted by seed §5 ranking. The
catalogue is no longer molt-blocked; one arithmetic-identity-grade
PASS exists; no PDE-regularity advance for BT-544.

-- end molt-validation --
