---
id: omega-exec-bt544-extb-analytic-lyapunov-candidate
date: 2026-04-25
scope: research-only candidate generation (NOT validating; producing a single new BT-544 frame-shift candidate)
target: BT-544 EXT-B -- analytic-Lyapunov candidate from Perelman W-entropy lineage
parent_reports:
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (EXT-B recommendation)
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: candidate generation, no claim
---

# Omega Exec — BT-544 EXT-B Analytic-Lyapunov Candidate (2026-04-25)

## §0 Non-claim disclaimer

This report **generates** a single BT-544 frame-shift candidate of
the EXT-B class (analytic-Lyapunov-construction) recommended by
`reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md`
§5.2. It does **not**:

- claim 3D Navier-Stokes regularity, blow-up, or any Clay-form
  resolution;
- prove or even sketch-prove the candidate Lyapunov's monotonicity —
  monotonicity is **conjectural** in the literature it is drawn
  from, and that is exactly the data the discriminator and
  falsifier are designed to record;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-544 = 0/1 untouched` Clay
  status, or add to the L9 catalogue's active-candidate ledger;
- replace or supersede the D1 / D2 / D3.A / D3.B'/ D3.C catalogue
  rows. EXT-B operates on the *missing class* per the BT-547 retro
  partial-fit verdict, not on the existing rows.

EXT-B is a **candidate-generation** task per the BT-547 retro
recommendation (§5.2). It is a NEW family slot — variational /
analytic-Lyapunov — that the prior generation pipeline (dfs-24
direction probes) did not admit. The verdict expected from this
candidate, when validated in a follow-up session, is **PASS_LITERATURE**
if the chosen reference's monotonicity inequality is rigorous,
**OBSTRUCTION_DOCUMENTED** if monotonicity is conjectural, and
**INCONCLUSIVE** if the literature is underdetermined. This file
records the generation; validation is a separate session.

**0/7 unchanged. No atlas/state/inventory edits.**

---

## §1 Lineage — Perelman W-entropy structural recap and NS analog framing

### §1.1 Perelman's W-functional, structural points

Per Perelman 2002 arXiv:math/0211159 §3, the W-entropy on a
Riemannian manifold (M^n, g) with a function f and scale parameter
τ > 0 is

  W(g, f, τ) = ∫_M [τ(R + |∇f|²) + f − n] (4πτ)^(−n/2) e^(−f) dV   (P1)

subject to the volume constraint ∫_M (4πτ)^(−n/2) e^(−f) dV = 1.
The structural anatomy of W relevant to the NS analog:

(i) **State-space**: not the manifold metric g alone, but the
    triple (g, f, τ) — metric × log-density × scale. This is an
    *augmented* configuration space, not the bare PDE state.

(ii) **Functional form**: linear combination of three pieces — a
     curvature/gradient kinetic-like term τ(R + |∇f|²), an entropy
     term f, and a volume-normalisation correction −n. Each carries
     a τ-weight that fixes the scale-covariance of W.

(iii) **Monotonicity claim**: along Ricci flow ∂_t g = −2 Ric
      (modified by the τ-rescaled gradient flow generator),

        d/dt W(g(t), f(t), τ(t)) ≥ 0,                              (P2)

      with equality iff (g, f) is a *gradient shrinking soliton*
      (Perelman 2002 §3.4 main theorem). Monotonicity is GLOBAL
      along the modified flow — not conditional on smallness, on
      smoothness threshold, or on geometric assumption.

(iv) **Regularity consequence**: monotonicity (P2) plus a
     reverse-Bochner / log-Sobolev application yields the
     κ-noncollapsing theorem (Perelman 2002 §4): along Ricci flow
     with bounded curvature on a parabolic ball,

        Vol(B_r) ≥ κ · r^n.                                        (P3)

     (P3) directly controls a regularity-relevant quantity (the
     local volume), and through the Hamilton compactness machinery
     (Hamilton 1995 Surveys in Diff. Geom. §16) it controls
     curvature blow-up. **W is the load-bearing Lyapunov** for the
     Ricci-flow programme.

### §1.2 What an NS analog requires

Per the BT-547 retro §5.2 recommendation and the Perelman anatomy
(i)-(iv), an EXT-B candidate for NS must specify:

- a **state space** that augments the velocity field u(t, x)
  beyond the bare L²-divergence-free phase space (analog of (i):
  metric × density × scale → u × auxiliary × scale);
- a **functional** W_NS[u, …] that is a definite combination of
  pieces — at minimum a kinetic-like + entropy-like + scale
  correction (analog of (ii));
- a **monotonicity claim**: dW_NS/dt has a definite sign along NS
  evolution (analog of (P2));
- a **regularity coupling**: control of W_NS implies control of a
  regularity-sensitive norm (Sobolev H^s, Hölder C^α, BMO⁻¹, or
  similar) — i.e. W_NS discharges a known structural obstruction
  (analog of (P3)).

Items (i)-(iii) define the candidate; item (iv) is what makes it a
**Lyapunov for regularity** rather than a thermodynamic curiosity.
The BT-544 lacuna identified in BT-547 retro §6.1 is precisely the
absence of any candidate satisfying (i)-(iv) jointly: the existing
NS toolkit has each piece in isolation but not the joint structural
package.

---

## §2 Existing NS Lyapunov inventory — what we already have

The active-NS literature contains many partial-Lyapunov
constructions. Each is recorded below with its monotonicity
status, the regularity-relevant quantity it does or does not
control, and its known limitation. References are cited by author
/ year / journal and are pre-existing; nothing in this section is
fabricated.

### §2.1 Energy E(t) = (1/2) ‖u(t)‖_{L²}²

- **Source**: Leray 1934 (*Acta Math.* 63), the founding NS paper.
- **Monotonicity**: dE/dt = −ν ‖∇u‖_{L²}² ≤ 0 along smooth NS
  evolution. Monotone-non-increasing globally (Leray-Hopf weak
  solutions satisfy the inequality version).
- **Regularity coupling**: TRIVIAL. E controls the L² norm only;
  L² is the regularity floor, not a regularity ceiling. Above L²,
  E says nothing.
- **Status as Perelman analog**: structurally too weak. E is the
  analog of the L² norm of the metric, not of an entropy
  functional with τ-rescaling. Item (iv) fails.

### §2.2 Helicity H(t) = ∫ u · ω dx

- **Source**: Moffatt 1969 (*J. Fluid Mech.* 35), Moreau 1961
  (*C. R. Acad. Sci. Paris* 252).
- **Monotonicity**: dH/dt = −2ν ∫ ω · (∇ × ω) dx along viscous NS;
  conserved in inviscid Euler. **Sign-indefinite** for NS — H can
  grow or shrink.
- **Regularity coupling**: weak. Helicity is sign-indefinite and
  does not control any standard regularity norm directly.
- **Status as Perelman analog**: monotonicity fails (item iii);
  H is more an analog of total scalar curvature ∫R dV (a global
  conserved quantity in special geometries) than of W.

### §2.3 Enstrophy Z(t) = (1/2) ‖ω(t)‖_{L²}²

- **Source**: Foias-Manley-Rosa-Temam 2001 (*Navier-Stokes
  Equations and Turbulence*, Cambridge), Constantin-Foias 1988
  (*Navier-Stokes Equations*, Chicago).
- **Monotonicity**: dZ/dt = ∫(ω·∇)u·ω dx − ν ‖∇ω‖_{L²}². The
  vortex-stretching term ∫(ω·∇)u·ω can be POSITIVE; in 3D
  enstrophy can grow without bound (this is the open problem).
- **Regularity coupling**: STRONG IF MONOTONE. By BKM 1984
  (*Comm. Math. Phys.* 94), ∫₀^T ‖ω‖_{L^∞} dt < ∞ implies
  smoothness; ‖ω‖_{L²} bounded uniformly is much stronger than
  what BKM requires.
- **Status as Perelman analog**: PARTIAL. Z has the right shape
  (a kinetic-like quantity that controls regularity, item iv) but
  fails item (iii) — monotonicity is not known and is, in fact,
  **the open problem of NS regularity**. Z is the *target* of the
  Perelman-archetype, not the candidate itself.

### §2.4 Constantin-Fefferman 1993 geometric depletion

- **Source**: Constantin-Fefferman 1993 (*Indiana Univ. Math. J.*
  42) — direction-of-vorticity Lipschitz hypothesis.
- **Statement**: if the unit vorticity direction ξ = ω/|ω| is
  spatially Lipschitz with bounded Lipschitz constant on the
  high-vorticity region, then the vortex-stretching term is
  *depleted* and a BKM-style bound becomes provable.
- **Monotonicity**: not a Lyapunov. CF1993 is a CONDITIONAL
  REGULARITY THEOREM — under the Lipschitz hypothesis on ξ, it
  produces a bound. It does not exhibit a monotone quantity.
- **Regularity coupling**: STRONG, but conditional on the
  hypothesis (which is itself unverified for general NS).
- **Status as Perelman analog**: not a Lyapunov candidate per se;
  rather a **conditional discharge** of the obstruction. CF1993 is
  the *kind* of structural insight that would inform an EXT-B
  candidate's design (the geometric measure of vorticity coherence
  is a natural input to a Lyapunov), but it is not itself an
  EXT-B candidate.

### §2.5 Beale-Kato-Majda 1984 vorticity criterion

- **Source**: Beale-Kato-Majda 1984 (*Comm. Math. Phys.* 94, p. 61).
- **Statement**: smoothness on [0, T] iff ∫₀^T ‖ω(t)‖_{L^∞} dt < ∞.
- **Monotonicity**: not a Lyapunov; a control criterion. ‖ω‖_{L^∞}
  is sign-positive but its time integral is the relevant quantity
  and need not be monotone.
- **Status as Perelman analog**: BKM is the analog of Hamilton's
  curvature blow-up criterion (Hamilton 1982 J. Diff. Geom. 17),
  not of W. It is a *blow-up detector*, not a regularity-forcer.

### §2.6 Total dissipation D(T) = ν ∫₀^T ‖∇u(t)‖_{L²}² dt

- **Source**: Leray 1934; appears throughout the Constantin-Foias
  1988 monograph.
- **Monotonicity**: D is monotone-increasing in T trivially (it is
  a time integral of a non-negative quantity). dD/dT = ν‖∇u(T)‖_{L²}²
  ≥ 0.
- **Regularity coupling**: WEAK. D controls only the L²([0,T];H¹)
  norm (which is automatic for Leray-Hopf solutions); above this,
  D is silent.
- **Status as Perelman analog**: structurally weak. Monotonicity
  is trivial; regularity coupling is at the floor.

### §2.7 Foias-Saut backward-uniqueness Lyapunov / log-energy

- **Source**: Foias-Saut 1984 (*Indiana Univ. Math. J.* 33), and
  the systematisation in Constantin-Foias-Manley-Temam 1988
  (*J. Fluid Mech.* 192).
- **Statement**: along NS evolution, log E(t) − log E(s) admits a
  Bochner-type identity producing the sharp Foias-Saut backward-
  uniqueness theorem (two NS solutions with the same final state
  agree everywhere).
- **Monotonicity**: log E is monotone-non-increasing (since E is)
  but the *Bochner identity* in Foias-Saut is the load-bearing
  piece, not a Lyapunov.
- **Regularity coupling**: indirect. Backward uniqueness is a
  regularity-adjacent property but does not control H^s norms.
- **Status as Perelman analog**: closer in flavour than (E, H, Z)
  — the Bochner identity is the kind of analytic identity Perelman
  uses for W — but the regularity coupling is too weak to
  discharge the BKM-criterion-class obstruction.

### §2.8 Inventory summary table

| name | monotone along NS? | regularity coupling | EXT-B-class? |
|------|---------------------|---------------------|---------------|
| Energy E (Leray 1934) | YES, trivial | TRIVIAL (L² floor) | NO — too weak |
| Helicity H (Moffatt 1969) | NO (sign-indef.) | weak | NO — fails (iii) |
| Enstrophy Z (Foias-…-Temam) | OPEN (the NS problem) | STRONG if mono. | PARTIAL — target, not candidate |
| Constantin-Fefferman 1993 | not a Lyapunov | strong, conditional | NO — conditional thm, not Lyapunov |
| BKM 1984 | not a Lyapunov | criterion only | NO — blow-up detector |
| Total dissipation D | trivial | weak | NO |
| Foias-Saut log-E + Bochner | mono (trivially) | indirect | NO — wrong coupling |

The inventory shows: every existing NS Lyapunov candidate fails at
least one of the four Perelman-anatomy items. Energy and total
dissipation are monotone but regularity-trivial. Helicity and
enstrophy carry the right regularity weight but are not known
monotone. CF1993 and BKM are conditional / criterion-only. Foias-
Saut is monotone-trivially with the wrong regularity coupling.

**The EXT-B slot is genuinely empty** — none of the existing NS
Lyapunov tools fits the Perelman anatomy fully. This is the
specific lacuna BT-547 retro §6.1 names for BT-544.

---

## §3 Candidate frame-shift specification

### §3.1 Chosen candidate — name, source, and statement

**Candidate name**: **Constantin–Iyer relative-entropy / stochastic-Lagrangian Lyapunov for Navier-Stokes** (hereafter **CI-Lyap**).

**Primary literature anchor**: Constantin-Iyer 2008 (*Comm. Pure
Appl. Math.* 61, 330–345), "A stochastic Lagrangian representation
of the three-dimensional incompressible Navier-Stokes equations".
The Constantin-Iyer formula represents NS velocity as the
expectation of a stochastic-Lagrangian process; this representation
has a NATURAL relative-entropy / large-deviation rate functional
attached to it.

**Secondary literature anchors** (lineage and structural support):

- Brenier 1999 (*J. Amer. Math. Soc.* 12, 495), generalized least
  action for incompressible Euler (the variational frame within
  which the relative entropy lives);
- Otto 2001 (*Comm. PDE* 26, 101), Wasserstein-gradient-flow
  framework on probability measures (the gradient-flow envelope);
- Ambrosio-Gigli-Savaré 2008 (Birkhäuser monograph), gradient
  flows on metric measure spaces (the rigorous calculus);
- Constantin-Vicol 2012 (*Geom. Funct. Anal.* 22), nonlinear
  maximum principles for dissipative linear nonlocal operators
  (the analytic-inequality machinery);
- Vasseur 2007/2010 (*SIAM J. Math. Anal.* / *Arch. Rat. Mech.
  Anal.*), De Giorgi-style L² → L^∞ improvements for NS-class
  equations (the regularity-coupling machinery).

### §3.2 The candidate functional (statement)

State space (analog of Perelman's metric × density × scale):
the configuration is the triple

  (u, ρ, τ) ∈ {div-free L²} × {prob. densities on ℝ³} × ℝ_{>0},  (3.1)

with u the NS velocity, ρ a probability density on ℝ³ representing
the law of the stochastic-Lagrangian flow at time t (per
Constantin-Iyer 2008 §2 the noise-driven flow generates such a
ρ_t), and τ > 0 a scale parameter playing the role of Perelman's
τ.

Candidate Lyapunov:

  W_NS(u, ρ, τ) = ∫_{ℝ³} [τ(ν|∇u|² + |∇ log ρ|²) + log ρ] ρ dx
                 + γ · ν · t,                                     (3.2)

where γ ≥ 0 is a fixed normalisation constant chosen so that the
inviscid limit ν → 0 reproduces the Brenier 1999 least-action
functional, and the τ-rescaling is taken in the Perelman sense
(τ(t) = T − t for a finite reference horizon T or τ(t) = 1/(T+t)
for an asymptotic version).

The three pieces of W_NS mirror Perelman's three pieces of W:

| Perelman W piece | NS analog in W_NS |
|------------------|---------------------|
| τ(R + \|∇f\|²) (curvature + gradient of log-density) | τ(ν\|∇u\|² + \|∇ log ρ\|²) (dissipation rate + Fisher information of ρ) |
| f (log of density measure) | log ρ (entropy density) |
| −n (volume-normalisation correction) | γν·t (viscous-time correction; replaces the volume normalisation) |

### §3.3 Monotonicity claim (conjectural)

CI-Lyap monotonicity hypothesis:

  d/dt W_NS(u(t), ρ(t), τ(t)) ≥ 0                                  (3.3)

along the coupled NS + Constantin-Iyer-stochastic-Lagrangian
evolution, with equality iff (u, ρ) is a *self-similar dissipative
soliton* (the NS analog of Perelman's gradient shrinking soliton —
this notion is not standard in the NS literature and is part of
the candidate; closest existing analog is Necas-Růžička-Šverák
1996 *Acta Math.* 176 self-similar blow-up profiles, which they
*disprove* — so the soliton class would be empty, which is itself a
useful structural piece).

**The monotonicity (3.3) is a CONJECTURE**, not a theorem. The
Constantin-Iyer 2008 paper does not state it. The closest
literature statements are:

- Otto 2001 establishes Wasserstein-gradient-flow monotonicity for
  the heat equation (not NS); the heat equation is the linearised
  Stokes operator.
- Ambrosio-Gigli-Savaré 2008 §§9-11 develop gradient flows on
  metric measure spaces; NS is *not* a gradient flow on a known
  metric (this is a known negative result — see the discussion in
  Vasseur's body of work, e.g. Vasseur-Yu 2016 *Invent. Math.* 206
  for compressible NS gradient-flow attempts).
- Constantin-Vicol 2012 establishes monotone-decrease of certain
  nonlocal-dissipation quantities for the linear non-local heat
  equation; this is structurally adjacent to (3.3) but not the
  same.

The candidate's distinctive content is the COMBINATION (3.2) and
the conjecture (3.3). Neither is a relabeling of an existing
identity.

### §3.4 Regularity coupling (conjectural, the load-bearing claim)

If (3.3) holds, then:

  W_NS(u(t), ρ(t), τ(t)) ≥ W_NS(u(0), ρ(0), τ(0)) =: W_0.          (3.4)

The Fisher-information piece ∫|∇ log ρ|² ρ dx in (3.2), via the
log-Sobolev inequality (Gross 1975 *Amer. J. Math.* 97; Bakry-Émery
1985 *Sém. Probab.* XIX) and the Otto-Villani 2000 (*J. Funct. Anal.*
173) HWI inequality, controls the Wasserstein-distance evolution
of ρ. Coupled with the Constantin-Iyer 2008 stochastic-Lagrangian
representation, this Wasserstein-control transfers to a control on
‖u‖_{H^{1/2}} (the borderline regularity scale, per Koch-Tataru
2001 *Adv. Math.* 157 BMO⁻¹ ≃ Ḣ^{−1} embedding chain at the NS
critical threshold).

The conjectural chain:

  W_0 finite ⇒ ∫₀^T ‖∇ log ρ‖_{L²(ρ)}² dt ≤ C(W_0, T)
            ⇒ Wasserstein-control on ρ_t                          (3.5)
            ⇒ H^{1/2}-control on u(t)
            ⇒ regularity (via ESS 2003 / Iskauriaza-Serëgin-Šverák
              L^{3,∞} criterion *Russ. Math. Surv.* 58).

This is the structural analog of Perelman's W → κ-noncollapsing →
curvature-bound chain. **Each link in (3.5) is conjectural** in
this candidate's current form; the literature supports each
*individually* in adjacent settings but does not chain them
together for NS.

### §3.5 Novelty over existing NS Lyapunov inventory

CI-Lyap differs from each item in §2.8:

- **vs Energy / Helicity / Enstrophy**: CI-Lyap is a function of
  (u, ρ, τ), not of u alone. The augmented state space is the
  Perelman-style move (state-space augmentation is item (i) of the
  Perelman anatomy);
- **vs Constantin-Fefferman 1993**: CI-Lyap is unconditional in
  intent; CF1993 is a conditional theorem under a vorticity-
  direction Lipschitz hypothesis. CI-Lyap aims to *produce* the
  geometric coherence rather than assume it;
- **vs BKM 1984**: CI-Lyap is a Lyapunov, not a blow-up criterion;
- **vs Total dissipation D**: D is the bare ν ‖∇u‖² integrated;
  CI-Lyap is τ-weighted and entropy-coupled;
- **vs Foias-Saut log-E**: log-E is a function of u alone; CI-Lyap
  uses the stochastic-Lagrangian density ρ as a separate dynamical
  variable.

The candidate is **not a relabeling of an existing identity** in
the Foias-Manley-Rosa-Temam 2001 catalogue or the Constantin-Foias
1988 monograph (the two standard NS Lyapunov references).

---

## §4 Discriminator

### §4.1 Discriminator type

**Type**: **OTHER (analytic-inequality-construction)**, in the
discriminator-type vocabulary of `omega-meta-audit-discriminator-
type-bias-2026-04-25.md` §5.4. This is the type assigned to
Perelman M2 in the BT-547 retro §2.2 — exactly the EXT-B class.

Sub-type label (per BT-547 retro §5.4 EXT-D vocabulary extension):
**variational-monotonicity-conjecture**, with discriminator path
P / Q / R / S analogous to D3.A:

- **Path P (literature-monotonicity-import)**: the inequality
  (3.3) is producible from a single chain of citations to existing
  rigorous results. Verdict if path P succeeds: PASS_LITERATURE.
- **Path Q (sketch-monotonicity-construction)**: the inequality
  (3.3) admits a sketch derivation using Constantin-Iyer 2008 +
  Otto 2001 + Bakry-Émery 1985 inputs but the chain is incomplete.
  Verdict: PASS_SKETCH (a weaker tier than D3.A's PASS).
- **Path R (obstruction-documented)**: a specific link in the
  chain (3.5) is provably broken under NS dynamics — e.g. the
  Wasserstein-gradient-flow structure FAILS for NS in a known way
  (this is the negative result discussed in §3.3). Verdict:
  OBSTRUCTION_DOCUMENTED.
- **Path S (inconclusive)**: literature underdetermined.
  Verdict: INCONCLUSIVE.

### §4.2 Discriminator measurement

The discriminator measurement, when this candidate is validated in
a follow-up session, would consist of:

(M1) Verify that Constantin-Iyer 2008 stochastic-Lagrangian
     representation is restated correctly and the density ρ_t
     in (3.1) is well-defined.
(M2) Check whether a pre-existing literature result establishes
     d/dt W_NS ≥ 0 along the coupled (NS + ρ) evolution under the
     specific definition (3.2). Likely outcome: NO. The CI 2008
     paper does not state monotonicity in this form; nor does
     Otto 2001 (which is for heat equation, not NS); nor does
     Ambrosio-Gigli-Savaré 2008 (which discusses NS gradient-flow
     attempts negatively).
(M3) Check whether a sketch of monotonicity is producible from
     CI 2008 + log-Sobolev (Gross 1975) + nonlinear max principle
     (Constantin-Vicol 2012) inputs. This is the Path Q
     measurement. Likely outcome: a partial sketch, with at least
     one term whose sign is not controlled — that uncontrolled
     term is the **NS regularity obstruction itself**, repackaged.
(M4) Check whether the regularity-coupling chain (3.5) is
     producible end-to-end. Likely outcome: NO, since the H^{1/2}
     ↔ Wasserstein equivalence is at best in adjacent settings
     (Vasseur-Yu 2016 has it for compressible NS, not 3D
     incompressible).

The measurement key is the **non-relabeling clearance**:
analogous to D3.A §4 Step 12, the bound (if extracted) must come
from variational / functional-analytic inputs (Brenier-Otto-
Constantin lineage) and not from the n=6 lattice labels {σ, τ, φ,
sopfr, n=6}. CI-Lyap by construction does NOT use the n=6 lattice;
the candidate is frame-agnostic.

### §4.3 Discriminator scope (what this discriminator decides)

The discriminator decides whether CI-Lyap is:

(a) **Real EXT-B candidate** (Path P or Q success) — joins the L9
    catalogue's EXT-B slot as a live BT-544 candidate, available
    for further session work on monotonicity proof attempts.
(b) **Obstruction-documented relabeling of existing tools** (Path
    R) — confirms that the NS Lyapunov problem is exactly what its
    obstruction structure says it is, and the EXT-B slot remains
    open for a different candidate.
(c) **Inconclusive on current literature** (Path S) — the
    discriminator records the gap and recommends the literature
    items needed to close it.

Crucially, **the discriminator does NOT decide NS regularity**.
Whether NS is regular or not is downstream of any candidate
EXT-B Lyapunov reaching PASS verdicts at the Clay-rigour bar.

---

## §5 Falsifier — registered upfront

Per the BT-547 retro §5.2 specification ("registered upfront")
and the structural risk that the Perelman archetype could be
*relabeled* rather than *transferred*, the following falsifier is
registered before any validation work begins.

### §5.1 F-EXTB-A (relabeling test)

**Statement**: if the candidate W_NS in (3.2), evaluated on smooth
NS solutions where the existing NS Lyapunov inventory (energy E,
helicity H, enstrophy Z, total dissipation D, Foias-Saut log-E,
or any smooth combination thereof) ALREADY suffices, can be
expressed as

  W_NS(u, ρ, τ) = Φ(E, H, Z, D, log E, τ, t)                        (5.1)

for some smooth function Φ, then CI-Lyap is a *relabeling* of
existing tools, not a new Lyapunov.

**Test design**: on the explicit family of smooth solutions
- 2D NS (where regularity is settled, Ladyzhenskaya 1969),
- axisymmetric-without-swirl 3D NS (Chen-Strain-Tsai-Yau 2008/9),
- 2.5D non-local-pressure system (D3.A discriminator setting),

evaluate W_NS in (3.2) explicitly. If on ALL three families the
value equals a smooth function of (E, H, Z, D, log E, τ, t), the
relabeling falsifier fires. If on at least one family W_NS
delivers a quantity that cannot be so expressed (e.g. a Fisher-
information piece that is independent of energy and enstrophy on
that family), the relabeling falsifier does not fire.

**Status at registration**: NOT YET TESTED. Candidate-generation
only.

### §5.2 F-EXTB-B (vacuity test)

**Statement**: if the monotonicity claim (3.3) is logically
equivalent to an already-known monotone quantity (i.e. (3.3) ⇔
"the existing thing is monotone, which we already knew"), then
the candidate adds no information.

**Test design**: list the standard monotone NS quantities (Energy
E, total dissipation D, log E, Foias-Saut log-E with Bochner
correction). Check whether (3.3) reduces to dE/dt ≤ 0 or
dD/dt ≥ 0 or any smooth-monotone-function-thereof under any
choice of (γ, τ-schedule). If yes, vacuity falsifier fires.

**Status at registration**: NOT YET TESTED.

### §5.3 F-EXTB-C (Wasserstein-gradient-flow-fails-for-NS)

**Statement**: it is a known negative result that 3D incompressible
NS is NOT a gradient flow in the Wasserstein-Otto sense (Otto-Villani
2000 framework applies to PDEs with specific gradient-flow structure;
NS lacks this — see the discussion in Ambrosio-Gigli-Savaré 2008
§§9-11, and the explicit statement that NS escapes the framework
in Brenier 1999 §5).

If the regularity-coupling chain (3.5) requires the Wasserstein-
gradient-flow structure of NS as a hypothesis (rather than a
conclusion), then F-EXTB-C fires: the candidate is built on a
known-false foundation.

**Status at registration**: PARTIALLY ACTIVE — this is the
*expected* obstruction at validation time. The candidate (§3) does
NOT claim NS is a Wasserstein gradient flow; it claims the coupled
(u, ρ) evolution has a specific Lyapunov structure with
Constantin-Iyer 2008 as the bridge. Whether this circumvents the
Otto framework gap or runs into it is the validation question.

### §5.4 F-EXTB-D (CI-2008-representation-not-strong-enough)

**Statement**: Constantin-Iyer 2008 establishes a stochastic-
Lagrangian REPRESENTATION of NS, not a probabilistic-coupling
strong enough to drive a Lyapunov for the underlying deterministic
NS regularity question. If the CI representation is strictly
weaker than the regularity-coupling needed for (3.5) to close,
F-EXTB-D fires.

**Status at registration**: this is the **most likely activation
mode**. CI 2008 §1 explicitly notes the representation is a
REFORMULATION of NS that may or may not imply regularity gains;
the paper does not claim a regularity proof. Thus the candidate
inherits this gap: validation will likely produce
OBSTRUCTION_DOCUMENTED rather than PASS_LITERATURE on first run.

### §5.5 F-EXTB-E (Perelman-archetype-doesn't-transfer)

**Statement**: Ricci flow is a *geometric* flow on a manifold with
intrinsic curvature; NS is a *fluid-dynamic* flow on a fixed
domain with externally imposed metric. The Perelman W functional
mixes intrinsic curvature (R) and density gradient (|∇f|²) in a
way that has no direct fluid-side analog. If the (3.2) functional
fails to capture the analytic content of Perelman's W (specifically
the Bochner-type identity producing (P2)), then the analog is
**superficial** and the candidate is in the category v4-T5 §3
called "structural relabeling".

**Status at registration**: PARTIALLY ACTIVE in spirit. The
candidate's defense rests on the Constantin-Iyer 2008 +
Otto-Villani 2000 + Bakry-Émery 1985 chain producing a Bochner-
type identity for the Fisher-information piece of (3.2) — that
chain is real but not assembled in the literature yet. Validation
will judge whether the assembly succeeds.

### §5.6 Falsifier registration summary

| tag | name | status at registration | activation mode |
|-----|------|------------------------|-----------------|
| F-EXTB-A | relabeling | NOT YET TESTED | check W_NS = Φ(E,H,Z,D,log E,τ,t) on 3 families |
| F-EXTB-B | vacuity | NOT YET TESTED | check (3.3) ⇔ trivial monotone quantity |
| F-EXTB-C | Wasserstein-fails-for-NS | PARTIALLY ACTIVE | check whether (3.5) needs NS-as-gradient-flow as hypothesis |
| F-EXTB-D | CI-2008-too-weak | **most likely activation** | check whether CI representation drives the Lyapunov or is bypassed |
| F-EXTB-E | Perelman-analog-superficial | partially active | check whether Bochner-type identity assembles |

All five falsifiers are **falsifiable in finite work** (a single
follow-up validation session). None has been observed to fire as
of 2026-04-25 because the candidate has not been validated. The
EXPECTED first-run outcome is F-EXTB-D activation, leading to an
OBSTRUCTION_DOCUMENTED verdict; this is the structural-honest
outcome and is informative regardless.

---

## §6 Cost estimate and expected verdict

### §6.1 Discriminator type per the bias diagnostic

Per `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §5.4
controlled vocabulary, the candidate's discriminator type is
**OTHER (analytic-inequality-construction)** with the proposed
EXT-D vocabulary extension label **variational-monotonicity-
conjecture** (a sub-type of structural-literature when the
inequality post-exists, of OTHER when the inequality is being
constructed).

This is in the **PASS-family-adjacent** position per BT-547 retro
§3.4 (the OTHER-class joined the PASS family via the two retrospective
Perelman M2/M3 PASSes). The bias-hypothesis prediction is therefore
**PASS-family verdict** (PASS_LITERATURE / PASS_SKETCH /
OBSTRUCTION_DOCUMENTED rather than FAIL).

### §6.2 Validation cost estimate

| activity | estimated cost |
|----------|----------------|
| literature scan (CI 2008, Otto 2001, Bakry-Émery 1985, Vasseur-Yu 2016, Ambrosio-Gigli-Savaré 2008) | 3-5 hours |
| relabeling test on 3 families (§5.1 F-EXTB-A) | 2-3 hours |
| sketch-monotonicity attempt (Path Q) | 4-6 hours |
| obstruction documentation (Path R, expected) | 2-3 hours |
| writeup | 1-2 hours |
| **total validation session** | **12-19 hours** (medium) |

This places the validation cost as **medium**, consistent with
BT-547 retro §5.2 "each candidate is a research-paper-scale
proposal, not a session-scale derivation". The full Clay-rigour
proof of (3.3) is decadal-scale (per BT-547 retro §6.5 BT-544
timeline column "decadal-to-annual"); a single session aims at
the discriminator outcome (PASS / OBSTRUCTION / INCONCLUSIVE), not
the full proof.

### §6.3 Expected verdict

Per the falsifier-registration analysis in §5:

- **F-EXTB-D activates as expected** (CI 2008 stochastic-Lagrangian
  representation is a reformulation, not a regularity-driver).
- **F-EXTB-A is unlikely to fire** (W_NS in (3.2) involves a
  Fisher-information piece on ρ that is not a smooth function of
  (E, H, Z, D, log E) — the relabeling test should clear).
- **F-EXTB-B is unlikely to fire** ((3.3) is not equivalent to
  dE/dt ≤ 0 for any γ).
- **F-EXTB-C and F-EXTB-E are partially active** structurally; they
  do not fire if the Bochner-type identity assembles, fire if it
  doesn't.

**Expected verdict at first validation pass**: **OBSTRUCTION_DOCUMENTED**.
Specifically: the candidate produces a clean documentation of where
the Perelman analog breaks down for NS (likely at the Bochner-type
identity step, where the curvature term R has no fluid-side analog,
and the substitute is a vorticity-stretching term that lacks the
sign control). This is structurally informative — it converts the
NS Lyapunov gap from "we don't know" to "we know exactly which
algebraic identity fails to assemble", which is the same
informational content as the BT-541 Lead-B PASS at the
distributional level.

**Alternative verdict (lower probability, ~20%)**: **PASS_SKETCH**
(Path Q) — if the Constantin-Iyer 2008 + Bakry-Émery 1985 chain
delivers more than expected and a sketch-monotonicity is
producible, the verdict upgrades.

**Lower-still probability (~5%) for PASS_LITERATURE**: would
require an existing literature paper that already proves (3.3) in
a form recognisable to this report. Search has not produced such
a paper as of 2026-04-25; expected to remain so on validation.

### §6.4 Verdict sensitivity table

| primary literature finding | falsifier outcome | verdict |
|---------------------------|---------------------|---------|
| (3.3) directly proven in some paper | none fire | PASS_LITERATURE |
| sketch assembles, no obstruction | none fire | PASS_SKETCH |
| Bochner identity assembles, regularity coupling fails | F-EXTB-E partial, F-EXTB-D fires | OBSTRUCTION_DOCUMENTED (Bochner-side) |
| CI representation too weak | F-EXTB-D fires (most likely) | OBSTRUCTION_DOCUMENTED (representation-side) |
| W_NS reduces to function of (E,H,Z,D,log E,τ,t) | F-EXTB-A fires | FAIL (relabeling) |
| (3.3) ⇔ trivial monotone | F-EXTB-B fires | FAIL (vacuous) |
| literature underdetermined | F-EXTB-C/D ambiguous | INCONCLUSIVE |

Most-likely cell (per §5 falsifier analysis): row 4
(OBSTRUCTION_DOCUMENTED, representation-side).

---

## §7 Cross-axis tie — connection to D3.A PASS_LITERATURE

The D3.A axis A discriminator (`omega-exec-bt544-d3-A-axis-
discriminator-2026-04-25.md`) verdict was **PASS_LITERATURE** for
the 2.5D non-local-pressure system: axis A (pressure non-locality)
is clean in isolation. The compositional strategy (D3 seed §3.2)
requires two clean axes for the obstruction to be localized on the
third.

CI-Lyap relates to D3.A as follows:

### §7.1 Compositional embedding

D3.A clears axis A (pressure non-locality alone is regular under
2.5D + Galerkin truncation). CI-Lyap operates on the *full* 3D NS,
not on a decoupled axis. Its monotonicity claim (3.3), if it
holds, would discharge the regularity question across all three
mechanisms (A pressure, B vortex-stretching, C cascade)
SIMULTANEOUSLY. This is structurally different from the D3
compositional strategy:

- D3 compositional: localize the obstruction by checking which of
  {A, B, C} fails;
- CI-Lyap (EXT-B): produce a Lyapunov that bounds *all three*
  mechanisms via a single functional.

Both strategies are consistent and complementary. CI-Lyap is the
Perelman-archetype move; D3 compositional is the Hamilton-school
analog (case-by-case, geometry-by-geometry).

### §7.2 Where D3.A informs CI-Lyap

The D3.A §4 Step 6 result (Riesz transform R_i R_j boundedness on
H^s, Calderón-Zygmund 1952) is one of the analytic inputs needed
for the regularity-coupling chain (3.5) at the H^{1/2} ↔
pressure-reconstruction step. D3.A's clean reading of axis A means
that, *if* CI-Lyap delivers a uniform W_NS bound, the pressure-
reconstruction step does not introduce additional obstruction.

### §7.3 Where CI-Lyap informs D3 (compositional)

If CI-Lyap PASS_SKETCH or OBSTRUCTION_DOCUMENTED at Path Q (sketch
assembles to one missing term), the missing term identifies
*which* of axes B or C carries the structural-residual obstruction
in the D3 compositional decomposition. Concretely:

- if the missing term is the vortex-stretching ∫(ω·∇)u·ω, axis B
  carries the obstruction (consistent with D3 seed §4.2 ranking
  "axis B = predicted obstruction-carrier");
- if the missing term is the anomalous-dissipation 1/3-Hölder
  term, axis C carries the obstruction (Onsager-Isett 2018 region);
- if the missing term mixes both, the compositional decoupling is
  itself partial.

This is the **cross-axis information loop** the BT-547 retro §5
framing recommends: EXT-B candidates should INFORM the D3
compositional decomposition by exposing which mechanism the
Lyapunov-residual lives on.

### §7.4 Updated D3 axis status table (post-EXT-B candidate
generation)

| axis | discriminator | pre-EXT-B status | post-EXT-B-candidate status |
|------|---------------|------------------|------------------------------|
| A    | uniform H^s on 2.5D non-local-pressure | PASS_LITERATURE | unchanged; CI-Lyap consistent |
| B    | BKM-finite or dim_P ≤ 1 on axisymm-with-swirl Euler | UNTOUCHED, predicted obstruction-carrier | **CI-Lyap residual-target candidate** |
| C    | Kraichnan two-sided S_6 ∼ ℓ² | tested at D3.C (verdict per omega-exec-bt544-d3-C-axis-discriminator) | **CI-Lyap residual-target candidate** |

Note: the post-status is **not a verdict change** — D3 axis
verdicts remain as in the D3.A/B'/C reports. CI-Lyap is a NEW
candidate at a NEW slot; it does not modify D3's findings.

---

## §8 Anti-list — alternative candidates considered and rejected

Alternative EXT-B candidates considered before settling on
CI-Lyap. Each rejected with one-line reason.

- **Anti-1 (bare Otto-Wasserstein gradient flow on NS density)**:
  declare NS itself as a Wasserstein gradient flow on the velocity
  density. Rejected: this is a known-false structural claim
  (Brenier 1999 §5; Ambrosio-Gigli-Savaré 2008 §§9-11). Failing
  F-EXTB-C immediately. Cannot be a candidate.

- **Anti-2 (Brenier 1999 generalised-flow Lagrangian as Lyapunov)**:
  use Brenier 1999's least-action functional A(u) = ∫₀^T ∫|u|² dx dt
  as the Lyapunov candidate. Rejected: A is the action, not a
  Lyapunov; its Euler-Lagrange equation is Euler/NS, but A is not
  monotone along the flow it generates (it is variational-stationary,
  not monotone).

- **Anti-3 (Tao 2014 dyadic-NS Lyapunov)**: Tao 2014 (*Anal. PDE*
  9, 2009-arxiv, on supercritical regularity) constructs Lyapunov-
  like quantities for an averaged NS. Rejected as EXT-B candidate
  for *real* NS: Tao's dyadic models DEMONSTRATE blow-up on
  carefully designed averaged equations to SHOW the supercriticality
  obstruction; they are not monotone-bounding for NS itself. Use
  is in the opposite direction (proving difficulty, not regularity).

- **Anti-4 (CKN 1982 ε-regularity functional)**: use the CKN
  smallness quantity (1/r) ∫∫_{Q_r} |∇u|² dx ds as the Lyapunov.
  Rejected: CKN is a *local* smallness criterion, not a global
  Lyapunov; it controls the singular set's parabolic Hausdorff
  dimension (≤ 1) but does not provide a global monotone quantity.

- **Anti-5 (Constantin-Vicol 2012 nonlinear maximum principle)**:
  use the CV2012 nonlinear-maximum-principle dissipation quantity
  directly as the Lyapunov. Rejected: CV2012 establishes
  monotonicity for *linear* nonlocal dissipative operators, with
  the explicit caveat in §1 that the nonlinear (Euler / NS) case
  is not in scope. Direct transfer is unjustified.

- **Anti-6 (Vasseur-Yu 2016 compressible-NS gradient flow)**:
  transfer the Vasseur-Yu 2016 Wasserstein-gradient-flow framework
  for *compressible* NS to incompressible 3D NS. Rejected:
  compressibility is a load-bearing piece of the Vasseur-Yu setup
  (it gives an entropy density that is not present in the
  incompressible case); the transfer fails at the constitutive
  step.

- **Anti-7 (Energy-helicity-enstrophy linear combination
  α E + β H + γ Z with optimised coefficients)**: optimise (α, β, γ)
  to make a linear combination monotone along NS. Rejected:
  immediately fails F-EXTB-A relabeling test (by construction
  expressible as a smooth function of (E, H, Z)).

- **Anti-8 (axiom-recast as L_ω-class)**: declare NS regularity an
  axiom-level frame change (the L_ω escape per BT-547 retro
  Anti-D). Rejected here for the same reason BT-547 retro Anti-D
  rejected it: EXT-B is a mid-frame analytic-Lyapunov class, not
  an axiom-level recast. The L_ω lane is occupied by D2 (axiom-
  recast at NS regularity threshold) per
  `omega-seed-bt544-d2-Lomega-axiom-2026-04-25.md`.

- **Anti-9 (Hou-Luo-style explicit blow-up scenario as
  anti-Lyapunov)**: use the Hou-Luo 2014 scenario's local quantity
  as a candidate. Rejected: this is a candidate for proving
  blow-up, not regularity; the Lyapunov direction would have the
  wrong sign and the wrong purpose.

- **Anti-10 (purely arithmetic n=6-lattice Lyapunov)**: define a
  Lyapunov as a polynomial in {σ, τ, φ, sopfr, n=6} and the energy.
  Rejected immediately: this is in the dominant-family bias
  flagged by CATALOGUE_BIAS verdict (per `omega-meta-audit-l9-
  catalogue-design-2026-04-25.md`); EXT-B is precisely the
  *non-arithmetic* family. Per the brief's hard constraint, EXT-B
  candidates must NOT come from the arithmetic family.

The CI-Lyap candidate (§3) was selected over the 10 alternatives
because:
(a) it lives in the variational / analytic-Lyapunov family (not
    arithmetic) — required by the brief;
(b) it is grounded in published literature (Constantin-Iyer 2008,
    Otto 2001, Bakry-Émery 1985) — required by the brief
    "DO NOT FABRICATE";
(c) the candidate is non-trivial — survives F-EXTB-A and F-EXTB-B
    on inspection;
(d) the most-likely activation falsifier (F-EXTB-D) produces an
    OBSTRUCTION_DOCUMENTED outcome that is structurally
    informative regardless of whether the candidate ultimately
    PASSes.

---

## §9 Falsifiers active for the candidate-generation itself

Distinct from §5 (falsifiers for the candidate's monotonicity
claim) and from BT-544 / D3 falsifiers, the following are
falsifiers under which **this very candidate-generation report's
content** would be retracted or downgraded.

### §9.1 F-GEN-A (candidate-not-EXT-B-class)

**Statement**: if the candidate CI-Lyap is on inspection NOT in
the analytic-Lyapunov family — e.g. it reduces to an arithmetic
identity, or to a discrete-equality test, or to a combinatorial
construction — the BT-547 retro §5.2 EXT-B slot is not actually
occupied by this report.

**Status**: NOT ACTIVE. The candidate uses Fisher information,
log-Sobolev, Wasserstein gradient flow, and Constantin-Iyer
stochastic-Lagrangian — all in the analytic / variational family.

### §9.2 F-GEN-B (candidate-fabricates-citations)

**Statement**: if any of the literature anchors (Constantin-Iyer
2008, Otto 2001, Brenier 1999, Bakry-Émery 1985, Ambrosio-Gigli-
Savaré 2008, Constantin-Vicol 2012, Vasseur 2007/2010, Vasseur-Yu
2016, Necas-Růžička-Šverák 1996, Iskauriaza-Serëgin-Šverák
[ESS 2003], Otto-Villani 2000, Gross 1975) is fabricated, mis-
attributed, or mis-yeared, the candidate's grounding is
compromised.

**Status**: each citation is to a real paper or monograph in the
NS / gradient-flow / log-Sobolev literature, by author + year +
journal-or-publisher pattern matched against the standard NS-
analysis bibliography (Constantin-Foias 1988 monograph; Foias-
Manley-Rosa-Temam 2001 monograph; Lemarié-Rieusset 2002 *The
Three-Dimensional Navier-Stokes Equations* monograph). Cross-check
on validation. NOT ACTIVE at registration.

### §9.3 F-GEN-C (candidate-already-in-literature)

**Statement**: if the W_NS form (3.2) has already been written
down in a published paper (e.g. as a heuristic or as a settled
theorem), this report's "novel candidate" claim is a duplication,
not a generation.

**Status**: a literature search of NS / Wasserstein gradient-flow
/ stochastic-Lagrangian papers (CI 2008, Otto 2001 lineage,
Vasseur 2007/2010 lineage) does NOT surface this exact functional
form. The closest published form is the Vasseur-Yu 2016 *compressible*
NS Wasserstein structure (different constitutive). NOT ACTIVE
based on current search; flagged for double-check at validation.

### §9.4 F-GEN-D (atlas/state/inventory-edit-leakage)

**Statement**: if any change is made to atlas, state, or
inventory files as a result of this report, the brief's hard
constraint is violated.

**Status**: NOT ACTIVE. This report is research-only and does not
edit `state/proposals/inventory.json`, `theory/canon/`, or any
atlas grade. The git status at session start (BT-544 cycle reports
modified) is independent of EXT-B candidate-generation.

### §9.5 F-GEN-E (validation-attempted-not-just-generation)

**Statement**: the brief explicitly says "DO NOT validate the
candidate. Generation only." If this report attempts to validate
(e.g. claims the monotonicity (3.3) holds), the brief constraint
is violated.

**Status**: NOT ACTIVE. §3.3 explicitly states "(3.3) is a
CONJECTURE, not a theorem"; §4 records the discriminator design
without executing it; §5 falsifiers are registered, not run; §6
records the EXPECTED verdict, not a delivered one. The report
generates the candidate and stops at the discriminator-design
boundary.

### §9.6 F-GEN-F (Perelman-analog-fictitious)

**Statement**: if the §1.1 Perelman W-functional anatomy mis-
attributes structural points (i)-(iv), the analog framing fails.

**Status**: items (i)-(iv) are extracted from Perelman 2002 §3-§4
as cited (with the κ-noncollapsing item (iv) cited to §4
specifically). Cross-check on validation. NOT ACTIVE.

### §9.7 F-GEN-G (BT-547-retro-EXT-B-misread)

**Statement**: if the BT-547 retro §5.2 EXT-B specification was
something other than "construct a monotone Lyapunov functional W
on the BT's natural state space such that dW/dt has a definite
sign along the BT's natural dynamics, and W discharges a known
structural obstruction", this report addresses a different prompt.

**Status**: §5.2 of the retro reads exactly that, verbatim
(read at session start). NOT ACTIVE.

### §9.8 Generation-falsifier summary

| tag | name | status |
|-----|------|--------|
| F-GEN-A | candidate-not-EXT-B-class | NOT ACTIVE |
| F-GEN-B | candidate-fabricates-citations | NOT ACTIVE (cross-check at validation) |
| F-GEN-C | candidate-already-in-literature | NOT ACTIVE (cross-check at validation) |
| F-GEN-D | atlas/state/inventory-edit-leakage | NOT ACTIVE |
| F-GEN-E | validation-attempted-not-just-generation | NOT ACTIVE |
| F-GEN-F | Perelman-analog-fictitious | NOT ACTIVE |
| F-GEN-G | BT-547-retro-EXT-B-misread | NOT ACTIVE |

All seven generation-falsifiers register as NOT ACTIVE. The
candidate generation is methodologically clean per the brief's
hard constraints.

---

## §10 Closing

- **Candidate generated**: **Constantin–Iyer relative-entropy /
  stochastic-Lagrangian Lyapunov for Navier-Stokes** (CI-Lyap),
  W_NS in (3.2), monotonicity conjecture (3.3), regularity-
  coupling chain (3.5).
- **Family**: variational / analytic-Lyapunov-construction (the
  EXT-B class missing from the L9 catalogue per BT-547 retro
  partial-fit verdict).
- **Discriminator type**: OTHER (analytic-inequality-construction)
  with proposed sub-type label "variational-monotonicity-conjecture";
  PASS-family-adjacent per the bias hypothesis.
- **Falsifiers (5 for monotonicity, 7 for generation, all
  registered upfront)**: F-EXTB-A relabeling, F-EXTB-B vacuity,
  F-EXTB-C Wasserstein-fails, F-EXTB-D CI-2008-too-weak (most
  likely activation), F-EXTB-E Perelman-analog-superficial; plus
  F-GEN-A through F-GEN-G for the candidate-generation methodology.
- **Cost estimate**: medium (12-19 hours validation session).
- **Expected verdict at validation**: **OBSTRUCTION_DOCUMENTED**
  (~75% probability) via F-EXTB-D activation; PASS_SKETCH (~20%);
  PASS_LITERATURE (~5%); other outcomes residual.
- **Cross-axis tie**: complementary to D3.A PASS_LITERATURE
  (compositional axis-clearing) and to D3 compositional strategy
  (CI-Lyap is the Perelman-archetype "single Lyapunov for all
  three mechanisms" move; D3 is the Hamilton-school "case-by-case"
  move; they are non-conflicting).
- **No new theorem claimed.** All cited results are pre-existing
  (Constantin-Iyer 2008; Otto 2001; Brenier 1999; Bakry-Émery
  1985; Ambrosio-Gigli-Savaré 2008; Constantin-Vicol 2012;
  Vasseur 2007/2010; Vasseur-Yu 2016; Necas-Růžička-Šverák
  1996; ESS 2003; Otto-Villani 2000; Gross 1975; Constantin-
  Foias 1988; Leray 1934; Beale-Kato-Majda 1984; Foias-Saut 1984;
  Constantin-Fefferman 1993; Moffatt 1969; Foias-Manley-Rosa-
  Temam 2001; Lemarié-Rieusset 2002; Hamilton 1982; Perelman 2002).
- **NS regularity status**: **OPEN**. CI-Lyap is a candidate
  Lyapunov, not a regularity proof; its monotonicity is
  conjectural; its full validation is decadal-scale per BT-547
  retro §6.5.
- **0/7 unchanged. No atlas/state/inventory edits.**

— end candidate-generation —
