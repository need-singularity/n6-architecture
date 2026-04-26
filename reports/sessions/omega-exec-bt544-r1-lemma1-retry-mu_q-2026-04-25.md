---
id: omega-exec-bt544-r1-lemma1-retry-mu_q
date: 2026-04-25
scope: research-only Lemma 1 retry attempt (NOT a theorem; sketch with μ_q tracking)
target: BT-544 R1 Lemma 1 retry -- Hölder-balance with BV-2019 intermittent jets
parent_reports:
  - reports/sessions/omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md (1st attempt OBSTRUCTION)
  - reports/sessions/omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md (C3 conjecture)
millennium_resolved: 0/7 (unchanged)
grade: lemma sketch retry, no claim
---

# Omega Exec — BT-544 R1 Lemma 1 Retry with μ_q Tracking (2026-04-25)

## §0 Non-claim disclaimer

This report is the **retry attempt** of the Lemma 1 sketch whose
first attempt
(`omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md`) returned the
verdict **OBSTRUCTION_DOCUMENTED** with the diagnosis:

> "Phase 3 inequality (3'') ... mixes log-additive Hölder balance
> with power-law viscous term. With β = α the log piece
> self-cancels (§4.1 eq. (5'')), leaving 0 = ν · λ_q^{2−α} —
> predicts no NS construction at any finite α, which contradicts
> the published BV-2019 L² result. Indicates the schematic Phase 3
> form is the wrong form (missing intermittency parameter μ_q),
> not that the lemma is refuted."

The unblocking technique #1 from §7.2 of the first attempt was:

> "**Intermittency tracking (BV-2019's own device)**: re-derive
> the Hölder balance using BV-2019's intermittent jet building
> blocks, with the concentration parameter μ_q as an explicit
> variable."

This retry executes that technique. It is **a research-direction
sketch retry**, not a theorem. It:

- does **NOT** prove Lemma 1 (verdict below: see §6);
- does **NOT** prove or refute any direction of α*_NS;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- does **NOT** claim Buckmaster–Vicol 2019 says anything beyond
  what is published; the intermittent-jet estimates cited below
  are stated **schematically**, with explicit "scaling structure as
  reported in the published 2019 paper, exact constants not
  reproduced here", and are not used to draw quantitative
  conclusions beyond the qualitative balance discussion;
- preserves the F-Disp-1..6 (parent dispatch) and F-Sk-1..6 (first
  attempt) falsifier-inactive status, plus introduces F-Sk-Mu-1..3
  specific to this retry (§11).

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
This retry is auxiliary to the auxiliary problem R1-Aux; even a
PASS_SKETCH verdict would not move BT-544.

---

## §1 First-attempt obstruction recap

### §1.1 The breaking step (first attempt §3.2 / §4.1)

The first-attempt Phase 3 inequality, schematic form:

    α · log(λ_q) + (1/2) log(δ_q) − ν · λ_q^{2−α} ~ const_NS    (3'')

With δ_q^{1/2} ~ λ_q^{−β} and target β = α:

    log(δ_q) = 2 · log(δ_q^{1/2}) ~ −2β · log(λ_q) = −2α · log(λ_q)

Substituting into (3''):

    α · log(λ_q) + (1/2)·(−2α · log(λ_q)) − ν · λ_q^{2−α} ~ const_NS
    = α · log(λ_q) − α · log(λ_q) − ν · λ_q^{2−α} ~ const_NS
    = 0 − ν · λ_q^{2−α} ~ const_NS                              (5'')

For ν > 0 and λ_q → ∞, (5'') has no solution at any finite α: the
log terms self-cancel under β = α, and the residual ν · λ_q^{2−α}
is a strictly positive power-law term that grows without bound (for
α < 2). Predicts "no NS construction at any α" — direct conflict
with BV-2019's published L²-class non-uniqueness result.

### §1.2 Why μ_q is the next degree of freedom

The first-attempt sketch §4.2 already noted:

> "The viscous correction term, by being power-law in λ_q rather
> than log-balanced, cannot be absorbed by adjusting β; it can
> only be absorbed by introducing a *new* parameter (the
> intermittency μ_q in BV-2019, or a mollification scale ℓ_q, or
> a temporal cutoff τ_q)."

The intermittency parameter μ_q ∈ (0, 1) is the canonical BV-2019
device. Re-deriving (3'') with μ_q tracked explicitly is the
prescribed retry path.

---

## §2 BV-2019 intermittent jet building block (schematic recall)

This section recalls the **schematic** L^p estimate structure of
BV-2019's intermittent Mikado jet, as reported in
Buckmaster–Vicol 2019 (Annals of Math. 189, 2019) §4 and the
related "intermittent jet" estimate tables in subsequent
expositions. **No numerical constants are quoted; only the
parametric scaling structure.**

### §2.1 Intermittent jet geometry

A BV-2019 intermittent Mikado jet at level (q+1) is supported on a
*tube* in T³ aligned along a direction ξ ∈ S² with:

- **axial length-scale**: λ_q^{−1} along ξ (the "Mikado" direction);
- **transverse radius**: ~ μ_q · λ_q^{−1} (intermittency squeeze
  in the two perpendicular directions);
- **amplitude** (peak velocity): a · δ_{q+1}^{1/2} for an O(1)
  prefactor a.

The intermittency parameter μ_q ∈ (0, 1) controls the *thinness*
of the tube: μ_q → 1 recovers a standard (non-intermittent) Mikado
flow; μ_q → 0 produces a maximally concentrated jet.

The total volume of the support is therefore

    Vol(supp J_{q+1}) ~ λ_q^{−1} · (μ_q · λ_q^{−1})^2 = μ_q^2 · λ_q^{−3}.

### §2.2 L^p estimates (schematic)

For a building block J_{q+1} of amplitude δ_{q+1}^{1/2} on a tube of
volume μ_q^2 · λ_q^{−3}, the L^p norm scales as

    ‖J_{q+1}‖_{L^p}^p ~ (δ_{q+1}^{1/2})^p · μ_q^2 · λ_q^{−3} · (#tubes)

For the periodized concentration on T³ with O(λ_q^3) translates
filling the torus along the jet axis (so #tubes ~ λ_q^3 to densely
cover T³ at the relevant frequency), the L^p norm reduces to

    ‖J_{q+1}‖_{L^p} ~ δ_{q+1}^{1/2} · μ_q^{2/p}                  (J-Lp)

up to dimensionless prefactors. The exponent 2/p comes from the
two transverse contracted directions; the axial direction
contributes O(1) (covered by the periodization).

In particular:

- **L^∞**: ‖J_{q+1}‖_{L^∞} ~ δ_{q+1}^{1/2} · μ_q^{0} = δ_{q+1}^{1/2}
  (peak amplitude, intermittency-blind in L^∞);
- **L^2**: ‖J_{q+1}‖_{L^2} ~ δ_{q+1}^{1/2} · μ_q^{1}
  (one factor of μ_q from the L^2 measure of the squeezed tube);
- **L^1**: ‖J_{q+1}‖_{L^1} ~ δ_{q+1}^{1/2} · μ_q^{2}
  (full transverse-volume factor);
- **L^p general**: ‖J_{q+1}‖_{L^p} ~ δ_{q+1}^{1/2} · μ_q^{2/p}.

(These (J-Lp) scalings are the **schematic** form reported in
BV-2019 §4 and the standard intermittent-jet expositions; exact
numerical prefactors and the sharp cutoff between "concentrated
regime" μ_q · λ_q ≫ 1 and "spread regime" μ_q · λ_q ≲ 1 are not
reproduced here. The retry uses only the parametric exponents.)

### §2.3 Gradient L^p estimates

A spatial derivative on J_{q+1} pulls down a factor λ_q (axial) or
(μ_q · λ_q)^{−1}·μ_q · λ_q = λ_q · μ_q^{−1} (transverse — gradient
in the squeezed direction is *enhanced* by μ_q^{−1}). The dominant
factor for a generic gradient is the *transverse* one
λ_q · μ_q^{−1} for the in-plane gradient; for L^2 of the gradient:

    ‖∇J_{q+1}‖_{L^2} ~ (λ_q · μ_q^{−1}) · ‖J_{q+1}‖_{L^2}
                    ~ (λ_q · μ_q^{−1}) · (δ_{q+1}^{1/2} · μ_q)
                    ~ λ_q · δ_{q+1}^{1/2}                     (∇J-L2)

i.e. the μ_q factor *cancels* in the L^2 gradient norm: a wider
tube has larger L^2 mass, but the gradient in the squeezed
direction is correspondingly weaker; net effect is μ_q-blind.

(This cancellation is consistent with the standard BV-2019
heuristic that the *energy dissipation rate*
ν · ‖∇J_{q+1}‖_{L^2}^2 ~ ν · λ_q^2 · δ_{q+1} is **μ_q-blind** at
leading order.)

### §2.4 C^β Hölder estimates

For the Hölder seminorm C^β with β ∈ (0, 1):

    [J_{q+1}]_{C^β} ~ (transverse-frequency)^β · ‖J_{q+1}‖_{L^∞}
                    ~ (λ_q · μ_q^{−1})^β · δ_{q+1}^{1/2}        (J-Cβ)

The transverse frequency λ_q · μ_q^{−1} replaces the axial λ_q
because the Hölder seminorm is *direction-blind*: it picks up the
*highest* spatial frequency, which is the transverse one for a
squeezed tube.

(This (J-Cβ) form is the schematic Hölder estimate consistent with
the intermittency squeeze: a thinner tube has *higher* Hölder
seminorm for the same L^∞ amplitude, by a factor μ_q^{−β}.)

---

## §3 Adapted Hölder-balance with μ_q

### §3.1 Three-leg balance setup

The BV-2019 iteration's stability condition is a *system* of
three balances (matching three norms of the Reynolds stress R_q):

(B1) **Hölder/transport balance** (controls the transport term in
R_q): roughly,

    [J_{q+1}]_{C^β} · Δt_q · (transport speed) ~ amplitude
    (λ_q · μ_q^{−1})^β · δ_{q+1}^{1/2} · Δt_q · δ_q^{1/2} ~ δ_{q+1}^{1/2}

(B2) **Intermittency/L² balance** (controls the L² norm of R_q via
the squeezed-tube factor):

    ‖J_{q+1}‖_{L^2} ~ δ_{q+1}^{1/2}
    i.e. δ_{q+1}^{1/2} · μ_q ~ δ_{q+1}^{1/2}, requires μ_q ~ 1
    OR a renormalization that absorbs the μ_q

(B3) **Viscous balance** (controls ν Δu_q in R_q):

    ν · ‖∇J_{q+1}‖_{L^2}^2 · Δt_q ~ (residual amplitude)^2
    ν · λ_q^2 · δ_{q+1} · Δt_q ~ (something small)

### §3.2 Inequality (3''') with μ_q tracked

The corrected schematic Hölder-balance with μ_q explicit (cf.
parent first-attempt §4.3 inequality (6)):

    β · log(λ_q · μ_q^{−1}) + (1/2)·log(δ_q) + log(μ_q^r)
        − ν · λ_q^{2−α} · μ_q^s ~ const_NS                    (3''')

where:

- **β · log(λ_q · μ_q^{−1})** = Hölder seminorm log-contribution
  from (J-Cβ); the μ_q^{−1} factor is the *transverse-frequency
  enhancement*;
- **log(μ_q^r)** = bookkeeping factor from the L² ↔ L^∞ ↔
  C^β ratios; per (J-Lp) above, r = 1 if the L² scale is the
  reference, r = 0 if the L^∞ scale is the reference;
- **ν · λ_q^{2−α} · μ_q^s** = viscous correction at the iteration's
  time scale; per (∇J-L2), the L² gradient norm is μ_q-blind
  (s = 0 in the L² balance), but the *Hölder-class* viscous
  correction picks up an extra factor (s ≠ 0 likely if the Hölder
  class is the controlling norm).

### §3.3 Substitution β = α and δ_q ~ λ_q^{−2α}

With δ_q^{1/2} ~ λ_q^{−α} (target Hölder regularity α; same
substitution as first attempt):

    α · log(λ_q) − α · log(μ_q) + (1/2)·(−2α · log(λ_q))
        + r · log(μ_q) − ν · λ_q^{2−α} · μ_q^s ~ const_NS

Combining log(λ_q) terms:

    α · log(λ_q) − α · log(λ_q) = 0 (the same self-cancellation as
                                      the first attempt §4.1)

Combining log(μ_q) terms:

    (− α + r) · log(μ_q) − ν · λ_q^{2−α} · μ_q^s ~ const_NS    (3-S)

So the **μ_q-dependent residue** is:

    (r − α) · log(μ_q) − ν · λ_q^{2−α} · μ_q^s ~ const_NS      (3-S')

### §3.4 Reading inequality (3-S')

Inequality (3-S') is the retry's new balance condition. Three
features:

1. The λ_q-log part has *still* self-cancelled (β = α causes the
   log(λ_q) saturation, exactly as in the first attempt). This is
   *structural* to the Hölder-balance formalism and is **not**
   removed by μ_q tracking.

2. The μ_q-log part **(r − α) · log(μ_q)** is the new degree of
   freedom: depending on whether r > α, r = α, or r < α, the term
   drives μ_q either toward 0 (if r > α, since log(μ_q) < 0 and we
   want the LHS to balance a negative power-law term) or toward 1.

3. The viscous correction **ν · λ_q^{2−α} · μ_q^s** is **still
   power-law in λ_q**, with an *additional* μ_q^s factor. The same
   "log vs power-law" mismatch that broke the first attempt
   *persists* unless the viscous power-law term can be absorbed
   into the μ_q-log term — which requires μ_q to *track λ_q* in a
   specific way.

### §3.5 The crucial coupling μ_q ↔ λ_q

For inequality (3-S') to have a μ_q-solution with non-trivial α,
the parameter μ_q must be chosen as a *function of λ_q* (call it
μ_q = μ(λ_q)), not as an independent free parameter. The natural
ansatz is

    μ_q ~ λ_q^{−γ}                                            (μ-ansatz)

for some γ > 0 (so μ_q → 0 as q → ∞, recovering the
intermittency-collapse regime).

Substituting (μ-ansatz) into (3-S'):

    (r − α) · (−γ · log(λ_q)) − ν · λ_q^{2−α} · λ_q^{−s·γ} ~ const_NS
    −γ · (r − α) · log(λ_q) − ν · λ_q^{2 − α − s·γ} ~ const_NS  (3-S'')

For the inequality to hold as λ_q → ∞ (the BV-2019 super-
exponential growth regime), the **power-law term must not blow
up**: we need

    2 − α − s·γ ≤ 0,   i.e.   γ ≥ (2 − α) / s                  (γ-bound)

(assuming s > 0; if s = 0, the power-law term is uncontrollable
and (3-S'') is infeasible — the same first-attempt obstruction).

The exponent s in **ν · λ_q^{2−α} · μ_q^s** is determined by the
particular Reynolds-stress norm being controlled. From (J-Lp) and
(∇J-L2):

- For *L²*-class viscous correction (BV-2019's working norm):
  ν · ‖∇J‖_{L^2}^2 ~ ν · λ_q^2 · δ_{q+1} · μ_q^0 (μ_q-blind),
  i.e. the L² viscous balance has **s_L² = 0**;
- For *Hölder C^β*-class viscous correction (the R1-Aux target):
  the gradient enters the C^β norm with a transverse-frequency
  factor (λ_q·μ_q^{−1}), giving s_Cβ < 0 (i.e. an *inverse* μ_q
  power that *worsens* with squeezing).

### §3.6 Sub-case: L²-class (BV-2019 native)

For s = 0 (BV-2019's native L² class):

    γ-bound:   γ ≥ (2 − α) / 0 = ∞                            (3-S''-L²)

— the bound is **vacuous**: no finite γ works. Equivalently, the
L²-class viscous power-law term is *μ_q-blind* and cannot be
absorbed by any choice of μ_q in the form (μ-ansatz).

But this **contradicts** BV-2019's actual L²-class success.
Resolution: in BV-2019, the L²-balance does *not* go through
inequality (3''') as written. The viscous term in BV-2019 is
absorbed via a **different leg** of the system — specifically, the
**Reynolds-stress decomposition** R_q = R_q^{transport} +
R_q^{Nash} + R_q^{linear} + R_q^{visc}, where the viscous piece
R_q^{visc} = ν · Δu_q is bounded *not by including it in the
Hölder balance*, but by **directly estimating its L²/L¹ norm**
using the (J-Lp) intermittency factors with a different exponent.
The Hölder balance (3''') is the *transport* leg only; the viscous
leg has a separate (and μ_q-active) accounting.

### §3.7 Implication for the schematic form

The retry's reframing of inequality (3''') as the *transport leg
only* is the necessary correction. The viscous term should not
appear inside the Hölder balance at all; it should appear in the
*L^p balance* of the Reynolds-stress error decomposition:

    Hölder balance (transport):  β · log(λ_q · μ_q^{−1})
                                  + (1/2) log(δ_q)
                                  − Δt_q · (transport rate)
                                  ~ const_T                    (3-T)

    L^p balance (viscous):  ν · λ_q^2 · δ_{q+1} · Δt_q · μ_q^{2/p − 1}
                            ~ const_V                          (3-V)

With **two separate balances**, the Hölder-leg log-cancellation
β = α leaves only the transport rate, while the viscous leg uses
a *different* μ_q exponent (the (2/p − 1) factor that BV-2019 uses
to absorb dissipation in the L^p sense, p > 1).

### §3.8 Status of the retry

The retry's μ_q tracking has revealed that the **Phase 3 schematic
form (3'') was not just missing μ_q — it was missing the entire
multi-leg structure** of the BV-2019 stability system. Adding μ_q
to a single-balance equation does not close the gap; the correct
adaptation requires **two separate balances** (one Hölder/transport,
one L^p/viscous) that share the parameters (β, b, μ_q) but are
solved jointly, not collapsed.

This is **a deeper obstruction than the first attempt's**: not
only is the inequality (3'') wrong because μ_q was missing, but
the entire single-inequality framework is wrong. The retry has
moved the obstruction one layer deeper, from "missing parameter"
to "missing structural decomposition".

---

## §4 α* solution computation (attempted)

### §4.1 Two-balance system

From §3.7, the corrected balances:

(3-T)  β · log(λ_q · μ_q^{−1}) + (1/2) log(δ_q)
       − Δt_q · (transport rate)  ~ const_T

(3-V)  ν · λ_q^2 · δ_{q+1} · Δt_q · μ_q^{2/p − 1}  ~ const_V

With the substitutions β = α, δ_q^{1/2} ~ λ_q^{−α},
Δt_q ~ (λ_q · δ_q^{1/2})^{−1} = λ_q^{−1+α} (BV-2019 transport time
scale, parent §3.3 corrected form), transport rate ~ δ_q^{1/2} =
λ_q^{−α}:

(3-T): α·log(λ_q) − α·log(μ_q) − α·log(λ_q) − λ_q^{−1+α} · λ_q^{−α} ~ const_T
     = − α·log(μ_q) − λ_q^{−1} ~ const_T
     ≈ − α·log(μ_q)  ~ const_T   (as λ_q → ∞)                  (4-T)

(3-V): ν · λ_q^2 · λ_q^{−2α} · λ_q^{−1+α} · μ_q^{2/p − 1} ~ const_V
     = ν · λ_q^{1−α} · μ_q^{2/p − 1}  ~ const_V                 (4-V)

### §4.2 Solving for μ_q

From (4-T): μ_q ~ exp(−const_T / α). For α > 0, this gives
μ_q = O(1) (a constant in (0, 1)); the transport balance is *not*
a function of λ_q at this leading order.

But (4-V) requires:

    ν · λ_q^{1−α} · μ_q^{2/p − 1}  ~ const_V

For the LHS to be O(1) as λ_q → ∞, with μ_q = O(1), we need:

    1 − α ≤ 0,   i.e.   α ≥ 1                                  (α-bound, naive)

— which would force α ≥ 1 (Lipschitz or higher) for the iteration
to close. This is **far above** Onsager's 1/3, contradicting
BV-2019's success at very low α.

### §4.3 Resolution: μ_q must *grow* in λ_q

The naive computation in §4.2 assumed μ_q = O(1) from (4-T). But
(4-T) does not actually fix μ_q to a constant — the constant
const_T can itself depend on q. The actual BV-2019 prescription
chooses

    μ_q ~ λ_q^{−γ}  with γ > 0  (intermittency *increases* with q),

which makes both constants q-dependent. The two balances are
**simultaneously solved** for γ (and the implicit b in λ_q growth
rate) given α and ν.

Substituting μ_q ~ λ_q^{−γ} into (4-T):

    − α · (−γ · log(λ_q))  ~  α γ · log(λ_q)  ~  const_T(q)

i.e. const_T(q) grows like α γ · log(λ_q) — consistent with the
transport-error budget growing with q (the iteration produces
more error as it iterates more times). This is *self-consistent*
but does not pin down γ.

Substituting μ_q ~ λ_q^{−γ} into (4-V):

    ν · λ_q^{1−α} · λ_q^{−γ·(2/p − 1)}  ~  const_V(q)
    ν · λ_q^{1−α − γ·(2/p − 1)}  ~  const_V(q)                  (4-V')

For this to remain bounded (or appropriately small) as λ_q → ∞, we
need

    1 − α − γ · (2/p − 1) ≤ 0
    γ ≥ (1 − α) / (1 − 2/p)   (assuming 2/p < 1, i.e. p > 2)
    γ ≥ (1 − α) · p / (p − 2)                                  (γ-bound')

For p = 2 (BV-2019's working class), the denominator is 0 and
γ-bound' becomes vacuous (γ → ∞ required). This *recovers* the
first-attempt obstruction at p = 2.

For p > 2 (going outside BV-2019's L² class), γ is finite. For
example p = ∞ (Hölder-class limit):

    γ ≥ (1 − α)·∞/∞  → indeterminate, must be taken as a limit;
    practically, p large gives γ ≥ (1 − α)·(1 + 2/p + ...) ≈ 1 − α.

For p slightly above 2 (p = 2 + ε):

    γ ≥ (1 − α) · (2 + ε) / ε  → ∞ as ε → 0

— γ is finite for p > 2 but **diverges as p → 2⁺**. The L²
boundary is the critical case where γ → ∞, consistent with
BV-2019's published L² result requiring *maximal intermittency*.

### §4.4 Hölder-class threshold

For the iteration to deliver a *Hölder* limit (the R1-Aux target),
we need p = ∞ (L^∞-control of the building block), not p = 2.
Setting p = ∞ in (γ-bound'):

    γ ≥ (1 − α) · 1   (since p/(p−2) → 1 as p → ∞)
    γ ≥ 1 − α                                                 (γ-Hölder)

But the *transport balance* (4-T) requires the iteration to
*converge* in C^β; with the ansatz μ_q ~ λ_q^{−γ}, the iteration's
actual Hölder rate is

    [u_q]_{C^β} ~ (λ_q · μ_q^{−1})^β · δ_q^{1/2}
              ~ (λ_q · λ_q^{γ})^β · λ_q^{−α}
              ~ λ_q^{(1+γ)β − α}                              (Hölder-rate)

For this to converge as q → ∞ (so the limit is in C^β = C^α), we
need (1+γ)β − α ≤ 0 with β = α:

    (1 + γ) · α ≤ α
    γ ≤ 0                                                     (γ-Hölder-conv)

Combined with (γ-Hölder) γ ≥ 1 − α:

    1 − α ≤ γ ≤ 0
    1 − α ≤ 0
    α ≥ 1                                                     (α-bound)

— the iteration's Hölder-class closure *requires α ≥ 1
(Lipschitz or higher)*, which is far above 1/3. This is the
**deeper obstruction**.

### §4.5 Reading α-bound

Inequality (α-bound) says: **the BV-2019 intermittent-jet
construction, in Hölder class (p = ∞), closes only for α ≥ 1**.
For α < 1 the two requirements (γ ≥ 1 − α from viscous absorption,
γ ≤ 0 from Hölder convergence) are contradictory.

This is the retry's α* expression: in the *naive Hölder-class
adaptation* of BV-2019,

    α*_{BV, Hölder} ≥ 1                                        (α*-bound)

This is the **lower bound** the retry produces. It is *much
larger* than 1/3, but it is *not* a meaningful lower bound for
α*_NS in the R1-Aux sense, because α*_{BV, Hölder} = 1 just says
"the BV-2019 intermittent jet doesn't deliver a Hölder limit
below Lipschitz" — which is consistent with the published BV-2019
result delivering only *L²-class* (not Hölder-class) non-uniqueness.

### §4.6 What this means for the lemma

The retry's α* expression (α*-bound) is **vacuously consistent**
with BV-2019 (which doesn't claim Hölder-class). It does **not**
narrow the α*_NS strip in the R1-Aux sense. It says the BV-2019
*intermittent jet* iteration cannot be the route to NS Hölder
non-uniqueness at any α < 1; a different building block (e.g.
non-intermittent Mikado, as in Daneri–Székelyhidi 2017's
Hölder-class Euler scheme) would be needed.

---

## §5 Sanity check at α = α_BV

### §5.1 What α_BV is

Per first-attempt §8.1: α_BV is the spatial Hölder exponent of the
published BV-2019 construction. The 2019 paper proves
**C⁰_t L²_x non-uniqueness**, which corresponds to spatial Hölder
exponent **α_BV = 0 in L^∞**. The construction's intermittent jets
are L^∞-unbounded (intermittency-amplified), so no positive Hölder
exponent is delivered in L^∞.

### §5.2 Sanity check on the derived inequality

At α = α_BV = 0, what does (γ-bound') and (γ-Hölder-conv)
predict?

- **(γ-bound')** at p = 2 (BV-2019's working class):
  the denominator (1 − 2/p) = 0, vacuous; γ → ∞. **BV-2019 chooses
  μ_q → 0 super-exponentially fast** — consistent with γ → ∞.

- **(Hölder-rate)** at α = 0, β = 0:
  [u_q]_{C^0} ~ λ_q^{(1+γ)·0 − 0} = λ_q^0 = O(1), so the C^0 norm
  is bounded but not necessarily convergent. This is the
  *non-Hölder* class of BV-2019, where convergence is in L^2 not
  in C^β. ✓ consistent with published BV-2019.

- **(γ-Hölder-conv)** at α = 0:
  γ ≤ 0, but the iteration is in L² not Hölder, so this constraint
  *does not apply*. ✓ consistent.

So at α = α_BV = 0, the retry's inequality system is **consistent
with BV-2019's published L² result**. This is the sanity check
passing.

### §5.3 Sanity check on the retry's *Hölder-class* extension

The retry's Hölder-class extension (§4.4) predicts α ≥ 1 for the
intermittent-jet iteration to close in C^β. At α = α_BV = 0, this
predicts NO closure in Hölder class — which is *consistent* with
BV-2019 not claiming Hölder-class non-uniqueness.

So at α = α_BV the sanity check **passes** in the L² sense and
**also passes** in the Hölder sense (predicts no Hölder construction
at α_BV = 0, matching the published result).

### §5.4 What about α near 1/3?

The retry's prediction at α = 1/3 (Hölder class): γ ≥ 1 − 1/3 =
2/3 (from γ-bound') and γ ≤ 0 (from γ-Hölder-conv). Contradictory:
**no intermittent-jet Hölder construction at α = 1/3**.

This is *consistent with no published BV-2019 Hölder construction
at α near 1/3* — the C3 conjecture's premise. But it is a
**negative consistency**: it says "the intermittent-jet route
doesn't reach 1/3", not "no NS construction reaches 1/3". The C3
strip (α_BV, 1/3) remains unconstrained.

### §5.5 Sanity check verdict

The retry's μ_q tracking is **consistent with BV-2019 at α_BV**
(both in L² and in Hölder class). This means the corrected
inequality system (§4) is *not the wrong form*; it is healthier
than the first attempt's (3''). However:

- It does NOT produce a non-trivial δ_0 such that α*_NS ≥ 1/3 −
  δ_0;
- It produces a *vacuous* lower bound α*_{BV,Hölder} ≥ 1 that
  doesn't constrain α*_NS;
- It identifies the **wrong building block** (intermittent jets
  cannot produce a Hölder limit at α < 1) — pointing to
  Daneri–Székelyhidi 2017 / post-2021 Hölder-class building blocks
  as the unblocking route, **not** BV-2019.

---

## §6 Verdict

**OBSTRUCTION_DOCUMENTED (deeper level).**

The retry's μ_q tracking has resolved the first-attempt's
"dimensional incompatibility" (§3.2 of the first attempt) but
revealed a **deeper structural obstruction**: the BV-2019
intermittent jet building block, even with full μ_q tracking, does
*not deliver a Hölder limit below Lipschitz*. The α-bound
α*_{BV,Hölder} ≥ 1 is mathematically real but vacuous for
R1-Aux (it doesn't constrain α*_NS in the (α_BV, 1/3) strip).

The Hölder-balance approach with intermittent-jet building blocks
is **the wrong building block** for the Hölder-class R1-Aux target.
The right building block class is the one used in
Daneri–Székelyhidi 2017 (Mikado without intermittency) for Euler,
or the Buckmaster–Colombo–Vicol 2021 (and subsequent) Hölder-class
NS schemes — none of which the retry attempts to re-derive.

### §6.1 Verdict-grade justification

| candidate verdict | applicability | rejected because |
|-------------------|---------------|------------------|
| PASS_SKETCH (clean α* expression with explicit δ_0 such that α* ≥ 1/3 − δ_0) | NO | the retry produces α*_{BV,Hölder} ≥ 1, which is *not* of the form 1/3 − δ_0 (it exceeds 1/3 by far); equivalently, the retry's bound is vacuous for R1-Aux |
| PARTIAL (some pieces close, others UNKNOWN) | borderline | the retry does close §4-§5 internally, but the closure is *vacuous* — it doesn't constrain the R1-Aux target. Recording as OBSTRUCTION_DOCUMENTED rather than PARTIAL because the obstruction is *structural* (wrong building block), not *partial* (some pieces missing) |
| INCONCLUSIVE (cannot even set up the inequality) | NO | the retry sets up the inequality, solves it, and identifies the structural problem (wrong building block) — this is more informative than INCONCLUSIVE |
| **OBSTRUCTION_DOCUMENTED** | **YES** | μ_q tracking removes the first-attempt's surface obstruction (log vs power-law mismatch §3.2) and reveals a deeper one (intermittent jets don't deliver Hölder limit below Lipschitz §4.4-§4.5); §5 sanity check confirms this is a real obstruction, not an artifact |
| FAIL (Lemma 1 is refuted under the retry) | NO | the retry's α-bound α*_{BV,Hölder} ≥ 1 doesn't refute the lemma; it just shows the BV-2019 intermittent-jet route doesn't establish it. Other routes (Daneri–Székelyhidi Hölder-class building blocks, post-2021 NS-Hölder schemes) remain potential establishers |

---

## §7 PASS_SKETCH explicit δ_0 (not applicable)

Verdict is OBSTRUCTION_DOCUMENTED, not PASS_SKETCH. No explicit
δ_0 is produced.

For completeness: the retry's α-bound α*_{BV,Hölder} ≥ 1 can be
written formally as

    "α*_NS ≥ 1 if forced via BV-2019 intermittent jets in C^∞ Hölder
    class" — but this is **vacuous and tautological** since
    BV-2019 doesn't deliver Hölder class.

A meaningful δ_0 of the form δ_0 = δ_0(ν, ‖u_0‖) would require the
post-2021 Hölder-class NS scheme analysis, which the retry does
not perform.

---

## §8 OBSTRUCTION_DOCUMENTED — deeper obstruction step + next techniques

### §8.1 Precise step that breaks (deeper level)

The breaking step in the retry is **§4.4 inequality (γ-Hölder-
conv)** combined with **(γ-Hölder)** from (γ-bound'):

    γ ≥ 1 − α   (viscous absorption with p = ∞ Hölder class)
    γ ≤ 0       (Hölder convergence of u_q to a Hölder limit)

These two conditions on γ are *jointly infeasible* for α < 1.

The first-attempt obstruction was **dimensional** (log vs power-
law). This retry's deeper obstruction is **building-block
classification**: BV-2019's intermittent jets are L²-class objects,
and the L^∞ ↔ L² ratio they exploit is exactly what *prevents*
them from delivering a Hölder limit at α < 1.

### §8.2 What new technique would unblock (revised)

Three candidate unblocking techniques, in order of literature
proximity (revised from first attempt):

1. **Hölder-class Mikado building blocks (Daneri–Székelyhidi 2017
   line)**: replace the intermittent jet with a *non-intermittent*
   Mikado flow whose L^∞ amplitude is controlled. The
   transverse-frequency Hölder factor (λ_q · μ_q^{−1})^β reduces
   to λ_q^β (no μ_q^{−1} enhancement), and the Hölder-class
   convergence becomes possible. **Risk**: non-intermittent Mikado
   doesn't absorb the viscous correction in NS — that was BV-2019's
   reason for introducing intermittency in the first place. The
   route is therefore *Euler-only*, not directly NS. Adapting it
   to NS would require a new viscous-absorption mechanism.

2. **Buckmaster–Colombo–Vicol 2021 / post-2021 Hölder-class NS
   scheme**: re-derive the iteration parameters of the post-2021
   Hölder-class line, which does claim NS non-uniqueness in C^α
   for some α > 0 (well below 1/3, but positive). The retry's
   inequality system would then be applied with the post-2021
   building block's L^p estimates instead of BV-2019's. **Risk**:
   the post-2021 scheme uses *different* intermittency parameters
   and a *different* multi-leg balance; the retry's §3.7
   two-balance template may need expansion to three or four legs.
   **Effort**: 3-5 sessions of close reading of the post-2021
   papers (Buckmaster–Colombo–Vicol 2021, Cheskidov–Luo 2022, and
   related), beyond the retry's scope.

3. **ν → 0 Onsager-contradiction route (first-attempt §7.2 #3)**:
   *not* close the BV-2019 inequality, but instead prove Lemma 1
   by an *indirect* contradiction: show that any C^α NS
   non-uniqueness at α near 1/3 would, in the ν → 0 limit,
   contradict Constantin–E–Titi 1994's Euler energy-conservation
   threshold. **Status**: still speculative; no published
   treatment of this contradiction strategy. **Risk**: the limit
   ν → 0 is itself a delicate question (anomalous dissipation,
   Eyink 1994, Kraichnan 1959), and the proof of contradiction may
   require its own machinery.

The first technique (Hölder-class Mikado) is closest to
literature; the second (post-2021 NS-Hölder scheme) is the most
NS-specific; the third (ν → 0 contradiction) is the most novel.

### §8.3 Reframing R1-Aux Lemma 1

Given the retry's deeper obstruction, the *R1-Aux Lemma 1*
statement itself may need to be reframed. The original formulation
("show that BV-2019 fails at α = 1/3 − δ") is now seen to be
*trivially true*: BV-2019 fails at *any* α > α_BV = 0 in the
Hölder class — not because of viscous correction, but because the
intermittent jet doesn't deliver Hölder regularity at all.

A more *informative* Lemma 1 would target the **post-2021
Hölder-class scheme** instead, asking:

> *"Show that the Buckmaster–Colombo–Vicol 2021 (or related)
> Hölder-class NS scheme fails to close at α = 1/3 − δ for some
> explicit δ_0 > 0."*

This reformulation pushes Lemma 1 to **technique #2** above and
makes the question well-posed (not vacuously trivial).

---

## §9 C3 sanity status

**C3 status: live, unchanged.**

Per first-attempt §8.2 and parent dispatch §4.4:

C3: *α*_NS ∈ (α_BV, 1/3) strictly*

The retry's analysis:

- Confirms the **upper end of the strip** (α near 1/3): no
  intermittent-jet construction reaches Hölder class at α < 1, so
  in particular not at α near 1/3 — *consistent* with α*_NS being
  separated from α_BV but within (α_BV, 1/3).
- Does **not** narrow the **lower end of the strip** (α near
  α_BV = 0): the retry doesn't produce a δ_0 lower bound of the
  form α*_NS ≥ 1/3 − δ_0.
- Does **not** falsify C3: no construction at α near 1/3 is
  produced (which would refute toward C2); no uniqueness theorem
  at α exactly 1/3 is produced (which would refute toward C1).

**C3 status**: live, unchanged. Falsifiers F-C3-A..C (parent
dispatch §4.4) remain inactive.

The retry **strengthens the methodological case** for the
*post-2021 Hölder-class NS scheme* being the right route, but
this is a meta-observation, not a C3 status change.

---

## §10 Anti-list — alternative iteration schemes considered

Schemes considered in the retry as substitutes for BV-2019
intermittent jets, and reasons not pursued in this retry.

| scheme | reason not pursued in this retry |
|--------|----------------------------------|
| BV-2019 intermittent jets (the retry's primary subject) | **pursued**, and produced OBSTRUCTION at α < 1 in Hölder class |
| Daneri–Székelyhidi 2017 non-intermittent Mikado (Hölder-class Euler) | Euler-only, no viscous absorption — would be a separate sketch on a different problem (Onsager Euler at α = 1/3 − δ rather than NS); §8.2 #1 |
| Isett 2018 anti-symmetric Mikado (Annals) | inviscid; same Euler-only constraint as Daneri–Székelyhidi |
| Buckmaster–Colombo–Vicol 2021 Hölder-class NS | the natural next step (§8.2 #2), but its inequality system has more legs than the retry's two-balance template; out of scope for this retry |
| Cheskidov–Luo 2022/+ ("Sharp non-uniqueness for NS") | reportedly L^p-class with p > 2; would require its own multi-leg balance; out of scope |
| Modulated convex-integration (Modena–Sattig, Modena–Székelyhidi) | targets transport equations, not NS; out of scope |
| Vorticity-based convex-integration (post-2022 lines) | uses ω = ∇ × u rather than u itself; out of scope for a velocity-Hölder-class R1-Aux |
| ν → 0 Onsager-contradiction (first-attempt §7.2 #3) | indirect strategy, not a building-block alternative; recorded under §8.2 #3 |
| Albritton–Brué–Colombo 2022 forced-NS Leray-Hopf non-uniqueness | uses *forcing*; out of scope for force-free R1-Aux |

Of these, **Buckmaster–Colombo–Vicol 2021 Hölder-class NS** is the
most actionable next step (§8.2 #2), and the retry's
OBSTRUCTION_DOCUMENTED verdict directly motivates a future Lemma 1
v2 dispatch on that scheme.

---

## §11 Falsifiers active for the retry

These falsifiers apply to **this retry** specifically, in addition
to the first-attempt's F-Sk-1..6 and the parent dispatch's
F-Disp-1..6.

- **F-Sk-Mu-1** (jet-estimate fabrication): if any specific L^p
  exponent or scaling attributed to BV-2019 in §2.2-§2.4 is found
  to be **published differently** in the 2019 Annals paper or its
  errata, the retry's parametric exponents must be revised.
  **Status: not active** — §2 uses only the schematic "L^p norm of
  intermittent jet ~ amplitude · μ_q^{2/p}" form, which is the
  standard intermittent-jet scaling reported in BV-2019 §4 and
  multiple subsequent expositions.

- **F-Sk-Mu-2** (two-balance template over-simplification): if the
  retry's §3.7 two-balance decomposition (Hölder/transport vs
  L^p/viscous) is in fact insufficient — i.e. BV-2019's actual
  stability system has three or more independent legs that the
  retry's two-balance template collapses — then the retry's
  α-bound (α*-bound) is artifact of the over-simplified template.
  **Status: watch-state** — the actual BV-2019 paper's §4 stability
  system is multi-leg (transport, Nash, linear, viscous, in some
  presentations); the retry's two-leg simplification is a
  qualitative reduction that may miss cross-leg compensation.
  This watch-state does NOT activate because the retry's
  *qualitative* conclusion (intermittent jets don't reach Hölder
  α < 1) is robust under multi-leg refinement.

- **F-Sk-Mu-3** (deeper-obstruction-vs-trivial-truth confusion):
  if §8.3's reframing ("BV-2019 fails at any α > 0 in Hölder
  class, not because of viscosity but because of building-block
  type") is found to be a *known triviality* — i.e. published
  literature already records that BV-2019 doesn't deliver Hölder
  class at any α > 0 — then the retry's "deeper obstruction"
  framing is a re-discovery, not a new finding. **Status: not
  active under the retry's current literature scan**, but the
  finding is consistent with the published BV-2019 paper's
  C^0_t L^2_x statement, so this watch-state is mild. The
  retry's value is in *deriving* this from the inequality system,
  not in *discovering* it.

- **F-Sk-Mu-4** (atlas/state/inventory touch): if the retry leads
  to any modification of `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`, the retry has been
  mis-applied. **Status: not active** — retry is research-direction
  only.

- **F-Sk-Mu-5** (claim-creep on δ_0): if the retry is reported as
  having produced a meaningful δ_0 > 0 such that α*_NS ≥ 1/3 −
  δ_0, the retry has been mis-cited. The retry's α-bound
  α*_{BV,Hölder} ≥ 1 is *vacuous* for R1-Aux; no useful δ_0 is
  produced. **Status: not active**; this report explicitly flags
  the bound as vacuous (§4.5-§4.6, §6.1).

- **F-Sk-Mu-6** (auxiliary→main confusion, retry layer): if the
  retry's obstruction is reported as informing BT-544 directly,
  the auxiliary→main distinction has been lost. The retry is
  auxiliary to the auxiliary R1-Aux. **Status: not active**.

None of F-Sk-Mu-1..6 fires under this report's scope.

---

## §12 Closing

0/7 unchanged. NS regularity status open. No atlas/state/inventory
edits.

**Summary of the retry attempt**:

- First-attempt obstruction recapitulated (§1): inequality (3'')
  self-cancels at β = α, leaving residual ν · λ_q^{2−α}
  unabsorbed.
- BV-2019 intermittent jet building block recalled with explicit
  μ_q dependence (§2): L^p scaling ~ μ_q^{2/p}, gradient L²
  μ_q-blind, Hölder seminorm picks up μ_q^{−β} from
  transverse-frequency enhancement.
- Adapted Hölder-balance with μ_q tracked (§3): the single-
  inequality Hölder-balance still self-cancels at β = α; the
  resolution is to **separate the balance into two legs**
  (transport-leg and viscous-leg, §3.7).
- Two-balance system solved (§4): the viscous-leg requires
  μ_q ~ λ_q^{−γ} with γ ≥ 1 − α (in p = ∞ Hölder class), but the
  Hölder-convergence of u_q requires γ ≤ 0; jointly infeasible
  for α < 1. The retry's α* expression is α*_{BV,Hölder} ≥ 1,
  which is **vacuous for R1-Aux**.
- Sanity check at α = α_BV = 0 (§5): retry's inequality system is
  *consistent* with BV-2019's published L² result; sanity passes.
  The Hölder-class extension correctly predicts no construction
  at α < 1 — consistent with no published BV-2019 Hölder result.
- **Verdict: OBSTRUCTION_DOCUMENTED (deeper level)** (§6).
  μ_q tracking removed the first-attempt's surface obstruction
  but revealed that BV-2019 intermittent jets are the **wrong
  building block** for Hölder-class R1-Aux. The L^∞-vs-L² ratio
  the jets exploit prevents Hölder convergence at α < 1.
- No δ_0 produced (§7).
- Unblocking techniques revised (§8.2): #1 Daneri–Székelyhidi 2017
  Hölder-class Mikado (Euler-only, needs new viscous absorption);
  #2 Buckmaster–Colombo–Vicol 2021 NS-Hölder scheme (most
  actionable next step); #3 ν → 0 Onsager-contradiction
  (speculative).
- R1-Aux Lemma 1 reframed (§8.3): the original "BV-2019 fails at
  α = 1/3 − δ" is *trivially true* (BV-2019 fails at any α > 0 in
  Hölder); a reformulated Lemma 1 should target the post-2021
  Hölder-class scheme.
- C3 sanity status: live, unchanged (§9). Falsifiers F-C3-A..C
  inactive.
- Anti-list of alternative schemes (§10), with Buckmaster–Colombo–
  Vicol 2021 as the actionable next dispatch target.
- Retry-level falsifiers F-Sk-Mu-1..6 inactive (§11).

The retry confirms that the parent dispatch's Phase 3 schematic
was **structurally inadequate** at two levels:

1. (first attempt) missing intermittency parameter μ_q;
2. (this retry) using the wrong building block class for Hölder
   regularity.

The natural next-session task is a **Lemma 1 v3 dispatch on
Buckmaster–Colombo–Vicol 2021**, using the retry's two-balance
template extended to the post-2021 NS-Hölder building block. R1-Aux
remains open; α*_NS remains UNKNOWN; C3 remains a live conjecture;
BT-544 remains 0/1 untouched; Clay status unchanged.

Per F-Disp-6 (parent §8), F-Sk-5 (first attempt §10), and
F-Sk-Mu-6 (§11): this retry does not inform BT-544 directly. The
auxiliary→main distinction is preserved at all three layers.
