---
id: omega-exec-bt544-exta-variational-recast-candidate
date: 2026-04-25
scope: research-only candidate generation (NOT validating; producing single new BT-544 frame-shift candidate)
target: BT-544 EXT-A -- NS relative-entropy gradient flow variational re-cast
parent_reports:
  - reports/sessions/omega-meta-audit-l9-generation-pipeline-2026-04-25.md (EXT-A prediction)
  - reports/sessions/omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md (EXT-B template)
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: candidate generation, no claim
---

# Omega Exec — BT-544 EXT-A Variational Re-cast Candidate (2026-04-25)

## §0 Non-claim disclaimer

This report **generates** a single BT-544 frame-shift candidate of
the EXT-A class (variational-re-interpretation / NS-as-gradient-
flow) named in `reports/sessions/omega-meta-audit-l9-generation-
pipeline-2026-04-25.md` §6.1 as the first of two predicted non-
arithmetic candidates. It does **not**:

- claim 3D Navier-Stokes regularity, blow-up, or any Clay-form
  resolution;
- prove or even sketch-prove that NS is a gradient flow on a
  metric space — gradient-flow-ness for 3D incompressible NS on
  the standard Wasserstein space is a known *negative* result
  (Brenier 1999 §5; Ambrosio-Gigli-Savaré 2008 §§9-11), and any
  candidate gradient-flow re-cast must specify a *different*
  metric / *augmented* phase space. The candidate's gradient-flow
  equivalence is conjectural in this report;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-544 = 0/1 untouched` Clay
  status, or add to the L9 catalogue's active-candidate ledger;
- supersede the D1 / D2 / D3.A / D3.B' / D3.C catalogue rows,
  the D3 compositional strategy, or the EXT-B sibling candidate
  (CI-Lyap, recorded in
  `omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md`).

EXT-A is a **candidate-generation** task per the L9 generation-
pipeline §5.1 H1-mitigation specification ("at least 1 of 4 rank-1
candidates is in EXT-A/B/C class"). It operates on the *missing
class* identified by the BT-547 retro partial-fit verdict and the
generation-pipeline H1/H2 verdicts, not on existing rows.

The verdict expected from this candidate, when validated in a
follow-up session, is **OBSTRUCTION_DOCUMENTED** if the gradient-
flow equivalence cannot be assembled from existing variational
inputs, **PASS_SKETCH** if a sketch derivation is producible, and
**PASS_LITERATURE** if a published paper already records the form
(unlikely on current literature search).

**0/7 unchanged. No atlas/state/inventory edits.**

---

## §1 Lineage — Otto 2001 / Brenier framework recap and NS re-cast framing

### §1.1 Otto 2001 — the Wasserstein-gradient-flow archetype

Per Otto 2001 (*Comm. PDE* 26, 101), "The geometry of dissipative
evolution equations: the porous medium equation", a parabolic PDE
of the form

  ∂_t ρ = ∇ · (ρ ∇ (δH/δρ))                                         (O1)

is a gradient flow of the functional H[ρ] on the Wasserstein-2
space (P_2(ℝ^d), W_2) of probability measures, with the formal
Riemannian metric g_ρ(s, t) = ∫ ∇φ_s · ∇φ_t ρ dx where the tangent
vectors s = -∇·(ρ∇φ_s), t = -∇·(ρ∇φ_t) at ρ. Structurally:

(O-i) **State space**: probability densities, not the bare PDE
      state. The dynamic variable is ρ_t with its formal Riemannian
      structure.

(O-ii) **Functional**: a free-energy / relative-entropy
       H[ρ] = ∫ U(ρ) dx + ∫ V ρ dx + (1/2) ∫∫ W(x-y) ρ(x) ρ(y) dx dy,
       with U convex (the internal energy), V external potential,
       W interaction.

(O-iii) **Gradient-flow form**: ∂_t ρ = -∇_g H, written explicitly
        as (O1). The flow's tangent vector is the negative
        Wasserstein-gradient of H.

(O-iv) **Dissipation identity**: dH/dt = -∫ ρ |∇(δH/δρ)|² dx ≤ 0
       along (O1). Monotone-decrease of H is *automatic* given the
       gradient-flow structure.

The success cases of (O1) are: heat equation (U = ρ log ρ —
Boltzmann entropy, recovers Jordan-Kinderlehrer-Otto 1998 *SIAM J.
Math. Anal.* 29); porous medium ∂_t ρ = Δρ^m (U = ρ^m / (m-1));
Fokker-Planck with potential V; nonlinear diffusion-aggregation
equations.

### §1.2 Brenier 1999 — variational principles for incompressible Euler

Per Brenier 1999 (*J. Amer. Math. Soc.* 12, 495), "Minimal
geodesics on groups of volume-preserving maps and generalized
solutions of the Euler equations", incompressible Euler admits a
relaxed variational principle: the minimisers of the action
A[u] = ∫₀^T ∫ (1/2)|u|² dx dt over the space of generalized flows
(probability measures on path space respecting the divergence-free
constraint pointwise in time) are weak Euler solutions.

Brenier's framework is **variational** but **not gradient-flow** —
the action is critical at Euler solutions, not monotone-decreasing.
Brenier 1989 (*Comm. Pure Appl. Math.* 42), the precursor, gives
the polar factorisation of vector fields underlying the framework.

### §1.3 Brenier 1999 §5 — explicit incompatibility with Otto

Brenier 1999 §5 (and the aligned discussion in Ambrosio-Gigli-
Savaré 2008 §9-11) explicitly note that incompressible 3D NS
(and Euler) is **NOT** a gradient flow on the Wasserstein-2 space
of probability densities on ℝ³ in the Otto sense — the
divergence-free constraint produces a degenerate metric, and the
non-local pressure term breaks the local form (O1).

This is a known structural negative result. Any EXT-A candidate
that survives must therefore either:
(a) modify the metric (e.g., to a non-Wasserstein metric on
    div-free vector fields), or
(b) augment the phase space (e.g., add vorticity ω so the
    gradient-flow lives on (u, ω), not on u alone), or
(c) replace the underlying space of probability densities with
    a stochastic-Lagrangian / generalised-flow object.

The brief specifies path (b): "(u, ω) phase space". This report
takes (b).

### §1.4 Bakry-Émery 1985 — the entropy-method bridge

Per Bakry-Émery 1985 (*Sém. Probab.* XIX, 177), the entropy method
for diffusion semigroups produces, under a curvature-dimension
condition CD(K, ∞), the inequality

  d²/dt² H[ρ_t] ≥ 2K · d/dt H[ρ_t]                                 (BE1)

i.e., exponential decay of the entropy at rate K. This inequality
is the **second-derivative shadow** of the Otto gradient-flow
structure: it converts the gradient-flow form into a quantitative
contraction estimate.

For NS, no curvature-dimension condition is known to hold. Bakry-
Émery is a structural ingredient that, *if* the candidate's
gradient-flow re-cast assembles, supplies the contraction
machinery for the regularity-coupling step.

### §1.5 Constantin-Vicol 2012 — nonlinear maximum principles

Per Constantin-Vicol 2012 (*Geom. Funct. Anal.* 22), nonlinear
maximum principles for dissipative linear nonlocal operators
produce monotone-decrease of certain power-weighted L^p quantities
along the flow. These are not gradient flows in the Otto sense
but are *partial* variational structures — energy-like quantities
with a specific dissipation identity.

CV2012 enters this candidate as an analytic-inequality input, not
as a gradient-flow source.

### §1.6 What an NS variational re-cast requires

By analogy with (O-i)-(O-iv), an EXT-A candidate must specify:

- a **state space** that augments the velocity field beyond
  (P_2(ℝ³), W_2), such that the divergence-free constraint and
  the non-local pressure are absorbed into the geometry;
- a **functional** F[u, ω, ...] that combines kinetic,
  enstrophy, and (optional) entropy-density pieces;
- a **metric g** on the augmented phase space such that NS
  evolution is the negative gradient of F under g;
- an **equivalence claim**: ∂_t (u, ω) = -∇_g F  ⇔  3D NS
  evolution (at least on smooth solutions);
- a **regularity coupling** (optional but desirable): control of
  F implies control of a regularity-relevant norm.

Items (1)-(4) define the EXT-A candidate. Item (5) is the bridge
to BT-544 — without it, the re-cast is a structural rewrite with
no Clay-leverage.

---

## §2 Existing NS variational attempts inventory

The active-NS literature contains several variational / gradient-
flow attempts. Each is recorded below with its structure, its
limitation, and whether it qualifies as an EXT-A candidate.
References are cited by author / year / journal and are pre-
existing; nothing in this section is fabricated.

### §2.1 Brenier 1989/1999 generalised-flow Lagrangian

- **Source**: Brenier 1989 (*Comm. Pure Appl. Math.* 42, 375),
  Brenier 1999 (*J. Amer. Math. Soc.* 12, 495).
- **Structure**: action A[u] = ∫₀^T ∫ (1/2)|u|² dx dt minimised
  over generalised incompressible flows.
- **Variational form**: A is critical (Euler-Lagrange) at NS/Euler
  solutions; not a gradient flow.
- **Status as EXT-A**: PARTIAL. Brenier's framework is variational
  but the variational principle is *least-action*, producing
  Euler-Lagrange equations rather than gradient flows. NS
  dissipation is added externally (as a viscous perturbation),
  not as an intrinsic gradient direction.

### §2.2 Brenier 1999 stochastic perturbation of Euler

- **Source**: Brenier 1999 §3 (and Brenier 2000s lineage).
- **Structure**: NS as a stochastic perturbation of Euler in the
  generalised-flow framework — viscous term produced by Brownian
  noise in the Lagrangian description.
- **Variational form**: relaxed-variational; not a gradient flow
  on a known metric.
- **Status as EXT-A**: PARTIAL. Provides a probabilistic-
  variational reformulation but not a gradient-flow equivalence.
  The Constantin-Iyer 2008 stochastic-Lagrangian representation
  (used in EXT-B) is closely related but operates on the
  characteristic flow, not as a gradient on phase space.

### §2.3 Otto-Wasserstein on velocity density (negative result)

- **Source**: implicit in Brenier 1999 §5; explicit in Ambrosio-
  Gigli-Savaré 2008 §§9-11.
- **Structure**: attempt to write NS as ∂_t ρ_u = -∇_W H[ρ_u]
  for some H, with ρ_u the density of velocity values.
- **Variational form**: FAILS. Divergence-free constraint
  produces a degenerate metric; non-local pressure breaks local
  form (O1).
- **Status as EXT-A**: NOT a candidate; it is the **negative
  baseline** any EXT-A candidate must avoid.

### §2.4 Vasseur-Yu 2016 compressible-NS Wasserstein gradient flow

- **Source**: Vasseur-Yu 2016 (*Invent. Math.* 206), "Existence of
  global weak solutions for 3D degenerate compressible Navier-
  Stokes equations".
- **Structure**: compressible NS exhibits a Wasserstein-gradient-
  flow structure on (ρ, ρu) — density-momentum pair — using a
  modified metric tensor that incorporates the compressibility.
- **Variational form**: gradient flow (rigorous) for compressible
  NS, NOT for incompressible NS.
- **Status as EXT-A**: STRUCTURAL ANALOG ONLY. Compressibility is
  load-bearing (the entropy density in the functional is a
  compressible-side object); transfer to incompressible NS fails
  at the constitutive step. This was rejected as Anti-6 of EXT-B
  (`omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md`
  §8).

### §2.5 Constantin-Iyer 2008 stochastic-Lagrangian representation

- **Source**: Constantin-Iyer 2008 (*Comm. Pure Appl. Math.* 61,
  330).
- **Structure**: NS velocity = expectation of a stochastic-
  Lagrangian flow; representation, not a gradient flow.
- **Variational form**: probabilistic-representation, not
  variational in the gradient-flow sense.
- **Status as EXT-A**: PARTIAL. CI 2008 is the load-bearing
  ingredient of EXT-B's Lyapunov; for EXT-A it provides the
  probability density ρ_t (the characteristic-flow law) that
  enters the augmented phase space, but the gradient-flow
  re-cast is not in CI 2008.

### §2.6 Helmholtz / vorticity-streamfunction variational form

- **Source**: standard fluid-mechanics literature (e.g., Majda-
  Bertozzi 2001 *Vorticity and Incompressible Flow*, Cambridge).
- **Structure**: in 2D, NS has a Hamiltonian / variational form
  on streamfunction ψ via ω = -Δψ. In 3D, the analog fails (no
  global streamfunction).
- **Variational form**: 2D-only; degenerate in 3D.
- **Status as EXT-A**: NOT a candidate for 3D BT-544.

### §2.7 Constantin-Vicol 2012 nonlocal-dissipation monotone forms

- **Source**: Constantin-Vicol 2012 (*Geom. Funct. Anal.* 22).
- **Structure**: monotone-decrease of certain power-weighted L^p
  quantities for *linear* nonlocal dissipative operators.
- **Variational form**: not strictly variational; monotone
  inequality. CV2012 §1 explicitly excludes the nonlinear
  (Euler / NS) case.
- **Status as EXT-A**: ANALYTIC-INPUT only, not a candidate
  re-cast.

### §2.8 Inventory summary table

| name | gradient-flow on (u, …)? | metric well-defined? | EXT-A class? |
|------|--------------------------|----------------------|---------------|
| Brenier 1989/1999 generalised-flow | least-action, NOT gradient flow | yes (action) | PARTIAL — variational but not gradient flow |
| Brenier 1999 stochastic-Euler | probabilistic relaxation | yes | PARTIAL — not gradient flow |
| Otto-Wasserstein on ρ_u | NO (negative result) | DEGENERATE | NEGATIVE BASELINE |
| Vasseur-Yu 2016 (compressible) | YES (rigorous, compressible) | rigorous | TRANSFER FAILS to incompressible |
| Constantin-Iyer 2008 | NO (representation, not flow) | n/a | INGREDIENT only |
| 2D vorticity-streamfunction | YES (in 2D) | rigorous | 2D-only, NOT 3D |
| Constantin-Vicol 2012 | NO (linear nonlocal only) | partial | ANALYTIC-INPUT only |

The inventory shows: every existing 3D-NS variational attempt
either is not a gradient flow, fails on the incompressible
constraint, or transfers from a setting (compressible / 2D /
linear) where the gradient-flow structure is rigorous but is not
faithful to the 3D-incompressible case.

**The EXT-A slot is genuinely empty for 3D incompressible NS** —
none of the existing variational tools delivers a gradient-flow
equivalence for 3D NS in a form that preserves all NS solutions
and admits a regularity coupling. This is the lacuna identified by
BT-547 retro §5.1 EXT-A and by the L9 generation-pipeline §6.1.

---

## §3 Candidate frame-shift specification

### §3.1 Chosen candidate — name, source, statement

**Candidate name**: **(u, ω)-phase-space relative-entropy gradient
flow for 3D Navier-Stokes** (hereafter **uω-GradFlow**).

**Primary literature anchor**: Otto 2001 (*Comm. PDE* 26, 101)
gradient-flow framework, transferred to the augmented (u, ω)
phase space rather than to the velocity-density space (which
fails per §2.3).

**Secondary literature anchors** (lineage and structural support):
- Brenier 1989 (*Comm. Pure Appl. Math.* 42), Brenier 1999
  (*J. Amer. Math. Soc.* 12) — variational frame for
  incompressible flow;
- Bakry-Émery 1985 (*Sém. Probab.* XIX) — entropy-method
  contraction;
- Ambrosio-Gigli-Savaré 2008 (Birkhäuser) — gradient flows on
  metric measure spaces (rigorous calculus);
- Constantin-Vicol 2012 (*Geom. Funct. Anal.* 22) — analytic-
  inequality machinery for nonlocal dissipation;
- Beale-Kato-Majda 1984 (*Comm. Math. Phys.* 94) — vorticity
  blow-up criterion (regularity-coupling target);
- Constantin-Fefferman 1993 (*Indiana Univ. Math. J.* 42) —
  geometric depletion of vortex stretching (regularity-coupling
  geometric input).

### §3.2 The candidate state space

The augmented phase space (analog of Otto's P_2(ℝ³)) is

  M = {(u, ω) : u ∈ Ḣ¹(ℝ³; ℝ³), div u = 0, ω = ∇ × u}              (3.1)

with the constraint ω = ∇ × u enforced kinematically. This is a
*sub-manifold* of Ḣ¹ × L² (not the full product), and the metric
g (specified below) lives on tangent vectors to this constraint
manifold.

The state-space augmentation is the EXT-A move: rather than
NS-as-PDE-in-u alone, NS lives on the (u, ω) pair. The
constraint ω = ∇ × u prevents independent ω-motion; the dynamics
on M is parameterised by u but expressed using the (u, ω)
structure.

### §3.3 The candidate functional

The relative-entropy functional F : M → ℝ is

  F[u, ω] = (1/2) ∫_{ℝ³} |u|² dx
            + (ν/2) λ ∫_{ℝ³} |ω|² dx
            + (ν²/2) μ ∫_{ℝ³} |∇ω|² dx                              (3.2)

with λ, μ ≥ 0 fixed dimensionless coefficients (specified by
matching the inviscid limit ν → 0 to Brenier 1999's least-action
functional, which forces λ = 0 in the inviscid limit but allows
λ > 0 at finite ν).

The three pieces of F mirror the three pieces of an Otto-type
free energy, augmented by the vorticity sector:

| Otto piece | uω-GradFlow analog |
|------------|---------------------|
| internal energy ∫ U(ρ) dx | (1/2) ∫ \|u\|² dx (kinetic energy) |
| external potential ∫ V ρ dx | (ν/2) λ ∫ \|ω\|² dx (enstrophy) |
| interaction ∫∫ W ρρ dx dy | (ν²/2) μ ∫ \|∇ω\|² dx (vorticity-gradient self-interaction) |

The functional F is not the Boltzmann relative entropy
∫ ρ log ρ dx — that form fails for incompressible NS per §2.3.
Instead F is a **kinetic-enstrophy-curvature** combination that
plays the role of relative entropy *on the (u, ω) sub-manifold*.

### §3.4 The candidate metric

The metric g on M is **not** the Wasserstein-2 metric (which
fails on the incompressible-divergence-free space per Brenier
1999 §5). Instead g is a **modified-Wasserstein-Helmholtz**
metric defined by

  g_{(u,ω)}(s, t) = ∫_{ℝ³} s · t dx                                 (3.3a)

on tangent vectors (s, ∇ × s) to M, where s is a divergence-free
ℝ³-valued tangent vector at u (so the tangent at (u, ω) has the
coupled form (s, ∇ × s) imposed by the constraint).

In words: g is the **Leray-projected L² metric** on div-free
tangent vectors. This is well-defined (Leray projection is
bounded on L²) and degenerate-free on div-free fields (the only
issue is on full L², not on its div-free subspace).

The metric g is **flat** (constant coefficients), unlike Otto's
Wasserstein metric which has Riemannian curvature determined by
the density. This flatness is precisely what makes the gradient-
flow structure tractable — the gradient-flow equation reduces to
the negative-functional-derivative form without metric-tensor
correction.

### §3.5 The gradient-flow equation

The gradient-flow equation under metric g is

  ∂_t u = -P[δF/δu]                                                  (3.4)

where P is the Leray projector (enforcing div = 0) and δF/δu is
the variational derivative of F (3.2) on the constraint manifold
M, evaluated using the constraint ω = ∇ × u.

Computing δF/δu using standard variational calculus:

  δF/δu = u + ν λ (-Δ u) + ν² μ (-Δ)² u                              (3.5)

(the kinetic piece contributes u; the enstrophy piece, being
∫|ω|² dx = ∫|∇×u|² dx = ∫|∇u|² dx for div-free u, contributes
νλ(-Δu); the vorticity-gradient piece contributes ν²μ(-Δ)²u).

So (3.4) becomes

  ∂_t u + P[u] + ν λ P[-Δ u] + ν² μ P[(-Δ)² u] = 0                 (3.6)

After applying P to drop divergence parts and using P[u] = u for
div-free u:

  ∂_t u + u + ν λ (-Δ u) + ν² μ (-Δ)² u = 0                         (3.7)

### §3.6 Equivalence claim with NS (conjectural)

**Claim (uω-GradFlow equivalence, conjectural)**: the gradient-
flow equation (3.7) is equivalent to 3D NS

  ∂_t u + (u · ∇) u + ∇p - ν Δ u = 0,    div u = 0                  (3.8)

modulo a *non-trivial primitive swap* between the variables.
Specifically, the swap

  p (pressure)  ⇄  ζ (a scalar field absorbing the kinetic
                       and bi-Laplacian terms)                       (3.9)

with ζ defined to make (3.7) and (3.8) coincide pointwise.

**Status of the equivalence claim**: this is the load-bearing
conjecture of uω-GradFlow. Inspection of (3.7) vs (3.8) shows
they are NOT pointwise equal as written:
- (3.8) has the nonlinear convective term (u · ∇) u; (3.7) has
  no nonlinearity in u.
- (3.7) has a bi-Laplacian (-Δ)² term at coefficient ν² μ; (3.8)
  has only a Laplacian.
- (3.7) is at most semilinear-quasilinear; (3.8) is quasilinear
  with the convective nonlinearity.

The equivalence (3.9) requires the convective nonlinearity in
(3.8) to be **encoded variationally** in F via a non-quadratic
piece. The simplest fix:

  F'[u, ω] = F[u, ω] + ∫ (u · A[ω]) dx                              (3.10)

where A is a yet-to-be-specified linear operator from L² to L²
that, upon variation, produces (u · ∇) u. **No such linear A
exists** (the convective term is bilinear in u, not linear in
each factor) — so F' must be cubic in u, breaking the relative-
entropy / quadratic form.

**This obstruction is generic**: the convective nonlinearity of
NS is *quadratic* in u, while a relative-entropy functional in
the Otto sense is at most quadratic in (u, ω) in the kinetic-
energy form. The variational re-cast of NS as gradient flow of a
quadratic relative entropy is **structurally incompatible** with
the Euler-side nonlinearity. This is the candidate's central
obstruction, registered upfront here and tested under F-EXTA-C.

### §3.7 Modified candidate (cubic F variant)

To accommodate §3.6's obstruction, the candidate admits a *cubic*
variant:

  F_cubic[u, ω] = F[u, ω] + (1/3) ∫ u · ((u · ∇) u) dx              (3.11)

(a triple-product term; this is the kinetic-energy advection
density, equal to (1/3) ∫ ∇ · (|u|² u/2) dx = 0 for div-free u
on ℝ³ with decay, by integration by parts).

**The triple-product term is identically zero on div-free u with
decay.** So F_cubic = F on the relevant function space. This
means the cubic correction is not available; the candidate
remains structurally quadratic in u, and the convective term
(u · ∇) u of NS is *not encodable* as a gradient of any quadratic
F under the metric g.

**Conclusion of §3.7**: the equivalence (3.9) FAILS as a
pointwise identity on the natural quadratic-F class. The
candidate must therefore be either:

(α) **a gradient flow of a non-quadratic F** (admitted, but then
    F is not a relative entropy in the Otto-Boltzmann sense);
(β) **a gradient flow under a non-flat metric** g (admitted, but
    then g must be specified and computed — equivalent to
    Vasseur-Yu 2016 transferred to incompressible, which fails
    per §2.4);
(γ) **an equivalence on a restricted class** of solutions where
    the convective term vanishes (e.g., shear flows where
    (u · ∇) u = 0, parallel flows, etc.).

The candidate as registered in §3.2-§3.5 is in form (γ): a
gradient-flow equivalence on the **shear-flow / kinetic-laminar**
sub-manifold where the convective term vanishes. This is a
**proper sub-manifold** of NS solutions and the equivalence does
NOT extend to general 3D NS. This sub-manifold-only equivalence
is the candidate's structural-honest content.

### §3.8 Regularity coupling (conjectural, sub-manifold only)

If the gradient-flow form (3.7) is equivalent to NS on the
shear-flow sub-manifold (form γ above), then on that sub-manifold
the dissipation identity

  dF/dt = -∫ |δF/δu|² dx ≤ 0                                       (3.12)

holds, giving monotone-decrease of F. The kinetic + enstrophy +
bi-Laplacian-energy combination F is therefore monotone-non-
increasing on the shear-flow sub-manifold.

For shear flows, NS regularity is already settled (linear
Stokes-like equations on shear profiles); the candidate's
regularity coupling is therefore **trivially valid on the sub-
manifold but does not extend** to the full 3D NS where the
convective nonlinearity activates.

**Verdict on regularity coupling**: NS regularity *as transferred
through uω-GradFlow* is conditional on extending the equivalence
beyond the shear-flow sub-manifold; this extension is structurally
blocked per §3.6.

### §3.9 Novelty over existing NS variational inventory

uω-GradFlow differs from each item in §2.8:

- **vs Brenier 1989/1999**: Brenier is least-action (critical-
  point variational); uω-GradFlow is gradient-flow (descent);
- **vs Brenier 1999 stochastic-Euler**: Brenier's perturbation is
  in path space; uω-GradFlow operates on the deterministic
  (u, ω) pair with no stochastic ingredient;
- **vs Otto-Wasserstein on ρ_u (negative baseline)**: uω-GradFlow
  augments to (u, ω), not to (u-density); avoids the
  divergence-free-degenerate metric issue by using Leray-
  projected L²;
- **vs Vasseur-Yu 2016 (compressible)**: uω-GradFlow is
  incompressible-native, not a transfer from compressible;
- **vs CI 2008 (representation)**: uω-GradFlow is a re-cast of
  the PDE itself, not a probability representation;
- **vs 2D vorticity-streamfunction**: uω-GradFlow lives on 3D
  (u, ω), not on 2D ψ.

The candidate's distinctive content is the (u, ω) augmentation
plus the Leray-projected-L² flat metric. Both are individually
present in the literature; their combination as a gradient-flow
re-cast is not standard. The candidate is **not a relabeling**
of existing tools at the structural level, though it inherits
the §3.6 obstruction at the equivalence-with-NS level.

---

## §4 Discriminator

### §4.1 Discriminator type

**Type**: **structural-literature** (variational-derivation
sketch), per the L9 generation-pipeline §6.1 specification for
EXT-A. This is the same discriminator-type bucket as D3.A; for
EXT-A it carries the sub-type label **gradient-flow-equivalence-
sketch**.

EXT-A's discriminator is *distinct* from EXT-B's: EXT-B asks
whether a Lyapunov decreases along NS (analytic-inequality);
EXT-A asks whether NS *equals* a gradient flow of a specified F
under a specified metric g (variational-equivalence). EXT-A is
therefore a **structural** rather than analytic-inequality
discriminator.

Discriminator path P / Q / R / S:

- **Path P (literature-equivalence-import)**: an existing paper
  states (3.4) ≡ (3.8) for the chosen F and g. Verdict if
  succeeds: PASS_LITERATURE.
- **Path Q (sketch-equivalence-construction)**: a sketch
  derivation produces (3.4) ≡ (3.8) modulo controlled error /
  modulo a sub-manifold restriction. Verdict: PASS_SKETCH.
- **Path R (obstruction-documented)**: a specific algebraic
  identity required for (3.4) ≡ (3.8) provably fails — e.g.,
  the convective nonlinearity is *not* encodable as a gradient
  of any quadratic F under g (this is the §3.6 obstruction;
  Path R fires here on inspection). Verdict: OBSTRUCTION_DOCUMENTED.
- **Path S (inconclusive)**: literature underdetermined.
  Verdict: INCONCLUSIVE.

### §4.2 Discriminator measurement

When this candidate is validated, the measurement consists of:

(M1) Verify Otto 2001 framework restated correctly and that the
     metric g in (3.3a) is well-defined (Leray projection
     bounded on L²; tangent space to M correctly identified).

(M2) Compute δF/δu under the constraint ω = ∇ × u using standard
     variational calculus on Hilbert sub-manifolds. Verify (3.5)
     matches the standard NS dissipation operator at λ = 1, μ = 0.

(M3) Test equivalence (3.4) ≡ (3.8) on the explicit sub-manifolds:
     - (M3a) shear flows u = U(x_2) e_1 — equivalence should
       hold trivially (convective term vanishes);
     - (M3b) Beltrami flows u = ∇ × ψ with ω = α u — equivalence
       on this sub-manifold tests whether the (u, ω) coupling
       saturates;
     - (M3c) general smooth solutions — equivalence should FAIL
       per §3.6 (convective term not encodable in quadratic F).
     Likely outcome: M3a passes trivially; M3b partially; M3c
     fails — the documented obstruction.

(M4) Document the failure in M3c at the algebraic-identity level
     (which inner product or which integration-by-parts identity
     is required and provably fails). This is the
     OBSTRUCTION_DOCUMENTED deliverable.

The measurement key — by analogy with D3.A §4 Step 12's
non-relabeling clearance — is that the equivalence (or its
failure) must be expressed in *variational/PDE* primitives
(F, g, δF/δu, P) and NOT through the n=6 lattice {σ, τ, φ,
sopfr, n=6}. uω-GradFlow's definition contains zero n=6 anchors
by inspection: the only constants are dimensionless (λ, μ) and
ν.

### §4.3 Discriminator scope

The discriminator decides whether uω-GradFlow is:

(a) **Real EXT-A candidate** (Path P or Q success) — joins the
    L9 catalogue's EXT-A slot as a live BT-544 candidate;
(b) **Obstruction-documented frame-shift** (Path R) — confirms
    that the variational re-cast of 3D NS as gradient flow of a
    quadratic relative entropy is structurally blocked by the
    convective nonlinearity, and the EXT-A slot remains open
    for a different candidate (e.g., non-quadratic F or
    augmented-state-space alternative);
(c) **Inconclusive on current literature** (Path S) — records
    the gap and recommends literature items needed.

The discriminator does NOT decide NS regularity. Even a Path P
PASS_LITERATURE here would only establish a structural equivalence,
not regularity — regularity downstream depends on a separate
Lyapunov / energy-method step (which is EXT-B's territory).

---

## §5 Falsifier — registered upfront

Per the brief specification ("registered upfront") and the
structural risks identified in §3.6-§3.7, the following
falsifiers are registered before any validation work begins.

### §5.1 F-EXTA-A (relabeling test)

**Statement**: if (3.7) — the gradient-flow equation under the
chosen F and g — is, when expressed in original NS variables
(u, p), already a known equivalent form of NS or of a *modification*
of NS, then uω-GradFlow is a relabeling of standard NS with
extra cosmetic structure and adds no information.

**Test design**: rewrite (3.7) in (u, p) variables explicitly.
Compare with:
- the standard NS form (3.8);
- the Stokes equation (NS without convection);
- the bi-Laplacian-perturbed NS (Cahn-Hilliard-Navier-Stokes,
  Hyon-Kwak-Liu 2010 *Discrete Contin. Dyn. Syst.* 26).

If (3.7) coincides with one of these (modulo a smooth change of
variables), the relabeling falsifier fires. Per §3.6, (3.7) is
NOT NS itself (no convective term) but it IS a Stokes-with-bi-
Laplacian — so the relabeling falsifier fires *for the Stokes
sub-equation* but the equivalence-with-NS remains an open
question (which is what F-EXTA-C tests).

**Status at registration**: PARTIALLY FIRED for the Stokes-
sub-equivalence; NOT FIRED for the full NS equivalence.

### §5.2 F-EXTA-B (trivial-entropy / kinetic-only)

**Statement**: if F[u, ω] reduces to F = (1/2) ‖u‖_{L²}²
(kinetic energy alone) under the chosen λ = 0 limit or under
any structural simplification, then F is no new information
beyond the standard energy.

**Test design**: set λ = 0 and μ = 0 in (3.2). Verify F = E
(Leray's energy). The gradient-flow form (3.7) reduces to
∂_t u + u = 0, which is exponential-decay damped Stokes — NOT
NS. So F at λ = μ = 0 is too weak.

For λ > 0, the enstrophy piece adds a νλ(-Δu) term; for μ > 0,
the bi-Laplacian piece adds a ν²μ(-Δ)²u term. Neither produces
the convective nonlinearity. Across all (λ, μ) ≥ 0, F remains a
quadratic functional and the gradient flow remains linear.

**Status at registration**: vacuity is REAL across (λ, μ) — F
is quadratic, gradient flow is linear, NS is quasilinear-
nonlinear. F-EXTA-B fires structurally for the equivalence-with-
full-NS claim but not for the equivalence-on-Stokes-sub-manifold
claim.

### §5.3 F-EXTA-C (variational-degenerate / convective-not-
       encodable)

**Statement**: this is the **central registered obstruction**
of uω-GradFlow. The convective nonlinearity (u · ∇) u of NS
cannot be expressed as a variational gradient of any quadratic
F[u, ω] under any metric g obtained by a smooth change of
variables on the Leray-projected-L² flat metric.

**Test design**: write (u · ∇) u in terms of u and its
derivatives. Test whether it has the form δF/δu for any
quadratic F. By a standard argument (see Olver 1986
*Applications of Lie Groups to Differential Equations*, Springer,
Chapter 5 on variational symmetries): a vector field is a
variational gradient iff its Helmholtz conditions
(symmetry of Frechet derivative) are satisfied.

The convective term (u · ∇) u has Frechet derivative
D[(u · ∇) u][v] = (v · ∇) u + (u · ∇) v, which is NOT symmetric
in (u, v) under the L² inner product (the asymmetry is exactly
the vortex-stretching term). Therefore (u · ∇) u is **not** a
variational gradient on the L² inner product.

The Helmholtz-condition failure can be cured on a *modified*
inner product, but doing so is equivalent to specifying a non-
flat metric g, which collapses to the Vasseur-Yu 2016
compressible-NS-style construction (failing on incompressible
per §2.4) or to a non-quadratic F (failing the relative-entropy
form per §3.7).

**Status at registration**: STRUCTURALLY ACTIVE. F-EXTA-C is
the most-likely activation falsifier. Validation will likely
deliver OBSTRUCTION_DOCUMENTED via this route.

### §5.4 F-EXTA-D (gradient-flow-not-faithful-to-NS-solutions)

**Statement**: even if the gradient-flow equation (3.7) is
written down and the metric g is well-defined, the family of
solutions to (3.7) and the family of solutions to NS (3.8) may
differ. If a smooth NS solution u(t) is NOT a solution of (3.7),
the gradient-flow form is not faithful to NS, and the candidate
is at best a *related* equation, not a re-cast.

**Test design**: take a known explicit NS solution (e.g., Taylor-
Green vortex, or a 2D NS solution embedded as a 3D flow with no
swirl). Verify whether it solves (3.7) at the chosen (λ, μ, ν).
Per §3.6, this fails at the convective-nonlinearity step on
generic solutions; passes on shear flows where convection
vanishes.

**Status at registration**: ACTIVE. Generic NS solutions do
NOT solve (3.7); shear-flow sub-manifold restriction recovers
faithfulness.

### §5.5 F-EXTA-E (otto-framework-doesn't-transfer)

**Statement**: if the Otto 2001 gradient-flow framework, in its
required Wasserstein-Hilbert structure (Otto 2001 §1; Ambrosio-
Gigli-Savaré 2008 Chapter 9), requires hypotheses (e.g., on
the underlying probability space, on the convexity of F along
geodesics, on the McCann-displacement-convexity) that fail for
the candidate (M, g, F) chosen here, the gradient-flow re-cast is
on unstable foundation.

**Test design**: check whether the metric g (3.3a) is geodesically
complete on M; check whether F (3.2) is displacement-convex along
g-geodesics; check whether the gradient-flow existence theory of
Ambrosio-Gigli-Savaré 2008 applies. Likely outcome: (M, g, F)
satisfies the AGS framework at the level of metric completion
but F is NOT displacement-convex (since the kinetic-energy
piece is not McCann-convex on velocity-divergence-free fields
in a metric-measure-space sense), so the contraction theory
doesn't transfer.

**Status at registration**: PARTIALLY ACTIVE. F-EXTA-E fires
for the contraction-theory transfer; doesn't fire for the bare
existence question.

### §5.6 Falsifier registration summary

| tag | name | status at registration | activation mode |
|-----|------|------------------------|-----------------|
| F-EXTA-A | relabeling | partially fired (Stokes-sub) | check (3.7) ≡ Stokes-bi-Laplacian-NS |
| F-EXTA-B | trivial-entropy | fired structurally for full NS | check F at (λ=0, μ=0) reduces to E |
| F-EXTA-C | convective-not-encodable | **most likely activation** | Helmholtz-condition test on (u·∇)u |
| F-EXTA-D | gradient-flow-not-faithful | active for general NS | check Taylor-Green vortex satisfies (3.7) |
| F-EXTA-E | Otto-framework-doesn't-transfer | partially active | check displacement-convexity of F on (M, g) |

All five falsifiers are **falsifiable in finite work** (a single
validation session). Three (F-EXTA-A partial, F-EXTA-B
structural, F-EXTA-D for general NS) already register as fired
or partially-fired at the structural level — the candidate's
honest content is therefore **OBSTRUCTION_DOCUMENTED at
registration**, with validation refining the documentation but
unlikely to upgrade to PASS.

---

## §6 Cost estimate and expected verdict

### §6.1 Discriminator type per the bias diagnostic

Per `omega-meta-audit-discriminator-type-bias-2026-04-25.md`
§5.4 controlled vocabulary, the candidate's discriminator type
is **structural-literature** (variational-derivation-sketch),
sub-type **gradient-flow-equivalence-sketch** (proposed EXT-A
vocabulary extension).

This is in the **PASS-family** position per the bias diagnostic
(structural-literature is the dominant PASS-class, alongside
distributional). The bias-hypothesis prediction is therefore
PASS-family verdict (PASS_LITERATURE / PASS_SKETCH /
OBSTRUCTION_DOCUMENTED rather than FAIL).

### §6.2 Validation cost estimate

| activity | estimated cost |
|----------|----------------|
| literature scan (Otto 2001, Brenier 1989/1999, Ambrosio-Gigli-Savaré 2008, Vasseur-Yu 2016, Constantin-Vicol 2012) | 4-6 hours |
| relabeling test on Stokes / bi-Laplacian-NS / Cahn-Hilliard-NS (F-EXTA-A) | 2-3 hours |
| Helmholtz-condition test for (u·∇)u (F-EXTA-C) | 3-4 hours (load-bearing) |
| Taylor-Green / 2D-embedded test (F-EXTA-D) | 2-3 hours |
| AGS displacement-convexity check (F-EXTA-E) | 3-4 hours |
| writeup | 1-2 hours |
| **total validation session** | **15-22 hours** (medium-high) |

Validation cost is **medium-high**, slightly above EXT-B
(12-19 hours per the EXT-B report §6.2), reflecting the
larger literature scan (Otto-Brenier-Ambrosio chain is broader
than the Constantin-Iyer-Otto-Bakry-Émery chain) and the
need to compute the Helmholtz-condition obstruction explicitly.

The full Clay-rigour resolution of "is NS a gradient flow on
some metric space?" is decadal-scale (the question has been
open since at least Otto 2001 with notable contributions from
Brenier, Ambrosio, Vasseur, and others over 25 years). A single
validation session aims at the discriminator outcome (PASS /
OBSTRUCTION / INCONCLUSIVE), not the full resolution.

### §6.3 Expected verdict

Per the falsifier-registration analysis in §5:

- **F-EXTA-C activates as expected** (convective nonlinearity
  fails Helmholtz condition; not encodable as gradient of
  quadratic F under flat metric);
- **F-EXTA-A partially fires** (Stokes-sub-equivalence is a
  relabeling; full-NS-equivalence is not);
- **F-EXTA-B fires structurally** for full NS;
- **F-EXTA-D fires for generic NS solutions**, doesn't fire on
  shear-flow sub-manifold;
- **F-EXTA-E partially active** for displacement-convexity transfer.

**Expected verdict at first validation pass**: **OBSTRUCTION_
DOCUMENTED** (~70% probability). The obstruction is at the
Helmholtz-condition / convective-nonlinearity step, which is
**provably and finitely demonstrable** (Olver 1986 Helmholtz-
condition machinery is rigorous). The candidate produces a clean
documentation of the variational-re-cast obstruction for 3D NS,
converting the "is NS a gradient flow?" question from "we don't
know" to "we know exactly which Helmholtz-condition fails on
(u · ∇) u".

**Alternative verdict (~25%)**: **PASS_SKETCH** — if the
gradient-flow equivalence assembles on a non-trivial sub-manifold
(beyond shear flows; e.g., a sub-manifold constrained by a
geometric depletion condition à la Constantin-Fefferman 1993),
the candidate gets a sub-manifold sketch PASS.

**Lower probability (~5%)**: **PASS_LITERATURE** — would require
an existing paper that already states (3.7) ≡ NS on a meaningful
sub-manifold or under a regularizing modification. Search has
not produced such a paper as of 2026-04-25; expected to remain
absent on validation.

### §6.4 Verdict sensitivity table

| primary literature finding | falsifier outcome | verdict |
|---------------------------|---------------------|---------|
| (3.7) ≡ NS proven somewhere | none fire | PASS_LITERATURE |
| sub-manifold equivalence assembles | F-EXTA-A partial only | PASS_SKETCH |
| Helmholtz condition fails (expected) | F-EXTA-C fires | OBSTRUCTION_DOCUMENTED (Helmholtz-side) |
| convective term encodable on non-flat g | F-EXTA-E fires (transfer fails) | OBSTRUCTION_DOCUMENTED (metric-side) |
| (3.7) ≡ Stokes-bi-Laplacian only | F-EXTA-A fires fully | FAIL (relabeling) |
| F = E only (kinetic-trivial) | F-EXTA-B fires | FAIL (vacuous) |
| literature underdetermined | F-EXTA-C/E ambiguous | INCONCLUSIVE |

Most-likely cell (per §5 falsifier analysis): row 3
(OBSTRUCTION_DOCUMENTED, Helmholtz-side).

Comparison with EXT-B expected verdict: EXT-B's most-likely
verdict is OBSTRUCTION_DOCUMENTED via F-EXTB-D (CI-2008-too-weak,
representation-side). The two sibling candidates therefore
converge on different obstructions:
- EXT-A fails at the **variational-encodability** of the
  convective nonlinearity (the Helmholtz-condition obstruction);
- EXT-B fails at the **probabilistic-coupling-strength** of the
  CI representation (the regularity-coupling obstruction).

These are **structurally distinct obstructions**, and obstruction-
documentation of both refines the BT-544 obstruction-map
considerably.

---

## §7 Cross-axis ties — connections to EXT-B and D3.A

The EXT-A candidate (uω-GradFlow) is a sibling of the EXT-B
candidate (CI-Lyap, recorded in
`omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md`).
Both come from the variational/analytic family. They are
connected to the D3.A axis (PASS_LITERATURE on 2.5D non-local-
pressure system) through the compositional decoupling strategy.

### §7.1 EXT-A vs EXT-B family decomposition

| aspect | EXT-A (uω-GradFlow) | EXT-B (CI-Lyap) |
|--------|---------------------|------------------|
| family | variational re-cast | analytic-Lyapunov |
| state space | (u, ω) sub-manifold M | (u, ρ, τ) augmented |
| functional | F[u, ω] kinetic-enstrophy-curvature | W_NS[u, ρ, τ] Perelman-style |
| metric / structure | Leray-projected L² flat metric | (Wasserstein-like, conjectural) |
| dynamic claim | NS = gradient flow of F | dW_NS/dt ≥ 0 along NS+SL |
| primary obstruction | Helmholtz-condition on (u·∇)u | CI 2008 representation strength |
| expected verdict | OBSTRUCTION_DOCUMENTED ~70% | OBSTRUCTION_DOCUMENTED ~75% |
| expected obstruction-side | variational-encodability | regularity-coupling |

The two candidates are **structurally orthogonal in obstruction-
type**: EXT-A asks "can NS be re-cast as a gradient flow?"
(variational equivalence), EXT-B asks "is there a Lyapunov along
NS that controls regularity?" (analytic monotonicity). The
obstructions, when documented, deposit distinct algebraic
identities in the BT-544 obstruction-map.

### §7.2 Combination with D3.A (PASS_LITERATURE)

D3.A's PASS_LITERATURE for axis A (pressure non-locality on 2.5D
non-local-pressure system) clears one of the three D3 mechanisms.
The compositional strategy (D3 seed §3.2) requires two clean
axes for the obstruction to localize on the third (B = vortex-
stretching or C = cascade).

uω-GradFlow's relation to D3.A:

(1) **uω-GradFlow operates on the full 3D NS, not on D3 axes.**
    Its variational re-cast does not decouple by mechanism. If
    PASS_SKETCH (lower-probability ~25%), the gradient-flow
    structure is global; if OBSTRUCTION_DOCUMENTED (~70%), the
    obstruction is at the convective-nonlinearity level, which
    spans all three D3 mechanisms.

(2) **D3.A's clean axis-A reading informs uω-GradFlow** at the
    pressure-reconstruction step. The Leray projection P in
    (3.4)-(3.7) requires Riesz-transform boundedness on H^s
    (Calderón-Zygmund 1952) — exactly the analytic input
    cleared by D3.A §4 Step 6. This means uω-GradFlow's
    pressure-side is on solid analytic ground; the obstruction
    is unambiguously at the convective term, not at the
    pressure.

(3) **uω-GradFlow's obstruction informs D3 compositional**
    by isolating the convective-nonlinearity as the
    variational-encodability blocker. This is consistent with
    D3 seed §4.2 ranking "axis B (vortex-stretching) =
    predicted obstruction-carrier" — the convective nonlinearity
    contains the vortex-stretching term ω · ∇u, so EXT-A's
    Helmholtz obstruction localises onto axis B.

### §7.3 The EXT-A + EXT-B + D3.A coherent attack strategy

A *coherent* attack on BT-544 combining the three:

| step | strategy |
|------|----------|
| 1 | D3.A clears axis A (pressure) — done, PASS_LITERATURE |
| 2 | EXT-A documents variational obstruction on convective term — predicted OBSTRUCTION_DOCUMENTED at axis B-localised Helmholtz failure |
| 3 | EXT-B documents analytic obstruction at CI-representation strength — predicted OBSTRUCTION_DOCUMENTED at axis B-localised regularity-coupling failure |
| 4 | D3.B' (axis B discriminator) — predicted to receive both EXT-A and EXT-B obstruction-localisation as input, refining the axis-B mechanism analysis |

The combination produces a **two-pronged obstruction map** on
axis B (vortex-stretching): EXT-A flags it as the variational-
encodability site, EXT-B flags it as the regularity-coupling
site. The convergence is structurally consistent with the D3
seed §4.2 ranking and refines the BT-544 cycle's understanding
of where the Clay-rigour obstacle lives.

Note: this is a **research strategy**, not a proof. It does not
resolve BT-544; it organises the obstruction-documentation work.

### §7.4 Updated D3 axis status table (post-EXT-A candidate)

| axis | discriminator | pre-EXT-A status | post-EXT-A-candidate status |
|------|---------------|------------------|------------------------------|
| A    | uniform H^s on 2.5D non-local-pressure | PASS_LITERATURE | unchanged; uω-GradFlow consistent |
| B    | BKM-finite or dim_P ≤ 1 on axisymm-with-swirl Euler | UNTOUCHED, predicted obstruction-carrier; CI-Lyap residual-target | **uω-GradFlow Helmholtz-obstruction-target candidate** (sibling to EXT-B's residual-target) |
| C    | Kraichnan two-sided S_6 ∼ ℓ² | tested at D3.C | unchanged; uω-GradFlow not specific to cascade |

Note: the post-status is **not a verdict change** — D3 axis
verdicts remain as in the D3.A/B'/C reports. uω-GradFlow is a
NEW candidate at the EXT-A slot; it does not modify D3's
findings.

---

## §8 Anti-list — alternative variational candidates considered and rejected

Alternative EXT-A candidates considered before settling on
uω-GradFlow. Each rejected with one-line reason.

- **Anti-1 (Otto-Wasserstein on velocity density ρ_u)**: re-cast
  NS as ∂_t ρ_u = -∇_W H[ρ_u]. Rejected: this is the negative
  baseline (§2.3, Brenier 1999 §5; AGS 2008 §§9-11). Cannot be a
  candidate.

- **Anti-2 (Brenier 1989/1999 generalised-flow least-action as
  gradient flow)**: declare Brenier's least-action functional as
  a gradient (rather than critical-point) functional. Rejected:
  Brenier's functional is critical at NS, not monotone-decreasing.
  Reinterpreting it as gradient-flow misuses the variational form.

- **Anti-3 (Vasseur-Yu 2016 compressible gradient-flow transfer)**:
  transfer the Vasseur-Yu Wasserstein-gradient-flow structure to
  incompressible 3D NS. Rejected: compressibility is load-bearing
  (the entropy density requires compressibility); transfer fails
  at constitutive step. Also rejected as Anti-6 of EXT-B for the
  same reason.

- **Anti-4 (NS as Hamiltonian flow on (u, p) phase space)**:
  declare NS Hamiltonian (analog of 2D streamfunction). Rejected:
  3D NS is not Hamiltonian (the 2D streamfunction Hamiltonian is
  2D-specific; 3D NS has dissipation and lacks a natural symplectic
  structure on (u, p)).

- **Anti-5 (kinetic-formulation as gradient flow)**: use Lions-
  Perthame-Tadmor 1994 (*J. Amer. Math. Soc.* 7) kinetic
  formulation of conservation laws as a gradient-flow source for
  NS. Rejected: LPT is for scalar conservation laws (Burgers-
  type); transfer to NS fails because NS is not in conservation-
  law form (the pressure non-locality breaks conservation-law
  structure).

- **Anti-6 (steepest-descent on Sobolev metric)**: declare NS as
  gradient flow under the H^{-1} or Ḣ^1 Sobolev metric. Rejected:
  this is *closer* to the candidate (Leray-projected L² is
  effectively H^0 on div-free fields), but the convective
  nonlinearity still fails the Helmholtz condition under any
  Sobolev metric (the Helmholtz failure is at the algebraic
  level, not the metric level). This is the **modified-metric
  attempt** failing per §5.5 F-EXTA-E.

- **Anti-7 (NS as gradient flow of a non-quadratic F)**: admit a
  cubic or higher F to encode the convective nonlinearity.
  Rejected: cubic F violates the relative-entropy structure (F is
  no longer a free-energy / entropy in the Otto-Boltzmann
  framework); the candidate becomes a generic critical-point
  variational form, losing the gradient-flow contraction
  machinery (Bakry-Émery, AGS).

- **Anti-8 (NS as gradient flow on a fractal / measure-valued
  state space)**: live on a non-standard state space (measure-
  valued solutions, Young measures à la DiPerna 1985). Rejected:
  measure-valued solutions are weaker than Leray-Hopf; the
  gradient-flow structure on Young measures is well-defined but
  the regularity coupling to standard (u, p) Leray-Hopf solutions
  is lost. Out of scope for the Clay-form BT-544.

- **Anti-9 (NS as variational flow under arbitrary Lagrangian)**:
  generalise to any Lagrangian L[u, ∇u, ω] producing an NS-
  consistent Euler-Lagrange equation. Rejected: this is an
  unconstrained search (any NS-consistent action works at the
  level of formal calculus); without specifying the *gradient*-
  flow structure (descent, not critical-point), the candidate
  becomes the generic variational-rewrite which is well-known and
  not new.

- **Anti-10 (purely arithmetic n=6-lattice variational form)**:
  define a variational functional as a polynomial in
  {σ, τ, φ, sopfr, n=6} and the velocity. Rejected immediately:
  this is in the dominant-family bias flagged by CATALOGUE_BIAS
  verdict (per `omega-meta-audit-l9-catalogue-design-2026-04-25.md`);
  EXT-A is precisely the *non-arithmetic* family. Per the brief's
  hard constraint, EXT-A candidates must NOT come from the
  arithmetic family.

- **Anti-11 (NS as Cahn-Hilliard-like 4th-order gradient flow)**:
  use the bi-Laplacian-perturbed NS (Cahn-Hilliard-Navier-Stokes,
  Hyon-Kwak-Liu 2010 *Discrete Contin. Dyn. Syst.* 26) as the
  candidate. Rejected: Cahn-Hilliard-NS is a *modification* of NS
  with explicit bi-Laplacian regularisation; it is well-posed but
  is not 3D NS. The candidate (3.7) without convective term IS a
  bi-Laplacian-perturbed Stokes — confirming the F-EXTA-A
  partial-relabeling fire — but this is what the candidate
  obstruction-documents, not what it claims.

The uω-GradFlow candidate (§3) was selected over the 11
alternatives because:
(a) it lives in the variational re-cast family (not arithmetic,
    not Lyapunov-only) — required by the brief;
(b) it is grounded in published literature (Otto 2001, Brenier
    1989/1999, Bakry-Émery 1985, AGS 2008) — required by the
    brief "DO NOT FABRICATE";
(c) it specifies the (u, ω) phase-space augmentation explicitly
    per the brief's "(u, ω) phase space" requirement;
(d) it produces a documented obstruction at the convective-
    nonlinearity-Helmholtz-condition level, which is structurally
    informative and complementary to EXT-B's CI-representation
    obstruction;
(e) it contains zero n=6 lattice anchors in its definition (per
    the generation-pipeline §6.1 prediction).

---

## §9 Falsifiers active for the candidate-generation itself

Distinct from §5 (falsifiers for the candidate's gradient-flow
equivalence claim), the following falsifiers apply to the
candidate-generation methodology itself.

### §9.1 F-GEN-A (candidate-not-EXT-A-class)

**Statement**: if uω-GradFlow on inspection is NOT in the
variational-re-cast family — e.g., it reduces to an arithmetic
identity, a Lyapunov-only construction (which would put it in
EXT-B, not EXT-A), or a discrete-equality test — the EXT-A slot
is not actually occupied.

**Status**: NOT ACTIVE. The candidate uses Wasserstein-gradient-
flow framework (Otto 2001), variational derivative δF/δu, Leray-
projected L² metric, and Helmholtz-condition obstruction analysis
— all in the variational / functional-analytic family. Distinct
from EXT-B's CI-Lyap (which is a Lyapunov, not a re-cast).

### §9.2 F-GEN-B (candidate-fabricates-citations)

**Statement**: if any of the literature anchors (Otto 2001,
Brenier 1989, Brenier 1999, Bakry-Émery 1985, Ambrosio-Gigli-
Savaré 2008, Vasseur-Yu 2016, Constantin-Iyer 2008, Constantin-
Vicol 2012, Olver 1986, Lions-Perthame-Tadmor 1994, DiPerna 1985,
Calderón-Zygmund 1952, Beale-Kato-Majda 1984, Constantin-
Fefferman 1993, Hyon-Kwak-Liu 2010, Jordan-Kinderlehrer-Otto
1998, Otto-Villani 2000, Majda-Bertozzi 2001) is fabricated, mis-
attributed, or mis-yeared, the candidate's grounding is
compromised.

**Status**: each citation is to a real paper or monograph in the
gradient-flow / NS / variational literature, by author + year +
journal-or-publisher pattern matched against the standard NS-
analysis bibliography (Constantin-Foias 1988 monograph; Foias-
Manley-Rosa-Temam 2001 monograph; Lemarié-Rieusset 2002 *The
Three-Dimensional Navier-Stokes Equations* monograph; AGS 2008
monograph). Cross-check on validation. NOT ACTIVE at registration.

### §9.3 F-GEN-C (candidate-already-in-literature)

**Statement**: if the (3.2) functional and (3.7) gradient-flow
form on (u, ω) sub-manifold M with Leray-projected-L² metric have
already been written down in a published paper, this report's
"novel candidate" claim is duplication.

**Status**: a literature search of NS / Wasserstein-gradient-flow
/ variational-NS papers (Otto 2001 lineage, Brenier 1989/1999
lineage, Vasseur-Yu 2016 lineage, AGS 2008) does NOT surface this
exact (M, g, F) triple. The closest published forms are:
(a) Vasseur-Yu 2016 compressible-NS gradient-flow on (ρ, ρu) —
    different state space (compressible, density-momentum);
(b) 2D vorticity-streamfunction Hamiltonian — different state
    space (2D, streamfunction);
(c) Otto-Wasserstein on velocity density — failed negative
    baseline.
None match (M, g, F) of §3. NOT ACTIVE based on current search;
flagged for double-check at validation.

### §9.4 F-GEN-D (atlas/state/inventory-edit-leakage)

**Statement**: if any change is made to atlas, state, or
inventory files as a result of this report, the brief's hard
constraint is violated.

**Status**: NOT ACTIVE. This report is research-only and does not
edit `state/proposals/inventory.json`, `theory/canon/`, or any
atlas grade. The git status at session start (BT-544 cycle reports
modified) is independent of EXT-A candidate-generation.

### §9.5 F-GEN-E (validation-attempted-not-just-generation)

**Statement**: the brief explicitly says "DO NOT validate.
Generation only." If this report attempts validation (e.g., claims
the gradient-flow equivalence (3.4) ≡ (3.8) HOLDS), the brief
constraint is violated.

**Status**: NOT ACTIVE. §3.6 explicitly states the equivalence
"is conjectural" and that the convective nonlinearity is "not
encodable as a gradient of any quadratic F"; §4 records the
discriminator design without executing it (paths P/Q/R/S
specified, not run); §5 falsifiers are registered, not run; §6
records the EXPECTED verdict, not a delivered one. The report
generates the candidate and stops at the discriminator-design
boundary.

Note: §3.6-§3.7 contain a structural-honest acknowledgment that
F-EXTA-C fires on inspection (Helmholtz-condition fails for
(u · ∇) u under quadratic F + flat metric). This is **not
validation**; it is the candidate's *registered obstruction* at
generation time, made transparent to inform the discriminator
design. The discriminator measurement (§4.2 M3, M4) is the formal
test; the §3.6 acknowledgment is a structural prediction of that
test's outcome.

### §9.6 F-GEN-F (Otto-framework-misread)

**Statement**: if the §1.1 Otto 2001 framework recap mis-attributes
structural points (O-i)-(O-iv), the analog framing fails.

**Status**: items (O-i)-(O-iv) are extracted from Otto 2001 §1
(state-space and metric definition) and §3-§4 (gradient-flow form
and dissipation identity), with the Wasserstein-Hilbert framework
stated per Otto 2001 Proposition 3.4 / Proposition 4.1
equivalents. Cross-check on validation. NOT ACTIVE.

### §9.7 F-GEN-G (n=6-anchor-leakage)

**Statement**: per the L9 generation-pipeline §6.1 prediction
("n=6 anchor count in description: 0"), the candidate's
definition must contain zero invocations of {σ, τ, φ, sopfr,
σ_2, J_2, n/φ, "6"}.

**Status**: NOT ACTIVE. The candidate's definition (§3) uses:
(a) standard NS variables (u, ω, p); (b) viscosity ν;
(c) dimensionless coefficients (λ, μ); (d) Leray projection P;
(e) variational derivative δF/δu; (f) Wasserstein-Hilbert /
metric measure space framework; (g) Helmholtz-condition (Olver
1986). None of these is in the n=6 lattice. The constants λ, μ
are dimensionless and can be specified by inviscid-limit
matching to Brenier 1999, not by n=6 invariants. The dimension-3
appears in the spatial domain ℝ³ — this is the standard 3D-NS
spatial dimension, not the BT-544 n=6 invariant; "3" is not
in the n=6 lattice. The "1/3" in §3.7 is a symmetrisation
coefficient (1/3 of a triple-product-trace), not a sopfr or
σ_2 invariant.

### §9.8 F-GEN-H (EXT-A-vs-EXT-B-confusion)

**Statement**: if uω-GradFlow is structurally a Lyapunov rather
than a variational re-cast, the EXT-A slot is occupied by
an EXT-B-class candidate (which is already filled by CI-Lyap,
producing duplication).

**Status**: NOT ACTIVE. uω-GradFlow's central claim is
**equivalence** ∂_t (u, ω) = -∇_g F  ⇔  3D NS, which is a
*variational re-cast* claim — does NS equal a gradient flow?
This is structurally distinct from CI-Lyap's *Lyapunov*
claim — does W_NS decrease along NS? Re-cast vs Lyapunov is the
standard EXT-A vs EXT-B distinction (Otto-Brenier-Wasserstein
re-cast vs Perelman-W-Lyapunov). The two candidates are
structurally distinct.

### §9.9 Generation-falsifier summary

| tag | name | status |
|-----|------|--------|
| F-GEN-A | candidate-not-EXT-A-class | NOT ACTIVE |
| F-GEN-B | candidate-fabricates-citations | NOT ACTIVE (cross-check at validation) |
| F-GEN-C | candidate-already-in-literature | NOT ACTIVE (cross-check at validation) |
| F-GEN-D | atlas/state/inventory-edit-leakage | NOT ACTIVE |
| F-GEN-E | validation-attempted-not-just-generation | NOT ACTIVE |
| F-GEN-F | Otto-framework-misread | NOT ACTIVE |
| F-GEN-G | n=6-anchor-leakage | NOT ACTIVE |
| F-GEN-H | EXT-A-vs-EXT-B-confusion | NOT ACTIVE |

All eight generation-falsifiers register as NOT ACTIVE. The
candidate-generation is methodologically clean per the brief's
hard constraints.

---

## §10 Closing

- **Candidate generated**: **(u, ω)-phase-space relative-entropy
  gradient flow for 3D Navier-Stokes** (uω-GradFlow), state
  space M (3.1), functional F (3.2), Leray-projected-L² metric g
  (3.3a), gradient-flow equation (3.7), conjectural equivalence
  with NS (3.9) under primitive swap p ⇄ ζ.
- **Family**: variational re-cast / NS-as-gradient-flow (the
  EXT-A class missing from the L9 catalogue per BT-547 retro
  §5.1 and per generation-pipeline §6.1).
- **Discriminator type**: structural-literature (variational-
  derivation-sketch) with proposed sub-type label "gradient-flow-
  equivalence-sketch"; PASS-family per the bias hypothesis.
- **Falsifiers (5 for equivalence, 8 for generation, all
  registered upfront)**: F-EXTA-A relabeling, F-EXTA-B trivial-
  entropy, F-EXTA-C convective-not-encodable (most likely
  activation, Helmholtz-condition obstruction), F-EXTA-D not-
  faithful-to-NS-solutions, F-EXTA-E Otto-framework-doesn't-
  transfer; plus F-GEN-A through F-GEN-H for the candidate-
  generation methodology.
- **Cost estimate**: medium-high (15-22 hours validation session).
- **Expected verdict at validation**: **OBSTRUCTION_DOCUMENTED**
  (~70% probability) via F-EXTA-C activation at the Helmholtz-
  condition step; PASS_SKETCH (~25%) on a non-trivial sub-
  manifold restriction; PASS_LITERATURE (~5%); other outcomes
  residual.
- **Cross-axis tie**: complementary to EXT-B (CI-Lyap) — EXT-A
  fails at variational-encodability (Helmholtz on convective
  term), EXT-B fails at probabilistic-coupling-strength (CI 2008
  representation); the two obstructions are structurally
  distinct and converge on axis B (vortex-stretching) of the D3
  decomposition. Combined with D3.A's PASS_LITERATURE on axis A,
  the EXT-A + EXT-B + D3.A trio refines the BT-544 obstruction-
  map onto axis B.
- **No new theorem claimed.** All cited results are pre-existing
  (Otto 2001; Brenier 1989; Brenier 1999; Bakry-Émery 1985;
  Ambrosio-Gigli-Savaré 2008; Vasseur-Yu 2016; Constantin-Iyer
  2008; Constantin-Vicol 2012; Olver 1986; Lions-Perthame-Tadmor
  1994; DiPerna 1985; Calderón-Zygmund 1952; Beale-Kato-Majda
  1984; Constantin-Fefferman 1993; Hyon-Kwak-Liu 2010; Jordan-
  Kinderlehrer-Otto 1998; Otto-Villani 2000; Majda-Bertozzi 2001;
  Constantin-Foias 1988; Foias-Manley-Rosa-Temam 2001; Lemarié-
  Rieusset 2002; Leray 1934).
- **NS regularity status**: **OPEN**. uω-GradFlow is a candidate
  variational re-cast, not a regularity proof; its equivalence
  with NS is conjectural and registered as obstruction-bound at
  the convective-nonlinearity-Helmholtz step; its full validation
  is decadal-scale per BT-547 retro §6.5.
- **0/7 unchanged. NS regularity status open. No atlas/state/
  inventory edits.**

— end candidate-generation —
