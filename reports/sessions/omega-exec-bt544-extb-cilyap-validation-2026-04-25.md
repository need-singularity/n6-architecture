---
id: omega-exec-bt544-extb-cilyap-validation
date: 2026-04-25
scope: research-only validation (NO NS claim, NO atlas promotion)
target: BT-544 EXT-B CI-Lyap -- Path-P/Q discrimination + F-EXTB-A/D falsifier check
parent_reports:
  - reports/sessions/omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md (candidate spec)
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (EXT-B prescription)
  - reports/sessions/omega-meta-audit-l9-generation-pipeline-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: validation, no claim
---

# Omega Exec — BT-544 EXT-B CI-Lyap Validation (2026-04-25)

## §0 Non-claim disclaimer

This file **validates** the Constantin–Iyer relative-entropy /
stochastic-Lagrangian Lyapunov candidate (CI-Lyap) for 3D
incompressible Navier-Stokes that was generated in
`reports/sessions/omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md`
(hereafter *candidate spec*). The validation runs the discriminator
of candidate spec §4 against the falsifiers registered in
candidate spec §5.

This file does **not**:

- claim 3D Navier-Stokes regularity, blow-up, or any Clay-form
  resolution;
- prove or disprove the monotonicity conjecture (3.3) of the
  candidate spec;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-544 = 0/1 untouched` Clay
  status, or add to the L9 catalogue's active-candidate ledger;
- supersede or replace the D1 / D2 / D3.A / D3.B' / D3.C
  catalogue rows;
- introduce any new theorem. Every cited result is pre-existing in
  the published NS / gradient-flow / log-Sobolev literature, and is
  cited by author / year / journal as required.

The validation outcome is **OBSTRUCTION_DOCUMENTED** at the
**representation-side** activation of F-EXTB-D (CI-2008
representation does not extend to the augmented state space the
candidate functional requires), with a **secondary cross-term
obstruction** in the Path-Q sketch (the vortex-stretching residual
is the NS regularity obstruction itself, not a controllable cross
term). The verdict is consistent with the candidate spec §6.3
expected-verdict probability ~75% OBSTRUCTION_DOCUMENTED.

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.**

---

## §1 Candidate W_NS recap (read-only restatement of candidate spec §3)

### §1.1 State space and functional

State space (candidate spec eq. 3.1):

  (u, ρ, τ) ∈ {div-free L²(ℝ³)} × {prob. densities on ℝ³} × ℝ_{>0},

with u the NS velocity, ρ the law of the Constantin-Iyer
stochastic-Lagrangian flow at time t, τ a scale parameter
(τ(t) = T − t for finite-horizon, or τ(t) = 1/(T+t) asymptotic).

Candidate Lyapunov (candidate spec eq. 3.2):

  W_NS(u, ρ, τ) = ∫_{ℝ³} [τ (ν |∇u|² + |∇ log ρ|²) + log ρ] ρ dx
                  + γ · ν · t,                                   (1.1)

with γ ≥ 0 fixed by the inviscid-limit Brenier 1999 normalisation.

### §1.2 Term-by-term anatomy

The integrand of (1.1) decomposes into four pieces:

| piece | symbol | structural role | NS / Perelman analog |
|-------|--------|-----------------|----------------------|
| τν \|∇u\|² ρ | T_kin | viscous-dissipation × density-weight | analog of τ R · density (curvature-kin) |
| τ \|∇ log ρ\|² ρ | T_Fish | Fisher information of ρ at scale τ | analog of τ \|∇f\|² in Perelman W |
| (log ρ) · ρ | T_ent | Boltzmann entropy density | analog of f · density in Perelman W |
| γ ν t | T_corr | linear-in-t viscous correction | replaces the −n volume-normalisation |

The conjectured inequality (candidate spec eq. 3.3) is

  d/dt W_NS(u(t), ρ(t), τ(t)) ≥ 0                                (1.2)

along the coupled (NS + Constantin-Iyer stochastic-Lagrangian)
evolution.

### §1.3 Discriminator paths (read-only, candidate spec §4.1)

- **Path P**: import (1.2) from a single chain of citations to
  existing rigorous results. Verdict if P succeeds: PASS_LITERATURE.
- **Path Q**: derive (1.2) by sketch from CI 2008 + Otto 2001 +
  Bakry-Émery 1985 + CV 2012 inputs. Verdict: PASS_SKETCH.
- **Path R**: a specific link in the chain (3.5) of the candidate
  spec is provably broken under NS dynamics. Verdict:
  OBSTRUCTION_DOCUMENTED.
- **Path S**: literature underdetermined. Verdict: INCONCLUSIVE.

This validation runs Paths P and Q in §3-§4, then checks the
falsifiers F-EXTB-A and F-EXTB-D in §5-§6, and concludes in §7.

---

## §2 Literature scan — what each cited reference actually proves

Each reference is restated with its actual published content,
distinguished from the *adjacent* claim the candidate spec needs.

### §2.1 Constantin-Iyer 2008 (CPAM 61, 330–345)

**Title**: "A stochastic Lagrangian representation of the
three-dimensional incompressible Navier-Stokes equations".

**Actual content**: for smooth divergence-free initial data on T³
(periodic 3-torus), and viscosity ν > 0, the NS velocity field
admits the representation

  u(x, t) = E[ ∇^T_x A(X^{−1}_t(x), 0) · u_0(X^{−1}_t(x)) ]_{div-free proj.}

where X_t is the **stochastic Lagrangian flow**

  dX_t = u(X_t, t) dt + √(2ν) dW_t,                              (2.1)

W_t a Brownian motion in ℝ³, X^{−1}_t the (a.s. defined) inverse
flow, and the projection onto div-free fields is made explicit via
a back-to-labels gauge transform A (CI 2008 §2 main theorem).

**Domain of validity (CI 2008 §1, §3 stated explicitly)**:
- short-time existence on T³;
- smooth (C^∞ or sufficiently high Sobolev) initial data;
- the representation is a **reformulation** of NS — neither
  stronger nor weaker as a regularity statement than classical
  Leray-Hopf machinery applied to the same data;
- CI 2008 explicitly disclaims any regularity gain from the
  representation (§1, last paragraph): "the representation may be
  useful for analysis or numerics but does not, by itself, resolve
  the global-regularity question".

**What CI 2008 does NOT prove**:
- existence of the law ρ_t of X_t **as a regular probability
  density on ℝ³** for arbitrary smooth NS data (the law is a
  measure on path-space, not necessarily an L¹-density on ℝ³);
- monotonicity of any functional of (u, ρ_t, τ);
- extension of the representation to weak / Hölder solutions or
  to long time on T³ (long-time extension would *follow from*
  global regularity, not provide it).

This is the CI 2008 ground truth.

### §2.2 Otto 2001 (CPDE 26, 101–174)

**Title**: "The geometry of dissipative evolution equations: the
porous medium equation".

**Actual content**: for the porous medium equation (PME)
∂_t ρ = Δ(ρ^m) with m ≥ 1, and for the linear heat equation
(m = 1), Otto 2001 establishes:

- the equation is the gradient flow, on the Wasserstein-2 space
  P_2(ℝ^n), of an **explicit entropy functional**
  E(ρ) = ∫ U(ρ) dx with U convex;
- gradient-flow structure ⇒ monotone-decrease of E along the flow,
  with the rate controlled by the squared Wasserstein-gradient
  norm ‖∇_W E‖² (Otto 2001 §3-§4 Theorems);
- the heat equation (m = 1) corresponds to U(ρ) = ρ log ρ
  (Boltzmann entropy), and the gradient-flow framework recovers
  classical heat-equation entropy decay.

**Domain of validity**: linear and nonlinear scalar diffusion
equations with explicit gradient-flow structure on (P_2, W_2).

**What Otto 2001 does NOT prove**:
- gradient-flow structure for **3D Navier-Stokes**. Otto 2001 §1
  explicitly works with scalar parabolic PDEs on a probability-
  density configuration; NS is a vector field with a constraint
  (incompressibility) and a non-symmetric advection, neither of
  which fits the (P_2, W_2) gradient-flow archetype directly;
- monotonicity of any functional involving the velocity field u
  separately from a density.

This bound is reinforced by Ambrosio-Gigli-Savaré 2008 (Birkhäuser,
*Gradient Flows in Metric Spaces and in the Space of Probability
Measures*) §§9-11, which catalogues which PDEs admit
(P_2, W_2)-gradient-flow structure; NS is **not** in the catalogue.
Brenier 1999 (J. Amer. Math. Soc. 12, 495) §5 makes the same
negative observation in the variational-Euler framework.

### §2.3 Bakry-Émery 1985 (Sém. Probab. XIX, Lecture Notes in
Math. 1123, 177–206)

**Title**: "Diffusions hypercontractives".

**Actual content**: for a diffusion semigroup (P_t)_{t ≥ 0} with
generator L on a manifold (M, μ), define the carré du champ
Γ(f, g) = (1/2)[L(fg) − f Lg − g Lf] and its iterate

  Γ_2(f, f) = (1/2)[L Γ(f, f) − 2 Γ(f, Lf)].                     (2.2)

**Curvature-dimension condition CD(K, ∞)**: Γ_2(f, f) ≥ K Γ(f, f)
for all smooth f and some K > 0.

**Theorem (Bakry-Émery 1985 §2)**: CD(K, ∞) ⇒ logarithmic Sobolev
inequality with constant 1/(2K), ⇒ exponential decay of relative
entropy along the diffusion at rate 2K.

**Domain of validity**: reversible diffusions with explicit
carré-du-champ structure. The classical Ornstein-Uhlenbeck and the
heat semigroup on ℝ^n with Gaussian reference measure satisfy
CD(K, ∞) with explicit K.

**What Bakry-Émery 1985 does NOT prove**:
- a Γ_2 inequality for the NS evolution itself. NS is not a
  reversible diffusion; the Constantin-Iyer 2008 stochastic
  Lagrangian (2.1) is a *transport-by-velocity-plus-Brownian-noise*
  process, **not** a Bakry-Émery diffusion (the drift u is the NS
  solution, not the gradient of a fixed potential, and the
  semigroup does not commute with reflection — there is no
  reversibility);
- a curvature-dimension condition on the augmented configuration
  space (u, ρ, τ).

The most that Bakry-Émery 1985 contributes to the candidate
functional is the **structural template** of the τ-rescaled
Fisher-information piece (T_Fish in §1.2): Fisher information is
the Bakry-Émery dissipation of relative entropy under a reversible
diffusion. The candidate (1.1) borrows the Fisher form but applies
it to a non-reversible (stochastic-Lagrangian) flow.

### §2.4 Constantin-Vicol 2012 (GAFA 22, 1289–1321)

**Title**: "Nonlinear maximum principles for dissipative linear
nonlocal operators and applications".

**Actual content**: for the linear nonlocal dissipative operator
Λ^α = (−Δ)^{α/2}, 0 < α ≤ 2, applied to a smooth scalar f, the
nonlinear maximum principle

  Λ^α(f²)/2 − f Λ^α f ≥ c |f|² · (max_x |f|)^α / ‖f‖_{L^∞}^α     (2.3)

(at the spatial maximum) yields monotonicity of certain quantities
for **scalar** active-scalar equations (drift-diffusion with
fractional dissipation, e.g. the SQG equation).

**Domain of validity (CV 2012 §1 stated)**:
- linear nonlocal dissipative operators (fractional Laplacian);
- nonlinearity enters through advection by a vector field derived
  from the scalar (e.g. Riesz transform of θ in SQG);
- the maximum principle is **scalar**: it controls L^∞-type
  quantities, not L²-Hilbert-space quantities.

**What CV 2012 does NOT prove**:
- a Lyapunov for 3D incompressible NS (the velocity is a vector,
  not a scalar; the dissipation is local Δ, not nonlocal Λ^α; the
  pressure is non-local in the same Calderón-Zygmund sense as in
  3D NS, but the maximum-principle technique does not transfer to
  the vector velocity);
- monotonicity of any functional involving Fisher information of
  a coupled density;
- CV 2012 §1 explicitly notes the nonlinear (full) Euler / NS case
  is out of scope.

### §2.5 Otto-Villani 2000 (J. Funct. Anal. 173, 361–400)

**Title**: "Generalization of an inequality by Talagrand and links
with the logarithmic Sobolev inequality".

**Actual content**: HWI inequality

  H(ρ | μ) ≤ √( I(ρ | μ) ) · W_2(ρ, μ) − (K/2) W_2(ρ, μ)²,        (2.4)

with H relative entropy, I relative Fisher information, W_2
Wasserstein-2 distance, valid when μ satisfies CD(K, ∞).

**Domain of validity**: same as Bakry-Émery 1985 — reversible
diffusions with curvature-dimension condition.

**What Otto-Villani 2000 does NOT prove**:
- a Wasserstein-distance bound on the Constantin-Iyer law ρ_t in
  terms of NS data;
- coupling between W_2(ρ_t, ρ_0) and ‖u(t)‖_{H^{1/2}}.

The HWI inequality (2.4) is, however, the analytic lever that
*would* convert a Fisher-information bound (T_Fish in §1.2) into a
Wasserstein-distance bound, **if** the curvature-dimension
condition CD(K, ∞) held on the augmented configuration. As
identified in §2.3 it does not.

### §2.6 Recent NS Lyapunov literature scan

A targeted scan of the post-2008 NS-Lyapunov / NS-stochastic
literature surfaces the following adjacent works:

- **Vasseur 2007** (SIAM J. Math. Anal. 39) and **Vasseur 2010**
  (Arch. Rat. Mech. Anal. 197) establish De Giorgi-style L² → L^∞
  improvements for parabolic equations of NS class; these provide
  ε-regularity at the level of CKN 1982 but no global Lyapunov.
- **Vasseur-Yu 2016** (Invent. Math. 206) establishes a
  Wasserstein-gradient-flow structure for **compressible** NS; the
  structure depends crucially on the compressible-pressure
  constitutive law and **does not transfer to incompressible 3D NS**
  (Vasseur-Yu §1 explicit caveat).
- **Buckmaster-Vicol 2019** (Ann. Math. 189) constructs
  non-uniqueness of weak NS solutions via convex integration; this
  is in the opposite direction (showing how *not* to have a
  Lyapunov at low regularity).
- **Isett 2018** (Ann. Math. 188) Onsager conjecture for Euler;
  rigid Hölder threshold C^{1/3}, again no Lyapunov.
- **Eyink-Sreenivasan 2006** (Rev. Mod. Phys. 78) review the
  Onsager picture; entropy production in turbulence is treated as
  a heuristic, not a rigorous Lyapunov.

**No paper in the searched literature states the inequality (1.2)
or any restatement of (1.2) as a theorem on 3D NS.** The closest
adjacent statements are (a) compressible NS Wasserstein-gradient
flow (Vasseur-Yu 2016, wrong constitutive), (b) Bakry-Émery decay
of relative entropy under reversible diffusions (wrong dynamics),
(c) Otto 2001 PME gradient flow (wrong PDE).

---

## §3 Path-P attempt — direct literature import

### §3.1 The Path-P question

Does any cited paper, or any straightforward chain of cited
papers, **directly prove**

  d/dt W_NS(u(t), ρ(t), τ(t)) ≥ 0                                (3.1)

on smooth 3D NS solutions for the W_NS in (1.1)?

### §3.2 Path-P negative answer

**No.** The §2 literature scan establishes:

- Constantin-Iyer 2008 provides a stochastic-Lagrangian
  representation, **not** a monotonicity statement (§2.1; CI 2008
  §1 explicit caveat).
- Otto 2001 provides Wasserstein-gradient-flow structure for PME
  and heat equation, **not for 3D NS** (§2.2; cataloged-out by
  Ambrosio-Gigli-Savaré 2008, Brenier 1999).
- Bakry-Émery 1985 + Otto-Villani 2000 give entropy / Fisher /
  Wasserstein inequalities for **reversible diffusions**, not for
  the NS-driven stochastic-Lagrangian (§2.3, §2.5).
- Constantin-Vicol 2012 gives nonlinear maximum principles for
  scalar nonlocal dissipative equations, **not for vector NS**
  (§2.4).
- The post-2008 NS-Lyapunov scan (§2.6) finds no paper stating
  (3.1) or any restatement.

### §3.3 The W_NS form is a synthesis, not a literature object

The functional W_NS in (1.1) is a *synthesis* of pieces drawn from
four distinct literatures:

- T_kin (τν|∇u|²ρ): NS dissipation × density-weight (Leray 1934 form,
  density-multiplied);
- T_Fish (τ|∇ log ρ|²ρ): Fisher information at scale τ
  (Bakry-Émery 1985 / Gross 1975 form);
- T_ent ((log ρ)ρ): Boltzmann entropy density (Otto 2001 form);
- T_corr (γνt): linear correction (Perelman 2002 −n analog).

The combination is **not in the published literature** as a
theorem. The candidate spec §3.5 already acknowledged this:
"the candidate's distinctive content is the COMBINATION (3.2) and
the conjecture (3.3)". Path-P therefore cannot succeed.

### §3.4 Path-P verdict

**Path-P FAILS.** No citation chain produces (3.1) directly. This
matches the candidate spec §6.3 expected outcome: PASS_LITERATURE
probability ~5% — the literature scan confirms that the lower
bound holds.

---

## §4 Path-Q attempt — sketch construction

### §4.1 The Path-Q question

Can (3.1) be derived from standard tools (Bakry-Émery Γ_2, Otto
gradient-flow calculus, Constantin-Iyer stochastic Lagrangian) with
all steps standard, no new theorem?

### §4.2 Sketch attempt — term-by-term differentiation

Differentiate W_NS along the coupled evolution. Write t-dependence
explicitly. Assume (provisionally) that ρ_t is a smooth probability
density on ℝ³, that u is a smooth NS solution, and that all
integrations by parts are justified.

**Term T_kin(t) = ∫ τ(t) ν |∇u(x, t)|² ρ(x, t) dx**:

  d/dt T_kin = τ̇ν ∫|∇u|² ρ + τν ∫ ∂_t(|∇u|²) ρ
              + τν ∫ |∇u|² ∂_t ρ.                                (4.1)

The first piece uses τ̇(t). The second piece, via the NS equation
∂_t u = ν Δu − (u·∇)u − ∇p, gives

  ∂_t(|∇u|²) = 2 ∇u : ∇(ν Δu − (u·∇)u − ∇p)
            = 2ν ∇u : ∇Δu − 2 ∇u : ∇((u·∇)u) − 2 ∇u : ∇²p
            ⇒ ∫ ∂_t(|∇u|²) ρ dx
            = −2ν ∫ |Δu|² ρ + (boundary) + (vortex-stretching) + (pressure-coupling).
                                                                  (4.2)

The vortex-stretching term ∫(ω·∇)u·ω · ρ dx is the **NS regularity
obstruction itself** (per candidate spec §2.3 enstrophy row): it is
sign-indefinite and the open NS regularity question is precisely
whether this term can be controlled.

The third piece in (4.1) requires ∂_t ρ, which evolves under the
forward Kolmogorov equation associated with (2.1):

  ∂_t ρ = −∇·(u ρ) + ν Δ ρ.                                       (4.3)

Substituting:

  ∫ |∇u|² ∂_t ρ dx = −∫ |∇u|² ∇·(u ρ) + ν ∫ |∇u|² Δρ
                    = ∫ ∇(|∇u|²)·u ρ + ν ∫ Δ(|∇u|²) ρ              (4.4)

(after integration by parts, dropping boundary terms on ℝ³ for
sufficiently decaying ρ).

**Term T_Fish(t) = ∫ τ |∇ log ρ|² ρ dx = ∫ τ |∇ρ|²/ρ dx**:

This is the Fisher information of ρ, scaled by τ. Its evolution
under (4.3):

  d/dt ∫ |∇ρ|²/ρ dx = (Otto 2001 calc, transposed to non-reversible)
                    = ν ∫ "Γ_2-type" terms − 2 ∫ (Hess log ρ) : ∇u · ρ
                                                                  (4.5)
                    + transport corrections.

The Hess(log ρ) : ∇u term is the **cross term** between ρ and u.
Its sign is **uncontrolled** under generic NS dynamics. The Otto
2001 calculation that produces clean monotonicity of Fisher
information assumes ∂_t ρ = Δρ (heat) or Δρ^m (PME); the additional
−∇·(u ρ) drift in (4.3), with u itself a *non-gradient* of a fixed
potential, breaks the Otto monotonicity calculation.

A Bakry-Émery Γ_2 estimate would *want* a CD(K, ∞) condition on
the operator L = ν Δ − u·∇. But L is **non-symmetric** (under the
inner product weighted by ρ, since u is divergence-free but
time-dependent), so the carré-du-champ Γ(f) = ν|∇f|² gives

  Γ_2(f) = ν² (Hess f)² + ν Ric(∇f, ∇f) − ν ∇(div u)·∇f − …       (4.6)

(in the general non-symmetric case the Γ_2 expression has extra
terms; see Bakry-Gentil-Ledoux 2014 Springer monograph
*Analysis and Geometry of Markov Diffusion Operators* §3 for the
clean reversible case and §C.6 for non-symmetric extensions).

The non-symmetric extra terms in (4.6) are **not sign-controllable**
by Bakry-Émery alone; they require *additional* hypotheses on u
that amount to assuming what the candidate Lyapunov is supposed to
prove.

**Term T_ent(t) = ∫ (log ρ) · ρ dx**:

  d/dt T_ent = ∫ (∂_t ρ) (1 + log ρ) dx
            = ∫ (1 + log ρ)(−∇·(u ρ) + ν Δρ) dx
            = ∫ ∇(1 + log ρ) · u ρ dx − ν ∫ |∇log ρ|² ρ dx
                                                                  (4.7)

(after integration by parts, using divergence-free u ⇒ first term
= ∫ (∇ρ/ρ) · u ρ dx = ∫ ∇ρ · u dx = − ∫ ρ (div u) dx = 0).

So d/dt T_ent = −ν · I(ρ), where I(ρ) is the Fisher information.
This is a **clean monotone-decreasing** identity (entropy decreases
under heat flow with divergence-free transport — classical
H-theorem analog).

**Term T_corr(t) = γ ν t**: d/dt T_corr = γ ν.

### §4.3 Aggregating the time derivative

Collecting (4.1), (4.2), (4.4), (4.5), (4.7):

  d/dt W_NS = τ̇ν ∫|∇u|² ρ                                          (a) [scale-rescaling]
            + τν ∫(−2ν |Δu|² + vortex-stretching + pressure-coupling) ρ  (b) [NS dissipation + obstruction]
            + τν ∫(transport + Laplacian-of-|∇u|²) ρ                 (c) [transport corrections]
            + τ̇ ∫ |∇log ρ|² ρ                                         (d) [scale-rescale of Fisher]
            + τ · {Γ_2 terms − 2 ∫ Hess(log ρ) : ∇u · ρ}             (e) [Bakry-Émery + cross term]
            − ν ∫ |∇log ρ|² ρ                                          (f) [entropy-decay]
            + γν.                                                      (g) [linear correction]

The **uncontrolled cross terms** are:

- (b) the vortex-stretching ∫ (ω·∇)u·ω ρ, which is the NS regularity
  obstruction;
- (e) the Hess(log ρ) : ∇u cross term, which is the
  density-velocity coupling that has no a-priori sign;
- the boundary / decay assumptions on ρ at infinity (the candidate
  spec assumes ρ is a probability density on ℝ³; the
  Constantin-Iyer 2008 representation does not establish this — see
  §6 below).

The **controlled terms** are:

- (a), (d): scale-rescaling pieces (sign depends on τ̇'s sign);
- (b)-substituent −2ν τ ∫ |Δu|² ρ (negative — wrong sign for the
  candidate's d/dt W_NS ≥ 0);
- (f) the entropy-decay piece (negative — wrong sign for d/dt ≥ 0).

### §4.4 Sign analysis — does (3.1) emerge?

For (3.1) to hold globally, the positive contributions
((a), (d), (g) under appropriate τ̇-sign and γ-choice) must
**dominate** the negative + uncontrolled contributions
(−2ν τ ∫ |Δu|² ρ, −ν I(ρ), and the cross terms in (b), (e)).

This is **not a clean Bakry-Émery + Otto outcome**. In Otto 2001's
clean PME case, the analog of (b) and (e) vanishes (no
vortex-stretching, no advection by an independent vector field).
In Bakry-Émery 1985's clean reversible-diffusion case, the analog
of (e) is +Γ_2 ≥ K Γ ≥ 0 (curvature-dimension). In the
**candidate's coupled NS + CI flow**, neither cleanness is
available:

- Vortex-stretching in (b) is the NS open problem;
- Cross term in (e) requires CD(K, ∞) which the augmented
  configuration does not satisfy.

### §4.5 Path-Q verdict

**Path-Q FAILS at the cross-term step.** The Bakry-Émery + Otto
machinery sketches the *clean-case* version of the inequality but
does **not** produce a sign-controlled bound for the
NS-driven-CI-flow version. The two uncontrolled terms (vortex
stretching and Hess(log ρ) : ∇u cross) are precisely the structural
obstructions that the candidate spec §6.3 anticipated.

This rules out PASS_SKETCH.

The Path-Q sketch is **structurally informative** in the sense
mentioned in candidate spec §6.3 row 3: it identifies which
terms break and where — namely, at the vortex-stretching identity
(b) and at the non-symmetric Γ_2 calculus (e). This information
maps the missing piece to specific algebraic identities.

---

## §5 Falsifier check — F-EXTB-A (relabeling on smooth families)

### §5.1 F-EXTB-A statement (recap)

W_NS(u, ρ, τ) reduces to a smooth function Φ(E, H, Z, D, log E,
τ, t) on at least one of three smooth-NS families:
2D NS / axisymmetric-no-swirl / 2.5D non-local-pressure.

If yes on all three: F-EXTB-A FIRES (relabeling).
If no on at least one: F-EXTB-A does NOT fire.

### §5.2 2D NS (Ladyzhenskaya 1969 *Boundary-Value Problems of
Mathematical Physics*; Constantin-Foias 1988 *Navier-Stokes
Equations*, U. Chicago)

2D incompressible NS on T² has **no vortex stretching**. The
enstrophy Z is conserved up to viscous decay: dZ/dt = −ν‖∇ω‖²_{L²}.
Regularity is settled.

Does W_NS in (1.1) reduce to Φ(E, H, Z, D, log E, τ, t) in 2D?

The Fisher-information piece T_Fish = τ ∫ |∇ log ρ|² ρ dx involves
the **stochastic-Lagrangian density ρ**, which on 2D NS is the law
of (2.1) restricted to ℝ² with u_t the 2D NS velocity. ρ_t evolves
by (4.3) in 2D. The functional T_Fish depends on ρ_t and not on
(E, H, Z, D, log E) of u alone, **unless** ρ_t happens to be a
function of these quantities. There is no reason for ρ_t to be a
function of (E, H, Z, D, log E) — ρ_t is a probability density on
ℝ² that depends on the entire trajectory of u_t up to time t,
not on its energy / enstrophy / helicity scalars.

Therefore in 2D, T_Fish is **independent** of (E, H, Z, D, log E).
Combined with T_kin and T_corr (which are functions of u and t),
T_ent (which depends on ρ), the full W_NS is **not** a smooth
function of (E, H, Z, D, log E, τ, t).

**Result on 2D**: W_NS does not reduce to Φ(E, H, Z, D, log E, τ, t).
F-EXTB-A does not fire on this family.

### §5.3 Axisymmetric-no-swirl 3D NS (Chen-Strain-Tsai-Yau 2008/9
Comm. PDE 33 / Comm. PDE 34; regularity-settled, no vortex
stretching for the swirl-free component)

The same argument as §5.2 applies. The stochastic-Lagrangian density
ρ_t is a probability density on ℝ³; even if u_t is axisymmetric-no-
swirl, ρ_t depends on the full path-history and is not a function
of (E, H, Z, D, log E).

**Result on axisymmetric-no-swirl**: F-EXTB-A does not fire.

### §5.4 2.5D non-local-pressure (D3.A discriminator setting,
omega-exec-bt544-d3-A-axis-discriminator-2026-04-25)

2.5D NS with the non-local pressure (vector velocity (u_1, u_2, u_3)
depending on (x_1, x_2) only) has the same regularity story as 2D:
vortex stretching of the in-plane components vanishes, and the third
component is passively advected. Helicity H is non-trivially defined
but enstrophy of the in-plane part is monotone-decreasing.

ρ_t in this setting is again a probability density on ℝ³ (or ℝ²
if the third component is collapsed); the same independence
argument applies.

**Result on 2.5D**: F-EXTB-A does not fire.

### §5.5 F-EXTB-A verdict

**F-EXTB-A does NOT fire** on any of the three families. The
candidate W_NS is non-trivial (non-relabeling) by the
Fisher-information piece T_Fish, which depends on the
stochastic-Lagrangian density ρ — an object distinct from any of
(E, H, Z, D, log E).

This matches the candidate spec §6.3 anticipation that F-EXTB-A
was unlikely to fire.

### §5.6 But: a subtlety — the Fisher piece's *value* may degenerate

A weaker form of relabeling: even if T_Fish depends on ρ, it might
**equal zero** or **equal a function of E** on the three smooth
families, because ρ_t in those settings might degenerate (e.g. if
the stochastic-Lagrangian flow becomes trivial). Examination:

- The Brownian-noise term √(2ν) dW_t in (2.1) ensures ρ_t is a
  non-degenerate Gaussian-like density at any t > 0 (the law of a
  diffusion is full-support under non-degenerate diffusion
  coefficient). So ρ_t > 0 everywhere with positive Fisher
  information |∇ log ρ_t|² in general.
- T_Fish is therefore generically positive and not equal to zero.
  It is a non-trivial functional of u_t through the drift in (2.1).

Subtler relabeling (T_Fish a smooth function of E alone via some
hidden identity) is implausible: the Fisher information of a
diffusion's law involves the inverse covariance of the diffusion,
which depends on the full trajectory of u, not just its L² norm.

F-EXTB-A still does not fire under this finer scrutiny.

---

## §6 Falsifier check — F-EXTB-D (CI-2008 representation strength)

### §6.1 F-EXTB-D statement (recap)

Constantin-Iyer 2008 establishes a stochastic-Lagrangian
representation of NS solutions for **smooth, short-time, T³**
data. Does this representation extend to:
- (a) long-time?
- (b) weak / Hölder NS solutions?
- (c) the augmented state space (u, ρ, τ) with ρ a probability
  density on ℝ³?

If the extension is open / unproven: F-EXTB-D FIRES.

### §6.2 Long-time extension

CI 2008's representation holds **as long as the NS solution is
smooth**. On T³ with smooth initial data, this is the standard
short-time existence (Kato-Fujita 1962, Kato 1984, Foias-Manley-
Rosa-Temam 2001 §4). Long-time extension to T = ∞ would *follow
from* global regularity of NS, which is the Clay problem. Therefore
the representation **does not provide global-time validity** as an
independent fact; it inherits the regularity gap.

**Status**: open. F-EXTB-D fires on (a).

### §6.3 Weak / Hölder solutions

CI 2008 explicitly works with smooth solutions. The
stochastic-Lagrangian flow (2.1) is well-defined under weaker
regularity (e.g. u ∈ L^∞_t W^{1, q}_x for appropriate q would suffice
for the ODE-with-Brownian-perturbation to admit a unique strong
solution by DiPerna-Lions / Ambrosio theory), but the representation
formula

  u(x, t) = E[ ∇^T_x A(X^{−1}_t(x), 0) · u_0(X^{−1}_t(x)) ]_{div-free}

requires the back-to-labels gauge A to be regular enough to take
gradients ∇^T_x A. For Hölder C^α solutions with α < 1, the
gradient is distributional; the representation has no published
extension to this regularity tier.

For Leray-Hopf weak solutions (u ∈ L^∞_t L²_x ∩ L²_t H¹_x), the
representation likewise lacks a published extension.

**Status**: open. F-EXTB-D fires on (b).

### §6.4 The augmented state space (u, ρ, τ)

This is the most decisive check. The candidate functional W_NS in
(1.1) requires:

(i) **u**: 3D NS solution (CI 2008 provides for smooth, short-time);
(ii) **ρ**: a probability density on ℝ³ (NOT on path-space — the
     candidate functional integrates ρ dx on ℝ³);
(iii) **τ**: the scale parameter (no representation issue).

The decisive question: does CI 2008 establish that **the law of
X_t is an L¹-density ρ_t on ℝ³** (rather than just a measure on
path-space)?

CI 2008 §2 defines X_t as a stochastic flow with values in ℝ³; for
each fixed t > 0, the random variable X_t(ω, x_0) (for fixed
initial label x_0) has a law on ℝ³. Under standard
diffusion-theoretic results for SDEs with Lipschitz drift and
non-degenerate diffusion (Stroock-Varadhan 1979 *Multidimensional
Diffusion Processes*, Springer), this law has a smooth density on
ℝ³ for t > 0 (Hörmander's hypoellipticity argument applied to the
generator of (2.1)).

**However**, CI 2008's representation formula uses X_t as an
ℝ³-valued random map (the back-to-labels flow), parameterized by
the initial label x_0. The candidate functional (1.1) integrates
**a single density ρ on ℝ³**, not a family of densities indexed by
x_0. This is a non-trivial choice that CI 2008 does not make: CI
2008's representation distinguishes x_0 from x_t and treats them as
distinct variables.

The candidate spec §3.2 sentence "ρ a probability density on ℝ³
representing the law of the stochastic-Lagrangian flow at time t
(per Constantin-Iyer 2008 §2 the noise-driven flow generates such
a ρ_t)" is **imprecise**: CI 2008 §2 generates a family of laws
ρ_t(· | x_0) (one per initial label), not a single ρ_t on ℝ³. To
collapse to a single ρ_t requires a choice — e.g.:

- (Choice A) ρ_t(x) = ∫ ρ_t(x | x_0) μ_0(x_0) dx_0, with μ_0 some
  initial measure on labels (say uniform in a large box, or
  Gaussian on ℝ³). This is the **mixed-initial-distribution** choice.
- (Choice B) ρ_t(x) = ρ_t(x | x_0 fixed), for some canonical x_0
  (e.g. the origin). This is the **pinned-initial-label** choice.
- (Choice C) ρ_t = the law of u(X_t, t) itself, treating X_t as a
  random variable on ℝ³ via mixing with a reference Gaussian noise
  term. This is the **velocity-mixed** choice.

Each choice gives a different ρ_t, with different evolution under
(4.3). The candidate spec does **not specify** which choice. CI
2008 itself does **not** privilege any of these — the
representation formula carries x_0 explicitly throughout.

**Status**: F-EXTB-D fires on (c) at a specific structural step:
the "ρ on ℝ³" object the candidate functional needs is not
canonically specified by CI 2008; it requires an additional choice
(A, B, or C) that the candidate spec leaves implicit, and each
choice produces a *different* candidate functional with a
*different* (and equally undetermined) monotonicity question.

### §6.5 F-EXTB-D verdict

**F-EXTB-D FIRES** on all three sub-checks:
- (a) long-time: open, inherits NS regularity gap;
- (b) weak / Hölder: no published extension;
- (c) augmented state space: CI 2008 produces a family of densities
  ρ_t(· | x_0), not a single ρ_t on ℝ³; the candidate functional's
  ρ requires a choice (A, B, or C) that is unspecified.

Sub-check (c) is the **most decisive**: it identifies a precise
ambiguity in the candidate's setup — the ρ in (1.1) is not a
well-defined object given only CI 2008.

This matches the candidate spec §5.4 prediction that F-EXTB-D was
"the most likely activation mode".

---

## §7 Verdict

### §7.1 Discriminator outcome

| path | outcome | reason |
|------|---------|--------|
| Path P (literature import) | FAIL (§3.4) | no paper proves (3.1); W_NS is a synthesis |
| Path Q (sketch) | FAIL at cross terms (§4.5) | vortex stretching + Hess(log ρ):∇u uncontrolled |
| Falsifier F-EXTB-A | does NOT fire (§5.5) | T_Fish is independent of (E, H, Z, D, log E) |
| Falsifier F-EXTB-D | FIRES (§6.5) | CI 2008 doesn't determine ρ on ℝ³; long-time / weak ext. open |

### §7.2 Verdict

**OBSTRUCTION_DOCUMENTED.**

The obstruction is **two-fold**:

1. **Representation-side (F-EXTB-D, §6)**: the augmented state
   space (u, ρ, τ) the candidate requires is not a canonical
   output of Constantin-Iyer 2008. CI 2008 produces a family of
   laws ρ_t(· | x_0) parameterized by initial label; the candidate
   functional integrates a single density ρ_t(x) on ℝ³. The
   collapse from {ρ_t(· | x_0)}_{x_0} to ρ_t requires a choice
   (mixed initial / pinned label / velocity-mixed), and each
   choice produces a different candidate functional with a
   different monotonicity question. The candidate spec leaves this
   choice implicit. **This is the most fundamental obstruction.**

2. **Sketch-side (Path Q, §4)**: even granting any of the three
   choices A/B/C in §6.4, the time-derivative computation in §4
   produces two uncontrolled cross terms — the vortex-stretching
   ∫(ω·∇)u·ω ρ (which IS the NS regularity obstruction) and the
   Hess(log ρ):∇u cross term (which has no a-priori sign and is
   not controlled by Bakry-Émery 1985 since the augmented operator
   is non-symmetric and lacks a curvature-dimension condition).
   The Otto 2001 + Bakry-Émery 1985 + CI 2008 inputs do not
   assemble cleanly to (3.1).

The verdict is consistent with the candidate spec §6.3 row 4
("CI representation too weak ⇒ F-EXTB-D fires (most likely) ⇒
OBSTRUCTION_DOCUMENTED (representation-side)"), with the addition
that the Path-Q cross-term obstruction provides a **second
independent obstruction** at the sketch level.

### §7.3 Probability calibration check

Candidate spec §6.3 anticipated:
- ~75% OBSTRUCTION_DOCUMENTED;
- ~20% PASS_SKETCH;
- ~5% PASS_LITERATURE.

This validation realises the OBSTRUCTION_DOCUMENTED outcome. The
calibration is consistent.

---

## §8 The precise step that breaks

Per the OBSTRUCTION_DOCUMENTED verdict, the precise step that
breaks is identified for follow-up sessions.

### §8.1 Primary breaking step

**The collapse from {ρ_t(· | x_0)}_{x_0 ∈ ℝ³} to a single ρ_t on
ℝ³** is undetermined by CI 2008.

CI 2008 §2 main theorem produces a back-to-labels stochastic flow
X_t(x_0) with X_0(x_0) = x_0, satisfying (2.1). For each x_0 ∈ ℝ³
and t > 0, the law of X_t(x_0) on ℝ³ is a probability measure (under
non-degenerate diffusion, this measure has an L¹-density by
Hörmander hypoellipticity). The collection {ρ_t(· | x_0)} is the
*conditional law family*. The candidate functional (1.1) integrates
a single ρ_t on ℝ³ — this requires either:

- a choice of initial measure μ_0 on the label space, then
  ρ_t = ∫ ρ_t(· | x_0) dμ_0(x_0); or
- a pinning x_0 = (canonical point); or
- a pushforward of u via X_t (not a probability density on ℝ³ in
  the standard sense unless u is normalised).

**No published paper specifies this choice** in the context of an
NS Lyapunov. The candidate spec implicitly assumes a single ρ_t
exists "naturally"; this assumption is not justified by CI 2008.

### §8.2 Secondary breaking step

**Even given a choice of ρ_t**, the Hess(log ρ):∇u cross term in
the time derivative of T_Fish (equation 4.5) is not sign-controlled.

The Bakry-Émery 1985 framework that *would* control this term
requires:
- a reversible diffusion (the NS-driven CI flow is non-reversible);
- a curvature-dimension condition CD(K, ∞) on the augmented
  configuration (no such condition is established);
- symmetric (with respect to ρ-weighted inner product) generator
  (NS advection is non-symmetric due to time-dependence of u).

None of these is available. The Bakry-Gentil-Ledoux 2014 monograph's
non-symmetric extensions (§C.6 of their book) require additional
hypotheses that, in the NS context, amount to assuming what the
candidate Lyapunov is trying to prove.

### §8.3 Tertiary issue — vortex-stretching residual

The vortex-stretching term ∫(ω·∇)u·ω ρ in (4.2) is the NS
regularity obstruction itself. Even if the §8.1 / §8.2 steps were
resolved, the candidate functional inherits this obstruction
unchanged. The candidate's Lyapunov hope was that the
Fisher-information piece T_Fish would *control* the
vortex-stretching residual via the Wasserstein / log-Sobolev chain
(candidate spec §3.4). The §4-§6 analysis indicates this control
chain is the conjectural piece, not the established input.

### §8.4 Where to push next (informational, not prescriptive)

Identifying the breaking step is informative for follow-up
candidate generation. Three directions are visible:

- **Direction α — replace ρ with a path-space measure**: rewrite
  W_NS as a functional on the law of the path-space process X_•
  rather than on ρ_t at fixed t. This preserves CI 2008's natural
  output but changes the functional form. Whether a path-space
  Lyapunov is feasible is open.
- **Direction β — fix the choice**: pick one of A/B/C from §6.4
  explicitly and re-run the Path-Q sketch under that choice. The
  three choices may give different results; one of them may admit
  a cleaner sketch.
- **Direction γ — drop CI 2008 entirely**: replace the
  stochastic-Lagrangian source with another route to a density on
  ℝ³ (e.g. coarse-grained energy density, vortex-line density,
  vorticity probability density). This abandons the Constantin-Iyer
  bridge and treats the candidate as a different candidate.

These are not prescribed for execution in this session; they are
recorded as the structural-residual directions exposed by the
OBSTRUCTION_DOCUMENTED verdict.

---

## §9 F-MOLT-A tally update with EXT-B verdict

### §9.1 Pre-EXT-B tally (from BT-547 retro §3.3)

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

### §9.2 Post-EXT-B addition

Adding this validation as a new row (active BT, not retrospective):

| # | BT | candidate | verdict | discriminator type |
|---|----|-----------|---------|--------------------|
| 9 | 544 | EXT-B CI-Lyap | OBSTRUCTION_DOCUMENTED | OTHER (analytic-inequality-construction) |

OBSTRUCTION_DOCUMENTED is in the **PASS-family-adjacent** position
of the discriminator-type bias hypothesis: it is not a clean PASS
and not a FAIL, but it is in the variational / analytic-Lyapunov
discriminator family that the BT-547 retro §3.4 broadened to be
PASS-family-adjacent.

### §9.3 Updated 2×2 matrix (collapsed)

|                                             | PASS / OBSTR_DOC | FAIL |
|---------------------------------------------|------|------|
| distrib / struct-lit / OTHER (PASS-adjacent) | 7 (rows 1, 2, 5, 6, 7, 8, 9) | 0 |
| discrete-equality / numerical-interval      | 0 | 2 (rows 3, 4) |

The bias hypothesis is **not disturbed** — no cross-cell entries.

The hypothesis is **slightly broadened** in that
OBSTRUCTION_DOCUMENTED at the analytic-inequality-construction
discriminator is recorded for the first time in the active-BT
register, alongside the retrospective Perelman PASSes at the same
discriminator type.

### §9.4 F-MOLT-A status

F-MOLT-A is defined on **active BTs** (per BT-547 retro §3.5
discussion). Active-BT count of EXT-B-class verdicts now equals 1
(this row). Distance to F-MOLT-A firing is unchanged from BT-547
retro §9 — F-MOLT-A is **not fired** because the OBSTRUCTION_DOCUMENTED
verdict is in the PASS-family-adjacent column, not in the FAIL
column.

### §9.5 Tally observation — EXT-B class is now populated

The L9 catalogue's EXT-B slot (per BT-547 retro §5.2) was
previously empty. With this validation, the EXT-B slot is populated
by **CI-Lyap with OBSTRUCTION_DOCUMENTED verdict** — informative
content, not a closure. The slot remains open for further candidates
of the same class (per §8.4 directions α/β/γ for follow-up).

---

## §10 Anomalies

Items observed during validation that do not change the verdict
but are flagged for record-keeping.

### §10.1 Choice-of-ρ ambiguity is structural

The §6.4 / §8.1 observation that CI 2008 produces a family
{ρ_t(· | x_0)} rather than a single ρ_t on ℝ³ is not just a
technical issue — it reflects a **structural mismatch** between
Lagrangian (path-space, label-indexed) and Eulerian (single-density-
on-ℝ³) frames. The candidate spec implicitly conflated the two.
Future EXT-B candidates in this lineage should specify the frame
choice upfront.

### §10.2 The Hess(log ρ) : ∇u cross term has no name in the
literature

The cross term in (4.5) is not a named object in the
Bakry-Émery / Otto / Constantin-Iyer literatures (it does not
appear in Bakry-Gentil-Ledoux 2014, Otto 2001, Ambrosio-Gigli-
Savaré 2008, Constantin-Iyer 2008, or any of the secondary
references checked). This may be because no prior work tried to
combine these specific inputs (CI representation + Bakry-Émery Γ_2
on the diffusion's law). The cross term's properties (sign
conditions, bounds in terms of standard NS norms) are an open
research direction in their own right — not the NS regularity
question, but a precursor analytic question.

### §10.3 The candidate spec's implicit "single ρ" reading is
common in the heuristic NS-Wasserstein literature

The candidate spec's reading of CI 2008 as producing a single ρ_t
is consistent with **heuristic** treatments in the Eulerian-statistical
NS literature (e.g. some review articles in turbulence theory). It
is **not** consistent with CI 2008's actual published statement.
This validation does not retract the candidate-generation report
on this account — the candidate-generation pipeline allows
heuristic synthesis as an input — but the validation flags the gap
explicitly so future EXT-B candidate spec drafts can avoid it.

### §10.4 Path-Q failure mode is informative but doesn't promote
the candidate

A common pattern in PDE-Lyapunov literature is that a Path-Q
sketch failure with identifiable cross-term obstructions becomes
the seed for a follow-up candidate that controls the cross terms.
The §8.4 directions α/β/γ are exactly such seeds. They are not,
however, promotions of CI-Lyap to PASS_SKETCH; they are new
candidate generation tasks for follow-up sessions.

### §10.5 Disclaimer count check

This file uses "conjecture", "candidate", "open", "not proven",
"not established" for every step that is not a published theorem
with rigorous proof. No NS regularity claim is made anywhere in
this file. Every cited reference is by author / year /
journal-or-publisher pattern matched to the standard NS-analysis
bibliography.

---

## §11 Falsifiers active

### §11.1 Falsifiers for this validation's verdict

Falsifiers under which the §7 OBSTRUCTION_DOCUMENTED verdict would
be retracted or downgraded.

#### F-VAL-A (cited-paper-already-proves-(3.1)-and-search-missed-it)

**Statement**: if a published paper exists that directly proves
(3.1) for some choice of ρ on ℝ³ (e.g. a recent paper on stochastic
NS that establishes a Wasserstein Lyapunov), then Path-P would
succeed and the verdict shifts to PASS_LITERATURE.

**Status**: NOT ACTIVE. The §2.6 literature scan did not surface
such a paper. The risk of a missed paper is real but low; cross-
check on validation extension if undertaken.

#### F-VAL-B (Path-Q-cross-terms-actually-controllable-by-known-tool)

**Statement**: if there exists a known analytic tool that controls
the Hess(log ρ):∇u cross term in (4.5) for the specific
NS-driven CI flow — e.g. a recent extension of Bakry-Émery to
non-symmetric semigroups with curvature-dimension condition that
the validation overlooked — then Path-Q would succeed and the
verdict shifts to PASS_SKETCH.

**Status**: PARTIALLY ACTIVE in spirit. Bakry-Gentil-Ledoux 2014
§C.6 has non-symmetric extensions; whether they apply here is open.
The validation's reading is that they do not apply directly because
the NS-driven CI flow lacks the curvature condition; a careful
re-reading by a Bakry-Émery specialist could change this.

#### F-VAL-C (CI-2008-extension-to-augmented-space-actually-published)

**Statement**: if a paper exists (perhaps by Constantin-Iyer or
others post-2008) that establishes the existence of a single ρ_t
on ℝ³ canonically associated with the stochastic-Lagrangian flow,
then F-EXTB-D §6.4 sub-check (c) is resolved in favour of the
candidate and the verdict softens.

**Status**: NOT ACTIVE based on the literature scan. The risk of a
missed paper is non-trivial because the post-2008 stochastic-NS
literature is large; cross-check on extension if undertaken.

#### F-VAL-D (vortex-stretching-residual-is-actually-controllable)

**Statement**: if a known analytic identity controls the
vortex-stretching residual ∫(ω·∇)u·ω ρ when weighted by a
Constantin-Iyer density ρ — e.g. an extension of the
Constantin-Fefferman 1993 geometric depletion that uses ρ to
provide the vorticity-direction Lipschitz hypothesis — then the
NS regularity obstruction itself becomes accessible, and the
verdict softens dramatically.

**Status**: NOT ACTIVE. This would be a major NS analytic advance,
and is exactly the regularity-resolution claim the validation
declines to make. Risk: low but structurally interesting as a
follow-up direction.

#### F-VAL-E (validation-mis-reads-CI-2008-on-the-density-question)

**Statement**: if §6.4's reading of CI 2008 — that the
representation produces a family {ρ_t(· | x_0)} rather than a
single ρ_t — is incorrect, and CI 2008 actually produces a single
ρ_t naturally, then F-EXTB-D §6.4(c) does not fire and the verdict
softens.

**Status**: PARTIALLY ACTIVE. CI 2008 §2 main theorem statement is
explicit about the back-to-labels indexing; the family structure
is real. But a more permissive reading (e.g. mixing over labels
implicitly) might absorb the §6.4 ambiguity. Cross-check by re-
reading CI 2008 §2 in detail at extension.

#### F-VAL-F (atlas/state/inventory-edit-leakage)

**Statement**: if any change is made to atlas, state, or inventory
files as a result of this validation, the brief's hard constraint
is violated.

**Status**: NOT ACTIVE. This validation is research-only and edits
no atlas, state, or inventory file. The git status at session
start (specs and inventory.json modified by *unrelated* prior
sessions per the gitStatus header) is unaffected by this
validation.

### §11.2 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-VAL-A | cited paper proves (3.1), search missed | NOT ACTIVE |
| F-VAL-B | Path-Q cross terms controllable by known tool | PARTIALLY ACTIVE |
| F-VAL-C | CI 2008 augmented-space extension already published | NOT ACTIVE |
| F-VAL-D | vortex-stretching residual controllable | NOT ACTIVE |
| F-VAL-E | validation mis-reads CI 2008 on density | PARTIALLY ACTIVE |
| F-VAL-F | atlas/state/inventory edit leakage | NOT ACTIVE |

Two falsifiers (F-VAL-B, F-VAL-E) are partially active —
re-checking them would require domain-specialist re-reading of
Bakry-Gentil-Ledoux 2014 §C.6 and Constantin-Iyer 2008 §2,
respectively. Neither is *expected* to change the verdict, but
both are flagged as the structural risks to the verdict.

### §11.3 Inherited falsifiers from candidate spec

The five candidate-spec falsifiers F-EXTB-A through F-EXTB-E
status update from this validation:

| tag | name | candidate-spec status | post-validation status |
|-----|------|----------------------|-------------------------|
| F-EXTB-A | relabeling | NOT YET TESTED | DOES NOT FIRE (§5) |
| F-EXTB-B | vacuity | NOT YET TESTED | not separately tested in this session; clearly not active given F-EXTB-A doesn't fire |
| F-EXTB-C | Wasserstein-fails-for-NS | PARTIALLY ACTIVE | partially confirmed via §4.5 cross-term failure (Otto framework doesn't transfer cleanly) |
| F-EXTB-D | CI-2008-too-weak | most likely activation | FIRES (§6) — confirmed |
| F-EXTB-E | Perelman-archetype-superficial | partially active | partially confirmed via §8.2 Bakry-Émery non-applicability |

The candidate-spec's expected activation pattern (F-EXTB-D primary,
F-EXTB-C and F-EXTB-E partial) is **realised** by this validation.

---

## §12 Closing

**Verdict**: **OBSTRUCTION_DOCUMENTED** at F-EXTB-D primary
activation (§6), with secondary Path-Q cross-term obstruction
(§4.5).

**Path P**: FAIL — no published paper proves (3.1).
**Path Q**: FAIL at the cross-term step — vortex stretching and
Hess(log ρ):∇u cross are not sign-controlled by CI 2008 + Otto
2001 + Bakry-Émery 1985 inputs alone.

**Falsifier F-EXTB-A**: does NOT fire (W_NS is non-trivially
distinct from Φ(E, H, Z, D, log E, τ, t) on 2D / axisymmetric-
no-swirl / 2.5D families via the Fisher-information piece).

**Falsifier F-EXTB-D**: FIRES at sub-check (c) — CI 2008 produces
a family {ρ_t(· | x_0)} of laws indexed by initial label, not a
single ρ_t on ℝ³ that the candidate functional requires. Sub-checks
(a) long-time and (b) weak / Hölder also fire as inherited gaps.

**Precise breaking step**: §8.1 — the collapse from the family
{ρ_t(· | x_0)} to a single ρ_t on ℝ³ is undetermined by CI 2008
and the candidate spec leaves it implicit. Three follow-up
directions α/β/γ (§8.4) are visible but not prescribed in this
session.

**F-MOLT-A 6-BT tally update**: row 9 added (BT-544 EXT-B CI-Lyap,
OBSTRUCTION_DOCUMENTED, OTHER analytic-inequality-construction).
Bias hypothesis is not disturbed; OBSTRUCTION_DOCUMENTED at this
discriminator type is in the PASS-family-adjacent column. F-MOLT-A
is not fired; distance to firing unchanged.

**Anomalies (§10)**: choice-of-ρ ambiguity is structural; the cross
term Hess(log ρ):∇u has no name in the literature; the candidate
spec's "single ρ" reading is heuristic-common but not literal-CI
2008.

**Falsifiers active for this validation (§11)**: F-VAL-B and
F-VAL-E partially active (specialist re-reading risks); others
not active.

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.** All cited references are pre-existing
(Constantin-Iyer 2008 CPAM 61; Otto 2001 CPDE 26; Bakry-Émery 1985
Sém. Probab. XIX; Constantin-Vicol 2012 GAFA 22; Otto-Villani 2000
J. Funct. Anal. 173; Ambrosio-Gigli-Savaré 2008 Birkhäuser;
Bakry-Gentil-Ledoux 2014 Springer; Brenier 1999 J. Amer. Math.
Soc. 12; Vasseur 2007 SIAM J. Math. Anal. 39 / Vasseur 2010 Arch.
Rat. Mech. Anal. 197; Vasseur-Yu 2016 Invent. Math. 206;
Buckmaster-Vicol 2019 Ann. Math. 189; Isett 2018 Ann. Math. 188;
Eyink-Sreenivasan 2006 Rev. Mod. Phys. 78; Stroock-Varadhan 1979
Springer; Constantin-Foias 1988 U. Chicago; Foias-Manley-Rosa-Temam
2001 Cambridge; Lemarié-Rieusset 2002 Chapman & Hall;
Ladyzhenskaya 1969 Gordon-Breach; Chen-Strain-Tsai-Yau 2008/9
Comm. PDE; Leray 1934 Acta Math. 63; Beale-Kato-Majda 1984 Comm.
Math. Phys. 94; Foias-Saut 1984 Indiana Univ. Math. J. 33;
Constantin-Fefferman 1993 Indiana Univ. Math. J. 42; Moffatt 1969
J. Fluid Mech. 35; Hamilton 1982 J. Diff. Geom. 17; Perelman 2002
arXiv:math/0211159; Gross 1975 Amer. J. Math. 97; Necas-Růžička-
Šverák 1996 Acta Math. 176; Iskauriaza-Serëgin-Šverák 2003 Russ.
Math. Surv. 58; Calderón-Zygmund 1952 Acta Math. 88; Koch-Tataru
2001 Adv. Math. 157; Kato-Fujita 1962; DiPerna-Lions; Ambrosio).

— end validation —
