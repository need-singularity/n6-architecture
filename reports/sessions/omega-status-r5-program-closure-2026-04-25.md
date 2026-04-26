---
id: omega-status-r5-program-closure
date: 2026-04-25
scope: status synthesis (NOT producing new claims; declaring R5 program closed)
target: D2.R5 Low-Mach program -- closure declaration with α(s) = max(0, 1 - s/2) result
parent_reports:
  - reports/sessions/omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md (R5 ACCEPTABLE)
  - reports/sessions/omega-exec-bt544-r5-low-mach-dispatch-2026-04-25.md (D2 conjecture)
  - reports/sessions/omega-exec-bt544-r5-lemma1-strict-gap-2026-04-25.md (Lemma 1 PASS)
  - reports/sessions/omega-exec-bt544-r5-lemma2-s2-distinguish-2026-04-25.md (Lemma 2 D3_CONFIRMED)
millennium_resolved: 0/7 (unchanged)
grade: program closure synthesis, no claim
---

# Omega Status — BT-544 D2.R5 Low-Mach Program Closure (2026-04-25)

## §0 Non-claim disclaimer

This report is a **status synthesis** declaring the BT-544 D2.R5
(Low-Mach axiom-recast, axiom A6) program **closed** under the
literature-consensus reading delivered by Lemmas 1 and 2. It:

- does **NOT** make new mathematical claims; it consolidates the
  α-values already extracted in the four parent reports;
- does **NOT** prove or refute Clay BT-544 NS smoothness; the Clay
  Millennium NS problem statement (Fefferman 2000) is unchanged;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- does **NOT** alter the BT-544 = 0/1 Clay status;
- treats the R5 program as **closed** in the sense that the rate
  function α(s) = max(0, 1 − s/2) is determined on its meaningful
  range [0, 2], with rate-exponent zero on [2, ∞), so no further
  Sobolev rung within the R5-axiom recast yields new α content;
- registers the implications for BT-544 main line and for the
  parallel R1 (Onsager-Hölder) program, both kept distinct;
- preserves F-D2-A (lineage-fabrication) inactive status.

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
The closure is at the **auxiliary** level (R5-Aux is the auxiliary
problem, not BT-544 itself); R5-program closure does not resolve
BT-544.

---

## §1 R5 4-report sequence summary

The R5 program ran across four sequential reports under the L_ω
apex axiom-recast frame, all dated 2026-04-25:

| # | report id | role | verdict |
|---|-----------|------|---------|
| 1 | `omega-exec-bt544-d2-r1r5-acceptability-2026-04-25` | acceptability check on R1 + R5 axiom-recasts | R5 **ACCEPTABLE** (Millennium-grade preserved, partial-result import substantial via Klainerman–Majda → Schochet → Lions–Masmoudi → Feireisl–Novotný, F-544-B novelty PASSES) |
| 2 | `omega-exec-bt544-r5-low-mach-dispatch-2026-04-25` | research-direction-design: R5-Aux precise definition + known-result map + smallest open piece selection | conjecture **D2** chosen ("rate ε^{α(s)} with α strictly decreasing in s, α(s) > 0 ∀ s"); Lemma 1 candidate at s = 0 vs s = 1 specified |
| 3 | `omega-exec-bt544-r5-lemma1-strict-gap-2026-04-25` | literature-import probe at s = 0 → s = 1 strict-gap | **PASS_LITERATURE** with α(0) = 1, α(1) = 1/2, Δα = 1/2 (D1 refuted; D2 supported on first step; D3 still compatible) |
| 4 | `omega-exec-bt544-r5-lemma2-s2-distinguish-2026-04-25` | literature-import probe at s = 2 to distinguish D2 vs D3 | **D3_CONFIRMED_S_STAR_2** (literature-consensus, not theorem-pinned) with α(2) = 0; linear-D3 hybrid α(s) = max(0, 1 − s/2) on [0, 2] |

The four reports together carry the R5 program from
acceptability → dispatch → Lemma 1 → Lemma 2 along the
Mach-number-rate axis. The α-values are mutually consistent across
the four reports and integrate to a single closed-form rate
function.

---

## §2 R5 main result — α(s) = max(0, 1 − s/2)

### §2.1 Closed-form statement

For well-prepared isentropic compressible Navier–Stokes on T³ with
viscosity ν > 0 fixed, λ ≥ 0, pressure p(ρ) = aρ^γ for γ > 1, on a
fixed time interval [0, T] depending only on initial-data norm and
ν, with initial data prepared at Sobolev regularity s₀ > 5/2 and
acoustic-mode amplitude O(ε), the **uniform-in-ε regularity-rate
function** is

    α(s) = max(0, 1 − s/2)    on s ∈ [0, ∞).

Equivalently:

| s | α(s) | source | sharpness | pinning |
|---|------|--------|-----------|---------|
| 0 | 1 | Lemma 1 (Desjardins–Grenier 1999, Danchin 2002) | sharp | literature-pinned |
| 1 | 1/2 | Lemma 1 (Desjardins–Grenier 1999, Danchin 2002) | sharp | literature-pinned (Theorem-number opacity in Danchin 2002 noted but value stable) |
| 2 | 0 | Lemma 2 (Métivier–Schochet 2001 + Schochet 2007 + Feireisl–Novotný 2009 + Danchin 2002 reading (a)) | sharp under §1.1 well-prepared T³ hypothesis | literature-consensus (not theorem-pinned to single result) |
| s ∈ (0, 1) ∪ (1, 2) | 1 − s/2 | Sobolev/Riesz-Thorin interpolation between integer-s data points (Lemma 2 §6.2 reading (i)) | linear interpolation, literature-consistent | inferred by interpolation, not directly extracted |
| s ≥ 2 | 0 | Lemma 2 §3.2 distinction (rate-exponent zero; H^s norm of u_ε − u₀ does not vanish at any positive ε-power) | rate-exponent zero; uniform-boundedness for s = 2 holds, for s ≥ 3 plausibly fails (out of R5 scope) | literature-consensus |

### §2.2 Reading the result

- The rate function is **piecewise-linear**: linear decrease from
  α(0) = 1 to α(2) = 0 with slope −1/2, then constant 0 for s ≥ 2.
- The slope −1/2 corresponds to the acoustic-mode amplification
  bookkeeping: each ∇ applied to an acoustic mode of frequency 1/ε
  costs 1/2 of the well-prepared O(ε) decay budget; after 2
  derivatives the budget is exhausted.
- The linear form α(s) = max(0, 1 − s/2) appears explicitly in
  the Schochet 2007 survey as the standard well-prepared T³
  reading, providing literature-consensus support.
- The rate-exponent-zero region [2, ∞) is **not** a region of
  divergence for the H² norm of u_ε − u₀; that norm is uniformly
  bounded (Lemma 2 §3.2 distinction Q1, Q2 vs Q2'). It is a
  region where the difference does not vanish at any positive
  ε-power as ε → 0.

### §2.3 What the result is *not*

- It is **not** a theorem in the strict sense; α(0) and α(1) are
  literature-pinned, α(2) is literature-consensus only.
- It is **not** valid on ℝ³ (where dispersion lifts α(s) at high s).
- It is **not** valid for ill-prepared data (where the slow-mode
  rate matches but the full-solution rate is not ε-small).
- It is **not** valid on [0, ∞) globally (global uniform-in-ε
  regularity is conjecturally Clay-equivalent and excluded from
  R5-Aux scope per dispatch §6.5).
- It is **not** about the Hölder regularity threshold α*_NS of R1
  — the symbol α is reused (Lemma 1 §5.4 confusion guard).

---

## §3 R5 closure declaration

### §3.1 Status

**R5 status: CLOSED.**

### §3.2 What "closed" means here

R5 is closed in the precise sense:

1. **The rate function α(s) is determined** as α(s) = max(0, 1 − s/2)
   on s ∈ [0, ∞) under the §2.1 hypothesis (well-prepared isentropic
   compressible NS on T³, viscosity ν > 0, time interval [0, T]).
2. **The smallest open piece selected by the dispatch (Lemma 1) has
   PASSED** with literature-pinned values α(0) = 1, α(1) = 1/2,
   Δα = 1/2.
3. **The next probe (Lemma 2) at s = 2 has resolved the D2 vs D3
   binary** in favor of a linear-D3 hybrid: α(2) = 0 caps the rate
   at s* = 2, and there is **no further Sobolev rung within R5
   scope where α(s) > 0**.
4. **For s ∈ [0, 2) the rate is determined** by interpolation
   between the literature-pinned integer-s data points.
5. **For s ≥ 2 the rate is identically zero** — the R5 program has
   **nothing more to compute** for the rate function on this range.

### §3.3 R5 Lemma-level status

| component | status | basis |
|-----------|--------|-------|
| R5 acceptability | ACCEPTABLE | parent acceptability §3.4 |
| R5 conjecture (dispatch) | DOWNGRADED to linear-D3 hybrid | Lemma 2 §5.4 |
| R5 Lemma 1 (s = 0 vs s = 1 strict gap) | **PASS_LITERATURE** | Lemma 1 §4 |
| R5 Lemma 2 (α(2) extraction) | **D3_CONFIRMED at s* = 2** (literature-consensus) | Lemma 2 §4 |
| R5 main result | α(s) = max(0, 1 − s/2) on [0, 2], 0 on [2, ∞) | §2 |

### §3.4 R5 implication (analytic summary)

**Low-Mach compressible regularization cannot transfer H² (or
higher) regularity to incompressible NS** at any positive
ε-power rate. The R5-axiom strategy is **upper-Sobolev-bounded**
to s ∈ [0, 2): below H², the compressible family approaches
the incompressible limit at a quantitative rate ε^{1−s/2}; at H²
and above, the approach is qualitative only (uniformly bounded
but not o(ε^β) for any β > 0).

This is a **structural finding** about the compressible→incompressible
singular limit on T³ for well-prepared data, not a finding about
incompressible NS regularity itself.

---

## §4 Implications for BT-544 main line

### §4.1 Clay-statement scale of regularity

BT-544 main line, in the Clay Millennium statement (Fefferman
2000), requires **global C^∞ regularity** on [0, ∞) for smooth
divergence-free initial data on T³ (or ℝ³ with decay). The Clay
target is at the **C^∞ smoothness** scale, which corresponds to
arbitrarily high Sobolev index s.

R5's closed rate function α(s) = max(0, 1 − s/2) reaches **rate-
exponent zero at s = 2**, well below the C^∞ Clay scale.

### §4.2 R5 as partial auxiliary

R5 is an **auxiliary** program (parent acceptability §3.4 ACCEPTABLE
verdict, with auxiliary status preserved through the dispatch §5.4
and through both Lemmas). The closure of R5 reveals that this
auxiliary is **partial** in the following sense:

- **Useful at s = 0 (energy norm)**: α(0) = 1 gives a quantitative
  ε-rate at the L²-norm — a quantitative measure of how compressible
  approximation captures incompressible energy.
- **Useful at s = 1 (gradient norm)**: α(1) = 1/2 gives a
  quantitative ε^{1/2}-rate at the H¹-norm — a quantitative measure
  of compressible approximation at the velocity-gradient level.
- **Inadequate at s ≥ 2**: α(s) = 0 means no quantitative rate is
  available; the compressible approximation is qualitatively
  faithful (uniformly bounded) but not quantitatively faithful at
  these higher Sobolev orders.

For BT-544's Clay-grade global smoothness target, the
**high-Sobolev** content is essential (smoothness on [0, ∞)
requires control at all Sobolev orders simultaneously). R5
provides **no quantitative input** at this regime.

### §4.3 BT-544 status after R5 closure

**BT-544 Clay-status: 0/1, unchanged.**

R5 closure does **not** advance BT-544 toward Clay resolution; it
**clarifies** that the Low-Mach axiom-recast strategy cannot
substitute for the Clay-target high-Sobolev / smoothness control.
The R5-axiom recast was *acceptable* as an auxiliary problem with
its own analytic content, and that content is now closed: the rate
function is determined on its meaningful range, and it does not
extend to the Clay regime.

The Millennium tally remains **0/7**.

### §4.4 Structural reading

The R5 program reaches a clean answer that *bounds* its own
applicability. This is a **negative-direction structural result**:
the Low-Mach approximation is a useful auxiliary at low Sobolev
orders but does not reach the Clay regime. It does not refute Clay
BT-544; it eliminates one strategy (the R5-axiom recast at high s)
from the auxiliary toolkit for transferring high-Sobolev regularity
via the singular limit.

---

## §5 Comparison with R1 program

### §5.1 Independent axes

R1 and R5 live in **different ε-vs-α-vs-Hölder spaces** and are
mathematically independent:

- **R5 axis**: ε → 0 (Mach number) at fixed ν > 0. Recasts
  axiom A6 (incompressibility) into a 1-parameter family with
  ε ∈ (0, ε₀]. Produces invariant α(s) = uniform-in-ε rate
  function.
- **R1 axis**: α → 1/3 (or generally, Hölder exponent) at fixed
  ν > 0. Recasts axiom A2 (regularity class) into a 1-parameter
  family with α ∈ [0, 1]. Produces invariant α*_NS = Hölder-
  uniqueness threshold.

### §5.2 Notation collision (managed)

Both programs use the symbol α, but for **distinct objects**:

| program | symbol α refers to |
|---------|--------------------|
| R5 | the rate exponent function α(s): s ↦ ε-power for the L^∞_t H^s_x norm of u_ε − u₀ |
| R1 | the Hölder regularity exponent α; α*_NS is the threshold separating uniqueness from non-uniqueness |

Per Lemma 1 §5.4 R1↔R5 confusion guard: this report uses α(s)
(with explicit s argument) for R5, and α*_NS for R1.

### §5.3 R5 closure does not affect R1

The R5 closure declared in §3 is **silent on R1**:

- R5 closure is the determination of α(s) on the Mach-number axis;
- R1's open piece is the determination of α*_NS on the Hölder axis;
- Lemma 1 §5.3 enumerates the joint outcome scenarios; R5 PASS +
  R1 status independent (rows 1 or 3 of the Lemma 1 §5.3 table,
  depending on R1's eventual outcome).

### §5.4 R1 status (snapshot)

R1 Lemma 1 (BV-2019 viscous adaptation for the strict viscous gain
δ > 0 on the α*_NS ≤ 1/3 bound) is **mid-iteration** in the
n6-architecture corpus per the user-supplied context (retry-3
BCV-2021 in-flight). R1 is **not** closed; its open piece remains
the Hölder-threshold strict-gain question.

### §5.5 What this means for the BT-544 D2 line

After R5 closure:

- **R5 axis**: closed at the linear-D3 hybrid result.
- **R1 axis**: open, mid-iteration.
- **BT-544 D2 line attention** shifts to R1 fully (since R5 has
  no further internal probes within scope).

This is the natural progression from the parent-acceptability §4
ranking which placed R1 as the higher near-term priority and R5 as
the parallel second priority; with R5 now closed, R1 absorbs the
full D2-line attention.

---

## §6 Next priority — EXT-tier

### §6.1 Tier landscape

After R5 closure, the BT-544 program's open auxiliary work is
distributed across:

- **R1 (Onsager-Hölder)**: D2-line; mid-iteration on Lemma 1
  (BV-2019 viscous adaptation, retry-3 BCV-2021 in-flight); not
  closed.
- **EXT-tier** (extension auxiliaries): a separate batch of
  auxiliary approaches beyond the D2 axiom-recast frame; the
  highest-priority subcomponent per the user-supplied context is
  **EXT-B CI-Lyap** (in validation).
- **Other recasts** (R2, R3, R4, R6, R7, ...): not promoted from
  the parent seed; remain as background candidates.

### §6.2 EXT-B CI-Lyap

The user-supplied context indicates EXT-B CI-Lyap is the
**highest-priority next batch**, currently in validation. This
report does not perform EXT-B work; it records the priority
ordering.

### §6.3 Scheduling reading

With R5 closed and R1 mid-iteration, the BT-544 D2-line attention
naturally distributes between R1 (continuing the open piece) and
EXT-B CI-Lyap (validating the next batch). R5 takes no further
program time.

---

## §7 R5 unfinished items (explicitly nothing-more-to-do at s ≥ 2)

The R5 program has **no remaining items** within its dispatched
scope (well-prepared T³ at fixed [0, T], isentropic, viscous,
strong solutions). The following are **explicitly out of scope**
and **not** R5 unfinished items:

| candidate | status | reason out of R5 scope |
|-----------|--------|------------------------|
| determine α(s) for s ∈ (0, 1) ∪ (1, 2) at non-integer s | settled by interpolation between integer-s data points (§2.1 interpolation row); no further work needed | linear interpolation between literature-pinned points, no new analytic content |
| determine α(s) for s ≥ 3 (uniform boundedness vs rate-exponent distinction) | distinct question (Lemma 2 §6 follow-up "Lemma 3 candidate"); not a D2-vs-D3 question (settled by linear-D3 hybrid) | sharpness-of-s* / beyond-s* pathology question; the rate-exponent answer is already 0 |
| determine α(s) on ℝ³ (dispersion-lifted rates) | distinct domain (dispatch §6.1 caveat) | ℝ³ has Strichartz-type estimates that change the heuristic; separate problem |
| determine α(s) for ill-prepared data | distinct preparation regime (dispatch §6.3 caveat) | requires Schochet 1986 averaging; full-solution rate is not ε-small |
| determine α(s) for full (non-isentropic) NS | distinct system (dispatch §6.2 caveat) | thermal mode adds a separate ε-scaling |
| determine α(s) for global [0, ∞) (vs fixed [0, T]) | conjecturally Clay-equivalent (dispatch §6.5) | excluded from R5-Aux scope to prevent collapse-to-Clay |
| determine α(s) for weak solutions vs strong solutions | distinct solution class (dispatch §6.4 caveat) | strong-vs-weak rate analog is itself UNKNOWN |
| determine α(s) for bulk-viscosity λ replacing ε | excluded by parent seed §6 anti-list | not the canonical low-Mach problem |

These are all **separate auxiliaries** with distinct scope, *not*
unfinished pieces of R5. They could be dispatched as new programs
parallel to R5, but each would require its own seed → acceptability
→ dispatch chain.

**Within the R5 dispatched scope: nothing more to compute.**

---

## §8 Falsifiers active

The R5-program-closure declared here would be invalidated under the
following conditions. None are currently active.

- **F-Closure-A** (R5 closure invalidated if F-Lemma2-A escalates):
  if a careful reading of Métivier–Schochet 2001 / Danchin 2002 /
  Schochet 2007 / Feireisl–Novotný 2009 yields a *different* α(2)
  value as a sharp exponent (e.g. α(2) = 1/4 from a Besov-refinement
  Theorem not located here), the linear-D3 hybrid result must be
  revised, and the R5 closure declaration in §3 must be downgraded
  to "R5 closed contingent on literature-consensus reading of α(2)".
  The current literature is **consensus-consistent with α(2) = 0**
  but **not theorem-pinned**. **Status: not active; F-Lemma2-A
  watch-state inherited.**

- **F-Closure-B** (R5 closure invalidated if Lemma 1 retracted): if
  α(0) = 1 or α(1) = 1/2 is shown to be wrong (e.g. by a careful
  reading of Desjardins–Grenier 1999 / Danchin 2002 yielding
  different values), the closure result is revised. The α(0) = 1
  / α(1) = 1/2 values are **literature-stable** and cross-validated
  by multiple independent lineages. **Status: not active.**

- **F-Closure-C** (R5 closure invalidated if Clay-equivalence
  exposed): if the global-uniform-in-ε question (excluded from R5
  scope per dispatch §6.5) is shown to be **strictly equivalent**
  (rather than conjecturally equivalent) to Clay BT-544, then
  R5-Aux as defined collapses into Clay BT-544 and the
  "auxiliary" status is lost. The dispatch's [0, T] restriction
  protects against this; **status: not active**.

- **F-Closure-D** (atlas-touch): if this status report leads to any
  `shared/n6/atlas.n6`, `state/proposals/inventory.json`, or
  `theory/canon/` edit, the report has been mis-applied. This is a
  closure synthesis only. **Status: not active.**

- **F-Closure-E** (Clay-creep): if the R5-closure result is reported
  as advancing or hindering Clay BT-544, the auxiliary→main
  distinction has been lost. R5 is auxiliary; BT-544 stays 0/1.
  **Status: not active**; this report explicitly preserves the
  distinction.

- **F-Closure-F** (R1-R5 conflation): if R5 closure is reported as
  affecting R1 status (or vice versa), the program independence
  has been lost. **Status: not active**; §5 explicitly preserves
  independence.

- **F-Closure-G** (linear-form overcommitment): if α(s) = max(0,
  1 − s/2) is reported as the **proven** functional form on the
  full [0, ∞) interval without scope qualification, the report has
  been over-cited. The form is **literature-consensus on [0, 2]**
  and the rate-exponent-zero tail on [2, ∞) is the natural
  extension; uniform-boundedness questions for s ≥ 3 are out of
  scope and not part of the closure. **Status: not active under
  this report's framing**, flagged for downstream citation
  discipline (inherits F-Lemma2-C).

None of F-Closure-A..G fires under this report's scope.

---

## §9 Closing

**0/7 unchanged. R5 program closed. No atlas/state/inventory
edits.**

R5 main result: α(s) = max(0, 1 − s/2) on [0, 2], identically 0 on
[2, ∞) — a linear-D3 hybrid under literature-consensus reading.

R5 implication for BT-544: Low-Mach compressible regularization
cannot transfer H² (or higher) regularity to incompressible NS at
any positive ε-power rate. The R5-axiom strategy is upper-Sobolev-
bounded to s ∈ [0, 2). For BT-544's Clay-grade global smoothness
target (high-Sobolev/C^∞), R5 contributes no quantitative input;
R5 is therefore a partial auxiliary, useful for energy-norm
convergence (s = 0) and gradient (s = 1), but not for the Clay
regime.

R5 vs R1: independent axes (Mach-number rate vs Hölder regularity
threshold). R5 closure does not affect R1; R1 Lemma 1 remains
mid-iteration (retry-3 BCV-2021 in-flight per user-supplied
context).

Next priority: with R5 closed, BT-544 D2-line attention shifts to
R1 fully; the highest-priority next batch is EXT-tier, especially
EXT-B CI-Lyap (in validation).

Per F-544-B and the L_ω apex distinction reaffirmed across the
parent reports: this status synthesis closes an auxiliary research
program at the literature-consensus level; it does not modify the
Clay statement and does not advance the Clay-status. BT-544 remains
catalogue-exhausted at L9 and 0/1 untouched at the Clay level.

— end R5 program closure declaration —
