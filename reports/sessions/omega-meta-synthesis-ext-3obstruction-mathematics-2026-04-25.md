---
id: omega-meta-synthesis-ext-3obstruction-mathematics
date: 2026-04-25
scope: research-only mathematical synthesis (NOT proving NS regularity; structural diagnosis)
target: EXT-A/B/C 3 OBSTRUCTIONs -- mathematical content + BKM unification + CF-BMO bridge
parent_reports:
  - reports/sessions/omega-exec-bt544-exta-uomega-gradflow-validation-2026-04-25.md (EXT-A Helmholtz)
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md (EXT-B representation)
  - reports/sessions/omega-exec-bt544-extc-qpc-surgery-validation-2026-04-25.md (EXT-C canonical-neighbourhood)
  - reports/sessions/omega-meta-synthesis-3pillar-obstruction-localization-2026-04-25.md (3-pillar)
  - reports/sessions/omega-exec-bt544-axisB-targeted-attack-2026-04-25.md (CF-BMO bridge)
millennium_resolved: 0/7 (unchanged)
grade: mathematical synthesis, no claim
---

# Omega Meta-Synthesis -- EXT 3-OBSTRUCTION Mathematical Anatomy (2026-04-25)

## §0 Non-claim disclaimer

This file is a **mathematical synthesis** of three EXT-tier
validations against BT-544 (3D Navier-Stokes regularity), each of
which returned `OBSTRUCTION_DOCUMENTED` with a structurally
distinct named obstruction. The synthesis does **not**:

- claim 3D Navier-Stokes regularity (smoothness or blow-up); the
  Clay BT-544 status remains `0/1 untouched`;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, or alter the L9 catalogue verdicts in any
  parent EXT-A / EXT-B / EXT-C / D3-axis-B' validation;
- prove any new theorem about Olver 1986, Constantin-Iyer 2008,
  Caffarelli-Kohn-Nirenberg 1982, Perelman 2003a/b, Beale-Kato-Majda
  1984, Constantin-Fefferman 1993, or any other cited reference.
  Each citation is at author / year / journal-or-section level,
  with no fabricated theorem statements;
- supersede the per-EXT verdicts. The synthesis only **aggregates**
  the mathematical content of the three obstructions, identifies
  their **common structure**, and records that all three localize
  on the **same algebraic object** (the vortex-stretching tensor
  ω·∇u acting on Λ²(ℝ³)).

The "common structure" claim of §5 and the "BKM unification" claim
of §6 are **consistent with** the three EXT validations, not
**proved by** them. They are post-hoc structural readings, not
theorems.

**Millennium tally**: 0/7 unchanged. **NS regularity status:
open.** No atlas / state / inventory edits.

---

## §1 EXT 3-OBSTRUCTION recap

For BT-544, three EXT-tier candidates were independently generated
and validated under the L9 generation pipeline. All three returned
`OBSTRUCTION_DOCUMENTED` with structurally distinct named
obstructions.

### §1.1 EXT-A — uω-GradFlow (variational gradient-flow recast)

- **candidate**: state space M = {(u, ω) : u ∈ Ḣ¹_div(ℝ³),
  ω = curl u}; functional F[u, ω] = (1/2)‖u‖² +
  (νλ/2)‖ω‖² + (ν²μ/2)‖∇ω‖²; Leray-projected L² flat metric g;
  conjectural ∂_t u = −P[δF/δu] equivalent to NS.
- **verdict**: `OBSTRUCTION_DOCUMENTED` at F-EXTA-C
  (convective-not-encodable).
- **named obstruction**: **Helmholtz obstruction**. The convective
  Fréchet derivative D[(u·∇)u][v] = (v·∇)u + (u·∇)v fails the
  Olver-1986-Chapter-5 Helmholtz symmetry condition under the L²
  inner product. The asymmetric "stretching" piece (v·∇)u has no
  integration-by-parts identity converting it to the symmetric
  form ⟨v, (w·∇)u⟩. By the standard variational-calculus
  inverse-problem machinery, no quadratic functional under any
  flat metric can have (u·∇)u as its variational gradient.
- **algebraic content**: the asymmetric piece IS the vortex-
  stretching algebra in variational dual (EXT-A §9.2: "Fréchet-
  asymmetry IS the BKM vortex-stretching algebra in variational
  dress").

### §1.2 EXT-B — CI-Lyap (Constantin-Iyer relative-entropy
       Lyapunov)

- **candidate**: state space (u, ρ_t, τ) with ρ_t the law of the
  Constantin-Iyer 2008 stochastic Lagrangian flow; functional
  W_NS = ∫[τν|∇u|² + τ|∇log ρ|² + log ρ] ρ dx + γνt; conjectural
  d/dt W_NS ≥ 0 along the coupled NS + CI flow.
- **verdict**: `OBSTRUCTION_DOCUMENTED` at F-EXTB-D
  (representation-strength) plus secondary Path-Q cross-term.
- **named obstruction**: **representation obstruction**. CI 2008
  produces a *family* {ρ_t(·|x_0)}_{x_0 ∈ ℝ³} of conditional laws
  indexed by initial Lagrangian label, NOT a single ρ_t(x) on ℝ³.
  W_NS = ∫[…] ρ_t dx requires the latter; the collapse needs an
  unspecified choice (mixed-initial-distribution / pinned-label /
  velocity-mixed). The candidate spec leaves the choice implicit.
- **secondary**: Path-Q time-derivative produces two uncontrolled
  cross terms — ∫(ω·∇)u·ω ρ unsigned (the NS regularity obstruction
  itself) and Hess(log ρ):∇u uncontrolled by Bakry-Émery 1985 since
  the NS-CI flow is non-symmetric and lacks CD(K, ∞).

### §1.3 EXT-C — QPC-Surgery (parabolic-cell surgery procedure)

- **candidate**: parabolic-cell partition Q_{k,j} of [0,T) × ℝ³ at
  level k with edge 2^{−k}; flagging rule (1/r_k)∫∫_{Q_{k,j}}|∇u|²
  ≥ ε_* · 2^{αk} (CKN ε_*); conjectural cardinality cascade
  #F_k ≤ C(α)·2^{(2−α)k} for some α ∈ (0,2); conjectural (T1)
  termination F_{k_*} = ∅ at finite k_*.
- **verdict**: `OBSTRUCTION_DOCUMENTED` at F-EXTC-D
  (T1-termination = NS regularity).
- **named obstruction**: **canonical-neighbourhood obstruction**.
  Perelman 2003a's surgery + extinction package (M3 archetype) is
  built on a third piece beyond parameter-cascade and finite-step
  termination: a **canonical-neighbourhood theorem** identifying
  geometric primitives ("ε-thin neck", "ε-cap", "ε-horn") that say
  *what shape* a high-curvature region has, enabling rigorous
  surgery cuts. NS lacks the analog: CKN+Lin 1998+Vasseur 2007
  transfer the *cell partition* and *parameter cascade* (parabolic
  scaling, ε-regularity, De Giorgi truncation), but Perelman's
  *geometric-content* third piece has no NS analog. F-EXTC-D
  fires: T1 termination on general 3D smooth data ⟺ NS regularity
  itself (relabeling).

### §1.4 Summary of the three named obstructions

| EXT | name | mathematical content |
|-----|------|----------------------|
| A | Helmholtz | (u·∇)u Fréchet asymmetric ⇒ no quadratic-F variational gradient under flat L² metric |
| B | representation | CI 2008 produces {ρ_t(·\|x_0)} family, not single ρ_t on ℝ³; cross-term ω·∇u·ω + Hess(log ρ):∇u uncontrolled |
| C | canonical-neighbourhood | NS lacks Perelman M3 third-piece (geometric-primitive recognition theorem); T1 ⟺ NS regularity |

The three obstructions arise in three structurally distinct
discriminator types: variational (EXT-A), stochastic-Lagrangian
analytic (EXT-B), procedure-class (EXT-C). The convergence of
their named obstructions onto a single algebraic object (§5) is
the load-bearing observation of this synthesis.

---

## §2 Mathematical anatomy of (A) Helmholtz

### §2.1 The Helmholtz symmetry condition (Olver 1986 Chapter 5)

A vector field V[u] on a function space H (with inner product
⟨·,·⟩) is the variational gradient V = δF/δu of some functional F
if and only if its Fréchet derivative

  DV[u] : v ↦ d/dε|_{ε=0} V[u + εv]                              (2.1)

is **symmetric** with respect to ⟨·,·⟩, i.e.

  ⟨DV[u][v], w⟩ = ⟨v, DV[u][w]⟩    ∀ admissible v, w.            (2.2)

This is the **Helmholtz symmetry condition** (Olver 1986
*Applications of Lie Groups to Differential Equations*, Springer,
Chapter 5 on variational symmetries; classical inverse-problem-of-
the-calculus-of-variations material). If (2.2) fails, no F exists
with V = δF/δu under the fixed inner product.

For a quadratic F[u] = (1/2)⟨u, A u⟩ with A self-adjoint,
δF/δu = A u and DV[u][v] = A v which is automatically symmetric
under ⟨·,·⟩ by self-adjointness. So *every* quadratic functional
yields a *linear* variational gradient with automatically-symmetric
Fréchet derivative. The candidate's F is quadratic (kinetic +
enstrophy + bi-Laplacian), so its δF/δu = u + νλ(−Δ)u + ν²μ(−Δ)²u
is linear in u — this is the candidate's "(3.7)" gradient flow,
which is a hyperviscous Stokes equation, **not** NS.

### §2.2 The convective Fréchet derivative

The NS convective term V_conv[u] := (u·∇)u is **bilinear** in u.
Its Fréchet derivative at u in direction v is

  DV_conv[u][v] = d/dε|_{ε=0} ((u + εv)·∇)(u + εv)
              = (v·∇)u + (u·∇)v.                                (2.3)

The right-hand side decomposes into two pieces with structurally
distinct roles:

- **(u·∇)v**: the "transport" piece. Under the L² inner product
  on div-free fields with decay, ∫((u·∇)v)·w dx = −∫ v·((u·∇)w) dx
  by integration by parts (using div u = 0 and ∂_j(u_j v_i w_i)
  integrating to zero). This piece is **antisymmetric** under
  swap (v ↔ w) — i.e. it would satisfy ⟨(u·∇)v, w⟩ + ⟨v, (u·∇)w⟩
  = 0, the *opposite* of the Helmholtz symmetric form (2.2).
- **(v·∇)u**: the "stretching" piece. There is **no integration-
  by-parts identity** that converts ⟨(v·∇)u, w⟩ into ⟨v, (w·∇)u⟩
  for general u — the gradient ∇u acts on different perturbation
  directions on the two sides, and there is no symmetric form
  identity.

### §2.3 The symmetric / antisymmetric split and Λ²(ℝ³)

The general algebraic identity (Helmholtz decomposition for the
*Fréchet-derivative* of a bilinear form, not for the velocity
field — both happen to be due to Helmholtz) splits any bilinear
operation into its symmetric and antisymmetric parts:

  (1/2)[(v·∇)u + (u·∇)v] = symmetric in (u, v)                   (2.4a)
  (1/2)[(v·∇)u − (u·∇)v] = antisymmetric in (u, v).              (2.4b)

The antisymmetric piece (2.4b) admits the explicit identity (e.g.
Majda-Bertozzi 2001 *Vorticity and Incompressible Flow*, Cambridge,
Chapter 1, vector calculus identities)

  (v·∇)u − (u·∇)v = (curl u) × v − (curl v) × u + ∇(u·v)
                   ≡ ω × v − (curl v) × u   (mod gradient).     (2.5)

Modulo gradient terms (which the Leray projector annihilates),
the antisymmetric piece of DV_conv[u][v] is **(curl u) × v = ω × v**
— directly a Λ²(ℝ³)-valued object, since ω = curl u takes values
in Λ²(ℝ³) ≅ ℝ³ (the Hodge dual identification of vorticity with
an antisymmetric 2-tensor on 3-space).

### §2.4 Helmholtz failure as Λ²-asymmetry

Combining §2.2 and §2.3:

- The transport piece (u·∇)v in DV_conv[u][v] is L²-antisymmetric
  by integration by parts on div-free u. This is *worse than*
  Helmholtz symmetric — it has the *opposite* sign.
- The stretching piece (v·∇)u in DV_conv[u][v] has no L²-symmetric
  identity; its antisymmetric component (in (u,v)) lives in Λ²(ℝ³)
  via (2.5) and equals the vorticity-cross-product ω × v.

The Λ² antisymmetry IS the variational dual of the BKM 1984
vortex-stretching algebra. Specifically, applying the curl operator
to (2.3) and pairing with ω gives

  ⟨curl(DV_conv[u][v]), ω⟩ = ⟨(ω·∇)u·v⟩ + (transport)            (2.6)

after standard vorticity-velocity manipulations (Beale-Kato-Majda
1984 *Comm. Math. Phys.* 94, eq. (1.4) records the vortex-
stretching object (ω·∇)u as the breakdown norm). So the same
algebraic object (the stretching piece (v·∇)u in (2.3)) is

- in EXT-A's variational frame: the asymmetric Fréchet piece that
  fails Helmholtz symmetry;
- in BKM 1984's analytic frame: the vortex-stretching production
  ω·∇u that drives ‖ω‖_{L^∞} growth.

These are **the same algebraic object** seen through two different
function-space pairings (Fréchet-symmetry vs L^∞-norm growth).

### §2.5 Anatomy summary (A)

The Helmholtz obstruction is *structurally* the Λ²(ℝ³) ≅ vorticity
antisymmetry of the convective Fréchet derivative. Cure paths:

- **Non-quadratic F**: a degree-3 F gives a quadratic δF/δu, with
  *totally symmetric* algebraic structure in two slots; (u·∇)u
  has the *wrong* algebraic structure (asymmetric in u, v slots)
  so degree-3 F still fails. Higher-degree fails analogously.
- **Non-flat metric** (field-dependent g): equivalent to absorbing
  the asymmetry into the metric, i.e. a Vasseur-Yu 2016
  Wasserstein-gradient-flow construction. Vasseur-Yu's metric is
  load-bearingly compressible; transfer to incompressible fails
  at the constitutive step (entropy density, candidate spec §2.4).
- Both cure paths leave the EXT-A slot open in principle but
  unrealised in literature as of 2026-04-25.

The mathematical content of the Helmholtz obstruction is:
**the vortex-stretching tensor (ω·∇)u, viewed as a Λ²(ℝ³) →
Λ²(ℝ³) operator on vorticity perturbations, is structurally
asymmetric and cannot be encoded as the variational gradient of
any quadratic functional under any flat metric** (EXT-A §2.4 +
§9.2; Olver 1986 Chapter 5).

---

## §3 Mathematical anatomy of (B) CI representation

### §3.1 The Constantin-Iyer 2008 representation

Constantin-Iyer 2008 (*Comm. Pure Appl. Math.* 61, 330–345) gives
a stochastic Lagrangian representation of NS: for smooth divergence-
free initial data on T³ and viscosity ν > 0, the velocity field
u(x, t) satisfies

  u(x, t) = E[ ∇^T_x A(X^{−1}_t(x), 0) · u_0(X^{−1}_t(x)) ]_{div-free}  (3.1)

where X_t is the **stochastic Lagrangian flow**

  dX_t = u(X_t, t) dt + √(2ν) dW_t,    X_0(x_0) = x_0,            (3.2)

with W_t Brownian in ℝ³ and A the back-to-labels gauge transform.
(3.2) is a transport-by-velocity-plus-Brownian-noise SDE indexed
by the initial Lagrangian label x_0.

### §3.2 The label-indexed family of laws

For each fixed initial label x_0 ∈ ℝ³ and fixed t > 0, the random
variable X_t(ω, x_0) has a law ρ_t(· | x_0) on ℝ³. Under standard
SDE theory (Stroock-Varadhan 1979 *Multidimensional Diffusion
Processes*, Springer; Hörmander hypoellipticity for the generator
of (3.2)), this law has a smooth L¹-density on ℝ³ for t > 0.

The collection {ρ_t(· | x_0)}_{x_0 ∈ ℝ³} is the **conditional law
family**. CI 2008 §2 uses this family throughout, with x_0 carried
explicitly as a Lagrangian-label parameter.

### §3.3 The candidate's "single ρ_t" requirement

The W_NS functional integrates a *single* density ρ_t on ℝ³:

  W_NS = ∫_{ℝ³} [τν|∇u|² + τ|∇log ρ|² + log ρ] ρ dx + γνt.        (3.3)

This requires collapsing the family {ρ_t(· | x_0)} to a single
ρ_t(x). CI 2008 does **not** privilege any such collapse. Three
natural choices (EXT-B §6.4):

- **(A) mixed-initial-distribution**:
  ρ_t(x) := ∫ ρ_t(x | x_0) μ_0(x_0) dx_0
  for some initial measure μ_0 on labels. Different μ_0 give
  different ρ_t.
- **(B) pinned label**: ρ_t(x) := ρ_t(x | x_0 = x_*) for some
  canonical x_* ∈ ℝ³ (e.g. origin). Highly choice-dependent.
- **(C) velocity-mixed**: ρ_t := law of u(X_t, t) viewed as a
  random variable on ℝ³. Requires a normalisation since u is not
  a probability density.

Each choice produces a *different* W_NS with a *different*
monotonicity question. None is "natural" in the CI 2008 sense.

### §3.4 The Path-Q cross terms

Given any choice (A)/(B)/(C), differentiate W_NS along the coupled
NS + CI flow. Term-by-term (EXT-B §4):

- T_kin = ∫ τν|∇u|² ρ dx: produces ∂_t|∇u|² which by NS
  ∂_t u = νΔu − (u·∇)u − ∇p contains the **vortex-stretching
  term** ∫(ω·∇)u · ω ρ dx, sign-indefinite (this IS the NS
  regularity obstruction itself, EXT-B §8.3).
- T_Fish = ∫ τ|∇log ρ|² ρ dx: under ∂_t ρ = −∇·(uρ) + νΔρ
  (forward Kolmogorov from (3.2)), the Otto-2001 monotonicity
  calculation produces a **cross term**

  −2 ∫ Hess(log ρ) : ∇u · ρ dx                                   (3.4)

  whose sign is uncontrolled by Bakry-Émery 1985 because the
  generator L = νΔ − u·∇ is **non-symmetric** under the ρ-weighted
  inner product (u is divergence-free but the time-dependent
  drift breaks ρ-symmetry; there is no curvature-dimension
  CD(K, ∞) on the augmented configuration).
- T_ent = ∫ (log ρ) ρ dx: clean H-theorem-style decay
  d/dt T_ent = −ν · I(ρ) ≤ 0 (Fisher-information dissipation);
  this is the **only** sign-controlled piece.

### §3.5 The vortex-stretching tensor invariant

The Path-Q cross term ∫(ω·∇)u·ω ρ dx, when computed in components,
reads

  ∫ ω_i (∂_j u_i) ω_j ρ dx = ∫ ⟨ω, S ω⟩ ρ dx                      (3.5)

where S = (1/2)(∇u + ∇u^T) is the strain-rate tensor (the
symmetric part of ∇u) and the antisymmetric part contributes zero
when sandwiched between two ω's (since ⟨ω, A ω⟩ = 0 for A
antisymmetric). The integrand

  ⟨ω, S ω⟩ = ω · S · ω                                            (3.6)

is the **vortex-stretching tensor invariant** — the rate at which
strain stretches vorticity along its own direction. This is the
exact same object that appears:

- in BKM 1984's enstrophy-growth equation:
  d/dt (1/2)‖ω‖² = −ν‖∇ω‖² + ∫⟨ω, S ω⟩ dx;
- in Constantin-Fefferman 1993's geometric-depletion analysis:
  ω · S · ω = |ω|² ξ · S · ξ with ξ = ω/|ω| the vorticity
  direction;
- in EXT-A §2.4's variational dual via the curl-pairing identity
  ⟨curl(DV_conv[u][v]), ω⟩ ⊃ ⟨(ω·∇)u, v⟩.

So EXT-B's Path-Q cross term **is** the BKM enstrophy-growth
production term, weighted by the CI density ρ.

### §3.6 The Hess(log ρ):∇u cross term

The second cross term (3.4) couples the Hessian of log ρ to the
velocity gradient ∇u. Decompose ∇u = S + Ω with S symmetric
(strain rate), Ω antisymmetric (vorticity):

  Hess(log ρ) : ∇u = Hess(log ρ) : S + Hess(log ρ) : Ω.           (3.7)

The Ω-piece vanishes (Hess is symmetric, contraction with
antisymmetric Ω is zero), so

  Hess(log ρ) : ∇u = Hess(log ρ) : S.                             (3.8)

The strain rate S is the *Sym² part* of ∇u (axis-A object,
pressure-coupled via ∂_i ∂_j p = −∂_i ∂_j (u_k u_l) trace);
Hess(log ρ) is determined by the law's geometry. In Otto 2001's
clean PME case, S = 0 (no advection) and the cross term is absent.
In NS, S is generically non-trivial and Hess(log ρ) is determined
by the integral curves of the velocity — there is no a-priori
sign relation.

The Hess(log ρ):S cross term is the **density-strain coupling**;
it has no name in the published literature (EXT-B §10.2: "the
cross term has no name in the Bakry-Émery / Otto / CI literatures
... no prior work tried to combine these specific inputs").

### §3.7 Anatomy summary (B)

The CI representation obstruction is *structurally* a frame-mismatch
between Lagrangian (label-indexed, path-space) and Eulerian
(single-density-on-ℝ³) descriptions of the stochastic flow, with a
secondary failure at the cross-term level once any frame is fixed.

The mathematical content of the representation obstruction is:
**the NS-CI flow is non-symmetric under the natural ρ-weighted
inner product, so Bakry-Émery Γ_2 calculus cannot control the
two cross terms (vortex-stretching ω·S·ω + density-strain
Hess(log ρ):S); the first cross term is exactly the BKM enstrophy-
growth production**.

---

## §4 Mathematical anatomy of (C) Canonical-neighbourhood

### §4.1 The Perelman 2003 surgery + extinction package

Perelman 2003a (arXiv:math/0303109 "Ricci flow with surgery on
three-manifolds") and 2003b (arXiv:math/0307245 "Finite extinction
time for the solutions to the Ricci flow on certain three-
manifolds") together prove the Poincaré conjecture via three
mathematically distinct ingredients (Cao-Zhu 2006 *Asian J. Math.*
10; Morgan-Tian 2007 *Ricci Flow and the Poincaré Conjecture*,
Clay Math. Inst., expand the exposition):

**(M3a) Parameter cascade**: surgery intervals carry parameters
(δ_i, r_i, h_i) — surgery-precision δ_i, canonical-neighbourhood
radius r_i, surgery-scale h_i — with consistency relations
δ_i < δ_0(r_i, ε), h_i < h_0(r_i, δ_i) propagated across surgery
levels.

**(M3b) Canonical-neighbourhood theorem** (Perelman 2003a §4):
every point of high-curvature in a Ricci-flow solution lies in a
canonical neighbourhood of one of three geometric types — an
**ε-thin neck** (cylinder S² × I), an **ε-cap** (capped half-cylinder),
or an **ε-horn** (degenerating cone). This is a *geometric-content*
theorem: it identifies the *shape* of high-curvature regions with
explicit geometric primitives.

**(M3c) Finite extinction** (Perelman 2003b; reproved by
Colding-Minicozzi 2008 *J. Amer. Math. Soc.* 21, 561–569 via
width-of-2-spheres min-max sweep-out): on simply-connected closed
3-manifolds, Ricci flow with surgery becomes extinct in finite time.

The three pieces are **mathematically distinct** and *each* is
necessary: (M3a) without (M3b) gives no surgery rule; (M3b)
without (M3c) gives no termination; (M3c) without (M3b) gives no
controlled cuts.

### §4.2 What QPC-Surgery transfers from Perelman

QPC-Surgery candidate (EXT-C §1) constructs:

- **parabolic-cell partition** Q_{k,j} = [t_j, t_j + 2^{−2k}] ×
  B(x_j, 2^{−k}) at level k (parabolic ratio temporal 2^{−2k} :
  spatial 2^{−k}, eq. 1.2 of EXT-C);
- **flagging rule** (1/r_k) ∫∫_{Q_{k,j}}|∇u|² ≥ ε_* · 2^{αk} with
  ε_* the CKN 1982 universal constant;
- **conjectural cardinality cascade** #F_k ≤ C(α) · 2^{(2−α)k};
- **conjectural (T1) termination** F_{k_*} = ∅ at finite k_*.

The transfer from Perelman M3 maps:
- Perelman M3a parameter cascade ↔ QPC's α-cascade #F_k bound;
- Perelman M3c finite extinction ↔ QPC's (T1) finite-step;
- Perelman M3b canonical-neighbourhood theorem ↔ ???.

### §4.3 The missing third piece in NS

The QPC procedure transfers M3a (parameter cascade) and M3c
(finite-step termination) but **not M3b** (canonical-neighbourhood
recognition theorem). The NS literature provides analogs of M3a
and M3c at the singular-set-size level:

- **CKN 1982 ε-regularity** (*Comm. Pure Appl. Math.* 35,
  771–831): universal ε_* with (1/r)∫∫_{Q_r}|∇u|² < ε_* ⇒ regular;
  parabolic Hausdorff dim of singular set ≤ 1. Provides M3a
  (parameter cascade via ε-regularity propagation) and partial
  M3c (Hausdorff bound on singular set).
- **Lin 1998** (*Comm. Pure Appl. Math.* 51, 241–257): compactness
  + blow-up reproof of CKN, methodologically distinct.
- **Vasseur 2007** (*NoDEA* 14, 753–785): De Giorgi-style L² → L^∞
  truncation cascade M_k → M_{k+1}; this is a parameter-cascade in
  *amplitude*, not in cell-cardinality.

What is **missing** is M3b: a **canonical-neighbourhood theorem
for NS** identifying the geometric shape of high-vorticity regions
with explicit geometric primitives. The literature has only
*partial* candidates:

- **vortex tubes** (Constantin-Procaccia-Segel 1995 *Phys. Rev. E*
  51, geometric description of vortex-line bundles): vortex tubes
  *are* the natural geometric primitive of NS high-vorticity
  regions, but their structure is not characterized at the
  Perelman-M3b strength (no theorem of the form "every high-
  vorticity region IS a vortex tube of one of finitely many types").
- **Constantin-Fefferman 1993 alignment**: ξ = ω/|ω| Lipschitz on
  supp(ω) provides geometric depletion. This is a *hypothesis* on
  ξ, not a *theorem* identifying ξ-shape.

### §4.4 The F-EXTC-D firing — T1 ⟺ NS regularity

Without M3b (canonical-neighbourhood theorem), the QPC (T1)
termination on general 3D smooth data is **logically equivalent**
to NS regularity itself (EXT-C §6.2). The forward direction:

- if F_{k_*} = ∅ at finite k_*, every spacetime point is enclosed
  in a non-flagged cell of size r_{k_*};
- by CKN ε-regularity, every point is regular;
- therefore the singular set is empty;
- modulo the standard CKN ε-regular-to-smooth upgrade
  (Caffarelli-Kohn-Nirenberg 1982 Prop. 1 + Serrin 1962 *Arch.
  Rat. Mech. Anal.* 9), u is smooth.

The reverse direction:

- if u is smooth on [0, T] × ℝ³, |∇u|² is continuous and bounded;
- the local parabolic-energy density (1/r_k) ∫∫_{Q_k}|∇u|² is
  O(r_k^4) by Lebesgue dominated convergence;
- so for k large enough, every cell is non-flagged: F_{k_*} = ∅.

So **(T1) ⟺ NS regularity** modulo standard upgrades. The QPC
procedure relabels the standing open problem as "finite-step
elimination of flagged cells"; it does not advance it (EXT-C §6.3).

### §4.5 Why M3b is missing — the algebraic content

The deep reason M3b is missing in NS, from the structural point
of view: M3b in Ricci flow uses the *intrinsic geometric structure*
of the metric tensor g_{ij}(t) — the high-curvature region's
*shape* is an attribute of g itself. NS has no analogous intrinsic
geometric tensor; the velocity u is a *vector field*, not a metric,
and its high-vorticity region's "shape" must be inferred from
*derived* geometric objects (vortex tubes, vorticity-direction
field ξ, strain-rate eigenframe).

Specifically: in Ricci flow, the canonical-neighbourhood theorem
is statements about the *Riemannian curvature tensor* Rm at a
high-curvature point — the tensor itself encodes the shape. In
NS, the analog candidate is the *velocity-gradient tensor* ∇u,
which decomposes into S + Ω (strain + vorticity); the shape of
the high-vorticity region depends on the *interaction* between S
and Ω, specifically the eigenstructure of S and the alignment of ξ
with S's eigenvectors.

The algebraic content of "what the canonical neighbourhood would
have to recognize" in NS reduces to: **the alignment / mis-
alignment of ξ with the strain-rate eigenframe near a high-
vorticity point** — which is exactly the content of the
**vortex-stretching tensor invariant** ⟨ω, S ω⟩ = |ω|² ξ · S · ξ
of (3.6). Constantin-Fefferman 1993 proves regularity *under the
hypothesis* that ξ is geometrically coherent (Lipschitz);
canonical-neighbourhood recognition for NS would mean *proving* a
shape-theorem on ξ, not assuming it.

### §4.6 Anatomy summary (C)

The canonical-neighbourhood obstruction is *structurally* the
absence of an NS analog of Perelman's geometric-shape recognition
theorem (M3b). The transferable ingredients (parameter cascade
M3a, finite extinction M3c) reduce to relabelings of CKN +
Vasseur-De-Giorgi machinery; the *non-transferable* ingredient
(geometric-content M3b) is exactly the recognition of vortex-tube
geometry, which depends on the alignment of ξ with the strain-rate
eigenframe — i.e. on the **vortex-stretching tensor invariant
⟨ω, S ω⟩**.

The mathematical content of the canonical-neighbourhood obstruction
is: **NS lacks a geometric-shape theorem on high-vorticity regions
strong enough to enable rigorous surgery; the missing ingredient
is exactly a shape-control statement on the vortex-direction field
ξ near high-|ω| points, which is the same algebraic structure
that drives BKM enstrophy growth (the strain-vortex alignment
ξ · S · ξ)**.

---

## §5 Common structure: vortex-stretching tensor as unified
       singularity

### §5.1 The single algebraic object across the three
       obstructions

Each EXT obstruction, when its "anatomy" is read at the algebraic
level, identifies the *same* algebraic object as the locus of
failure:

| EXT | obstruction | algebraic object |
|-----|-------------|------------------|
| A | Helmholtz (variational dual) | antisymmetric Fréchet piece (v·∇)u in DV_conv[u][v]; under curl-pairing, becomes (ω·∇)u — the **vortex-stretching tensor** acting on Λ²(ℝ³) ≅ vorticity |
| B | representation (cross term) | Path-Q cross term ∫(ω·∇)u·ω ρ dx = ∫⟨ω, Sω⟩ ρ dx — the **vortex-stretching tensor invariant** weighted by CI density |
| C | canonical-neighbourhood (missing M3b) | absent geometric-shape theorem on ξ-alignment near high-|ω| points; the missing ingredient is shape-control on the **vortex-stretching tensor invariant** ⟨ω, Sω⟩ = |ω|² ξ·S·ξ |

The three obstructions are **three readings** of the same
mathematical object:

- in EXT-A's variational frame: the asymmetric Fréchet piece in
  the convective derivative IS the vortex-stretching algebra
  (EXT-A §9.2);
- in EXT-B's stochastic-Lagrangian frame: the Path-Q cross term IS
  the BKM enstrophy-growth production (3.5)–(3.6);
- in EXT-C's procedure-class frame: the missing canonical-
  neighbourhood ingredient IS shape-control on ξ near high-|ω|
  points, controlled by ⟨ω, Sω⟩.

### §5.2 The three frameworks each require a different "good
       behaviour" of the vortex-stretching tensor

Each framework family requires the vortex-stretching tensor to
behave in a specific structurally distinct way that NS does not
provide:

- **(A) variational frame** requires: D V_conv to be **L²-symmetric**
  (Helmholtz condition (2.2)). NS provides: D V_conv has Λ²-
  antisymmetric piece (vorticity-cross-product ω × v).
- **(B) stochastic-Lagrangian / Bakry-Émery frame** requires: the
  cross term ⟨ω, Sω⟩ to be **signed** (positive or negative-
  definite under ρ-weighted inner product). NS provides: ⟨ω, Sω⟩
  is sign-indefinite — it is positive when ξ aligns with the
  positive-eigenvalue eigenvector of S, negative when ξ aligns
  with the negative-eigenvalue eigenvector. (Constantin 1994
  *SIAM Rev.* 36 records the empirical observation that ξ
  preferentially aligns with the *intermediate* eigenvector of S,
  giving statistical-but-not-pointwise depletion.)
- **(C) procedure-class frame** requires: a **shape-theorem** on
  ξ identifying the geometric primitive (vortex tube) of high-|ω|
  regions. NS provides: only Constantin-Fefferman 1993 *hypothesis*
  on ξ Lipschitz; no shape-theorem.

Each framework's required-but-absent property is a **different
structural property** of the same tensor (symmetry / signedness /
shape-control). The three obstructions are not three independent
attacks; they are three structurally distinct probes of the same
mathematical object, each finding it deficient in a different
specific way.

### §5.3 Λ²(ℝ³) as the unified singularity

The unifying algebraic frame is **Λ²(ℝ³)**, the space of
antisymmetric 2-tensors on ℝ³ — equivalently, the space where
vorticity ω = curl u takes values via the Hodge dual.

- in EXT-A: the failed Helmholtz symmetry is at the Λ² level —
  (2.4b) is a Λ²-valued antisymmetric piece, (2.5) realises it as
  ω × v.
- in EXT-B: the cross term ⟨ω, Sω⟩ pairs ω ∈ Λ²(ℝ³) against the
  strain S ∈ Sym²(ℝ³); the pairing is the natural Λ²-Sym² coupling.
- in EXT-C: the missing canonical-neighbourhood theorem is on
  ξ ∈ Λ²(ℝ³) (vorticity-direction field, viewed as a section of
  the unit-Λ² bundle).

The 3-pillar synthesis (parent report
`omega-meta-synthesis-3pillar-obstruction-localization-2026-04-25.md`
§2.3) had already collapsed the 3-difficulty / 3-pillar map onto a
2-difficulty / 3-pillar map with axis-B obstruction = "Λ²
antisymmetry = convective-asymmetry = vortex-stretching". The
present synthesis sharpens this: **the Λ² antisymmetry is the
vortex-stretching tensor (ω·∇)u acting on vorticity, and the three
EXT obstructions are three structurally distinct probes of the
same Λ²-valued operator finding it deficient in three different
specific ways**.

This is the **mathematical content of axis-B localization**: not
just "axis B is hard", but *axis B is hard because the vortex-
stretching tensor (a Λ²(ℝ³)-valued operator) lacks three specific
structural properties simultaneously — symmetry under L² pairing,
signedness under ρ-weighted pairing, and shape-control near high-
|ω| points*.

---

## §6 BKM 1984 unification: three frameworks as complementary
       BKM perspectives

### §6.1 Beale-Kato-Majda 1984

Beale-Kato-Majda 1984 (*Comm. Math. Phys.* 94, 61–66, "Remarks on
the breakdown of smooth solutions for the 3-D Euler equations"):
a smooth NS (or Euler) solution u on [0, T*) with T* < ∞ extends
past T* if and only if

  ∫_0^{T*} ‖ω(t)‖_{L^∞(ℝ³)} dt < ∞.                              (BKM)

The criterion says **the L^∞ time-integral of vorticity is the
breakdown norm**. Whether (BKM) holds is the open question;
whether it would imply NS regularity is a theorem.

### §6.2 BKM as the *minimal* statement about vortex-stretching

BKM is the **minimal** statement saying "vortex-stretching is the
NS regularity obstruction". The vortex-stretching tensor (ω·∇)u
drives the enstrophy-growth equation

  d/dt (1/2)‖ω‖_{L²}² = −ν‖∇ω‖_{L²}² + ∫⟨ω, Sω⟩ dx               (6.1)

and the L^∞ vorticity grows under the same stretching mechanism.
The (BKM) criterion records: *if* ‖ω‖_{L^∞} stays integrable,
then the stretching does not blow up; *otherwise* it does, and
this dichotomy is exhaustive.

BKM does not say *how* to control ‖ω‖_{L^∞}; it only identifies
the relevant norm. Subsequent work has refined the identification:

- **CF 1993** (*Indiana Univ. Math. J.* 42, 775–789): under the
  hypothesis that ξ = ω/|ω| is Lipschitz-coherent on supp(ω),
  the stretching production ⟨ω, Sω⟩ is *geometrically depleted*
  and ‖ω‖_{L^∞} stays finite. This is a sufficient condition for
  (BKM) finiteness.
- **Constantin-Fefferman-Procaccia 1996** (*Comm. PDE* 21,
  559–571): extension to Euler under related geometric hypotheses.

### §6.3 The three EXT obstructions are complementary BKM
       perspectives

Reading the three EXT obstructions through the BKM lens:

**EXT-A (variational dual of BKM)**: the Helmholtz failure at
DV_conv[u][v] = (v·∇)u + (u·∇)v is precisely the variational dual
of BKM's stretching algebra. The asymmetric piece (v·∇)u, under
curl-pairing with ω, becomes (ω·∇)u — the same operator BKM
identifies as the breakdown driver. EXT-A says: *the variational
gradient-flow framework cannot encode (u·∇)u under flat metric
because doing so would require (u·∇)u to be Helmholtz-symmetric,
which is exactly the property that (ω·∇)u lacks under L² pairing —
i.e. variational encodability of NS is equivalent to a structural
property that vortex-stretching does not have*.

**EXT-B (stochastic-Lagrangian dual of BKM)**: the Path-Q cross
term ∫⟨ω, Sω⟩ ρ dx is precisely the BKM enstrophy-growth
production weighted by the CI density. EXT-B says: *the Bakry-
Émery / Otto framework cannot produce a Lyapunov for NS-CI flow
because doing so would require the cross term ⟨ω, Sω⟩ to be
sign-controlled under ρ-weighted pairing, which is exactly the
property that BKM's stretching-production lacks — i.e. analytic
Lyapunov for NS-CI is equivalent to a structural property that
vortex-stretching does not have*.

**EXT-C (procedure-class dual of BKM)**: the missing canonical-
neighbourhood theorem on high-|ω| regions is precisely the missing
shape-recognition of BKM's stretching geometry. EXT-C says: *the
Perelman M3 surgery + extinction template cannot transfer to NS
because doing so would require a shape-theorem on vorticity-
direction near high-|ω| points, which is exactly the property
that BKM-driven NS lacks — i.e. quantitative-procedure-termination
for NS is equivalent to a structural property that vortex-tube
geometry does not have*.

### §6.4 The three frameworks are not independent attacks

The three EXT frameworks (variational / stochastic-Lagrangian /
procedure-class) share no common machinery beyond divergence-free
hypothesis (3-pillar synthesis §3, parent report). Their
convergent obstruction onto the same algebraic object is therefore
**a non-trivial structural agreement** — they are three
*complementary perspectives* on BKM, each probing a different
structural property of the vortex-stretching tensor:

- **EXT-A** probes *symmetry* (Helmholtz, L²-pairing);
- **EXT-B** probes *signedness* (Bakry-Émery, ρ-weighted pairing);
- **EXT-C** probes *shape-recognition* (canonical neighbourhood,
  geometric primitive).

All three find the vortex-stretching tensor deficient in their
respective property. This is **structurally consistent** — and
**mathematically informative**: it says the three framework
families are *complementary perspectives on BKM*, not independent
attacks. The "axis-B localization" of the parent 3-pillar synthesis
is sharpened to "BKM unification": every framework family that
attempts a frame-shift on NS reduces to BKM at the bottleneck.

### §6.5 The honest content of "BKM unification"

The BKM unification claim is **structural diagnosis**, not a
theorem. It says:

- (a) the three EXT obstructions identify three structurally
  distinct properties (symmetry / signedness / shape-recognition)
  that NS would need for the respective framework family to work;
- (b) all three properties refer to the same underlying algebraic
  object (the vortex-stretching tensor (ω·∇)u acting on Λ²(ℝ³));
- (c) BKM 1984 is the minimal statement that vortex-stretching is
  the NS regularity obstruction;
- (d) therefore, the three EXT obstructions are three
  *complementary refinements* of BKM rather than three independent
  attacks.

(a)–(d) are **consistent with** the three EXT validations and
**not proved by** them (since each EXT verdict is on its own
candidate, not on a cross-framework theorem). The unification is
useful as a **research-direction map**: future EXT candidates that
attempt a different framework family (say, geometric-measure-
theoretic, harmonic-analytic, or Hamiltonian-dynamical) should
expect to hit BKM at *some* structural bottleneck, and the three
framework families above already enumerate the natural candidates.

---

## §7 CF-BMO bridge: what would unblock

### §7.1 The Constantin-Fefferman 1993 unblocking direction

If all three EXT frameworks fail at the BKM bottleneck (vortex-
stretching tensor deficient in symmetry / signedness / shape-
control), the natural unblocking direction is to **add a hypothesis
that controls the vortex-stretching tensor directly**. The
literature precedent is Constantin-Fefferman 1993:

  **CF 1993 hypothesis**: ξ = ω/|ω| is Lipschitz-coherent on
  supp(ω) with modulus controlled by |ω|^{−1}.

Under (CF), the strain-vortex alignment ξ · S · ξ is *geometrically
depleted*: the stretching production ⟨ω, Sω⟩ = |ω|² ξ · S · ξ is
controlled relative to the dissipation ν‖∇ω‖_{L²}², and BKM
finiteness ∫_0^{T*} ‖ω‖_{L^∞} dt < ∞ follows by Gronwall.

CF 1993 + BKM 1984 *would* yield NS regularity *if* the CF
Lipschitz hypothesis held on actual NS solutions. The hypothesis
is **conjectured** but **not verified** at this regularity tier —
Constantin 1994 (*SIAM Rev.* 36) reports statistical alignment
(ξ aligns with the *intermediate* eigenvector of S empirically),
not pointwise Lipschitz coherence.

### §7.2 The CF-BMO candidate

The axis-B targeted attack
(`omega-exec-bt544-axisB-targeted-attack-2026-04-25.md`) generated
a candidate **CF-BMO** that relaxes the CF Lipschitz hypothesis to
BMO (bounded mean oscillation):

  **CF-BMO hypothesis** (axis-B-attack §3.2 eq. 3.2):

  ‖ξ(·, t)‖_{BMO(supp(ω))} ≤ C_BMO · log(2 + ‖ω(·, t)‖_{L^∞}).    (7.1)

with C_BMO depending only on data u_0.

**Conjectural conclusion** (axis-B-attack §3.2 eq. 3.3): under
(7.1), the stretching production admits a logarithmically-corrected
geometric depletion

  d/dt (1/2)‖ω‖_{L²}² ≤ −ν‖∇ω‖_{L²}² + κ‖ω‖_{L²}² log(2 + ‖ω‖_{L^∞})  (7.2)

with κ a constant depending on (ν, ‖u_0‖_{H^s}, C_BMO). Combined
with BKM, this would yield global smoothness via double-exponential
Gronwall.

The CF-BMO candidate is **the natural axis-B-targeted candidate
the three EXT obstructions point toward**:

- it *engages directly* with the vortex-stretching tensor
  (controlling ξ on supp(ω));
- it *relaxes* the CF 1993 Lipschitz hypothesis to BMO — a
  rigorous harmonic-analytic relaxation (John-Nirenberg 1961,
  Sarason 1975 VMO);
- it sits *strictly between* "ξ ∈ L^∞ trivially" (no depletion,
  EXT-B Path-Q regime) and "ξ Lipschitz" (full CF depletion).

### §7.3 What CF-BMO would resolve and what it would not

If CF-BMO held (i.e. (7.1) verified on actual NS solutions and
(7.2) derived rigorously), the three EXT obstructions would be
**locally unblocked but not globally cured**:

- **EXT-A (Helmholtz)**: not directly unblocked. The variational
  encodability of (u·∇)u under flat metric remains structurally
  impossible (Olver 1986 Chapter 5 algebra is unconditional). What
  CF-BMO provides is *control on the vortex-stretching tensor*
  separate from the variational framework, so the EXT-A slot
  would be replaced by a non-variational analytic-control route.
- **EXT-B (representation)**: partially unblocked. The Path-Q
  cross term ⟨ω, Sω⟩ ρ would be controllable under (7.2),
  resolving the secondary obstruction. The primary representation
  obstruction (CI 2008 produces {ρ_t(·|x_0)}, not single ρ_t)
  remains and requires an independent fix (axis-B-attack §3.4
  honest scope-limitation).
- **EXT-C (canonical-neighbourhood)**: partially unblocked. The
  missing M3b shape-theorem would be replaced by a *quantitative
  BMO bound* on ξ — not a shape-recognition theorem in the
  Perelman sense, but a quantitative-control substitute. The
  procedure-termination question would still need separate
  algebra (shifting the cardinality cascade #F_k bound under
  (7.1) ξ-control), but the canonical-neighbourhood analog would
  exist.

### §7.4 Status of CF-BMO as of 2026-04-25

The CF-BMO candidate is **registered**, with discriminator and
falsifiers specified (axis-B-attack §3, §5, §6 of that report).
It has **not been validated**; the axis-B-attack §6 explicitly
anticipates F-AXISB-B firing (the BMO-coherence hypothesis (7.1)
is itself conjectural and probably as hard as the original Clay
problem on the NS solutions where it would be needed).

The CF-BMO is the *direct* axis-B attack the three EXT
obstructions point toward, in the same way the three EXT
obstructions are themselves complementary BKM refinements. It is
the next obvious candidate but is not expected to close BT-544;
its value is in the precision of the registered hypothesis (7.1)
relative to the unregistered "vortex-stretching is hard" folk
intuition.

---

## §8 Anti-list: alternative unifications considered

This section records alternative unifications considered for the
3 EXT obstructions and the structural reasons each was not
adopted. The list is informational; none of these alternatives
would change the per-EXT verdicts.

### §8.1 Alternative 1: pressure-non-locality unification

**Alternative**: the three obstructions all reduce to pressure
non-locality (∇p = ∇(−Δ)^{−1} ∇·∇·(u⊗u), Riesz-transform of the
quadratic NS nonlinearity).

**Rejection**: the 3-pillar synthesis (parent report §2.1) already
established that pressure non-locality is **universally clearable**
across the three pillars: D3.A clears via 2D Calderón-Zygmund;
EXT-B clears via the back-to-labels gauge in CI 2008; EXT-A ties
it into the metric without breaking on it. *No EXT obstruction
locates the failure at pressure non-locality* (3-pillar synthesis
§2.2 first observation). This unification is structurally false.

### §8.2 Alternative 2: energy-cascade (Onsager) unification

**Alternative**: the three obstructions all reduce to anomalous
energy dissipation at threshold regularity C^{1/3} (Onsager 1949;
Eyink-Sreenivasan 2006 *Rev. Mod. Phys.* 78; Isett 2018 *Ann.
Math.* 188).

**Rejection**: Onsager / energy-cascade is axis-C in the D3
mechanism-decouple (`omega-seed-bt544-d3-mechanism-decouple-2026-
04-25.md` §1.2). The D3.C discriminator returned FAIL_INTERMITTENCY
(intermittency corrections deviate from K41), which couples *back*
to axis B via field-geometry but does not *replace* axis B as the
obstruction-carrier. The EXT obstructions are at the variational /
analytic / procedure level, not at the Onsager-threshold level.
Onsager-threshold unification is a different question (sharp
regularity threshold for energy conservation), not the same
question as BKM bottleneck.

### §8.3 Alternative 3: Sobolev-embedding / Riesz-transform L^∞-
       boundedness unification

**Alternative**: the three obstructions reduce to the failure of
Riesz transforms to be L^∞-bounded — the underlying technical fact
behind every "logarithmic loss" in NS estimates.

**Rejection**: Riesz transforms appear in EXT-A's Leray projector
and in EXT-B's CI back-to-labels gauge, but they are *cleared* in
both (Calderón-Zygmund 1952 *Acta Math.* 88 H^s boundedness
suffices). The L^∞-non-boundedness of Riesz transforms is a
secondary issue (driving log-loss in Sobolev embedding) but is not
the *load-bearing* obstruction in any of the three EXT validations.
Furthermore, Riesz-L^∞ failure is a *linear* operator-theoretic
fact about Calderón-Zygmund kernels; the EXT obstructions are
*non-linear* (vortex-stretching is bilinear in u). The unifications
are at different logical levels.

### §8.4 Alternative 4: convex-integration / non-uniqueness
       unification

**Alternative**: the three obstructions reduce to the
convex-integration-permitted non-uniqueness at low regularity
(Buckmaster-Vicol 2019 *Ann. Math.* 189; Cheskidov-Luo 2022).

**Rejection**: convex integration is at the **opposite direction**
of the three EXT candidates (which all attempt regularity-side
frame-shifts, not non-uniqueness-side). BV 2019 is the
"negative-direction" precedent showing weak NS solutions are
non-unique at low regularity; it does not impinge on the smooth-
solution / Clay-question side that EXT-A/B/C all engage with.
Convex integration unification is structurally orthogonal.

### §8.5 Alternative 5: Calderón-Zygmund boundedness on Λ² /
       Sym² unification

**Alternative**: the three obstructions reduce to a specific
Calderón-Zygmund boundedness question for kernels valued in
Λ²(ℝ³) ⊗ Λ²(ℝ³) (the natural target space for a vortex-stretching
operator).

**Rejection**: this is closer to the truth (it correctly identifies
Λ² as the unifying space, §5.3) but is too narrow. The three EXT
obstructions are not all Calderón-Zygmund-boundedness questions:
EXT-A is variational (algebraic Helmholtz), EXT-B is analytic
(cross-term sign), EXT-C is procedure-class (canonical neighbourhood).
The Calderón-Zygmund framing captures the *algebraic shape* of
the unified object (Λ²-valued tensor) but misses the framework-
specific structural properties (symmetry / signedness / shape-
control) each EXT requires of it.

### §8.6 Adopted unification (BKM, §6)

The adopted unification — that the three EXT frameworks are
complementary BKM perspectives, each probing a different structural
property of the same Λ²-valued vortex-stretching tensor — passes
all five rejection tests above:

- it correctly identifies pressure non-locality as cleared (§5.2,
  consistent with 3-pillar synthesis);
- it correctly identifies axis B (vortex-stretching) as the
  obstruction-carrier without confusing it with axis C
  (Onsager-threshold);
- it correctly identifies Λ²(ℝ³) as the unifying space without
  reducing the algebra to linear Riesz-boundedness;
- it correctly distinguishes regularity-side frame-shifts (EXT-A/B/C)
  from negative-direction non-uniqueness (BV 2019);
- it captures the framework-specific structural properties
  (symmetry / signedness / shape-control) that linear Calderón-
  Zygmund framing would miss.

The BKM unification is therefore the *minimal sufficient*
structural reading of the three EXT obstructions. It is consistent
with the parent 3-pillar synthesis and sharpens its axis-B
localization claim from "axis B is where the obstruction lives"
to "axis B is where the obstruction lives, and the three
framework-family attempts are three complementary BKM refinements".

---

## §9 Falsifiers active for the synthesis

Falsifiers under which the §5 / §6 unification claims would be
retracted or downgraded.

### §9.1 F-SYN-A (the three obstructions are NOT the same algebraic object)

**Statement**: if the three EXT obstructions, properly read, do
*not* reduce to the same Λ²-valued algebraic object — e.g. if
EXT-A's Helmholtz failure is genuinely about a *different* tensor
than EXT-B's Path-Q cross term — then §5's unification is
structurally unjustified.

**Status**: **NOT ACTIVE**. The identifications are explicit:
- EXT-A §9.2 + this report §2.4: Fréchet asymmetry IS BKM
  vortex-stretching;
- EXT-B §4.2(b) + §8.3: Path-Q cross is the NS regularity
  obstruction itself;
- EXT-C §6 + this report §4.5: missing M3b is shape-control on ξ
  near high-|ω|.

All three reduce, by the explicit references, to the same
vortex-stretching tensor on Λ²(ℝ³). The risk is low.

### §9.2 F-SYN-B (BKM is not the *minimal* statement)

**Statement**: if there is a strictly weaker statement about
vortex-stretching that the three EXT obstructions all refine —
e.g. a "BKM_∗" criterion at a regularity weaker than L^∞ — then
the BKM unification of §6 is misnamed.

**Status**: **PARTIALLY ACTIVE** in spirit. Refinements of BKM
exist (e.g. Kozono-Taniuchi 2000 *Comm. Math. Phys.* 214 BMO
version, Planchon 2003 weak-L^∞ version); whether one of these is
"closer to" the EXT obstructions than BKM 1984 is open. The
unification is invariant under such refinements (the underlying
object remains the vortex-stretching tensor); the choice of "BKM"
as the label is the canonical / minimal reading. A specialist
re-reading could re-label without changing the structural content.

### §9.3 F-SYN-C (CF-BMO is not the right unblocking direction)

**Statement**: if a candidate other than CF-BMO is a more direct
consequence of the three EXT obstructions — e.g. a stochastic-
quantization / quantum-field-theoretic frame-shift, or a
geometric-measure-theoretic frame-shift — then §7's CF-BMO bridge
is misdirected.

**Status**: **NOT ACTIVE** at the structural level. CF-BMO is the
*literature-precedent* unblocking direction (CF 1993 is the
load-bearing positive-direction axis-B result). Other candidates
are conceivable but lack the literature pedigree. The risk is low
but is flagged for future-candidate generation: if a non-CF axis-B
candidate emerges that engages the three EXT obstructions as
naturally as CF-BMO, the CF-BMO bridge is replaced by the new
candidate's bridge.

### §9.4 F-SYN-D (a missed framework family)

**Statement**: if there is a *fourth* natural framework family
(beyond variational / stochastic-Lagrangian / procedure-class) that
attempts NS frame-shift and that does NOT reduce to BKM at the
bottleneck — i.e. whose obstruction is at a different algebraic
object — then the BKM unification is incomplete.

**Status**: **PARTIALLY ACTIVE**. Candidate fourth frameworks
include: harmonic-analytic (Triebel-Lizorkin / Besov frame-shifts),
Hamiltonian-dynamical (Euler-Hamiltonian + viscous-perturbation),
geometric-measure-theoretic (currents / varifolds for vorticity).
Whether any of these would produce an EXT-tier candidate that
reduces to BKM is open; the present synthesis covers only the
three frameworks the EXT validations actually examined.

### §9.5 F-SYN-E (atlas / state / inventory edit leakage)

**Statement**: if any change is made to atlas, state, or inventory
files as a result of this synthesis, the brief's hard constraint
is violated.

**Status**: **NOT ACTIVE**. This synthesis is research-only and
edits no atlas, state, or inventory file. The git status at session
start (specs and inventory.json modified by *unrelated* prior
sessions per the gitStatus header) is unaffected.

### §9.6 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-SYN-A | three obstructions not same algebra | NOT ACTIVE |
| F-SYN-B | BKM not minimal | PARTIALLY ACTIVE |
| F-SYN-C | CF-BMO not right unblocking | NOT ACTIVE |
| F-SYN-D | missed framework family | PARTIALLY ACTIVE |
| F-SYN-E | atlas/state/inventory leakage | NOT ACTIVE |

Two falsifiers (F-SYN-B, F-SYN-D) are partially active; neither is
expected to change the structural content of the unification, but
both flag refinement directions for future synthesis sessions.

---

## §10 Closing

This synthesis aggregates the mathematical content of the three
EXT-tier validations against BT-544 (3D Navier-Stokes regularity)
that each returned `OBSTRUCTION_DOCUMENTED` with a structurally
distinct named obstruction.

**The three obstructions and their algebraic content** (§§2-4):

- **EXT-A Helmholtz**: D[(u·∇)u][v] = (v·∇)u + (u·∇)v has a Λ²-
  antisymmetric piece that fails Olver-1986-Chapter-5 Helmholtz
  symmetry under L²; equivalently, no quadratic functional under
  any flat metric has (u·∇)u as variational gradient.
- **EXT-B representation**: CI 2008 produces a label-indexed
  family {ρ_t(·|x_0)}_{x_0 ∈ ℝ³}, not a single ρ_t on ℝ³ that the
  candidate W_NS requires; secondary Path-Q cross term ⟨ω, Sω⟩ ρ
  is unsigned by Bakry-Émery 1985 (NS-CI flow non-symmetric, no
  CD(K, ∞)).
- **EXT-C canonical-neighbourhood**: NS lacks Perelman-2003a M3b
  geometric-shape-recognition theorem; CKN 1982 + Lin 1998 +
  Vasseur 2007 transfer the parameter-cascade and finite-step
  termination ingredients but not the geometric-content piece;
  T1 termination on general 3D smooth data ⟺ NS regularity (F-EXTC-D
  fires).

**Common structure** (§5): all three obstructions reduce to the
**vortex-stretching tensor** (ω·∇)u acting on Λ²(ℝ³) ≅ vorticity,
each framework requiring a different structural property
(symmetry / signedness / shape-recognition) that NS does not
provide. The three obstructions are three structurally distinct
probes of the *same* Λ²-valued operator.

**BKM unification** (§6): the three EXT framework families
(variational / stochastic-Lagrangian / procedure-class) are
**complementary refinements of Beale-Kato-Majda 1984**, each
probing a different structural property of the vortex-stretching
tensor. They are not three independent attacks; they are three
perspectives on the same BKM bottleneck.

**CF-BMO bridge** (§7): the natural unblocking direction is to
relax the Constantin-Fefferman 1993 Lipschitz hypothesis on
ξ = ω/|ω| to BMO (axis-B-attack candidate `CF-BMO`,
`omega-exec-bt544-axisB-targeted-attack-2026-04-25.md` §3). CF-BMO
would partially unblock EXT-B (cross-term control) and EXT-C
(canonical-neighbourhood substitute) but not EXT-A (Helmholtz
algebra is unconditional). CF-BMO is registered but not validated
and is not expected to close BT-544; its value is in the precision
of the registered hypothesis relative to the unregistered "vortex-
stretching is hard" folk intuition.

**Anti-list** (§8): five alternative unifications considered and
rejected (pressure-non-locality / Onsager-threshold / Riesz-L^∞ /
convex-integration / Λ²-Calderón-Zygmund). The BKM unification is
the *minimal sufficient* structural reading.

**Falsifiers active for the synthesis** (§9): F-SYN-B and F-SYN-D
partially active (BKM-minimality and missed-framework risks);
others not active.

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.** All cited references are pre-existing
(Olver 1986 Springer; Constantin-Iyer 2008 *Comm. Pure Appl. Math.*
61; Caffarelli-Kohn-Nirenberg 1982 *Comm. Pure Appl. Math.* 35;
Lin 1998 *Comm. Pure Appl. Math.* 51; Vasseur 2007 *NoDEA* 14;
Perelman 2003a arXiv:math/0303109; Perelman 2003b arXiv:math/0307245;
Cao-Zhu 2006 *Asian J. Math.* 10; Morgan-Tian 2007 Clay Math.
Inst.; Colding-Minicozzi 2008 *J. Amer. Math. Soc.* 21;
Beale-Kato-Majda 1984 *Comm. Math. Phys.* 94; Constantin-Fefferman
1993 *Indiana Univ. Math. J.* 42; Constantin-Fefferman-Procaccia
1996 *Comm. PDE* 21; Constantin 1994 *SIAM Rev.* 36; Otto 2001
*Comm. PDE* 26; Bakry-Émery 1985 *Sém. Probab.* XIX;
Otto-Villani 2000 *J. Funct. Anal.* 173; Ambrosio-Gigli-Savaré
2008 Birkhäuser; Bakry-Gentil-Ledoux 2014 Springer; Brenier 1999
*J. Amer. Math. Soc.* 12; Vasseur-Yu 2016 *Invent. Math.* 206;
Stroock-Varadhan 1979 Springer; Constantin-Vicol 2012 *Geom. Funct.
Anal.* 22; Calderón-Zygmund 1952 *Acta Math.* 88; Majda-Bertozzi
2001 Cambridge; Serrin 1962 *Arch. Rat. Mech. Anal.* 9;
Ladyzhenskaya 1969 Gordon-Breach; Kozono-Taniuchi 2000 *Comm.
Math. Phys.* 214; John-Nirenberg 1961 *Comm. Pure Appl. Math.* 14;
Sarason 1975 *Trans. Amer. Math. Soc.* 207; Hamilton 1982
*J. Diff. Geom.* 17; Buckmaster-Vicol 2019 *Ann. Math.* 189;
Isett 2018 *Ann. Math.* 188; Eyink-Sreenivasan 2006 *Rev. Mod.
Phys.* 78; Elgindi 2021 *Ann. Math.* 194).

— end synthesis —
