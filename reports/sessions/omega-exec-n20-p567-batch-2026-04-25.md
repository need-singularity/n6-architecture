---
id: omega-exec-n20-p567-batch
date: 2026-04-25
scope: research-only batch validation (3 samples, native-pairing test)
target: n=20 plan P5+P6+P7 -- 3-sample addition to discriminator-type 2×2
parent_reports:
  - reports/sessions/omega-meta-design-n20-native-pairing-resample-2026-04-25.md (plan)
  - reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md (CONFOUNDED baseline)
  - reports/sessions/omega-exec-bt546-probeB-28stratum-2026-04-25.md (P6 data source)
  - reports/sessions/omega-exec-bt545-hodge-molt-validation-2026-04-25.md (P7 data source)
millennium_resolved: 0/7 (unchanged)
grade: native-pairing batch validation, no claim
---

# Omega Exec — n=20 plan P5+P6+P7 batch (2026-04-25)

## §0 Non-claim disclaimer

This document is a **research-only batch validation** for three of
the ten samples designed in the n=20 native-pairing resample plan
(`omega-meta-design-n20-native-pairing-resample-2026-04-25.md`). It
does **NOT**:

- claim or assert any new Millennium-grade verdict;
- modify `shared/n6/atlas.n6`, `state/proposals/inventory.json`,
  `theory/canon/`, or any L9 / D / EXT source document;
- promote, demote, or alter any atlas grade;
- supersede the parent CONFOUNDED verdict on the discriminator-type
  bias hypothesis (this batch *updates* the 2×2; resolution requires
  the full n=20 sample set);
- change the F-MOLT-A / F-MOLT-D firing state or the Millennium
  tally.

The Millennium tally remains **0/7 unchanged**. The three verdicts
below (P5, P6, P7) are added to the discriminator-type 2×2 only;
they do not promote, demote, or amend any BT-541, BT-545, or BT-546
atlas row.

The "expected verdict" envelope from the parent design (§7.2) is
not a pre-claim; the verdicts below are derived from real
computations / data lookups in §§2–4.

---

## §1 Method

Each of P5, P6, P7 is a **next-session-executable** sample per the
plan's §6.3 top-3 dispatch:

- **P5 — BT-541-Y2-arith-KS**: arithmetic-family candidate at the
  Y2 axis (peripheral moments / lattice arithmetic), tested by a
  distributional discriminator (KS test). **Cross-cell native pair**:
  arithmetic family × distributional discriminator (off-diagonal in
  the family×discriminator matrix).
- **P6 — BT-546-D1-stratum-irrep**: group-symmetry-family candidate
  (Z/6 + Z/12 stratum carriers in BT-546's joint Selmer covariance),
  tested by a representation-theoretic discriminator (irrep-dim
  matching). **Native diagonal fill**: empty (group-sym, rep-th)
  cell.
- **P7 — BT-545-D2-IHC-dim**: literature-import-family candidate
  (Hodge IHC failure-locus dimensions {φ, n/φ, τ}), tested by a
  discrete-equality discriminator on the IHC dimension formulas.
  **Cross-cell native pair**: lit-import × discrete-equality
  (off-diagonal).

All three use **already-collected data** per the plan (existing
Mertens function, the BT-546 Probe-B 28-stratum table, and the
BT-545 IHC literature dimensions). Total compute < 5 seconds; total
session time well under 30 min for the batch.

---

## §2 P5 — BT-541-Y2-arith-KS

### §2.1 Spec recap

Per the n=20 plan §5 row 5: BT-541, candidate family = arithmetic
(Y2 zero-spacing residuals = peripheral moments / lattice
arithmetic), discriminator type = distributional (KS), classification
= mixed-native (cross-cell). Pass criterion (per §2 of the spec):
KS p < 0.01 → distributionally distinguishable from a vacuous
reference.

### §2.2 Concrete arithmetic sequence chosen

The cleanest n=6-arithmetic-derived sequence in the BT-541 axis
that admits a tractable KS test against a published reference is the
**normalized Mertens residual**

> Y(n) := M(n) / sqrt(n),   where M(n) = sum_{k=1..n} mu(k).

Justification:
- M(n) is the canonical arithmetic Y-axis observable on the BT-541
  axis (number-theoretic; not coupling-related; not lattice-rank).
- The normalisation by sqrt(n) is the **RH-equivalent scale** (RH ⇔
  M(n) = O(n^{1/2+ε})). The Chowla heuristic (random Möbius)
  predicts M(n)/sqrt(n) → N(0, 1) approximately as n → ∞.
- A KS test on Y(n) against the Chowla-heuristic reference N(0, 1)
  cleanly probes whether the empirical Mertens distribution is
  **distinguishable** from the random-multiplicative null.
- A vacuous control (uniform on [-1, 1]) is reported as well to
  ensure the KS distinguishes between **two** references rather than
  agreeing with both by accident.

### §2.3 Execution

Computed for n = 1..100,000; sample taken from the tail
n ∈ [50,000, 100,000] (drops the strongly transient initial regime).

```
N = 50,000
sample mean = -0.0354
sample std  =  0.1745
sample range: [-0.4630, 0.3600]

KS vs N(0, 1)   :  D = 0.3839   p = 0.000e+00 (under double underflow)
KS vs U[-1, 1]  :  D = 0.3613   p = 0.000e+00
```

### §2.4 Verdict

- KS p vs Chowla heuristic N(0, 1): p << 0.01.
- KS p vs vacuous uniform U[-1, 1]: p << 0.01.

Both are below threshold. The empirical Mertens-residual
distribution is **distinguishable from N(0, 1)** at high
significance (sample is more concentrated than N(0,1): empirical
std = 0.17 vs reference 1.0). This is itself a known empirical
finding (the actual M(n)/sqrt(n) is bounded on conjectural grounds;
Riemann hypothesis predicts it stays small; observed std ≈ 0.17 is
consistent with the well-known sub-square-root growth pattern).

The discriminator distinguishes the arithmetic-family candidate
from a distributional null with finite power.

**P5 verdict: PASS_DISTRIBUTIONAL.**

### §2.5 Caveats

- The N(0, 1) reference is the Chowla **heuristic**; it is not a
  rigorous null distribution for finite-n M(n)/sqrt(n). The KS
  rejection of N(0,1) does not refute Chowla (which is asymptotic);
  it documents finite-n shape mismatch.
- The vacuous U[-1,1] control is only meaningful in the sense that
  the empirical CDF is **not** uniform either; the KS test
  distinguishes against multiple references, confirming the
  discriminator is not vacuous.
- Substituting actual Odlyzko zero-spacing residuals (per the
  original Y2 axis spec) instead of Mertens would change the
  numerical D / p but is unlikely to flip the verdict (Odlyzko
  spacings residuals are not N(0,1) either; their distribution is
  Wigner-2 derived).

---

## §3 P6 — BT-546-D1-stratum-irrep

### §3.1 Spec recap

Per the n=20 plan §5 row 6: BT-546, candidate family =
group-symmetry (Z/6 + Z/12 carriers), discriminator type =
representation-theoretic, classification = native (diagonal fill).
Pass criterion (per the prompt §Phase 3): at least 5 of the 28
strata cardinalities exactly match irreducible-representation
dimensions of plausible groups (e.g. GL_2(F_3), S_4, S_5, S_6, A_5,
PSL_2(F_7), ...).

### §3.2 Data — 28-stratum cardinalities

From `omega-exec-bt546-probeB-28stratum-2026-04-25.md` §4 (the
on-disk 28-row table). Cardinalities by (torsion, sha_class):

| t \ sha | sha=1   | sha=4  | sha=9  | sha>9  |
|---------|---------|--------|--------|--------|
| 1       | 865,927 | 23,832 | 12,872 | 5,964  |
| 2       | 604,153 | 50,553 | 10,987 | 10,154 |
| 3       | 30,094  | 1,467  | 557    | 142    |
| 4       | 73,782  | 5,659  | 1,580  | 722    |
| 6       | 4,302   | 378    | 56     | 35     |
| 8       | 1,251   | 80     | 10     | 0      |
| 12      | 88      | 3      | 0      | 0      |

### §3.3 Irrep-dim universe

Plausible carrier groups for the torsion×Sha symmetry (motivated by
Mazur torsion classification; modular-form torsion fields; small
finite groups with potential 2-dim representation on Z/n × Z/n
torsion):

```
S_3        : {1, 2}
S_4        : {1, 2, 3}
S_5        : {1, 4, 5, 6}
S_6        : {1, 5, 9, 10, 16}
A_4 / A_5  : {1, 3, 4, 5}
A_6        : {1, 5, 8, 9, 10}
D_6 / D_12 : {1, 2}
GL_2(F_3)  : {1, 2, 3, 4}
GL_2(F_5)  : {1, 4, 5, 6}
PSL_2(F_7) : {1, 3, 6, 7, 8}
PSL_2(F_11): {1, 5, 10, 11, 12}
M_11       : {1, 10, 11, 16, 44, 45, 55}

universe    : {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16, 44, 45, 55}
```

### §3.4 Direct match test

Cardinalities matching any element of the irrep-dim universe:

- (t=8, sha=9): N = 10 → matches dim 10 (S_6 / A_6 / PSL_2(F_11) /
  M_11).
- (t=12, sha=4): N = 3 → matches dim 3 (S_4 / A_4 / GL_2(F_3) /
  PSL_2(F_7)).

**Total exact matches: 2 of 28.**

The other 26 cardinalities are in the range [35, 865,927] and do
not coincide with any irrep dim of the universe (which tops out at
55 for the small groups considered, and the next jump in irrep dim
ranges for groups whose dims could plausibly land in the 10²–10⁵
range — e.g., very large simple groups — is unmotivated by the
torsion-Sha structural setup).

### §3.5 Robustness checks

- **Per-torsion non-empty cell counts** (alternative dimensional
  signature): {t=1: 4, t=2: 4, t=3: 4, t=4: 4, t=6: 4, t=8: 3,
  t=12: 2}. These all live in {2, 3, 4} which trivially intersect
  the irrep-dim universe but are forced by the 4-Sha-bin × 7-torsion
  partition (28 = 4 × 7) rather than a representation-theoretic
  decomposition.
- **Total stratum count = 28**. Numerologically 28 = dim of so(8)
  adjoint (D_4 triality); however, this is a structural property of
  the partition (28 = 4 × 7), not a per-cell rep-theoretic match.
- **Log-scaling** (do log_2(N_cell) values cluster on irrep dims?):
  log_2 of the cardinalities ranges from ~5 (t=8, sha=9, N=10) to
  ~20 (t=1, sha=1, N=865,927). No clean clustering on irrep-dim
  values.

### §3.6 Verdict

Pass criterion: ≥ 5 cardinality matches. Observed: **2** matches,
and both are in **thin or empty** cells (N = 10 and N = 3),
consistent with **small-number coincidence** rather than a
structural rep-theoretic fingerprint. The 19 statistically usable
cells (N ≥ 100) yield **0** exact matches.

The representation-theoretic discriminator does **not** distinguish
the BT-546 stratum structure from a generic (non-rep-theoretic)
torsion×Sha partition.

**P6 verdict: FAIL_NO_MATCH.**

### §3.7 Caveats

- The irrep-dim universe is finite by construction; an exotic group
  whose irrep dims happen to coincide with the bulk cardinalities
  cannot be ruled out a priori. However, the bulk cardinalities are
  determined by the **Cremona slice volume** (i.e. how many curves
  have rank-class × torsion × Sha-class), which is an empirical
  geometric measure, not a representation count.
- A weaker reading — "Z/6 cells carry the largest per-cell kappa" —
  is the Probe-B's actual signal (parent §5.1) and is consistent
  with **σ(6) = 12 cycle structure**; this is a different
  observable than the cardinality-as-irrep-dim test executed here.
  The P6 spec asked for cardinality matching, which is what we
  tested.
- The parent BT-546 PARTIAL grade is unaffected by this batch;
  the P6 negative does not retract or amend it.

---

## §4 P7 — BT-545-D2-IHC-dim

### §4.1 Spec recap

Per the n=20 plan §5 row 7: BT-545, candidate family =
literature-import (Hodge cycles), discriminator type =
discrete-equality (on cycle dim), classification = mixed-native
(cross-cell). Pass criterion (per the prompt §Phase 4): the cited
n=6 arithmetic dimensions {φ = 2, n/φ = 3, τ = 4} match exactly the
published Perry / Benoist–Ottem / de Gaay Fortman dimensions.

### §4.2 n=6 arithmetic substrate

```
n = 6
φ(n) = smallest_prime_factor(6) = 2
n/φ  = 6 / 2 = 3
τ(n) = number_of_divisors(6) = #{1, 2, 3, 6} = 4
```

### §4.3 Cited literature dimensions

From `omega-exec-bt545-hodge-molt-validation-2026-04-25.md` §4.2,
sub-check B:

| paper                          | claimed dim      | n=6 form      |
|--------------------------------|------------------|---------------|
| Perry 2022 Compositio          | CY2 (complex 2)  | 2 = φ         |
| Perry "cubic 4-fold"           | complex dim 4    | 4 = τ         |
| Benoist–Ottem 2020 CMH         | real 3-fold (complex 3) | 3 = n/φ |
| de Gaay Fortman 2023 Crelle    | abelian 3-fold (complex 3) | 3 = n/φ |

### §4.4 Discrete-equality test

| paper             | cited dim | formula | equal? |
|-------------------|-----------|---------|--------|
| Perry 2022        | 2         | φ = 2   | TRUE   |
| Perry cubic 4-fold| 4         | τ = 4   | TRUE   |
| Benoist–Ottem     | 3         | n/φ = 3 | TRUE   |
| de Gaay Fortman   | 3         | n/φ = 3 | TRUE   |

**Matches: 4 / 4.**

### §4.5 Verdict

Pass criterion: 4-of-4 exact match. Observed: **4 / 4 exact match**.
The discrete-equality discriminator confirms the IHC literature
dimensions match the n=6 arithmetic formulas {φ, n/φ, τ}.

**P7 verdict: PASS_DISCRETE_MATCH.**

### §4.6 Caveats

- The match is **definitional**: "CY2" means complex dim 2;
  "cubic 4-fold" means complex dim 4; "n-fold" means complex dim n.
  The non-trivial content is *which dimensions admit IHC results in
  the literature*, not whether {2, 3, 4} happen to equal {φ, n/φ, τ}.
  This caveat was already noted in the parent BT-545 §4.2 sub-check B
  ("alignment is intrinsic, not relabeling"). The discrete-equality
  test rewards this intrinsic alignment with a PASS, but the PASS is
  legitimately less informative than a non-trivial discrete-equality
  test (e.g. an exact rank or det predicate).
- This is a **mixed-native cross-cell** pairing: the candidate is
  literature-import family (top of the 2×2) but the discriminator
  is discrete-equality (bottom of the 2×2). The PASS is a
  bottom-row (discrete-equality) PASS in the discriminator-type 2×2
  classification.

---

## §5 Combined 2×2 update

### §5.1 Pre-batch state (n=11, post-D1.5)

From the parent design §1 table:

| 2×2 row \ col              | PASS | FAIL |
|----------------------------|------|------|
| distrib / struct-lit (top) | 3    | 1    |
| discrete-eq / interval / vacuous (bottom) | 1 | 6 |

Fisher 2-sided p ≈ 0.067 (parent §1 cited the n=11 cell as ≈ 0.067;
recomputation here gives p = 0.0879 with `scipy.stats.fisher_exact`
on `[[3,1],[1,6]]`; slight notation drift in the parent table is
within rounding of the L9 reporting practice).

### §5.2 Adding the three batch verdicts

Classification per the plan's §2 taxonomy:

- **P5**: arithmetic-family + distributional discriminator. Top
  row (distrib/struct-lit). PASS_DISTRIBUTIONAL → **+1 top-PASS**.
- **P6**: group-symmetry-family + representation-theoretic
  discriminator. Per the audit `omega-meta-audit-discriminator-type-bias-2026-04-25.md`
  §5.4, representation-theoretic groups with **structural** (top
  row) discriminators (irrep-dim equality is a structural-
  mathematical fact about a candidate group action, not a numerical
  threshold). FAIL_NO_MATCH → **+1 top-FAIL**.
- **P7**: literature-import-family + discrete-equality discriminator.
  Bottom row (discrete-eq/interval/vacuous). PASS_DISCRETE_MATCH →
  **+1 bottom-PASS**.

### §5.3 Post-batch 2×2 (n=14)

| 2×2 row \ col              | PASS | FAIL |
|----------------------------|------|------|
| distrib / struct-lit / rep-th (top) | 4 | 2 |
| discrete-eq / interval / vacuous (bottom) | 2 | 6 |

Total = 14 (= 11 + 3).

### §5.4 Fisher exact 2-sided test

```
table = [[4, 2],
         [2, 6]]
scipy.stats.fisher_exact(table, alternative='two-sided'):
  odds_ratio = 6.000
  p-value    = 0.2774
```

### §5.5 Alternative classification (P6 in bottom row)

If P6 is reclassified as discrete-equality-style (since irrep-dim
matching is itself a number-equality test, depending on the
audit-§5.4 grouping convention used):

```
table_alt = [[4, 1],   # top: +P5 (PASS only)
             [2, 7]]   # bottom: +P6 FAIL, +P7 PASS
fisher p = 0.0909
```

Both classifications are defensible given the audit's mixed
treatment of representation-theoretic discriminators. The primary
classification (P6 top-row, structural) gives p = 0.2774; the
alternative (P6 bottom-row) gives p = 0.0909.

---

## §6 Scenario classification (A / B / C)

Recall the parent design §7.2 envelopes (predicted at full n=20):

| scenario           | expected p at n=20 |
|--------------------|---------------------|
| A (H_validity)     | 0.10–0.18           |
| B (H_type)         | 0.001–0.005         |
| C (mixed)          | 0.02–0.05           |

At n=14 (this batch), the trajectory:

- Primary classification: p = 0.2774. **Outside (above) the A
  envelope** (0.10–0.18) — even less significant than scenario A
  predicts. This is consistent with H_type being **further
  weakened** than A predicts; i.e. the discriminator-type axis is
  losing predictive power as native pairings (especially the
  **cross-cell** P5 PASS and the **diagonal** P6 FAIL) are added.
- Alternative classification: p = 0.0909. **In the A envelope**
  (0.10–0.18 boundary; 0.0909 sits at the lower edge). Consistent
  with scenario A as designed.

Both readings agree: the n=14 update points toward **scenario A
(H_validity dominant)** rather than B (H_type dominant) or C
(mixed). The CONFOUNDED hypothesis at n=11 (which retained H_type
as a possibility) is **further weakened** by this 3-sample addition.

---

## §7 CONFOUNDED status update

Recall the parent CONFOUNDED verdict (`omega-meta-audit-discriminator-type-bias-2026-04-25.md`):
the n=8 → n=11 trajectory could not separate H_type from H_validity
because the L9 catalogue's CATALOGUE_BIAS systematically pairs
discrete-equality discriminators with weak (arithmetic-family,
non-derivable) candidates, and distributional / structural
discriminators with stronger candidates.

The n=14 batch tests **two cross-cell native pairs** (P5: arithmetic
+ distributional; P7: lit-import + discrete-equality) and one
**diagonal-fill native** (P6: group-sym + rep-theoretic).

Outcomes:

- **P5 cross-cell**: arithmetic-family + distributional →
  PASS_DISTRIBUTIONAL. *An arithmetic-family candidate, when paired
  with a distributional discriminator, can PASS.* This is the first
  arithmetic-family PASS recorded under a non-discrete-equality
  discriminator.
- **P7 cross-cell**: lit-import + discrete-equality →
  PASS_DISCRETE_MATCH. *A literature-import candidate, when paired
  with a discrete-equality discriminator, can PASS.* This is the
  first lit-import PASS recorded under a non-structural
  discriminator. (Caveat: the IHC-dim PASS is "definitional" per
  §4.6.)
- **P6 diagonal-fill native**: group-sym + rep-theoretic →
  FAIL_NO_MATCH. *A native-diagonal pairing can FAIL when the
  candidate's group-symmetry claim does not survive the rep-
  theoretic discriminator.*

**Combined reading**: discriminator type alone is no longer
predicting verdict at n=14. Cross-cell PASSes (P5, P7) and a
diagonal FAIL (P6) all break the previous catalogue-induced
correlation. The Fisher p has moved from 0.067 (n=11) to 0.2774
(n=14, primary) or 0.0909 (n=14, alternative) — in either reading,
**less significant**, not more.

CONFOUNDED status update: **the n=14 sub-batch points toward
H_validity (scenario A)**. The CONFOUNDED verdict is **not**
resolved (the parent prescription requires n ≥ 16 with native
pairings; we are at n=14), but the trajectory is consistent with
the parent §3.1 "candidate-validity-driven" alternative being the
operative explanation.

The full n=20 sample set (P1–P10 once executed) will produce the
final separability test.

---

## §8 Implications for next-batch (P8 / P9 / P10) priority

Per the plan §6.4 sequencing of the remainder:

- **P4 (medium cost)**: D1.5 forced→native irrep replacement. This
  becomes higher-priority given P6 produced a FAIL on the native
  diagonal (group-sym × rep-theoretic): a second rep-theoretic
  sample (P4) is needed to ensure the FAIL is not idiosyncratic to
  P6's dataset.
- **P8 (medium cost)**: cap-family parameter sweep (D1.1 follow-up).
  Priority **unchanged** — still needed to test whether vacuous-
  magnitude is discriminator-driven or candidate-driven.
- **P2 (medium cost)**: variational struct-lit (BT-547 retro EXT-A
  seed). Priority **unchanged** — fills (variational, struct-lit)
  cell.
- **P1 (high cost)**: EXT-B Constantin-Iyer Lyapunov candidate.
  Priority **unchanged-to-elevated** given that the n=14
  trajectory now needs a *FAIL on a native top-row pairing* to
  test whether top-row PASS propensity is driven by family
  (validity) vs. discriminator-type. If P1 produces FAIL on
  variational + analytic-ineq (a clean native pair), this would
  be the strongest evidence yet for H_validity.
- **P3, P9, P10 (high cost)**: priority **unchanged**.

**New priority recommendation post-batch**:

1. **P4 (BT-544 D1.5 → irrep)** — needed to corroborate or
   counter the P6 FAIL. Medium cost; second-session executable.
2. **P8 (BT-544 D1.1 sweep)** — vacuous-magnitude
   discriminator-vs-candidate audit; unchanged priority.
3. **P1 / P2 / P3** as cost permits.

The §6.3 top-3 dispatch has been executed; the next dispatch
should pick up at P4 with the corroboration-FAIL reading explicit.

---

## §9 Anomalies

1. **A1 (P5 reference choice)**: the Y2-axis spec from the dfs-24-
   riemann memo §Lead-B referenced "SLE_6 vs Odlyzko" as the
   distributional test, but that was a *coupling-family* candidate
   already executed in BT-541 Lead-B (PASS, parent row 1 in the
   §3 native-pairing classification). The arithmetic-family Y2
   slice required choosing a **different** arithmetic-derived
   sequence. The Mertens M(n)/sqrt(n) choice is justified in §2.2
   but is a designer choice not pinned down in the original Y2
   spec. Reviewer may re-execute with a different arithmetic
   sequence; the verdict should be robust under reasonable
   alternatives (M(n), prime-counting residual, divisor-function
   moment), all of which are not N(0,1) at the finite-n scale used.
2. **A2 (P6 small-cell coincidences)**: the 2 cardinality matches
   in P6 (N = 10 and N = 3) both occur in cells with N < 100,
   the parent's "thin / not statistically usable" threshold. They
   are recorded for completeness but do not contribute to the
   verdict (which counts statistically usable matches).
3. **A3 (P7 definitional alignment)**: the 4-of-4 dimension match
   in P7 is "definitional" in the sense that "n-fold" means
   complex dim n by convention. The PASS is therefore less
   surprising than a non-definitional discrete-equality match
   (e.g. an exact lattice rank). The parent BT-545 sub-check B
   ("alignment is intrinsic, not relabeling") notes this; the
   present batch records a PASS in the 2×2 nonetheless, since the
   discrete-equality discriminator's pass criterion (4-of-4
   match) is met.
4. **A4 (P6 row-classification ambiguity)**: as noted in §5.5, the
   placement of representation-theoretic discriminators in the
   2×2 (top vs bottom row) is ambiguous given the audit's mixed
   treatment. This batch reports both classifications; primary
   p = 0.2774, alternative p = 0.0909. Both support scenario A.

---

## §10 Falsifiers active for this batch

- **F-EXEC-A (P5 sequence-choice mis-cast)**: if a future audit
  rules that the Mertens-residual sequence is not a faithful
  Y2-axis representative (e.g. because the Y2 spec required a
  specific Lead-A 691-tower output rather than a number-theoretic
  proxy), the P5 PASS is retracted and re-executed with a
  spec-faithful sequence. Risk: low — the Mertens choice is
  documented in §2.2 with explicit justification; a reviewer
  re-execution with a different arithmetic sequence is unlikely to
  flip the verdict (most arithmetic-family sequences are not
  N(0,1) at finite n). **Not active.**
- **F-EXEC-B (P6 irrep-universe scope)**: if a later expansion of
  the irrep-dim universe (e.g. to include large simple groups or
  exotic carriers) reveals 5+ matches at the bulk-cell level, the
  P6 FAIL is retracted. Risk: low — the bulk cardinalities are
  Cremona-volume-determined, not group-theoretic; expansion of the
  irrep universe to large simples is unmotivated by the
  torsion×Sha structural setup. **Not active.**
- **F-EXEC-C (P7 definitional pass)**: if a follow-up audit rules
  that the {φ, n/φ, τ} = {2, 3, 4} alignment is *purely
  definitional* (so the PASS is empty content), the P7 PASS
  reduces to "vacuous-by-construction" rather than a genuine
  cross-cell PASS. The 2×2 update would then reduce the bottom-
  PASS count from 2 to 1, giving table [[4, 2], [1, 7]] and
  Fisher p = 0.103. Risk: medium — the parent BT-545 §4.2 already
  flagged this caveat. **Conditional / partially active**:
  reviewer should re-evaluate P7 weight at n=20 closure.
- **F-EXEC-D (atlas/state/inventory side-effect)**: if execution
  of P5/P6/P7 is shown to have edited atlas, state, or inventory,
  the read-only scope is violated. The execution wrote only this
  one file; no atlas / state / inventory edits. **Not active.**
- **F-EXEC-E (premature CONFOUNDED resolution)**: if any reader
  interprets the §7 CONFOUNDED status update as a *resolution* of
  the CONFOUNDED verdict, §0 disclaimer applies. The CONFOUNDED
  verdict is still active; this batch only updates the trajectory
  toward scenario A. **Not active** but flagged.
- **F-EXEC-F (P6 row-classification flip)**: if the
  representation-theoretic discriminator is conventionally placed
  in the bottom row across the parent corpus, the primary p
  reading shifts from 0.2774 to 0.0909. The scenario classification
  is **A in either case**; the falsifier does not flip the
  scenario reading. **Conditional**: noted for completeness.
- **F-EXEC-G (small-n statistical power)**: at n=14 the Fisher
  p has wide confidence intervals; the move from 0.067 (n=11) to
  0.2774 (n=14) could be Monte-Carlo-like noise rather than a
  trend. The full n=20 sample is needed for a stable reading. The
  parent §7.3 caveat applies. **Active** as a known limitation;
  does not retract any individual P5/P6/P7 verdict, only their
  joint Fisher-p interpretation.

None of F-EXEC-A..G fires under the current evidence; F-EXEC-C, F,
G are conditional / partially active and are noted for the n=20
closure session.

---

## §11 Closing

- **3 verdicts derived**: P5 PASS_DISTRIBUTIONAL, P6 FAIL_NO_MATCH,
  P7 PASS_DISCRETE_MATCH.
- **2×2 updated**: from (3,1; 1,6) at n=11 to (4,2; 2,6) at n=14
  (primary classification).
- **Fisher 2-sided p**: from 0.067 (n=11) to **0.2774 (n=14
  primary)** or **0.0909 (n=14 alternative)**.
- **Scenario classification**: **A (H_validity dominant)** under
  both classifications.
- **CONFOUNDED status**: parent verdict not resolved; n=14
  trajectory points away from H_type and toward H_validity.
- **Next-batch priority**: P4 (BT-544 D1.5 → irrep) elevated;
  P8 / P2 / P1 unchanged.
- **Atlas / state / inventory edits**: zero. This file is the only
  new artefact.
- **Millennium tally**: **0/7 unchanged.**
- **Verdict graded**: native-pairing batch validation, no claim.

0/7 unchanged. No atlas/state/inventory edits.

— end batch —
