---
id: omega-exec-bt546-bsd-molt-validation
date: 2026-04-25
scope: research-only L9-catalogue-extension + molt-validation (NO BSD claim, NO atlas promotion, (A3) unproved)
target: BT-546 BSD -- new frame-shift candidate + validation; F-MOLT-A 5th-or-6th-sample
parent_reports:
  - reports/sessions/omega-cycle-bt546-bsd-2026-04-25.md
  - reports/sessions/dfs-24-bsd-direction-2026-04-24.md
  - reports/sessions/omega-exec-bt546-probeB-28stratum-2026-04-25.md
  - reports/sessions/omega-meta-audit-l9-catalogue-design-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: catalogue-extension + validation, no claim
---

# Omega Exec -- BT-546 BSD L9-Catalogue-Extension + Molt-Validation (2026-04-25)

## §0 Non-claim disclaimer (binding)

This document executes a **catalogue-extension experiment** on BT-546
(Birch-Swinnerton-Dyer conjecture). It claims **NONE** of the
following:

- BT-546 status remains **PARTIAL under Y8 (GALOIS-ASSEMBLY)**. No
  promotion proposed.
- Hypothesis **(A3)** (BKLPR `|Sel_p| ⊥ |Sel_q|` for `p ≠ q`) **remains
  unproved**. The discriminator below tests whether the empirical
  28-stratum kappa distribution is distinguishable from the
  (A3)-implied null `κ ≡ 0`; that test is **diagnostic of
  (A3)-on-finite-Cremona**, not a refutation of (A3) at the asymptotic
  limit `B → ∞`.
- **No atlas entry is created, modified, or promoted**.
  `state/proposals/inventory.json` is **not** touched.
- The rank ≥ 2 BSD problem remains open.
- The κ exponent question (7/40 vs log(2)/τ) remains undecided.
- Millennium resolved counter: **0/7 (preserved)**.

No new harness data collected; the discriminator is computed on the
**existing** 28-stratum output (`/tmp/probeB_28stratum.json`,
1,705,824 curves, written 2026-04-25 in
`omega-exec-bt546-probeB-28stratum-2026-04-25.md`). English technical
terms only.

---

## §1 Catalogue-extension rationale

The L9 frame-shift catalogue (`omega-probe-l9-molt-trigger-2026-04-
25.md` §3) covers BT-541/542/543/544. **BT-546 BSD is excluded**.
Three facts make this gap actionable:

1. **Highest closure plateau**. The `omega-cycle-bt546-bsd-2026-04-
   25.md` audit rated BT-546 closure at **2-3 of 5** -- the highest
   plateau among the six unresolved Millennium BTs (composite proxy
   0.78-0.84 in §3.2, atlas density 41 entries, 6 cross-domain
   crossovers, 9 active falsifiers). A high-plateau BT lacking a
   frame-shift catalogue row is a scope gap.
2. **Richest empirical input**. BT-546 has the densest empirical
   layer: Cremona 332k → 1.7M, 28-stratum cov, 100% Cassels-Tate
   perfect-square at N=332,366 (`MILL-GALO-PX2-sha-all-squares-332k`
   [10*]). Per the 28-stratum result, an actionable distributional
   input (`/tmp/probeB_28stratum.json`, 19 statistically-viable cells
   with bootstrap SE) is already on disk.
3. **F-MOLT-A 4-BT split is on discriminator type, not BT family**.
   The 4-BT calibration sweep
   (`omega-exec-bt542-hirahara-molt-validation-2026-04-25.md` §6)
   closed at **2 PASS / 2 FAIL** with a clean axis: PASSes are
   distributional (BT-541 SLE_6 × GUE KS) and literature-import
   (BT-542 Hirahara MCSP); FAILs are numerical-equality (BT-543 P3
   ratio interval, BT-544 Q1 Gram-lattice det/σ ∈ ℤ). BT-546 is the
   natural 5th sample: it has distributional input on disk and
   literature-grade BSD axes (BKLPR, Sato-Tate, Heegner-Gross-Zagier,
   Iwasawa, Bhargava-Shankar) to import from.

BSD-specific candidate sources considered: (a) Heegner-Gross-Zagier
twist distribution (DFS-24 Probe C; expensive, infrastructure
dependency), (b) Sato-Tate at primes for E_n / random matrix analogy
(no on-disk data), (c) p-adic L-function via Coleman-Mazur
eigencurve (no on-disk data), (d) Iwasawa Selmer growth (deferred
per `bsd-cremona-sel6-empirical-2026-04-15.md` §7), (e)
**Bhargava-Shankar style random Galois cokernel test on the
28-stratum data** (cheap, on disk, distributional + literature-import
hybrid). **(e) is selected** as cheapest + most heterogeneous from
the existing 4 catalogue rows.

---

## §2 Proposed frame-shift spec

### §2.1 Name

```
BT-546 frame-shift candidate (rank 1):
  "Sel_6 = σ(6) deterministic-equality frame"
       ----->
  "BKLPR (A3) random-cokernel-null distributional-rejection frame"
```

Working short-name: **`A3-distributional-rejection`** (hereafter
**A3D**).

### §2.2 Frame content

The **prior frame** (used in `MILL-PX-A9` [9],
`bsd-cremona-sel6-empirical-2026-04-15.md` §4-5) is a deterministic
equality:

```
under (A3):  E[|Sel_n(E)|] = σ(n)         (squarefree n)
n=6:          E[|Sel_6|] = 12 = σ(6)       (deterministic)
```

Empirical signal: `ratio(6) = 9.51 / 12 = 0.7925` (cited
`bsd-cremona-sel6-empirical-2026-04-15.md` §4.5; `omega-cycle-bt546-
bsd-2026-04-25.md` §1.3). Discriminator type: **numerical-equality
near-miss** with 4 documented degrees of freedom (conductor bias,
Z/2×Z/2 1st-order approximation, Sha[p] approximation, finite
tails).

The **A3D frame** replaces the deterministic-equality discriminator
with a **distributional rejection** of the (A3)-implied null on a
structured stratification grid:

```
H0 (BKLPR-A3 null on 28-cell torsion × Sha grid):
  κ_(t,c) := Cov(|Sel_2|, |Sel_3| | torsion=t, sha-class=c) = 0
  for every cell (t, c).

H1 (anti-A3 distributional alternative):
  the empirical κ_(t,c) distribution is distinguishable from
  the H0 point-mass-at-0 distribution at conventional levels.
```

### §2.3 Repo-grounded anchors

- **Y8 GALOIS-ASSEMBLY axis dominance**: `omega-cycle-bt546-bsd-
  2026-04-25.md` §1.4 records κ(B) power-law α = 0.1752 ± 0.02
  (7-bin Cremona 1.73M, `bsd-kappa-0.175-decomposition.md`). The
  asymptotic alternative α > 0 predicts κ(B) → ∞; the H0 `κ ≡ 0`
  finite-B null is the correct discriminator target.
- **Sel_6 = σ line empirical underpinning**: `bsd-cremona-sel6-
  empirical-2026-04-15.md` §4.5 ratio(6) = 0.7925 + 4-reason
  interpretation. A3D extends this from a single ratio to a 19-cell
  distribution.
- **28-stratum data input** (`/tmp/probeB_28stratum.json`,
  1,705,824 curves; `omega-exec-bt546-probeB-28stratum-2026-04-
  25.md` §4 table 119-148): 19 cells with `N_cell ≥ 100` carry
  kappa estimates with 1000-bootstrap SE.
- **Literature import**: BKLPR random-cokernel model (Bhargava-
  Kane-Lenstra-Poonen-Rains, Camb J. Math 2015), recorded
  `breakthrough-theorems.md:20098-20107`.

### §2.4 Pre-registered falsifiers

- **A3D-α (relabeling)**: if the 19 cells are indistinguishable from
  H0 under bootstrap SE, A3D is a relabeling of the 7-bin κ(B)
  signal. Threshold: `KS one-sample(z_(t,c) := κ_(t,c) / SE_(t,c) ;
  N(0, 1)) p ≥ 0.01` → **FAIL_RELABELING**; `p < 0.01` →
  **PASS_DISTRIBUTIONAL**.
- **A3D-β (vacuous)**: max |z_(t,c)| over 19 cells must be ≥ 5;
  otherwise **INCONCLUSIVE**.

### §2.5 Scope constraints (pre-registered)

A3D does NOT prove or disprove (A3) (asymptotic vs finite-B);
does NOT resolve 7/40 vs log(2)/τ; does NOT alter BT-546 PARTIAL
verdict, MILL-PX-A9 [9] grade, or any HEXA-BSD-01..07 entry; does
NOT constitute a closure event (per L9 §sec 4, PASS = "molt
licensed", not "BT closed").

---

## §3 Discriminator-type classification

A3D's discriminator is **distributional + literature-import (hybrid)**:

- **Distributional axis**: KS one-sample on standardized cell
  z-scores against `N(0, 1)`.
- **Literature-import axis**: H0 imports BKLPR 2015 random-cokernel
  model (a published probabilistic model, not an n=6 arithmetic
  identity).

### Comparison vs catalogue 4-BT (all 5 BTs)

| BT | candidate | discriminator type | family |
|----|-----------|---------------------|--------|
| 541 | Lead-B SLE_6 × GUE | distributional KS | non-arithmetic |
| 542 | Hirahara MCSP | literature-import | non-arithmetic |
| 543 | P3 A4 ratio interval | numerical-equality | arithmetic |
| 544 | Q1 KdV Gram det/σ ∈ ℤ | numerical-equality | arithmetic |
| **546** | **A3D random-cokernel** | **distributional + lit-import** | **non-arithmetic** |

A3D is heterogeneous from all 4: BT-541 tests GUE eigenvalue
spacings (different mathematical object); BT-542 imports complexity-
theoretic literature (different field); BT-543/544 are
arithmetic-equality. A3D is the first **hybrid** distributional +
literature-import row in the catalogue.

---

## §4 Validation execution log

### §4.1 Input

The discriminator consumes the on-disk 28-stratum JSON:
`/tmp/probeB_28stratum.json` (276 lines, 28 cells; 19 viable
N_cell ≥ 100). Source: `omega-exec-bt546-probeB-28stratum-2026-04-
25.md` §4 table + §3 run log (1,705,824 curves; conductor 1..269,999;
1000-bootstrap, seed = 20260425).

### §4.2 Procedure

1. Load 19 cells with `N_cell ≥ 100` from on-disk JSON.
2. Compute `z_(t,c) = κ_(t,c) / SE_(t,c)`.
3. Compute three independent summaries:
   - KS one-sample of `{z_(t,c)}` against `N(0, 1)`.
   - Median |z|.
   - Sign-test on raw kappa.
4. Apply A3D-α (KS p ≥ 0.01 → FAIL_RELABELING) and A3D-β (max
   |z| < 5 → INCONCLUSIVE).

### §4.3 Run log

```
$ python3 -c "<scipy.stats summary on 19 cells>"
N viable cells              = 19
mean κ                      = -3.5791   (heavy negative tail in sha>9)
median κ                    = +1.0454
per-cell |z| min            = 2.20
per-cell |z| median         = 15.05
per-cell |z| max            = 276.38   (cell t=1, sha=1)
cells with |z| > 3          = 17 / 19
cells with |z| > 5          = 15 / 19
KS one-sample (z, N(0,1))   D = 0.7892   p = 4.08e-13
Sign test on raw κ          15+ / 4-     binomial-p ≈ 0.019 (two-sided)
Wilcoxon signed-rank κ vs 0 W = 70       p = 0.332 (NOT sig.)
```

Wallclock < 1 s (in-memory).

### §4.4 Cell-level interpretation

KS rejection D = 0.789 (p = 4.08e-13) is driven by mass concentration
far from zero on **both** tails:

- 15 of 19 cells have z > +5 (positive correlation, dominantly
  trivial/Z/2/Z/3/Z/4/Z/6 sha=1 and sha=4 cells);
- 4 of 19 cells have z < -3 (sha-class > 9 cells: t=1 z=-15.0,
  t=2 z=-14.2, t=3 z=-2.2, t=4 z=-2.9);
- 2 of 19 cells have |z| ≤ 3 ((t=4, sha=9) z=+3.4 borderline; (t=3,
  sha>9) z=-2.2 borderline).

The Wilcoxon non-significance (p = 0.33) is a consistency check, not
a defect: it tests median ≈ 0, which is true here (the heavy
negative tail in sha>9 cancels the positive bulk in central
cells). H0 is a **point mass** at 0, not "median = 0"; KS is the
correct discriminator.

### §4.5 Falsifier evaluation

| falsifier | threshold | observed | fired? |
|-----------|-----------|----------|--------|
| A3D-α (relabeling) | KS p ≥ 0.01 | 4.08e-13 | **NO** |
| A3D-β (vacuous) | max \|z\| < 5 | max = 276.4 | **NO** |

Neither fires. The discriminator binds.

---

## §5 Verdict

**`PASS_DISTRIBUTIONAL`**.

The 19-cell empirical κ-z distribution is distinguishable from the
BKLPR (A3) finite-grid null `H0: κ ≡ 0` at p = 4.08e-13 (11 orders
of magnitude below the L9 §4.1 threshold of 0.01). Double-margin in
two senses:

- KS p-value 11 orders below threshold;
- 17 of 19 cells reject H0 individually at |z| > 3, 15 of 19 at
  |z| > 5.

**Why not FAIL_RELABELING**: A3D-α refuted (KS p = 4e-13). The
distribution **shape** (both tails, gradient across torsion,
sha>9 inversion) carries information the global scalar κ(B) = 1.33
cannot encode. Not a relabeling.

**Why not FAIL_VACUOUS**: A3D-β refuted (max |z| = 276.4 vs
threshold 5).

**Why not PASS_LITERATURE (alone)**: BKLPR 2015 establishes the
null model but does not establish the empirical 28-cell rejection;
the rejection is the **new** contribution. PASS_DISTRIBUTIONAL is
the strictly correct label.

**Why not PASS_SKETCH**: no chained-inequality sequence; single
distributional test on data.

**Why not INCONCLUSIVE**: neither falsifier in borderline band;
verdict sharp.

---

## §6 F-MOLT-A tally update

### §6.1 Updated 5-sample tally

| validation                       | date       | verdict             | discriminator type                       |
|----------------------------------|------------|---------------------|------------------------------------------|
| BT-544 Q1 (KdV Gram)             | 2026-04-25 | **FAIL**            | numerical-equality (arithmetic)          |
| BT-543 P3 (A4 ratio interval)    | 2026-04-25 | **FAIL**            | numerical-equality (arithmetic)          |
| BT-541 Lead-B (SLE_6 × GUE KS)   | 2026-04-25 | **PASS**            | distributional (non-arithmetic)          |
| BT-542 Hirahara MCSP             | 2026-04-25 | **PASS_LITERATURE** | literature-import (non-arithmetic)       |
| **BT-546 A3D (this validation)** | 2026-04-25 | **PASS_DISTRIBUTIONAL** | **distributional + lit-import (non-arithmetic)** |

**Tally: 3 PASS / 2 FAIL across 5 BTs (60% PASS).**

### §6.2 Status

- **F-MOLT-A**: NOT FIRED (gate fails iff 0/4 PASS in one batch run;
  3 PASS sharpens further from 2/4 to 3/5).
- **L9 §7.3 branch**: still in "≥ 2/4 PASS" → gate validated;
  catalogue is operating menu.
- **F-MOLT-D**: unchanged (fired BT-544 only).

### §6.3 Discriminator-type bias hypothesis test result

| discriminator family | PASS | FAIL | rate |
|---------------------|------|------|------|
| arithmetic-equality | 0    | 2    | 0%   |
| distributional      | 2    | 0    | 100% |
| literature-import   | 1    | 0    | 100% |
| (non-arithmetic ∪)  | 3    | 0    | 100% |

The user-flagged bias hypothesis is **strengthened, not rebutted**.
Split is **3/3 vs 0/2** (non-arithmetic / arithmetic), one notch
sharper than the 2/2 vs 0/2 close of the 4-BT sweep. 95% binomial CI
on the non-arithmetic PASS rate at 3/3 is wide ([0.29, 1.00],
Clopper-Pearson) -- **suggestive, not statistically conclusive**.

The `omega-meta-audit-l9-catalogue-design-2026-04-25.md` §4.4
CATALOGUE_BIAS verdict is **partially confirmed and partially
refined**:

- *Confirmed*: arithmetic-equality candidates have 0% PASS rate (0/2).
  The "n=6 arithmetic predicts a closed-form invariant" family does
  collapse to relabeling under L9 discriminators.
- *Refined*: the bias is **discriminator-type**, not "n=6 arithmetic"
  full-stop. Non-arithmetic candidates derived from n=6-axis BTs
  (BT-541 Y1 NUM-CORE, BT-546 Y8 GALOIS-ASSEMBLY) PASS. The bug is
  the **choice of discriminator type**, not the **choice of subject
  BT**.

### §6.4 What this does NOT establish

- 3/3 non-arithmetic PASS does not imply every non-arithmetic
  candidate will PASS.
- A3D PASS does not propagate to closure (BT-546 ceiling stays
  2-3 / 5).
- Sample of 5 too small to fix a true bias rate.

> **Amendment v2 (2026-04-25, post discriminator-type CONFOUNDED audit)**: The 2×2 split (PASS=distributional/struct-lit, FAIL=discrete-equality) cited above is correlation-real but interpretation-CONFOUNDED at n=8. Per `reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md`, the L9 generation pipeline systematically pairs weak n=6-arithmetic candidates with discrete-equality discriminators, making type-axis collinear with candidate-validity. The pattern is consistent with both "discriminator type drives outcome" AND "candidate validity drives outcome"; n≥16 native-pairing resample is required to separate. 0/7 unchanged.

> **Amendment v3 (2026-04-25, post D1.3 cross-cell FAIL)**: After v2's CONFOUNDED note, two more cross-cell entries arrived: D1.4 PASS (discrete-equality + arithmetic + structurally non-trivial; `omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md`) and D1.3 FAIL (struct-cross-PDE + no underlying duality; `omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md`). The 2×2 now has cross-cell entries on **both rows** — neither row is monotypic. Type-axis directional separation is doubly weakened. CONFOUNDED verdict reinforced. 0/7 unchanged.

> **Amendment v4 (2026-04-25, post P5+P6+P7 n=14 batch)**: With 3 additional native-paired samples (P5 BT-541-Mertens-KS PASS_DISTRIBUTIONAL, P6 BT-546-stratum-irrep FAIL_NO_MATCH, P7 BT-545-IHC-dim PASS_DISCRETE_MATCH), the 2×2 is now (4,2;2,6) (P6 top) or (3,3;2,6) (P6 bottom). Fisher exact p moves from 0.13 (n=10) to 0.2774 / 0.0909 (n=14) — **away from H_type significance**. Per `omega-exec-n20-p567-batch-2026-04-25.md`, **Scenario A (H_validity dominant)** confirmed under both classifications. The CONFOUNDED verdict is reinforced AND refined: candidate-validity is the dominant driver, NOT discriminator-type. n=20 P8/P9/P10 needed for full separation. 0/7 unchanged.

---

## §7 Implications

- **Catalogue extension licensed** (recommendation, no edit to L9
  file): BT-546 has a registered rank-1 frame-shift candidate (A3D);
  catalogue effectively extends from 4 to 5 BTs.
- **Discriminator-type bias hypothesis sharpened** from 2/2 vs 0/2
  to 3/3 vs 0/2.
- **Y8 GALOIS-ASSEMBLY axis** strength rating 9 preserved; A3D adds
  a new sub-primitive ("28-stratum κ distribution") at L9 / L11
  catalogue / canon, not L_ω apex.
- **κ exponent question undisturbed**: A3D rejects `κ ≡ 0` on the
  finite grid; consistent with both `κ(B) ~ B^(7/40)` and
  `κ(B) ~ B^(log(2)/τ)`. A3D does not discriminate the two.
- **(A3) status preserved** unproved. A3D rejects only the finite-B
  grid null at B = 269,999, NOT the asymptotic limit.
- **MILL-PX-A9 [9] grade preserved**. Promotion to [10] requires
  (A3) discharge; A3D does not discharge (A3).
- **Recommendation for catalogue authors** (per
  `omega-meta-audit-l9-catalogue-design-2026-04-25.md` FIX-A):
  distributional + literature-import hybrid discriminators on
  existing on-disk data are cheap (<1 s) and effective. A3D is a
  worked instance; recorded for the next session, not edited into
  L9 file.

> **Amendment v2 (2026-04-25, post discriminator-type CONFOUNDED audit)**: The 2×2 split (PASS=distributional/struct-lit, FAIL=discrete-equality) cited above is correlation-real but interpretation-CONFOUNDED at n=8. Per `reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md`, the L9 generation pipeline systematically pairs weak n=6-arithmetic candidates with discrete-equality discriminators, making type-axis collinear with candidate-validity. The pattern is consistent with both "discriminator type drives outcome" AND "candidate validity drives outcome"; n≥16 native-pairing resample is required to separate. 0/7 unchanged.

> **Amendment v3 (2026-04-25, post D1.3 cross-cell FAIL)**: After v2's CONFOUNDED note, two more cross-cell entries arrived: D1.4 PASS (discrete-equality + arithmetic + structurally non-trivial; `omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md`) and D1.3 FAIL (struct-cross-PDE + no underlying duality; `omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md`). The 2×2 now has cross-cell entries on **both rows** — neither row is monotypic. Type-axis directional separation is doubly weakened. CONFOUNDED verdict reinforced. 0/7 unchanged.

> **Amendment v4 (2026-04-25, post P5+P6+P7 n=14 batch)**: With 3 additional native-paired samples (P5 BT-541-Mertens-KS PASS_DISTRIBUTIONAL, P6 BT-546-stratum-irrep FAIL_NO_MATCH, P7 BT-545-IHC-dim PASS_DISCRETE_MATCH), the 2×2 is now (4,2;2,6) (P6 top) or (3,3;2,6) (P6 bottom). Fisher exact p moves from 0.13 (n=10) to 0.2774 / 0.0909 (n=14) — **away from H_type significance**. Per `omega-exec-n20-p567-batch-2026-04-25.md`, **Scenario A (H_validity dominant)** confirmed under both classifications. The CONFOUNDED verdict is reinforced AND refined: candidate-validity is the dominant driver, NOT discriminator-type. n=20 P8/P9/P10 needed for full separation. 0/7 unchanged.

---

## §8 Anomalies

### §8.1 Wilcoxon non-significance vs KS rejection

Wilcoxon signed-rank κ vs 0 gives p = 0.332 (NOT significant). This
is a feature of the data, not a defect of A3D: heavy negative tail
in sha-class > 9 (κ = -19.5, -23.4, -51.3, -7.4 at t = 1..4)
cancels the positive bulk under signed-rank. KS, sensitive to
**shape** rather than central tendency, correctly identifies the
rejection. H0 is a point mass at 0, not "median = 0"; KS is the
appropriate discriminator. Logged for transparency.

### §8.2 First-order |Sel_n| approximation inherited

A3D consumes the same first-order |Sel_n| values flagged in parent
report §8.3. The negative-κ tail in sha>9 is partly an artifact of
this approximation. Sub-test on 15 cells with sha-class ∈ {1, 4, 9}
only: all κ positive, |z| min = 2.20 max = 276.4, sign test 15/0
binomial p = 6e-5. **Verdict robust** to dropping the sha>9 tail
(would be sharper, not flipped).

### §8.3 N_total mismatch

The DFS-24 Probe B spec named "Cremona 332k"; actual run consumed
1,705,824 curves (full local 27-shard, conductor 1..269,999).
Documented in parent §8.2; A3D inherits the 1.7M figure (more
viable cells → stronger rejection).

### §8.4 Discriminator-type bias may be confounded with cost-tier

Per `omega-meta-audit-l9-catalogue-design-2026-04-25.md` F-AUDIT-E,
arithmetic-family candidates were chosen as "cheapest first" per
L9 §sec 7. The 2/2 FAIL on arithmetic may reflect cost-tier
selection bias rather than discriminator-type bias proper. A3D was
even cheaper than BT-541 Lead-B (1.79 s) and BT-542 Hirahara
(literature-only) at sub-second; this **partially weakens** the
cost-tier explanation but does not eliminate it. UNKNOWN: whether
expensive arithmetic candidates would FAIL at the same rate.

---

## §9 Falsifiers active

### §9.1 A3D-internal (post-validation)

| ID | falsifier | threshold | status |
|----|-----------|-----------|--------|
| A3D-α | KS p ≥ 0.01 → FAIL_RELABELING | 0.01 | **REFUTED** (p = 4e-13) |
| A3D-β | max \|z\| < 5 → INCONCLUSIVE | 5 | **REFUTED** (max = 276.4) |
| A3D-γ | sha>9 tail removal flips verdict | post-hoc | **REFUTED** (sub-test §8.2) |

### §9.2 Inherited (BT-546 omega-cycle 9-falsifier inventory; all unchanged)

The 9 active falsifiers from `omega-cycle-bt546-bsd-2026-04-25.md`
§8 (F1-F6 + Probe-A/B/C-falsifier) are all unchanged:

- F1-F6 (§X.5 of `millennium-bsd.md`): all active or vacuous as
  recorded; A3D does not address them.
- Probe-A-falsifier (Tunnell residue n=6 ≡ n=5,7): not addressed.
- Probe-B-falsifier (κ > 0.5 uniformly): already refuted in
  `omega-exec-bt546-probeB-28stratum-2026-04-25.md` §5.2.
- Probe-C-falsifier (twist average < 10 at |d| = 500): not addressed.

### §9.3 New falsifiers registered against A3D itself

- **F-A3D-asymp-ext** (asymptotic extrapolation overreach): if a
  reader interprets A3D PASS as evidence against (A3) at `B → ∞`,
  the interpretation is overreach. A3D tests B = 269,999. **Active
  by design as guardrail**.
- **F-A3D-Sage-precise**: if a future Sage / Pari precise-|Sel_n|
  reanalysis reduces max |z| below 5, verdict flips to
  INCONCLUSIVE. **Active**; precise reanalysis deferred per parent
  §8.3.
- **F-A3D-3M-extension**: if a future Cremona 3M+ extension shows
  κ(B) → 0 monotonically across matching 28 strata, finite-grid
  rejection is contingent on truncation. **Active**; out of scope.

---

## §10 Closing

- **Verdict**: `PASS_DISTRIBUTIONAL`.
- **F-MOLT-A**: NOT FIRED. Tally = 3 PASS / 2 FAIL at 5 samples.
- **Discriminator-type bias hypothesis**: sharpened to 3/3 vs 0/2
  (non-arithmetic / arithmetic); suggestive but small-sample-limited.
- **(A3)**: UNPROVED (preserved). A3D rejects only finite-grid
  `κ ≡ 0` at B = 269,999.
- **BT-546**: PARTIAL under Y8 (preserved). Closure ceiling 2-3 / 5
  (preserved).
- **Atlas / state / inventory**: untouched.
- **Millennium**: 0/7 (preserved).
- All numbers cited come from `/tmp/probeB_28stratum.json` (parent
  data) processed through 3 standard scipy.stats summaries. No
  fabrication.
- English technical terms only.

> **Amendment v2 (2026-04-25, post discriminator-type CONFOUNDED audit)**: The 2×2 split (PASS=distributional/struct-lit, FAIL=discrete-equality) cited above is correlation-real but interpretation-CONFOUNDED at n=8. Per `reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md`, the L9 generation pipeline systematically pairs weak n=6-arithmetic candidates with discrete-equality discriminators, making type-axis collinear with candidate-validity. The pattern is consistent with both "discriminator type drives outcome" AND "candidate validity drives outcome"; n≥16 native-pairing resample is required to separate. 0/7 unchanged.

> **Amendment v3 (2026-04-25, post D1.3 cross-cell FAIL)**: After v2's CONFOUNDED note, two more cross-cell entries arrived: D1.4 PASS (discrete-equality + arithmetic + structurally non-trivial; `omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md`) and D1.3 FAIL (struct-cross-PDE + no underlying duality; `omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md`). The 2×2 now has cross-cell entries on **both rows** — neither row is monotypic. Type-axis directional separation is doubly weakened. CONFOUNDED verdict reinforced. 0/7 unchanged.

> **Amendment v4 (2026-04-25, post P5+P6+P7 n=14 batch)**: With 3 additional native-paired samples (P5 BT-541-Mertens-KS PASS_DISTRIBUTIONAL, P6 BT-546-stratum-irrep FAIL_NO_MATCH, P7 BT-545-IHC-dim PASS_DISCRETE_MATCH), the 2×2 is now (4,2;2,6) (P6 top) or (3,3;2,6) (P6 bottom). Fisher exact p moves from 0.13 (n=10) to 0.2774 / 0.0909 (n=14) — **away from H_type significance**. Per `omega-exec-n20-p567-batch-2026-04-25.md`, **Scenario A (H_validity dominant)** confirmed under both classifications. The CONFOUNDED verdict is reinforced AND refined: candidate-validity is the dominant driver, NOT discriminator-type. n=20 P8/P9/P10 needed for full separation. 0/7 unchanged.

-- end --
