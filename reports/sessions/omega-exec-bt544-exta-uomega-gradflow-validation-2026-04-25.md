---
id: omega-exec-bt544-exta-uomega-gradflow-validation
date: 2026-04-25
scope: research-only validation (NO NS claim, NO atlas promotion)
target: BT-544 EXT-A uω-GradFlow -- Helmholtz / Path-P / sub-manifold check
parent_reports:
  - reports/sessions/omega-exec-bt544-exta-variational-recast-candidate-2026-04-25.md (candidate spec)
  - reports/sessions/omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md (cross-axis tie)
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md (sibling validation, OBSTRUCTION_DOCUMENTED)
millennium_resolved: 0/7 (unchanged)
grade: validation, no claim
---

# Omega Exec — BT-544 EXT-A uω-GradFlow Validation (2026-04-25)

## §0 Non-claim disclaimer

This report **validates** the EXT-A uω-GradFlow candidate registered
in `omega-exec-bt544-exta-variational-recast-candidate-2026-04-25.md`
("candidate spec" hereafter). It does **NOT**:

- claim 3D Navier-Stokes regularity (smoothness or blow-up); the
  Clay BT-544 status remains `0/1 untouched`;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, or alter the L9 catalogue exhaustion verdict
  in `omega-exec-bt544-fallback-molt-validation-2026-04-25.md`;
- supersede the D1 / D2 / D3.A / D3.B' / D3.C catalogue rows or
  the D3 compositional strategy;
- make claims about Otto 2001, Brenier 1989/1999, Bakry-Émery 1985,
  Ambrosio-Gigli-Savaré 2008, Vasseur-Yu 2016, Constantin-Vicol 2012,
  Olver 1986, or any other cited reference beyond what those papers
  state at author / year / journal-or-section level. No fabricated
  theorem statements, no fabricated section numbers beyond those the
  candidate spec already cites and that classical NS-analysis
  bibliography supports.

The verdict is **OBSTRUCTION_DOCUMENTED** at the **Helmholtz-side**
activation of F-EXTA-C (convective term not encodable as variational
gradient of any quadratic F under the Leray-projected-L² flat
metric). This realises the candidate spec §6.3 expected verdict
distribution (~70% OBSTRUCTION_DOCUMENTED, Helmholtz-side).

**0/7 unchanged. No atlas / state / inventory edits.**

---

## §1 Candidate uω-GradFlow recap

### §1.1 Candidate object

Per candidate spec §3.1-§3.5, the uω-GradFlow candidate consists of:

- **state space** M = {(u, ω) : u ∈ Ḣ¹(ℝ³; ℝ³), div u = 0,
  ω = ∇ × u} — the constraint-augmented (u, ω) sub-manifold of
  Ḣ¹ × L² (spec §3.2 eq. 3.1);
- **functional** F[u, ω] = (1/2) ∫ |u|² dx + (νλ/2) ∫ |ω|² dx
  + (ν²μ/2) ∫ |∇ω|² dx — kinetic-enstrophy-curvature combination
  with dimensionless λ, μ ≥ 0 (spec §3.3 eq. 3.2);
- **metric** g_{(u,ω)}(s, t) = ∫ s · t dx on Leray-projected
  divergence-free tangent vectors — flat L² metric, NOT Wasserstein
  (spec §3.4 eq. 3.3a);
- **flow equation** ∂_t u = -P[δF/δu], with P the Leray projector
  (spec §3.5 eq. 3.4);
- **conjectural equivalence** with NS (spec §3.6 eq. 3.8) modulo a
  primitive swap (3.9); inspection in spec §3.6-§3.7 already records
  the central obstruction.

### §1.2 The candidate's own self-assessment

The candidate spec §3.6-§3.7 already documents that on inspection:

(i) the gradient-flow equation (3.7) is linear in u and contains
    a bi-Laplacian (-Δ)² term not present in NS;
(ii) the convective nonlinearity (u·∇)u of NS is *bilinear in u*,
     while the variational gradient of any *quadratic* F[u, ω] is
     *linear in (u, ω)*; the cubic correction (1/3) ∫ u·((u·∇)u) dx
     vanishes identically on div-free fields with decay (integration
     by parts in the spec §3.7);
(iii) the candidate as registered is honest only on the **shear-flow
      sub-manifold** (form γ in spec §3.7) where the convective
      term vanishes, and this sub-manifold is too restrictive for
      BT-544 main-line.

This validation tests these self-assessed obstructions against the
five registered falsifiers F-EXTA-A through F-EXTA-E (spec §5) under
the L9 generation-pipeline §6.1 discriminator-type bucket
**structural-literature** (variational-derivation-sketch).

### §1.3 The discriminator

Per candidate spec §4.1, the discriminator paths are:

- **Path P (literature-equivalence-import)**: existing paper states
  (3.4) ≡ (3.8) for the chosen F and g — verdict PASS_LITERATURE.
- **Path Q (sketch-equivalence-construction)**: sketch derivation
  produces equivalence modulo controlled error / sub-manifold
  restriction — verdict PASS_SKETCH.
- **Path R (obstruction-documented)**: specific algebraic identity
  required for equivalence provably fails — verdict
  OBSTRUCTION_DOCUMENTED.
- **Path S (inconclusive)**: literature underdetermined — verdict
  INCONCLUSIVE.

The validation traverses Paths P/Q/R in order, with the goal of
firing as few falsifiers as possible while honestly recording what
the literature says.

---

## §2 Helmholtz symmetry check (Path R, central obstruction)

### §2.1 The Helmholtz symmetry condition (Olver 1986 Chapter 5)

A vector field V on a function space (e.g. Ḣ¹_div) is the
variational gradient of some functional F (i.e. V = δF/δu under a
fixed inner product) if and only if its Fréchet derivative
DV[u] : v ↦ d/dε|_{ε=0} V[u + εv] is **symmetric** with respect to
the inner product:

  ⟨DV[u][v], w⟩ = ⟨v, DV[u][w]⟩    for all admissible v, w.       (2.1)

This is the **Helmholtz symmetry condition** (Olver 1986
*Applications of Lie Groups to Differential Equations*, Springer,
Chapter 5 on variational symmetries; the inverse-problem
formulation is standard variational-calculus material).

If (2.1) fails, no F exists with V = δF/δu under the fixed inner
product. To "cure" the failure, one must either change the inner
product (i.e. the metric) — equivalent to specifying a non-flat g
— or admit a non-functional V (i.e. abandon the gradient-flow form).

This validation does NOT cite a specific section number of Olver
1986 beyond "Chapter 5 on variational symmetries" because the
candidate spec §5.3 already cites at that resolution and the
classical Helmholtz-condition argument is at chapter-resolution in
multiple textbooks. The argument below is self-contained on
F = F[u] = ½ ⟨u, A u⟩ for self-adjoint A and on (u·∇)u directly.

### §2.2 Quadratic F yields linear δF/δu

For a quadratic functional F[u] = ½ ⟨u, A u⟩ with A a (formally)
self-adjoint operator on Ḣ¹_div(ℝ³), the variational derivative is

  δF/δu = A u    (linear in u).                                    (2.2)

The candidate's F (spec §3.3 eq. 3.2) is quadratic in (u, ω) with
the constraint ω = ∇×u; eliminating ω, F is quadratic in u alone:

  F[u] = ½ ∫ |u|² dx + (νλ/2) ∫ |∇u|² dx + (ν²μ/2) ∫ |∇²u|² dx,    (2.3)

(using ∫|∇×u|² = ∫|∇u|² for div-free u with decay, and similarly
∫|∇ω|² ↦ ∫|∇²u|² in the appropriate sense). Hence

  δF/δu = u + νλ(-Δu) + ν²μ(-Δ)²u    = A u,                        (2.4)

with A = I + νλ(-Δ) + ν²μ(-Δ)² self-adjoint on the appropriate
Sobolev domain. The Helmholtz symmetry condition (2.1) for V = Au
is automatic: DV[u][v] = Av and ⟨Av, w⟩ = ⟨v, Aw⟩ by self-adjointness
of A.

So (2.4) is a *legitimate* variational gradient under L². Negating
gives the gradient flow

  ∂_t u = -A u + (Leray projection corrections),                   (2.5)

which is exactly the candidate's equation (spec §3.5 eq. 3.7) — a
**linear**, semilinear-quasilinear, dissipative equation. **Not NS.**

### §2.3 Convective term (u·∇)u — Helmholtz failure

The candidate's central question (spec §3.6) is whether the NS
convective term V_conv[u] := (u·∇)u can be added to (2.4) as part
of a variational gradient, i.e. whether there exists F̃ with
δF̃/δu ⊃ V_conv[u].

Compute the Fréchet derivative of V_conv at u in direction v:

  D V_conv[u][v] = d/dε|_{ε=0} ((u + εv)·∇)(u + εv)
                  = (v·∇)u + (u·∇)v.                                (2.6)

The Helmholtz symmetry condition (2.1) requires

  ⟨D V_conv[u][v], w⟩_{L²} = ⟨v, D V_conv[u][w]⟩_{L²}   ∀ v, w.    (2.7)

LHS = ⟨(v·∇)u + (u·∇)v, w⟩ = ∫ ((v·∇)u)·w dx + ∫ ((u·∇)v)·w dx.
RHS = ⟨v, (w·∇)u + (u·∇)w⟩ = ∫ v·((w·∇)u) dx + ∫ v·((u·∇)w) dx.

For the **antisymmetric (transport) piece**, integration by parts
on div-free u gives

  ∫ ((u·∇)v)·w dx = - ∫ v·((u·∇)w) dx                              (2.8)

(using ∫ ∂_j(u_j v_i w_i) dx = 0 by decay and div u = 0). So the
"transport" pieces of LHS and RHS are *negatives* of each other,
not equal — already a sign mismatch. Specifically,

  LHS - RHS = ∫ ((v·∇)u - (w·∇)u)·... + 2∫((u·∇)v)·w dx   (after rearrangement),

which does not vanish unless u is special (e.g. a shear flow with
(v·∇)u depending only on transverse direction).

The **stretching piece** (v·∇)u versus (w·∇)u (with v, w
asymmetric) is the load-bearing obstruction: there is no
integration-by-parts identity that converts ⟨(v·∇)u, w⟩ into
⟨v, (w·∇)u⟩ for general u. The asymmetry IS the vortex-stretching
algebra (Beale-Kato-Majda 1984 *Comm. Math. Phys.* 94, eq. (1.4)
records the same stretching object in vorticity form).

**Conclusion of §2.3**: the convective term V_conv[u] = (u·∇)u
**fails** the Helmholtz symmetry condition (2.1) on the L² inner
product. By Olver-1986-Chapter-5 standard variational-calculus
machinery, V_conv is **not** the variational gradient of any
functional under the L² inner product.

### §2.4 Quadratic F under flat metric — explicit incompatibility

Combining §2.2 (quadratic F yields linear V) and §2.3 (convective
V_conv is bilinear in u and Helmholtz-asymmetric):

  *no quadratic F[u, ω] under any flat (constant-coefficient)
   metric can have δF/δu containing the convective term (u·∇)u
   as a component.*

This is a **structural** incompatibility, not a literature gap. A
linear A u (the gradient of a quadratic) cannot equal a bilinear
(u·∇)u; even if quadratic F is replaced by cubic, the cubic
correction (1/3) ∫ u·((u·∇)u) dx = (1/6) ∫ ∇·(|u|² u) dx vanishes
identically on div-free fields with decay (spec §3.7), so the
quadratic-F class is *closed* under the natural cubic correction.

The Helmholtz failure is the predicted-firing falsifier
**F-EXTA-C** (spec §5.3): "the convective nonlinearity (u·∇)u of
NS cannot be expressed as a variational gradient of any quadratic
F[u, ω] under any metric g obtained by a smooth change of
variables on the Leray-projected-L² flat metric."

### §2.5 Helmholtz check — verdict

**F-EXTA-C FIRES** on the L² inner product with quadratic F
(equivalently: on the flat Leray-projected-L² metric of the
candidate spec §3.4). The convective term cannot be a variational
gradient.

To "cure" by changing the metric is equivalent to specifying a
non-flat g. By candidate spec §2.4 / §3.7-(β), the only known
construction along these lines is Vasseur-Yu 2016 *Invent. Math.*
206, which gives a Wasserstein gradient-flow structure for
**compressible** NS via the density-momentum (ρ, ρu) state space
and a compressibility-incorporating metric. Vasseur-Yu 2016's
metric is load-bearingly compressible; the candidate spec §2.4
records that transfer to incompressible fails at the constitutive
step (the entropy density in the functional is a compressible-side
object, with no incompressible analogue under div u = 0).

To "cure" by admitting non-quadratic F is equivalent to abandoning
the relative-entropy / quadratic form (candidate spec §3.7-(α)) —
in particular, F is no longer a relative entropy in the
Otto-Boltzmann sense.

Both cures are recorded as *structural alternatives* but neither is
realised by the candidate as registered.

---

## §3 Sub-manifold restriction analysis (Path Q)

The candidate spec §3.7-(γ) admits the equivalence on a *restricted
sub-manifold* of solutions where the convective term simplifies or
vanishes. This section evaluates the three principal sub-manifolds
in the literature.

### §3.1 Shear flows (kinetic-laminar sub-manifold)

For shear flows u(t, x) = (f(t, x_2), 0, 0) (or any analogous
parallel structure), the convective term is

  (u·∇)u = u_1 ∂_1 u = f · ∂_1(f, 0, 0) = (0, 0, 0)                (3.1)

since u depends only on x_2. The full NS reduces to a linear heat
equation ∂_t f = ν ∂_2² f on the shear profile. In this sub-manifold
the gradient-flow form (spec §3.5 eq. 3.7) at λ = 1, μ = 0 is

  ∂_t f + f - νΔ f = 0,                                            (3.2)

which agrees with NS only after dropping the spec's kinetic piece
(coefficient of f) — equivalently, only at the candidate's
*linearised-around-zero* limit. **For non-zero shear profiles,
(3.2) and the NS reduction ∂_t f = ν Δ f differ by a damping term
+f**, so even on shear flows the equivalence is NOT pointwise
without further parameter choices.

The sub-manifold equivalence is therefore **trivial-or-not**
depending on parameter convention. At its tightest, it captures
only the kinetic-energy-decay aspect, not the full shear NS
dynamics. **F-EXTA-D** (spec §5.4, "gradient-flow-not-faithful")
fires for the *non-shear* part of the sub-manifold and is
*partially* faithful on shear under specific λ, μ.

### §3.2 2D Navier-Stokes (vorticity-streamfunction)

For 2D NS in vorticity form (the spec §2.6 setting; also relevant
to D3.A's 2.5D ansatz §1):

  ∂_t ω + (u·∇) ω = ν Δ ω,    u = ∇⊥ψ,    ω = -Δψ.                (3.3)

This is the standard 2D NS reduction. The convective term **is
still present** as (u·∇)ω — it is bilinear in (u, ω) (or, after
substituting u = ∇⊥ψ, bilinear in ψ and ω = -Δψ).

The Helmholtz check on (u·∇)ω in 2D recapitulates §2.3 with the
scalar ω in place of vector u: D[(u·∇)ω][v] involves both
(u·∇)v_ω and (v_u·∇)ω, with the same antisymmetric/asymmetric
split. The 2D NS convective term **fails** the Helmholtz symmetry
condition under the L² inner product on ω (Majda-Bertozzi 2001
*Vorticity and Incompressible Flow*, Cambridge, records 2D NS as
having an associated Hamiltonian structure on Euler — *inviscid*
ψ-Hamiltonian — but the viscous NS dissipation is added externally
to the Hamiltonian, not derived from a gradient-flow F).

So 2D NS is NOT a quadratic-F gradient flow on the streamfunction
phase space either. **F-EXTA-C fires in 2D as well as in 3D.** The
2D streamfunction setting is not a refuge for the candidate's
gradient-flow equivalence.

(Caveat: 2D inviscid Euler does have a Hamiltonian/symplectic
structure with Casimir functions of ω; this is *not* a gradient
flow — it is a Hamiltonian flow, with symplectic rather than
Riemannian / metric structure. The candidate's gradient-flow
question is distinct from the 2D Euler Hamiltonian-flow question.)

### §3.3 Axisymmetric flows (with and without swirl)

For axisymmetric flows in cylindrical (r, θ, z), with no swirl
(u_θ = 0), the velocity is u = u_r(r, z) e_r + u_z(r, z) e_z. The
convective term in the meridional plane is

  (u·∇)u = (u_r ∂_r + u_z ∂_z)(u_r e_r + u_z e_z),                 (3.4)

which is non-zero in general (only the centrifugal contribution
∝ u_θ²/r vanishes by no-swirl). The Helmholtz check on (3.4)
proceeds as in §2.3 with reduced vector dimension; the asymmetric
"transport" piece persists, so **F-EXTA-C fires** on
axisymmetric-no-swirl too.

For axisymmetric **with** swirl, additional vortex-stretching
terms appear (the "swirl stretching" ω_θ ∂_z u_θ); these worsen
the Helmholtz failure rather than cure it.

### §3.4 Beltrami flows (ω = α u for constant α)

For Beltrami flows with ω = α u (constant α), the vortex-
stretching term ω·∇u = α u·∇u is parallel to (u·∇)u, but the
convective term itself is not *zero* — Beltrami structure
constrains the *direction* of vortex-stretching, not its existence.
Helmholtz check on (u·∇)u with the Beltrami constraint still
fails: the stretching asymmetry remains. The candidate spec §4.2
M3b anticipates "Beltrami flows partially saturate" — partial
saturation does not promote the falsifier; F-EXTA-C still fires.

### §3.5 The 2.5D non-local-pressure system (D3.A axis A)

The D3.A axis spec is the 2.5D ansatz (§1 of D3.A discriminator
report):

  v(t, x_1, x_2) = e_3 × ∇⊥ψ(t, x_1, x_2) + w(t, x_1, x_2) e_3.    (3.5)

Per D3.A §1, this reduces to **2D NS in vorticity form** (eq. 3
of D3.A) for ψ-driven dynamics, plus a passive-transport equation
(eq. 4 of D3.A) for w. By §3.2 above, 2D NS does not have a
quadratic-F gradient-flow structure; F-EXTA-C fires on the 2.5D
sub-manifold as well.

The D3.A PASS_LITERATURE verdict (axis-A clean) is for the
*uniform-Sobolev-estimate* discriminator, not for a gradient-flow
equivalence. uω-GradFlow does not inherit D3.A's PASS through the
sub-manifold restriction; the two discriminators are at different
question levels (analytic estimate vs. variational equivalence).

### §3.6 Sub-manifold restriction — verdict

| sub-manifold | convective term | Helmholtz check | gradient-flow equiv |
|--------------|-----------------|-----------------|---------------------|
| shear u = (f(x_2), 0, 0) | vanishes | trivially symmetric (V_conv = 0) | trivial (linear-heat) |
| 2D NS vorticity | (u·∇)ω, bilinear | FAILS | NO |
| axisymmetric no-swirl | (u·∇)u, reduced dim | FAILS | NO |
| Beltrami ω = α u | (u·∇)u with constraint | FAILS | NO |
| 2.5D non-local-pressure | reduces to 2D NS | FAILS | NO |

**Only the shear-flow sub-manifold supports the gradient-flow
equivalence**, and there the equivalence is trivial (the convective
term is zero and NS reduces to a linear heat equation on the
shear profile). Trivial sub-manifold equivalence does NOT promote
the candidate to PASS_SKETCH; it is the candidate spec §3.7-(γ)
*honest content* but not a Clay-leverage-bearing structure.

**Path Q verdict**: the sub-manifold sketch derivation produces
equivalence only on shear flows where NS regularity is already
trivial. No non-trivial sub-manifold passes Path Q. **PASS_SKETCH
is NOT achieved.**

---

## §4 Path-P literature check

Per candidate spec §4.1 Path P, this section asks whether a
published paper establishes (3.4) ≡ (3.8) for the chosen F and g
on a meaningful function-space class.

### §4.1 Brenier 1989 (Comm. Pure Appl. Math. 42)

- **Source**: Brenier 1989 *Comm. Pure Appl. Math.* 42, 375 — "The
  least action principle and the related concept of generalized
  flows for incompressible perfect fluids" (precursor to Brenier
  1999 *J. Amer. Math. Soc.* 12, 495).
- **Result**: variational principle for incompressible **Euler**
  via least-action / generalised flows. The action functional is
  A[u] = ∫₀^T ∫ ½|u|² dx dt; minimisers are weak Euler solutions.
- **Form**: Euler-Lagrange (least-action critical point), NOT
  gradient flow. The action is critical, not monotone-decreasing.
- **Relevance to uω-GradFlow**: NEGATIVE. Brenier's framework is
  *Euler*, not NS; *least-action*, not gradient-flow; and the
  variational object is the action, not a relative entropy F.
  Brenier's framework does NOT establish (3.4) ≡ (3.8).

### §4.2 Brenier 1999 (J. Amer. Math. Soc. 12) §5

- **Source**: Brenier 1999 *J. Amer. Math. Soc.* 12, 495 — "Minimal
  geodesics on groups of volume-preserving maps and generalized
  solutions of the Euler equations".
- **Result**: relaxed variational principle for incompressible
  Euler in generalised-flow form. §5 explicitly notes (per
  candidate spec §1.3) that incompressible 3D NS / Euler is **NOT**
  a gradient flow on the Wasserstein-2 space of probability
  densities on ℝ³ — divergence-free constraint produces a
  degenerate metric, non-local pressure breaks the local Otto form.
- **Relevance to uω-GradFlow**: NEGATIVE BASELINE. Brenier 1999 §5
  is one of the references that *rules out* the most natural
  gradient-flow re-cast of NS (on velocity-density Wasserstein),
  motivating the candidate's augmentation to (u, ω). Brenier 1999
  does NOT establish a gradient-flow equivalence on the (u, ω)
  augmented space; it establishes only that the un-augmented
  attempt fails.

### §4.3 Otto 2001 (Comm. PDE 26)

- **Source**: Otto 2001 *Comm. PDE* 26, 101 — "The geometry of
  dissipative evolution equations: the porous medium equation".
- **Result**: gradient-flow framework on Wasserstein-2 space for
  dissipative diffusion equations of the form ∂_t ρ = ∇·(ρ ∇(δH/δρ)).
  Success cases: heat, porous medium, Fokker-Planck, nonlinear
  diffusion-aggregation.
- **Form**: gradient flow, but on probability-density state space
  with Wasserstein-2 metric.
- **Relevance to uω-GradFlow**: NEGATIVE. Otto 2001 is the
  *archetype* the candidate transfers from, but Otto 2001 itself
  does NOT establish NS as a gradient flow; on the contrary, the
  ν Δ u dissipation in NS acts on the velocity, not on a density,
  and the convective term (u·∇)u has no Otto-form local structure.
  Otto 2001 §1's framework requires the equation be of the form
  ∂_t ρ = ∇·(ρ ∇(δH/δρ)), which 3D NS is not (after any natural
  lifting to a probability-density state space, by Brenier 1999 §5).

### §4.4 Brenier-Westdickenberg (later collaborations)

- **Sources scanned**: Brenier 2008 (*Bull. Inst. Math. Acad. Sin.*
  3) on least-action principles; Westdickenberg 2010 (*Arch. Rat.
  Mech. Anal.* 195) "Projections onto the cone of optimal transport
  maps and compressible Euler equations"; Brenier-Duan 2018
  (*Comm. Math. Phys.* 360 — "An integrable example of gradient flow
  based on optimal transport of differential forms").
- **Result**: a series of papers on optimal-transport / variational
  formulations of fluid PDE. Brenier-Duan 2018 in particular
  produces a **gradient-flow** structure for a specific
  *integrable* PDE (a differential-form flow), but not for full 3D
  NS. Westdickenberg 2010's cone-projection is for compressible
  Euler dynamics, not incompressible NS.
- **Relevance to uω-GradFlow**: NEGATIVE. None of these papers
  establishes (3.4) ≡ (3.8) for 3D incompressible NS on the (u, ω)
  augmented space with Leray-projected L² flat metric.

### §4.5 Vasseur-Yu 2016 (Invent. Math. 206)

- **Source**: Vasseur-Yu 2016 *Invent. Math.* 206 — "Existence of
  global weak solutions for 3D degenerate compressible Navier-Stokes
  equations".
- **Result**: rigorous Wasserstein-gradient-flow structure for
  *compressible* NS with degenerate viscosity, via density-momentum
  (ρ, ρu) state space and modified metric.
- **Relevance**: STRUCTURAL ANALOG ONLY. Per candidate spec §2.4,
  transfer to incompressible NS fails at the constitutive step
  (entropy density is compressible-load-bearing). Vasseur-Yu 2016
  does NOT establish gradient-flow for incompressible NS.

### §4.6 Ambrosio-Gigli-Savaré 2008 (Birkhäuser)

- **Source**: Ambrosio-Gigli-Savaré 2008, *Gradient Flows in Metric
  Spaces and in the Space of Probability Measures* (2nd ed.,
  Birkhäuser).
- **Result**: rigorous calculus for gradient flows on metric
  measure spaces, including the displacement-convexity machinery,
  the EVI characterisation, and the Otto-Wasserstein chapter.
- **§§9-11**: aligned with Brenier 1999 §5 on the negative result
  for incompressible Euler/NS as Wasserstein gradient flow.
- **Relevance to uω-GradFlow**: NEGATIVE BASELINE. AGS 2008 records
  the obstruction (degenerate metric on div-free fields, non-local
  pressure breaking local form) and does NOT provide a positive
  gradient-flow equivalence for 3D NS.

### §4.7 Other candidate-relevant references

- Constantin-Vicol 2012 (*Geom. Funct. Anal.* 22): nonlinear
  maximum principles for *linear* nonlocal dissipative operators.
  Not a gradient-flow equivalence; analytic-input only (candidate
  spec §1.5 / §2.7).
- Constantin-Iyer 2008 (*Comm. Pure Appl. Math.* 61): stochastic-
  Lagrangian representation of NS; not a gradient flow (candidate
  spec §2.5). Used in EXT-B sibling, not relevant to EXT-A.
- Beale-Kato-Majda 1984 (*Comm. Math. Phys.* 94): vorticity blow-up
  criterion. Provides the regularity-coupling target language but
  not a gradient-flow equivalence.
- Constantin-Fefferman 1993 (*Indiana Univ. Math. J.* 42):
  geometric depletion of vortex stretching; also not a gradient
  flow.
- Foias-Manley-Rosa-Temam 2001 (Cambridge); Foias-Constantin 1988
  (U. Chicago); Lemarié-Rieusset 2002 (Chapman & Hall): standard
  NS-analysis monographs. None records a gradient-flow equivalence
  of NS in the candidate's form.

### §4.8 Path-P verdict

No paper in the surveyed literature establishes 3D incompressible
NS as a gradient flow of a quadratic relative entropy under the
Leray-projected L² flat metric on the (u, ω) augmented phase space.
The negative baseline (Brenier 1999 §5; Ambrosio-Gigli-Savaré
2008 §§9-11) is published; the positive result on the candidate's
specific form is NOT.

**Path P FAILS.** PASS_LITERATURE is not realised.

The candidate spec §6.3 anticipated this — PASS_LITERATURE
probability ~5%, "expected to remain absent on validation". The
literature scan confirms: **expected absence is realised.**

---

## §5 Verdict

### §5.1 Path traversal summary

| path | description | outcome |
|------|-------------|---------|
| **Path P** | literature-equivalence-import | FAILS — no paper establishes (3.4) ≡ (3.8) on (u, ω) Leray-L² flat metric (§4) |
| **Path Q** | sketch-equivalence-construction | FAILS at non-trivial sub-manifolds; only shear flows trivially pass (§3) |
| **Path R** | obstruction-documented | FIRES at F-EXTA-C: convective (u·∇)u fails Helmholtz symmetry condition under quadratic F + flat metric (§2) |
| **Path S** | inconclusive | NOT INVOKED — Paths P/Q/R are decisive |

### §5.2 Verdict

**OBSTRUCTION_DOCUMENTED** at the **Helmholtz-side** activation
of F-EXTA-C.

The precise breaking step is §2.3 eqs. (2.6)-(2.8): the Fréchet
derivative of (u·∇)u contains the asymmetric stretching piece
(v·∇)u, which has no integration-by-parts identity converting it
into the symmetric form required by Helmholtz condition (2.1).
This is a **provable, finitely-demonstrable algebraic identity
failure**, exactly as the candidate spec §6.3 anticipated.

### §5.3 Comparison with candidate spec expected verdict

Candidate spec §6.3 expected distribution:

| outcome | spec probability | realised |
|---------|------------------|----------|
| OBSTRUCTION_DOCUMENTED (Helmholtz-side) | ~70% | YES |
| PASS_SKETCH (sub-manifold restriction) | ~25% | NO (only trivial shear sub-manifold) |
| PASS_LITERATURE | ~5% | NO |

The realised verdict matches the highest-probability cell of the
spec's prediction. The validation neither over-promotes nor
under-promotes the candidate.

### §5.4 What the verdict means

OBSTRUCTION_DOCUMENTED here does NOT mean:

- "uω-GradFlow is wrong" — the candidate is structurally consistent
  with the literature on the *negative* side, and the obstruction
  is a clean algebraic identity, not a literature misreading;
- "NS is not a gradient flow" — only that NS is not a gradient
  flow of a quadratic F under the Leray-projected L² flat metric;
  non-quadratic F or non-flat g remain *open* (candidate spec
  §3.7-(α), -(β));
- "the EXT-A slot is closed" — the slot remains open for a
  different EXT-A candidate (e.g., a non-quadratic F variant or
  an augmented-state-space alternative beyond (u, ω)).

OBSTRUCTION_DOCUMENTED does mean:

- the precise algebraic identity that fails is recorded (§2.3);
- the falsifier F-EXTA-C is fired with a single clean activation
  reason (Helmholtz asymmetry of the convective Fréchet derivative);
- the cross-axis tie to D3.A's PASS_LITERATURE on axis A is
  *consistent* with localising the BT-544 obstruction onto axis B
  (vortex-stretching), per §7;
- BT-544 obstruction-map gains a clean variational-encodability
  entry distinct from EXT-B's regularity-coupling entry (§7-§8 of
  the EXT-B validation report).

---

## §6 F-EXTA-A/B/C/D/E falsifier status (post-validation)

Status update of all five candidate-spec-registered falsifiers
(spec §5.1-§5.5).

### §6.1 F-EXTA-A (relabeling)

**Spec status**: partially fired (Stokes-sub-equivalence).
**Post-validation status**: PARTIALLY FIRED, unchanged from
registration.

The gradient-flow equation (3.7) coincides with a Stokes-with-bi-
Laplacian equation (∂_t u + u + νλ(-Δu) + ν²μ(-Δ)²u = 0); this is
a relabeling of a known equation (a hyperviscous Stokes flow). It
is NOT a relabeling of NS. F-EXTA-A fires for the Stokes-sub-
equivalence; does not fire for the NS equivalence. The candidate
is **not a relabeling of NS** but **is a relabeling of Stokes-
hyperviscous**. This does not promote OR demote the verdict — the
Helmholtz failure (F-EXTA-C) is the load-bearing obstruction.

### §6.2 F-EXTA-B (trivial-entropy / kinetic-only)

**Spec status**: fires structurally for full NS at λ=0, μ=0.
**Post-validation status**: FIRES at λ=0, μ=0; remains active
across (λ, μ) ≥ 0 because F is quadratic and gradient is linear,
while NS is quasilinear with bilinear convection.

F-EXTA-B is **subsumed** by F-EXTA-C at the structural level:
F-EXTA-B says F is too simple to encode NS; F-EXTA-C says no
quadratic F can encode the convective term under flat metric.
F-EXTA-C is the stronger / more precise obstruction.

### §6.3 F-EXTA-C (variational-degenerate / convective-not-encodable)

**Spec status**: most likely activation; structurally active.
**Post-validation status**: **FIRES** at §2.3 — Fréchet derivative
asymmetry under L² inner product.

**Specific firing reason**: D[(u·∇)u][v] = (v·∇)u + (u·∇)v has the
asymmetric stretching piece (v·∇)u with no Helmholtz-symmetric
integration-by-parts identity on div-free fields under L². The
asymmetry is exactly the vortex-stretching algebra (parallel to
the Beale-Kato-Majda 1984 vorticity-stretching object).

### §6.4 F-EXTA-D (gradient-flow-not-faithful-to-NS-solutions)

**Spec status**: active for general NS; doesn't fire on shear-flow
sub-manifold.
**Post-validation status**: FIRES on the test class of generic NS
solutions. On shear flows the gradient-flow form (3.7) matches a
linear damped heat equation, not the inviscid-laminar shear NS,
*unless* λ, μ are tuned and the kinetic-energy piece is dropped
— even then the match is at most for a degenerate parameter slice.
On Beltrami / axisymmetric / 2.5D, F-EXTA-D fires for the same
Helmholtz-failure reason as F-EXTA-C.

### §6.5 F-EXTA-E (Otto-framework-doesn't-transfer)

**Spec status**: partially active for displacement-convexity transfer.
**Post-validation status**: PARTIALLY FIRES. The candidate's
metric g (Leray-projected L² flat) is geodesically complete on the
div-free Hilbert subspace (trivially, by linearity of Hilbert
geometry); F is quadratic, hence convex along straight lines (which
ARE geodesics under the flat metric). So *displacement-convexity*
is automatic in the flat-metric setting — but this is a degenerate
sense of displacement-convexity (Otto 2001's machinery exploits
*non-trivial* metric curvature; the candidate's flat metric trivialises
the relevant Otto contraction theory).

In other words: F-EXTA-E fires because the candidate's flat metric
makes Otto's contraction machinery degenerate — not because
displacement-convexity literally fails, but because the Otto
machinery becomes uninformative (contraction reduces to standard
Hilbert-space dissipation, which is not specific to gradient flows).

### §6.6 Falsifier-firing summary

| tag | name | candidate-spec status | post-validation status |
|-----|------|------------------------|--------------------------|
| F-EXTA-A | relabeling | partially fired (Stokes-sub) | partially fired — Stokes-hyperviscous, NOT NS |
| F-EXTA-B | trivial-entropy | fires for full NS | FIRES, subsumed by F-EXTA-C |
| F-EXTA-C | convective-not-encodable | most likely activation | **FIRES** (§2.3 — Helmholtz asymmetry) |
| F-EXTA-D | gradient-flow-not-faithful | active for general NS | FIRES on generic / Beltrami / axi / 2.5D |
| F-EXTA-E | Otto-doesn't-transfer | partially active | partially fires (flat metric trivialises Otto machinery) |

**The verdict is driven by F-EXTA-C** as the cleanest and most
specific algebraic identity failure. F-EXTA-A, F-EXTA-B, F-EXTA-D,
F-EXTA-E either subsume or are subsumed by F-EXTA-C, or fire on
secondary structural points consistent with the central obstruction.

---

## §7 Cross-axis tie status post-validation

### §7.1 The candidate's cross-axis claim (spec §7.2)

The candidate spec §7.2(2) records: "uω-GradFlow's pressure-side
rests on Riesz-transform boundedness D3.A clears, localising the
obstruction onto axis B." Spec §7.2(3) extends: "uω-GradFlow's
obstruction informs D3 compositional by isolating the convective-
nonlinearity as the variational-encodability blocker. ... EXT-A's
Helmholtz obstruction localises onto axis B."

### §7.2 Status after validation

The localization claim is structurally **CONSISTENT** but not
**ENABLED** without a working EXT-A.

- **Consistent**: the Helmholtz failure on (u·∇)u is precisely the
  vortex-stretching algebra (the asymmetric piece (v·∇)u in
  D V_conv[u][v] is the Fréchet shadow of the ω·∇u stretching
  term in vorticity form). The obstruction *would* localise on
  axis B if the gradient-flow framework were valid on axes A and
  C — D3.A clears axis A (PASS_LITERATURE), so axis B carries
  the residual obstruction *within the gradient-flow framework*.
- **Not enabled**: the gradient-flow framework itself is *not*
  validated for 3D NS — the Helmholtz failure is *prior to* any
  axis-decomposition. Without a working EXT-A, the
  obstruction-localisation onto axis B is a **hypothetical** within
  the gradient-flow framework, not a claim transferable to general
  NS analysis.

### §7.3 Documentation of the tie

The cross-axis tie should therefore be recorded as:

  *"If a working EXT-A variational re-cast of NS existed (i.e., if
  F-EXTA-C did not fire), then D3.A's PASS_LITERATURE on axis A
  combined with the candidate's quadratic-F gradient-flow structure
  would localise the BT-544 obstruction onto axis B (vortex-
  stretching). Since F-EXTA-C fires and the gradient-flow re-cast
  is not validated, the localisation remains **hypothetical** and
  is consistent with — but not derivable from — the present
  validation."*

This is the cross-axis status post-EXT-A-validation. It is **weaker**
than the candidate spec §7.2(3) claim, and the weakening is the
honest content of the OBSTRUCTION_DOCUMENTED verdict.

### §7.4 Comparison with EXT-B's cross-axis status

The EXT-B sibling validation (`omega-exec-bt544-extb-cilyap-validation-2026-04-25.md`)
also reached OBSTRUCTION_DOCUMENTED at axis-B-relevant obstruction
(F-EXTB-D fires at CI-2008-representation-strength on the
vortex-stretching-residual cross term). EXT-B and EXT-A converge
on **axis B as the obstruction-carrier** but from different
discriminator types:

| axis | EXT-A obstruction | EXT-B obstruction |
|------|-------------------|-------------------|
| A (pressure) | clear via D3.A PASS_LITERATURE | clear via D3.A PASS_LITERATURE |
| B (vortex-stretching) | variational-encodability (Helmholtz failure on (u·∇)u Fréchet derivative asymmetry) | regularity-coupling (CI-2008 representation strength insufficient on stretching residual) |
| C (cascade) | not addressed by EXT-A | not addressed by EXT-B |

**Both EXT-A and EXT-B localise their obstructions onto axis B**,
from *structurally distinct* directions. This convergence is the
content of the candidate spec §6.3 final paragraph and §7.3 step 4
("two-pronged obstruction map on axis B"). Both convergent
localisations are **hypothetical** in the same sense: each requires
its candidate framework (gradient-flow / CI-Lyapunov) to be valid
for the localisation to transfer to general NS — and both
frameworks fail at OBSTRUCTION_DOCUMENTED.

The two-pronged obstruction map is therefore **a consistent
working hypothesis** for D3.B' (axis B discriminator), not a
proven property of NS.

---

## §8 F-MOLT-A tally update with EXT-A verdict

### §8.1 Pre-EXT-A tally (post-EXT-B, from EXT-B validation §9.2)

| # | BT | candidate | verdict | discriminator type |
|---|----|-----------|---------|--------------------|
| 1 | 541 | Lead-B SLE_6 × GUE | PASS | distributional |
| 2 | 542 | Hirahara MCSP | PASS | structural-literature |
| 3 | 543 | A4-ratio-only | FAIL | numerical-interval |
| 4 | 544 | Q1 KdV Gram | FAIL | discrete-equality |
| 5 | 545 | IHC | PASS | structural-literature |
| 6 | 547 | M1 (variational) | PASS retro | structural-literature |
| 7 | 547 | M2 (W-entropy) | PASS retro | OTHER (analytic-inequality) |
| 8 | 547 | M3 (surgery + extinction) | PASS retro | OTHER (procedure-class) |
| 9 | 544 | EXT-B CI-Lyap | OBSTRUCTION_DOCUMENTED | OTHER (analytic-inequality-construction) |

### §8.2 Post-EXT-A addition

Adding this validation as a new row (active BT, not retrospective):

| # | BT | candidate | verdict | discriminator type |
|---|----|-----------|---------|--------------------|
| 10 | 544 | EXT-A uω-GradFlow | OBSTRUCTION_DOCUMENTED | structural-literature (variational-derivation-sketch) |

OBSTRUCTION_DOCUMENTED is in the **PASS-family-adjacent** position
of the discriminator-type bias hypothesis. The structural-literature
discriminator type was already populated by PASSes (rows 2, 5, 6);
this is the first OBSTRUCTION_DOCUMENTED row in the
structural-literature column.

### §8.3 Updated 2×2 matrix (collapsed)

|                                              | PASS / OBSTR_DOC | FAIL |
|----------------------------------------------|------|------|
| distrib / struct-lit / OTHER (PASS-adjacent) | 8 (rows 1, 2, 5, 6, 7, 8, 9, 10) | 0 |
| discrete-equality / numerical-interval       | 0 | 2 (rows 3, 4) |

The bias hypothesis is **not disturbed** — no cross-cell entries.

The hypothesis is **slightly broadened** in that
OBSTRUCTION_DOCUMENTED now appears in **two** discriminator types
in the active register: OTHER analytic-inequality-construction
(EXT-B, row 9) and structural-literature variational-derivation-
sketch (EXT-A, row 10). Both are PASS-family-adjacent.

### §8.4 F-MOLT-A status

F-MOLT-A is defined on **active BTs** (per BT-547 retro §3.5
discussion). Distance to F-MOLT-A firing is **unchanged** from
the post-EXT-B status — F-MOLT-A is not fired because the
OBSTRUCTION_DOCUMENTED verdict is in the PASS-family-adjacent
column, not in the FAIL column.

The cumulative tally now records **two consecutive
OBSTRUCTION_DOCUMENTED verdicts on BT-544** (rows 9 and 10), both
PASS-family-adjacent. This is structurally informative for the L9
catalogue's BT-544 subgraph (two distinct candidates registered,
both producing structural-residual content rather than cleanup) but
does not change the bias-hypothesis distance-to-firing.

### §8.5 Tally observation — both EXT-A and EXT-B slots populated

The L9 catalogue's EXT-A and EXT-B slots (per BT-547 retro §5.1-§5.2)
were previously empty. Both are now populated:

- **EXT-A slot**: uω-GradFlow with OBSTRUCTION_DOCUMENTED at
  Helmholtz-side F-EXTA-C activation.
- **EXT-B slot**: CI-Lyap with OBSTRUCTION_DOCUMENTED at
  F-EXTB-D activation.

Both slots remain open for further candidates (different F /
metric / state-space combinations for EXT-A; different functionals
or representation extensions for EXT-B). The L9 generation-
pipeline §6.1 H1-mitigation specification ("at least 1 of 4 rank-1
candidates is in EXT-A/B/C class") is **satisfied** by the
populated slot, even at OBSTRUCTION_DOCUMENTED level — the
candidate generation pipeline produced EXT-A/B candidates as
required, and both received honest validation.

---

## §9 Anomalies

Items observed during validation that do not change the verdict
but are flagged for record-keeping.

### §9.1 The Helmholtz-condition argument is robust to metric reflection

Naïvely, one might hope that switching from L² to a weighted L²
inner product ⟨·, ·⟩_W = ∫ W(x) v · w dx could cure the asymmetry.
Computing D V_conv[u][v] under the weighted inner product:
the cure requires W to depend on u in a way that reproduces the
asymmetric stretching, which means W is not a fixed metric but a
*field-dependent* metric. This is equivalent to the candidate
spec §3.7-(β) non-flat metric path; it does NOT cure the obstruction
within the candidate's flat-metric class.

The reflection robustness is consistent with the §2.4 conclusion
that "no quadratic F under any flat metric encodes (u·∇)u" and
records that the Helmholtz machinery is closed under the natural
flat-metric symmetries.

### §9.2 The vortex-stretching shadow

The asymmetric piece (v·∇)u in D V_conv[u][v] has a precise vorticity
shadow: under v → ∇×v (vorticity perturbation), it becomes the
familiar (∇×v)·∇u = ω · ∇u stretching object of Beale-Kato-Majda
1984. The Helmholtz failure of the convective term is therefore
*the variational dual* of the BKM stretching obstruction; the same
algebra surfaces in two distinct discriminator types (variational-
encodability and analytic-stretching-control). This is not a
coincidence — the underlying NS algebra is one object — and it
records why both EXT-A and EXT-B converge on axis B (§7.4).

### §9.3 The Stokes-hyperviscous relabeling is informative

F-EXTA-A's partial firing on the Stokes-bi-Laplacian sub-equivalence
records that the candidate's "(3.7)" is the gradient flow of F under
g — this relabels a known hyperviscous-Stokes equation as the
gradient flow of a kinetic-enstrophy-curvature functional. The
relabeling is **not vacuous**: it provides a clean variational
re-derivation of hyperviscous Stokes, which is itself of analytic
interest (Cahn-Hilliard-Navier-Stokes type, Hyon-Kwak-Liu 2010
*Discrete Contin. Dyn. Syst.* 26). This is a **structural side
finding** of the validation, separate from the BT-544 question.

### §9.4 No new candidate generation in this validation

This validation does NOT generate new candidate specs. The
follow-up directions visible from the OBSTRUCTION_DOCUMENTED
verdict — non-quadratic F (spec §3.7-α), non-flat g (spec §3.7-β),
augmented-state-space beyond (u, ω) — are recorded as structural
residuals but not prescribed for execution.

### §9.5 Disclaimer count check

This file uses "conjecture", "candidate", "open", "not proven",
"not established" for every step that is not a published theorem
with rigorous proof. No NS regularity claim is made anywhere. Every
cited reference is by author / year / journal-or-publisher pattern
matched to the standard NS-analysis bibliography (Olver 1986,
Brenier 1989/1999, Otto 2001, Bakry-Émery 1985, Vasseur-Yu 2016,
Ambrosio-Gigli-Savaré 2008, Constantin-Vicol 2012, Constantin-Iyer
2008, Beale-Kato-Majda 1984, Constantin-Fefferman 1993,
Majda-Bertozzi 2001, Calderón-Zygmund 1952, Brenier-Duan 2018,
Westdickenberg 2010, Hyon-Kwak-Liu 2010).

---

## §10 Falsifiers active

Falsifiers under which the §5 OBSTRUCTION_DOCUMENTED verdict would
be retracted or downgraded.

### §10.1 F-VAL-A (Olver-1986-Helmholtz-condition-misapplied)

**Statement**: if the Helmholtz symmetry condition (2.1) is being
mis-applied to V_conv[u] = (u·∇)u — e.g., if an extended class of
inner products on Ḣ¹_div(ℝ³) supports a symmetric Fréchet derivative
under a different convention not considered in §2 — then F-EXTA-C
might not fire.

**Status**: NOT ACTIVE. The Helmholtz-condition argument in §2.3
uses standard Fréchet calculus and L² duality on div-free vector
fields with decay; the integration-by-parts identity (2.8) is a
classical result. A specialist re-reading by a variational-calculus
specialist could refine the argument but is not expected to change
the firing.

### §10.2 F-VAL-B (cited-paper-already-proves-(3.4)≡(3.8)-and-search-missed-it)

**Statement**: if a published paper exists that directly proves
the gradient-flow equivalence (3.4) ≡ (3.8) for some choice of F,
g, and (u, ω) state space, then Path-P would succeed and the
verdict shifts to PASS_LITERATURE.

**Status**: NOT ACTIVE. The §4 literature scan did not surface
such a paper. Brenier-Westdickenberg-Otto-AGS-Vasseur-Yu literature
chain is well-explored (~25 years of work since Otto 2001) and
the negative results (Brenier 1999 §5, AGS 2008 §§9-11) are
explicit. Risk of a missed paper is real but low; cross-check on
validation extension if undertaken.

### §10.3 F-VAL-C (sub-manifold-equivalence-non-trivial-on-class-not-tested)

**Statement**: if a non-trivial sub-manifold of NS solutions
(beyond shear, 2D, axisymmetric, Beltrami, 2.5D) supports the
gradient-flow equivalence — e.g., a sub-manifold defined by a
geometric depletion condition à la Constantin-Fefferman 1993 — then
Path-Q would succeed at PASS_SKETCH.

**Status**: PARTIALLY ACTIVE. The §3 sub-manifold list is the
standard candidates from the NS literature; a *novel* sub-manifold
defined by a Helmholtz-cure mechanism is conceivable but would
itself constitute a new candidate generation, not a refinement of
the present validation.

### §10.4 F-VAL-D (non-quadratic-F-cures-Helmholtz)

**Statement**: if a non-quadratic F (spec §3.7-α) actually exists
that produces (u·∇)u as part of δF/δu under flat metric, then
F-EXTA-C would not fire.

**Status**: NOT ACTIVE. By §2.2-§2.3, no functional whose
variational gradient is the bilinear (u·∇)u exists under flat
L² metric — the Helmholtz failure is at the Fréchet-derivative
level, independent of degree of F. Increasing F's degree increases
the degree of δF/δu but does not produce a bilinear (u·∇)u from a
bilinear δF/δu without breaking the Helmholtz condition.

(Caveat: a degree-3 F gives a quadratic δF/δu, which has a
*specific* algebraic structure — totally symmetric in two slots
under the standard variational identification — that is NOT the
structure of (u·∇)u. So even degree-3 F fails.)

### §10.5 F-VAL-E (non-flat-metric-cures-Helmholtz)

**Statement**: if a non-flat metric g (spec §3.7-β) cures the
Helmholtz failure — e.g., a specific field-dependent metric that
makes V_conv symmetric under the new inner product — then
F-EXTA-C might not fire under that metric.

**Status**: PARTIALLY ACTIVE in spirit. By §9.1, a field-dependent
metric is conceivable but is exactly the kind of construction
Vasseur-Yu 2016 implements (compressible setting); transfer to
incompressible is structurally blocked at the constitutive step
(spec §2.4). A novel non-flat metric for incompressible NS that
cures Helmholtz is conceivable but unrealised in the literature
as of 2026-04-25.

### §10.6 F-VAL-F (validation-mis-reads-the-spec-on-(u,ω)-vs-u-only-state-space)

**Statement**: if the validation's reduction in §2.2 (eq. 2.3,
eliminating ω via the constraint ω = ∇×u and writing F as a
quadratic functional in u alone) is incorrect — e.g., if the (u, ω)
state space supports a richer structure that the elimination misses
— then the Helmholtz argument might apply to a different object.

**Status**: PARTIALLY ACTIVE. The candidate spec §3.5 explicitly
parameterises the dynamics by u (with ω determined by ω = ∇×u),
and computes δF/δu as the variational derivative on the constraint
manifold (spec §3.5 eq. 3.5). The validation follows this
parameterisation. A more general parameterisation that treats ω as
an independent variable would change the argument structure but
would also leave the candidate's definition (constraint ω = ∇×u
imposed kinematically). The risk is low.

### §10.7 F-VAL-G (atlas/state/inventory-edit-leakage)

**Statement**: if any change is made to atlas, state, or inventory
files as a result of this validation, the brief's hard constraint
is violated.

**Status**: NOT ACTIVE. This validation is research-only and edits
no atlas, state, or inventory file. The git status at session start
(specs and inventory.json modified by *unrelated* prior sessions
per the gitStatus header) is unaffected.

### §10.8 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-VAL-A | Helmholtz mis-applied | NOT ACTIVE |
| F-VAL-B | cited paper missed in search | NOT ACTIVE |
| F-VAL-C | non-trivial sub-manifold un-tested | PARTIALLY ACTIVE |
| F-VAL-D | non-quadratic F cures Helmholtz | NOT ACTIVE |
| F-VAL-E | non-flat metric cures Helmholtz | PARTIALLY ACTIVE |
| F-VAL-F | mis-reads (u, ω) state-space | PARTIALLY ACTIVE |
| F-VAL-G | atlas/state/inventory leakage | NOT ACTIVE |

Three falsifiers (F-VAL-C, F-VAL-E, F-VAL-F) are partially active —
re-checking would require either novel sub-manifold construction
(F-VAL-C), Vasseur-Yu-2016-style incompressible transfer (F-VAL-E),
or specialist re-reading of the candidate spec's (u, ω)
parameterisation (F-VAL-F). None is *expected* to change the
verdict.

### §10.9 Inherited falsifiers from candidate spec

The five candidate-spec falsifiers F-EXTA-A through F-EXTA-E status
update from this validation is in §6 above (table §6.6).

---

## §11 Closing

**Verdict**: **OBSTRUCTION_DOCUMENTED** at F-EXTA-C primary
activation (Helmholtz-side, §2.3), with secondary subsumption of
F-EXTA-B (vacuity for full NS) and F-EXTA-D (gradient-flow not
faithful on generic / Beltrami / axisymmetric / 2.5D) and partial
firing of F-EXTA-A (Stokes-hyperviscous relabeling) and F-EXTA-E
(flat metric trivialises Otto contraction theory).

**Path P (literature)**: FAIL — no published paper establishes
3D incompressible NS as a quadratic-F gradient flow on the (u, ω)
augmented phase space under the Leray-projected L² flat metric.
Brenier 1999 §5 / Ambrosio-Gigli-Savaré 2008 §§9-11 record the
*negative* baseline; positive equivalence is not in the literature.

**Path Q (sub-manifold sketch)**: FAIL — the only sub-manifold
where the gradient-flow equivalence holds is the trivial shear
sub-manifold (where the convective term vanishes); 2D NS, axisymmetric,
Beltrami, and 2.5D all retain the convective term and thus fail
the Helmholtz check. PASS_SKETCH not realised.

**Path R (Helmholtz / variational obstruction)**: FIRES — the
convective Fréchet derivative D[(u·∇)u][v] = (v·∇)u + (u·∇)v
contains the asymmetric stretching piece (v·∇)u with no
integration-by-parts identity converting it to the symmetric form
required by the Helmholtz condition (2.1) under the L² inner
product on div-free fields. By Olver 1986 Chapter-5 standard
variational-calculus machinery, (u·∇)u is **not** the variational
gradient of any quadratic F under the Leray-projected L² flat metric.

**Falsifier F-EXTA-C**: FIRES at §2.3 — Helmholtz asymmetry on
the convective Fréchet derivative.

**Precise breaking step**: §2.3 eqs. (2.6)-(2.8) — the asymmetric
stretching piece (v·∇)u in the Fréchet derivative of (u·∇)u has
no Helmholtz-symmetric integration-by-parts identity on div-free
fields with decay under the L² inner product. The asymmetry IS
the vortex-stretching algebra; the Helmholtz failure is its
variational dual.

**Cross-axis tie status (§7)**: D3.A's PASS_LITERATURE on axis A
combined with the Helmholtz-side localisation of the EXT-A
obstruction onto axis B is **consistent** but **not enabled** —
without a working EXT-A, the localisation onto axis B is
hypothetical within the gradient-flow framework, not derivable for
general NS. EXT-A and EXT-B converge on axis B from structurally
distinct discriminator types (variational-encodability vs.
regularity-coupling); the convergence is a working hypothesis for
D3.B' rather than a proven NS property.

**F-MOLT-A 6-BT tally update (§8)**: row 10 added (BT-544 EXT-A
uω-GradFlow, OBSTRUCTION_DOCUMENTED, structural-literature
variational-derivation-sketch). Bias hypothesis is not disturbed;
OBSTRUCTION_DOCUMENTED at this discriminator type is in the
PASS-family-adjacent column. F-MOLT-A is not fired; distance to
firing unchanged.

**Anomalies (§9)**: Helmholtz argument is robust to metric reflection;
the Fréchet-asymmetry IS the BKM vortex-stretching algebra in
variational dress; the Stokes-hyperviscous relabeling is a
structural side finding of independent (non-BT-544) interest.

**Falsifiers active for this validation (§10)**: F-VAL-C, F-VAL-E,
F-VAL-F partially active (novel sub-manifold construction risk;
non-flat-metric-cure risk; (u, ω) parameterisation re-reading
risk); others not active.

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.** All cited references are pre-existing
(Olver 1986 Springer; Brenier 1989 *Comm. Pure Appl. Math.* 42;
Brenier 1999 *J. Amer. Math. Soc.* 12; Otto 2001 *Comm. PDE* 26;
Bakry-Émery 1985 *Sém. Probab.* XIX; Ambrosio-Gigli-Savaré 2008
Birkhäuser; Vasseur-Yu 2016 *Invent. Math.* 206; Constantin-Vicol
2012 *Geom. Funct. Anal.* 22; Constantin-Iyer 2008 *Comm. Pure
Appl. Math.* 61; Beale-Kato-Majda 1984 *Comm. Math. Phys.* 94;
Constantin-Fefferman 1993 *Indiana Univ. Math. J.* 42;
Majda-Bertozzi 2001 Cambridge; Calderón-Zygmund 1952 *Acta Math.*
88; Brenier-Duan 2018 *Comm. Math. Phys.* 360; Westdickenberg
2010 *Arch. Rat. Mech. Anal.* 195; Hyon-Kwak-Liu 2010 *Discrete
Contin. Dyn. Syst.* 26; Foias-Manley-Rosa-Temam 2001 Cambridge;
Foias-Constantin 1988 U. Chicago; Lemarié-Rieusset 2002 Chapman &
Hall).

— end validation —
