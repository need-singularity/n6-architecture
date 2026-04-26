---
id: omega-meta-synthesis-3pillar-obstruction-localization
date: 2026-04-25
scope: research-only synthesis (NOT proving NS regularity; localizing obstruction)
target: BT-544 obstruction map -- D3.A + EXT-A + EXT-B unified picture
parent_reports:
  - reports/sessions/omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md (Pillar 1 PASS_LITERATURE)
  - reports/sessions/omega-exec-bt544-exta-variational-recast-candidate-2026-04-25.md (Pillar 2 OBSTRUCTION_DOCUMENTED at F-EXTA-C)
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md (Pillar 3 OBSTRUCTION_DOCUMENTED at F-EXTB-D + Path-Q cross term)
  - reports/sessions/omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md (D3 axis B prediction §4.2)
millennium_resolved: 0/7 (unchanged)
grade: cross-pillar synthesis, no claim
---

# Omega Meta-Synthesis -- BT-544 3-Pillar Obstruction Localization (2026-04-25)

## §0 Non-claim disclaimer

This file is a **synthesis** of three independent attacks on the
BT-544 NS regularity question (D3.A discriminator, EXT-A
variational gradient-flow recast, EXT-B Constantin-Iyer Lyapunov)
that each reached a partial verdict in 2026-04-25 reports. It does
**not**:

- claim 3D Navier-Stokes regularity (smoothness or blow-up);
- promote any atlas entry, modify `state/proposals/inventory.json`,
  modify `theory/canon/`, or alter the `BT-544 = 0/1 untouched`
  Clay status;
- prove a new theorem; every cited result is pre-existing in the
  published NS / variational / stochastic-Lagrangian literature
  and is referenced by author / year as required;
- supersede the per-pillar verdicts. The synthesis only
  *aggregates* what the three pillars say about *where the
  obstruction lives*.

The localization claim of §3 is **consistent with** the three
pillars, not **proved by** them. Any cross-pillar conclusion is
weaker than the per-pillar verdicts on which it rests.

**Millennium tally**: 0/7 unchanged. **NS regularity status:
open.** No atlas / state / inventory edits.

---

## §1 The three pillars (verdicts + key mechanisms)

### §1.1 Pillar 1 -- D3.A (Sym² / pressure non-locality axis)

**Source**: `reports/sessions/omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md`.

**Verdict**: **PASS_LITERATURE** (Path P).

**Setup**: the 2.5D ansatz v = e_3 × ∇⊥ψ + w·e_3 (D3.A §1)
decouples 3D NS into:

  ∂_t ω_h + u_h · ∇_h ω_h = ν Δ_h ω_h        (2D NS, vorticity form)
  ∂_t w   + u_h · ∇_h w   = ν Δ_h w           (2D linear adv-diff)

Vortex-stretching (ω·∇)v vanishes identically in this ansatz
(D3.A §1 paragraph "Vortex-stretching is absent"). The non-local
pressure is a 2D Riesz-transform composition acting on the planar
nonlinearity (D3.A §1, Step 6 of §4).

**Key mechanisms cleared**:

- 2D NS global regularity: Ladyzhenskaya 1959/1969;
- linear advection-diffusion regularity: Friedman 1964;
  Ladyzhenskaya-Solonnikov-Ural'ceva 1968;
- Riesz-transform H^s boundedness (pressure non-locality):
  Calderón-Zygmund 1952;
- BKM in 2D automatic via maximum principle on ω_h (Beale-Kato-
  Majda 1984).

**Statement**: pressure non-locality is *not* the obstruction
when the other two axes are decoupled. The 2.5D system is
literature-settled.

**Caveat from D3.A §6**: the verdict is "PASS via literature"
because the 2.5D system reduces to known-regular components; it
is *not* a Clay-relevant result. The compositional information
gain is real (axis A's cleanness is recorded), but the per-axis
estimate is not novel.

### §1.2 Pillar 2 -- EXT-A (uω-GradFlow / variational recast)

**Source**: `reports/sessions/omega-exec-bt544-exta-variational-recast-candidate-2026-04-25.md`
(the candidate report's §3.6 + §5.3 + §6 deliver Path R
on inspection; no separate validation file is required because
the obstruction is structurally visible at the candidate-spec
level).

**Verdict**: **OBSTRUCTION_DOCUMENTED** at F-EXTA-C
(convective-not-encodable). Path R fires by direct Helmholtz-
condition inspection (EXT-A §6.1 Path R; §3.6 first
identification).

**Setup**: candidate functional F[u, ω] (kinetic + enstrophy +
bi-Laplacian) on a Leray-projected-L² flat metric g; conjectured
gradient flow ∂_t u = −grad_g F.

**The breaking mechanism (EXT-A §3.6 + §5.3)**: the convective
term (u·∇)u of NS has Fréchet derivative

  D[(u·∇)u][v] = (v·∇)u + (u·∇)v

which is **not symmetric** in (u, v) under the L² inner product
— the asymmetry is precisely the vortex-stretching component.
By Olver 1986 (*Applications of Lie Groups to Differential
Equations*, Springer, Chapter 5), a vector field is a
variational gradient iff its Helmholtz conditions (symmetry of
Fréchet derivative) hold. The convective term fails this test.

**Status of falsifiers** (from EXT-A §5):

- F-EXTA-C: **STRUCTURALLY ACTIVE / fires** (most likely
  activation; convective-not-encodable);
- F-EXTA-A: partially (Stokes-sub-equivalence is relabeling);
- F-EXTA-B: vacuity real for full NS, fires for equivalence-with-
  full-NS;
- F-EXTA-D: ACTIVE for generic NS solutions; passes on
  shear-flow sub-manifold;
- F-EXTA-E: PARTIALLY ACTIVE (Otto framework transfer fails on
  incompressible NS).

**Statement**: variational gradient-flow framework, under any
flat metric g obtained by smooth change of variables on
Leray-projected L², **cannot encode the convective nonlinearity
of 3D NS** as a quadratic-functional gradient. The obstruction is
generic (EXT-A §3.6 paragraph "this obstruction is generic").

### §1.3 Pillar 3 -- EXT-B (Constantin-Iyer relative-entropy
       Lyapunov)

**Source**: `reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md`.

**Verdict**: **OBSTRUCTION_DOCUMENTED**. Two-fold (EXT-B §7.2):

- **Primary** (representation-side, F-EXTB-D fires): the
  Constantin-Iyer 2008 stochastic-Lagrangian representation
  produces a *family* of laws {ρ_t(· | x_0)}_{x_0 ∈ ℝ³} indexed
  by the back-to-labels initial label, **not a single ρ_t(x) on
  ℝ³** that the candidate functional W_NS requires. The collapse
  to a single ρ_t needs an unspecified choice (mixed-initial /
  pinned-label / velocity-mixed; EXT-B §6.4 / §8.1).
- **Secondary** (sketch-side, Path Q fails at the cross term):
  even granting any choice of ρ_t, the time-derivative
  computation (EXT-B §4.3 (b) and (e)) produces two uncontrolled
  cross terms:

  (b) the vortex-stretching residual ∫(ω·∇)u·ω · ρ dx — **this
      is the NS regularity obstruction itself**;
  (e) the Hess(log ρ):∇u cross term — sign-uncontrolled because
      the NS-driven CI flow is non-reversible and the augmented
      operator lacks a curvature-dimension condition CD(K, ∞)
      in the sense of Bakry-Émery 1985.

**Key mechanisms tested** (EXT-B §2):

- Constantin-Iyer 2008 (CPAM 61): stochastic-Lagrangian
  representation, smooth + short-time only;
- Otto 2001 (CPDE 26): gradient-flow structure for PME / heat
  on (P_2, W_2), explicitly absent for incompressible NS
  (catalogued out by Ambrosio-Gigli-Savaré 2008 §§9-11; Brenier
  1999 §5);
- Bakry-Émery 1985 (Sém. Probab. XIX): CD(K, ∞) for reversible
  diffusions only; NS-driven CI flow is non-reversible;
- Constantin-Vicol 2012 (GAFA 22): nonlinear maximum principle
  scalar-only;
- Otto-Villani 2000 (J. Funct. Anal. 173): HWI inequality
  requires CD(K, ∞), unavailable here.

**Statement**: analytic-Lyapunov / Bakry-Émery-Γ_2 framework,
applied to the augmented (u, ρ_t, τ) state space induced by
Constantin-Iyer 2008, **localizes the unresolved difficulty onto
two terms**: the vortex-stretching residual (NS regularity
obstruction itself) and the non-reversible cross term Hess(log
ρ):∇u. Pressure non-locality is *cleared* by the Brownian-flow
machinery (the Riesz-projector enters CI 2008 cleanly via the
back-to-labels gauge); the sign-uncontrolled obstruction lives on
vortex-stretching and on the density-velocity cross.

---

## §2 Cross-pillar obstruction map

The three NS regularity difficulties — pressure non-locality,
convective nonlinearity, vortex-stretching — are read by each
pillar as follows.

### §2.1 The map

| Difficulty | Pillar 1 (D3.A) | Pillar 2 (EXT-A) | Pillar 3 (EXT-B) |
|------------|-----------------|------------------|-------------------|
| Pressure non-locality (Riesz-transform of (u·∇)u) | **CLEARED** via Calderón-Zygmund 1952 H^s boundedness on 2D Riesz transforms (D3.A §4 Step 6) | TIED to F via Riesz/Leray projection in g definition; not the breaking step | **CLEARED** via CI 2008 back-to-labels gauge A absorbing the projection into the Brownian flow (EXT-B §2.1) |
| Convective nonlinearity (u·∇)u | partially handled in 2.5D — convective is 2D advection by u_h (D3.A §1 eqs (3)-(4)); **not the obstruction** in this restricted regime | **OBSTRUCTION** — Helmholtz condition fails per Olver 1986 (EXT-A §3.6, §5.3); not a variational gradient of any quadratic F under flat g | partially handled — convective enters via the drift in (2.1) and is absorbed into ∂_t ρ via Kolmogorov forward equation (EXT-B §4.2 eq 4.3) |
| Vortex-stretching (ω·∇)u | **OUTSIDE 2.5D scope by construction** — vanishes identically in v = e_3 × ∇⊥ψ + w·e_3 ansatz (D3.A §1) | **absorbed but not controlled** — F's enstrophy piece νλ‖ω‖² captures L² growth but the (ω·∇)u sign control is exactly the convective-encodability question (EXT-A §3.6) | **OBSTRUCTION** — cross term ∫(ω·∇)u·ω ρ in (4.2) is the NS regularity obstruction itself; Hess(log ρ):∇u in (4.5) is sign-uncontrolled by Bakry-Émery (EXT-B §4.3 (b) and (e)) |

### §2.2 What the pattern says

Reading the table column-by-column shows three things:

- **Pressure non-locality is universally clearable**. Two of the
  three pillars explicitly clear it (D3.A via 2D
  Calderón-Zygmund; EXT-B via Brownian-flow gauge); the third
  (EXT-A) ties it into F via the Leray projection but does not
  break on it. **No pillar locates the obstruction at pressure
  non-locality.**
- **Convective nonlinearity is variationally stuck**. Only one
  pillar (EXT-A) directly tests variational encodability, and it
  fails by Helmholtz-condition (Olver 1986). The other two
  pillars route around: D3.A by 2.5D ansatz (convective becomes
  2D advection), EXT-B by Kolmogorov forward (convective becomes
  drift in ∂_t ρ).
- **Vortex-stretching is analytically stuck**. Two of the three
  pillars hit a hard wall on (ω·∇)u or its derivatives:
  - D3.A sidesteps by ansatz (vortex-stretching vanishes);
  - EXT-A's enstrophy term is a quadratic surrogate, not a
    variational encoding of (ω·∇)u — the convective-encodability
    failure is *the same algebraic obstruction* as the
    vortex-stretching asymmetry ((v·∇)u + (u·∇)v non-symmetric);
  - EXT-B's Path Q breaks on ∫(ω·∇)u·ω ρ (vortex-stretching
    residual, sign-indefinite).

### §2.3 The convective vs vortex-stretching equivalence

A subtlety: EXT-A's "convective-not-encodable" obstruction and
EXT-B's "vortex-stretching residual" obstruction are **the same
algebraic object** seen through different frameworks.

In EXT-A (variational frame): the Fréchet derivative
D[(u·∇)u][v] = (v·∇)u + (u·∇)v fails symmetry under L² pairing,
*and the asymmetric piece is exactly the vortex-stretching term*
(EXT-A §5.3 explicit identification: "the asymmetry is exactly
the vortex-stretching term").

In EXT-B (analytic-Lyapunov frame): Path Q's failing cross term
is ∫(ω·∇)u·ω ρ, which is the vorticity production / stretching
that lacks sign control.

Reading these together: the 3-pillar map's "convective" and
"vortex-stretching" rows are **not independent obstructions** —
they are two presentations of the same Λ²-asymmetry of the
quadratic NS nonlinearity. The Sym² (symmetric) part is the
strain rate S; the Λ² (antisymmetric) part is the vorticity ω;
the cross-coupling (S, ω) drives both pillars 2 and 3.

This collapses the 3-difficulty / 3-pillar map onto a sharper
2-difficulty / 3-pillar map:

| Difficulty | Pillar 1 | Pillar 2 | Pillar 3 |
|------------|----------|----------|----------|
| Pressure non-locality (Sym² coupling via Riesz) | CLEARED | tied | CLEARED |
| Λ² antisymmetry (= convective-asymmetry = vortex-stretching) | OUTSIDE 2.5D | OBSTRUCTION (Helmholtz) | OBSTRUCTION (Path-Q cross term) |

In this collapsed map, the localization is unambiguous: **the
Λ² antisymmetry — equivalently axis B — carries the obstruction
across both attempted frameworks (variational and
analytic-Lyapunov)**, and the only pillar that "passes" it is the
one that ansatz-erases it (D3.A's 2.5D switches Λ² off by
construction).

---

## §3 Localization claim

> **The BT-544 obstruction localizes onto axis B
> (vortex-stretching with intermittent dissipation) — consistent
> with three independent attacks of distinct framework families.**

Restated as a per-pillar trio of observations:

- D3.A passes by switching axis B off by ansatz; pressure axis A
  is verified clean in isolation.
- EXT-A obstruction-documents at the algebraic identity
  ((v·∇)u + (u·∇)v) − ((u·∇)v + (v·∇)u) = (ω-asymmetry); the
  failing component is the antisymmetric-Λ² part of the
  convective term, i.e. axis B.
- EXT-B obstruction-documents at Path-Q cross term ∫(ω·∇)u·ω ρ,
  i.e. directly axis B.

**Each pillar uses a distinct framework family**:

| pillar | framework family | mathematical lineage |
|--------|------------------|----------------------|
| D3.A   | classical PDE estimates / Galerkin theory | Ladyzhenskaya 1959; Calderón-Zygmund 1952; BKM 1984; Constantin-Foias 1988 |
| EXT-A  | variational / Wasserstein gradient-flow | Otto 2001; Brenier 1999; Olver 1986; Ambrosio-Gigli-Savaré 2008 |
| EXT-B  | stochastic Lagrangian / analytic Lyapunov / Bakry-Émery Γ_2 | Constantin-Iyer 2008; Bakry-Émery 1985; Otto-Villani 2000; Bakry-Gentil-Ledoux 2014 |

These three families share no common machinery beyond
divergence-free hypothesis; the convergent pointing-to axis B is
**a non-trivial structural agreement**.

**This is what the D3 seed §4.2 already predicted**:

> "Axis B is the obstruction-carrier ... discriminator B is the
> same difficulty as the original Clay problem restricted to
> axisymmetric Euler. ... The historical record of NS research is
> essentially the record of attempts on axis B (BKM 1984,
> Constantin-Fefferman 1993 vorticity-direction-coherence, Hou-Luo
> 2014, Elgindi 2021)."

The 3-pillar pattern **confirms the D3-seed prediction**, with
the additional observation that two distinct *external*
framework attempts (variational EXT-A, analytic-Lyapunov EXT-B)
both bottom out at axis B. The D3 seed only argued from
classical-PDE-difficulty; the present synthesis adds two
non-classical witnesses.

---

## §4 Caveats — what this synthesis does NOT establish

### §4.1 Partial verdicts only

Each pillar reaches a *partial* verdict:

- D3.A is PASS_LITERATURE on axis A in isolation, **not** on full
  3D NS. The compositional argument (D3-seed §3.2) requires *two*
  clean axes to localize the obstruction onto the third; only one
  is fully cleared (D3.A §7 "F-544-B-survival path now
  half-supported").
- EXT-A is OBSTRUCTION_DOCUMENTED at the
  framework-encodability level — meaning the *framework* does
  not encode the convective term, **not** that the convective
  term is non-encodable in any conceivable framework.
- EXT-B is OBSTRUCTION_DOCUMENTED at the
  representation-side (F-EXTB-D, ρ_t ambiguity) and at the
  Path-Q cross-term. The cross-term obstruction is "the NS
  regularity obstruction itself" (EXT-B §8.3 explicit), so
  EXT-B's localization is essentially restating the open
  problem.

The 3-pillar localization is therefore **"axis B carries the
obstruction across these three framework families"**, not "axis
B is the obstruction proof-block of NS regularity in absolute
terms".

### §4.2 Consistency, not proof

The localization is *consistent with* each pillar; it is not
*proved by* any pillar nor by the trio together. A synthesis from
three partial verdicts cannot exceed the strength of the
strongest partial verdict, and no partial verdict here proves
that axis B is the *unique* obstruction-carrier — only that, in
the framework families tested so far, axis B is the consistent
breaking point.

### §4.3 Non-novelty of the located obstruction

The vortex-stretching obstruction is **not new content**:

- BKM 1984 already states ‖ω‖_{L^∞} as the regularity-controlling
  norm;
- Constantin-Fefferman 1993 already analyses vortex-direction
  geometric depletion as the governing mechanism;
- Hou-Luo 2014 already provides numerical evidence for
  axisymmetric Euler blow-up driven by vortex-stretching at
  hyperbolic stagnation points;
- Elgindi 2021 already proves rigorous C^{1,α} blow-up for 3D
  Euler in a related axisymmetric setting.

The 3-pillar synthesis is a *new bookkeeping*: it ties existing
vortex-stretching obstructions to **specific failure modes of
new framework attempts** (variational, analytic-Lyapunov). The
synthesis does not relocate the obstruction — it confirms that
modern framework attempts re-encounter the same classical
obstruction.

### §4.4 The "axis B" label is structural, not exhaustive

D3-seed §4 ranks axes (A, B, C) by difficulty estimate, not by
exhaustive coverage. Axis C (Onsager/Kraichnan) was not part of
this 3-pillar set — it has its own discriminator
(`omega-exec-bt544-d3-C-axis-discriminator-2026-04-25.md`). A
genuinely 3-axis localization would also fold C in. The current
synthesis covers two axes (A cleared by D3.A, B obstructed by
EXT-A and EXT-B) and is silent on C in this trio. C's status is
in the D3.C report and is not claimed here.

---

## §5 What it would take to break the axis-B obstruction

If axis B is the bottleneck, the path to unblock it is **not** a
new framework choice (variational and analytic-Lyapunov have both
failed at axis B, per pillars 2 and 3). The path is to engage the
axis B literature directly:

### §5.1 Hou-Luo 2014 / Chen-Hou 2022 numerical evidence

Hou-Luo 2014 (*PNAS* 111, 12968-12973; subsequently expanded in
the Hou group's preprints) provides high-resolution numerical
simulations of axisymmetric 3D Euler with swirl on a domain with
solid boundary, exhibiting self-similar blow-up at a hyperbolic
stagnation point on the boundary axis. Chen-Hou 2022 (and
follow-ups) extend the analysis to a broader class of
self-similar profiles and provide computer-assisted proofs for
related approximate models.

**Why this matters for the 3-pillar synthesis**: Hou-Luo's setup
is precisely the "axis B in isolation" regime that D3-seed §2.2
identified (axisymmetric-with-swirl Euler). The numerical
evidence suggests axis B may be **structurally blow-up-supporting
at the Euler level**.

### §5.2 Elgindi 2021 rigorous Euler blow-up

Elgindi 2021 (*Annals of Math.* 194, 647-727) proves rigorous
finite-time singularity for 3D incompressible Euler with
axisymmetric C^{1,α} initial data (low-regularity threshold,
α small). Subsequent work by Elgindi-Ghoul-Masmoudi 2021 and
related authors extends to slightly different reduced systems.

**Why this matters**: the Elgindi blow-up is *rigorous*, not
numerical; it confirms that vortex-stretching can produce
finite-time singularity in inviscid 3D incompressible flow at a
specific (low) regularity tier. The Beale-Kato-Majda 1984
criterion is therefore *active* — axis B can fire.

### §5.3 Constantin-Fefferman 1993 vorticity-direction regularity

Constantin-Fefferman 1993 (*Indiana Univ. Math. J.* 42, 775-789)
provides a regularity criterion based on the Lipschitz continuity
of the vorticity direction ω/|ω| in regions where |ω| is large.
This is a **viscosity-positive** result (it applies to NS, not
just Euler) and is the closest analytic tool to "axis B
regularity by geometric depletion".

**Why this matters**: it is the only widely-cited regularity
result that *targets axis B directly* and survives in the
viscous case. Any progress on axis B for full NS likely engages
this lineage.

### §5.4 Synthesis of §5.1-§5.3

The combination Hou-Luo numerics + Elgindi rigorous +
Constantin-Fefferman geometric is the **state of axis B in the
literature**. None of the three pillars in this synthesis (D3.A,
EXT-A, EXT-B) engages this literature directly — D3.A
sidesteps, EXT-A and EXT-B obstruction-document. To make
*progress* on axis B for BT-544 requires a session that engages
the Hou-Luo / Elgindi / Constantin-Fefferman triple as the
target lineage, not the present three pillars.

---

## §6 Honest reframe — NS regularity ≈ blow-up-prevention

The 3-pillar localization sharpens the BT-544 question from
"prove global C^∞ regularity" to a more precise form:

> **Does viscosity ν > 0 prevent the axis-B blow-up that Hou-Luo
> 2014 numerics + Elgindi 2021 rigorous Euler blow-up indicate
> would occur in the inviscid limit?**

This reframing is not original — it is implicit in any modern
discussion of NS regularity that takes Hou-Luo and Elgindi
seriously. What the 3-pillar synthesis adds is the **negative
information** that two distinct external frameworks (variational
gradient-flow, analytic-Lyapunov) **do not bypass** the
viscous-vs-blow-up question — they re-encounter it on axis B.

Three structurally distinct outcomes are conceivable:

- (R+) Viscosity *does* prevent the blow-up (NS regular). Then
  the axis-B obstruction is real but ν-bounded, and the proof
  technology must explicitly use ν > 0 in a way that exceeds
  current viscous-regularization machinery.
- (R−) Viscosity does *not* prevent the blow-up (NS blows up at
  finite time for some smooth initial data). Then BT-544 is
  resolved with the *opposite* answer to the Clay statement.
- (R?) Viscosity is *insufficient* to settle the question with
  current technology (axis-B-with-viscosity is genuinely open
  and may remain so). Then BT-544 is in the same status as
  axisymmetric-Euler-with-swirl: a state-of-the-art open problem.

The 3-pillar synthesis, **on its own**, does not distinguish
(R+), (R−), (R?). It only states that axis B is where the
distinction must be made. The three pillars are **diagnostic**,
not prognostic.

---

## §7 Implications for BT-544 main line + R1/R5 auxiliaries

### §7.1 Main line (Clay statement, global C^∞ regularity)

The 3-pillar synthesis says: **proving the Clay form of BT-544
requires resolving axis B**. Pressure non-locality (axis A) is
known clean in isolation; convective-as-Λ² (axis B antisymmetric
component) is the open structural piece.

This is consistent with the historical record of NS research and
with the D3-seed prediction. It does *not* provide new tools; it
sharpens the *target*.

### §7.2 R1 auxiliary (α-Hölder threshold, Onsager direction)

R1's question is whether the Hölder threshold α_c = 1/3 (Onsager
1949) applies on a strict gap or is sharp at exactly 1/3 (per
the BCV 2021 / BMNV 2023 convex-integration line). This lives
on **axis C** (Onsager / cascade) of the D3 decomposition, not
on axis B.

**Implication**: even if R1 is fully resolved (in either
direction — strict gap or sharp 1/3), it does not address the
axis-B obstruction localized by the 3-pillar synthesis. R1
progress is informative for the Onsager direction of NS theory
but is **decoupled from the BT-544 main-line obstruction**.

### §7.3 R5 auxiliary (ε-Mach low-Mach limit)

R5's question is the low-Mach-number limit of compressible NS
returning to incompressible NS. This is a **boundary-layer /
asymptotic-expansion** question, not a vorticity-stretching
question. It also does not engage axis B.

**Implication**: R5 progress is informative for the
incompressible-vs-compressible interface but is **decoupled from
the BT-544 main-line obstruction**.

### §7.4 Conclusion on BT-544 sub-structure

The auxiliary problems R1 / R5 do not reach axis B. The
auxiliary problems Q1 / Q5 (kinetic / Onsager-based) similarly
do not. The full L9 catalogue (per
`omega-exec-bt544-fallback-molt-validation-2026-04-25.md`) was
catalogue-exhausted *without* engaging axis B directly.

The 3-pillar synthesis re-affirms what was already implicit:
**no auxiliary in the current BT-544 ledger targets axis B
directly**. Future auxiliaries that aim at the BT-544 main line
should target axis B with frameworks designed for it — most
plausibly the Hou-Luo / Elgindi / Constantin-Fefferman lineage
(per §5).

---

## §8 Future attack recommendations (axis-B-targeted)

Without prescribing execution, the 3-pillar synthesis surfaces
the following candidate directions for future BT-544 sessions:

### §8.1 Direction-1 — Hou-Luo numerical-evidence-driven
        analytical reduction

Engage the Hou-Luo 2014 self-similar profile as a *target*
solution candidate. Question: does the Hou-Luo profile, when
modified to include viscosity ν > 0 with appropriate scaling,
yield a regular self-similar dissipative solution (R+) or a
finite-time blow-up (R−)? This is the standard "viscous
self-similar profile" question, currently open.

**Pillar contact**: this direction does *not* engage pillars 1
or 2 directly; it is a targeted axis-B attempt. It bypasses the
variational and analytic-Lyapunov framework attempts that the
present synthesis showed bottom out at axis B.

### §8.2 Direction-2 — Constantin-Fefferman geometric depletion
        at the Λ² level

Engage Constantin-Fefferman 1993's vorticity-direction
regularity criterion as the controlling norm. Question: under
what conditions on ω/|ω|'s Lipschitz continuity does the 3D NS
solution remain regular? This is a known direction with
intermediate progress; the 3-pillar synthesis suggests it is the
*viscous-side* analog of Hou-Luo / Elgindi.

**Pillar contact**: pillar 3's failure mode (Hess(log ρ):∇u
cross term) suggests that some form of ω-coherence hypothesis is
the missing input. Constantin-Fefferman 1993 is the concrete
implementation of such a hypothesis.

### §8.3 Direction-3 — Elgindi-style rigorous lower-regularity
        blow-up adjacency for NS

Engage Elgindi 2021's C^{1,α} Euler blow-up and ask whether
adding viscosity ν > 0 to the Elgindi setup changes the
blow-up time or eliminates it. This is a "viscous extension of
Elgindi" question; some variants are known, full NS open.

**Pillar contact**: this is purely an axis-B direction; no
contact with pillars 1, 2, 3.

### §8.4 Note on direction novelty

None of §8.1-§8.3 is a new direction in the literature. The
3-pillar synthesis does *not* generate new attack ideas; it
generates **negative information** about which attack ideas are
*not promising* (variational, analytic-Lyapunov), and points the
remaining attention budget toward the existing axis-B lineage.

---

## §9 Anti-list — alternative localizations considered

Localizations considered and rejected in this synthesis:

### §9.1 "Pressure non-locality is the obstruction"

The Bardos-Titi-style heuristic that pressure non-locality is
the structural source of NS difficulty. **Rejected**: D3.A
clears pressure non-locality on the 2.5D ansatz with classical
Calderón-Zygmund tools, and EXT-B's CI-2008 Brownian flow
absorbs the projection cleanly. No pillar locates the
obstruction here.

### §9.2 "Convective nonlinearity per se"

A reading of EXT-A's F-EXTA-C as "the convective term is the
obstruction". **Rejected as imprecise**: EXT-A's failing
algebraic identity is the antisymmetric component of the
convective derivative, which is exactly vortex-stretching. The
*symmetric* part of (u·∇)u (the strain-rate part) is encoded
fine. Locating the obstruction at "convective nonlinearity"
without the Sym²/Λ² decomposition over-attributes.

### §9.3 "Energy cascade / Onsager exponent" (axis C)

A localization onto axis C. **Not addressed by this 3-pillar
trio** — only D3.C engages axis C, and D3.C is outside the
present synthesis. Per D3-seed §4 ranking, axis C is "low-to-
moderate difficulty" and likely fires F-D3-C (relabeling); it is
*not* the obstruction-carrier in the D3-seed prediction. The
3-pillar synthesis is consistent with this in that no pillar
points at axis C, but the synthesis cannot positively rule axis C
out.

### §9.4 "All three axes equally obstructed"

A reading that no localization is possible because all three NS
difficulties are equally entangled. **Rejected**: the 3-pillar
map (§2.1) is asymmetric — pressure non-locality is cleared
by two pillars, vortex-stretching is obstructed by two pillars.
If all three were equally entangled, no asymmetry would be
visible.

### §9.5 "Framework choice itself is the issue"

A reading that variational / analytic-Lyapunov frameworks are
the wrong frameworks for NS, and a framework switch (e.g.
mode-mode interaction analysis, dynamical-system attractor
theory) would resolve the issue. **Partially valid as a critique
of EXT-A and EXT-B individually**, but this critique applies
*because* both frameworks fail at axis B. The localization
survives the critique: the framework choice is *exactly what is
being diagnosed* by the 3-pillar synthesis (pillars 2 and 3 fail
*at axis B*, regardless of how the failure is framed).

---

## §10 Falsifiers active for the 3-pillar synthesis

Falsifiers under which the §3 localization claim would be
retracted or downgraded.

### §10.1 F-SYN-A — pillar verdicts misread

**Statement**: if D3.A's PASS_LITERATURE / EXT-A's
OBSTRUCTION_DOCUMENTED-on-inspection / EXT-B's
OBSTRUCTION_DOCUMENTED verdicts are misread by this synthesis
(e.g. EXT-A's Path-R activation is weaker than synthesis
claims, or EXT-B's Path-Q failure is more easily fixable), the
localization weakens.

**Status**: NOT ACTIVE based on direct reading of source
reports' §6 (D3.A), §3.6+§5.3+§6.1 (EXT-A), §7+§8 (EXT-B).
Each source report's verdict is explicitly stated in its
closing section.

### §10.2 F-SYN-B — convective-stretching equivalence overstated

**Statement**: §2.3's claim that EXT-A's convective-not-
encodable failure and EXT-B's vortex-stretching residual are
"the same algebraic obstruction" overstates the connection. They
might be independent obstructions, with the 3-pillar pattern
becoming weaker (each pillar fails at a *different* place,
reducing the localization strength).

**Status**: PARTIALLY ACTIVE. The Fréchet-derivative asymmetry
in EXT-A §5.3 ((v·∇)u + (u·∇)v) and the cross term in EXT-B
§4.3 (∫(ω·∇)u·ω ρ) are algebraically related (both involve the
vorticity-coupling structure of the convective nonlinearity)
but are *not* literally identical objects — one is on tangent
vectors v, the other is on ρ-weighted integrals. A careful
algebraic proof of literal equivalence is *not* provided in this
synthesis. The §2.3 paragraph reads them as "two presentations
of the same Λ²-asymmetry", which is a structural reading, not a
proof. A specialist re-reading could downgrade §2.3 from
"identical" to "structurally cognate".

### §10.3 F-SYN-C — fourth-pillar verdict contradicts

**Statement**: if a fourth independent attack (e.g. D3.B'
discriminator on axisymmetric-with-swirl Euler, or EXT-C
procedure-class candidate, or EXT-D vocabulary extension)
reaches an OBSTRUCTION_DOCUMENTED verdict at a *different* axis
(e.g. axis C or pressure non-locality), the localization claim
is contradicted.

**Status**: NOT ACTIVE as far as the present synthesis covers.
D3.B' (axisymmetric-with-swirl) and D3.C (Kraichnan/Onsager)
discriminator reports exist on disk
(`omega-exec-bt544-d3-Bprime-axis-discriminator-2026-04-25.md`,
`omega-exec-bt544-d3-C-axis-discriminator-2026-04-25.md`) and
are not folded in here. Their verdicts could either reinforce
the axis-B localization (D3.B' fails on axis B itself) or
introduce new information. Cross-check would be a follow-up
synthesis task.

### §10.4 F-SYN-D — axis-B literature already breaks the
        obstruction

**Statement**: if Hou-Luo 2014 / Elgindi 2021 / Chen-Hou 2022 or
their follow-ups have *already* resolved axis-B-with-viscosity
in either direction (R+ regular or R− blow-up) and the
synthesis missed that paper, the BT-544 main-line obstruction
status is different from what is stated here.

**Status**: NOT ACTIVE based on standard awareness of the field
through 2022 (the latest dates the candidate reports cite). The
field has not closed axis B with viscosity as of the cited
dates. Risk of missed post-2022 paper is real but low; a
literature recheck would be appropriate before any forward
session.

### §10.5 F-SYN-E — atlas/state/inventory edit leakage

**Statement**: if any change is made to atlas, state, or
inventory files as a result of this synthesis, the brief's hard
constraint is violated.

**Status**: NOT ACTIVE. This synthesis is research-only and
edits no atlas, state, or inventory file. The git status at
session start (specs and inventory.json modified by *unrelated*
prior sessions per the gitStatus header) is unaffected.

### §10.6 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-SYN-A | pillar verdicts misread | NOT ACTIVE |
| F-SYN-B | convective-stretching equivalence overstated | PARTIALLY ACTIVE |
| F-SYN-C | fourth-pillar verdict contradicts | NOT ACTIVE (not yet folded in) |
| F-SYN-D | axis-B literature already breaks obstruction | NOT ACTIVE |
| F-SYN-E | atlas/state/inventory edit leakage | NOT ACTIVE |

One falsifier (F-SYN-B) is partially active and would benefit
from a specialist algebraic check; this is flagged as the
structural risk to the §2.3 equivalence reading.

---

## §11 Closing

**Localization claim**: BT-544 obstruction localizes onto axis B
(vortex-stretching / Λ² antisymmetry of convective nonlinearity)
— consistent with three independent attacks (D3.A
PASS_LITERATURE on axis A in isolation; EXT-A
OBSTRUCTION_DOCUMENTED at F-EXTA-C convective-not-encodable;
EXT-B OBSTRUCTION_DOCUMENTED at F-EXTB-D + Path-Q
vortex-stretching cross term).

**Caveat**: consistency, not proof. Each pillar reaches a
partial verdict; the localization is a cross-pillar
*bookkeeping* that ties existing vortex-stretching obstructions
(BKM 1984; Constantin-Fefferman 1993; Hou-Luo 2014;
Elgindi 2021) to specific failure modes of new framework
attempts.

**Implications**:

- BT-544 main line (Clay form) reduces, structurally, to the
  axis-B-with-viscosity question;
- R1 / R5 auxiliaries do not reach axis B and are decoupled
  from the main-line obstruction;
- Future axis-B-targeted attacks should engage the
  Hou-Luo / Elgindi / Constantin-Fefferman lineage directly,
  not the variational or analytic-Lyapunov framework families
  that the 3-pillar synthesis showed bottom out at axis B.

**Honest reframe**: NS regularity ≈ "does viscosity prevent the
axis-B blow-up that Hou-Luo numerics + Elgindi rigorous Euler
results indicate would occur in the inviscid limit?" This
reframe is already implicit in the modern NS literature; the
3-pillar synthesis confirms it.

**Falsifiers active**: F-SYN-B partially (convective vs
vortex-stretching equivalence reading is structural, not
algebraically proved); others not active.

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.**

All cited references are pre-existing (Beale-Kato-Majda 1984
*Comm. Math. Phys.* 94; Calderón-Zygmund 1952 *Acta Math.* 88;
Constantin-Fefferman 1993 *Indiana Univ. Math. J.* 42;
Constantin-Foias 1988 U. Chicago; Constantin-Iyer 2008 *CPAM*
61; Constantin-Majda-Tabak 1994 *Nonlinearity* 7;
Constantin-Vicol 2012 *GAFA* 22; Chen-Hou 2022 (Hou-group
follow-up); Elgindi 2021 *Annals of Math.* 194; Friedman 1964;
Hou-Luo 2014 *PNAS* 111; Ladyzhenskaya 1959/1969;
Ladyzhenskaya-Solonnikov-Ural'ceva 1968; Olver 1986
*Applications of Lie Groups to Differential Equations*,
Springer; Otto 2001 *CPDE* 26; Otto-Villani 2000 *J. Funct.
Anal.* 173; Bakry-Émery 1985 *Sém. Probab.* XIX;
Bakry-Gentil-Ledoux 2014 Springer; Brenier 1999 *J. Amer.
Math. Soc.* 12; Ambrosio-Gigli-Savaré 2008 Birkhäuser;
Onsager 1949 *Nuovo Cimento* 6 Suppl.; Stroock-Varadhan 1979
Springer).

— end synthesis —
