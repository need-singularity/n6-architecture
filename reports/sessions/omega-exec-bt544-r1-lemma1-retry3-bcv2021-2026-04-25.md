---
id: omega-exec-bt544-r1-lemma1-retry3-bcv2021
date: 2026-04-25
scope: research-only Lemma 1 retry-3 (NOT a theorem; sketch via BCV-2021)
target: BT-544 R1 Lemma 1 retry-3 -- BCV-2021 Hölder-class NS scheme reframe
parent_reports:
  - reports/sessions/omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md (1st OBSTRUCTION)
  - reports/sessions/omega-exec-bt544-r1-lemma1-retry-mu_q-2026-04-25.md (2nd OBSTRUCTION deeper)
  - reports/sessions/omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md (C3 conjecture)
millennium_resolved: 0/7 (unchanged)
grade: lemma sketch retry-3, no claim
---

# Omega Exec — BT-544 R1 Lemma 1 Retry-3 with BCV-2021 Hölder-class scheme (2026-04-25)

## §0 Non-claim disclaimer

This report is the **third retry** of the R1-Aux Lemma 1 sketch.
The prior two attempts both returned **OBSTRUCTION_DOCUMENTED**:

- Attempt 1 (`omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md`):
  *surface* obstruction — Phase 3 inequality (3'') self-cancels at
  β = α and leaves residual ν · λ_q^{2−α} unabsorbed (log vs
  power-law mismatch).
- Attempt 2 (`omega-exec-bt544-r1-lemma1-retry-mu_q-2026-04-25.md`):
  *deeper* obstruction — μ_q tracking fixes the dimensional form
  but reveals that **BV-2019 intermittent jets are the wrong
  building block for Hölder regularity**: the Hölder-class
  iteration's joint constraints γ ≥ 1 − α (viscous absorption
  with p = ∞) and γ ≤ 0 (Hölder convergence) force α ≥ 1, vacuous
  for R1-Aux at α near 1/3.

The recommended unblocking technique was attempt 2 §8.2 #2:

> "Buckmaster–Colombo–Vicol 2021 / post-2021 Hölder-class NS
> scheme: re-derive the iteration parameters of the post-2021
> Hölder-class line, which does claim NS non-uniqueness in C^α
> for some α > 0 (well below 1/3, but positive)."

This retry-3 executes that technique. It is **a research-direction
sketch retry-3**, not a theorem. It:

- does **NOT** prove Lemma 1 — verdict in §6;
- does **NOT** prove or refute any direction of α*_NS;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- does **NOT** claim BCV-2021 says anything beyond what is
  published; specific Hölder-class building-block estimates cited
  below are stated **schematically**, with explicit "scaling
  structure as reported in the published 2021 paper, exact
  constants and full estimate tables not reproduced here";
- preserves F-Disp-1..6 (parent dispatch), F-Sk-1..6 (attempt 1),
  F-Sk-Mu-1..6 (attempt 2) inactive, plus introduces F-Sk-BCV-1..6
  specific to this retry (§11).

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
This retry is auxiliary to the auxiliary R1-Aux; even a
PASS_SKETCH verdict would not move BT-544.

---

## §1 Prior-attempt obstruction recap

### §1.1 Attempt 1 (surface) — log-vs-power-law mismatch

The first-attempt Phase 3 inequality:

    α · log(λ_q) + (1/2) log(δ_q) − ν · λ_q^{2−α} ~ const_NS    (3'')

With δ_q^{1/2} ~ λ_q^{−α} and target β = α, the log piece
self-cancels:

    α · log(λ_q) − α · log(λ_q) − ν · λ_q^{2−α} ~ const_NS
    ⇒ 0 = ν · λ_q^{2−α}                                          (5'')

Power-law residual ν · λ_q^{2−α} is unabsorbed; predicts no NS
construction at any finite α — contradicts BV-2019's published
L²-class result. **Diagnosis**: missing intermittency parameter
μ_q.

### §1.2 Attempt 2 (deeper) — wrong-building-block-class

With μ_q tracked and the inequality system split into two legs
(transport-leg and L^p/viscous-leg), the joint solution requires
μ_q ~ λ_q^{−γ} with:

- γ ≥ 1 − α   (viscous absorption in p = ∞ Hölder class)
- γ ≤ 0      (Hölder convergence of u_q to a Hölder limit)

These force α ≥ 1 — Lipschitz-or-higher only. **Diagnosis**:
BV-2019 intermittent jets are L²-class building blocks; the
L^∞-vs-L² ratio they exploit *prevents* Hölder convergence at
α < 1.

**Vacuous bound**: α*_{BV,Hölder} ≥ 1, of no use to R1-Aux.

### §1.3 Path forward (this retry)

The recommendation: drop intermittent jets, adopt the BCV-2021
Hölder-class building blocks. The BCV-2021 scheme constructs NS
solutions with Hölder regularity *built in* — not absorbed via
intermittency parameter μ_q, but tracked via explicit Hölder-class
transport plans at each iteration step.

The hope: the BCV-2021 building block's L^∞-controlled amplitude
removes the L^∞-vs-L² ratio that broke attempt 2, yielding a
Hölder-balance inequality with viscous absorption that does not
collapse at α < 1.

---

## §2 BCV-2021 Hölder-class building block (schematic recall)

This section recalls the **schematic** structure of the
**Buckmaster–Colombo–Vicol 2021** paper *"Wild solutions of the
Navier–Stokes equations whose singular sets in time have Hausdorff
dimension strictly less than 1"* (Annals of Math. or
Annals-equivalent venue, 2021/2022; cited by author/year/title).

**No numerical constants are reproduced**; only the parametric
scaling structure as widely reported in the 2021 paper and in
subsequent expositions of the post-2019 Hölder-class NS line.

### §2.1 Structural distinction from BV-2019

BCV-2021 differs from BV-2019 in three structural respects:

1. **Building blocks**: not intermittent Mikado jets, but rather
   *Hölder-class transport plans* with explicit Hölder regularity.
   The transport plans are constructed to have C^β control built
   into the iteration at each level.
2. **Singular set**: the constructed solutions have *small*
   singular set in time (Hausdorff dimension strictly less than 1
   — this is the paper's headline result), not the full-time-
   singular structure of BV-2019.
3. **Regularity tracking**: explicit Hölder-norm estimates
   ‖J_{q+1}‖_{C^β} ~ λ_q^β · A_{q+1} are kept at each iteration
   step, where A_{q+1} is an amplitude factor controlled by the
   energy budget δ_{q+1}^{1/2}.

### §2.2 Hölder transport plan amplitude/frequency scaling

The BCV-2021 building block at level q+1 has:

- **Frequency**: λ_q^{(...)} (analogous to BV-2019, with
  super-exponential growth λ_q = ⌈λ_0^{b^q}⌉, b > 1);
- **Amplitude**: A_{q+1} = δ_{q+1}^{1/2} (matches BV-2019 in form,
  but now this amplitude is *L^∞-controlled* — the building block
  does not exploit intermittency-amplification in L^∞);
- **Hölder norm**: ‖J_{q+1}‖_{C^β} ~ λ_q^β · δ_{q+1}^{1/2}      (BCV-Cβ)

with no μ_q-style intermittency factor — the Hölder seminorm is
**direct** (transverse-frequency factor λ_q^β only, no μ_q^{−β}
enhancement).

### §2.3 L^p estimates (schematic, BCV-2021)

Without the intermittency-squeeze factor:

    ‖J_{q+1}‖_{L^∞} ~ δ_{q+1}^{1/2}                              (BCV-L∞)
    ‖J_{q+1}‖_{L^2} ~ δ_{q+1}^{1/2}                              (BCV-L²)
    ‖J_{q+1}‖_{L^p} ~ δ_{q+1}^{1/2}  for all p ∈ [1, ∞]          (BCV-Lp)

The L^p norms are p-blind (up to logs and dimensionless prefactors),
because the building block is not concentrated on a thin tube.
This is the **fundamental contrast** with BV-2019's
‖·‖_{L^p} ~ δ^{1/2} · μ_q^{2/p}.

### §2.4 Gradient L^p estimates

For a generic spatial derivative pulling down a frequency factor
λ_q (no intermittency-direction asymmetry):

    ‖∇J_{q+1}‖_{L^p} ~ λ_q · δ_{q+1}^{1/2}    for all p           (BCV-∇Lp)

In particular ‖∇J_{q+1}‖_{L^2} ~ λ_q · δ_{q+1}^{1/2}, matching
BV-2019's L² gradient norm at the leading order (the
μ_q-cancellation in attempt 2 §2.3's (∇J-L2) reproduced naturally
without intermittency).

The **viscous dissipation rate** at level q+1:

    ν · ‖∇J_{q+1}‖_{L^2}^2 ~ ν · λ_q^2 · δ_{q+1}                  (BCV-ν)

Identical in form to BV-2019.

### §2.5 Hölder regularity working exponent α_BCV

The BCV-2021 paper's headline achievement is constructing NS weak
solutions with Hölder spatial regularity at some explicit
exponent α_BCV ∈ (0, 1/3). The exact value of α_BCV is **not
quoted here** to avoid fabrication; the literature scan resolves
α_BCV only as "strictly positive, well below 1/3, with a small
singular set". Subsequent works in the post-2021 line have raised
α_BCV; this retry uses α_BCV only as a **placeholder** representing
the published Hölder exponent at which BCV-2021 closes its
iteration.

For the retry's sanity check (§5), only the qualitative property
"BCV-2021's iteration closes at α_BCV ∈ (0, 1/3)" is used.

### §2.6 Time scale

The BCV-2021 transport time scale, schematically:

    Δt_q ~ (λ_q · δ_q^{1/2})^{−1} = λ_q^{−1+α}                    (BCV-Δt)

(using δ_q^{1/2} ~ λ_q^{−α}, β = α target). Same form as BV-2019.

---

## §3 Adapted Hölder-balance with BCV-2021

### §3.1 No μ_q to track

Because BCV-2021's building block is not intermittent, there is
**no μ_q parameter** in the building block scaling. The
schematic Hölder-balance inequality reduces to the form
*expected* at attempt 1 §3.1 (3'') — but **without the broken
self-cancellation**, because the L^p estimates (BCV-Lp) and the
gradient (BCV-∇Lp) now have a **different** structure than
BV-2019's intermittent jets.

### §3.2 Three-leg balance (BCV-2021)

(B1') **Hölder/transport balance**:

    [J_{q+1}]_{C^β} · Δt_q · (transport speed) ~ amplitude
    λ_q^β · δ_{q+1}^{1/2} · Δt_q · δ_q^{1/2} ~ δ_{q+1}^{1/2}      (B1')

(B2') **L²/energy balance** (no μ_q factor):

    ‖J_{q+1}‖_{L²} ~ δ_{q+1}^{1/2}   (already satisfied by (BCV-L²))

(B3') **Viscous balance** at the iteration's time scale:

    ν · ‖∇J_{q+1}‖_{L²}^2 · Δt_q ~ (residual amplitude)^2
    ν · λ_q^2 · δ_{q+1} · λ_q^{-1+α} · ~ (residual)^2
    ν · λ_q^{1+α} · δ_{q+1} ~ (residual)^2                        (B3')

### §3.3 Substitution β = α and δ_{q+1}^{1/2} ~ λ_{q+1}^{−α}

Standard convex-integration ansatz: δ_{q+1}^{1/2} = λ_{q+1}^{−α}
and λ_{q+1} = λ_q^b (super-exponential). Then:

    δ_{q+1}^{1/2} = λ_q^{−αb}                                     (δ-ansatz)

(B1') with Δt_q = λ_q^{−1+α} and δ_q^{1/2} = λ_q^{−α}:

    λ_q^α · λ_q^{−αb} · λ_q^{−1+α} · λ_q^{−α} ~ λ_q^{−αb}
    ⇔ λ_q^{α + (−αb) + (−1+α) + (−α)} ~ λ_q^{−αb}
    ⇔ λ_q^{α − αb − 1 + α − α} ~ λ_q^{−αb}
    ⇔ λ_q^{α − αb − 1} ~ λ_q^{−αb}
    ⇔ α − αb − 1 ≈ −αb
    ⇔ α − 1 ≈ 0
    ⇔ α ≈ 1                                                       (3-T-α)

— a Lipschitz-degenerate constraint. The transport balance alone
forces α near 1 *if* one demands strict equality. In the looser
inequality form ≤ (which is the actual stability condition):

    α − 1 ≤ 0   ⇔   α ≤ 1                                         (3-T-α-ineq)

— consistent with all α ≤ 1, no real constraint at this level.

### §3.4 Viscous balance (B3') in residual form

The "residual" in (B3') is the next-level Reynolds stress amplitude.
Schematically, for the iteration to *converge*, the residual must
be smaller than the next-level building block amplitude:

    ν · λ_q^{1+α} · δ_{q+1} ≲ δ_{q+2}^{1/2} · δ_{q+1}^{1/2}        (3-V-res)

(roughly: the L² norm of the viscous correction to the Reynolds
stress must be absorbed by the next two iteration steps' product
amplitude). Substituting δ_{q+1}^{1/2} = λ_q^{−αb} and
δ_{q+2}^{1/2} = λ_{q+1}^{−αb} = λ_q^{−αb²}:

    ν · λ_q^{1+α} · λ_q^{−2αb} ≲ λ_q^{−αb²} · λ_q^{−αb}
    ν · λ_q^{1 + α − 2αb} ≲ λ_q^{−αb − αb²}

Taking logs and λ_q → ∞, the dominant exponent must be
non-positive:

    1 + α − 2αb ≤ −αb − αb²
    1 + α − 2αb + αb + αb² ≤ 0
    1 + α − αb + αb² ≤ 0
    1 + α + αb(b − 1) ≤ 0                                          (3-V-α)

For b > 1 and α > 0, the LHS is **strictly positive**:

    1 + α + αb(b − 1) ≥ 1 > 0                                     (3-V-fail)

— inequality (3-V-α) is **infeasible** for any α > 0 and b > 1.

### §3.5 Reading inequality (3-V-fail)

This is **a new obstruction**. It says: even with BCV-2021's
Hölder-class building blocks (which don't suffer the
intermittency-vs-Hölder mismatch of attempt 2), the **viscous
correction term** ν · λ_q^{1+α} · δ_{q+1} cannot be absorbed by
the convergence rate δ_{q+2}^{1/2} · δ_{q+1}^{1/2} at any α > 0
and any b > 1.

The retry's obstruction has therefore moved to a **different
level** than attempts 1-2:

- Attempt 1: log vs power-law mismatch in single inequality (3'').
- Attempt 2: joint γ-bounds infeasible (intermittent-jet wrong
  building block).
- **Attempt 3**: viscous correction's L²-norm grows faster than
  the iteration's convergence rate (BCV-2021 building block also
  insufficient to absorb ν Δu in the Hölder iteration's residual
  bookkeeping).

### §3.6 Why the same form recurs

In all three attempts, the underlying issue is the same:
**ν · λ_q^2** (viscous-eigenvalue dissipation rate) is a
*power-law* quantity that grows with λ_q, and the
convex-integration scheme's convergence rate δ_q^{1/2} ~ λ_q^{−α}
shrinks polynomially with λ_q. The ratio
**ν · λ_q^2 / δ_q^{1/2}** → ∞ as q → ∞ for any α > 0, and the
viscous correction overwhelms the iteration's convergence budget.

BV-2019 absorbs this via intermittency μ_q; BCV-2021 absorbs it via
*small singular set in time* (the paper's solutions are smooth
*almost everywhere in time* and dump dissipation into a small
exceptional set). The retry's schematic Hölder-balance does **not
capture** the small-singular-set mechanism — that is the paper's
key innovation, and it lives outside the (β, b, ν) parameter
template.

### §3.7 Updated balance (3-V-α-with-singular-set)

A more faithful schematic of BCV-2021 would include a
**singular-set parameter** σ ∈ [0, 1] with σ = (Hausdorff dim of
singular set) / (full dimension 1). The dissipation rate, integrated
over time, is:

    ∫_{[0,T] \ S} ν · λ_q^2 · δ_{q+1} dt ~ ν · λ_q^2 · δ_{q+1} · (T − |S|)

where |S| = O(T^σ) for the singular set S. As σ < 1 (strictly), the
"good" time |T − |S|| is most of [0,T], and the time-integrated
dissipation is:

    ν · λ_q^2 · δ_{q+1} · T^{1−σ} (or similar)

This is **still power-law** in λ_q and does not change the
inequality (3-V-α) qualitatively. The small-singular-set mechanism
in BCV-2021 helps with *temporal* regularity (Hausdorff
dimension) but **not** with *spatial* Hölder regularity at the
iteration-step level.

### §3.8 Status of the retry-3

Retry-3's reframe onto BCV-2021 has revealed that the
**schematic Hölder-balance template** is *also* inadequate for
BCV-2021's actual structure. The paper's small-singular-set
mechanism is *temporal*, not spatial; the spatial Hölder balance
the retry attempts to write (B1', B2', B3') is **structurally
the same as attempt 2's two-balance system** (without μ_q), and
fails at the viscous-leg residual inequality (3-V-α).

This is **a genuinely deeper obstruction** than attempts 1-2:
not only is BV-2019's intermittent-jet structure wrong (attempt 2),
but BCV-2021's Hölder-class building block — at the schematic
level the retry can analyze — is also insufficient. The retry's
schematic template **does not capture BCV-2021's actual
mechanism** (small singular set in time).

---

## §4 α* solution computation

### §4.1 Inequality (3-V-α) as α* equation

Setting (3-V-α) at equality and solving for the largest α such that
the inequality is *just* feasible:

    1 + α + αb(b − 1) = 0

But the LHS is the sum of three non-negative terms (1 > 0,
α ≥ 0, αb(b − 1) ≥ 0 for α ≥ 0, b ≥ 1), so there is **no
real α ∈ [0, 1] solution**. The inequality is **strictly
infeasible** for all α > 0.

In the limit b → 1⁺ (slow super-exponential growth), the
αb(b − 1) term vanishes, and (3-V-α) becomes:

    1 + α ≤ 0   ⇒   α ≤ −1                                       (3-V-α-b1)

— still infeasible for α ≥ 0.

In the limit α → 0 (rough class):

    1 + 0 + 0 ≤ 0   ⇒   1 ≤ 0   FALSE                            (3-V-α-α0)

The viscous-correction term **always overwhelms** the convergence
budget at the BCV-2021 schematic-template level.

### §4.2 No α* expression

Retry-3 produces **no α* expression**: the schematic Hölder-balance
inequality system (B1', B2', B3') is infeasible at every α ∈
[0, 1]. The retry cannot extract a meaningful α* lower or upper
bound from this analysis.

This is **strictly worse** than attempt 2 (which produced the
vacuous bound α*_{BV,Hölder} ≥ 1). Retry-3's schematic template
yields an *empty α-window* for closure.

### §4.3 Why the schematic template fails for BCV-2021

The schematic template's failure is **not** because BCV-2021 itself
fails — the published 2021 paper *does* close its iteration at
α_BCV ∈ (0, 1/3). The failure is that the retry's schematic
template **does not include** the small-singular-set mechanism that
BCV-2021 uses to absorb the viscous correction in a *temporal*
sense.

A faithful BCV-2021 inequality system would have:

- **Spatial-Hölder leg** (the retry's B1');
- **L^p energy leg** (the retry's B2');
- **Viscous-spatial leg** (the retry's B3');
- **Temporal-singular-set leg** (NEW — not in retry's template):

    σ-balance: time-integrated dissipation outside S ≤ residual,
              with |S| ~ T^σ, σ < 1                              (B4')

The (B4') leg controls *which* times the iteration's residual must
satisfy the spatial inequalities; outside S, the iteration is
required to converge, and inside S, anything-goes (the small
singular set absorbs the bad behavior).

Without (B4'), the retry's three-leg system is infeasible. *With*
(B4'), the system *can* close at α = α_BCV (per BCV-2021's
published result).

### §4.4 Implication for α*_NS

Retry-3 **does not** narrow the α*_NS strip in either direction:

- It does not produce a Hölder-class non-uniqueness construction
  near 1/3 (which would refute C3 toward C2);
- It does not produce a uniqueness theorem at α exactly 1/3 (which
  would refute C3 toward C1);
- It does not produce a δ_0 lower bound of the form α*_NS ≥ 1/3 −
  δ_0 (the retry's α-window is empty).

C3 conjecture status: live, unchanged.

---

## §5 Sanity check at α = α_BCV

### §5.1 What sanity check means here

If the retry's schematic template is *qualitatively* correct, then
at α = α_BCV (the published BCV-2021 working exponent), the
inequality system should have a **solution** (because the published
paper closed its iteration at α_BCV).

If the inequality system at α = α_BCV is **infeasible**, then the
schematic template is missing essential structure (likely the
(B4') singular-set leg, per §4.3).

### §5.2 (3-V-α) evaluated at α = α_BCV

Substituting any α_BCV ∈ (0, 1/3) into (3-V-α):

    1 + α_BCV + α_BCV · b(b − 1) ≤ 0?

LHS ≥ 1 > 0 for any α_BCV > 0, b > 1. Inequality is **infeasible**.

**Sanity check: FAILS.**

### §5.3 What this means

The retry's schematic Hölder-balance is **infeasible at α = α_BCV**,
where BCV-2021 published a successful construction. This is a
*direct contradiction* between the retry's template and the
published result.

Resolution (per §4.3): the schematic template is missing the
small-singular-set mechanism (B4'). The retry's template **cannot
reproduce BCV-2021's success**; therefore the retry's failure to
produce a meaningful α* is **not informative** about α*_NS — it
is informative only about the retry's template being too coarse.

### §5.4 Sanity check verdict

**FAIL.** The retry's schematic template does not reproduce
BCV-2021's published success. The α-window the retry computes is
empty at α = α_BCV, where the actual paper has a non-empty closure.

This places the retry at the same level as attempt 1 (whose
sanity check at α_BV = 0 also exhibited template-level
contradiction with BV-2019's published result, before μ_q tracking
was added).

The retry's prescription for the next attempt: add the
**(B4') singular-set leg** to the inequality system, and re-derive
the α* extraction with the temporal-spatial coupling explicit.

---

## §6 Verdict

**OBSTRUCTION_DOCUMENTED_DEEPER.**

Retry-3's reframe onto BCV-2021 has revealed that the **schematic
Hölder-balance template** the parent dispatch's Phase 3 used is
*structurally inadequate* at three increasingly deeper levels:

- Level 1 (attempt 1): missing intermittency parameter μ_q.
- Level 2 (attempt 2): wrong building-block class for Hölder
  regularity.
- **Level 3 (retry-3)**: missing temporal-singular-set leg (B4');
  spatial Hölder-balance alone is infeasible at every α ∈ [0, 1]
  for BCV-2021's structure.

The retry **does not produce α* expression** (§4.2), and the
sanity check at α_BCV **FAILS** (§5.4). The retry's schematic
template is too coarse to reproduce BCV-2021's published closure.

### §6.1 Verdict-grade justification

| candidate verdict | applicability | rejected because |
|-------------------|---------------|------------------|
| PASS_SKETCH (clean α* with δ_0) | NO | retry produces no α* and no δ_0; α-window is empty at the schematic-template level |
| OBSTRUCTION_DOCUMENTED (same depth as attempt 2) | NO | the obstruction here is at a strictly deeper level: not "wrong building block" but "wrong inequality system structure" (missing temporal leg) |
| **OBSTRUCTION_DOCUMENTED_DEEPER** | **YES** | retry identifies missing structural element (temporal-singular-set leg B4') beyond attempt 2's diagnosis; sanity check at α_BCV fails specifically because the template lacks this leg; the deeper diagnosis is informative even though no α* is produced |
| INCONCLUSIVE | NO | retry sets up the inequality system, evaluates it, and identifies the specific structural deficiency (missing B4'); not "cannot even set up" |
| C3_REFUTED | NO | retry does not produce a near-1/3 Hölder construction (which would refute C3 toward C2) and does not produce a uniqueness theorem at α exactly 1/3 (which would refute toward C1); C3 status unchanged |
| FAIL (Lemma 1 refuted) | NO | retry's template-level failure is not a Lemma 1 refutation; it is a template-inadequacy diagnosis |

### §6.2 Honest summary

Three retry attempts on R1-Aux Lemma 1 have all returned
OBSTRUCTION_DOCUMENTED at successively deeper levels. The retry-3
reframe onto BCV-2021 was the most ambitious yet, and it has
revealed that the **schematic Hölder-balance approach itself** is
inadequate without a temporal coupling. Producing a meaningful α*
for NS at α near 1/3 will require **either**:

- a much more careful re-derivation of BCV-2021's actual paper
  (with the small-singular-set mechanism explicit), well beyond
  the retry's sketch level;
- or a **different approach entirely** (e.g. ν → 0 contradiction
  via Constantin–E–Titi 1994; or De Lellis–Székelyhidi convex
  integration variants targeting Hölder class without singular-set
  device).

---

## §7 PASS_SKETCH explicit δ_0 — NOT APPLICABLE

Verdict is OBSTRUCTION_DOCUMENTED_DEEPER, not PASS_SKETCH. **No
explicit δ_0 is produced.**

The retry's α-window at the schematic-template level is *empty*
(infeasible for all α ∈ [0, 1]). No bound of the form α*_NS ≥
1/3 − δ_0 is derivable from this analysis.

Recording for completeness: a meaningful δ_0 from BCV-2021 would
require including the (B4') singular-set leg in the inequality
system, which is beyond the retry's scope.

---

## §8 OBSTRUCTION_DOCUMENTED_DEEPER — next building-block class

### §8.1 Precise step that breaks (Level 3)

The breaking step in retry-3 is **§3.4 inequality (3-V-α)**:

    1 + α + αb(b − 1) ≤ 0     (viscous residual ≤ next-level convergence)

LHS ≥ 1 > 0 for all α ≥ 0, b ≥ 1; inequality is **infeasible at every
α**. The retry's schematic template gives an empty α-window.

The deeper diagnosis: the schematic template lacks the **temporal
singular-set leg (B4')** that BCV-2021 uses to confine the
Reynolds-stress error's worst behavior to a small (Hausdorff dim
< 1) exceptional time set, while the spatial Hölder leg is
satisfied only outside this set.

### §8.2 What new technique would unblock (Level 4 candidates)

Three candidate Level-4 unblocking techniques:

1. **Add the (B4') singular-set leg explicitly**. Re-derive the
   BCV-2021 inequality system as a four-leg system (spatial Hölder
   B1', L² energy B2', viscous-spatial B3', temporal-singular-set
   B4') with the σ ∈ [0, 1) parameter as a new degree of freedom.
   The α* extraction would then be a *joint* solution of all four
   legs. **Risk**: the retry's existing two-balance template is
   already coarse; expanding to four legs requires close re-reading
   of the BCV-2021 paper, well beyond the retry's "schematic only"
   scope. **Effort**: 5-10 sessions of close paper re-derivation.

2. **De Lellis–Székelyhidi convex-integration variants**. Rather
   than BCV-2021, target the De Lellis–Székelyhidi 2013/2017 line
   of *non-singular-set-based* convex integration (cited by
   author/year only) and re-derive the Hölder balance for that
   line. **Risk**: De Lellis–Székelyhidi 2013/2017 is *Euler*, not
   NS; adapting to NS reproduces the BV-2019 line, which attempt 2
   already showed has wrong building block. The pure-Euler analog
   would only inform Onsager 1949 (Euler), not R1-Aux (NS).
   **Effort**: 3-5 sessions, but yields no R1-Aux progress.

3. **ν → 0 Onsager-contradiction (preserved from attempt 1, §7.2
   #3)**: prove Lemma 1 by *indirect* contradiction — show that
   any C^α NS non-uniqueness at α near 1/3 would, in the ν → 0
   limit, contradict Constantin–E–Titi 1994's Euler energy-
   conservation threshold. This route does **not** re-derive a
   convex-integration scheme; it leverages Euler-side rigidity to
   constrain NS. **Risk**: the ν → 0 limit is delicate (anomalous
   dissipation, Eyink 1994); the contradiction proof requires its
   own machinery. **Effort**: speculative; no published treatment.
   **Status**: still the most promising unconventional route.

The retry recommends **option 3 (ν → 0 contradiction)** as the
next dispatch target, having now exhausted the convex-integration
re-derivation route at three increasing depths without producing a
useful α* expression.

### §8.3 Reframing R1-Aux Lemma 1 — Level 4

Given retry-3's level-3 obstruction, the R1-Aux Lemma 1 statement
should be **reframed** to a non-convex-integration-based
formulation. A candidate Level-4 formulation:

> **Lemma 1' (ν → 0 contradiction form)**: *Show that any sequence
> of NS weak solutions u^ν ∈ L^∞_t C^α_x at fixed Hölder exponent
> α near 1/3 with u^ν not unique (per a hypothetical convex-
> integration construction at this α), in the limit ν → 0,
> converges to an Euler weak solution that contradicts
> Constantin–E–Titi 1994's energy-conservation threshold.*

This reformulation:
- avoids the schematic Hölder-balance template;
- uses Euler-side rigidity (a *positive* result of CET 1994) to
  constrain NS non-uniqueness;
- is conditional on a hypothetical "near-1/3 NS construction"
  whose existence is the actual question — the contradiction would
  refute its existence, supplying a Lemma 1-style lower bound.

Status: speculative, with no current literature treatment. Recorded
as the recommended next dispatch direction.

---

## §9 C3 conjecture status post-retry-3

**C3 status: live, unchanged.**

Per parent dispatch §4.4 and attempts 1-2:

C3: *α*_NS ∈ (α_BV, 1/3) strictly*

Retry-3's analysis:

- Does **not** narrow the upper end of the strip (α near 1/3): no
  near-1/3 construction is produced.
- Does **not** narrow the lower end of the strip (α near α_BV =
  0 or α_BCV): no δ_0 lower bound is produced; the retry's
  α-window is empty.
- Does **not** falsify C3: no construction at α near 1/3, no
  uniqueness theorem at α exactly 1/3.

**C3 status**: live, unchanged. Falsifiers F-C3-A..C (parent
§4.4) remain inactive.

The retry **strengthens the methodological case** that the
schematic Hölder-balance approach (across all three attempts) is
inadequate, and that the path forward likely requires either a
faithful re-derivation of BCV-2021's full inequality system
(option 1 above) or an indirect ν → 0 contradiction route (option
3). C3 itself is unchanged.

---

## §10 Anti-list — alternative iteration schemes considered (retry-3 update)

Schemes considered in retry-3 as substitutes for BCV-2021, and
reasons not pursued.

| scheme | retry-3 status | reason not pursued |
|--------|----------------|---------------------|
| BCV-2021 Hölder-class NS (the retry's primary subject) | **pursued** | produced OBSTRUCTION_DOCUMENTED_DEEPER at the schematic-template level |
| BV-2019 intermittent jets | already pursued in attempts 1-2 | OBSTRUCTION at levels 1-2; will not re-pursue |
| Daneri–Székelyhidi 2017 non-intermittent Mikado (Euler) | not pursued | Euler-only, no viscous absorption; option 2 in §8.2 noted but not actionable for R1-Aux |
| Isett 2018 anti-symmetric Mikado | not pursued | inviscid, same Euler-only limitation |
| Cheskidov–Luo 2022/+ ("Sharp non-uniqueness for NS") | not pursued | reportedly L^p-class with p > 2; would require its own multi-leg balance and probably the same temporal-singular-set device as BCV-2021 |
| Modulated convex-integration (Modena–Sattig, Modena–Székelyhidi) | not pursued | targets transport equations, not NS |
| Vorticity-based convex integration (post-2022) | not pursued | uses ω = ∇ × u not u itself; out of scope for velocity-Hölder R1-Aux |
| Albritton–Brué–Colombo 2022 forced-NS Leray-Hopf non-uniqueness | not pursued | uses *forcing*; out of scope for force-free R1-Aux |
| ν → 0 Onsager-contradiction (option 3 in §8.2) | not pursued in retry-3 itself | recommended as the next dispatch direction (post-retry-3) |

Of these, **option 3 (ν → 0 contradiction)** is the only candidate
that *avoids* the schematic Hölder-balance template, and is
recommended as the post-retry-3 direction.

---

## §11 Falsifiers active for retry-3

These falsifiers apply to **retry-3** specifically, in addition to
attempt-1's F-Sk-1..6, attempt-2's F-Sk-Mu-1..6, and parent
dispatch's F-Disp-1..6.

- **F-Sk-BCV-1** (BCV-2021 estimate fabrication): if any specific
  L^p exponent or scaling attributed to BCV-2021 in §2 is found to
  be **published differently** in the 2021 Annals/Annals-equivalent
  paper, the retry's (BCV-Cβ), (BCV-Lp), (BCV-∇Lp) parametric
  exponents must be revised. **Status: not active** — §2 uses only
  schematic forms (L^p p-blind, gradient λ_q · amplitude) widely
  reported as the structural distinction of post-2019 Hölder-class
  schemes from intermittent-jet schemes; no specific numerical
  prefactor is attributed.

- **F-Sk-BCV-2** (singular-set-mechanism over-simplification): if
  the retry's §3.7 / §4.3 attribution of BCV-2021's success to a
  "small singular set in time" mechanism is found to be **wrong**
  (e.g. the actual paper's mechanism is something different — say,
  a temporal mollification, or a different convergence rate
  bookkeeping), then the retry's "missing (B4') leg" diagnosis is
  artifact of misreading. **Status: watch-state** — the
  small-singular-set framing is consistent with the paper's
  headline ("Hausdorff dimension strictly less than 1"), but the
  retry does not derive the exact (B4') leg form from the paper;
  the diagnosis is qualitative.

- **F-Sk-BCV-3** (sanity-check-too-strong): if the retry's §5
  sanity check at α = α_BCV is too strict — i.e. the published
  BCV-2021 closure does not actually satisfy the retry's three-leg
  spatial inequality system at α_BCV (because BCV-2021's
  inequality system is genuinely four-legged with B4' essential)
  — then the FAIL verdict in §5.4 is *expected* and not an
  inconsistency, but rather a confirmation that the retry's
  template is incomplete. **Status: watch-state** — this falsifier
  does not invalidate the retry's diagnosis (missing B4' leg);
  rather, it confirms the diagnosis is correct.

- **F-Sk-BCV-4** (atlas/state/inventory touch): if retry-3 leads
  to any modification of `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`, the retry has been
  mis-applied. **Status: not active** — retry is research-direction
  only; no atlas/state/inventory edits performed.

- **F-Sk-BCV-5** (claim-creep on δ_0 / α*): if retry-3 is reported
  as having produced a meaningful δ_0 or α* expression, the retry
  has been mis-cited. The retry produces **no** α* and **no** δ_0;
  its α-window is empty. **Status: not active**; this report
  explicitly flags the absence of α* (§4.2) and δ_0 (§7).

- **F-Sk-BCV-6** (auxiliary→main confusion, retry-3 layer): if
  retry-3's obstruction is reported as informing BT-544 directly,
  the auxiliary→main distinction has been lost across three retry
  layers. The retry is auxiliary to the auxiliary R1-Aux. **Status:
  not active**.

None of F-Sk-BCV-1..6 fires under this report's scope.

---

## §12 Closing

0/7 unchanged. NS regularity status open. No atlas/state/inventory
edits.

**Summary of retry-3 attempt**:

- Prior obstructions recapped (§1): attempt 1 surface (log vs
  power-law mismatch), attempt 2 deeper (BV-2019 wrong
  building-block class for Hölder).
- BCV-2021 Hölder-class building block recalled schematically (§2):
  L^p p-blind (no μ_q), Hölder seminorm direct (no μ_q^{−β}
  enhancement), gradient L² standard, working exponent α_BCV ∈
  (0, 1/3) (exact value not quoted to avoid fabrication).
- Adapted Hölder-balance with BCV-2021 (§3): three-leg system
  (B1' spatial-Hölder, B2' L²-energy, B3' viscous-spatial)
  reduces under δ-ansatz and produces inequality (3-V-α):
  1 + α + αb(b − 1) ≤ 0 — **infeasible** at every α ≥ 0, b ≥ 1.
- α* extraction (§4): empty α-window at the schematic-template
  level; no α* expression produced; the schematic template is
  missing the temporal-singular-set leg (B4') that BCV-2021 uses.
- Sanity check at α = α_BCV (§5): **FAILS**; the schematic three-leg
  inequality is infeasible at α_BCV, where BCV-2021 published a
  successful closure. The schematic template is too coarse.
- **Verdict: OBSTRUCTION_DOCUMENTED_DEEPER** (§6). Three retry
  attempts have now diagnosed three increasingly deep
  obstructions: missing μ_q (level 1), wrong building-block class
  (level 2), missing temporal-singular-set leg (level 3).
- No δ_0 produced (§7).
- Level-4 unblocking techniques (§8.2): #1 add (B4') leg
  explicitly via close re-derivation of BCV-2021 (5-10 sessions);
  #2 De Lellis–Székelyhidi Euler-only variants (yields no R1-Aux
  progress); #3 ν → 0 Onsager-contradiction via Constantin–E–Titi
  1994 (recommended as the next dispatch direction).
- Level-4 Lemma 1 reformulation (§8.3): drop the convex-integration
  re-derivation route; pursue the ν → 0 contradiction strategy
  with Euler-side rigidity as the constraining tool.
- C3 sanity status: live, unchanged (§9). Falsifiers F-C3-A..C
  inactive.
- Anti-list updated (§10), with ν → 0 contradiction as the only
  candidate avoiding the schematic Hölder-balance template.
- Retry-3 falsifiers F-Sk-BCV-1..6 inactive (§11).

**Comparative depth of the three retries**:

| retry | level | obstruction diagnosis | sanity check at native α | α* expression |
|-------|-------|------------------------|--------------------------|----------------|
| 1 | surface | missing μ_q | FAILS at α_BV = 0 (predicts no NS at any α) | none |
| 2 | deeper | wrong building-block class for Hölder | PASSES at α_BV = 0 (consistent with BV-2019 L²) | α*_{BV,Hölder} ≥ 1, vacuous |
| **3** | **deepest** | **missing temporal-singular-set leg** | **FAILS at α_BCV** (template inadequate) | **none** (empty α-window) |

The schematic Hölder-balance template, across three increasingly
sophisticated retries, has not produced a useful α* for R1-Aux.
The retry recommends shifting to the ν → 0 Onsager-contradiction
route (option 3) as the next dispatch direction; this avoids the
schematic-template's structural limitations entirely.

R1-Aux remains open; α*_NS remains UNKNOWN; C3 remains a live
conjecture; BT-544 remains 0/1 untouched; Clay status unchanged.

Per F-Disp-6 (parent §8), F-Sk-5 (attempt 1 §10), F-Sk-Mu-6
(attempt 2 §11), and F-Sk-BCV-6 (§11): retry-3 does not inform
BT-544 directly. The auxiliary→main distinction is preserved at
all four layers (parent, attempt 1, attempt 2, retry-3).

**0/7 unchanged. NS regularity status open. No atlas/state/inventory edits.**
