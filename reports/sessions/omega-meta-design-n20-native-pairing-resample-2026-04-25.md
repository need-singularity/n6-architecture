---
id: omega-meta-design-n20-native-pairing-resample
date: 2026-04-25
scope: methodology design (NOT executing the resample; planning n=20 sample set)
target: type-axis separability -- n=20 native-pairing resample design
parent_reports:
  - reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md (CONFOUNDED)
  - reports/sessions/omega-amend-d13-cross-cell-2026-04-25.md (n=10 update)
  - reports/sessions/omega-status-fmolt-unified-tally-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d1-5-group-symmetry-2026-04-25.md (D1.5 FAIL → n=11)
  - reports/sessions/omega-meta-audit-l9-generation-pipeline-2026-04-25.md (CATALOGUE_BIAS upstream)
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (EXT-A/B/C/D classes)
  - reports/sessions/omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md (EXT-B candidate seed)
millennium_resolved: 0/7 (unchanged)
grade: methodology design, no claim
---

# Omega Meta-Design — n=20 Native-Pairing Resample Plan (2026-04-25)

## §0 Non-claim disclaimer

This document is a **methodology design only**. It does **NOT**:

- execute any new molt-validation, dispatch, or sketch;
- claim or pre-claim any verdict (PASS, FAIL, ACCEPTABLE) for any of
  the proposed n=20 samples;
- modify `shared/n6/atlas.n6`, `state/proposals/inventory.json`,
  `theory/canon/`, or any L9 / D / EXT source document;
- promote, demote, or alter any atlas grade;
- supersede the parent CONFOUNDED verdict on the discriminator-type
  bias hypothesis;
- change the F-MOLT-A / F-MOLT-D firing state or the Millennium tally.

The Millennium tally remains **0/7 unchanged**. This file produces a
single new design artefact (this report); no other repo file is
edited as a side-effect of authoring it.

The "expected verdict" column in §5 is **not** a pre-claim. Where an
expected sign is given, it is either (a) a structural impossibility
note (cannot PASS by construction of the discriminator), or (b) a
range estimate based on the parent literature already cited; in no
case is a numerical PASS/FAIL committed prior to execution.

---

## §1 CONFOUNDED verdict recap (n=8 → n=11 trajectory)

Per `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §4 the
discriminator-type bias hypothesis was scored **CONFOUNDED** at n=8.
The evolution since:

| n  | 2x2 (PASS-fam top, FAIL-fam bottom)  | Fisher 2-sided p | new row(s)                | source                                       |
|----|--------------------------------------|------------------|---------------------------|----------------------------------------------|
| 8  | (3, 0; 0, 5)                         | 0.036            | initial sweep             | parent audit §2                              |
| 9  | (3, 0; 1, 5)                         | 0.095            | + D1.4 PASS               | D1.4 §5.2                                    |
| 10 | (3, 1; 1, 5)                         | ~0.13            | + D1.3 FAIL               | omega-amend-d13-cross-cell §6.2              |
| 11 | (3, 1; 1, 6)                         | ~0.067           | + D1.5 FAIL               | omega-exec-bt544-d1-5-group-symmetry §6.3    |

(D1.5 was classified as **discrete-equality + vacuous-magnitude
hybrid** in §3.2 of the D1.5 report; for the 2x2 it lands in the
bottom row.)

The Fisher p has oscillated 0.036 → 0.095 → 0.13 → 0.067 over the
4-row delta. None of these movements **separates** the two competing
explanations:

- **H_type**: discriminator type axis drives PASS/FAIL.
- **H_validity**: candidate-validity axis drives PASS/FAIL; the
  type axis is collinear because the L9 catalogue's generation
  process (CATALOGUE_BIAS, parent meta-audit) systematically pairs
  discrete-equality discriminators with arithmetic-family candidates
  (which lack analytic content) and distributional / structural-
  literature discriminators with stronger candidates.

The audit's §5.5 prescription is the operative path forward:

> Resample at n ≥ 16 with **native pairings** to separate
> discriminator-type from candidate-validity (currently collinear
> via the parent CATALOGUE_BIAS).

This file is the design that prescription requires.

---

## §2 Native pairing taxonomy

A pairing of (candidate family, discriminator type) is **native** if
the discriminator naturally tests the candidate's substantive claim;
**forced** (non-native) if the discriminator is applied because the
catalogue defaulted to it, not because it interrogates what the
candidate actually claims.

The taxonomy below is taken from the user's specification in the
prompt and cross-checked against the discriminator vocabulary in
`omega-meta-audit-discriminator-type-bias-2026-04-25.md` §5.4.

| candidate family                   | native discriminator type            | what the discriminator natively tests                                                                                       |
|------------------------------------|---------------------------------------|------------------------------------------------------------------------------------------------------------------------------|
| arithmetic-family                  | discrete-equality                    | whether a closed-form algebraic / lattice identity holds (rank, det/σ ∈ ℤ, exact ratio)                                      |
| coupling / correlation-family      | distributional                       | whether two empirical distributions on a coupling pair are statistically distinguishable (KS, χ², Anderson-Darling)          |
| literature-import-family           | structural-literature                | whether a chain of published partial results licenses the claim (citation chain, peer-review ratification)                    |
| variational / Lyapunov-family      | analytic-inequality                  | whether a constructed functional has a definite-sign time derivative along the flow (monotonicity check)                     |
| procedure-class-family             | parameter-bound                      | whether a procedure terminates with bounded parameters across the relevant input class (termination + bound check)           |
| group-symmetry-family              | representation-theoretic             | whether the irreducible-representation dimension / character / branching rule matches the predicted decomposition            |
| info-theoretic / cap-family        | vacuous-magnitude (with sweep)       | whether the cap is non-vacuous in some parameter regime (Φ/cap close to 1)                                                  |
| axiom-recast (L_ω)                 | axiom-recast acceptability           | whether the recast preserves Millennium-grade and admits partial-result import (separate grid; not molt-validation)          |

A pairing is **native** when the row matches; **forced** when a
candidate from one row is judged by a discriminator from another row
(e.g., a group-symmetry candidate judged by discrete-equality on a
swirl magnitude — D1.5; a coupling-family candidate judged by
discrete-equality on a Gram-rank — Q1, with the relaxation that the
arithmetic identity at rank=3/det/σ ∈ ℤ is itself derivable, so Q1
is borderline-native).

The hypothesis to be tested at n=20: **under native pairing, type
and verdict should partially decorrelate** — if H_validity is
dominant, native pairings will produce PASSes in every row whose
candidate is structurally real, and FAILs in every row whose
candidate is structurally weak; the type axis will no longer
predict outcome. If H_type is dominant, native pairings will
**still** produce a type-vs-verdict correlation (with PASSes
concentrated in distributional / structural-literature rows even
under native pairing) because the discriminator type itself is the
binding axis.

---

## §3 Existing 11 samples — native vs forced classification

The 11 molt-validation rows (excluding D2 ACCEPTABLE rows, which
sit on the L_ω axiom-recast grid) classified per §2.

| #  | BT row                  | candidate family                  | discriminator type            | verdict                | native or forced                | rationale                                                                                                              |
|----|-------------------------|-----------------------------------|-------------------------------|------------------------|----------------------------------|------------------------------------------------------------------------------------------------------------------------|
| 1  | BT-541 Lead-B           | coupling / correlation            | distributional (KS)           | PASS                   | **native**                       | KS test on coupling distribution is the canonical test for independence/coupling claims.                                |
| 2  | BT-542 Hirahara         | literature-import (meta-complex.) | structural-literature         | PASS_LITERATURE        | **native**                       | Hirahara 2018/2022 + OS-2017 + RR-1997 chain is the canonical structural-literature path.                              |
| 3  | BT-543 P3               | arithmetic (lattice ratios)       | numerical-interval            | FAIL                   | **native**                       | A ratio falling inside a number-theoretic-derived interval is the natural arithmetic-family discriminator.            |
| 4  | BT-544 Q1               | arithmetic (lattice algebra)      | discrete-equality             | FAIL                   | **native**                       | rank=3 / det/σ ∈ ℤ on a 6×6 Gram is the canonical lattice-arithmetic test (per audit §5.3, "honest" discrete-eq).     |
| 5  | BT-544 Q5               | arithmetic (Sobolev/Besov rebrand)| discrete-equality             | FAIL                   | **forced (mild)**                | Q5 is a relabeling without analytic content; the discrete-eq discriminator records the absence rather than testing a derivable equality. |
| 6  | BT-544 KPZ d=7          | arithmetic (scaling exponent)     | discrete-equality             | FAIL                   | **native**                       | (χ_d, z_d) = (1/d, (d-1)/d) is a closed-form scaling identity; discrete-eq is the natural test.                       |
| 7  | BT-544 D1.1 HVC         | info-theoretic / cap              | vacuous-magnitude             | FAIL_VACUOUS           | **native** (with sweep deferred) | Cap-style discriminator on Φ/cap is the canonical info-theoretic test; the parameter sweep (deferred) is the §5.1 caveat. |
| 8  | BT-544 D1.3 NS↔MHD      | literature-import (cross-PDE)     | structural-cross-PDE          | FAIL_NO_DUALITY        | **forced**                       | The "duality" candidate has no published transfer theorem; structural-cross-PDE was applied because the candidate gestured at one. |
| 9  | BT-544 D1.4 She-Leveque | arithmetic (β-frame residual)     | discrete-equality             | PASS                   | **native**                       | -2/9 = ζ_6 inner-product is an exact arithmetic identity; discrete-eq is the natural test.                            |
| 10 | BT-544 D1.5 group-sym   | group-symmetry (SO(2) axisym)     | discrete-equality (on swirl s*) | FAIL_NO_LITERATURE_PATH | **forced**                       | Native discriminator would be representation-theoretic (irrep decomposition of the swirl operator); discrete-eq on a magnitude is mismatched. |
| 11 | BT-545 IHC              | literature-import (Hodge IHC)     | structural-literature         | PASS_LITERATURE        | **native**                       | IHC verdict map (de Gaay Fortman / Benoist-Ottem) is a published-literature chain; structural-literature is canonical. |
| 12 | BT-546 A3D              | coupling / correlation (28-strat.)| distributional (KS on z)       | PASS_DISTRIBUTIONAL    | **native**                       | KS test on z-scores across 28-stratum is the canonical distributional test on the coupling.                            |

(Row count = 12; the n=11 figure in §1 reflects the 11 rows that
have entered the discriminator-type 2x2 contingency; BT-546 enters
at n=11 in the dashboard tally per §4.2 of the F-MOLT unified
status, reaching n=11 with D1.5 inclusion and BT-546 inclusion.
Slight n discrepancy across reports reflects the cutoff date of
each parent audit; for design purposes, the operative existing-
sample count is 11–12.)

### §3.1 Forced-pairing summary

- **Forced (clear)**: Q5 (discrete-eq on a relabeling), D1.3
  (structural-cross-PDE on a non-existent duality), D1.5 (discrete-eq
  on a swirl magnitude when group-rep is native).
- **Native**: 9 of 12 rows.
- **Forced fraction**: 3/12 = 25%.

These forced pairings are exactly the rows the §3 alternative-
explanation audit (parent §3.1) flagged as candidate-validity-
driven. Removing them (or replacing them with native counterparts)
would clarify the type axis.

---

## §4 n=20 gap analysis — which cells need fills?

### §4.1 Current 4×4 (family × discriminator) cell counts

Aggregating over candidate family (rows) × discriminator type
(cols), with native pairings on the diagonal and forced pairings
off-diagonal:

| family ↓ \ discr →    | discrete-eq | distrib | struct-lit | analytic-ineq | param-bound | rep-theoretic | vacuous-mag | total |
|------------------------|-------------|---------|------------|---------------|-------------|---------------|-------------|-------|
| arithmetic             | 4 (3F,1P)   | 0       | 0          | 0             | 0           | 0             | 0           | 4     |
| coupling/correlation   | 0           | 2 (2P)  | 0          | 0             | 0           | 0             | 0           | 2     |
| literature-import      | 0           | 0       | 3 (3P)     | 0             | 0           | 0             | 0           | 3     |
| variational / Lyapunov | 0           | 0       | 0          | **0**         | 0           | 0             | 0           | 0     |
| procedure-class        | 0           | 0       | 0          | 0             | **0**       | 0             | 0           | 0     |
| group-symmetry         | 1 (1F, forced) | 0    | 0          | 0             | 0           | **0**         | 0           | 1     |
| info-theoretic / cap   | 0           | 0       | 0          | 0             | 0           | 0             | 1 (1F)      | 1     |
| forced-cross-PDE       | 0           | 0       | 1 (1F)     | 0             | 0           | 0             | 0           | 1     |
| **total**              | 5           | 2       | 4          | 0             | 0           | 0             | 1           | 12    |

### §4.2 Empty cells needing fills

The diagonal (native pairings) has **3 empty cells**:

- **(variational, analytic-inequality)**: 0 entries. EXT-B class.
- **(procedure-class, parameter-bound)**: 0 entries. EXT-C class.
- **(group-symmetry, representation-theoretic)**: 0 entries. EXT-D
  vocabulary extension.

The off-diagonal that the audit explicitly recommends populating
to test type-axis separability:

- **(non-arithmetic, discrete-equality)**: 1 entry (D1.5, forced).
  Need more cleanly-native non-arithmetic + discrete-eq pairs (e.g.,
  representation-theoretic candidates with discrete-equality on
  irrep dimensions).
- **(arithmetic, distributional)**: 0 entries. Need arithmetic
  candidates whose validity is testable via KS-type distributional
  test.

### §4.3 What 8–10 additional samples must achieve

For type-axis separability via native pairing:

1. Fill **3 empty diagonal cells** (variational, procedure-class,
   group-symmetry-native) — minimum 3 samples.
2. Add **2–3 cleanly cross-cell native pairs** (arithmetic +
   distributional; non-arithmetic + discrete-eq native) — 2–3
   samples.
3. Reinforce **distrib / struct-lit row** with 2–3 more native
   samples (so n in that row reaches ≥ 6 for stable Fisher).
4. Optionally: 1 more cap-family sample with parameter sweep
   (D1.1 follow-up) to see whether vacuous-magnitude is
   discriminator-driven or candidate-driven.

Total: 8–10 additional samples → final n = 11 + 8 to 11 + 10 = 19
to 21. The user's target is n=20.

---

## §5 8–10 additional sample proposals

Each row gives BT id, candidate family, discriminator type, native
or forced classification, expected cost, source seed, and a brief
information-value note. **No verdict is pre-claimed.**

| #  | proposal id          | BT  | candidate family            | discriminator type            | native/forced | cost   | source seed                                                          | information-value note                                                       |
|----|----------------------|-----|------------------------------|-------------------------------|---------------|--------|----------------------------------------------------------------------|------------------------------------------------------------------------------|
| P1 | EXT-B-CI-Lyap        | 544 | variational / Lyapunov      | analytic-inequality           | **native**    | high   | omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md      | First (variational, analytic-ineq) cell entry; tests EXT-B class.            |
| P2 | EXT-A-NS-relentropy  | 544 | variational / Lyapunov      | structural-literature (variational-derivation) | **native** | medium | BT-547 retro §5.1 EXT-A seed                                         | Second EXT-A entry; (variational, struct-lit) covers M1-class molts.         |
| P3 | EXT-C-RH-explform    | 541 | procedure-class             | parameter-bound               | **native**    | high   | BT-547 retro §5.3 EXT-C seed                                         | First (procedure-class, param-bound) entry; tests M3-class molts.            |
| P4 | EXT-D-D1.5-irrep     | 544 | group-symmetry              | representation-theoretic      | **native**    | medium | D1.5 §3.2 + EXT-D seed                                               | Replaces D1.5's forced pairing with the native irrep-dim test.               |
| P5 | BT-541-Y2-arith-KS   | 541 | arithmetic (Y2 zero-spacing residuals) | distributional (KS)        | **mixed-native** | low | dfs-24-riemann §"Lead-A normality" / Y2 residuals                    | Cross-cell: arithmetic candidate + distributional discriminator (off-diag).  |
| P6 | BT-546-D1-stratum-irrep | 546 | group-symmetry (Z/6+Z/12)  | representation-theoretic      | **native**    | low    | omega-exec-bt546-bsd §6.4 + Probe-B 28-stratum                       | Second rep-theoretic entry; uses BT-546 stratum data already collected.      |
| P7 | BT-545-D2-IHC-dim    | 545 | literature-import (Hodge cycles) | discrete-equality (on cycle dim)| **mixed-native** | low | bt545-hodge §6.3 IHC-DIM falsifier                                | Cross-cell: lit-import candidate + discrete-eq discriminator.                |
| P8 | BT-544-D1.1-sweep    | 544 | info-theoretic / cap        | vacuous-magnitude (with R, ν, T sweep) | **native** | medium | bt544-d1-1-hvc §7.2 deferred sweep                                  | Tests whether vacuous-magnitude is discriminator-driven (audit §5.1 caveat). |
| P9 | BT-543-EXT-B-chromo  | 543 | variational / Lyapunov      | analytic-inequality           | **native**    | high   | BT-547 retro §5.2 EXT-B seed for BT-543 (chromomagnetic-energy)      | Third (variational, analytic-ineq) entry; balances EXT-B across BTs.         |
| P10| BT-542-EXT-B-meta    | 542 | meta-Lyapunov (resource-monotone)| analytic-inequality (meta-form) | **native** | high   | l9-generation-pipeline §6.4 meta-complexity-resource-monotone seed   | First analytic-ineq for non-PDE BT; broadens the variational class.          |

### §5.1 Coverage check

After 10 additions, the family × discriminator matrix becomes
(approximate diagonal-fill, additions in **bold**):

| family ↓ \ discr →    | discrete-eq | distrib | struct-lit | analytic-ineq | param-bound | rep-theoretic | vacuous-mag |
|------------------------|-------------|---------|------------|---------------|-------------|---------------|-------------|
| arithmetic             | 4           | **+P5** | 0          | 0             | 0           | 0             | 0           |
| coupling/correlation   | 0           | 2       | 0          | 0             | 0           | 0             | 0           |
| literature-import      | **+P7**     | 0       | 3          | **+P2**       | 0           | 0             | 0           |
| variational / Lyapunov | 0           | 0       | **+P2**    | **+P1, P9, P10** | 0       | 0             | 0           |
| procedure-class        | 0           | 0       | 0          | 0             | **+P3**     | 0             | 0           |
| group-symmetry         | (D1.5)      | 0       | 0          | 0             | 0           | **+P4, P6**   | 0           |
| info-theoretic / cap   | 0           | 0       | 0          | 0             | 0           | 0             | 1 + **P8**  |

Cells filled by P1–P10:
- (variational, analytic-ineq): +3 (P1, P9, P10).
- (variational, struct-lit): +1 (P2 secondary classification).
- (procedure-class, param-bound): +1 (P3).
- (group-symmetry, rep-theoretic): +2 (P4, P6).
- (arithmetic, distrib) cross-cell: +1 (P5).
- (lit-import, discrete-eq) cross-cell: +1 (P7).
- (cap, vacuous-mag) reinforcement: +1 (P8).

The 3 empty diagonals are filled; 2 cross-cell natives are
introduced; the cap row gets its parameter-sensitivity sweep.

---

## §6 Sequencing + top-3 next-session dispatches

### §6.1 Ranking criteria

- **C1 cost**: low first (next-session executable).
- **C2 information value**: cells with current 0 entries first (P1,
  P3, P4 fill empty diagonals; high information).
- **C3 audit-recommended priority**: per BT-547 retro §6.1, BT-544
  EXT-B has highest urgency.

### §6.2 Combined ranking

| rank | proposal | cost   | info-value | audit-priority | combined-score | rationale                                                   |
|------|----------|--------|------------|----------------|----------------|--------------------------------------------------------------|
| 1    | **P5**   | low    | medium     | medium         | **HIGH**       | Lowest cost; cross-cell native (arithmetic + distrib) = high info; can use existing Y2 residual data.|
| 2    | **P6**   | low    | high       | medium         | **HIGH**       | Lowest cost; fills (group-sym, rep-theoretic) empty diagonal; uses already-collected 28-stratum data.|
| 3    | **P7**   | low    | medium-high| medium         | **HIGH**       | Lowest cost; cross-cell native (lit-import + discrete-eq); uses existing IHC-DIM falsifier framing. |
| 4    | P4       | medium | high       | medium         | medium-high    | Replaces forced D1.5 with native pairing; medium cost (irrep computation).                          |
| 5    | P8       | medium | medium     | medium         | medium         | Discriminator-vs-candidate test for vacuous-mag; medium cost (parameter sweep).                     |
| 6    | P2       | medium | high       | high           | medium-high    | Variational struct-lit; medium cost (literature derivation sketch).                                 |
| 7    | P1       | high   | high       | **highest**    | high (long)    | Highest audit priority (BT-544 EXT-B); high cost (Lyapunov construction sketch); already seeded.    |
| 8    | P3       | high   | high       | high           | high (long)    | EXT-C procedure-class; high cost (procedure construction).                                          |
| 9    | P9       | high   | high       | high           | high (long)    | BT-543 EXT-B; high cost; speculative per parent §F-DIAG-E.                                          |
| 10   | P10      | high   | medium-high| medium         | medium-high (long)| BT-542 meta-Lyapunov; high cost; broadens variational beyond PDEs.                                |

### §6.3 Top-3 next-session dispatches

**Top-3 by combined score (C1 dominant, C2 next):**

1. **P5 — BT-541 Y2 arithmetic-residual KS test** (low cost,
   cross-cell native, off-diagonal information). Uses existing
   Odlyzko zero database + Y2 residual derivation; KS statistic
   computation is sub-second.
2. **P6 — BT-546 28-stratum representation-theoretic test** (low
   cost, fills empty diagonal, uses BT-546 Probe-B data already on
   record). Computes irrep-dim decomposition of the stratum action;
   the data is already collected per `omega-exec-bt546-probeB-28stratum`.
3. **P7 — BT-545 IHC discrete-equality on cycle dimension** (low
   cost, cross-cell native, uses BT-545 Hodge data already on
   record). Tests whether the IHC failure-locus split at dim n/φ=3
   admits a discrete-equality reformulation that is honest.

These three together: 3 new rows; 2 cross-cell + 1 native-diagonal
fill; all three are next-session executable and total estimated
compute < 2h.

### §6.4 Sequencing of remainder

After top-3 (n=14), the next batch (P4, P8, P2 — medium cost)
brings n=17. The high-cost trio (P1, P3, P9 or P10) closes to
n=20 over 2–3 subsequent sessions. The high-cost EXT-B/C/D
constructions inherit from the BT-547 retro recommendations and
the EXT-B candidate already seeded in
`omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md`.

---

## §7 Statistical power estimate at n=20

### §7.1 Fisher exact two-sided p, scenario-by-scenario

The current contingency at n=11 is (3, 1; 1, 6) with Fisher
2-sided p ~ 0.067 (per D1.5 §6.3). At n=20, three scenarios:

#### Scenario A — H_validity dominant (audit's working null)

If candidate-validity drives outcome and native pairings produce
PASSes for structurally-real candidates and FAILs for weak ones,
the type-vs-verdict correlation **decays**. Expected 2x2 at n=20:
roughly (5, 3; 3, 9) — a few PASSes leak into the discrete-eq row
(D1.4-class arithmetic identities; P5/P7 cross-cell), and a few
FAILs leak into the distrib/struct-lit row (D1.3-class forced
pairings; high-cost EXT failures).

For (5, 3; 3, 9): Fisher 2-sided p ≈ 0.10–0.18 (no longer
significant). H_type would be **rejected as the dominant axis**.

#### Scenario B — H_type dominant

If discriminator type drives outcome regardless of candidate
validity, native pairings will not equalize PASS rates. Expected
2x2 at n=20: roughly (8, 1; 1, 10).

For (8, 1; 1, 10): Fisher 2-sided p ≈ 0.001–0.005 (highly
significant). H_type would be **confirmed at α=0.01**.

#### Scenario C — Mixed (both axes active)

Both axes contribute non-trivially. Expected 2x2: roughly
(7, 2; 2, 9). Fisher 2-sided p ≈ 0.02–0.05 (borderline
significant).

### §7.2 Power summary table

| scenario        | expected 2x2 (post-n=20)    | Fisher 2-sided p | verdict at α=0.05 | distinguishability |
|-----------------|------------------------------|-------------------|---------------------|--------------------|
| A (H_validity)  | (5, 3; 3, 9)                | 0.10–0.18         | NOT REJECTED null   | type axis weakened |
| B (H_type)      | (8, 1; 1, 10)               | 0.001–0.005       | REJECTED null       | type axis confirmed|
| C (mixed)       | (7, 2; 2, 9)                | 0.02–0.05         | borderline          | mixed reading      |

At n=20, scenarios A and B are **distinguishable at α=0.05** with
adequate power. Scenario C remains borderline; an n=24 extension
(rounds 5–6 of the EXT class) would resolve it if needed.

### §7.3 Caveats on the power estimate

- The estimate assumes the new 8–10 rows are **independent draws**
  from the same population; if EXT-A/B/C constructions cluster
  (e.g., 3 of 3 variational candidates fail because no one has yet
  constructed a working Lyapunov), the effective sample size is
  smaller.
- The native-vs-forced classification is a **design choice** that
  partially determines the outcome. If reviewers re-classify some
  P-rows as forced rather than native, the cleanly-separable
  signal weakens.
- The CONFOUNDED parent verdict was reached **at the boundary of
  what n=8 can say**. A genuinely separable outcome at n=20
  requires not just sample size but also pairing-cleanliness;
  the §3 forced-pairing inventory (3 of 12 rows) is a starting
  defect that the §5 proposals partially remediate.

---

## §8 Anti-list — candidates considered and rejected

Candidates and pairings that were considered for the n=20 plan and
**rejected** with reason.

| AL# | candidate / pairing                                                         | rejection reason                                                                                                                                   |
|-----|------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| AL1 | BT-544 Q2 (next-cheapest dispatch per D1.4 §6.4) with discrete-eq          | Q2 candidate is again arithmetic-family with discrete-eq native; adds another diagonal arithmetic + discrete-eq entry, which already has 4 rows.    |
| AL2 | BT-544 D1.2 Pólya recurrence with discrete-eq                              | Same as AL1; adds redundant diagonal; does not contribute to type-axis separability.                                                                |
| AL3 | BT-544 D2 R1 Lemma 1 retry (intermittency-aware) with sketch                | This is an L_ω axiom-recast follow-up, not a molt-validation; falls on the ACCEPTABLE grid (parent §6) and does not enter the 2x2 directly.        |
| AL4 | BT-547 D-tier (Perelman extensions)                                        | Out of scope per Anti-E (`omega-exec-bt547-poincare-L9-retro` §7); BT-547 is Perelman-solved and excluded from active tally.                          |
| AL5 | BT-545 D-tier (Hodge D-tier extensions beyond IHC)                         | Not designed; would require a separate seed session. Cost-deferred to post-n=20.                                                                    |
| AL6 | BT-546 D-tier beyond Probe-B + P6                                          | Same as AL5; designed seed not available; cost-deferred.                                                                                            |
| AL7 | BT-541 Lead-A normality probe (full re-execution rather than P5 residuals) | Cost too high (full Odlyzko statistical re-validation); P5 captures the cross-cell native pair at much lower cost.                                  |
| AL8 | BT-541 EXT-A variational re-interpretation                                 | No clean variational re-cast of RH exists in the literature at the candidate-generation level; would require new structural work. Defer to post-n=20.|
| AL9 | BT-542 EXT-A variational                                                   | Meta-complexity has no canonical variational structure; the EXT-B-meta route (P10) is more honest. EXT-A-meta rejected as forced.                   |
| AL10 | BT-541 explicit-formula descent **as discrete-equality** (mismatched)     | Forced pairing: procedure-class candidate judged by discrete-equality on a numerical bound. Native is parameter-bound; P3 keeps the native pairing. |

---

## §9 Falsifiers active for the resample design

Self-falsifiers under which **this design** would be retracted or
substantially revised.

- **F-DESIGN-A** (mis-classified native/forced): if any of the 12
  existing-sample classifications in §3 is shown to mis-pair (e.g.,
  if Q5 is re-judged native rather than forced-mild), the §4
  cell-count diagnosis shifts. Risk: low — classifications are
  sourced from the parent reports' own framing of native vs forced.
  **Not active.**
- **F-DESIGN-B** (proposal cost mis-estimate): if P5/P6/P7 turn
  out to be medium or high cost rather than low, the top-3
  next-session dispatch sequence breaks. Risk: medium — P5/P6/P7
  rely on existing data, but residual derivation / irrep-dim
  computation may surface unanticipated complications. **Partially
  active**: if a top-3 candidate proves higher-cost than expected,
  the next-session dispatch falls back to P4 or P8.
- **F-DESIGN-C** (proposal cannot be instantiated): if EXT-B / EXT-C
  / EXT-D candidates prove not to admit a coherent construction
  (per parent F-DIAG-E), P1 / P3 / P4 cannot produce verdicts and
  the n=20 target is not reached at the diagonal-fill level. Risk:
  medium — the EXT-B candidate (Constantin-Iyer Lyapunov) is
  already seeded but its monotonicity is conjectural. **Partially
  active**: if EXT-B yields OBSTRUCTION_DOCUMENTED rather than
  PASS_LITERATURE / FAIL, that is itself a row in the 2x2 (treated
  as INCONCLUSIVE rather than PASS or FAIL — § note in §7 about
  effective sample size).
- **F-DESIGN-D** (n=20 still under-powered): if at n=20 the Fisher
  2-sided p sits in the 0.05–0.15 band (Scenario C boundary), the
  CONFOUNDED verdict is **not** resolved and an n=24 extension is
  required. Risk: medium — the §7 power estimate gives 5/10
  scenario weight to this band. **Conditional**: design contains
  the next-batch sequencing that handles this contingency.
- **F-DESIGN-E** (atlas/state/inventory side-effect): if execution
  of any P1–P10 sample is shown to have edited atlas, state, or
  inventory beyond the molt-validation report itself, the read-only
  scope of this design is violated. The design is read-only and
  produces only this one new file. **Not active.**
- **F-DESIGN-F** (fabricated expected-verdict): if any reader
  interprets §5's "expected sign" or §7's "expected 2x2" as a
  pre-claim of PASS/FAIL on a specific row, the §0 disclaimer is
  to be re-read. The expected-2x2 in §7 is a **sensitivity
  envelope** to enable the power calculation, not a forecast.
  **Not active** but marked for review.
- **F-DESIGN-G** (native-pairing axiom is itself the confound): if
  the very act of "native pairing" pre-selects for PASSes (because
  natural discriminators ARE more permissive on natural candidates),
  the n=20 resample inherits a different bias from the one it
  removes. Risk: low-medium — addressed by including 2–3 cross-cell
  native pairs (P5, P7) and forcing-replaced D1.5 → P4. **Partially
  active**: a follow-up audit comparing native-PASS-rate vs
  native-FAIL-rate post-n=20 can detect this if it dominates.

None of F-DESIGN-A..G fires under the current evidence; F-DESIGN-B,
C, D, G are conditional and would activate only on specific
execution failure modes.

---

## §10 Closing

- **Design output**: 8–10 additional sample proposals (P1–P10),
  bringing the discriminator-type 2x2 contingency from n=11 to
  n=19–21 (target n=20).
- **Native-vs-forced map**: each proposal has explicit native or
  forced classification per the §2 taxonomy.
- **Cell-fill**: 3 empty diagonals filled (variational +
  analytic-ineq; procedure-class + param-bound; group-symmetry +
  rep-theoretic); 2 cross-cell native pairs added (arithmetic +
  distrib via P5; lit-import + discrete-eq via P7); 1 cap-family
  parameter sweep added (P8).
- **Top-3 next-session dispatches**: P5 (BT-541 Y2 arith-residual
  KS), P6 (BT-546 28-stratum rep-theoretic), P7 (BT-545 IHC
  cycle-dim discrete-eq). All three are low cost (next-session
  executable) and use data already collected.
- **Statistical power at n=20**: scenarios A (H_validity dominant)
  and B (H_type dominant) are distinguishable at α=0.05 with
  Fisher exact p ranges 0.10–0.18 vs 0.001–0.005; scenario C
  (mixed) sits at 0.02–0.05 (borderline) and would warrant n=24
  extension if it materialises.
- **Atlas / state / inventory edits**: zero. This file is the only
  new artefact.
- **Millennium tally**: **0/7 unchanged.**
- **Verdict graded**: methodology design, no claim. The n=20 plan
  is **design-only**. No new molt-validations were executed; no
  PASS/FAIL was issued; no atlas grade was modified.

0/7 unchanged. n=20 plan is design-only. No atlas/state/inventory
edits.

— end design —
