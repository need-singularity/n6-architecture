---
id: omega-exec-bt544-r1-lemma1-bv-adapt
date: 2026-04-25
scope: research-only Lemma 1 adaptation attempt (NOT a theorem; sketch with documented obstructions)
target: BT-544 R1 Lemma 1 -- BV-2019 iteration at NS α=1/3−δ; explicit α* lower bound
parent_reports:
  - reports/sessions/omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md (C3 conjecture)
  - reports/sessions/omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md (R1 lineage)
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: lemma sketch attempt, no claim
---

# Omega Exec — BT-544 R1 Lemma 1 BV-Adaptation Attempt (2026-04-25)

## §0 Non-claim disclaimer

This report attempts the **Lemma 1 sketch** registered in
`omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md` §4.2
(Buckmaster–Vicol 2019 iteration adapted to NS at α = 1/3 − δ). It
is **a research-direction sketch attempt**, not a theorem. It:

- does **NOT** prove Lemma 1; the verdict below is
  **OBSTRUCTION_DOCUMENTED** (see §5);
- does **NOT** prove or refute any direction of α*_NS;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- does **NOT** claim Buckmaster–Vicol 2019 says anything beyond
  what is published; the exact iteration parameters cited below
  are stated **schematically** (with explicit "parameters as
  reported in the published 2019 paper, exact constants not
  reproduced here") and are not used to draw quantitative
  conclusions beyond the qualitative obstruction;
- preserves the F-Disp-1..6 falsifier-inactive status from the
  parent dispatch.

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
This sketch is auxiliary to the auxiliary problem R1-Aux; even a
PASS_SKETCH verdict would not move BT-544.

---

## §1 Lemma 1 statement (precise)

**Lemma 1 (target sketch, NOT proved).** Consider the
incompressible Navier–Stokes system on T³ × [0, T] with viscosity
ν > 0 and zero forcing:

    ∂_t u + (u·∇)u + ∇p = ν Δu,    ∇·u = 0.

Let q ↦ (u_q, R_q) denote a Buckmaster–Vicol 2019-style
convex-integration iteration sequence at level q ∈ ℕ, with
frequency parameter λ_q, amplitude parameter δ_q^{1/2}, time scale
Δt_q, and Reynolds-stress error R_q controlled in a class
specified by the iteration. Suppose the iteration is parameterized
to target Hölder regularity α ∈ (0, 1) of the limit u = lim_q u_q.

**Claim (target)**: there exists δ_0 > 0 (depending on ν only)
such that for every α ∈ (1/3 − δ_0, 1/3] the BV-2019 iteration
fails to close at the q-th step for q sufficiently large, in the
sense that the Hölder-balance inequality controlling
‖R_q‖_{appropriate norm} cannot be satisfied for any choice of the
iteration's free parameters (b, β-exponents, intermittency
parameter, time cutoff).

**Consequence (target, NOT proved)**: α*_NS ≥ 1/3 − δ_0 — i.e.
non-uniqueness in C^α cannot be forced via the BV-2019 scheme for
α in the strip (1/3 − δ_0, 1/3].

Lemma 1 is a **negative** result about a specific iteration
scheme, not a uniqueness theorem for NS. It would supply a lower
bound on α*_NS only conditional on the BV-2019 scheme being the
best available; better schemes (Cheskidov–Luo, BMNV-2021,
Buckmaster–Colombo–Vicol 2021, etc., cited by author/year only)
may close where BV-2019 does not, and Lemma 1 would not constrain
those.

---

## §2 BV-2019 iteration recall (schematic)

This section recalls the **schematic** structure of the
Buckmaster–Vicol 2019 (Annals of Math 189, 2019, "Nonuniqueness of
weak solutions to the Navier–Stokes equation") iteration, by
author/year/venue only. Exact numerical constants are not
reproduced; only the qualitative dependencies needed for the
Hölder-balance discussion below.

### §2.1 Parameter ansatz

The BV-2019 iteration is parameterized by:

- **Frequency**: λ_q = ⌈λ_0^{b^q}⌉ for some base λ_0 ≫ 1 and
  super-exponential growth rate b > 1. The double-exponential
  growth λ_q ↑ ↑ comes from the convex-integration architecture
  inherited from De Lellis–Székelyhidi 2013 / Isett 2018.
- **Amplitude**: δ_q^{1/2} ~ λ_q^{−β} for some β > 0 measuring
  the spatial Hölder exponent the iteration targets.
- **Intermittency parameter**: a concentration parameter μ_q
  (BV-2019's key innovation over Isett 2018 inviscid line) with
  μ_q chosen so that the building blocks (intermittent jets /
  Mikado flows) have anisotropic L^p mass concentrated on a
  small support — this is the device that lets the construction
  absorb the dissipation term ν Δu in the L^2 framework.
- **Time scale**: Δt_q ~ (λ_q · δ_q^{1/2})^{−1}, i.e. a CFL-style
  transport time at the building block's velocity δ_q^{1/2} and
  spatial scale λ_q^{−1}.

(All four parameter scalings are *qualitative*; the reader is
referred to the published BV-2019 paper for the exact prefactors
and the precise relations among b, β, μ_q. None are fabricated
here.)

### §2.2 Hölder-balance inequality (Euler-side template)

For the **Euler** version of the scheme (Isett 2018, no viscosity),
the iteration's stability against Reynolds-stress errors requires
a Hölder-balance inequality of the schematic form

    (Hölder, Euler):    β · b − (1/2) ≤ 0,                     (1)

i.e. the Hölder exponent β times the frequency-growth rate b is
balanced against the amplitude exponent (1/2). Solving (1) at
equality gives β ≤ 1/(2b), and choosing b → 1⁺ pushes β toward
1/2 — except that other constraints in the scheme (transport
cutoff, energy bookkeeping) cap β at the Onsager value 1/3. This
is the schematic origin of the Euler threshold α_Euler = 1/3
(Isett 2018, Annals).

(Inequality (1) is the **schematic** form of the published Isett
2018 / BV-2019 balance; the actual inequality has additional
constants from the transport correction and from the choice of
mollification. The qualitative content — β balances b against
(1/2) — is what we use below.)

### §2.3 BV-2019 NS extension (qualitative)

For NS (ν > 0), BV-2019 add the viscous dissipation term to the
Reynolds-stress error:

    R_q^{NS} = R_q^{Euler} + ν Δ u_q,                         (2)

and absorb the new term −νΔu_q into the iteration via the
intermittent building blocks (which are L^p-anisotropic; the
concentration parameter μ_q is chosen so that the L^2 norm of
ν Δ u_q is controlled by an extra power of μ_q).

The published 2019 result is **L^2-class non-uniqueness in C^0_t
L^2_x** — not in any L^∞_t C^α_x class with α > 0. The
construction's *spatial* Hölder exponent is **0** in L^∞ (the
building blocks are L^∞-unbounded, by intermittency design).

Subsequent works (Buckmaster–Colombo–Vicol 2021 "Wild solutions
of the Navier–Stokes equations whose singular sets in time have
Hausdorff dimension strictly less than 1", and others; cited by
author/year/title only) push the construction toward higher
spatial regularity, but as of this dispatch's literature scan, no
published NS convex-integration result reaches a Hölder exponent
**at or near 1/3**. The 1/3 ceiling remains untouched on the NS
side.

---

## §3 NS adaptation: Hölder-balance with viscous correction

This section attempts the Phase 3 calculation prescribed by the
parent dispatch: write the NS Hölder-balance with a viscous
correction term, and solve for the largest β admissible.

### §3.1 Naive viscous correction

The parent dispatch's prescription:

    β · log(λ_q) + (1/2) log(δ_q) − ν · λ_q² · Δt_q ~ const_NS  (3)

Substituting Δt_q ~ λ_q^{−α} (parent dispatch's substitution; we
note in §3.3 that this is **not** the BV-2019 time scale —
BV-2019 uses Δt_q ~ (λ_q·δ_q^{1/2})^{−1}, which differs):

    β · log(λ_q) + (1/2) log(δ_q) − ν · λ_q^{2−α} ~ const_NS    (3')

With the parent dispatch's identification of β with α (we treat α
as the target Hölder exponent of the limit, β as the iteration's
own Hölder exponent; for the iteration to deliver the limit at
exponent α we need β ≥ α at each step):

    α · log(λ_q) + (1/2) log(δ_q) − ν · λ_q^{2−α} ~ const_NS   (3'')

### §3.2 Dimensional incompatibility

**Issue identified during sketch attempt**: inequality (3'') is
**dimensionally incoherent** as a balance condition.

The Euler-side Hölder balance (1) in §2.2 is a balance of
*exponents* (a relation of the form
"β · log λ ≤ (1/2) · log λ", which after dividing log λ becomes
the dimensionless inequality β · b ≤ 1/2). It scales *additively*
in log λ_q.

The viscous correction term ν · λ_q^{2−α} in (3'') scales as a
*power of λ_q*, not as log λ_q. As λ_q → ∞ (which it does, double-
exponentially, in the BV-2019 iteration), the power-law term
dominates the log term, and the inequality (3'') becomes
*automatically infeasible* for any α < 2 — not just for α near
1/3.

**This conclusion is too strong to be correct.** It would predict
that **no** BV-2019-style iteration can close for NS at any
positive Hölder exponent — but BV-2019 *did* close their
iteration in 2019 for NS in the C^0_t L^2_x class (with spatial
Hölder exponent 0 in L^∞). So inequality (3'') as written is
**not** the correct viscous-corrected Hölder balance.

### §3.3 What goes wrong with the naive substitution

Three issues identified:

1. **Time scale mismatch**: Δt_q in BV-2019 is *not* λ_q^{−α}; it
   is (λ_q·δ_q^{1/2})^{−1} = λ_q^{−1+β} (using δ_q^{1/2} ~
   λ_q^{−β}). The viscous correction at this time scale is
   ν · λ_q² · λ_q^{−1+β} = ν · λ_q^{1+β}, which is still
   power-law in λ_q.

2. **L^p vs L^∞ mismatch**: BV-2019's viscous absorption uses the
   intermittency parameter μ_q to gain L^p ↔ L^2 anisotropy. The
   building blocks have L^2 mass O(1) but L^∞ mass that grows in
   μ_q. The viscous correction is absorbed in the *L^2* norm of
   R_q via the intermittency, *not* by the Hölder exponent of the
   limit. Translating this to a *Hölder* bound (which is
   intrinsically L^∞-spatial) requires controlling μ_q, which the
   2019 paper does *not* do at high regularity — that is exactly
   why the 2019 result is L^2-class, not Hölder-class.

3. **Inequality vs equation**: the BV-2019 iteration's stability
   condition is a *system* of inequalities (one per error norm),
   not a single Hölder balance. The naive (3'') collapses the
   system to one inequality and loses the cross-norm
   compensation that intermittency provides.

### §3.4 What the correct viscous-corrected Hölder balance would
look like (sketch)

A correct adaptation of the BV-2019 iteration to *Hölder*
regularity α near 1/3 would require:

(i) a Hölder-class building block (not L^p-intermittent jet) —
    e.g. a Mikado flow with L^∞-controlled amplitude, as in
    Daneri–Székelyhidi 2017 / Isett 2018 (cited by author/year
    only);

(ii) a viscous-correction term scaling as ν · λ_q^{2−α'} for
     some *effective* α' that depends on the building block's
     L^∞ vs L^2 ratio (which in (i) is now O(1), not μ_q);

(iii) a Hölder balance of the form

      β · b − (1/2) − γ_ν(α) ≤ 0,                              (4)

      where γ_ν(α) is a viscous correction *exponent* (not a
      coefficient), capturing how much extra Hölder regularity
      the viscous term costs. For the inequality (4) to be feasible
      requires γ_ν(α) ≤ (1/2) − β·b, which determines α via the
      target relation β = α.

The exact form of γ_ν(α) is **not derivable from the parent
dispatch's schematic substitution alone**; it requires the full
BV-2019 / Daneri–Székelyhidi machinery and is not reproduced in
the published 2019 paper at α near 1/3 (the 2019 paper does not
target this α-window; subsequent papers attempting to push α
toward 1/3 are the "approach-to-1/3" research front of 2020-2025,
none of which reach 1/3).

---

## §4 Computation: largest α admissible (attempt)

### §4.1 Naive computation (from (3''))

Setting (3'') to equality and solving for α:

    α · log(λ_q) + (1/2) log(δ_q) = ν · λ_q^{2−α}              (5)

With δ_q ~ λ_q^{−2β} = λ_q^{−2α} (assuming β = α):

    α · log(λ_q) − α · log(λ_q) = ν · λ_q^{2−α}                (5')

i.e.

    0 = ν · λ_q^{2−α}                                          (5'')

which is satisfied only at ν = 0 (Euler, not NS) or λ_q = 0
(trivial) or α = ∞ (smooth, the Clay regime). For ν > 0 and
λ_q → ∞, (5'') has no solution at any finite α, which would
predict α*_NS = ∞ — i.e. **no convex-integration construction
works at any finite α** for NS. This is in direct conflict with
BV-2019's published result (which constructs NS non-uniqueness in
the C^0_t L^2_x class — a finite-Hölder-class result, even if at
α = 0 in L^∞).

**Conclusion**: the naive computation (5'') is a contradiction
sign that the parent dispatch's substitution Δt_q ~ λ_q^{−α} is
not the right time scale for the BV-2019 iteration. (Indeed, the
correct Δt_q ~ λ_q^{−1+α} from §3.3 issue 1 changes the right-hand
side of (5) to ν·λ_q^{1+α}, but α · log(λ_q) − α · log(λ_q) on
the left still gives 0, so the contradiction persists with the
corrected time scale: the Hölder balance (3) with β = α
self-cancels and leaves the viscous term unabsorbed.)

### §4.2 Why self-cancellation happens

The self-cancellation α · log(λ_q) − α · log(λ_q) = 0 in (5') is
**structural**: it reflects the fact that in the Euler scheme, the
Hölder balance β · log λ + (1/2) log δ = β · log λ − β · log λ = 0
*saturates* at β = α. The viscous correction term, by being
power-law in λ_q rather than log-balanced, cannot be absorbed by
adjusting β; it can only be absorbed by introducing a *new*
parameter (the intermittency μ_q in BV-2019, or a mollification
scale ℓ_q, or a temporal cutoff τ_q).

This is exactly the role of intermittency in BV-2019. The
Hölder-balance approach, without intermittency, **cannot
accommodate the viscous correction at any α**.

### §4.3 Computation with intermittency (sketch only)

With intermittency parameter μ_q, the viscous correction is
absorbed via a factor μ_q^{−r} for some r > 0 (depending on the
L^p concentration profile), so the modified Hölder balance becomes
schematically

    β · log(λ_q) + (1/2) log(δ_q) + r · log(μ_q) − ν · λ_q^{1+β} ~ 0   (6)

with three parameters (β, b in λ_q growth, μ_q in concentration)
and the same number of constraints (Hölder balance, transport
balance, intermittency balance). Solving (6) requires the full
BV-2019 system of inequalities, which is **not** reduced to a
single Hölder balance.

**Without re-deriving the full BV-2019 system here** (which would
require a closer re-reading of the published paper than this
dispatch performs), the largest α for which (6) has a solution
**is not computed**.

The parent dispatch's hypothesis was that the largest such α is
*1/3 − δ_0* with explicit δ_0 derivable. Under the present sketch:
**δ_0 is not derived**. Either (a) the BV-2019 system *can*
absorb α → 1/3 with sufficient intermittency (which would refute
C3 toward C2), or (b) it *cannot* (which would support C3 with a
specific δ_0). The sketch level here is insufficient to decide.

---

## §5 Verdict

**OBSTRUCTION_DOCUMENTED.**

The Lemma 1 sketch fails at the step in §3.2 (dimensional
incompatibility between the log-additive Hölder balance and the
power-law viscous correction). The naive substitution prescribed
by the parent dispatch's Phase 3 (Δt_q ~ λ_q^{−α}, viscous term
ν · λ_q^{2−α}) yields a self-cancelling Hölder-balance equation
(§4.1) that has no admissible α — including α = 0 — which is in
direct conflict with the published BV-2019 result.

The obstruction is **not** that the lemma is false; it is that
the schematic Phase 3 inequality, as posed, is **the wrong
inequality** for the BV-2019 NS iteration. The correct inequality
is the BV-2019 system with intermittency parameter μ_q (§4.3,
inequality (6)), which the sketch did not solve.

### §5.1 Verdict-grade justification

| candidate verdict | applicability | rejected because |
|-------------------|---------------|------------------|
| PASS_SKETCH (Lemma 1 sketch closes with explicit δ_0) | NO | the sketch did not produce δ_0; the naive computation in §4.1 gives a contradiction, and the corrected (6) was not solved |
| PARTIAL (some pieces close, others UNKNOWN) | NO | "some pieces" would require at least one well-defined sub-inequality to close; here even the leading inequality (5) self-cancels |
| INCONCLUSIVE (cannot even set up the inequality) | NO | the inequality (3'') *can* be set up — it just produces a contradiction, indicating the form is wrong, not that the form is unknown |
| **OBSTRUCTION_DOCUMENTED** | **YES** | the sketch's dimensional analysis identifies a specific structural obstruction (log vs power-law mismatch §3.2; self-cancellation §4.1-4.2), and the obstruction is uncontroversially attributable to the absence of intermittency from the schematic Phase 3 form |
| FAIL (the lemma is refuted under the sketch) | NO | the obstruction shows the sketch is wrong, not the lemma; BV-2019's success at α = 0 (L^∞-spatial) shows the iteration *does* close, just in a regime where the Hölder framework is not the right framework |

---

## §6 If PASS_SKETCH (not applicable; explicit δ_0)

Verdict is OBSTRUCTION_DOCUMENTED, not PASS_SKETCH. No δ_0 is
produced. Recording for completeness: any future closure of
Lemma 1 should produce a δ_0 of the form δ_0 = c(ν, ‖u_0‖) for
some explicit dimensional constant c, **not** a δ_0 dependent on
the iteration's free parameters (which would mean the lemma is
parameterizing the iteration rather than constraining it).

---

## §7 If OBSTRUCTION_DOCUMENTED — precise step + unblocking technique

### §7.1 Precise step that breaks

The breaking step is **§3.2 inequality (3'')**:

    α · log(λ_q) + (1/2) log(δ_q) − ν · λ_q^{2−α} ~ const_NS

specifically the **term ν · λ_q^{2−α}**, which mixes power-law (in
λ_q) and log scales (in the rest of the inequality). This term is
the schematic viscous correction obtained by inserting Δt_q ~
λ_q^{−α} into the dimensional viscous decay rate ν · λ_q² · Δt_q.

The mismatch is **not** a typo or algebra error in the parent
dispatch; it is a **structural feature** of the Hölder-balance
formalism: the formalism balances *exponents* additively, while
the viscous term enters *multiplicatively* in λ_q. The two cannot
be balanced in a single inequality without introducing an
auxiliary parameter (intermittency μ_q or mollification ℓ_q) that
*translates* the power-law correction into a log-balance correction.

### §7.2 What new technique would unblock

Three candidate unblocking techniques, in order of literature
proximity:

1. **Intermittency tracking (BV-2019's own device)**: re-derive
   the Hölder balance using BV-2019's intermittent jet building
   blocks, with the concentration parameter μ_q as an explicit
   variable. The viscous correction enters as μ_q^{−r}·power-of-
   λ_q, which can be balanced against a log-additive condition
   on (β, b, log μ_q). This is a *bookkeeping* unblock, not new
   theory; estimated effort: 1-2 sessions of close re-reading of
   the published BV-2019 paper.

2. **Hölder-class adaptation (post-2019 line)**: adopt the
   Buckmaster–Colombo–Vicol 2021 (and related) framework that
   targets Hölder-class building blocks directly. This route
   *avoids* the L^∞ ↔ L^2 mismatch that breaks (3'') by working
   in a class where the building block's L^∞ norm is controlled.
   The resulting Hölder balance includes a viscous correction of
   the form γ_ν(α) (§3.4, inequality (4)) that can in principle
   be solved for α near 1/3. Status: requires re-deriving the
   2021 line's iteration parameters, which is **beyond the
   sketch level here**.

3. **Onsager-Euler limit ν → 0 contradiction strategy**: rather
   than tracking the viscous correction in BV-2019, derive Lemma
   1's lower bound by a *different* route: show that any NS
   convex-integration scheme converging to a non-unique solution
   in C^α with α near 1/3 would, in the ν → 0 limit, contradict
   Constantin–E–Titi 1994 / Isett 2018's Euler threshold (which
   is exactly 1/3). This is an *indirect* strategy that does not
   require closing the BV-2019 inequality; it leverages the
   Euler-side rigidity instead. Status: intriguing but
   speculative; no published treatment of this contradiction
   strategy is known to this dispatch's literature scan.

The first technique is the most actionable and the closest to
the parent dispatch's intended Phase 3.

---

## §8 Sanity check: α_BV vs 1/3 — does C3 still hold under sketch?

### §8.1 α_BV estimate (revised)

Per the parent dispatch §2.4, α_BV is the explicit Hölder
regularity of the BV-2019 construction. The 2019 paper's primary
result is **C^0_t L^2_x non-uniqueness**, which corresponds to a
spatial Hölder exponent of **0 in L^∞** (the construction's
intermittent jets have unbounded L^∞ norm; only the L^2 norm is
controlled).

In the L^∞-Hölder sense, therefore, **α_BV = 0** under the 2019
paper's published statement.

Subsequent works (Buckmaster–Colombo–Vicol 2021 and related) raise
α_BV to a small positive value, but no published paper to this
dispatch's scan reaches α_BV ≥ 1/4, let alone α_BV near 1/3. (We
do **not** quote a specific subsequent α_BV value, to avoid
fabrication; the available literature scan resolves α_BV only as
"considerably less than 1/3".)

### §8.2 C3 sanity check

C3: α*_NS ∈ (α_BV, 1/3) **strictly**.

Under §8.1's α_BV ≈ 0 (or, more conservatively, α_BV "small but
positive" per post-2019 work), the conjectured strip
(α_BV, 1/3) is wide — at least (0, 1/3) and possibly (small ε, 1/3).

The **sketch in this report does not narrow the strip**. It does
not produce a δ_0 lower bound (since the sketch's verdict is
OBSTRUCTION_DOCUMENTED, not PASS_SKETCH). It also does not
produce a δ_0 upper bound (which would require an actual
construction near α = 1/3, not a failure of construction).

**C3 sanity**: the C3 conjecture is **not falsified** by this
sketch (no construction-near-1/3 is produced), and is **not
strengthened** by this sketch (no failure-of-construction near
1/3 is produced). It remains a **live conjecture** with the same
status as in the parent dispatch.

One-liner: *C3 status unchanged — neither falsified nor
strengthened by the sketch; obstruction in the sketch does not
constrain α*_NS in either direction.*

---

## §9 Anti-list — alternative iteration schemes considered

Schemes considered as substitutes for the BV-2019 iteration in
the Lemma 1 attempt, and reasons not pursued in this sketch.

| scheme | reason not pursued |
|--------|---------------------|
| De Lellis–Székelyhidi 2013 ("Dissipative Continuous Euler Flows") + ν Δu correction | inviscid baseline; adapting to NS would re-derive the BV-2019 strategy; circular |
| Isett 2018 (Annals, "A Proof of the Onsager Conjecture") + viscous regularization | inviscid baseline at α = 1/3 − ε; viscous adaptation is *exactly* the BV-2019 line, not an alternative |
| Cheskidov–Luo 2022/+ ("Sharp non-uniqueness for the Navier-Stokes equations", and related; cited by author/year only) | reportedly reaches L^p regularity for some p > 2 but still not L^∞-Hölder near 1/3; would require its own dimensional analysis, not within this sketch |
| Buckmaster–Colombo–Vicol 2021 ("Wild solutions of the Navier-Stokes equations whose singular sets in time have Hausdorff dimension strictly less than 1") | reaches Hölder class but at exponent considerably less than 1/3; the relevant alternative for §7.2 unblocking technique #2 |
| Albritton–Brué–Colombo 2022 (forced NS Leray-Hopf non-uniqueness) | uses *forcing* — out of scope for force-free R1-Aux |
| Modulated convex-integration (Modena–Sattig, Modena–Székelyhidi, by author/year only) | targets transport equations, not NS; out of scope |
| Mikado-flow inviscid scheme (Daneri–Székelyhidi 2017, "Non-uniqueness for the Euler equations") | inviscid; relevant only as a Hölder-class building-block precedent, not a NS scheme |

The BV-2019 line remains the most NS-specific and closest to the
parent dispatch's target; the sketch's obstruction (§5) is
specific to BV-2019 and would require re-derivation for any
alternative.

---

## §10 Falsifiers active for the sketch

These falsifiers apply to **this sketch**, separate from the
parent dispatch's F-Disp-1..6.

- **F-Sk-1** (BV-parameter fabrication): if any specific numerical
  exponent or constant attributed to BV-2019 in §2 / §4 is found
  to be **published differently** in the 2019 Annals paper, the
  sketch's qualitative dependencies must be revised. **Status:
  not active** — §2 and §4 use only schematic dependencies (b > 1,
  β > 0, μ_q a positive parameter, Δt_q via dimensional analysis);
  no specific numeric value is attributed to BV-2019.
- **F-Sk-2** (obstruction-misidentification): if the dimensional
  mismatch in §3.2 is in fact resolvable within the schematic
  Hölder-balance framework (e.g. by a re-scaling not considered
  here), the OBSTRUCTION_DOCUMENTED verdict is premature.
  **Status: watch-state** — the sketch's analysis identifies one
  obstruction; alternative resolutions are not exhaustively ruled
  out, but the §4.1 self-cancellation gives a strong sign that
  the obstruction is structural.
- **F-Sk-3** (lemma-existence drift): if Lemma 1 is reported, on
  the basis of this sketch, as having a specific α*_NS lower
  bound, the sketch has been mis-cited. **Status: not active** —
  this report explicitly produces no δ_0 and registers
  OBSTRUCTION_DOCUMENTED.
- **F-Sk-4** (atlas-touch): if this sketch leads to any
  atlas/state/inventory edit, the sketch has been mis-applied.
  **Status: not active** — sketch is research-direction only.
- **F-Sk-5** (auxiliary→main confusion): if the sketch's
  obstruction is reported as informing BT-544 directly, the
  auxiliary→main distinction has been lost. **Status: not active**.
- **F-Sk-6** (literature fabrication): if any cited author/year
  is found not to correspond to a published paper, the sketch's
  citation discipline has failed. **Status: not active** — all
  citations in §2, §7.2, §9 are by author/year/venue only,
  matching publicly known references; no quoted text or specific
  numerical claims attributed.

None of F-Sk-1..6 fires under this report's scope.

---

## §11 Closing

0/7 unchanged. NS regularity status open. No atlas/state/inventory
edits.

**Summary of the sketch attempt**:

- Lemma 1 statement recorded (§1).
- BV-2019 iteration recalled schematically (§2; no fabricated
  constants).
- Phase 3 Hölder-balance with viscous correction *attempted* (§3),
  identified as **dimensionally incoherent** in the schematic form
  prescribed by the parent dispatch (§3.2, log vs power-law
  mismatch).
- Phase 4 computation produced a **self-cancelling** equation
  (§4.1, equations (5')-(5'')) that contradicts the published
  BV-2019 result, indicating the schematic form is the wrong form
  rather than the lemma being refuted.
- **Verdict: OBSTRUCTION_DOCUMENTED** (§5). No δ_0 produced (§6).
- Obstruction step localized (§7.1: viscous correction term in
  inequality (3'')); three unblocking techniques recorded (§7.2:
  intermittency tracking, Hölder-class adaptation post-2021,
  ν → 0 contradiction).
- C3 sanity check: status **unchanged** by the sketch (§8.2).
- Anti-list of alternative iteration schemes (§9), none pursued
  in this sketch.
- Sketch-level falsifiers F-Sk-1..6 inactive (§10).

The sketch attempt confirms that the parent dispatch's Phase 3
schematic was *too coarse* to close Lemma 1 in one pass; the
correct adaptation requires intermittency-aware bookkeeping
(§7.2 #1) which is the next-session task. R1-Aux remains open;
α*_NS remains UNKNOWN; C3 remains a live conjecture; BT-544
remains 0/1 untouched; Clay status unchanged.

Per F-Disp-6 (parent §8) and F-Sk-5 (§10): this sketch does not
inform BT-544 directly. The auxiliary→main distinction is preserved.
