---
id: omega-exec-bt544-extc-procedure-class-candidate
date: 2026-04-25
scope: research-only candidate generation (NOT validating; procedure-class candidate)
target: BT-544 EXT-C -- procedure-class molt with parameter bounds
parent_reports:
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (EXT-C prescription)
  - reports/sessions/omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md (EXT-B template)
  - reports/sessions/omega-meta-audit-l9-generation-pipeline-2026-04-25.md (predicted classes)
millennium_resolved: 0/7 (unchanged)
grade: candidate generation, no claim
---

# Omega Exec — BT-544 EXT-C Procedure-Class Candidate (2026-04-25)

## §0 Non-claim disclaimer

This report **generates** a single BT-544 frame-shift candidate of
the EXT-C class (procedure-class-with-parameter-bounds molt)
recommended by `reports/sessions/omega-exec-bt547-poincare-L9-retro-
2026-04-25.md` §5.3. It does **not**:

- claim 3D Navier-Stokes regularity, blow-up, or any Clay-form
  resolution;
- prove or even sketch-prove the candidate procedure's termination
  — termination is **conjectural** and is exactly the data the
  discriminator and falsifier are designed to record;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-544 = 0/1 untouched` Clay
  status, or add to the L9 catalogue's active-candidate ledger;
- replace or supersede the D1 / D2 / D3.A / D3.B' / D3.C catalogue
  rows. EXT-C operates on the *missing class* per the BT-547 retro
  partial-fit verdict, parallel to but distinct from EXT-A
  (variational reinterpretation) and EXT-B (analytic-Lyapunov
  construction).

EXT-C is a **candidate-generation** task per the BT-547 retro
recommendation (§5.3). It is a NEW family slot — procedure-class
with parameter bounds — that the prior generation pipeline (dfs-24
direction probes) did not admit. The verdict expected from this
candidate, when validated in a follow-up session, is
**OBSTRUCTION_DOCUMENTED** if the procedure is shown to terminate
only under hypotheses equivalent to NS regularity itself,
**PASS_SKETCH** if a partial termination argument assembles on a
restricted family, **PASS_LITERATURE** if a pre-existing rigorous
form already covers the candidate, and **INCONCLUSIVE** if the
literature is underdetermined. This file records the generation;
validation is a separate session.

**0/7 unchanged. No atlas/state/inventory edits.**

---

## §1 Lineage — Perelman M3 surgery as procedure-class molt

### §1.1 Perelman M3 structural recap

Per `omega-exec-bt547-poincare-L9-retro-2026-04-25.md` §1.3 and
§2.3, Perelman's M3 (Perelman 2003a arXiv:math/0303109; Perelman
2003b arXiv:math/0307245) is the *quantitative-surgery + finite-
extinction* molt that closed the Ricci-flow programme on simply-
connected closed 3-manifolds. The structural anatomy of M3 relevant
to the NS analog:

(i) **A procedure**, not an identity or an inequality. Surgery is
    an iterative operation that modifies the Ricci-flow solution
    each time a curvature singularity forms, by removing a
    canonical neck-region and gluing in a standard cap.

(ii) **Parameters with explicit bounds**. Each surgery interval
     carries scale parameters (δ_i, r_i, h_i) per Perelman 2003a
     §4-§5: r_i is the canonical-neighbourhood radius, δ_i is the
     surgery-precision parameter, h_i is the surgery scale. The
     bounds δ_i < δ_0(r_i, ε) and h_i < h_0(r_i, δ_i) are
     PARAMETER-BOUND CONSISTENCY conditions — the choice at level i
     constrains levels i+1, i+2, ….

(iii) **Termination via sweep-out**. On simply-connected closed
      3-manifolds, the Perelman 2003b finite-extinction theorem uses
      a min-max sweep-out (Colding-Minicozzi 2008 *J. Amer. Math.
      Soc.* 21 reproof) to show the surgery procedure terminates in
      finite time: extinction occurs before infinite surgery
      accumulation.

(iv) **Regularity claim from termination**. Termination of the
     surgery procedure plus the canonical-neighbourhood structure
     yields the Poincaré conjecture (every simply-connected closed
     3-manifold is S^3).

The M3 molt is therefore: **bounded-parameter iterative procedure +
finite-step termination → topological regularity claim**. It is
neither a single algebraic identity (M1's variational
reinterpretation in the Perelman-Sergeyev tradition) nor a single
inequality (M2's W-monotonicity); it is a *constructive procedure*
with a controlled parameter cascade.

### §1.2 What an NS analog requires

Per the BT-547 retro §5.3 EXT-C recommendation and the M3 anatomy
(i)-(iv), an EXT-C candidate for NS must specify:

- a **procedure** that iteratively modifies the NS solution u(t, x)
  via a bounded operation (analog of (i): surgery → some
  cellular/scale-decomposition surgery on (t, x)-domain or on
  Fourier modes);
- **parameters** (analog of (ii)) with **explicit bounds** that are
  consistent across iteration levels (analog of Perelman's r_i, δ_i,
  h_i cascade);
- a **termination condition** (analog of (iii)): the procedure
  halts in finite steps under a verifiable structural hypothesis;
- a **regularity-coupling claim** (analog of (iv)): termination
  yields control of a regularity-sensitive norm (Sobolev H^s,
  Hölder C^α, BMO⁻¹, parabolic Hausdorff dimension of singular set,
  or similar).

The BT-544 lacuna identified in BT-547 retro §6.1 includes the
absence of a procedure-class candidate satisfying (i)-(iv) jointly.
The existing NS toolkit has each piece in isolation but not the
joint procedural package (§2 below).

---

## §2 Existing NS procedure-class inventory — what we already have

The active-NS literature contains procedure-style constructions.
Each is recorded below with its parameter structure, termination
status, and known limitation. References are cited by author /
year / journal and are pre-existing; nothing in this section is
fabricated.

### §2.1 Caffarelli-Kohn-Nirenberg 1982 partial-regularity procedure

- **Source**: Caffarelli-Kohn-Nirenberg 1982 (*Comm. Pure Appl.
  Math.* 35, 771-831), "Partial regularity of suitable weak
  solutions of the Navier-Stokes equations".
- **Procedure**: at each parabolic point (t_0, x_0), check whether
  the local smallness criterion
    (1/r) ∫∫_{Q_r(t_0, x_0)} |∇u|² dx ds < ε_*
  holds for some r > 0. If yes, point is regular. The singular set
  S = {points failing the smallness for all r} is then covered by
  parabolic balls and shown to have 1D parabolic Hausdorff measure
  zero.
- **Parameters**: ε_* (smallness threshold, fixed universal
  constant); r (cover radius, taken to 0 in the Vitali-style
  argument).
- **Termination**: the *cover-and-extract-Vitali-subcover*
  procedure terminates as r → 0 by the Vitali covering lemma. The
  output is a measure-theoretic statement on S.
- **Status as M3 analog**: PARTIAL. CKN has parameter (ε_*) and a
  covering-procedure structure, but the parameter is FIXED (not a
  multi-level cascade), and the regularity output is a measure-
  theoretic bound on the singular set, NOT a global
  regularity/extinction claim. CKN is the *closest existing
  procedure* to the M3 archetype but lacks the parameter-cascade
  and termination-to-regularity structure.

### §2.2 Lin 1998 / Vasseur 2007 streamlined CKN procedures

- **Sources**: Lin 1998 (*Comm. Pure Appl. Math.* 51, 241-257), "A
  new proof of the Caffarelli-Kohn-Nirenberg theorem"; Vasseur 2007
  (*NoDEA Nonlinear Differential Equations Appl.* 14, 753-785), "A
  new proof of partial regularity of solutions to Navier-Stokes
  equations".
- **Procedure**: De Giorgi-style L²→L^∞ improvement: at each level
  k, define truncation u_k = (|u| − M_k)_+; energy estimate gives
  improvement in M_k. Iterate.
- **Parameters**: M_k (truncation level), C_k (energy constant);
  bound M_{k+1} ≤ θ M_k with θ < 1.
- **Termination**: the De Giorgi iteration terminates when M_k
  reaches a controlled level, yielding L^∞ bound on the regular
  set.
- **Status as M3 analog**: CLOSER. Vasseur's De Giorgi iteration is
  a genuine multi-level parameter-cascade (M_k → M_{k+1}) with a
  contraction θ. But the output is again the CKN partial-
  regularity result (1D parabolic Hausdorff measure zero on
  singular set) — not a global regularity/extinction claim. The
  parameter cascade exists; the regularity-coupling output is
  still measure-theoretic, not norm-controlled.

### §2.3 Hou-Luo 2014 numerical surgery / blow-up scenario

- **Source**: Hou-Luo 2014 (*Multiscale Model. Simul.* 12,
  1722-1776; with extended commentary in Tao 2016 unfinished
  blog series), "Toward the finite-time blowup of the 3D
  axisymmetric Euler equations".
- **Procedure**: numerically evolve axisymmetric Euler, monitor
  vorticity at the boundary of a tubular region, refine grid as
  ‖ω‖_{L^∞} grows; the procedure is to repeatedly subdivide the
  computational domain near the suspected blow-up point.
- **Parameters**: grid scale h_k (with refinement h_{k+1} = h_k/2
  near the singular tube); time step Δt_k (CFL-bounded by h_k).
- **Termination**: numerical termination at h_k below double-
  precision; the *mathematical* termination is OPEN — Hou-Luo
  observe what looks like finite-time blow-up but the result is
  numerical, not a theorem.
- **Status as M3 analog**: STRUCTURALLY ALIGNED but POINTS THE
  WRONG DIRECTION. Hou-Luo is a procedure with parameter cascade
  (h_k) and conjectural termination, but its termination would
  give a *blow-up*, not regularity. The procedure is a "find the
  singularity" surgery, not a "remove the singularity" surgery
  (the Perelman direction).

### §2.4 Tao 2016 averaged-NS blow-up procedure

- **Source**: Tao 2016 (*J. Amer. Math. Soc.* 29, 601-674),
  "Finite-time blowup for an averaged three-dimensional Navier-
  Stokes equation".
- **Procedure**: replace the NS bilinear B(u, u) = (u·∇)u + ∇p
  with a carefully averaged B̃(u, u) that preserves energy and
  scaling; show that for B̃, finite-time blow-up occurs by
  cascade-on-dyadic-scales.
- **Parameters**: dyadic scales 2^k (Fourier shell index k);
  amplitude a_k at shell k; cascade time τ_k.
- **Termination**: cascade hits a self-similar accelerating
  pattern with τ_k summable, so total time T < ∞. Tao 2016
  Theorem 1.4 establishes this RIGOROUSLY for the averaged
  equation.
- **Status as M3 analog**: PROCEDURE-CLASS YES, but again wrong
  direction (proves blow-up of averaged NS, not regularity of
  true NS). Crucially, Tao 2016 demonstrates that *some* averaged
  NS admits a procedure-class blow-up; it is a barrier theorem,
  not a regularity-construction. As a methodological model for
  EXT-C, however, it is the cleanest published procedure-class
  with parameter cascade and rigorous termination.

### §2.5 Buckmaster-Vicol 2019 convex-integration procedure

- **Source**: Buckmaster-Vicol 2019 (*Ann. of Math.* 189,
  101-144), "Nonuniqueness of weak solutions to the Navier-Stokes
  equation".
- **Procedure**: Nash-style iterative convex integration. At each
  level q, build u_q = u_{q-1} + perturbation(λ_q, μ_q, …) such
  that the Reynolds stress decreases by a factor.
- **Parameters**: λ_q (frequency at level q, λ_{q+1} = λ_q^b for
  b > 1); μ_q (amplitude); δ_q (Reynolds-stress level,
  δ_{q+1} = δ_q · ε for ε < 1).
- **Termination**: iteration converges in C^0_{t,x} norm (and in
  C^β for β small) to a non-unique weak solution. Termination is
  RIGOROUS at the C^0 level.
- **Status as M3 analog**: STRONGLY ALIGNED structurally
  (parameter cascade, termination argument, regularity-related
  output) but produces NON-UNIQUENESS, not uniqueness/regularity.
  As with Hou-Luo and Tao 2016, the procedure exists but points
  away from regularity.

### §2.6 Cheskidov-Luo 2022 sharp non-uniqueness procedure

- **Source**: Cheskidov-Luo 2022 (*Invent. Math.* 229, 987-1054),
  "Sharp nonuniqueness for the Navier-Stokes equations".
- **Procedure**: refined convex-integration scheme reaching the
  L^1_t L^∞_x non-uniqueness threshold (sharp endpoint).
- **Parameters**: as Buckmaster-Vicol with sharper λ_q, μ_q
  scheduling.
- **Termination**: converges to non-unique weak solutions in
  L^1_t L^∞_x.
- **Status as M3 analog**: same as Buckmaster-Vicol — procedure-
  class candidate proving the wrong direction.

### §2.7 Inventory summary table

| name | parameter cascade? | termination? | regularity coupling | EXT-C-class? |
|------|--------------------|--------------|---------------------|---------------|
| CKN 1982 | NO (single ε_*) | YES (Vitali) | partial-regularity | PARTIAL — single-parameter |
| Lin 1998 / Vasseur 2007 | YES (M_k cascade) | YES (De Giorgi) | partial-regularity | PARTIAL — closer, output still measure-theoretic |
| Hou-Luo 2014 | YES (h_k) | OPEN (numerical only) | blow-up (conjectural) | NO — wrong direction, no theorem |
| Tao 2016 | YES (dyadic 2^k) | YES (rigorous) | blow-up of *averaged* NS | NO — wrong direction, wrong equation |
| Buckmaster-Vicol 2019 | YES (λ_q, μ_q, δ_q) | YES (rigorous) | non-uniqueness | NO — wrong direction |
| Cheskidov-Luo 2022 | YES (sharper λ_q, μ_q) | YES (rigorous) | sharp non-uniqueness | NO — wrong direction |

The inventory shows: NS has procedure-class methods with parameter
cascades and rigorous termination, but they all point AWAY from
regularity (Tao, Buckmaster-Vicol, Cheskidov-Luo prove blow-up or
non-uniqueness; Hou-Luo conjectures blow-up). The CKN / Lin /
Vasseur partial-regularity line goes TOWARD regularity but stops
at a measure-theoretic statement on the singular set, not at a
global regularity/extinction claim.

**The EXT-C slot is genuinely empty in the regularity direction**:
no published NS procedure has the M3 anatomy (parameter cascade +
termination + regularity claim → "no singular set" or "extinction
of high-vorticity region"). This is the lacuna BT-547 retro §5.3
names for BT-544.

---

## §3 Candidate frame-shift specification

### §3.1 Chosen candidate — name, source, and statement

**Candidate name**: **Quantitative parabolic-cell surgery for
3D Navier-Stokes** (hereafter **QPC-Surgery**).

**Primary literature anchor**: Caffarelli-Kohn-Nirenberg 1982
(*Comm. Pure Appl. Math.* 35, 771-831). The CKN parabolic-cell
covering of the singular set is the procedural seed; the candidate
extends it with a multi-level parameter cascade and a termination
argument modeled on Perelman 2003a §4-§5.

**Secondary literature anchors** (procedure-cascade and termination
machinery):

- Lin 1998 (*Comm. Pure Appl. Math.* 51, 241-257) — De Giorgi-
  style level-cascade for CKN.
- Vasseur 2007 (*NoDEA* 14, 753-785) — alternative level-cascade
  with energy-improvement contraction.
- Tao 2016 (*J. Amer. Math. Soc.* 29) — model for rigorous dyadic-
  cascade termination (in averaged-NS direction).
- Buckmaster-Vicol 2019 (*Ann. of Math.* 189) — model for rigorous
  q-level termination.
- Perelman 2003a (arXiv:math/0303109 §4-§5) — model for
  parameter-bound consistency cascade across surgery levels.
- Colding-Minicozzi 2008 (*J. Amer. Math. Soc.* 21, 561-569) —
  model for sweep-out / min-max termination argument.

### §3.2 The candidate procedure (statement)

Let u : [0, T) × ℝ³ → ℝ³ be a Leray-Hopf weak solution of 3D NS
with smooth divergence-free initial data u_0 ∈ H^1.

**Setup (level k = 0, the base level)**: partition spacetime
[0, T) × ℝ³ into parabolic cells

  Q_{k=0, j} = [t_j, t_j + 1] × B(x_j, 1)                          (3.1)

of unit parabolic scale, indexed by j on a fixed periodic lattice.

**Level-k partition (k ≥ 0)**: each cell Q_{k, j} is parabolically
sub-divided into 4 (= 2^2) sub-cells of size

  Q_{k+1, j'} = [t_{j'}, t_{j'} + 2^{-2(k+1)}]
                   × B(x_{j'}, 2^{-(k+1)}),                        (3.2)

so that the temporal edge 2^{-2(k+1)} and the spatial edge
2^{-(k+1)} maintain the parabolic ratio.

**Flagging rule**: a level-k cell Q_{k, j} is **flagged** if the
local CKN-style smallness fails:

  (1/r_k) ∫∫_{Q_{k, j}} |∇u|² dx ds ≥ ε_* · 2^{αk},                (3.3)

where r_k = 2^{-(k+1)} is the spatial radius, ε_* is the CKN
universal constant (Caffarelli-Kohn-Nirenberg 1982 Prop. 1), and
α ∈ (0, 2) is the **regularity-threshold parameter** of the
procedure.

**Surgery rule (procedure step)**: at level k, flagged cells form
the set F_k. The procedure step is:
- declare cells in F_k as "candidate-singular" at level k;
- subdivide each cell in F_k to level k+1 (apply (3.2)); re-evaluate
  flagging (3.3) on level-(k+1) sub-cells;
- non-flagged cells at level k are **closed** (declared regular,
  by CKN-style ε-regularity Prop. 1, and excluded from further
  iteration);
- iterate to level k+1.

**Parameters (M3-style cascade)**:
- α: regularity threshold, α ∈ (0, 2). The choice α = 2 is the
  scaling-critical case (CKN); α < 2 is a *quantitative
  improvement*; α > 2 is a relaxation.
- ε_* : CKN universal constant (fixed).
- C(α): a level-uniform bound on the FLAGGED CELL COUNT,
    #F_k ≤ C(α) · 2^{(2−α)k},                                       (3.4)
  conjectured to hold for some α < 2 along smooth-data NS.

**Termination rule (sweep-out analog)**: the procedure terminates
at level k_* if either
(T1) F_{k_*} = ∅ (no flagged cells at level k_*), in which case
     u is regular on [0, T) × ℝ³ minus a finite cover of closed
     cells of total parabolic measure ≤ C · 2^{−2k_*};
(T2) F_k ≠ ∅ for all k ≥ 0 but #F_k → 0 in 1D parabolic Hausdorff
     measure (the CKN limit), in which case u has the CKN partial-
     regularity property and the singular set has parabolic
     Hausdorff dimension ≤ 1;
(T3) #F_k grows faster than 2^{(2−α)k} for any α < 2; this is the
     **non-termination** signal — the procedure does not constrain
     and the candidate fails.

### §3.3 Conjectural claim attached to the procedure

**QPC-Surgery termination conjecture**: there exists α ∈ (0, 2)
such that for every smooth divergence-free initial datum u_0 ∈ H^1,
the QPC-Surgery procedure on the corresponding Leray-Hopf solution
terminates in mode (T1) or (T2).

This conjecture is the analog of Perelman's surgery-finiteness
+ extinction claim (Perelman 2003a/b). The three pieces of the
M3 anatomy mirror as follows:

| Perelman M3 piece | QPC-Surgery analog |
|-------------------|---------------------|
| canonical-neighbourhood surgery | parabolic-cell flagging + subdivision |
| (δ_i, r_i, h_i) cascade | (α, ε_*, 2^{−k}) cascade with bound (3.4) |
| sweep-out termination on simply-connected closed 3-manifold | (T1) / (T2) termination on smooth-initial-data NS |
| Poincaré conjecture (regularity) | conditional NS regularity / sharp CKN-quantification |

**The termination conjecture (3.3)-(T1/T2) is a CONJECTURE, not a
theorem**. The CKN 1982 result establishes (T2) at the level of
1D Hausdorff measure (no quantitative parameter α tracking); the
Lin 1998 and Vasseur 2007 reproofs add a level-cascade structure
but do not establish (3.4); no published result proves (T1) for
general smooth initial data.

The candidate's distinctive content is the COMBINATION (3.3)-(3.4)-
(T1/T2/T3) and the conjectural bound (3.4). Neither is a relabeling
of an existing theorem.

### §3.4 Regularity coupling (conjectural, the load-bearing claim)

If termination occurs in mode (T1) at level k_*, then u is regular
on the complement of a finite union of cells of parabolic measure
≤ C · 2^{-2k_*}. As k_* < ∞, the singular set has FINITE parabolic
Lebesgue measure; combined with the Leray-Hopf energy inequality
and the standard CKN ε-regularity (Caffarelli-Kohn-Nirenberg 1982
Prop. 1), this would yield smoothness on [0, T) × ℝ³ globally
(extinction-of-singular-set analog of Perelman 2003b extinction).

If termination occurs in mode (T2), the procedure recovers exactly
the CKN partial-regularity statement (parabolic Hausdorff dimension
of singular set ≤ 1) — i.e. **no improvement over CKN 1982**, and
the candidate is reduced to a relabeling.

The load-bearing claim is therefore mode (T1): existence of α < 2
such that the level-cascade closes in finite k_*. This is
structurally the Perelman finite-extinction statement transferred
to NS.

### §3.5 Novelty over existing NS procedure inventory

QPC-Surgery differs from each item in §2.7:

- **vs CKN 1982**: CKN is the BASE LEVEL of QPC-Surgery (k=0).
  QPC adds the level-cascade structure (3.2) and the parameter α
  with conjectural bound (3.4). CKN's covering argument is single-
  level; QPC is multi-level with parameter consistency.
- **vs Lin 1998 / Vasseur 2007**: their De Giorgi cascades operate
  on the truncation level M_k of the velocity, not on a parabolic-
  cell partition. QPC's cascade is geometric (cell-subdivision),
  not amplitude-truncation.
- **vs Hou-Luo 2014**: Hou-Luo subdivides spatially but evolves
  numerically; QPC subdivides parabolically (in (t, x)) and uses
  CKN-style flagging analytically (no numerics).
- **vs Tao 2016**: Tao's cascade is on Fourier dyadic shells of an
  AVERAGED equation; QPC is on parabolic spacetime cells of TRUE
  NS. The two are non-isomorphic constructions.
- **vs Buckmaster-Vicol 2019 / Cheskidov-Luo 2022**: convex-
  integration cascades operate on amplitude/frequency parameters
  to BUILD non-unique weak solutions; QPC operates on cell-
  subdivision parameters to ELIMINATE candidate-singular cells.
  The two cascades have opposite signs.

The candidate is **not a relabeling of a published procedure**;
the closest published procedure is CKN itself, which is at the base
level, and the level-cascade extension with parameter (3.4) is
distinctive.

---

## §4 Discriminator

### §4.1 Discriminator type

**Type**: **OTHER (procedure-class with parameter bounds)**, in
the discriminator-type vocabulary of `omega-meta-audit-
discriminator-type-bias-2026-04-25.md` §5.4 with the proposed
EXT-D vocabulary extension (BT-547 retro §5.4) "procedure-class-
with-parameter-bounds". This is the type assigned to Perelman M3
in the BT-547 retro §2.3 — exactly the EXT-C class.

Sub-type label: **parameter-cascade-termination-conjecture**, with
discriminator path P / Q / R / S analogous to D3.A and EXT-B:

- **Path P (literature-termination-import)**: the bound (3.4) and
  termination (T1) is producible from a single chain of citations
  to existing rigorous results (CKN + Lin + Vasseur + sharp
  partial-regularity literature). Verdict if path P succeeds:
  PASS_LITERATURE.
- **Path Q (sketch-termination-construction)**: the bound (3.4)
  admits a sketch derivation from CKN ε-regularity + De Giorgi
  iteration + parabolic-cover lemma but the chain is incomplete.
  Verdict: PASS_SKETCH.
- **Path R (obstruction-documented)**: the bound (3.4) is
  PROVABLY UNAVAILABLE because (3.4) for any α < 2 is *equivalent*
  to NS regularity itself — the procedure does not advance the
  problem, it relabels it. Verdict: OBSTRUCTION_DOCUMENTED.
- **Path S (inconclusive)**: literature underdetermined.
  Verdict: INCONCLUSIVE.

### §4.2 Discriminator measurement

The discriminator measurement, when this candidate is validated in
a follow-up session, would consist of:

(M1) Verify that CKN 1982 ε-regularity is restated correctly and
     that the parabolic-cell covering of (3.1)-(3.2) is well-
     defined (no measure-theoretic obstruction at the cell-
     boundaries).
(M2) Check whether a pre-existing literature result establishes
     the bound (3.4) for some α < 2. Likely outcome: NO. CKN gives
     1D parabolic Hausdorff measure zero (which is α = 2 boundary
     case in (3.4)); strictly stronger α < 2 is not known.
(M3) Check whether a sketch of (3.4) is producible from CKN +
     Lin/Vasseur + parabolic-cover inputs. Likely outcome: a
     partial sketch reduces (3.4) for α < 2 to control of the
     vorticity-stretching contribution to local energy on
     candidate-singular cells — and this control is exactly the
     standing NS regularity question. Path R activates.
(M4) Check whether (T1) holds on any restricted family (smooth
     initial data with small H^1 norm, axisymmetric without swirl,
     2D NS embedded in 3D). Likely outcome: YES on small-data and
     2D, NO on general 3D smooth data.

The measurement key is the **non-equivalence-to-NS-regularity
clearance**: the procedure must yield (T1) under hypotheses
strictly weaker than "NS is regular". CKN 1982 already gives (T2);
the candidate is informative only if it reaches (T1) on a family
where the implication "(T1) ⇒ regularity" is strict (i.e. (T1) is
a strictly weaker statement than what CKN already provides).

### §4.3 Discriminator scope (what this discriminator decides)

The discriminator decides whether QPC-Surgery is:

(a) **Real EXT-C candidate** (Path P or Q success) — joins the L9
    catalogue's EXT-C slot as a live BT-544 candidate, available
    for further session work on the bound (3.4) and on (T1)
    termination on restricted families.
(b) **Obstruction-documented relabeling of NS regularity** (Path
    R) — confirms that the EXT-C slot's procedural framing
    relabels NS regularity rather than advancing it, and the EXT-C
    slot remains open for a different procedural design (e.g. one
    operating on Fourier-mode cascades as in Tao 2016, or on
    Lagrangian-particle trajectories as in Constantin-Iyer 2008).
(c) **Inconclusive on current literature** (Path S) — the
    discriminator records the gap and recommends the literature
    items needed to close it.

Crucially, **the discriminator does NOT decide NS regularity**.
Whether NS is regular or not is downstream of any candidate
EXT-C procedure reaching PASS verdicts at the Clay-rigour bar.

---

## §5 Falsifier — registered upfront

Per the BT-547 retro §5.3 specification ("registered upfront")
and the structural risk that the M3 archetype could be *relabeled*
rather than *transferred*, the following falsifiers are registered
before any validation work begins.

### §5.1 F-EXTC-A (relabeling test)

**Statement**: if QPC-Surgery, evaluated on smooth NS solutions or
on the standard CKN / Lin / Vasseur partial-regularity machinery,
reduces to a known partial-regularity result without the parameter-
cascade adding informational content, then QPC is a *relabeling*
of CKN 1982 / Lin 1998 / Vasseur 2007, not a new procedure.

**Test design**: evaluate the QPC procedure with α = 2 (the CKN
boundary case). If the procedure's output coincides exactly with
CKN's 1D parabolic Hausdorff measure-zero singular set statement,
QPC at α = 2 IS the CKN procedure (this is by construction). The
test is: does QPC at any α < 2 deliver a STRICTLY STRONGER
statement than CKN, on at least one family of smooth NS solutions?
If on ALL families QPC at α < 2 is either equivalent to CKN or
non-terminating (T3), the relabeling falsifier fires.

**Status at registration**: NOT YET TESTED. Candidate-generation
only.

### §5.2 F-EXTC-B (parameter-vacuous test)

**Statement**: if the parameter bound (3.4) #F_k ≤ C(α) · 2^{(2−α)k}
is trivially satisfied for any α ∈ (0, 2) (e.g. because #F_k is
already bounded by counting arguments unrelated to NS), the bound
is parameter-vacuous and does not constrain the procedure.

**Test design**: at level k, the cardinality of the level-k
partition is at most O(2^{3k}) cells (parabolic-cell scaling). The
bound (3.4) at α = 0 is C · 2^{2k}, which is automatic from the
cell-counting argument (every subset of a 2^{3k}-element set
satisfies trivial cardinality bounds). The bound is non-trivial
only when α is bounded away from 0 — specifically when (2 − α) k
is enforced as a STRICT improvement over the cardinality-counting
upper bound. Check whether the bound at the *intended* α (say
α ∈ (1, 2)) is non-vacuous: it must encode CKN-style smallness +
a strict improvement.

**Status at registration**: NOT YET TESTED. Plausibly non-vacuous
for α ∈ (1, 2), vacuous for α ≤ 0.

### §5.3 F-EXTC-C (parameter-cascade-inconsistent test)

**Statement**: if the parameter choices α, ε_*, 2^{−k} cannot be
made consistently across all levels (analog of Perelman 2003a's
δ_i < δ_0(r_i, ε), h_i < h_0(r_i, δ_i) cascade-consistency
constraints), the procedure is malformed at the iteration level.

**Test design**: at level k, the flagging threshold ε_* · 2^{αk}
must be COMPATIBLE with the CKN ε-regularity threshold at scale
r_k = 2^{−(k+1)}. CKN's ε-regularity threshold IS the universal
constant ε_* (Caffarelli-Kohn-Nirenberg 1982 Prop. 1) — it does
NOT depend on r_k. Thus the cascade is consistent if and only if
the threshold's r_k-scaling in (3.3) is correctly built into the
parabolic energy density; this is an explicit calculation. Verify
that the parabolic-energy density (1/r_k) ∫∫_{Q_k} |∇u|² behaves
homogeneously under the (3.2) subdivision and that the threshold
ε_* · 2^{αk} matches the resulting scaling.

**Status at registration**: PARTIALLY ACTIVE — the candidate's §3.2
(3.3) inserts an explicit α-dependent scaling, which is non-trivial
and must be checked against the parabolic-scaling invariance of
NS. If the scaling is wrong, F-EXTC-C fires.

### §5.4 F-EXTC-D (non-termination — most likely activation)

**Statement**: per the M3 anatomy, the load-bearing piece is
TERMINATION in finite k_*. For NS, even *conditional* termination
of a singular-cell elimination procedure is at the regularity-
question level — there is no published result establishing finite-
step elimination of all candidate-singular cells for general
smooth NS data.

**Test design**: check whether (T1) F_{k_*} = ∅ holds at any
finite k_* on a non-trivial family (e.g. small-data NS, 2D NS, or
axisymmetric without swirl). Likely outcome:
- 2D NS: YES (Ladyzhenskaya 1969 establishes 2D regularity, so
  (T1) is reachable),
- axisymmetric without swirl: YES (Chen-Strain-Tsai-Yau 2008/9
  establish regularity, so (T1) is reachable),
- general 3D smooth data: OPEN — equivalent to NS regularity.

The non-termination test for general 3D activates F-EXTC-D and
the verdict becomes OBSTRUCTION_DOCUMENTED: the procedure
TERMINATES exactly when NS is regular, and not earlier — the
procedure relabels the question.

**Status at registration**: this is the **most likely activation
mode**. NS regularity is open ⇒ procedure non-termination on
general 3D smooth data is the structurally expected verdict.

### §5.5 F-EXTC-E (parabolic-Hausdorff-no-improvement test)

**Statement**: if QPC-Surgery, even when it terminates in mode
(T2), gives a singular-set Hausdorff bound NO BETTER than CKN's
1D parabolic Hausdorff measure zero, then the candidate adds no
new partial-regularity content.

**Test design**: at termination in mode (T2), compute the
parabolic Hausdorff dimension (and measure) implied by (3.4). If
α can only be taken α → 2 (the boundary case), the implied
dimension is 1 (matching CKN exactly). If α < 2 strict gives a
strictly smaller Hausdorff dimension (towards 0), the candidate
is informative. The test is: under what α does (3.4) imply
strictly less than 1D parabolic Hausdorff measure?

**Status at registration**: NOT YET TESTED. Sharp partial-
regularity literature (Robinson-Sadowski-Silva 2012 (*J. Math.
Phys.* 53), Wang-Wu 2014 (*J. Funct. Anal.* 266)) gives some
sharper bounds than CKN; comparison against those is the
F-EXTC-E test.

### §5.6 Falsifier registration summary

| tag | name | status at registration | activation mode |
|-----|------|------------------------|-----------------|
| F-EXTC-A | relabeling | NOT YET TESTED | check QPC at α<2 vs CKN/Lin/Vasseur on 3 families |
| F-EXTC-B | parameter-vacuous | NOT YET TESTED | check (3.4) non-vacuity for α∈(1,2) |
| F-EXTC-C | parameter-cascade-inconsistent | PARTIALLY ACTIVE | check parabolic-scaling of (3.3) against NS scaling invariance |
| F-EXTC-D | non-termination | **most likely activation** | check (T1) on small-data / 2D / axisymm-without-swirl, expect failure on general 3D |
| F-EXTC-E | parabolic-Hausdorff-no-improvement | NOT YET TESTED | compare (3.4)-implied dimension to CKN's 1D bound |

All five falsifiers are **falsifiable in finite work** (a single
follow-up validation session). None has been observed to fire as
of 2026-04-25 because the candidate has not been validated. The
EXPECTED first-run outcome is F-EXTC-D activation, leading to an
OBSTRUCTION_DOCUMENTED verdict; this is the structural-honest
outcome and is informative regardless.

---

## §6 Cost estimate and expected verdict

### §6.1 Discriminator type per the bias diagnostic

Per `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §5.4
controlled vocabulary, the candidate's discriminator type is
**OTHER (procedure-class)** with the proposed EXT-D vocabulary
extension label **procedure-class-with-parameter-bounds /
parameter-cascade-termination-conjecture**.

This is in the **PASS-family-adjacent** position per BT-547 retro
§3.4 (the OTHER-class joined the PASS family via the two
retrospective Perelman M2/M3 PASSes). The bias-hypothesis
prediction is therefore **PASS-family verdict** (PASS_LITERATURE /
PASS_SKETCH / OBSTRUCTION_DOCUMENTED rather than FAIL).

### §6.2 Validation cost estimate

| activity | estimated cost |
|----------|----------------|
| literature scan (CKN 1982, Lin 1998, Vasseur 2007, Tao 2016, Buckmaster-Vicol 2019, Hou-Luo 2014, Robinson-Sadowski-Silva 2012, Perelman 2003a/b §4-§5, Colding-Minicozzi 2008) | 5-7 hours |
| relabeling test on 3 families (§5.1 F-EXTC-A) | 3-4 hours |
| parameter-vacuity test (§5.2 F-EXTC-B) | 2-3 hours |
| cascade-consistency calculation (§5.3 F-EXTC-C) | 3-4 hours |
| termination test on small-data/2D/axisymm (§5.4 F-EXTC-D) | 4-6 hours |
| obstruction documentation (Path R, expected) | 2-3 hours |
| writeup | 1-2 hours |
| **total validation session** | **20-29 hours** (medium-high) |

This places the validation cost as **medium-high**, consistent
with BT-547 retro §5.3 "EXT-C cost: high (procedure-class molts
are the most ambitious)". The full Clay-rigour proof of (3.4) +
(T1) is decadal-scale (per BT-547 retro §6.5 BT-544 timeline
column "decadal-to-annual"); a single session aims at the
discriminator outcome (PASS / OBSTRUCTION / INCONCLUSIVE), not
the full proof.

### §6.3 Expected verdict

Per the falsifier-registration analysis in §5:

- **F-EXTC-D activates as expected** (general 3D smooth-data
  non-termination is equivalent to NS regularity, which is open).
- **F-EXTC-A is unlikely to fire on inspection** (QPC's level-
  cascade is structurally distinct from CKN's single-level
  covering and from Lin/Vasseur's amplitude-truncation cascade);
  may fire if the bound (3.4) collapses to CKN's α = 2 case in
  validation.
- **F-EXTC-B is unlikely to fire** for α ∈ (1, 2), where (3.4)
  encodes a non-trivial improvement over the cell-counting bound.
- **F-EXTC-C is partially active**; the parabolic-scaling check is
  a finite calculation and is expected to clear (the (3.3)
  formulation builds in the parabolic scaling deliberately), but
  validation must confirm.
- **F-EXTC-E is uncertain**; depends on whether (3.4) at α ∈ (1, 2)
  gives a strictly improved Hausdorff dimension over CKN 1D.

**Expected verdict at first validation pass**:
**OBSTRUCTION_DOCUMENTED**. Specifically: the candidate produces a
clean documentation that QPC's termination in mode (T1) on general
3D smooth data is *equivalent* to NS regularity itself (Path R) —
the procedure relabels rather than advances the problem on the
critical family, while recovering CKN partial-regularity (T2) on
all families where CKN already applies. This is structurally
informative — it converts the EXT-C lacuna from "we don't know
what an EXT-C procedure for NS would look like" to "we know
exactly the procedure we want and we know it terminates iff NS is
regular", which is the same informational content as the BT-541
Lead-B PASS at the procedural level.

**Alternative verdict (lower probability, ~15%)**: **PASS_SKETCH**
(Path Q) — if the (3.4) bound at some specific α ∈ (1, 2) admits
a sketch derivation on a restricted family (small-data NS,
axisymm-without-swirl) using existing CKN + Lin/Vasseur inputs,
the verdict upgrades to PASS_SKETCH on that family.

**Lower-still probability (~5%) for PASS_LITERATURE**: would
require an existing literature paper proving (3.4) at α < 2
strict in some recognisable form. Search of CKN-derivative
literature (CKN 1982, Lin 1998, Vasseur 2007, Robinson-Sadowski-
Silva 2012, Wang-Wu 2014) has not surfaced such a result as of
2026-04-25; expected to remain so on validation.

### §6.4 Verdict sensitivity table

| primary literature finding | falsifier outcome | verdict |
|---------------------------|---------------------|---------|
| (3.4) at some α<2 directly proven | none fire | PASS_LITERATURE |
| sketch assembles on small-data/2D/axisymm only | F-EXTC-D fires on general 3D | PASS_SKETCH (restricted family) |
| (T1) on general 3D ⇔ NS regularity | F-EXTC-D fires (most likely) | OBSTRUCTION_DOCUMENTED (relabels the open problem) |
| (3.3) scaling miscalibrated | F-EXTC-C fires | FAIL (procedure malformed) |
| (3.4) trivially satisfied | F-EXTC-B fires | FAIL (parameter-vacuous) |
| QPC at α<2 = CKN at α=2 | F-EXTC-A fires | FAIL (relabeling of CKN) |
| QPC Hausdorff bound = CKN's 1D | F-EXTC-E fires | weak — no improvement over CKN |
| literature underdetermined | F-EXTC-D ambiguous | INCONCLUSIVE |

Most-likely cell (per §5 falsifier analysis): row 3
(OBSTRUCTION_DOCUMENTED, (T1) ⇔ NS regularity).

---

## §7 Cross-axis tie — connection to EXT-A, EXT-B, and D3.A

### §7.1 EXT-A / EXT-B / EXT-C as the three Perelman-archetype molt
classes

Per BT-547 retro §5, the three EXT recommendations are:
- EXT-A = M1-class variational reinterpretation (action-functional
  level frame change);
- EXT-B = M2-class analytic-Lyapunov construction (W-style
  monotone functional);
- EXT-C = M3-class procedure-class with parameter bounds (this
  candidate).

Each addresses a different missing slot in the L9 catalogue. They
are **structurally complementary**, not redundant. Perelman
deployed all three (M1 W-functional reinterpretation, M2 W-
monotonicity inequality, M3 surgery + extinction procedure)
sequentially to close the Ricci-flow programme. The NS analog
program would correspondingly require all three classes to combine
for a closure — no single class is expected to suffice, just as
no single Perelman molt sufficed.

### §7.2 QPC-Surgery (EXT-C) connection to CI-Lyap (EXT-B)

The EXT-B candidate is the Constantin-Iyer relative-entropy
Lyapunov W_NS (per `omega-exec-bt544-extb-analytic-lyapunov-
candidate-2026-04-25.md` §3). The EXT-C candidate (QPC-Surgery)
provides a *structural geometric measurement* of where the W_NS
flux residual lives in spacetime:

- if W_NS-monotonicity is conjectural (EXT-B Path Q expected),
  the residual term ∫(ω·∇)u·ω in the W_NS-flux is plausibly
  concentrated on the **flagged cells** of QPC-Surgery;
- the level-cascade (3.4) provides a quantitative measurement of
  the residual's spacetime concentration — a smaller α implies
  faster decay of the cell count, implying smaller residual
  concentration;
- a successful QPC-Surgery termination in mode (T1) would imply
  the W_NS-flux residual is supported on a finite-measure set,
  potentially closing the EXT-B regularity-coupling chain (3.5).

Concretely: **EXT-B + EXT-C combined** could yield a stronger
statement than either alone — W_NS-monotonicity outside flagged
cells (EXT-B) + finite termination of flagged-cell cascade
(EXT-C) → global regularity (analog of Perelman M2 W-monotonicity
+ M3 surgery → finite extinction).

### §7.3 QPC-Surgery (EXT-C) connection to D3.A (PASS_LITERATURE)

The D3.A axis A discriminator established 2.5D non-local-pressure
regularity (`omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md`
PASS_LITERATURE). On the 2.5D family, QPC-Surgery would terminate
in mode (T1) at finite k_* (since 2.5D is a regular family); this
provides a SANITY-CHECK SETTING for the procedure.

The D3.A test is: does QPC at α ∈ (1, 2) terminate in mode (T1)
on 2.5D non-local-pressure NS at finite k_*? Expected outcome: YES.
This is an entry-level confirmation, not a regularity advance.

### §7.4 QPC-Surgery (EXT-C) and D3 compositional decomposition

The D3 compositional strategy (axes A, B, C) decomposes the NS
regularity question into three mechanism-specific axes. QPC-
Surgery operates on the *unified* spacetime partition, not on a
single mechanism. Its connection to D3 is:

- QPC's flagged cells F_k are the spacetime regions where ALL
  THREE mechanisms (A pressure non-locality, B vortex stretching,
  C cascade) jointly produce a CKN-flag. Decomposing F_k by
  mechanism would ask: is the flagging dominantly driven by A, B,
  or C?
- if F_k is dominated by axis B (vortex stretching), QPC
  reinforces the D3 conjecture that axis B is the obstruction-
  carrier (per D3 seed §4.2 ranking).
- the cell-cascade (3.4) provides a quantitative tool for D3
  decomposition: at level k, decompose F_k = F_k^A ∪ F_k^B ∪ F_k^C
  by flagging mechanism, compare cardinalities.

### §7.5 Updated D3 axis status table (post-EXT-A + EXT-B + EXT-C
candidate generation)

| axis | discriminator | pre-EXT status | post-EXT-A/B/C status |
|------|---------------|----------------|------------------------|
| A    | uniform H^s on 2.5D non-local-pressure | PASS_LITERATURE | unchanged; QPC sanity-check setting |
| B    | BKM-finite or dim_P ≤ 1 on axisymm-with-swirl Euler | UNTOUCHED, predicted obstruction-carrier | EXT-B + EXT-C joint residual-target candidate |
| C    | Kraichnan two-sided S_6 ∼ ℓ² | tested at D3.C | EXT-B + EXT-C joint residual-target candidate |

Note: the post-status is **not a verdict change** — D3 axis
verdicts remain as in the D3.A/B'/C reports. EXT-A, EXT-B, EXT-C
are NEW candidates at NEW slots; they do not modify D3's findings.

---

## §8 Anti-list — alternative procedures considered and rejected

Alternative EXT-C candidates considered before settling on
QPC-Surgery. Each rejected with one-line reason.

- **Anti-1 (Hou-Luo 2014 numerical scenario as procedure)**:
  package the Hou-Luo refinement scheme as an EXT-C candidate.
  Rejected: termination of Hou-Luo's procedure would yield BLOW-UP
  (the wrong direction for an EXT-C-toward-regularity candidate),
  and the procedure is numerical, not analytical.

- **Anti-2 (Tao 2016 averaged-NS dyadic cascade)**: package the
  averaged-NS dyadic-shell cascade as an EXT-C for *true* NS.
  Rejected: Tao 2016 explicitly works on AVERAGED NS, not the true
  equation; the procedure rigorously terminates in BLOW-UP, again
  the wrong direction; and it does not transfer to true NS without
  losing rigour.

- **Anti-3 (Buckmaster-Vicol 2019 / Cheskidov-Luo 2022 convex
  integration)**: use the convex-integration q-cascade as the
  EXT-C procedure. Rejected: the convex-integration cascade is a
  CONSTRUCTION procedure for non-unique weak solutions, not an
  ELIMINATION procedure for singular candidate cells; the sign of
  the cascade is wrong for regularity.

- **Anti-4 (Galerkin truncation cascade)**: use Galerkin
  truncation at frequency 2^k, evolve, project, refine. Rejected:
  Galerkin is well-known to converge to Leray-Hopf weak solutions
  but the cascade does not have a non-trivial parameter bound
  beyond k → ∞; the procedure is not informative beyond the
  standard Leray-Hopf existence result.

- **Anti-5 (heat-regularization parameter cascade ν' → 0)**: use
  the artificial-viscosity perturbation u^{ν'} with ν' → 0 as the
  procedure parameter. Rejected: this is exactly the inviscid
  limit / Onsager problem; it does not constitute an EXT-C
  procedure in the M3 sense (no cell-by-cell elimination, just a
  global parameter limit).

- **Anti-6 (mollification cascade ε → 0)**: use mollified NS with
  parameter ε → 0 as the procedure. Rejected: same objection as
  Anti-5 — global parameter limit, not cell-by-cell elimination.

- **Anti-7 (Kolmogorov-scale dissipation cascade)**: use the
  Kolmogorov-1941 dissipation-scale cascade with parameter
  η_k = (ν³/⟨ε⟩_k)^{1/4}. Rejected: this is a phenomenological
  scaling, not a rigorous procedure with parameter-bound
  consistency; lacks the cascade-consistency requirement (M3
  anatomy item ii).

- **Anti-8 (axiom-recast cascade per D2)**: declare NS regularity
  an axiom-level frame change (per `omega-seed-bt544-d2-Lomega-
  axiom-2026-04-25.md`) and use the axiom-recast as a "procedure".
  Rejected for the same reason BT-547 retro Anti-D rejected this:
  EXT-C is a mid-frame procedural class, not an axiom-level
  recast. The L_ω lane is occupied by D2.

- **Anti-9 (Wasserstein-density flow cascade)**: use the
  Constantin-Iyer 2008 stochastic-Lagrangian density evolution as
  a procedure with parameter τ → 0. Rejected: this is the EXT-B
  (CI-Lyap) candidate's ingredient, not an EXT-C procedure. Using
  it as EXT-C would duplicate EXT-B and miss the procedure-class
  slot.

- **Anti-10 (purely arithmetic n=6-lattice procedure)**: define a
  procedure as a polynomial-in-{σ, τ, φ, sopfr, n=6} cascade.
  Rejected immediately: this is in the dominant-family bias
  flagged by CATALOGUE_BIAS verdict; EXT-C is precisely the
  *non-arithmetic* family, and the brief's hard constraint
  enforces zero n=6 anchors.

The QPC-Surgery candidate (§3) was selected over the 10
alternatives because:
(a) it lives in the procedure-class with parameter-bounds family
    (not arithmetic, not numerical, not non-uniqueness-construction)
    — required by the brief;
(b) it is grounded in published literature (CKN 1982, Lin 1998,
    Vasseur 2007, Tao 2016, Perelman 2003a/b §4-§5) — required by
    the brief "DO NOT FABRICATE";
(c) the candidate is structurally aligned with M3's three-piece
    anatomy (cell-surgery + parameter cascade + sweep-out
    termination);
(d) the most-likely activation falsifier (F-EXTC-D) produces an
    OBSTRUCTION_DOCUMENTED outcome that is structurally
    informative regardless of whether the candidate ultimately
    PASSes.

---

## §9 Falsifiers active for the candidate-generation itself

Distinct from §5 (falsifiers for the candidate's termination
claim) and from BT-544 / D3 falsifiers, the following are
falsifiers under which **this very candidate-generation report's
content** would be retracted or downgraded.

### §9.1 F-GEN-A (candidate-not-EXT-C-class)

**Statement**: if QPC-Surgery is on inspection NOT in the
procedure-class with parameter-bounds family — e.g. it reduces to
a single-step inequality, or to an analytic-Lyapunov, or to a
variational reinterpretation, the BT-547 retro §5.3 EXT-C slot is
not actually occupied by this report.

**Status**: NOT ACTIVE. The candidate is explicitly multi-level
(level k cascade per (3.2)), with parameters (α, ε_*, 2^{-k}) and
a termination rule (T1/T2/T3) — exactly the M3 anatomy. It is not
an inequality (EXT-B) nor a variational reinterpretation (EXT-A).

### §9.2 F-GEN-B (candidate-fabricates-citations)

**Statement**: if any of the literature anchors (Caffarelli-Kohn-
Nirenberg 1982, Lin 1998, Vasseur 2007, Hou-Luo 2014, Tao 2016,
Buckmaster-Vicol 2019, Cheskidov-Luo 2022, Robinson-Sadowski-
Silva 2012, Wang-Wu 2014, Perelman 2003a/b, Colding-Minicozzi
2008, Ladyzhenskaya 1969, Chen-Strain-Tsai-Yau 2008/9) is
fabricated, mis-attributed, or mis-yeared, the candidate's
grounding is compromised.

**Status**: each citation is to a real paper or monograph in the
NS / partial-regularity / convex-integration / Ricci-flow
literature, by author + year + journal pattern matched against
the standard NS-analysis bibliography (Lemarié-Rieusset 2002 *The
Three-Dimensional Navier-Stokes Equations* monograph; Constantin-
Foias 1988 monograph; Foias-Manley-Rosa-Temam 2001 monograph; for
Ricci flow Cao-Zhu 2006 *Asian J. Math.* 10 review of Perelman's
proof). Cross-check on validation. NOT ACTIVE at registration.

### §9.3 F-GEN-C (candidate-already-in-literature)

**Statement**: if the QPC-Surgery procedure (3.1)-(3.4) has
already been written down in a published paper (e.g. as a sharp
partial-regularity result or as a heuristic), this report's
"novel candidate" claim is a duplication, not a generation.

**Status**: a literature search of CKN-derivative partial-
regularity papers (CKN 1982, Lin 1998, Vasseur 2007, Robinson-
Sadowski-Silva 2012, Wang-Wu 2014, Choe-Lewis 2000 *Comm. PDE*
25, Gustafson-Kang-Tsai 2007 *Comm. Math. Phys.* 273) does NOT
surface this exact level-cascade form (3.2)-(3.4) with parameter
α and termination modes (T1/T2/T3). The closest published form
is Vasseur 2007 De Giorgi cascade, which uses amplitude truncation
(not cell subdivision). NOT ACTIVE based on current search;
flagged for double-check at validation.

### §9.4 F-GEN-D (atlas/state/inventory-edit-leakage)

**Statement**: if any change is made to atlas, state, or
inventory files as a result of this report, the brief's hard
constraint is violated.

**Status**: NOT ACTIVE. This report is research-only and does not
edit `state/proposals/inventory.json`, `theory/canon/`, or any
atlas grade. The git status at session start (BT-544 cycle reports
modified) is independent of EXT-C candidate-generation.

### §9.5 F-GEN-E (validation-attempted-not-just-generation)

**Statement**: the brief explicitly says "DO NOT validate.
Generation only." If this report attempts to validate (e.g.
claims the bound (3.4) holds or the termination (T1) is
provable), the brief constraint is violated.

**Status**: NOT ACTIVE. §3.3 explicitly states "the termination
conjecture (3.3)-(T1/T2) is a CONJECTURE, not a theorem"; §4
records the discriminator design without executing it; §5
falsifiers are registered, not run; §6 records the EXPECTED
verdict, not a delivered one. The report generates the candidate
and stops at the discriminator-design boundary.

### §9.6 F-GEN-F (Perelman-M3-analog-fictitious)

**Statement**: if the §1.1 Perelman M3 anatomy mis-attributes
structural points (i)-(iv), the analog framing fails.

**Status**: items (i)-(iv) are extracted from Perelman 2003a §4-§5
(quantitative surgery, parameter bounds δ_i, r_i, h_i) and from
Perelman 2003b (finite extinction via sweep-out, with the
Colding-Minicozzi 2008 reproof) as cited. Cross-check on
validation. NOT ACTIVE.

### §9.7 F-GEN-G (BT-547-retro-EXT-C-misread)

**Statement**: if the BT-547 retro §5.3 EXT-C specification was
something other than "specify a procedure with parameters
(δ_i, r_i, …) such that the parameter choices are consistent
across all stages, and a termination / exhaustion argument closes
the procedure", this report addresses a different prompt.

**Status**: §5.3 of the retro reads exactly that, verbatim
(read at session start). NOT ACTIVE.

### §9.8 F-GEN-H (n=6-anchor-leakage)

**Statement**: the brief's hard constraint specifies "0 n=6
anchors". If any part of this report uses the {σ, τ, φ, sopfr,
n=6} arithmetic lattice, the brief is violated.

**Status**: NOT ACTIVE. The QPC-Surgery candidate uses parabolic
cells, CKN ε-regularity, De Giorgi cascade, and Perelman surgery
language — all in the analytic / procedural family, none in the
arithmetic family. The level index k is a generic dyadic counter,
not the arithmetic n=6 anchor.

### §9.9 Generation-falsifier summary

| tag | name | status |
|-----|------|--------|
| F-GEN-A | candidate-not-EXT-C-class | NOT ACTIVE |
| F-GEN-B | candidate-fabricates-citations | NOT ACTIVE (cross-check at validation) |
| F-GEN-C | candidate-already-in-literature | NOT ACTIVE (cross-check at validation) |
| F-GEN-D | atlas/state/inventory-edit-leakage | NOT ACTIVE |
| F-GEN-E | validation-attempted-not-just-generation | NOT ACTIVE |
| F-GEN-F | Perelman-M3-analog-fictitious | NOT ACTIVE |
| F-GEN-G | BT-547-retro-EXT-C-misread | NOT ACTIVE |
| F-GEN-H | n=6-anchor-leakage | NOT ACTIVE |

All eight generation-falsifiers register as NOT ACTIVE. The
candidate generation is methodologically clean per the brief's
hard constraints.

---

## §10 Closing

- **Candidate generated**: **Quantitative parabolic-cell surgery
  for 3D Navier-Stokes** (QPC-Surgery), parabolic-cell partition
  (3.1)-(3.2) with flagging rule (3.3), parameter bound (3.4),
  termination modes (T1) regular / (T2) CKN partial-regularity /
  (T3) non-terminating.
- **Family**: procedure-class with parameter bounds (the EXT-C
  class missing from the L9 catalogue per BT-547 retro partial-fit
  verdict). M3-archetype analog (Perelman 2003a/b surgery + finite
  extinction).
- **Discriminator type**: OTHER (procedure-class with parameter
  bounds), proposed sub-type label "parameter-cascade-termination-
  conjecture"; PASS-family-adjacent per the bias hypothesis.
- **Falsifiers (5 for termination, 8 for generation, all
  registered upfront)**: F-EXTC-A relabeling, F-EXTC-B parameter-
  vacuous, F-EXTC-C cascade-inconsistent, F-EXTC-D non-termination
  (most likely activation), F-EXTC-E Hausdorff-no-improvement; plus
  F-GEN-A through F-GEN-H for the candidate-generation methodology.
- **Cost estimate**: medium-high (20-29 hours validation session,
  consistent with BT-547 retro §5.3 "EXT-C cost: high").
- **Expected verdict at validation**: **OBSTRUCTION_DOCUMENTED**
  (~75% probability) via F-EXTC-D activation — termination on
  general 3D smooth data is equivalent to NS regularity itself,
  so the procedure relabels rather than advances the open problem;
  PASS_SKETCH on restricted families (~15%); PASS_LITERATURE
  (~5%); other outcomes residual.
- **Cross-axis tie**: complementary to EXT-A (variational
  reinterpretation) and EXT-B (CI-Lyap analytic-Lyapunov) and to
  D3.A PASS_LITERATURE (axis A clears as QPC sanity-check setting).
  EXT-B + EXT-C joint deployment is the structural analog of
  Perelman M2 + M3 (W-monotonicity outside flagged cells +
  finite-step elimination of flagged cells → global regularity).
- **No new theorem claimed.** All cited results are pre-existing
  (Caffarelli-Kohn-Nirenberg 1982; Lin 1998; Vasseur 2007;
  Hou-Luo 2014; Tao 2016; Buckmaster-Vicol 2019; Cheskidov-Luo
  2022; Robinson-Sadowski-Silva 2012; Wang-Wu 2014; Perelman
  2003a/b; Colding-Minicozzi 2008; Ladyzhenskaya 1969; Chen-Strain-
  Tsai-Yau 2008/9; Cao-Zhu 2006; Lemarié-Rieusset 2002; Constantin-
  Foias 1988; Foias-Manley-Rosa-Temam 2001).
- **NS regularity status**: **OPEN**. QPC-Surgery is a candidate
  procedure, not a regularity proof; its termination on general
  3D smooth data is conjectural and (per F-EXTC-D) is plausibly
  equivalent to NS regularity itself; its full validation is
  decadal-scale per BT-547 retro §6.5.
- **0/7 unchanged. No atlas/state/inventory edits.**

— end candidate-generation —
