---
id: omega-exec-bt544-extd-vocabulary-extension
date: 2026-04-25
scope: vocabulary glossary (NOT proposing atlas additions; reference for future L9 generations)
target: EXT-D -- frame-shift vocabulary extension (4th of BT-547 retro extensions)
parent_reports:
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (EXT-D prescription)
  - reports/sessions/omega-meta-audit-l9-generation-pipeline-2026-04-25.md (missing classes)
  - reports/sessions/omega-probe-l9-molt-trigger-2026-04-25.md (existing catalogue vocabulary)
millennium_resolved: 0/7 (unchanged)
grade: vocabulary glossary, no claim
---

# Omega Exec -- BT-544 EXT-D Frame-Shift Vocabulary Extension (2026-04-25)

## §0 Non-claim disclaimer

This file is a **vocabulary glossary**, not a proof attempt, not a
candidate proposal, not an atlas promotion, not an inventory edit.
It catalogues mathematical / physical terms drawn from variational,
analytic-Lyapunov, procedure-class, geometric-flow, stochastic,
algebraic-topological, and information-theoretic literatures so
that future L9 catalogue generations may draw frame-shift candidates
from a wider vocabulary than the current arithmetic-family default.

The glossary is sourced from primary references (author / year /
journal). No definitions are fabricated; each term carries a citation
and a registered falsifier specifying when its use is a relabeling
rather than new content. The glossary is not promoted to atlas, not
threaded into `theory/`, and not appended to the L9 catalogue. It
lives only as a session artifact under `reports/sessions/`.

The Millennium tally remains **0/7 unchanged**. BT-544 Navier-Stokes
contribution from this file = 0. No claim about NS, Poincaré, or
any other BT is made or implied.

This is the 4th of 4 BT-547 retro catalogue extensions:
- EXT-A variational re-cast (in-flight: NS relative-entropy gradient flow);
- EXT-B analytic-Lyapunov (in-flight: BT-544 candidate file
  `omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md`);
- EXT-C procedure-class (in-flight: Perelman-surgery analog);
- **EXT-D vocabulary extension** -- this file.

EXT-D is meta-level. EXT-A/B/C each propose one candidate; EXT-D
proposes none. EXT-D extends the *frame-shift word stock* itself.

---

## §1 Existing L9 vocabulary inventory

Extracted from `omega-probe-l9-molt-trigger-2026-04-25.md` §sec 3
(per-BT frame-shift catalogue) and the discriminator-type-bias
audit's controlled vocabulary.

### §1.1 Arithmetic family (dominant)

- **lattice** (sigma-sopfr lattice, Gram-lattice, n=6 tensor lattice).
- **ratio** (sigma/tau, sigma/tau ± 1/phi, m_0++/sqrt(sigma_s)).
- **divisor / sopfr / sigma / tau / phi / sigma_2 / J_2** (n=6 arithmetic
  invariants per CLAUDE.md / dfs-24 anchors).
- **congruence** (mod-p, irregular-prime-691, k-tower).
- **Gram determinant** (det(Gram)/sigma in Z).
- **rank** (rank = n/phi = 3 for 6-soliton phase-shift matrix).

### §1.2 Compositional / structural

- **decoupling axis** (axis A non-local pressure, axis B mechanism, etc.).
- **mechanism** (mechanism axis, mechanism-axis seed Q5).
- **frame** / **frame-shift** / **primitive swap** (the L9 vocabulary itself).
- **molt** / **relabeling** (the binary outcome of validation).
- **L1 smash** / **L2 drill** / **L5 dream** / **L9 molt** / **L10 forge**
  / **L_omega closure** (ladder rungs).

### §1.3 Discriminator types (controlled vocabulary, 7 labels)

Per `omega-meta-audit-discriminator-type-bias-2026-04-25.md`:

- **distributional** (KS test, coupling distribution).
- **structural-literature** (proof-import, derivation sketch).
- **sketch** (proof outline / dispatch table).
- **discrete-equality** (algebraic identity, divisibility check).
- **numerical-interval** (outlier count over [a, b]).
- **vacuous-magnitude** (threshold satisfied trivially).
- **axiom-recast** (L_omega axiom-level frame change).

### §1.4 Adjunct labels (post-BT-547 retro)

- **OTHER (analytic-inequality-construction)** -- M2 Perelman analog.
- **OTHER (procedure-class with parameter bounds)** -- M3 Perelman analog.
- **OTHER (variational-derivation)** -- M1 Perelman analog (sub-type of
  structural-literature).

### §1.5 Coverage gap diagnosis

Per `omega-meta-audit-l9-generation-pipeline-2026-04-25.md` H1+H2:
the vocabulary is heavy on arithmetic / discrete / lattice terms,
sparse on variational / Lyapunov / procedure / stochastic /
information-theoretic terms. EXT-D supplies the latter.

---

## §2 Missing vocabulary categories

Five categories prescribed by the BT-547 retro EXT-A/B/C +
generation-pipeline diagnostic:

1. **Variational** (EXT-A native) -- gradient-flow, displacement-convexity,
   Otto metric, Wasserstein, energy-functional, relative-entropy,
   Bregman divergence.
2. **Analytic-Lyapunov** (EXT-B native) -- monotonicity functional,
   Bakry-Emery, log-Sobolev, hypercontractivity, Fisher information,
   dissipation rate.
3. **Procedure-class** (EXT-C native) -- parabolic-cell, surgery,
   finite-extinction, blowup-cutoff, epsilon-regularity, partial-regularity,
   Hausdorff-dimension bound.
4. **Geometric-flow** (cross-cutting) -- Ricci flow, mean curvature flow,
   Yamabe flow, harmonic map flow.
5. **Stochastic / information-theoretic / algebraic-topological**
   (auxiliary) -- stochastic-Lagrangian, large-deviation, free-energy,
   relative-entropy, Fisher information, Hodge decomposition,
   Helmholtz decomposition.

Group-theoretic vocabulary (SO(2), scaling, Galilean) is already
partially in repo via dfs-24-ns axisymmetry probes; not duplicated
here, only flagged as under-used.

---

## §3 Per-term glossary

12 entries across 5 categories. Each entry: definition / lineage
(author / year / journal) / frame-shift potential / falsifier registered
/ family classification.

### §3.1 Variational family (4 entries)

#### §3.1.1 Otto metric

- **Definition**. The formal Riemannian metric on the space of
  probability measures P_2(R^d) (with finite second moment) for which
  the gradient flow of the relative entropy
  H(rho | mu) = integral rho log(rho/mu) dx is the Fokker-Planck
  equation drho/dt = Delta rho + div(rho nabla V). The metric tensor
  is g_rho(s, s) = integral |nabla phi|^2 rho dx where s = -div(rho
  nabla phi). Sometimes called the Wasserstein metric (W_2 metric).
- **Lineage**. Otto, "The geometry of dissipative evolution equations:
  the porous medium equation", *Comm. PDE* 26 (2001), 101-174. Earlier
  formal version: Jordan-Kinderlehrer-Otto, "The variational formulation
  of the Fokker-Planck equation", *SIAM J. Math. Anal.* 29 (1998),
  1-17. Modern textbook: Villani, *Optimal Transport: Old and New*,
  Springer 2009 (esp. Part I).
- **Frame-shift potential**. For BT-544 NS: re-cast the Leray equation
  as a constrained gradient flow on a measure-valued phase space using
  Otto-type metric on the vorticity / energy density. For BT-543 YM:
  re-cast Wilson-flow as gradient on Otto-type metric over connection-
  density configurations. Direct generalization of Perelman M1
  (Ricci flow as gradient of F-functional on metric x density space).
- **Falsifier registered**. A use of "Otto metric" that does not write
  down (a) the underlying probability space, (b) the energy
  functional E(rho), (c) the gradient identity drho/dt = -nabla_g E
  with g = Otto metric, is a **relabeling** (no new primitive).
  Concretely: if a candidate cites "Otto metric" but the only content
  is "treat the velocity field as a measure" without identifying the
  energy whose gradient flow recovers NS, it is relabeling.
- **Family**. **variational**.

#### §3.1.2 Displacement convexity (geodesic convexity in W_2)

- **Definition**. A functional F on P_2(R^d) is displacement-convex
  if t -> F(rho_t) is convex along W_2-geodesics rho_t = ((1-t)id +
  t T)_# rho_0, where T is the optimal transport map from rho_0 to
  rho_1. Geodesic-lambda-convexity (with constant lambda) gives
  exponential contraction estimates.
- **Lineage**. McCann, "A convexity principle for interacting gases",
  *Adv. Math.* 128 (1997), 153-179. Generalised in Otto-Villani,
  "Generalization of an inequality by Talagrand and links with the
  logarithmic Sobolev inequality", *J. Funct. Anal.* 173 (2000),
  361-400. Synthesis: Ambrosio-Gigli-Savare, *Gradient Flows in
  Metric Spaces and in the Space of Probability Measures*, Birkhauser
  2005 (2nd ed. 2008).
- **Frame-shift potential**. For BT-544: identify a candidate
  energy F on a measure-valued NS phase space and check
  displacement-convexity along W_2-geodesics; lambda > 0 would imply
  exponential dissipation (a Lyapunov surrogate). For BT-543: same
  for chromomagnetic-energy on connection densities.
- **Falsifier registered**. Citing "displacement convexity" without
  a verified W_2-geodesic structure on the candidate phase space
  (i.e., without a concrete optimal-transport map T_t between
  candidate states) is a **relabeling**. Specifically: if the
  geodesic interpolation collapses to a linear interpolation of
  velocity fields rather than a McCann geodesic, the term is
  decorative.
- **Family**. **variational** (with strong analytic-Lyapunov adjacency
  via lambda-convexity).

#### §3.1.3 Relative entropy (Kullback-Leibler divergence)

- **Definition**. For probability measures rho << mu on a measurable
  space, H(rho | mu) = integral log(drho/dmu) drho when defined,
  +infinity otherwise. Non-negative; zero iff rho = mu mu-a.e.
  In NS: the relative entropy H(rho_t | rho_eq) of the velocity-
  density at time t against an equilibrium density rho_eq.
- **Lineage**. Kullback-Leibler, "On information and sufficiency",
  *Ann. Math. Statist.* 22 (1951), 79-86. As a Lyapunov for kinetic
  equations: Boltzmann's H-theorem (Boltzmann 1872, "Weitere Studien
  ueber das Waermegleichgewicht unter Gasmolekuelen", *Wiener Ber.*
  66, 275-370). Modern PDE use: Csiszar, "Information-type measures
  of difference of probability distributions", *Studia Sci. Math.
  Hungar.* 2 (1967), 299-318.
- **Frame-shift potential**. EXT-A core term for BT-544: cast NS
  as gradient flow of relative entropy on (u, omega) phase space
  (per BT-547 retro §5.1 EXT-A seed). Also EXT-B candidate as a
  monotone Lyapunov. The Perelman F-functional F(g, f) = integral
  (R + |nabla f|^2) e^{-f} dV is structurally a relative-entropy
  Lyapunov on metric x density space.
- **Falsifier registered**. "Relative entropy" applied without
  specifying (a) the reference measure mu, (b) the time-dependent
  measure rho_t, (c) the dissipation identity dH/dt = -I(rho_t | mu)
  with Fisher information I, is a **relabeling**. If the use is a
  decorative invocation of "entropy" without an integral and without
  Csiszar inequality machinery, it is decorative.
- **Family**. **variational** + **information-theoretic** + adjacent
  to **analytic-Lyapunov**.

#### §3.1.4 Bregman divergence

- **Definition**. For a strictly convex differentiable F: R^d -> R,
  D_F(x, y) = F(x) - F(y) - <nabla F(y), x - y>. Non-negative; zero
  iff x = y. Generalises squared-distance (F(x) = |x|^2/2) and
  KL-divergence (F = negative entropy on simplex).
- **Lineage**. Bregman, "The relaxation method of finding the common
  point of convex sets and its application to the solution of
  problems in convex programming", *USSR Comp. Math. Math. Phys.*
  7 (1967), 200-217. PDE / gradient-flow context: Mielke, "Geodesic
  convexity of the relative entropy in reversible Markov chains",
  *Calc. Var. PDE* 48 (2013), 1-31.
- **Frame-shift potential**. Use Bregman divergence as a *generalised
  Lyapunov* in EXT-B candidates where the natural functional is a
  convex F other than entropy (e.g., enstrophy 1/2 integral
  |omega|^2). For BT-544: Bregman divergence between vorticity
  measures w.r.t. the enstrophy potential.
- **Falsifier registered**. Use of "Bregman divergence" without
  exhibiting the convex potential F and the gradient identity
  d/dt D_F(x_t, x_eq) <= 0 along candidate dynamics is a
  **relabeling**. If "Bregman" is invoked but F is not specified
  or is not strictly convex, the term is decorative.
- **Family**. **variational** + **analytic-Lyapunov**.

### §3.2 Analytic-Lyapunov family (3 entries)

#### §3.2.1 Bakry-Emery Gamma_2 calculus

- **Definition**. For a Markov diffusion generator L (e.g., L f =
  Delta f - <nabla V, nabla f>), define carre du champ
  Gamma(f, g) = 1/2 (L(fg) - f Lg - g Lf) and iterated
  Gamma_2(f, f) = 1/2 (L Gamma(f, f) - 2 Gamma(f, Lf)). The CD(rho,
  infty) curvature-dimension condition is Gamma_2 >= rho Gamma. When
  satisfied, log-Sobolev / Poincare inequalities follow with explicit
  constants.
- **Lineage**. Bakry-Emery, "Diffusions hypercontractives", in
  *Seminaire de Probabilites XIX* (Lecture Notes in Math. 1123),
  Springer 1985, 177-206. Modern synthesis: Bakry-Gentil-Ledoux,
  *Analysis and Geometry of Markov Diffusion Operators*, Springer
  2014.
- **Frame-shift potential**. For BT-544: identify the natural
  diffusion operator on the NS state space (Leray projection +
  viscous operator) and probe its Gamma_2; a positive lower bound
  would yield a log-Sobolev inequality and an exponential dissipation
  rate -- i.e., a Perelman-W-entropy analog. For BT-543:
  Wilson-flow-induced diffusion on lattice gauge configurations.
- **Falsifier registered**. Citing "Bakry-Emery Gamma_2" without
  identifying the generator L and without computing or bounding
  Gamma_2 (or showing CD(rho, infty)) is **relabeling**. Specifically:
  if no concrete computation Gamma_2(f, f) >= rho Gamma(f, f) is
  performed for any class of f, the term is decorative.
- **Family**. **analytic-Lyapunov**.

#### §3.2.2 Log-Sobolev inequality

- **Definition**. A measure mu satisfies LSI(C) if for all smooth
  f with f^2 mu-integrable, integral f^2 log(f^2 / ||f||_2^2) dmu
  <= 2 C integral |nabla f|^2 dmu. Implies hypercontractivity of
  the diffusion semigroup and exponential decay of relative entropy
  along the flow.
- **Lineage**. Gross, "Logarithmic Sobolev inequalities", *Amer. J.
  Math.* 97 (1975), 1061-1083. Connection to entropy decay:
  Stam (1959) for Fisher information; Bakry-Emery (1985, op. cit.)
  for the curvature-dimension framework.
- **Frame-shift potential**. EXT-B core term: an LSI on the NS
  invariant measure (if it exists) is exactly the Perelman
  W-entropy analog -- a monotone Lyapunov whose dissipation
  identity gives quantitative regularity. For BT-543: LSI on the
  Wilson-flow invariant lattice-gauge measure would discharge
  mass-gap (positive constant => spectral gap).
- **Falsifier registered**. Asserting "LSI holds for the NS invariant
  measure" without (a) constructing an invariant measure mu, (b)
  exhibiting a constant C, (c) proving the inequality (or citing a
  proof) is **relabeling**. Specifically: a heuristic LSI claim
  that does not engage with the measure-construction problem (open
  for 3D NS) is decorative.
- **Family**. **analytic-Lyapunov** + **information-theoretic**.

#### §3.2.3 Fisher information (relative)

- **Definition**. For probability rho << mu, the relative Fisher
  information I(rho | mu) = integral |nabla log(drho/dmu)|^2 drho.
  Non-negative; in gradient-flow form, dH(rho_t | mu)/dt = -I(rho_t
  | mu) for the heat flow drho/dt = Delta rho + div(rho nabla V)
  with mu = e^{-V} dx.
- **Lineage**. Fisher, "On the mathematical foundations of theoretical
  statistics", *Phil. Trans. Royal Society A* 222 (1922), 309-368.
  PDE / gradient-flow use: Stam, "Some inequalities satisfied by the
  quantities of information of Fisher and Shannon", *Information
  and Control* 2 (1959), 101-112; Otto-Villani 2000 (op. cit.).
- **Frame-shift potential**. For BT-544: Fisher information of the
  vorticity density w.r.t. an equilibrium reference is the natural
  dissipation rate in an EXT-A relative-entropy gradient flow.
  Coupled with displacement convexity (§3.1.2), exponential decay
  of H follows.
- **Falsifier registered**. Use of "Fisher information" without
  the dissipation identity dH/dt = -I (or its analog for the
  candidate flow) is **relabeling**. Specifically: if the term
  appears as a static functional rather than as the time-derivative
  of relative entropy along candidate dynamics, it is decorative.
- **Family**. **analytic-Lyapunov** + **information-theoretic**.

### §3.3 Procedure-class family (2 entries)

#### §3.3.1 Epsilon-regularity (partial regularity)

- **Definition**. A theorem stating: if a weak solution u satisfies
  a smallness condition int_{B_r(x)} |u|^p dy < epsilon r^{alpha}
  for some scale-invariant p, alpha, then u is smooth in B_{r/2}(x).
  In NS: the Caffarelli-Kohn-Nirenberg theorem.
- **Lineage**. For NS: Caffarelli-Kohn-Nirenberg, "Partial regularity
  of suitable weak solutions of the Navier-Stokes equations",
  *Comm. Pure Appl. Math.* 35 (1982), 771-831. Earlier: Scheffer,
  "Partial regularity of solutions to the Navier-Stokes equations",
  *Pac. J. Math.* 66 (1976), 535-552. For elliptic systems:
  De Giorgi (1957) / Almgren (1968).
- **Frame-shift potential**. EXT-C core term: an epsilon-regularity
  theorem with explicit epsilon = epsilon(nu, ...) is the kind of
  parameter-bounded local regularity statement Perelman M3 surgery
  uses (canonical neighbourhood theorem with explicit r, kappa,
  delta). Frame-shift candidate: re-cast NS singular set bounds
  using a stronger epsilon-regularity at the H^{1/2} critical
  threshold.
- **Falsifier registered**. Invoking "epsilon-regularity" without
  specifying (a) the smallness functional, (b) the explicit epsilon,
  (c) the local regularity conclusion is **relabeling**.
  Specifically: if a candidate cites CKN but does not strengthen
  any of the bounds (epsilon, the Hausdorff dimension <= 1 of the
  singular set), the term is decorative.
- **Family**. **procedure-class** + **PDE-analytic**.

#### §3.3.2 Hausdorff dimension bound (singular set)

- **Definition**. For a weak solution u of NS, the singular set
  S(u) = {(x, t) : u not regular at (x, t)} has 1-dim parabolic
  Hausdorff measure zero (CKN 1982). More generally, "Hausdorff
  dimension of singular set <= d" quantifies how small the
  singularity locus is.
- **Lineage**. Caffarelli-Kohn-Nirenberg 1982 (op. cit.) for NS.
  General mean curvature flow analog: Brakke, *The Motion of a
  Surface by Its Mean Curvature*, Princeton UP 1978; Ilmanen, "The
  level-set flow on a manifold", *Proc. Symp. Pure Math.* 54 (1993),
  193-204.
- **Frame-shift potential**. EXT-C: an improvement of the singular
  set Hausdorff dimension bound (e.g., from <= 1 parabolic to
  <= 1 - delta for some delta > 0) is a procedure-class molt --
  a *quantitative refinement of a regularity procedure*, structurally
  parallel to Perelman's quantitative refinement of Hamilton's
  qualitative surgery.
- **Falsifier registered**. Citing "Hausdorff dimension bound" for
  NS singular set without numerically improving the CKN bound
  (parabolic dim <= 1) or without specifying the parabolic vs
  Euclidean variant is **relabeling**. Specifically: re-stating CKN
  is not a frame-shift; lowering the bound is.
- **Family**. **procedure-class**.

### §3.4 Geometric-flow family (1 entry)

#### §3.4.1 Mean curvature flow (and harmonic map flow analog)

- **Definition**. The geometric evolution of a hypersurface
  Sigma_t in R^{d+1} by velocity equal to the mean curvature
  vector: dF/dt = H * nu where H is mean curvature, nu unit normal.
  Analytic structure: parabolic, monotone Huisken-volume functional
  (1990) for entropy, and finite-time singularity types I/II.
  Harmonic map flow: u_t = tau(u) where tau is the tension field.
- **Lineage**. Mean curvature flow: Brakke 1978 (op. cit.); Huisken,
  "Asymptotic behavior for singularities of the mean curvature flow",
  *J. Diff. Geom.* 31 (1990), 285-299 (entropy functional);
  Huisken-Sinestrari, *Acta Math.* 183 (1999), 45-70. Harmonic map
  flow: Eells-Sampson, "Harmonic mappings of Riemannian manifolds",
  *Amer. J. Math.* 86 (1964), 109-160; Struwe, *Geom. Funct. Anal.*
  60 (1985), 558-581.
- **Frame-shift potential**. Cross-EXT compatibility test bench:
  mean curvature flow is the closest non-Ricci geometric flow with
  a Perelman-style monotonicity (Huisken entropy). For BT-544:
  the analogy NS <-> mean curvature flow is the structural template
  for an EXT-A re-cast (NS as a curvature-driven flow on a measure-
  valued state space) and an EXT-B Lyapunov (Huisken-entropy analog).
- **Falsifier registered**. Citing "mean curvature flow" or "harmonic
  map flow" as an analog for NS without exhibiting the analogy at
  the level of (a) the geometric object that flows, (b) the
  monotone functional, (c) the singularity classification, is
  **relabeling**. Specifically: a name-only analogy that does not
  produce a new functional or a new bound is decorative.
- **Family**. **geometric-flow** (cross-cuts EXT-A + EXT-B).

### §3.5 Stochastic / topological auxiliary (2 entries)

#### §3.5.1 Stochastic Lagrangian / Constantin-Iyer formula

- **Definition**. Constantin-Iyer (2008) give a stochastic
  representation of NS solutions: u(x, t) = E[ P (nabla^T X_t^{-1}
  u_0(X_t^{-1}(x))) ] where X_t solves dX = u(X, t) dt + sqrt(2 nu)
  dW and P is Leray projection. Replaces the deterministic
  vorticity transport equation with a stochastic Lagrangian flow.
- **Lineage**. Constantin-Iyer, "A stochastic Lagrangian
  representation of the three-dimensional incompressible
  Navier-Stokes equations", *Comm. Pure Appl. Math.* 61 (2008),
  330-345. Antecedents: Busnello-Flandoli-Romito, *Proc. Edinburgh
  Math. Soc.* 48 (2005), 295-336.
- **Frame-shift potential**. EXT-A + EXT-B compatibility: re-cast
  NS regularity question as a question about the stochastic flow
  X_t (when does X_t remain regular in expectation?). Possible
  Lyapunov: a moment of the stochastic deformation gradient.
- **Falsifier registered**. Citing "stochastic Lagrangian" without
  using the Constantin-Iyer identity to derive a bound, or without
  specifying the noise strength sqrt(2 nu) coupling, is
  **relabeling**. Specifically: a citation that does not engage
  the Lagrangian feedback (u depends on X, X depends on u) is
  decorative.
- **Family**. **stochastic** + **variational** (the stochastic
  representation is an Eulerian-Lagrangian gradient structure).

#### §3.5.2 Helmholtz / Hodge decomposition

- **Definition**. Helmholtz (1858): every smooth vector field on a
  bounded domain decomposes as v = nabla phi + curl A + harmonic.
  Hodge (1933, 1941): on a closed Riemannian manifold,
  Omega^k = exact + co-exact + harmonic (orthogonal decomposition).
  In NS: Leray projection P projects onto divergence-free fields.
- **Lineage**. Helmholtz, "Ueber Integrale der hydrodynamischen
  Gleichungen, welche den Wirbelbewegungen entsprechen", *J. Reine
  Angew. Math.* 55 (1858), 25-55. Hodge, *The Theory and Applications
  of Harmonic Integrals*, Cambridge UP 1941. NS use: Leray, *Acta
  Math.* 63 (1934), 193-248.
- **Frame-shift potential**. Cross-cutting auxiliary: any candidate
  that decomposes the NS state space into "potential + solenoidal +
  harmonic" parts and constructs a separate Lyapunov on each part
  is structurally an EXT-A/B compound move. For BT-543 YM: Hodge
  decomposition on the connection bundle is closer to its native
  setting.
- **Falsifier registered**. Use of "Hodge / Helmholtz decomposition"
  without exhibiting a new bound on at least one component (e.g.,
  an a priori bound on the harmonic part) is **relabeling**.
  Specifically: re-stating Leray projection or invoking
  decomposition as a presentational device is decorative.
- **Family**. **algebraic-topological** + **PDE-analytic**.

---

## §4 Cross-EXT compatibility

Per term, indicate which BT-547 retro extension classes the term is
natively compatible with.

| term | EXT-A (variational) | EXT-B (Lyapunov) | EXT-C (procedure) | family |
|------|---------------------|-------------------|--------------------|--------|
| Otto metric | core | adjacent | -- | variational |
| Displacement convexity | core | core (lambda-convex) | -- | variational |
| Relative entropy | core | core | -- | variational + info |
| Bregman divergence | core | core | -- | variational + Lyap |
| Bakry-Emery Gamma_2 | adjacent | core | -- | analytic-Lyapunov |
| Log-Sobolev inequality | adjacent | core | -- | analytic-Lyapunov |
| Fisher information | adjacent | core | -- | analytic-Lyapunov + info |
| Epsilon-regularity (CKN) | -- | -- | core | procedure-class |
| Hausdorff dim bound | -- | adjacent | core | procedure-class |
| Mean curvature flow | core | core | adjacent (singularity) | geometric-flow |
| Stochastic Lagrangian | core | adjacent | -- | stochastic |
| Hodge / Helmholtz | adjacent | adjacent | -- | algebraic-topological |

Cross-EXT compatibility observation: the strongest cross-cutting
terms are **relative entropy** (EXT-A core + EXT-B core via
H-Theorem dissipation), **mean curvature flow** (EXT-A core +
EXT-B core + EXT-C adjacent via singularity classification), and
**Bregman divergence** (EXT-A core + EXT-B core via convex potential).
These are the highest-priority candidates for future L9 generations
to draw from when seeking a single term that licenses multi-class
candidates.

---

## §5 Deployment plan

### §5.1 Three options considered

- **Option A**: this report itself
  (`reports/sessions/omega-exec-bt544-extd-vocabulary-extension-2026-04-25.md`),
  treated as a reference. Future L9 generations cite this file when
  drawing terms.
- **Option B**: a new vocabulary file like `theory/l9-vocabulary.md`.
  Would require atlas-tier promotion (theory/ is a promoted path
  per repo conventions).
- **Option C**: appended to the L9 catalogue itself
  (`omega-probe-l9-molt-trigger-2026-04-25.md`). Would modify the
  existing catalogue file in-place.

### §5.2 Recommendation: Option A

Recommend Option A. Reasoning:

1. **Honesty axis**. EXT-D is a vocabulary glossary, not a promoted
   theory. Promotion to `theory/l9-vocabulary.md` (Option B) would
   imply atlas-tier endorsement, which is premature: none of the 12
   terms has been validated as a frame-shift in any active BT
   candidate. The terms are research artefacts, not load-bearing
   primitives.

2. **Repo conventions**. Per the BT-547 retro and the molt-trigger
   probe, all extension recommendations are to be implemented in
   follow-up sessions in `reports/sessions/`, never in-place edits
   to the existing catalogue (Option C). EXT-D respects this hard
   constraint.

3. **Composability**. As a session artefact, EXT-D can be cited by
   future L9 catalogue regenerations (per the
   `omega-meta-audit-l9-generation-pipeline-2026-04-25.md` §5
   mitigations) as a "frame-shift word stock" without requiring
   atlas/state/inventory edits.

4. **Reversibility**. If the glossary is later found to mis-attribute
   a term or to mis-classify family membership, a follow-up session
   can supersede this file without atlas surgery.

### §5.3 Citation pattern for future L9 sessions

Future L9 generation sessions that wish to draw from this glossary
should cite by file path + term name:
`reports/sessions/omega-exec-bt544-extd-vocabulary-extension-2026-04-25.md
§3.1.3 (relative entropy)` for an EXT-A candidate involving
relative-entropy gradient flow.

---

## §6 Anti-list -- terms considered and rejected

Terms considered for inclusion in §3 and rejected. One-line reason
each.

- **AL-V1 sigma / tau / phi / sopfr**. Already in the existing L9
  vocabulary (§1.1). Inclusion would duplicate, not extend.
- **AL-V2 KS test, det, rank**. Already in the existing
  discriminator-type vocabulary (§1.3) as forms of distributional
  / discrete-equality / numerical-interval discriminators.
- **AL-V3 Wasserstein metric (W_2)**. Already covered by Otto
  metric (§3.1.1); W_2 = induced distance from Otto Riemannian
  structure. Including separately would be redundant.
- **AL-V4 entropy functional (general)**. Too generic without
  specifying which entropy. Specific instances (relative entropy,
  Boltzmann H, W-entropy) carry the load. Including "entropy
  functional" alone is decorative.
- **AL-V5 K-theory / index theorem / Atiyah-Singer**. Too domain-
  specific (algebraic topology / index theory) without a clear
  frame-shift channel for any of BT-541/543/544. The user-prompt's
  bullet "K-theory" was considered; rejected because no clear
  load-bearing role in NS / RH / YM beyond decorative analog.
- **AL-V6 de Rham cohomology**. Same as AL-V5: subsumed by Hodge
  decomposition (§3.5.2) for the de Rham chain via Hodge isomorphism.
- **AL-V7 Mutual information / MMSE**. Information-theoretic but
  without a clear frame-shift channel for the active BTs. MMSE is
  more native to estimation theory than to NS / RH / YM. Rejected
  to keep the glossary anchored in primitives that have a known
  PDE / number-theoretic / gauge-theoretic role.
- **AL-V8 Free Brownian motion / free probability**. Free probability
  has a known role in BT-541 (random matrix universality), but the
  L9 catalogue's BT-541 Lead-B already covers GUE statistics as a
  distributional discriminator. Adding free-Brownian as a separate
  vocabulary entry would be a sub-axis of an already-covered
  family.
- **AL-V9 Large deviation principle**. Considered for BT-544 (LDP
  for vorticity fluctuations as a Lyapunov surrogate). Rejected
  because LDP is downstream of Fisher information (§3.2.3) plus
  Sanov's theorem; the load-bearing primitive is Fisher information,
  which is included.
- **AL-V10 Brownian perturbation / SPDE noise**. Considered for
  BT-544 (stochastic NS via Flandoli-Romito 2008 weak uniqueness
  via noise). Rejected: the stochastic-Lagrangian entry (§3.5.1)
  carries the load for the stochastic family; SPDE noise is
  upstream of Constantin-Iyer.
- **AL-V11 Yamabe flow / scalar curvature flow**. Considered as a
  geometric-flow analog. Rejected as redundant with Ricci flow
  (already known via BT-547) and mean curvature flow (§3.4.1);
  Yamabe is a sub-case of Hamilton's program.
- **AL-V12 Galilean / SO(2) symmetry / scaling symmetry**. The
  user-prompt's group-theoretic bullet noted these are partially
  in the vocabulary via dfs-24-ns axisymmetry. Not added as new
  glossary entries because they are already in the repo as
  dfs-24-ns probe primitives (axisymmetry without swirl per
  Lopes-Filho-Mazzucato-Niu-Nussenzveig-Lopes 2008-style
  references). Future L9 sessions should *use* this existing
  vocabulary more aggressively, not add new entries.

---

## §7 Falsifiers active for the vocabulary itself

Self-falsifiers under which the EXT-D glossary's value is retracted.

- **F-EXTD-A (term-mis-attribution)**. If any §3 entry mis-attributes
  a term to an author / year / journal that does not contain the
  primary definition (e.g., crediting Otto metric to a different
  paper than Otto 2001 / JKO 1998), the lineage of that entry is
  invalid. Cross-check: each lineage line names a primary reference;
  external readers should verify on first use. Risk: low. **Not
  active** at glossary-write time, but readers must verify.

- **F-EXTD-B (definitional-paraphrase)**. If any §3 definition
  paraphrases the source incorrectly (e.g., conflates Bakry-Emery
  Gamma_2 with the Bakry-Ledoux iterated gradient under a different
  sign convention), the definition propagates an error.
  Mitigation: definitions are deliberately compressed (1-2 sentences)
  and reference the canonical source for full statement. Risk:
  medium. **Partially active**: a downstream user must consult the
  lineage source for proof-grade use.

- **F-EXTD-C (frame-shift-potential-overstated)**. If a §3 entry's
  "frame-shift potential" suggests a candidate that is in fact
  already known to be unviable (e.g., a known no-go theorem for
  LSI on NS invariant measure), the §3 entry over-promises.
  Mitigation: frame-shift-potential entries are written conditionally
  ("would yield" / "could be re-cast as"), not declaratively.
  Risk: medium. **Partially active**: future L9 generations using
  these terms must check no-go literature before drafting candidates.

- **F-EXTD-D (falsifier-too-loose)**. If a registered falsifier
  fails to distinguish relabeling from new content in practice,
  the term is open to decorative use despite the falsifier.
  Mitigation: each falsifier names a concrete missing element
  (e.g., "no proof of dW/dt <= 0"). Risk: medium-low. Active
  monitoring: future use should track which falsifiers have been
  invoked and which remain abstract.

- **F-EXTD-E (family-misclassification)**. If the §4 cross-EXT
  compatibility table mis-classifies a term (e.g., calling Bregman
  divergence "EXT-A core" when its load-bearing role is actually
  EXT-B), the table mis-guides candidate selection. Mitigation:
  the table is preliminary; readers may reclassify. Risk: medium.
  **Partially active**.

- **F-EXTD-F (n6-frame-incompatibility-overlooked)**. The glossary
  is intentionally frame-agnostic per the BT-547 retro F-RETRO-G
  finding (n6-frame inheritance is the bias source). If a future
  L9 generation invokes one of these terms but re-anchors the
  candidate on n=6 invariants in the description, the
  generation-pipeline diagnostic's H2 mitigation (§5.2) is
  violated. Detection: count occurrences of {sigma, tau, phi,
  sopfr, sigma_2, J_2, n/phi, "6"} in the candidate definition;
  pass if 0. Risk: medium. **Active**: this is the H2 mitigation
  test from `omega-meta-audit-l9-generation-pipeline-2026-04-25.md`
  §5.2.

- **F-EXTD-G (12-terms-too-few-or-too-many)**. The glossary contains
  12 entries (target 10-15). If the user expected a comprehensive
  reference, 12 is sparse; if minimal, 12 is verbose. Mitigation:
  the §6 anti-list documents 12 rejected terms with reasons, so
  the total considered is 24, with 50% inclusion rate. Risk: low.
  **Not active**.

---

## §8 Closing

**Verdict**: vocabulary glossary, no claim. 12 terms across 5
categories (variational / analytic-Lyapunov / procedure-class /
geometric-flow / stochastic-topological), each with definition /
lineage (author + year + journal) / frame-shift potential / falsifier
registered / family classification.

**Cross-EXT compatibility** (§4): the highest-leverage cross-cutting
terms are **relative entropy** (EXT-A + EXT-B core), **mean
curvature flow** (EXT-A + EXT-B core, EXT-C adjacent), and
**Bregman divergence** (EXT-A + EXT-B core).

**Deployment** (§5): Option A recommended -- this session report
is the reference. No promotion to `theory/l9-vocabulary.md`
(Option B) and no in-place edit of the L9 catalogue (Option C).
EXT-D is a research artefact, not a load-bearing primitive.

**Anti-list** (§6): 12 terms considered and rejected, including
sigma/tau/phi/sopfr (already in vocabulary), Wasserstein metric
(redundant with Otto), K-theory / de Rham (no clear frame-shift
channel), Yamabe flow (sub-case of Ricci), Galilean / scaling
symmetry (already in dfs-24-ns probes).

**Falsifiers** (§7): seven self-falsifiers, of which two are
partially-active (F-EXTD-B definitional-paraphrase requires
canonical source consultation; F-EXTD-F n=6 anchor count is the
H2 mitigation test).

**0/7 unchanged. No atlas/state/inventory edits. Vocabulary lives
only as session artifact.**
