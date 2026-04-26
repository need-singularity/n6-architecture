---
id: omega-exec-bt544-r5-low-mach-dispatch
date: 2026-04-25
scope: research-direction-design (NOT proving R5-Aux; mapping known results)
target: BT-544 R5 Low-Mach auxiliary problem -- precise definition + known-result map + smallest open piece
parent_reports:
  - reports/sessions/omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md (R5 ACCEPTABLE)
  - reports/sessions/omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md (R1 template)
  - reports/sessions/omega-seed-bt544-d2-Lomega-axiom-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: auxiliary-problem dispatch, no claim
---

# Omega Exec — BT-544 R5 Low-Mach Dispatch (2026-04-25)

## §0 Non-claim disclaimer

This report performs **research-direction-design** for the auxiliary
problem R5-Aux registered in
`omega-seed-bt544-d2-Lomega-axiom-2026-04-25.md` §5.2 and graded
ACCEPTABLE in
`omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md` §3.4. It:

- does **NOT** prove R5-Aux in either direction (no claim about
  uniform-in-ε regularity rate);
- does **NOT** prove or refute Clay BT-544 NS smoothness; the Clay
  Millennium NS problem statement (Fefferman 2000) is unchanged;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- treats R5-Aux as a research-design artifact: a precise restatement
  + a known-result map + a smallest-open-piece selection;
- registers a **conjecture** (not a proof claim) for the
  ε → 0 regularity-rate behaviour, marked clearly as conjectural,
  with proposed evidence steps;
- preserves the F-D2-A (lineage-fabrication) inactive status: every
  cited result is by author/year/title only, no fabricated quotes.

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
The dispatch is at the L_ω apex (axiom-recast) per the parent seed,
and is **auxiliary** to BT-544; resolving R5-Aux does not resolve
BT-544.

This dispatch is the **parallel** of the R1 dispatch
(`omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md`):
where R1 recasts axiom A2 (regularity class) along the
Onsager-Hölder lineage, R5 recasts axiom A6 (incompressibility)
along the Klainerman–Majda → Schochet → Lions–Masmoudi →
Feireisl–Novotný lineage. Structural format follows R1 for
parallelism.

---

## §1 R5-Aux precise definition

### §1.1 Working statement

**R5-Aux (NS-low-Mach problem).** Let T³ = (ℝ/ℤ)³ be the periodic
3-torus. For ε ∈ (0, 1] (Mach number), consider the (isentropic)
compressible Navier–Stokes system

    ∂_t ρ_ε + ∇·(ρ_ε u_ε) = 0,
    ∂_t (ρ_ε u_ε) + ∇·(ρ_ε u_ε ⊗ u_ε) + (1/ε²) ∇ p(ρ_ε) = ν Δu_ε + (ν + λ) ∇(∇·u_ε),
    ρ_ε(0) = 1 + ε ρ_ε'(0),    u_ε(0) = u_ε^0,

with viscosities ν > 0, λ ≥ 0 fixed and zero forcing, and pressure
law p(ρ) = ρ^γ for γ > 1 (the precise γ-exponent does not affect
the dispatch's main statement). Initial data are well-prepared in
the standard sense (∇·u_ε^0 = O(ε) in suitable norm), so that
acoustic-mode oscillations of frequency 1/ε are bounded in
L²-energy uniformly in ε.

In this scaling the divergence constraint is **relaxed** to
∇·u_ε = O(ε) uniform-in-ε rather than ∇·u = 0 exactly, and the
ε → 0 limit (formally) recovers the incompressible NS system on
T³.

Define the **uniform-in-ε regularity-rate function**

    α(s) := sup { β ≥ 0 : ‖u_ε - u_0‖_{L^∞_t H^s([0,T] × T³)} ≤ C(s, T, data) · ε^β,
                   uniform in ε ∈ (0, ε_0] for some ε_0 > 0 }

where u_0 is the (assumed-existing on [0, T]) incompressible-NS
solution with the corresponding well-prepared limit initial data,
and s ≥ 0 indexes the Sobolev scale.

The **R5-Aux problem** is to *determine the function s ↦ α(s) on
its existence interval [0, T]*, in particular:
- (E1) at which Sobolev index s does α(s) become positive
  (uniform-in-ε convergence, with rate)?
- (E2) does α(s) remain bounded away from 0 as s increases, or
  does it degrade as s → ∞?
- (E3) at what s = s* (if any) does α(s) drop to 0, signalling
  loss of uniform-in-ε regularity at higher Sobolev orders?

### §1.2 Variants and parameters

The dispatch fixes the following choices, distinguishing them from
related literature:

| parameter | this dispatch | rationale |
|-----------|---------------|-----------|
| domain | T³ | matches Schochet 1986 periodic setting; avoids decay-at-infinity bookkeeping that ℝ³ introduces; the parent seed §2.5 also targets T³. ℝ³ variant noted in §6 caveat. |
| viscosity | ν > 0, λ ≥ 0 fixed | this is the NS (not Euler) regime; ν = 0 (compressible Euler low-Mach) is the Klainerman–Majda 1982 setting, distinct (cf. §2). |
| initial-data preparation | well-prepared (∇·u_ε^0 = O(ε)) | standard reduction eliminating the acoustic-resonance preparation issue (Schochet 1986); ill-prepared variant noted in §6 caveat. |
| pressure law | p(ρ) = ρ^γ, γ > 1 (isentropic) | matches Lions 1996 / Feireisl–Novotný 2009 isentropic framework; full NS (with temperature) is a separate problem (§6 caveat). |
| time interval | [0, T] fixed (depending only on data norm and ν) | not the full global-in-time question; matches Klainerman–Majda 1982 local skeleton plus Lions–Masmoudi 1998 weak-global structure; global strong uniform-in-ε is conjecturally Clay-equivalent and is **not** the dispatch target. |
| solution class | classical strong solutions for ε > 0 (compressible NS) on [0, T]; limit u_0 in the corresponding incompressible strong class | matches the regularity-rate question; weak-solution variant (Lions–Masmoudi 1998 layer) noted in §6 caveat. |

### §1.3 What R5-Aux is *not*

- R5-Aux is **not** the Clay NS smoothness problem (which fixes
  ε = 0 and asks for global existence on [0, ∞));
- R5-Aux is **not** the Klainerman–Majda 1982 local low-Mach
  existence theorem (which establishes uniform-in-ε *existence* on
  a time interval, not the **regularity rate** as a function of
  Sobolev index s);
- R5-Aux is **not** the Lions–Masmoudi 1998 global weak
  incompressible limit (which passes to the limit at the level of
  weak solutions, not strong-regularity uniform-in-ε bounds);
- R5-Aux is **not** the Feireisl–Novotný 2009 monograph framework
  (which gives uniform energy estimates in unbounded domains, not
  the rate function α(s));
- R5-Aux is **not** the global uniform-in-ε strong-regularity
  question (which is conjecturally equivalent to Clay NS and is
  rejected by the parent seed §6 anti-list as
  HARDER_THAN_AUXILIARY).

R5-Aux is the determination of the *rate function* α(s) on a fixed
interval [0, T], characterizing how compressible-NS solutions
approach incompressible-NS solutions in higher-Sobolev norms as
the Mach number ε → 0.

---

## §2 Distinction from existing results

This section traces how each lineage citation in
`omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md` §5 (R5
side) maps onto the R5-Aux problem.

### §2.1 Klainerman–Majda 1982 ("Compressible and Incompressible Fluids", Comm. Pure Appl. Math.)

**Original content (cited by author/year/title only)**:
For the **compressible Euler** system (ν = 0) with well-prepared
initial data, Klainerman and Majda established **uniform-in-ε
local-in-time existence** of smooth solutions, and proved
convergence to the incompressible Euler limit. The time interval
is *uniform in ε* but *finite*; the regularity is high (smooth)
but the *rate of convergence* is not the focus of the 1982
statement (the result is a uniform existence + qualitative limit
result).

**Mapping onto R5-Aux**:
- This is an **Euler** statement (ν = 0); R5-Aux is a **NS**
  statement (ν > 0 fixed).
- The 1982 result establishes the **existence skeleton** on which
  R5-Aux can build: uniform-in-ε existence on [0, T] is taken as a
  *given* (with viscosity, the analogous skeleton holds; cf.
  Hagstrom–Lorenz 2002, Métivier–Schochet 2001, by author/year
  only).
- The 1982 result does **not** quantify α(s); it shows a uniform
  existence interval but the **rate** of convergence in Sobolev
  norms is not its theorem.
- Implication for R5-Aux: Klainerman–Majda 1982 is a *prerequisite*
  (uniform existence on [0, T]) but not an *answer* (rate
  function α(s)).

### §2.2 Schochet 1986 ("The Compressible Euler Equations in a Bounded Domain: Existence of Solutions and the Incompressible Limit", Comm. Math. Phys.)

**Original content (cited by author/year/title only)**:
Refinement of the Klainerman–Majda framework on a bounded domain
(and a closely-related periodic-domain refinement in the
J. Differential Equations strand of Schochet's work). The
contribution is a **fast-acoustic-mode averaging** technique that
removes the requirement of preparing initial data exactly: an
**ill-prepared** initial datum can be averaged in the fast
acoustic time-scale to yield convergence of the slow (vortical)
mode to the incompressible limit.

**Mapping onto R5-Aux**:
- Schochet's averaging gives the *mechanism* by which fast acoustic
  oscillations (frequency 1/ε) decouple from the slow incompressible
  dynamics — this is exactly the mechanism that the rate function
  α(s) measures.
- For *well-prepared* data (the dispatch's choice in §1.2),
  Schochet's averaging is bypassed; for *ill-prepared* data
  (§6 caveat), it is essential.
- Implication for R5-Aux: Schochet 1986 supplies the **averaging
  template** for any rate calculation; the averaged-mode rate is
  expected to be the dominant contribution to α(s).

### §2.3 Lions 1996 (*Mathematical Topics in Fluid Mechanics, Vol. 2*)

**Original content (cited by author/year/title only)**:
Monograph foundation of the global weak-solution theory for
compressible NS, including the construction of finite-energy weak
solutions on T³ and ℝ³, and the framework for passing to singular
limits at the weak level.

**Mapping onto R5-Aux**:
- This is **monograph background** rather than a single theorem.
- The weak-solution layer it establishes is **not** the
  strong-regularity layer of R5-Aux; R5-Aux works above the weak
  layer (strong solutions on [0, T]).
- Implication for R5-Aux: Lions 1996 ensures the compressible-NS
  side is well-defined on T³; R5-Aux's strong-regularity question
  is then well-posed within the Lions framework.

### §2.4 Lions–Masmoudi 1998 ("Incompressible Limit for a Viscous Compressible Fluid", J. Math. Pures Appl. — venue ambiguity flagged in parent §5)

**Original content (cited by author/year/title only)**:
Global incompressible limit for the **weak** solutions of
compressible NS: as ε → 0, finite-energy weak solutions of
compressible NS converge (in suitable senses, including weak
convergence and convergence-up-to-acoustic-modes) to a Leray–Hopf
weak solution of incompressible NS.

**Mapping onto R5-Aux**:
- This is a **weak-level** convergence result, not a
  **strong-regularity rate** result.
- The convergence is qualitative (passing to the limit in the
  equations) rather than quantitative (a rate ε^β).
- Some norms do converge with a quantifiable rate in subsequent
  work (Desjardins–Grenier 1999, Danchin 2002, Masmoudi 2007 —
  cited generally by author/year only); these subsequent results
  give *partial* α(s) values for *low* Sobolev indices s, but a
  **complete s ↦ α(s) characterization** is not in the published
  record to this dispatch's knowledge.
- Implication for R5-Aux: Lions–Masmoudi 1998 establishes that
  α(s) > 0 for *some* (low) s in the weak sense; the
  strong-regularity uniform rate at *higher* s, and the
  s-dependence of α, is the open content of R5-Aux.

### §2.5 Feireisl–Novotný 2009 (*Singular Limits in Thermodynamics of Viscous Fluids*, Birkhäuser monograph)

**Original content (cited by author/year/title only)**:
Comprehensive monograph treatment of singular limits for
compressible viscous (and heat-conducting) fluids in unbounded and
exterior domains. Provides uniform-in-ε **energy** estimates and
density-fluctuation estimates, and a unified framework for
low-Mach, low-stratification, and related limits.

**Mapping onto R5-Aux**:
- The uniform energy estimates ε^{-2} ‖p(ρ_ε) - p(1)‖ ≲ C and
  related density-fluctuation bounds are *exactly* the uniform-in-ε
  starting estimates that R5-Aux's rate function α(s) must improve
  upon.
- Feireisl–Novotný's framework is on **unbounded/exterior**
  domains; the T³ specialization (the dispatch's choice) inherits
  the energy estimates more directly because acoustic modes are
  countable rather than continuous.
- Implication for R5-Aux: Feireisl–Novotný 2009 gives α(s = 0)
  bounds (energy-level uniform rates, typically α(0) = 1 or 2
  depending on quantity); higher-s rates are *not* the monograph's
  central theorem and are the R5-Aux open content.

### §2.6 Why α(s) is conjecturally s-dependent

The compressible-incompressible singular limit has *two* distinct
dynamical scales:
- the **slow vortical scale** (incompressible flow, time-scale 1);
- the **fast acoustic scale** (compressible pressure waves, time-
  scale ε).

The fast acoustic scale enters every Sobolev norm with a scaling
factor 1/ε^k for some k(s) depending on how many derivatives are
applied. Heuristically:

- Pro: the well-prepared scaling localizes acoustic energy, so
  α(s) > 0 at all s in principle, with the **rate** ε^β
  controlling how fast the acoustic component decays in the
  Sobolev-s norm.
- Con: each spatial derivative applied to an acoustic-mode
  pressure wave produces a factor 1/ε from the spatial frequency
  (acoustic waves at frequency 1/ε), so higher Sobolev indices
  *amplify* the acoustic-mode contribution, suggesting α(s) is
  **decreasing in s**.

Both heuristics are present in the literature; the **decay rate
of α(s) as s grows** is the open content of R5-Aux. Conjecturally
(Klainerman–Majda → Schochet → Lions–Masmoudi → Feireisl–Novotný
line, per parent acceptability §3.1), α(s) lies in (0, 1] for
s = 0 (Feireisl–Novotný energy level), is conjectured positive for
s ≤ s* with some s-dependent decay, and is conjecturally **0** at
some critical s* where the acoustic-mode amplification exactly
cancels the well-prepared decay. Whether s* is finite (a true
*compressibility-singularity barrier*) or infinite (uniform
regularity at all Sobolev orders) is the sharpest open piece.

---

## §3 Known-result ε-map table

The table below partitions the (ε, s) plane into regimes and
records what is known per regime. **UNKNOWN** entries are explicit.

### §3.1 Master table

| ε regime | Existence (compressible NS_ε) | Uniform regularity rate α(s) | Singular limit (ε → 0) | Reference / status |
|----------|------------------------------|-------------------------------|------------------------|---------------------|
| ε = 0 (incompressible NS) | local: yes (classical); global strong: **UNKNOWN** = Clay BT-544 | not applicable (ε = 0 is the limit, not the family) | not applicable | classical / Clay BT-544 |
| ε ≪ 1, well-prepared, s = 0 (energy norm) | uniform local on [0, T] (Klainerman–Majda 1982 Euler skeleton; NS analog Hagstrom–Lorenz / Métivier–Schochet by author/year only) | α(0) ∈ [1, 2] (Feireisl–Novotný 2009 energy bounds; Desjardins–Grenier 1999 give explicit ε rates for subclasses) | weak: yes (Lions–Masmoudi 1998); strong: yes for low s | well-established |
| ε ≪ 1, well-prepared, s ∈ (0, s_low] for some "low" s_low | uniform local on [0, T] (same skeleton) | α(s) > 0 conjectured; partial results in Danchin 2002, Masmoudi 2007 (by author/year only) for specific (s, p) Besov indices | strong: yes in those Besov classes | partially established; **explicit s-dependence UNKNOWN** at general s_low |
| ε ≪ 1, well-prepared, s ∈ (s_low, s*) | uniform local on [0, T] (skeleton holds at high s if data are smooth) | **UNKNOWN** — this is the conjectural "rate-decay window" | strong: conjecturally yes with decreasing rate | **the heart of the R5-Aux open piece** |
| ε ≪ 1, well-prepared, s = s* | uniform local on [0, T] (skeleton holds); acoustic amplification reaches the well-prepared decay | **CONJECTURE**: α(s*) = 0 (no uniform-in-ε rate at this Sobolev index) | strong: conjecturally **fails** uniformly; ε > 0 family loses uniform regularity at this scale | **R5-Aux open piece** — existence and value of s* conjectural |
| ε ≪ 1, well-prepared, s > s* | uniform local on [0, T] (skeleton); acoustic amplification dominates | **CONJECTURE**: α(s) = 0 for all s > s* (no uniform rate) | strong: ε → 0 limit blows up in H^s for s > s* | **CONJECTURE** under the s* hypothesis; UNKNOWN under literature |
| ε ≪ 1, ill-prepared | uniform local on [0, T] for slow-mode component (Schochet 1986 averaging); fast-mode L^∞ is O(1) not O(ε) | α(s) for slow mode same as well-prepared; α(s) for full solution **not** ε-small | weak: yes (averaged); strong: requires Schochet averaging | well-established for slow mode; full-solution rate ill-defined at ε = 0 |
| ε ~ O(1) (compressible NS proper) | local strong: classical (Matsumura–Nishida 1980, by author/year only); global strong: **UNKNOWN** | not applicable (no singular limit at fixed ε) | not applicable | separate problem class — compressible NS regularity, also open |

### §3.2 Reading the table

Three regimes are well-understood:

1. **ε = 0 limit** (incompressible NS): classical, with the global
   smoothness question = Clay BT-544.
2. **ε ≪ 1, s = 0** (energy norm, well-prepared): Feireisl–Novotný
   2009 + Lions–Masmoudi 1998 give uniform rates.
3. **ε ~ O(1)** (compressible NS proper): a separate open problem
   class with its own classical theory.

Three regimes are **UNKNOWN** in the precise α(s)-rate sense:

- s ∈ (s_low, s*): the conjectural "rate-decay window" where
  α(s) ∈ (0, 1] is decreasing in s but positive.
- s = s*: the conjectured *compressibility-singularity barrier*
  where α(s*) = 0.
- s > s*: complete loss of uniform rate (under the s* hypothesis).

The **R5-Aux open piece** is the determination of α(s) and s*.
Three conjectural positions:

- **D1**: u_ε converges to incompressible NS u_0 in
  C([0, T]; H^s) at the rate ε^{1/2} **uniform in s** (the
  "standard" rate, with no s-dependence). Equivalently α(s) = 1/2
  for all s ≥ 0; s* = ∞ (no singularity barrier).
- **D2**: rate is ε^{α(s)} for some s-dependent α — α(s) is
  positive for all s but **decreasing** in s, with a specific
  functional form to determine.
- **D3**: α(s) drops to 0 at a finite s* — there exists a
  **compressibility-singularity barrier** in the singular limit,
  i.e. above some Sobolev index uniform-in-ε regularity fails.

---

## §4 Smallest open piece — chosen conjecture and actionable evidence

### §4.1 Choice of conjecture

The dispatch selects **D2**: *rate is ε^{α(s)} for some
s-dependent α(s), with α(s) > 0 for all s ≥ 0 but **strictly
decreasing in s** beyond s = 0*.

**Reasons for selecting D2 over D1, D3**:

1. **D1 (uniform rate ε^{1/2} at all s)** would mean the acoustic-
   mode amplification does **not** affect the rate. This
   contradicts the heuristic that each spatial derivative pulls
   out a factor 1/ε from acoustic waves of frequency 1/ε; if true
   it would be a striking *insensitivity* result.
2. **D3 (finite s* with α(s*) = 0)** would mean a genuine
   compressibility-singularity barrier exists. While physically
   plausible, this is the *strongest* of the three positions and
   would require constructing a counterexample (a sequence of
   well-prepared data with H^{s*}-norm blowing up uniformly in ε).
   It is a stronger claim than D2.
3. **D2** is the *generic* prediction: the rate decreases in s
   but stays positive, capturing the acoustic-amplification
   heuristic without committing to a finite barrier. Subsumes the
   "near-D1 at low s, near-D3 at high s" interpolation.

D2 is also the most *actionable*: it predicts a specific
functional form α(s) (linear, e.g. α(s) = α(0) - c · s for some
slope c > 0; or piecewise-linear; or rational) that can be probed
by isolating the acoustic-mode contribution to a fixed Sobolev
norm at specific s-values where partial results exist
(Desjardins–Grenier 1999 at s = 1, Masmoudi 2007 at related
indices, by author/year only).

### §4.2 Smallest discrete result that would advance D2

**Lemma 1 candidate (R5-Aux probe)**:

> *Show that for well-prepared compressible NS on T³ at viscosity
> ν > 0 fixed and ε ∈ (0, ε_0], the uniform-in-ε rate at
> s = 1 (gradient-norm) is strictly less than the rate at s = 0
> (energy-norm), for at least one class of well-prepared initial
> data.*

This would establish a **strict s-dependence** α(1) < α(0),
ruling out D1 (which requires α(0) = α(1)) and partially
supporting D2 (under the further requirement that α(s) remain
positive at s = 1, which would be incompatible with a
near-zero-s singularity barrier in D3).

**Why this is the smallest open piece**:

- It is **localized**: it isolates a single Sobolev index pair
  (s = 0, s = 1), rather than the global functional form
  α(s).
- It is **NS-specific**: it isolates the ν > 0 case; the Euler
  (ν = 0) Klainerman–Majda 1982 case is ruled out as the source.
- It is **falsifiable**: either α(0) = α(1) (refuting D2,
  supporting D1) or α(0) > α(1) (supporting D2's strict
  s-dependence).
- It is **template-driven**: Desjardins–Grenier 1999 + Masmoudi
  2007 (and Danchin 2002 in critical Besov) provide the iteration
  scheme template at s = 0; the only new analytic content is
  tracking the gradient-norm rate explicitly.

### §4.3 Actionable evidence step (1-line)

> **Track the explicit ε-power of the gradient-norm
> ‖∇(u_ε - u_0)‖_{L^∞_t L²_x} in the Desjardins–Grenier 1999 (or
> Danchin 2002) energy-method estimates, comparing it to the
> energy-norm ε-power, and record any strict gap as a candidate
> lower bound for α(0) - α(1) > 0.**

This is a literature-scan-plus-careful-bookkeeping task, not a new
proof; estimated effort 1-2 sessions of close reading + 1 session
of bookkeeping. No new theorems are required.

### §4.4 Falsifiers for D2

- **F-D2-A** (rate-uniformity surprise): if a published result
  (post-2020) establishes a *uniform-in-s* rate ε^{1/2} (or any
  fixed power) for well-prepared compressible NS on T³, D2
  collapses toward D1 (no s-dependence). **Status: not active
  under current literature** — no such uniform rate is in the
  public record to this dispatch's knowledge; the rate at higher
  s is consistently weaker than at s = 0 in the partial results.
- **F-D2-B** (singularity-barrier evidence): if a published
  construction (post-2020) exhibits a sequence of well-prepared
  data with H^s-norm of u_ε - u_0 not vanishing as ε → 0 for
  some finite s = s*, D2 collapses toward D3 (finite barrier).
  **Status: not active** — no such construction is published to
  this dispatch's knowledge; D3 remains open.
- **F-D2-C** (literature-already-resolved): if α(s) is already
  fully characterized in the published record (e.g. in a 2010-2025
  monograph or paper this dispatch did not locate), the smallest-
  open-piece selection is premature. **Status: watch-state**, not
  active under current scan; the specific *function* s ↦ α(s) is
  not in the literature this dispatch surveyed, only partial
  values at specific s.
- **F-D2-D** (γ-exponent dependence): if α(s) depends sensitively
  on the pressure law γ (i.e. different γ produce qualitatively
  different α(s)), the dispatch's γ > 1 isentropic choice (§1.2)
  becomes restrictive and a γ-family extension is needed. **Status:
  not active** under the dispatch's choice; γ-sensitivity is a
  related caveat (§6).

---

## §5 Cross-axis tie: R5-Aux and BT-544 main line

R5-Aux is **auxiliary** to BT-544 (per the parent seed and parent
acceptability), but the two are connected via the following
structural observation, parallel to the R1 dispatch §5.

### §5.1 The compressible–incompressible corridor

BT-544's main line targets *global smoothness for smooth
incompressible initial data on [0, ∞)*. R5-Aux targets *uniform-
in-ε regularity rates on a fixed [0, T] for the compressible
family*. These are *different* statements, and BT-544's
incompressible global-smoothness target is consistent with bounded
α(s) on a finite [0, T] (the rate function is about how the
compressible family approaches the incompressible limit, not about
global existence at ε = 0).

R5-Aux sits **adjacent** to BT-544:
- BT-544 is the ε = 0 endpoint, asking about global behaviour
  on [0, ∞);
- Klainerman–Majda 1982 / Schochet 1986 are the local-in-time
  uniform-existence skeleton at ε > 0;
- R5-Aux asks for the **rate function** α(s) characterizing how
  ε > 0 compressible solutions approach the ε = 0 incompressible
  limit on the local interval [0, T].

A precise α(s) would localize the "compressibility resilience" of
incompressible NS, giving BT-544 a *quantitative measure* of how
much of the compressible-NS structure survives the singular limit.

### §5.2 Indirect implication for BT-544

If D2 is correct and α(s) is strictly decreasing in s, then:

- For initial data well-prepared at high Sobolev index s, the
  compressible approximation u_ε is uniformly close to the
  incompressible u_0 in lower-Sobolev norms but **less uniformly
  close** in higher-Sobolev norms — the higher-Sobolev structure
  of u_0 is *not* recoverable as a singular limit at a fixed rate.
- BT-544's global-smoothness question is then refined: not just
  "does smoothness propagate at ε = 0?", but also "at what
  Sobolev index does the compressible regularization stop being a
  uniform proxy for the incompressible smoothness?".

This refinement is **not a proof step** for BT-544; it is a
structural clarification. The Clay-status remains 0/1, and R5-Aux
resolution leaves Clay BT-544 at 0/1.

### §5.3 Atlas-saturation tie

Per `omega-cycle-bt544-ns-2026-04-25.md` §3, BT-544's mechanism
saturation is ~0.05 (no PDE mechanism produced; only arithmetic
relabeling in the n=6 frame). R5-Aux *if resolved* would produce a
**genuine PDE invariant** (the function α(s) characterizing
compressible-incompressible singular-limit rates is a real
analytic object, not an arithmetic relabeling). Together with R1's
α*_NS, this would *raise* the mechanism saturation toward the
omega ceiling reference 0.835 — but only for the auxiliary
problems, not for BT-544 itself. The R1 + R5 pair would together
populate two distinct PDE-mechanism axes (regularity-class
threshold vs singular-limit rate), both auxiliary to BT-544.

### §5.4 Parallel to R1 dispatch

R1 and R5 are **structurally parallel** auxiliary recasts:
- R1 recasts axiom A2 (regularity class) → invariant α*_NS
  (Hölder-uniqueness threshold);
- R5 recasts axiom A6 (incompressibility) → invariant α(s)
  (uniform-in-ε rate function).

Both produce *new analytic invariants* (PASS the F-544-B novelty
test); both have *partial-result density* (R1: Onsager →
Constantin–E–Titi → Isett → Buckmaster–Vicol; R5: Klainerman–Majda
→ Schochet → Lions–Masmoudi → Feireisl–Novotný); both are
*Millennium-grade-preserving* (do not trivialize, do not over-
shoot). They are *independent* directions, and the parent
acceptability §4 ranks R1 as the higher-priority near-term
dispatch (R5 second). This dispatch executes R5 as the parallel
direction.

---

## §6 Caveats

### §6.1 T³ vs ℝ³

The dispatch fixes T³. On ℝ³:
- Acoustic modes are continuous (not discrete), so Schochet
  averaging requires Strichartz-type dispersive estimates rather
  than fast-mode averaging on a torus;
- Decay-at-infinity bookkeeping changes (Schwartz-class data,
  energy density vs energy);
- Feireisl–Novotný 2009 covers the unbounded case in detail; the
  uniform energy rate α(0) is comparable but the s-dependence may
  differ because dispersive estimates can *raise* α(s) on ℝ³
  (acoustic energy disperses to infinity, supplying a uniform-in-s
  decay) where periodic acoustic energy does **not** disperse;
- The expected α(s) function is **plausibly different** on T³
  and ℝ³ (unlike the R1 case, where domain-independence is
  conjectured): T³ traps acoustic waves (no dispersion), making
  the rate-decay heuristic **stronger** on T³.

**Implication**: R5-Aux on T³ is the dispatched form. The ℝ³
variant has a *different* expected rate function and is recorded
as a *related but distinct* auxiliary.

### §6.2 Isentropic vs full NS

The dispatch fixes isentropic compressible NS (p = ρ^γ). For full
compressible NS (with temperature, Fourier heat conduction):
- An additional thermal time-scale appears, with its own ε-scaling;
- Feireisl–Novotný 2009 monograph treats both cases; the rate
  function α(s) for the full system is conjecturally
  *no-better-than* the isentropic case at any s;
- The full NS variant is a separate auxiliary, not absorbed.

**Implication**: α(s) for isentropic and α(s) for full NS may
differ. The dispatch chooses isentropic for cleanliness; full NS
is a related auxiliary.

### §6.3 Periodic (well-prepared) vs decaying / ill-prepared

The dispatch uses well-prepared initial data on T³. Variants:
- **Ill-prepared** data (Schochet 1986 averaging): the slow-mode
  rate is comparable to well-prepared, but the full-solution rate
  α(s) is **not** ε-small (acoustic O(1) mode persists). The
  dispatched α(s) refers to the well-prepared case.
- **Strongly decaying** data on ℝ³ (Schwartz-class): adds a
  dispersive contribution (§6.1).
- **Boundary data** on a bounded domain with no-slip: a separate
  problem class with boundary-layer issues; not addressed by this
  dispatch.

**Implication**: well-prepared T³ is the cleanest setting; other
variants are noted as related but distinct.

### §6.4 Strong vs weak solution

The dispatch uses *strong* solutions on [0, T] for both ε > 0 and
ε = 0. For *weak* solutions (Lions–Masmoudi 1998 layer):
- The convergence is qualitative (passing to the limit), with
  rates available only for specific norms;
- Whether α(s) (as a strong-regularity rate) matches the weak-
  level convergence in any meaningful sense is **UNKNOWN** at
  this dispatch level.

**Implication**: α(s) for strong solutions and any analog for
weak solutions may differ. The dispatch chooses strong to match
the higher-Sobolev rate question; the weak variant is a separate
auxiliary.

### §6.5 Time interval [0, T] vs global

The dispatch fixes [0, T]. Global uniform-in-ε regularity is
conjecturally Clay-equivalent (parent acceptability §3.1) and is
**not** the dispatch target. If global uniform-in-ε regularity
were established, then by passing to the limit one would obtain a
global smooth incompressible solution under whatever well-prepared
hypothesis is uniform — which is conjecturally Clay-equivalent.
The dispatch deliberately stays on [0, T] to avoid this
collapse-to-Clay.

### §6.6 Conjecture-status reminder

D2 is a **conjecture**, not a proof claim. The dispatch registers
it for direction-design only. Falsifiers F-D2-A..D are recorded in
§4.4 and remain inactive under current literature.

---

## §7 Anti-list — conjectures considered and rejected

These conjectural formulations of R5-Aux were considered as the
"smallest open piece" target and rejected in favor of D2.

| candidate conjecture | reason rejected |
|---------------------|-----------------|
| α(s) = 0 for all s ≥ 0 (no uniform-in-ε rate at any Sobolev index) | this contradicts Feireisl–Novotný 2009 directly (energy-level uniform rates are established at s = 0); rejected as falsified by published result |
| α(s) = ∞ for some s (super-polynomial rate, e.g. exponential ε^∞ convergence) | viscous compressible NS uniform decay is rate-limited by the acoustic-mode L²-norm being only O(ε); super-polynomial decay would require a non-standard analytic structure with no lineage support; rejected as too broad |
| α(s) = 1 for all s (uniform first-order rate at every Sobolev index) | would imply the acoustic-mode amplification is exactly cancelled by well-prepared decay at every Sobolev order; no lineage support; rejected as too narrow without independent evidence |
| α(s) expressible in n=6 lattice arithmetic (e.g. α(s) = (n - s)/n = (6 - s)/6, or α(s) = 1/(n/φ) = 1/3 reading Lions–Masmoudi as a lattice fact) | F-Acc-3 (parent §7) novelty-collapse falsifier — would relabel α(s) as a lattice arithmetic fact, not an analytic invariant; rejected as the F-544-B failure mode |
| determine α(s) for **global** [0, ∞) interval | conjecturally Clay-equivalent (parent §3.1); rejected as HARDER_THAN_AUXILIARY (collapses R5-Aux to Clay) |
| determine α(s) for **full** (non-isentropic) NS with thermal mode | changes problem identity (§6.2 caveat); rejected as a related but distinct auxiliary |
| determine α(s) for **bulk-viscosity-modified** compressible NS (parent seed §6 anti-list) | parameter ε replaced by bulk-viscosity λ, which is less canonical than Mach number; rejected per parent seed |
| D3' "α(s) drops to 0 at *exactly* s* = some explicit closed-form value" | any closed-form prediction at this dispatch stage would be premature; recorded as too narrow until at least one of the Lemma 1 probes returns evidence on whether α(s) is monotonic or has a barrier |

The chosen D2 ("α(s) > 0 for all s, strictly decreasing in s
beyond s = 0") is the most actionable: narrow enough to admit a
Lemma 1 probe (the s = 0 vs s = 1 rate gap), broad enough to
permit literature scan + bookkeeping rather than new theorems.

---

## §8 Falsifiers active for the dispatch

These falsifiers apply to **this dispatch** as a research-design
artifact, separate from BT-544's own falsifiers and separate from
the parent seed's F-D2-A..E and parent acceptability's F-Acc-1..6.
(Note: the seed's F-D2-A..E falsifiers refer to the seed-level
recast acceptability and live in a different namespace from this
dispatch's F-D2-A..D rate-conjecture falsifiers in §4.4.)

- **F-Disp-1** (conjecture-fabrication): if the D2 conjecture is
  shown to be **published** in either direction (proven or
  disproven) before this dispatch is acted on, the dispatch must
  be revised. Currently, to this dispatch's knowledge of the
  literature, the rate function s ↦ α(s) for well-prepared
  compressible NS on T³ is only partially characterized, and
  D1/D2/D3 are all live possibilities. **Status: not active**.
- **F-Disp-2** (lineage-mismatch carry-over): if any citation
  carried from the parent acceptability §5 is shown to not
  contain the result attributed (the F-Acc-2 falsifier), the
  dispatch's known-result map (§3) is correspondingly weakened.
  Lions–Masmoudi 1998 venue ambiguity (parent §5 footnote) is
  flagged on the R5 side — the canonical incompressible-limit
  paper appears in J. Math. Pures Appl. 77 rather than the seed-
  cited Ann. Inst. H. Poincaré C; this dispatch records both
  attributions and notes the result (global incompressible-limit
  for weak compressible NS solutions) does exist in the published
  record. **Status: flagged on Lions–Masmoudi venue, not failing**.
- **F-Disp-3** (open-piece-misidentification): if Lemma 1 (§4.2)
  is shown to be **trivial** (e.g. the s = 0 vs s = 1 rate gap is
  already explicitly documented in Desjardins–Grenier 1999 or
  Danchin 2002 or a sequel), the smallest-open-piece selection is
  premature; the next-smaller piece must be identified (e.g.
  s = 1 vs s = 2 gap, or a specific Besov refinement). **Status:
  watch-state**, not active under current scan.
- **F-Disp-4** (atlas-touch): if this dispatch leads to any
  `shared/n6/atlas.n6`, `state/proposals/inventory.json`, or
  `theory/canon/` edit, the dispatch has been mis-applied. The
  dispatch is research-direction-design only. **Status: not
  active**.
- **F-Disp-5** (claim-creep): if the D2 conjecture is reported
  outside this report as a "proven" result rather than a
  conjecture, the dispatch has been mis-cited. **Status: not
  active**; this report registers D2 as conjecture only.
- **F-Disp-6** (auxiliary→main confusion): if R5-Aux resolution is
  reported as resolving Clay BT-544, the auxiliary→main distinction
  has been lost. R5-Aux is auxiliary; BT-544 stays 0/1. The
  global-uniform-in-ε variant is excluded from R5-Aux precisely to
  prevent this collapse (§6.5). **Status: not active**.
- **F-Disp-7** (R1-R5 confusion): if the R1 dispatch's invariant
  α*_NS (Hölder-uniqueness threshold) is conflated with R5's
  invariant α(s) (uniform-in-ε rate function), the recasts have
  been merged inappropriately. They are independent invariants on
  different axiom recasts (A2 vs A6). **Status: not active**;
  this dispatch is parallel to R1, not joint with it.

None of F-Disp-1..7 fires under this report's scope.

---

## §9 Closing

0/7 unchanged. NS Clay statement unchanged. No atlas/state/inventory
edits. R5-Aux precise definition recorded (§1); known-result map
across (ε, s) plane charted (§3) with explicit UNKNOWN entries at
s ∈ (s_low, s*) and s ≥ s*; conjecture **D2** selected as the most
actionable position for the rate function s ↦ α(s) (§4.1); Lemma 1
(s = 0 vs s = 1 rate gap in Desjardins–Grenier / Danchin energy
methods) identified as the smallest open piece (§4.2-§4.3); cross-
axis tie to BT-544 main line via the compressible–incompressible
corridor (§5); caveats on T³-vs-ℝ³, isentropic-vs-full,
well-prepared-vs-ill-prepared, strong-vs-weak, [0,T]-vs-global
recorded (§6); anti-list of considered-and-rejected conjectures
(§7); dispatch-level falsifiers F-Disp-1..7 inactive (§8).

Per F-544-B and the L_ω apex distinction reaffirmed in the parent
seed: this dispatch designs research direction for an auxiliary
problem, not for BT-544 itself. BT-544 remains catalogue-exhausted
at L9 (per
`omega-exec-bt544-fallback-molt-validation-2026-04-25.md` §7) and
0/1 untouched at the Clay level. Resolving R5-Aux would inform
BT-544 indirectly via the §5.1 corridor; it would not resolve
BT-544.

R5 dispatch parallels R1 dispatch
(`omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md`); R1
remains the higher-priority near-term direction per the parent
acceptability §4 ranking, with R5 the second priority pursued in
parallel under this dispatch. Both auxiliaries are independent;
neither is BT-544; together they populate two distinct
PDE-mechanism axes within the L_ω axiom-recast frame.
