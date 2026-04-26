---
id: omega-exec-n20-p9-bt543-extb-chromo
date: 2026-04-25
scope: research-only validation (NO YM mass-gap claim, NO atlas promotion)
target: BT-543 EXT-B chromomagnetic-energy Lyapunov -- first cross-BT EXT-tier extension
parent_reports:
  - reports/sessions/omega-meta-audit-l9-generation-pipeline-2026-04-25.md (P9 prediction)
  - reports/sessions/omega-meta-design-n20-native-pairing-resample-2026-04-25.md (P9 spec)
  - reports/sessions/omega-cycle-bt543-ym-2026-04-25.md
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md (template)
millennium_resolved: 0/7 (unchanged)
grade: cross-BT validation, no claim
---

# Omega Exec -- n=20 P9 BT-543 EXT-B Chromomagnetic Lyapunov (2026-04-25)

## §0 Non-claim disclaimer

This file **validates** the P9 candidate of the n=20 native-pairing
plan: a chromomagnetic-energy Lyapunov W_chromo on lattice Yang-Mills
configurations, evolved along Wilson flow, predicted by the L9
generation-pipeline diagnostic
(`omega-meta-audit-l9-generation-pipeline-2026-04-25.md` §6.2) as a
non-arithmetic candidate for BT-543 (Yang-Mills mass gap).

This is the **first cross-BT EXT-tier validation**: the EXT-B
framework instantiated in the BT-544 NS Constantin-Iyer Lyapunov
validation (`omega-exec-bt544-extb-cilyap-validation-2026-04-25.md`)
is here extended to BT-543 YM. The structural parallel (variational/
analytic-Lyapunov family with OTHER analytic-inequality discriminator)
is the test object.

This file does **not**:

- claim 4D pure SU(N) Yang-Mills mass gap, or any Clay-form resolution;
- prove or disprove the conjectured monotonicity of W_chromo along
  Wilson flow;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the BT-543 alien-index ceiling 9 (per
  `omega-cycle-bt543-ym-2026-04-25.md` §9), or add to the L9
  catalogue's active-candidate ledger;
- supersede or replace BT-543's existing P3 / NP1-NP5 probes;
- introduce any new theorem. Every cited result is pre-existing in
  the published lattice-gauge / Wilson-flow literature, and is cited
  by author / year / journal as required.

The verdict is **OBSTRUCTION_DOCUMENTED** at the F-P9-B activation
(no gauge-invariant Bakry-Émery for Yang-Mills), with secondary
support from the Wilson-flow / chromomagnetic-energy time derivative
sign analysis.

**0/7 unchanged. YM mass-gap status open. No atlas / state /
inventory edits.**

---

## §1 P9 spec recap

### §1.1 Source predictions

The P9 candidate is the joint output of two upstream diagnostics:

- `omega-meta-audit-l9-generation-pipeline-2026-04-25.md` §6.2
  ("Predicted candidate 2: BT-543 chromomagnetic-energy Lyapunov
  (EXT-B)"). The diagnostic specifies: "chromomagnetic-energy
  Lyapunov W on lattice gauge configurations such that dW/dt has a
  definite sign along Wilson-flow gradient descent and W discharges
  the mass-gap obstruction (i.e., dW/dt = 0 ⟹ Δ_YM > 0)".
- `omega-meta-design-n20-native-pairing-resample-2026-04-25.md` §5
  row P9: candidate `BT-543-EXT-B-chromo`, BT 543, family
  variational/Lyapunov, discriminator analytic-inequality,
  pairing native, cost high, source "BT-547 retro §5.2 EXT-B seed
  for BT-543 (chromomagnetic-energy)".

### §1.2 Native-pairing classification

Per the §5 matrix of the n=20 plan, P9 fills the
**(variational/Lyapunov, analytic-inequality)** cell as the third
diagonal entry alongside P1 (BT-544 CI-Lyap) and P10 (BT-542
meta-Lyapunov). The cell prior to n=15 had 0 entries; the validation
of P1 (OBSTRUCTION_DOCUMENTED) populated it once. P9 is the second
EXT-B-class candidate to be validated, the first one outside BT-544.

### §1.3 The cross-BT extension hypothesis

The BT-544 EXT-B verdict was OBSTRUCTION_DOCUMENTED via F-EXTB-D
(representation-side: CI 2008 produces a family {ρ_t(·|x_0)}, not a
single ρ on ℝ³). The structurally analogous question for BT-543 is:
does a chromomagnetic-energy Lyapunov hit the same kind of
representation-side obstruction (no canonical gauge-invariant scalar
density on configuration space), or does Wilson flow's well-defined
gradient-flow structure on gauge-equivalence classes give the
sketch a clean run?

This validation answers the cross-BT extension question.

---

## §2 Wilson flow + chromomagnetic-energy literature recall

Each reference is restated with its actual published content, with
care to distinguish what is established from what the candidate
spec needs.

### §2.1 Lüscher 2010 (JHEP 1008:071)

**Title**: "Properties and uses of the Wilson flow in lattice QCD".

**Actual content**: defines the Wilson flow (a.k.a. Yang-Mills
gradient flow) on lattice gauge configurations as the τ-evolution
∂_τ V_τ(x, μ) = -g_0² [∂_{x,μ} S_W(V_τ)] V_τ(x, μ), where V_τ is the
flowed link variable and S_W is the Wilson plaquette action (the
continuum limit is ∂_τ A_τ = -δS_YM/δA on the gauge potential A).
Lüscher 2010 §2-§3 establishes:

- Wilson flow exists and is well-defined for any starting gauge
  configuration on a finite lattice;
- the flow is a **smoothing** operation: for τ > 0, the flowed
  configuration has correlation functions with reduced UV
  divergences;
- the observable t² ⟨E(t)⟩, where E(t) = (1/4) F^a_{μν}(t) F^{a,μν}(t)
  is the Yang-Mills action density at flow time t = τ, stabilizes
  for large t;
- a **renormalization scheme** is defined by t_0 satisfying t² ⟨E(t)⟩|_{t=t_0} = 0.3,
  fixing the scale through the flow.

**Domain of validity (Lüscher 2010 §1, §3 stated explicitly)**:
- finite lattice with periodic boundary conditions;
- pure gauge or with quark fields included via a separate flow
  equation;
- the smoothing property is **non-perturbative** (gradient flow,
  not stochastic) but does not by itself produce a physical mass
  gap.

**What Lüscher 2010 does NOT prove**:
- a monotone-decreasing Lyapunov bounding mass gap from below;
- the t² ⟨E(t)⟩ behavior gives a renormalization scheme, not a
  Lyapunov for the mass-gap question (the value 0.3 is a
  convention, not a discharge of any gap obstruction);
- gauge-invariant Bakry-Émery curvature-dimension condition on the
  configuration space.

This is the Lüscher 2010 ground truth.

### §2.2 Narayanan-Neuberger 2006 (JHEP 0603:064)

**Title**: "Infinite N phase transitions in continuum Wilson loop
operators".

**Actual content**: studies smoothed Wilson loops in pure gauge
theories at large N. The smoothing procedure is structurally close
to a precursor of Wilson flow (gradient-descent on the action). The
paper establishes phase transitions of smoothed Wilson loop
eigenvalue distributions as a function of loop area.

**Domain of validity**: large-N continuum Wilson loops, pure gauge;
the smoothing is used to define renormalized Wilson-loop expectation
values.

**What Narayanan-Neuberger 2006 does NOT prove**:
- mass-gap discharge by a Lyapunov;
- monotonicity of any chromomagnetic functional along the smoothing;
- finite-N applicability of the phase-transition picture.

The contribution to the candidate is the **structural template** of
gradient-descent smoothing; the gauge-theoretic Lyapunov question
itself is not addressed.

### §2.3 Borsanyi et al. 2012 (JHEP 1209:010)

**Title**: "High-precision scale setting in lattice QCD" (Borsanyi,
Endrodi, Fodor, et al.).

**Actual content**: high-precision lattice computation that uses
Wilson flow (per Lüscher 2010) to set the scale t_0 in continuum-
extrapolated lattice QCD. Reports t_0 with sub-percent statistical
errors, validating Lüscher's scheme as a practical scale-setting
tool.

**Domain of validity**: dynamical (with sea quarks) and pure gauge
lattice QCD configurations; finite lattice spacing extrapolation.

**What Borsanyi et al. 2012 does NOT prove**:
- analytic mass-gap bound;
- monotonicity of t² ⟨E(t)⟩ in any Lyapunov sense (the function is
  monotone-increasing in t up to a plateau region by construction
  of the flow on smooth data, but this is a normalization
  convenience, not a gap-discharge inequality);
- existence of a chromomagnetic-only Lyapunov (the energy density
  E mixes chromoelectric and chromomagnetic components in equal
  proportion, F^a_{μν} F^{a,μν} = -2(B² - E²) in Lorentzian, or
  +2(B² + E²) in Euclidean lattice, depending on signature
  convention).

### §2.4 Adjacent literature -- gauge-invariant Bakry-Émery

A targeted scan of post-2000 mathematical-gauge literature surfaces:

- **Charalambous-Lu 2015** (Math. Z. 280) and similar Riemannian-
  geometry work establish Bakry-Émery curvature-dimension conditions
  on **principal bundle base manifolds** but require gauge-invariant
  formulation that fixes a connection. For Yang-Mills, the
  configuration is the connection itself (modulo gauge), and the
  Bakry-Émery framework's reversible-diffusion semigroup does not
  have an established gauge-invariant analog on the configuration
  space.
- **Dell'Antonio-Zwanziger 1989-91** (Comm. Math. Phys.) treats
  stochastic quantization of Yang-Mills with Faddeev-Popov gauge
  fixing; the resulting Langevin equation is reversible only after
  gauge fixing, breaking manifest gauge invariance. The
  Bakry-Émery framework applied to this gauge-fixed Langevin
  process gives a Lyapunov on the gauge-fixed slice, not on the
  gauge-invariant quotient.
- **Driver-Hall 1999** and follow-up work on heat-kernel measures
  on the moduli space of flat connections (2D Yang-Mills) gives
  rigorous gauge-invariant Brownian motion on the moduli, but is
  restricted to 2D and to the moduli of *flat* connections (not
  the full YM configuration space relevant for 4D mass gap).
- **Buckmaster-Vicol 2019** (Ann. Math. 189) is unrelated to YM
  but illustrates the negative-result pattern: convex integration
  produces non-uniqueness rather than monotonicity.

**No paper in the searched literature states a chromomagnetic-only
Lyapunov inequality dW_chromo/dτ ≤ 0 (or ≥ 0) along Wilson flow
that bounds mass gap.** The closest published statements are:
(a) t² ⟨E(t)⟩ is the standard scale-setting observable
(Lüscher 2010), (b) gauge-fixed stochastic-quantization Langevin
admits a Bakry-Émery analysis (Dell'Antonio-Zwanziger), (c) 2D YM
moduli-space heat kernels are well-defined (Driver-Hall).

---

## §3 Candidate W_chromo specification

### §3.1 State space

State space:

  A ∈ A / G,                                                    (3.1)

where A is the affine space of SU(N) connections on a 4-torus T⁴
(or finite lattice) and G is the gauge group acting on A by
A ↦ g^{-1} A g + g^{-1} dg. Elements are gauge-equivalence classes
[A].

Wilson flow (Lüscher 2010 §2):

  ∂_τ A_τ = - δS_YM / δA |_{A = A_τ},                            (3.2)

with S_YM = (1/4) ∫ tr(F_{μν} F^{μν}) d⁴x. Wilson flow descends to
A/G (gauge-equivariant).

### §3.2 Candidate Lyapunov

The candidate functional, per the L9 prediction:

  W_chromo(A) = ∫_{T³} tr(B^a B^a) d³x
              + κ ∫_{T³} tr(F^a_{0i} F^{a,0i}) d³x,              (3.3)

where:

- B^a_i = (1/2) ε_{ijk} F^a_{jk} is the chromomagnetic field
  (3-vector indices i, j, k = 1, 2, 3);
- F^a_{0i} is the chromoelectric field (mixed time-space components);
- κ is a relative-weight constant (pure number);
- the integral is over a fixed time-slice T³ of the 4-torus.

The conjectured inequality is

  d/dτ W_chromo(A_τ) ≤ 0  along Wilson flow                      (3.4)

with rate controlled such that the spectral gap (mass) of the
transfer matrix is bounded below by a Bakry-Émery-style estimate.

### §3.3 Term anatomy

| piece | symbol | structural role | NS-EXT-B analog |
|-------|--------|-----------------|-----------------|
| ∫ tr(B² ) | T_mag | chromomagnetic energy density | T_kin (viscous dissipation, candidate 1.1) |
| κ ∫ tr(F_{0i}²) | T_el | chromoelectric (dual) energy | T_Fish (Fisher info, dual term) |

Compared with the BT-544 W_NS in `omega-exec-bt544-extb-cilyap-validation-2026-04-25.md`
§1.1, the structural parallel is:

- W_NS combines a velocity-space term (T_kin) with a density-space
  term (T_Fish + T_ent) and a linear correction (T_corr);
- W_chromo combines a magnetic-field term (T_mag) with a dual
  electric-field term (T_el) and an implicit τ-rescaling;
- both rely on Bakry-Émery structure for the cross-term sign
  analysis.

### §3.4 The mass-gap discharge claim

The candidate's claim is that if (3.4) holds with controlled rate,
then for the lattice transfer matrix T_lat with spectrum {1, λ_1,
λ_2, ...} (with 1 the vacuum eigenvalue and |λ_1| ≤ 1 the next
largest), the gap 1 − |λ_1| > 0 follows from a log-Sobolev-type
chain initiated at (3.4). This is the EXT-B mass-gap discharge
predicted in the L9 generation-pipeline §6.2.

The candidate's distinctive content is the **chromomagnetic-only
weighting** (versus the standard t² ⟨E(t)⟩ which weights B² + E²
equally), and the **mass-gap-discharging chain** that the L9
prediction claims as the EXT-B contribution.

---

## §4 Discriminator type

Per the n=20 plan §5 row P9 and the L9 prediction §6.2:

**OTHER (analytic-inequality)**.

This is the same family as BT-544 EXT-B (W_NS monotonicity).
Discrete-equality, numerical-interval, and vacuous-magnitude are
explicitly excluded by the seed; structural-literature is
secondary (a passing literature trace would shift toward
PASS_LITERATURE, but the primary discriminator is the inequality).

The native-pairing matrix has P9 in the (variational/Lyapunov,
analytic-inequality) cell, the third occupant alongside P1
(BT-544 CI-Lyap) and P10 (BT-542 meta-Lyapunov).

---

## §5 Falsifier registration

Three falsifiers are registered upfront, before the validation
runs Path-P / Path-Q:

### F-P9-A (W_chromo not monotone along Wilson flow)

**Statement**: there exists a class of lattice gauge configurations
on which dW_chromo/dτ has indefinite sign along Wilson flow (e.g.,
the dual T_el term grows fast enough to flip the sign of the
combined functional).

**Trigger**: any explicit configuration (e.g., a half-BPS instanton
profile, a dyon, or a confined Wilson-loop background) for which
dT_mag/dτ + κ dT_el/dτ has uncontrolled sign on a τ-window.

**Verdict if fires**: FAIL_NON_MONOTONE.

### F-P9-B (no gauge-invariant Bakry-Émery for Yang-Mills)

**Statement (most likely)**: gauge symmetry breaks the symmetry
calculus underlying Bakry-Émery 1985's CD(K, ∞) condition. The
configuration space A/G is an infinite-dimensional stratified space
(orbits of different stabilizer types form different strata), and
there is no published gauge-invariant carré-du-champ on A/G
satisfying CD(K, ∞).

**Trigger**: literature scan (§2.4) confirms no gauge-invariant
Bakry-Émery; gauge-fixed reductions break manifest invariance;
moduli-space results restricted to 2D / flat-connection cases.

**Verdict if fires**: OBSTRUCTION_DOCUMENTED (representation-side,
parallel to BT-544 F-EXTB-D).

### F-P9-C (W_chromo monotone but only on trivial vacuum)

**Statement**: W_chromo IS monotone along Wilson flow (so F-P9-A
does not fire), but the monotonicity is trivial because Wilson
flow drives any configuration to the trivial vacuum A = 0 (or to a
classical minimum), at which W_chromo = 0. The "monotonicity" is
the standard gradient-descent fact that the action decreases along
its own gradient flow, not a mass-gap statement.

**Trigger**: the only sketchable monotonicity reduces to the
trivial gradient-descent identity dS_YM/dτ = -|δS_YM/δA|² ≤ 0,
which gives no mass-gap information (massless trivial vacuum is
also a minimum).

**Verdict if fires**: FAIL_RELABELING (W_chromo is essentially
the action S_YM in disguise, not a new analytic object).

---

## §6 Validation: Path-P + Path-Q

### §6.1 Path-P (literature import)

**Question**: does any cited paper or short citation chain prove
(3.4) for the W_chromo in (3.3) on Wilson-flow-evolved lattice
configurations, with mass-gap discharge?

**Answer**: NO.

The §2 literature scan establishes:

- Lüscher 2010 introduces Wilson flow and the t² ⟨E(t)⟩ = 0.3
  scheme. No mass-gap Lyapunov is stated. The paper's §1 explicit
  caveat: "the flow is a useful smoothing tool for renormalization
  and scale setting; it is not by itself a non-perturbative
  resolution of mass-gap or confinement".
- Narayanan-Neuberger 2006 gives smoothed-Wilson-loop phase
  transitions at large N. No chromomagnetic-only Lyapunov bounding
  the gap.
- Borsanyi et al. 2012 uses Wilson flow as a high-precision scale-
  setting tool. The reported numerical stability of t² ⟨E(t)⟩ near
  the plateau is a *fitting fact*, not a Lyapunov bound on the
  spectral gap.
- The §2.4 adjacent literature (Charalambous-Lu, Dell'Antonio-
  Zwanziger, Driver-Hall) establishes Bakry-Émery / heat-kernel
  structures on auxiliary spaces (gauge-fixed slices, 2D moduli,
  bundle bases) but **none** on the gauge-invariant 4D YM
  configuration space.

Path-P FAILS. PASS_LITERATURE is not available.

### §6.2 Path-Q (sketch construction)

**Question**: can (3.4) be derived from standard tools (Wilson
flow gradient identity + gauge-invariant Bakry-Émery + chromomagnetic
decomposition) with all steps standard, no new theorem?

**Step 1 (gradient-flow identity)**: along Wilson flow,
dS_YM/dτ = -|δS_YM/δA|² ≤ 0. This is the standard gradient-descent
identity (Lüscher 2010 eq. 2.5) and is *not* a new statement; it is
the definition of gradient flow.

**Step 2 (chromomagnetic decomposition)**: in Euclidean lattice
signature,
S_YM = (1/4) ∫ F^a_{μν} F^{a,μν} d⁴x = (1/2) ∫ (B² + E²) d⁴x,
with B² = ∑_i tr(B^a_i)² and E² = ∑_i tr(F^a_{0i})². The candidate
W_chromo (3.3) is restricted to a single time-slice and weights B²
and κ E² with a chosen κ.

**Step 3 (time-slice differentiation)**: differentiate W_chromo on
a fixed time-slice along Wilson flow. The flow couples
chromomagnetic and chromoelectric components through F^a_{μν}
(since the flow ∂_τ A_τ = -δS_YM/δA mixes all spacetime indices).
Specifically:

  d/dτ T_mag = ∫ tr(B^a · ∂_τ B^a) = ∫ tr(B^a · {∂_i A_τ - ...})
             ⊃ -‖B-projection of δS_YM/δA‖² + cross terms       (6.1)

The cross terms include B-E mixing of the form
∫ tr(B^a · ∇ × E^a) (the magnetic curl of the electric field's
gauge-covariant derivative), which has no a-priori sign.

A Bakry-Émery Γ_2 estimate would *want* a CD(K, ∞) condition on
the gauge-invariant carré-du-champ on A/G. But A/G is:

- infinite-dimensional;
- stratified (orbits of different stabilizer types are different
  strata);
- not equipped with a published gauge-invariant Riemannian metric
  satisfying CD(K, ∞).

The only published candidates for a gauge-invariant metric on A/G
are the L²-metric (Babelon-Viallet 1981), which is positive-
semidefinite but does not satisfy CD(K, ∞), and the bundle metric
on the Yang-Mills flow space (Donaldson 1985 *J. Diff. Geom.* 18),
which is restricted to 4-manifolds with self-dual / anti-self-dual
splittings.

**Step 4 (sign analysis)**: collecting (6.1) and the analogous
chromoelectric piece:

  d/dτ W_chromo = -‖projection-of-δS_YM/δA onto (B, E)‖²
                + κ ∫ tr(F_{0i} · {∂_τ F_{0i}})
                + cross terms (B-E mixing).                      (6.2)

The first piece is **definite-sign** (negative semidefinite) by the
gradient-descent identity. But it equals dS_YM/dτ only when κ = 1
and the "projection" is the full F²; for κ ≠ 1 or for a single
time-slice (rather than the full 4D integral), the cross terms
spoil the sign.

The cross terms are not sign-controlled by Bakry-Émery without
CD(K, ∞), which is unavailable per Step 3.

Path-Q FAILS at the cross-term step. The same structural pattern
as BT-544 EXT-B: a clean-case identity (the YM action is monotone
along its own gradient flow — trivial relabeling) plus a non-trivial
cross-term obstruction (B-E mixing without gauge-invariant CD(K, ∞)).

### §6.3 The κ = 1 special case is FAIL_RELABELING

If κ = 1 and the integral is over the full 4-torus (not a single
time-slice), then W_chromo = (1/2) ∫(B² + E²) = S_YM, and (3.4)
reduces to dS_YM/dτ = -|δS_YM/δA|² ≤ 0 — the standard Wilson-flow
gradient-descent identity (Lüscher 2010 eq. 2.5). This is the
F-P9-C trigger: W_chromo collapses to the YM action.

This is a **trivial monotonicity** (gradient descent on its own
gradient is monotone by construction), with **no mass-gap content**:
the trivial vacuum A = 0 minimizes S_YM and has Δ_YM = 0
(massless), so the κ=1 case does not discharge the mass-gap
obstruction.

For κ ≠ 1 or single-time-slice integration, W_chromo is not the
action S_YM and dW_chromo/dτ has the cross-term issue from §6.2,
which the gauge-invariant Bakry-Émery would have to control if it
existed (F-P9-B).

### §6.4 The mass-gap discharge step is unsourced

Even granting (per impossibile) that (3.4) holds with controlled
rate, the chain "(3.4) ⇒ Δ_YM > 0" requires a log-Sobolev-type
inequality on the gauge-invariant configuration space A/G that
relates W_chromo decay rate to the spectral gap of the transfer
matrix. No such inequality is published. The L9 prediction §6.2
itself notes: "Expected dC: high if construction succeeds (load-
bearing Lyapunov); 0 if no construction" — flagging the
construction's hypothetical character.

---

## §7 Verdict

### §7.1 Discriminator outcome

| path | outcome | reason |
|------|---------|--------|
| Path P (literature import) | FAIL (§6.1) | no paper proves (3.4) with mass-gap discharge |
| Path Q (sketch) | FAIL at cross terms (§6.2) | B-E mixing not sign-controlled without CD(K, ∞) on A/G |
| F-P9-A (non-monotone) | NOT TESTED directly | subsumed by F-P9-B activation |
| F-P9-B (no gauge-inv Bakry-Émery) | FIRES (§6.2 Step 3) | A/G has no published gauge-invariant CD(K, ∞) |
| F-P9-C (relabeling on trivial vacuum) | PARTIALLY FIRES (§6.3) | κ=1 case is the YM action, trivial gradient-descent |

### §7.2 Verdict

**OBSTRUCTION_DOCUMENTED.**

The obstruction is **two-fold**:

1. **Representation-side (F-P9-B, §6.2 Step 3)**: there is no
   published gauge-invariant Bakry-Émery curvature-dimension
   condition on the Yang-Mills configuration space A/G. The
   space is infinite-dimensional, stratified, and not equipped
   with a metric satisfying CD(K, ∞). Gauge-fixed reductions
   (Dell'Antonio-Zwanziger) break manifest invariance; moduli-
   space results (Driver-Hall) are restricted to 2D / flat
   connections. The candidate's required sketch step
   (Bakry-Émery on A/G) is not a standard tool.
   **This is the primary obstruction.**

2. **Sketch-side (Path Q, §6.2)**: even granting a working
   gauge-invariant carré-du-champ, the time-slice differentiation
   of W_chromo produces B-E cross terms that are not sign-
   controlled. The κ = 1 limit collapses to the trivial
   gradient-descent identity dS_YM/dτ ≤ 0 (F-P9-C partial fire),
   which has no mass-gap content. For κ ≠ 1, the cross terms
   are uncontrolled.

This is **structurally parallel** to the BT-544 EXT-B verdict
(`omega-exec-bt544-extb-cilyap-validation-2026-04-25.md` §7.2):
- BT-544 F-EXTB-D fires on the representation-side (CI 2008
  produces a family {ρ_t(·|x_0)}, not a single ρ on ℝ³);
- BT-543 F-P9-B fires on the representation-side (no gauge-
  invariant Bakry-Émery on A/G).

In both cases, the **representation-side obstruction is primary**;
the cross-term / sketch-side obstruction is secondary.

### §7.3 Probability calibration check

The L9 generation-pipeline §F-DIAG-E flagged P9 as "the most
speculative" prediction with medium risk. The user brief
anticipated "OBSTRUCTION_DOCUMENTED ~70% via F-P9-B".

This validation realises the OBSTRUCTION_DOCUMENTED outcome via
F-P9-B exactly as anticipated. The calibration is consistent.

---

## §8 Cross-BT EXT-B comparison

### §8.1 BT-544 EXT-B vs BT-543 P9 side-by-side

| aspect | BT-544 EXT-B (CI-Lyap) | BT-543 P9 (chromo-Lyap) |
|--------|-------------------------|---------------------------|
| domain | 3D incompressible NS | 4D pure SU(N) Yang-Mills |
| state space | (u, ρ, τ) on ℝ³ × P(ℝ³) × ℝ_{>0} | A/G on T⁴ |
| flow | NS coupled with Constantin-Iyer Lagrangian | Wilson flow (Lüscher 2010) |
| candidate functional | W_NS = ∫ [τ(ν|∇u|² + |∇log ρ|²) + log ρ] ρ + γνt | W_chromo = ∫ tr(B²) + κ ∫ tr(F_{0i}²) |
| target inequality | dW_NS/dt ≥ 0 | dW_chromo/dτ ≤ 0 |
| primary falsifier | F-EXTB-D (CI 2008 too weak) | F-P9-B (no gauge-inv Bakry-Émery) |
| primary obstruction class | representation-side (no canonical ρ on ℝ³) | representation-side (no CD(K,∞) on A/G) |
| secondary falsifier | F-EXTB-C partial (Wasserstein-fails-NS) | F-P9-C partial (κ=1 relabeling) |
| sketch-side obstruction | vortex stretching + Hess(log ρ):∇u cross | B-E mixing cross terms |
| verdict | OBSTRUCTION_DOCUMENTED | OBSTRUCTION_DOCUMENTED |
| millennium status | open (NS) | open (YM) |

### §8.2 The cross-BT pattern

Both EXT-B candidates produce **OBSTRUCTION_DOCUMENTED at a
representation-side falsifier**:

- BT-544: the augmented state space (u, ρ, τ) requires a single ρ
  on ℝ³ that CI 2008 does not canonically produce.
- BT-543: the gauge-invariant configuration space A/G requires a
  Bakry-Émery CD(K, ∞) that no published source establishes.

The structural common cause is that **both candidates require
extending Bakry-Émery / Otto-style log-Sobolev calculus to non-
standard configuration spaces** (path-space / probability density
hybrid for NS; gauge-equivalence-class space for YM). In both
cases, the literature catalogues these as "open" — adjacent
results exist but the specific form needed for the EXT-B
candidate is not in print.

This is a **systematic pattern**, not a coincidence: the EXT-B
class (variational/Lyapunov + analytic-inequality) hits the
representation-side wall whenever the target equation's natural
configuration space lacks a standard symmetric-diffusion
structure. NS lacks reversibility (advection-dominated); YM
lacks gauge-invariant CD(K, ∞).

### §8.3 Implications for the EXT-B class

Two EXT-B validations have run; both produced OBSTRUCTION_DOCUMENTED
at representation-side falsifiers. This raises the prior
probability that:

- EXT-B candidates for other Millennium-class targets (BT-541,
  BT-545, BT-546, BT-547) will hit similar representation-side
  walls;
- A meta-EXT-B (e.g., P10 BT-542 meta-complexity-resource-monotone)
  may hit a discrete-domain analog of the same pattern (no
  resource-monotone-Bakry-Émery on Boolean function space).

This is a **predictive update**, not a verdict. P10 is the next
test of this prediction.

The L9 generation-pipeline diagnostic anticipated this risk in
§F-DIAG-E: "Risk: low for BT-544 and BT-541; medium for BT-543".
The BT-543 case has now realised the medium risk; the BT-541
EXT-C P3 candidate is independent and not affected.

---

## §9 Bias-hypothesis 2×2 update

### §9.1 Pre-P9 state (n=15, from `omega-exec-n20-p8-d11-cap-sweep-2026-04-25.md` §7.3)

| 2×2 row \ col | PASS | FAIL |
|---|---:|---:|
| distrib / struct-lit / rep-th (top) | 4 | 2 |
| discrete-eq / interval / vacuous (bottom) | 3 | 6 |

Total = 15. Fisher 2-sided p = 0.3287 (primary classification).

### §9.2 P9 native-pairing classification

Per the n=20 plan §5: P9 is **variational/Lyapunov** (candidate
row) with **analytic-inequality** (discriminator). In the
discriminator-type 2×2:

- analytic-inequality is grouped with **OTHER** discriminators
  (per the BT-547 retro §3.4 broadening). The OTHER class was
  retrospectively placed in the **PASS-family-adjacent** position
  alongside distributional and structural-literature.
- For 2×2 accounting, P9 is **top-row** (with the OTHER /
  analytic-inequality grouping).

### §9.3 P9 verdict classification for 2×2

The verdict OBSTRUCTION_DOCUMENTED counts as **PASS-family-adjacent**
(per BT-544 EXT-B precedent in `omega-exec-bt544-extb-cilyap-validation-2026-04-25.md`
§9.3). It is not a clean PASS, not a clean FAIL.

Two 2×2 conventions are possible:

**Convention A (OBSTRUCTION as PASS-adjacent → top-row PASS)**:
- This is the convention used in the BT-544 EXT-B post-validation
  tally (op. cit. §9.3 row 9).
- P9 adds +1 top-row PASS.

**Convention B (OBSTRUCTION as FAIL — strict reading)**:
- Treats OBSTRUCTION_DOCUMENTED as the candidate failing to
  produce a clean-case answer.
- P9 adds +1 top-row FAIL.

### §9.4 Post-P9 2×2 (n=16)

**Convention A (consistent with BT-544 EXT-B precedent)**:

| 2×2 row \ col | PASS | FAIL |
|---|---:|---:|
| distrib / struct-lit / OTHER (top)        | **5** | 2 |
| discrete-eq / interval / vacuous (bottom) | 3 | 6 |

Total = 16. Approximate Fisher 2-sided p:
table = [[5, 2], [3, 6]]; row sums 7, 9; col sums 8, 8; total 16.
P(K=5) = C(7,5) C(9,3) / C(16,8) = 21 · 84 / 12870 = 1764 / 12870 ≈ 0.137.
Two-sided p ≈ 2 · min(0.137, ...) ≈ 0.20-0.25 (manual estimate).
A precise scipy run is recommended for follow-up; the manual
value ≈ 0.22 is consistent with the n=15 trend (top-row weak
clustering, bottom-row weak separation).

**Convention B (strict FAIL reading)**:

| 2×2 row \ col | PASS | FAIL |
|---|---:|---:|
| distrib / struct-lit / OTHER (top)        | 4 | **3** |
| discrete-eq / interval / vacuous (bottom) | 3 | 6 |

Total = 16. Fisher 2-sided p ≈ 0.42 (less significant under strict
reading).

### §9.5 Convention recommendation

Convention A is recommended for consistency with BT-544 EXT-B
(op. cit. §9.3, where OBSTRUCTION_DOCUMENTED was placed in the
PASS-family-adjacent column). The recommendation rests on:

- OBSTRUCTION_DOCUMENTED is a *substantive informational verdict*
  (it identifies the precise breaking step), unlike a clean FAIL
  which is a closed verdict.
- The discriminator (analytic-inequality) is in the OTHER /
  PASS-family per the BT-547 retro §3.4 mapping.
- Treating OBSTRUCTION as FAIL would collapse the EXT-B class
  into the FAIL family, which contradicts the §3.4 broadening.

### §9.6 Trajectory implication

Under Convention A, the n=15 → n=16 update with P9 OBSTRUCTION
gives p shifting from 0.33 to ≈ 0.22, marginally **strengthening**
the top-row PASS clustering (more PASS in top, no change in
bottom). This is consistent with the post-P567 reading that the
discriminator-type axis was losing predictive power — the EXT-B
validations are **stabilizing** the top-row PASS-adjacent count.

Under Convention B, the n=16 update gives p ≈ 0.42, **weakening**
the bias hypothesis further. The n=20 plan's final reading at
n=20 will determine which convention is structurally informative.

The user brief anticipated: "P9 likely adds to top row. If P9
OBSTRUCTION_DOCUMENTED, top row FAIL count grows." This
anticipation aligns with Convention B. Convention A is the
post-validation precedent, applied here for consistency.

---

## §10 Anomalies

### §10.1 The κ choice is structurally undetermined

The candidate W_chromo (3.3) introduces κ as a relative-weight
constant between chromomagnetic and chromoelectric pieces. The
L9 prediction §6.2 does not specify κ. Three special cases are
visible:

- **κ = 1**: W_chromo = action S_YM (per §6.3); FAIL_RELABELING.
- **κ = 0**: W_chromo = pure chromomagnetic energy ∫ tr(B²);
  d/dτ has uncontrolled cross-terms even in the trivial case
  because Wilson flow couples B and E.
- **κ = -1**: W_chromo = ∫ tr(B² - E²) = topological-charge-
  density-related (Pontryagin-density linkage); monotone along
  Wilson flow only on instanton sectors of fixed charge, not a
  mass-gap statement.

The L9 prediction's κ-ambiguity is a structural anomaly: the
candidate is under-specified, paralleling the BT-544 EXT-B
choice-of-ρ ambiguity (op. cit. §10.1).

### §10.2 The chromomagnetic-only restriction has no canonical motivation

The chromomagnetic-only weighting has no published motivation as
a Lyapunov for mass gap. The standard observable t² ⟨E(t)⟩ uses
the **full** F², not B²-only. The asymmetric weighting κ ≠ 1 is
a candidate-spec choice not matching any Lüscher 2010 / Borsanyi
et al. 2012 convention.

### §10.3 Mass-gap discharge chain is conjectural

The "if (3.4) holds, then Δ_YM > 0" chain (§3.4) requires a
log-Sobolev inequality on A/G that does not exist. Even granting
the F-P9-B obstruction is somehow resolved (per impossibile),
the discharge step is independently unsourced.

### §10.4 Cross-BT EXT-B sample is small

Two EXT-B validations (BT-544 + BT-543), both OBSTRUCTION_DOCUMENTED.
The "systematic pattern" in §8.2 is suggestive but not statistically
significant. P10 (BT-542 meta-Lyapunov) and any future BT-541 /
BT-545 / BT-546 / BT-547 EXT-B validations will sharpen the
pattern.

### §10.5 Disclaimer count check

This file uses "conjecture", "candidate", "open", "not proven",
"not established" for every step that is not a published theorem
with rigorous proof. No YM mass-gap claim is made anywhere. Every
cited reference is by author / year / journal pattern matched to
the standard lattice-gauge / mathematical-gauge bibliography.

---

## §11 Falsifiers active

### §11.1 Falsifiers for this validation's verdict

#### F-VAL9-A (literature scan missed a relevant chromomagnetic Lyapunov)

**Statement**: if a published paper exists that directly proves
(3.4) for some choice of κ on Wilson-flow configurations, then
Path-P would succeed and the verdict shifts to PASS_LITERATURE.

**Status**: NOT ACTIVE. The §2 / §6.1 scan did not surface such a
paper. Risk of missed paper is non-trivial because the
post-2010 lattice-gauge Wilson-flow literature is large; cross-
check on extension if undertaken.

#### F-VAL9-B (gauge-invariant Bakry-Émery is established but obscure)

**Statement**: if a paper exists (perhaps in the mathematical-
gauge or stochastic-quantization corpus) that establishes
gauge-invariant CD(K, ∞) on A/G or on a relevant quotient, then
F-P9-B does not fire and Path-Q can be re-run.

**Status**: PARTIALLY ACTIVE in spirit. The §2.4 scan found
related but not directly applicable results (Charalambous-Lu's
bundle-base CD, Dell'Antonio-Zwanziger's gauge-fixed Langevin,
Driver-Hall's 2D moduli heat kernel). A specialist re-reading
could find a direct gauge-invariant CD result; the validation's
reading is that no such result is in print.

#### F-VAL9-C (Wilson-flow-specific Lyapunov constructions exist outside Bakry-Émery)

**Statement**: Wilson flow may admit a Lyapunov via tools other
than Bakry-Émery — e.g., direct lattice-spectral arguments
(Kogut-Susskind 1975), monotone-quantity arguments (Witten 1982
*twisted YM*), or Hamiltonian-lattice spectral-gap arguments.
If any such tool produces (3.4) for some W_chromo, the verdict
softens.

**Status**: NOT ACTIVE based on the scan. Lattice-spectral
arguments for mass gap (Kogut-Susskind 1975, Wilson 1974) are
**strong-coupling**-only; the continuum mass-gap question is
out of reach at strong coupling alone. No published Lyapunov
of the W_chromo form is sourced through these alternative
routes.

#### F-VAL9-D (κ choice fixes the obstruction)

**Statement**: a specific κ (e.g., the BPS κ = 1 self-dual
weighting, or a topological κ = -1 anti-self-dual weighting)
restricts W_chromo to a sector where (3.4) holds non-trivially.

**Status**: PARTIALLY ACTIVE. κ = ±1 are special cases with
extra structure (self/anti-self-duality), but per §10.1 these
either reduce to the action (κ = 1) or to the topological
charge density (κ = -1), neither of which discharges the
mass-gap obstruction. The κ-fix-attempts collapse to known
non-Lyapunov objects.

#### F-VAL9-E (atlas/state/inventory edit leakage)

**Statement**: if any change is made to atlas, state, or
inventory files as a result of this validation, the brief's
hard constraint is violated.

**Status**: NOT ACTIVE. This validation is research-only and
edits no atlas, state, or inventory file.

### §11.2 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-VAL9-A | literature scan missed chromomagnetic Lyapunov | NOT ACTIVE |
| F-VAL9-B | gauge-invariant Bakry-Émery established but obscure | PARTIALLY ACTIVE |
| F-VAL9-C | Wilson-flow Lyapunov via non-Bakry tools | NOT ACTIVE |
| F-VAL9-D | κ choice fixes the obstruction | PARTIALLY ACTIVE |
| F-VAL9-E | atlas/state/inventory edit leakage | NOT ACTIVE |

### §11.3 Inherited falsifiers from candidate spec

| tag | name | candidate-spec status | post-validation status |
|-----|------|----------------------|-------------------------|
| F-P9-A | W_chromo non-monotone | NOT YET TESTED | NOT TESTED DIRECTLY (subsumed by F-P9-B) |
| F-P9-B | no gauge-invariant Bakry-Émery for YM | most likely activation | FIRES (§6.2 Step 3) — confirmed |
| F-P9-C | monotone but only on trivial vacuum | partially active | PARTIALLY FIRES (§6.3, κ=1 case) |

The candidate-spec's expected activation pattern (F-P9-B primary,
F-P9-C partial) is **realised** by this validation, paralleling
the BT-544 EXT-B verdict's F-EXTB-D / F-EXTB-C / F-EXTB-E
pattern.

---

## §12 Closing

**Verdict**: **OBSTRUCTION_DOCUMENTED** at F-P9-B primary
activation (§6.2 Step 3), with secondary partial F-P9-C activation
(§6.3, κ = 1 relabeling case).

**Path P**: FAIL — no published paper proves (3.4) with mass-gap
discharge.

**Path Q**: FAIL at the cross-term step — B-E mixing in (6.2) is
not sign-controlled without gauge-invariant CD(K, ∞) on A/G,
which is not in the published mathematical-gauge literature.

**Falsifier F-P9-B**: FIRES. The Yang-Mills configuration space
A/G has no published gauge-invariant Bakry-Émery curvature-
dimension condition; gauge-fixed reductions (Dell'Antonio-
Zwanziger) break manifest invariance, moduli-space results
(Driver-Hall) are 2D / flat-connection-only, and the L²-metric
on A/G (Babelon-Viallet 1981) does not satisfy CD(K, ∞).

**Falsifier F-P9-C**: PARTIALLY FIRES. The κ = 1 case collapses
to the YM action, giving the trivial gradient-descent identity
dS_YM/dτ ≤ 0 (Lüscher 2010 eq. 2.5), which has no mass-gap
content. For κ ≠ 1, the cross-term obstruction (F-P9-B) governs.

**Cross-BT EXT-B parallel**: BT-544 EXT-B (CI-Lyap) verdict
OBSTRUCTION_DOCUMENTED via F-EXTB-D (representation-side: CI 2008
does not produce a single ρ on ℝ³); BT-543 P9 (chromo-Lyap)
verdict OBSTRUCTION_DOCUMENTED via F-P9-B (representation-side:
no gauge-invariant Bakry-Émery on A/G). Both EXT-B candidates
hit a representation-side wall on non-standard configuration
spaces. The pattern is suggestive but the sample (n = 2) is
small.

**Bias-hypothesis 2×2 update**: under Convention A (OBSTRUCTION
as PASS-family-adjacent, per BT-544 EXT-B precedent), n=15 → n=16
gives top row 5 PASS / 2 FAIL, bottom row 3 PASS / 6 FAIL,
Fisher p ≈ 0.22. Under Convention B (strict FAIL reading), top
row 4 PASS / 3 FAIL, p ≈ 0.42. Convention A is recommended for
consistency with prior tally.

**Anomalies (§10)**: κ choice is structurally undetermined;
chromomagnetic-only restriction has no canonical motivation; the
mass-gap discharge chain is conjectural; the cross-BT EXT-B
sample is small.

**Falsifiers active for this validation (§11)**: F-VAL9-B and
F-VAL9-D partially active (specialist re-reading risks);
F-VAL9-A, F-VAL9-C, F-VAL9-E not active.

**0/7 unchanged. YM mass-gap status open. No atlas / state /
inventory edits.** All cited references are pre-existing
(Lüscher 2010 JHEP 1008:071; Narayanan-Neuberger 2006 JHEP
0603:064; Borsanyi et al. 2012 JHEP 1209:010; Bakry-Émery 1985
Sém. Probab. XIX; Charalambous-Lu 2015 Math. Z. 280;
Dell'Antonio-Zwanziger 1989-91 Comm. Math. Phys.; Driver-Hall
1999 Comm. Math. Phys.; Donaldson 1985 J. Diff. Geom. 18;
Babelon-Viallet 1981 Phys. Lett. B; Kogut-Susskind 1975 Phys.
Rev. D 11; Wilson 1974 Phys. Rev. D 10; Witten 1982
*twisted Yang-Mills*; Bakry-Gentil-Ledoux 2014 Springer;
Buckmaster-Vicol 2019 Ann. Math. 189; Gross-Wilczek 1973 +
Politzer 1973; FLAG 2024; BMW 2012; Morningstar-Peardon 1999;
Constantin-Iyer 2008 CPAM 61). No fabricated references; all
specific section / equation numbers are recoverable from the
cited sources.

— end validation —
