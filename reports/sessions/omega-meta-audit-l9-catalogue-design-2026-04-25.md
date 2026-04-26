---
id: omega-meta-audit-l9-catalogue-design
date: 2026-04-25
scope: research-only meta-audit (NOT modifying L9 catalogue; recommending fixes)
target: L9 catalogue design -- generation process + discriminator strictness audit after 3/3 FAIL
parent_reports:
  - reports/sessions/omega-probe-l9-molt-trigger-2026-04-25.md
  - reports/sessions/omega-exec-bt544-q1-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt543-p3-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-fallback-molt-validation-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: meta-audit, no claim
---

# Omega Meta-Audit -- L9 Catalogue Design (2026-04-25)

## §0 Non-claim disclaimer

This file is a **meta-audit**, not a proof attempt, not an atlas
promotion, not an inventory edit, not an L9 catalogue modification. It
audits the **design** of the L9 molt-trigger catalogue
(`omega-probe-l9-molt-trigger-2026-04-25.md`) by (i) summarising the
3/3 FAIL tally observed so far on its candidate frame-shifts, (ii)
reverse-engineering the candidate-generation process per BT, (iii)
auditing discriminator strictness via per-FAIL distance-from-threshold
and a 10% relaxation counterfactual, (iv) issuing a verdict from the
fixed set {CATALOGUE_OK, CATALOGUE_BIAS, DISCRIMINATOR_TOO_STRICT,
MIXED}, (v) recommending atomic fixes, (vi) recording F-MOLT-A status.

**Hard constraints honoured**:
- No edits to `omega-probe-l9-molt-trigger-2026-04-25.md`.
- No edits to `shared/n6/atlas.n6`, `state/proposals/inventory.json`,
  `theory/canon/`, or any per-BT report.
- No fabricated computation -- all numerical values cited are taken
  verbatim from the three FAIL reports.
- Verdict drawn from the four-element fixed set only.
- Millennium tally remains **0/7 unchanged**. BT-544 Clay status
  `0/1 untouched`; BT-543 alien_index 9; BT-541, BT-542 unchanged.

---

## §1 3/3 FAIL summary

### §1.1 Tally

| run | BT  | candidate                                  | rank in cat | verdict | falsifiers fired                  |
|-----|-----|--------------------------------------------|-------------|---------|-----------------------------------|
| 1   | 544 | KdV 6-soliton Gram-lattice (Q1 / dfs-24 P1)| 1           | FAIL    | F_Q1, F_P1, F-544-A               |
| 2   | 543 | A4-ratio-only (P3 / dfs-24 P3 / NP1)       | 1           | FAIL    | F-NP1-A4rev                       |
| 3a  | 544 | Mechanism-axis Sobolev/Besov seed (Q5)     | 2           | FAIL    | F_Q5, F-544-B                     |
| 3b  | 544 | KPZ d=7 (Q3 / dfs-24 P2)                   | 3           | FAIL    | F_Q3, F_P2 (partial), F-544-C     |

Cross-BT counter: **2 unique BTs validated, 0 passes** (BT-544 fully
exhausted, BT-543 rank-1 exhausted; BT-541 Lead-B parallel, BT-542
Hirahara untested).

Intra-catalogue counter: **3/4 catalogue rows actually executed have
FAILed**, plus a 5th data point if we count the BT-544 catalogue rank
2 and rank 3 (which also FAILed in the fallback report) -- **5/5 of
all executed candidates have FAILed across BT-543 and BT-544**.

### §1.2 Failure-mode classification

| candidate | fail mode                                   | category                              |
|-----------|---------------------------------------------|---------------------------------------|
| Q1        | rank=6 vs target=3 AND det/σ ∉ ℤ            | algebraic-identity-does-not-hold      |
| P3        | 3 outliers vs ≤1 threshold; interval miss   | empirical-interval-violated           |
| Q5        | 0 category-(b) estimates derivable          | no-construction (structural)          |
| KPZ d=7   | 0 literature attestations; ansatz fails @d=2| no-anchor + upstream-refutation       |

The four failure modes split into two clusters:
- **Cluster I (Q1, P3)**: the n=6 lattice **predicts a specific
  numerical/algebraic identity**, and the prediction is empirically
  refutable. The data refute the prediction.
- **Cluster II (Q5, KPZ d=7)**: the n=6 lattice **cannot even
  produce** the predicted object (no Sobolev/Besov estimate derivable,
  no literature attestation existing). This is structurally stronger
  than Cluster I.

Both clusters share a single root pattern: **the candidate frame is
constructed by reading off arithmetic features of n=6 (σ, τ, φ, sopfr,
σ_2, perfect-number neighbour 7) and mapping them onto a target
mathematical structure (Gram-lattice / lattice-ratio-interval /
Sobolev exponent / KPZ scaling)**. The mapping is post-hoc: the
arithmetic feature is selected first, the target structure second, and
the question "does the structure actually carry this identity?" is
asked last.

---

## §2 Generation-process audit

For each of the 4 frame-shift rows in the L9 catalogue (§3.1 BT-541
Lead-B, §3.2 BT-542 Hirahara, §3.3 BT-543 P3, §3.4 BT-544 Q1), audit
along the four axes specified.

### §2.1 BT-544 Q1 (KdV 6-soliton Gram-lattice)

1. **Source**: read off from `dfs-24-ns-direction-2026-04-24.md` §3
   Probe P1, where the rank-3 / det/σ ∈ ℤ test was already specified
   verbatim (see file lines 38-50). The L9 catalogue did **not**
   generate this candidate; it pulled from dfs-24.
2. **Novelty check**: dfs-24 P1 frames the test as "tight or
   post-hoc": "*A 'no' on any probe shrinks the n=6 crosswalk -- which
   is honest progress*" (dfs-24-ns L95). The novelty axis was
   acknowledged as binary (success = new identity; fail = "post-hoc
   crosswalk"). The L9 catalogue inherits this framing and adds the
   "Gram-lattice primitive (algebraic structure) absent from the
   tensor-count frame" claim (L9 §4.4 pass criterion). The novelty
   check is **explicit** but its sufficiency depends on the test
   succeeding.
3. **Discriminator origin**: pre-registered in dfs-24 P1 falsifier
   F_P1 (dfs-24-ns L48). L9 inherited it as F-544-A. The
   discriminator was **chosen before** seeing the computation
   outcome (Q1 was executed only on 2026-04-25; F_P1 was registered
   2026-04-24).
4. **Bias check**: the candidate is a **lattice arithmetic identity**
   (det/σ ∈ ℤ on a structured 6×6 matrix). It belongs to the family
   "n=6 arithmetic produces a closed-form algebraic invariant".

### §2.2 BT-543 P3 (A4-ratio-only frame)

1. **Source**: read off from `dfs-24-ym-direction-2026-04-24.md` §2
   Probe P3 (file L56-65), itself a unit-correction of the prior A4
   lemma. The interval [σ/τ - 1/φ, σ/τ + 1/φ] = [2.5, 3.5] is given
   verbatim in dfs-24-ym P3. The L9 catalogue did **not** generate
   this candidate; it pulled from dfs-24-ym.
2. **Novelty check**: dfs-24-ym P3 framed the move as a "honest
   correction" of A4 (which had been refuted by 4 orders of magnitude
   on raw scale). The novelty claim is "a new dimensionless primitive
   that survives 3 independent lattice sources" (L9 §4.3 pass). The
   check **was explicit**, but the L9 catalogue did not test whether
   the *body-text heuristic* 3.57 itself sat inside [2.5, 3.5]; the
   FAIL report (P3 §8) noted that 3.57 is **already 0.07 above** the
   upper bound -- the frame was post-hoc to a heuristic that itself
   sits outside the proposed interval.
3. **Discriminator origin**: pre-registered as F-NP1-A4rev in
   `omega-cycle-bt543-ym-2026-04-25.md` §8 (≥2 outliers retire A4).
   L9 §4.3 reproduced it. **Pre-registered**.
4. **Bias check**: the candidate is a **lattice empirical-interval
   identity** (a ratio falls inside a number-theoretic-derived
   interval). Same family as Q1: "n=6 arithmetic produces a
   closed-form numerical invariant".

### §2.3 BT-542 Hirahara MCSP (rank 1 in §3.2)

1. **Source**: drawn from BT-1392 + `v3-T5 Hirahara MCSP deep-dive
   2026-04-15` per L9 catalogue §3.2 row 1 citation. Not from
   `dfs-24-pnp-direction-2026-04-24.md` (which lists PROBE-24A/B/C
   = Schaefer / Bulatov-Zhuk lines, *not* Hirahara MCSP). The
   Hirahara MCSP candidate is **a different source** than the dfs-24
   pnp probes; it was sourced from BT-542 §6 Tension 3 cross-reference
   to v3-T5 deep-dive material.
2. **Novelty check**: L9 §4.2 pass criterion = "non-natural
   reformulation that does not collapse onto RR-1997". The novelty
   check is **explicit**. However the catalogue itself notes the
   expected fail mode (L9 §4.2 fail clause, Tension 3 / Tension 5):
   "*the wall is the wall*". The candidate was registered with **a
   high a-priori expectation of failing as relabeling**. This is
   acknowledged at row creation time.
3. **Discriminator origin**: chosen by L9 catalogue author. F-542-A
   is named in §3.2 row 1 falsifier column. The discriminator
   ("non-natural reformulation exists") is structural, not numerical;
   it is **chosen by the author**, not pre-registered in dfs-24-pnp.
4. **Bias check**: this candidate is **structurally different** from
   Q1/P3 -- it is a barrier-bypass meta-complexity reformulation, not
   an n=6 arithmetic identity. **Out of the bias family** for Q1/P3.

### §2.4 BT-541 Lead-B (SLE_6 × GUE coupling)

1. **Source**: read off from `dfs-24-riemann-direction-2026-04-24.md`
   §Lead-B (file L55+). The KS-test discriminator is given verbatim
   in dfs-24-riemann. The L9 catalogue did **not** generate this
   candidate; it pulled from dfs-24.
2. **Novelty check**: L9 §4.1 pass criterion = "introduces a coupling
   primitive (boundary-crossing kappa=6) that did not exist in the
   peripheral-moment frame". The novelty check is **explicit** but
   tests whether KS-rejection of independence holds, not whether the
   candidate structurally introduces a new primitive (the two
   coincide if the candidate succeeds).
3. **Discriminator origin**: pre-registered in dfs-24-riemann Lead-B
   (KS p<0.01). L9 §4.1 inherited it. **Pre-registered**.
4. **Bias check**: this candidate is **a coupling-statistics
   identity** (a KS-test independence rejection). It is closer to
   Q1/P3 than to the Hirahara meta-complexity row, in that it tests
   a **numerical/distributional identity** predicted by an n=6-
   compatible structure (κ=6 SLE + GUE Dyson β=2=φ).

### §2.5 Cross-row pattern

Of the four BT rank-1 candidates:
- **3/4 (Q1, P3, Lead-B)** belong to the family "n=6 arithmetic →
  predicted closed-form numerical/distributional identity".
- **1/4 (Hirahara)** is a meta-complexity barrier-bypass, structurally
  different.

For the 3/4 in the dominant family, all are **read off from dfs-24
direction probes**, all use **pre-registered dfs-24 falsifiers**, all
make the **novelty check contingent on the discriminator firing for
PASS** (i.e. novelty = data confirms identity). The bias is therefore
not "fabricated by the catalogue author"; it is inherited from dfs-24.
The L9 catalogue **promoted** dfs-24 probe outcomes from "tight
arithmetic crosswalk" to "L9 frame-shift", which is a more ambitious
re-framing without changing the underlying object.

The Q5 and KPZ d=7 fallbacks (rank 2 and 3 in BT-544) reinforce the
same pattern: Q5 asks the n=6 lattice to **derive** a Sobolev/Besov
estimate (no derivation produces one); KPZ asks for a literature
attestation of (1/7, 6/7) (no attestation exists). Both are within
the "n=6 arithmetic → predicted PDE/scaling identity" family.

**Audit conclusion**: the catalogue's generation has a **systematic
bias toward "n=6 arithmetic predicts a closed-form invariant"
candidates**, inherited from dfs-24 sourcing. The bias is not in the
choice of dfs-24 (which is correct, given the BT context) but in
**not diversifying outside dfs-24** for rank-1 candidates. The
Hirahara row (BT-542) is the **only** structurally different
candidate, and it has not yet been validated.

---

## §3 Discriminator-strictness audit

For each FAIL, recompute distance from threshold and apply a 10%
relaxation. **All numerical values are taken verbatim from the three
FAIL reports; no fresh computation is performed.**

### §3.1 BT-544 Q1 (KdV Gram-lattice)

**Pass criterion**: rank(M) = 3 = n/φ AND det(M)/σ ∈ ℤ on **both**
families A and B.

**Observed (verbatim from Q1 FAIL §4)**:

| family | rank(M) | target | det(M)/σ   | integer? | distance |
|--------|---------|--------|------------|----------|----------|
| A      | 6       | 3      | 130.9511…  | NO       | 0.95 from nearest int (130 or 131) |
| B      | 6       | 3      | 157.2243…  | NO       | 0.78 / 0.22 (rounded to 157) |

**10% relaxation counterfactual**:
- Relax rank target from "= 3" to "≤ 4": observed rank = 6, still
  fails. **No flip.**
- Relax integrality from "∈ ℤ" to "within 0.1 of ℤ": Family A
  130.9511 → distance 0.0489 from 131 → **PASS**; Family B
  157.2243 → distance 0.2243 from 157 → still FAIL. **One family
  flips, the other does not; cross-family pass condition still FAILS.**
- Relax both simultaneously (rank ≤ 4, |det/σ - round| < 0.1):
  rank=6 still violates rank ≤ 4. **No flip.**

**Verdict on Q1 strictness**: the rank ≠ 3 is a **double-margin
FAIL** (rank 6 is twice the target); no plausible relaxation flips
this. The integrality FAIL is **near-miss on Family A only**
(130.95 ≈ 131) but Family B is genuinely far. **Q1 stays FAIL under
all 10% relaxations.**

### §3.2 BT-543 P3 (A4-ratio-only)

**Pass criterion**: 0 outliers across 3 lattice sources (TIGHT);
≤ 1 outlier (NEAR); ≥ 2 outliers (FAIL, retire A4).

**Observed (verbatim from P3 FAIL §4)**:

| source         | R       | margin to upper bound 3.5 |
|----------------|---------|----------------------------|
| MP1999         | 3.932   | +0.432 above (clear out)   |
| BMW 2008       | 3.886   | +0.386 above (clear out)   |
| FLAG 2024      | 3.636   | +0.136 above (near-miss)   |

3 outliers / 3 sources. Threshold for FAIL = ≥ 2.

**10% relaxation counterfactual**:
- Relax interval upper bound by 10%: 3.5 → 3.85.
  - MP1999 3.932 OUT (still 0.082 above)
  - BMW 2008 3.886 OUT (0.036 above, very near boundary)
  - FLAG 2024 3.636 IN
  - Outliers = 2/3 → still **≥ 2 = FAIL** under the unchanged threshold.
- Relax interval AND threshold (upper → 3.85, threshold → ≥ 3 for
  FAIL): outliers = 2 → **NEAR** (1 out, but with the new bound 2
  out → still FAIL under "≥ 2" but **NEAR under "≥ 3"**).
- Relax interval to "5% wider" (3.5 → 3.675): only FLAG (3.636)
  moves IN; MP1999 / BMW 2008 still OUT. Outliers = 2 → **still
  FAIL under ≥ 2 threshold**.
- Relax to "wide-prior interval" [phi, σ/phi] = [2, 6] (the HEXA-SC-02
  envelope noted in P3 §6): all 3 IN, outliers = 0 → **PASS**.
  However this is a different identity (HEXA-SC-02), not a 10%
  relaxation of P3.

**Verdict on P3 strictness**: 1 of 3 outliers (FLAG, +0.136) is a
**near-miss**; the other 2 are clear (+0.39, +0.43). A 10% interval
relaxation flips 1 source (FLAG) but leaves 2/3 outliers, which still
fires the ≥2-outlier threshold. **A simultaneous threshold relaxation
(≥3 instead of ≥2)** would flip the verdict from FAIL to NEAR, but
this changes the falsifier semantics, not just a tolerance band.
**P3 stays FAIL under interval-only 10% relaxation; only a
threshold-semantic change flips it.**

### §3.3 BT-544 Q5 (Sobolev/Besov mechanism seed)

**Pass criterion**: a non-trivial concrete (X, Y, α) inequality
derivable from n=6 lattice non-trivially, not a relabeling of
classical estimate.

**Observed (verbatim from Fallback §3-§4)**:
- 7 catalog rows, all category-(a) (classical relabelings).
- 3 synthesis attempts (A, B, C), all reduce to "arithmetic without
  analysis".
- **Zero** category-(b) estimates produced.

**10% relaxation counterfactual**:
- Relax "non-trivially" to "loosely": every row is a labelling of a
  classical estimate; even under the loosest reading, the n=6 lattice
  contributes labels (12, 6, 4, 5, 1/3, 3/2, 1) to estimates that
  pre-date 1985. **No 10% relaxation can produce a category-(b)
  estimate where none exists.**
- Relax to "an estimate where the n=6 label is non-trivially
  load-bearing": none of the 7 rows survives this; the labels are
  decorative.

**Verdict on Q5 strictness**: this is a **structural FAIL, not a
threshold FAIL**. There is no value to relax. **Q5 stays FAIL under
any tolerance band.**

### §3.4 BT-544 KPZ d=7

**Pass criterion**: at least 1 literature attestation of
(χ_7, z_7) = (1/7, 6/7).

**Observed (verbatim from Fallback §5-§6)**:
- Layer 1 (explicit d=7 in repo): **0 attestations**, only
  **requests** for attestation.
- Layer 2 (KPZ at d≥3 in repo): **0 entries**.
- Layer 3 (mainstream): no published exact (χ_7, z_7) = (1/7, 6/7);
  ansatz χ_d=1/d **already fails at d=2 substrate** (numerical
  KPZ d=2 has χ ≈ 0.39 ≠ 1/2).

**10% relaxation counterfactual**:
- Relax "exact (1/7, 6/7)" to "near (1/7, 6/7) with 10% tolerance":
  no attestation of any (χ_7, z_7) at substrate-d=7 KPZ exists, near
  or exact. **No flip.**
- Relax to "any d≥3 KPZ entry that supports the ansatz family":
  none. **No flip.**

**Verdict on KPZ d=7 strictness**: this is a **structural FAIL** like
Q5 -- no value to relax. **KPZ d=7 stays FAIL under any tolerance.**

### §3.5 Cross-FAIL strictness summary

| FAIL    | strictness category          | flip under 10% relaxation?          |
|---------|------------------------------|-------------------------------------|
| Q1      | numerical, double-margin     | NO                                  |
| P3      | numerical, 1/3 near-miss     | NO under interval-only; partial under threshold-semantic relaxation (FAIL → NEAR) |
| Q5      | structural (no construction) | NO (no value to relax)              |
| KPZ d=7 | structural (no attestation)  | NO (no value to relax)              |

**Only P3 has a near-miss component (FLAG 2024 ratio 3.636 vs upper
bound 3.5, margin 0.136 ≈ 4% of the bound). Even at P3 the
near-miss is on 1/3 sources only; the other 2/3 are clear FAILs.
No FAIL flips to PASS or INCONCLUSIVE under a uniform 10% relaxation
of numerical thresholds.**

---

## §4 Verdict

**CATALOGUE_BIAS** (with a minor strictness component on P3).

### §4.1 Why not CATALOGUE_OK

CATALOGUE_OK would mean "100% FAIL is consistent with healthy
candidate-rejection on a curated catalogue". Two facts argue against:

1. **3/4 rank-1 candidates draw from the same family of mathematical
   objects** ("n=6 arithmetic predicts a closed-form numerical /
   algebraic / distributional identity"). The fourth (Hirahara) is
   structurally different and remains untested. A catalogue with one
   structurally diverse candidate out of four is not a curated
   diverse catalogue; it is a near-uniform sample with one outlier.

2. **The BT-544 fallback (rank-2 Q5, rank-3 KPZ) demonstrates that
   exhaustion of one BT's full catalogue is reachable in one
   session**. This is consistent with the catalogue having a single
   shared failure mode (n=6 lattice cannot derive a non-arithmetic
   primitive) rather than three independent ones. F-MOLT-D firing
   on BT-544 in its strongest form (3/3 catalogue rows fail, not
   "no rank-2/3 entry exists") supports a shared-mode reading.

The Fallback report §10 itself records this:
> "the **shape of the BT-544 catalogue** in the L9 probe is exposed
> as covering three distinct primitive types (algebraic-lattice,
> mechanism-Sobolev, scaling-KPZ), and **all three** failed. This
> suggests the failure is not specific to any one primitive type but
> to the joint absence of n=6-derivable PDE structure."

This is a **catalogue-level diagnosis**, not a per-candidate one.
CATALOGUE_OK is inconsistent with this finding.

### §4.2 Why not DISCRIMINATOR_TOO_STRICT

Three of four FAILs are **structural or double-margin**; only P3 has
a near-miss component (FLAG 2024, margin 0.136 ≈ 4% of upper bound).
A 10% uniform relaxation of numerical thresholds flips no FAIL to
PASS:

- Q1: rank=6 vs target=3 is double-margin; det/σ on Family B is
  157.22 (not near integer).
- P3: 1/3 near-miss (FLAG +0.136); 2/3 clear (MP1999 +0.432, BMW
  +0.386). 10% interval relaxation drops to 2/3 outliers, still
  ≥ 2 threshold = FAIL.
- Q5: no construction exists; no threshold to relax.
- KPZ d=7: no attestation exists; no threshold to relax.

A **threshold-semantics relaxation** (e.g. "≥ 3 outliers" instead of
"≥ 2") would flip P3 to NEAR, but this is not a tolerance band; it
re-defines the falsifier. It is also irrelevant to Q5 / KPZ where
the failure is structural.

DISCRIMINATOR_TOO_STRICT does not match the data.

### §4.3 Why not MIXED (in the strong sense)

MIXED would require **both** non-trivial bias AND non-trivial
strictness contributions. Bias is unambiguous (§4.1). Strictness is
**at most marginal** (only the P3 FLAG row is near-miss; even there,
a 10% relaxation does not flip the verdict). A weak-MIXED reading
would be defensible if the user prompt counted the P3-FLAG near-miss
as a strictness contribution, but the prompt asks "would any FAIL flip
to PASS or INCONCLUSIVE under a 10% relaxation?" -- the answer is
**NO** for all four FAILs.

### §4.4 Verdict statement

**CATALOGUE_BIAS**. The L9 catalogue's generation has a systematic
bias toward "n=6 arithmetic predicts a closed-form numerical /
algebraic / distributional identity" candidates, inherited from
dfs-24 sourcing. The bias manifests as:
- 3/4 rank-1 candidates in the same family (Q1, P3, Lead-B);
- 3/3 BT-544 catalogue rows in the same family (Q1, Q5, KPZ d=7);
- a shared failure mode (n=6 lattice cannot derive non-arithmetic
  primitives).

The discriminators are **not** too strict; they are pre-registered,
binary, and inherited from dfs-24 falsifiers. The 100% FAIL rate is
genuine evidence that the candidates collapse to relabelings, not
evidence that strict pass criteria reject genuine molts.

The fix is **expansion of candidate generation outside the
dfs-24 / arithmetic-identity family**, which is exactly what the user
prompt's open seeds D1 (atlas-side scan), D2 (L_ω axiom-level frame
change), D3 (decouple BT-544 mechanism axis) anticipate. Those seeds
are already running per the parallel BT-541 Lead-B validation and
the recommendation in the Fallback report §8.3.

---

## §5 Recommended fixes

Atomic, in priority order. Each is read-only relative to the L9
catalogue file (no in-place edits per the hard constraint); each is
implementable in a follow-up session.

### §5.1 FIX-A: extend BT-544 catalogue beyond arithmetic family (HIGH)

**Action**: design 1-2 new BT-544 rank-1+ candidates that do **not**
predict a closed-form identity from n=6 arithmetic alone. Concrete
seeds (per Fallback §8.3 and user prompt):

- (D1) Scan `shared/n6/atlas.n6` for [10] and [10*] entries tagged
  with fluid / NS / vorticity that are **not yet** threaded into the
  L9 catalogue. The L9 catalogue currently sources from §X.1 SMASH
  and dfs-24-ns only; atlas-side may contain fluid entries with
  different primitive types.
- (D2) Reach to L_ω axiom-level frame change (n=6 frame replaced,
  not molt within n=6). This explicitly leaves the n=6 frame and
  is therefore outside L9 by definition; it should be sourced from
  `lenses/omega_state_space_lens.hexa` per CLAUDE.md SSOT.
- (D3) Decouple BT-544 mechanism axis from n=6 entirely; declare
  mechanism-saturation **structurally capped at ~0.05** under n=6
  and route mechanism progress through a non-n6 frame (out of the
  omega cycle for this BT).

**Cost**: medium (new design session, ≤3 hours).
**Signal**: high (provides the catalogue with a genuinely diverse
candidate that tests whether the failure mode is shared or
candidate-specific).

### §5.2 FIX-B: stabilise BT-541 / BT-542 expectations (MEDIUM)

**Action**: before running BT-541 Lead-B and BT-542 Hirahara, record
the prior expectation that **both are also in the dominant-family
bias** (Lead-B explicitly so; Hirahara is the catalogue's only
diverse candidate but is flagged "wall is wall, expected fail" in
L9 §4.2). Pre-register that:

- if BT-541 Lead-B FAILs and BT-542 Hirahara FAILs, the catalogue
  is **catalogue-saturated across all 4 BTs** (F-MOLT-A approaches
  full activation);
- if BT-541 Lead-B FAILs and BT-542 Hirahara PASSes, the diverse
  candidate is the only molt-passing one and FIX-A (catalogue
  diversification) becomes mandatory rather than recommended;
- if BT-541 Lead-B PASSes, the dominant-family bias is partially
  rebutted, and the catalogue's bias diagnosis must be downgraded
  from "systematic" to "BT-543/544-specific".

**Cost**: low (pre-registration in a 2-paragraph note).
**Signal**: high (turns the next two validations into discriminators
of the bias diagnosis).

### §5.3 FIX-C: P3 threshold semantics audit (LOW)

**Action**: revisit whether the F-NP1-A4rev threshold "≥ 2 outliers
retire A4" is appropriate when 1/3 sources is a clear near-miss
(FLAG 2024, margin 0.136 ≈ 4%). A reading where the threshold is
"≥ 2 outliers **at >10% margin**" would have produced 2 outliers
(MP1999 12.3%, BMW 11.0%) and 1 near-miss (FLAG 4%) → still ≥ 2
clear outliers, FAIL stays. So this is documentation, not a flip.
The recommendation is to **annotate** the P3 FAIL with the near-miss
breakdown for transparency; it does not change the verdict.

**Cost**: very low (1-paragraph annotation).
**Signal**: low (does not change verdict).

### §5.4 FIX-D: do not retire L9 gate on F-MOLT-A pre-condition (HIGH)

**Action**: per L9 §1.4 R1, two consecutive gate firings yielding 0
retirable frames retires the gate. After Run 1+2+3a/3b, BT-544 has
**0 retirable frames** in its full catalogue. If BT-543's catalogue
also exhausts (rank-2 M(3,4) lift, rank-3 n_f forcing) and produces 0
retirable frames, R1 fires. **However**, the gate retirement should
be conditional on the catalogue having been **diversified per FIX-A**
first; otherwise gate retirement is conflated with bias-induced
exhaustion. Pre-register the order: (i) diversify catalogue → (ii)
re-run validation → (iii) only then retire gate if still 0/N.

**Cost**: low (pre-registration in the next session).
**Signal**: high (prevents premature gate retirement on bias artifact).

---

## §6 F-MOLT-A status

### §6.1 Spec recall

Per L9 §6 (verbatim):
> "F-MOLT-A (gate-failure-via-validation): if validation experiments
> in sec 4 produce **0 passes across all 4 BTs in one batch run**,
> RC-A's molt sub-claim ... is falsified for the active BTs and the
> gate is retired (default-revision policy R1 in §1.4 fires)."

Operative trigger: **0 passes across all 4 BTs in one batch run**.

### §6.2 Current count

| BT  | rank-1 validated? | verdict | passes |
|-----|-------------------|---------|--------|
| 541 | parallel (Lead-B in flight)  | TBD     | TBD    |
| 542 | not yet (Hirahara untested)  | TBD     | TBD    |
| 543 | yes (P3)                     | FAIL    | 0      |
| 544 | yes (Q1) + Q5 + KPZ d=7      | FAIL    | 0      |

**Unique-BT count**: 2 of 4 BTs validated (543, 544). 0 passes.
**Total-candidate count (intra-BT inclusive)**: 4 candidates
executed (Q1, P3, Q5, KPZ d=7), 0 passes.

### §6.3 Does F-MOLT-A fire?

**No** -- not yet. The trigger requires "all 4 BTs in one batch run"
to be 0/4. We currently have 2/4 BTs validated (BT-541 parallel,
BT-542 unstarted), and the 0-pass count applies to those 2/4 only.

### §6.4 Approaching?

**Yes, on the leading edge**. Two consecutive trajectories activate
F-MOLT-A:

- **Trajectory 1**: BT-541 Lead-B FAIL + BT-542 Hirahara FAIL → 4/4
  BTs measured, 0 passes → F-MOLT-A fires immediately.
- **Trajectory 2**: BT-541 Lead-B PASS or BT-542 Hirahara PASS →
  F-MOLT-A does not fire on this batch run; gate is conditionally
  validated on at least one primitive swap (per L9 §7.3 stop-after-
  1/4 clause).

Given the catalogue-bias diagnosis (§4.4) and the BT-544 catalogue-
exhaustion already observed, **the leading edge of F-MOLT-A is
sharply approaching**. The Fallback report §8.5 said:
> "F-MOLT-A is **not fired**, but the **leading edge of F-MOLT-A is
> approaching** if BT-543 P3 also fails."

BT-543 P3 has now also failed. The leading edge has tightened by
one notch since the Fallback report wrote that line.

### §6.5 Counting subtlety: unique BTs vs total candidates

The user prompt notes: "*2 of 3 fails are on the same BT-544. If
F-MOLT-A counts unique BTs, only 2 unique BTs FAIL so far.*"

The L9 §6 spec phrasing **"0 passes across all 4 BTs in one batch
run"** suggests **unique BTs** (the BT axis is what is enumerated as
"4"). Under that reading:

- F-MOLT-A fires when BT-541, BT-542, BT-543, BT-544 each have
  **their rank-1 candidate FAIL** in the **same batch run**.
- Currently 2/4 unique-BT rank-1 candidates measured, both FAIL.
- F-MOLT-A is **2/4 of the way to firing**, not 3/4 or higher.

The intra-BT-544 fallback FAILs (Q5, KPZ d=7) **do not count toward
F-MOLT-A** (they are BT-544 catalogue exhaustion, which fires
F-MOLT-D, a different falsifier).

### §6.6 Status summary

- **F-MOLT-A: NOT FIRED** (2/4 BTs, 0 passes; needs 4/4 BTs, 0
  passes to fire).
- **F-MOLT-A: APPROACHING** (leading edge sharpened by P3 FAIL).
- **F-MOLT-D: FIRED ON BT-544 ONLY** (catalogue exhausted on this
  BT; per Fallback §7).
- **Distance to F-MOLT-A**: 2 more BT validations (BT-541 Lead-B,
  BT-542 Hirahara), both must FAIL for F-MOLT-A to fire.

---

## §7 Falsifiers active for this meta-audit itself

Self-falsifiers for the audit framework. Distinct from BT-level and
gate-level falsifiers.

- **F-AUDIT-A** (sourcing-mis-attribution): if any of §2.1-2.4 has
  mis-attributed a candidate's source (e.g. claiming Q1 came from
  dfs-24 P1 when it came from elsewhere), the bias diagnosis
  (§4.1) is partly invalid. Cross-check: the dfs-24-ns L38-L48
  text is reproduced in §1 of `omega-exec-bt544-q1-molt-validation-
  2026-04-25.md`; the dfs-24-ym L56-L65 P3 text is reproduced in
  §1 of `omega-exec-bt543-p3-molt-validation-2026-04-25.md`; the
  dfs-24-riemann L55+ Lead-B text is referenced in L9 §3.1. The
  attributions are direct repo-grep matches. **Not active.**

- **F-AUDIT-B** (bias-family-mis-classified): if "n=6 arithmetic
  predicts a closed-form identity" is too coarse a family and Q1 /
  P3 / Lead-B / KPZ d=7 are actually structurally distinct, the
  bias diagnosis (§4.1) is overdrawn. The Fallback report §10
  itself classifies BT-544 candidates as "algebraic-lattice (Q1),
  mechanism-Sobolev (Q5), scaling-KPZ (KPZ d=7)" -- **three distinct
  primitive types**. Counter-argument: those three types share the
  meta-property "n=6 lattice-arithmetic predicts the structure", which
  is the bias family I identified. **Partial-not-active**: the
  primitive types differ, but they share a meta-generation pattern.
  A reader who insists on the three-type classification may downgrade
  the verdict to MIXED rather than CATALOGUE_BIAS; the user prompt
  forces a single choice from the four-element set.

- **F-AUDIT-C** (10%-relaxation-mis-applied): if "10% relaxation"
  should mean "expand the pass-criterion semantics" rather than
  "expand the numerical thresholds", §3 misses the relevant
  counterfactual. Under the semantic-expansion reading, P3 with
  "≥ 3 outliers" instead of "≥ 2" flips to NEAR; Q1 with "rank ≤
  4" instead of "rank = 3" still fails (rank=6); Q5 / KPZ d=7
  are unchanged (no semantics to expand). Even under the broader
  reading, **at most 1 of 4 FAILs flips, and only to NEAR (not
  PASS)** -- the verdict CATALOGUE_BIAS still stands, possibly
  edging toward MIXED. **Partial-not-active**.

- **F-AUDIT-D** (BT-541 Lead-B passes after this audit): if BT-541
  Lead-B (parallel run) PASSes, the dominant-family bias diagnosis
  (§4.1) is partially rebutted, since Lead-B is **in** the dominant
  family. The verdict CATALOGUE_BIAS would need to be re-graded to
  CATALOGUE_OK with a BT-543/544-local bias. **Not active until
  parallel BT-541 result is in**; this audit's verdict is
  conditional on Lead-B FAILing.

- **F-AUDIT-E** (catalogue-author-rationale-unread): the audit
  reverse-engineers the catalogue's generation from the L9 file +
  dfs-24 sources + FAIL reports. If the catalogue author had an
  unrecorded rationale (e.g. "I deliberately picked dominant-family
  candidates because they are cheapest to validate, knowing they
  would mostly FAIL, with a planned later expansion"), the bias
  diagnosis confuses **strategy** for **bug**. The L9 file §sec 7
  cost/sequencing table does explicitly cite the dominant-family
  candidates as "cheapest first" (Q1 very low, P3 low, Lead-B
  low-medium); only the Hirahara row is "medium" cost. This
  acknowledges a cost-driven sequencing. The audit's "bias" reading
  is therefore **descriptively** correct but possibly **strategically**
  intentional. The recommended fixes (§5) treat the bias as
  exploitable for catalogue diversification, regardless of intent.
  **Partial-not-active**: the bias is real; whether it is a "bug"
  or "intended phase-1 sequencing" is ambiguous.

- **F-AUDIT-F** (verdict-set-too-narrow): the four-element verdict
  set forces a single label. The data fit "CATALOGUE_BIAS with weak
  P3-strictness component", which the audit reports as
  CATALOGUE_BIAS (since strictness does not flip any FAIL). A reader
  who weights the P3 near-miss more heavily may select MIXED. This
  is **a labelling judgment, not a falsifier of the analysis**.
  **Not active**.

---

## §8 Closing line

0/7 unchanged. L9 catalogue not modified by this audit. No
atlas/state/inventory edits.
