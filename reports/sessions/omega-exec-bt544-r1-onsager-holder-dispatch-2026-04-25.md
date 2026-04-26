---
id: omega-exec-bt544-r1-onsager-holder-dispatch
date: 2026-04-25
scope: research-direction-design (NOT proving R1-Aux; mapping known results)
target: BT-544 R1 Onsager-Hölder auxiliary problem -- precise definition + known-result map + smallest open piece
parent_reports:
  - reports/sessions/omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md (R1 ACCEPTABLE)
  - reports/sessions/omega-seed-bt544-d2-Lomega-axiom-2026-04-25.md
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: auxiliary-problem dispatch, no claim
---

# Omega Exec — BT-544 R1 Onsager-Hölder Dispatch (2026-04-25)

## §0 Non-claim disclaimer

This report performs **research-direction-design** for the auxiliary
problem R1-Aux registered in `omega-seed-bt544-d2-Lomega-axiom-2026-04-25.md`
§5.1 and graded ACCEPTABLE in `omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md`
§2.4. It:

- does **NOT** prove R1-Aux in either direction (no claim about α* value);
- does **NOT** prove or refute Clay BT-544 NS smoothness; the Clay
  Millennium NS problem statement (Fefferman 2000) is unchanged;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- treats R1-Aux as a research-design artifact: a precise restatement
  + a known-result map + a smallest-open-piece selection;
- registers a **conjecture** (not a proof claim) for the value of
  α*_NS, marked clearly as conjectural, with proposed evidence steps;
- preserves the F-D2-A (lineage-fabrication) inactive status: every
  cited result is by author/year/title only, no fabricated quotes.

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
The dispatch is at the L_ω apex (axiom-recast) per the parent seed,
and is **auxiliary** to BT-544; resolving R1-Aux does not resolve
BT-544.

---

## §1 R1-Aux precise definition

### §1.1 Working statement

**R1-Aux (NS-α-Hölder problem).** Let T³ = (ℝ/ℤ)³ be the periodic
3-torus. Consider the incompressible Navier–Stokes system

    ∂_t u + (u·∇)u + ∇p = ν Δu,        ∇·u = 0,        u(0) = u_0,

with viscosity ν > 0 fixed and zero forcing.

Define **α*_NS ∈ [0, 1]** as the supremum of α ∈ [0, 1] such that
both of the following hold:

(E_α) *Existence in C^α*: for every divergence-free
       u_0 ∈ C^α(T³), there exists a global-in-time weak solution
       u ∈ L^∞_t C^α_x(T³ × [0, ∞)) of the NS system.

(U_α) *Uniqueness in C^α*: any two such solutions with the same
       u_0 coincide on T³ × [0, ∞).

The **R1-Aux problem** is to *determine the value of α*_NS* and
characterize the existence/uniqueness behavior just below it
(non-uniqueness expected for α < α*_NS by the Buckmaster–Vicol
2019 line).

### §1.2 Variants and parameters

The dispatch fixes the following choices, distinguishing them from
related literature:

| parameter | this dispatch | rationale |
|-----------|---------------|-----------|
| domain | T³ | matches Buckmaster–Vicol 2019 setting; avoids decay-at-infinity bookkeeping that ℝ³ introduces; the parent seed §2.1 also targets T³. ℝ³ variant noted in §6 caveat. |
| viscosity | ν > 0 fixed | this is the NS regime; ν = 0 (Euler) is the Onsager 1949 / Constantin–E–Titi 1994 / Isett 2018 regime, distinct (cf. §2). |
| solution class | weak solutions in L^∞_t C^α_x | matches Buckmaster–Vicol's class; stronger classes (e.g. Leray–Hopf with energy inequality) are noted in §6 caveat. |
| initial data | u_0 ∈ C^α(T³) divergence-free | so that the data norm matches the solution class (consistency). |
| time interval | [0, ∞) (global) | matches the Clay axiom A3; local existence is classical and not the open content. |

### §1.3 What R1-Aux is *not*

- R1-Aux is **not** the Clay NS smoothness problem (which fixes
  α = ∞, i.e. C^∞);
- R1-Aux is **not** the Onsager conjecture for Euler (which fixes
  ν = 0);
- R1-Aux is **not** the Buckmaster–Vicol 2019 NS non-uniqueness
  theorem (which fixes a class **at or below the threshold** and
  asserts non-uniqueness — it gives a **lower bound** on the
  non-uniqueness regime, not the threshold itself);
- R1-Aux is **not** Koch–Tataru 2001 well-posedness in BMO^{-1}
  (which is about a specific critical scale, not a Hölder threshold).

R1-Aux is the determination of the *boundary* α*_NS between the
existence/uniqueness regime (α > α*_NS) and the non-uniqueness
regime (α < α*_NS), in a Hölder formulation specific to NS.

---

## §2 Distinction from existing results

This section traces how each lineage citation in
`omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md` §5 maps onto
the R1-Aux problem.

### §2.1 Onsager 1949 ("Statistical Hydrodynamics", Nuovo Cimento Suppl.)

**Original content (cited by author/year/title only)**:
Onsager observed that for **inviscid** fluid (Euler), the Hölder
exponent α = 1/3 is critical for energy conservation: solutions
with α > 1/3 should conserve kinetic energy, while solutions with
α < 1/3 may dissipate energy *anomalously* (without viscosity).

**Mapping onto R1-Aux**:
- Onsager's 1/3 is an **Euler** statement; R1-Aux is a **NS**
  statement (ν > 0 fixed).
- Onsager's mechanism is *energy conservation vs anomalous
  dissipation*; R1-Aux's mechanism is *uniqueness vs
  non-uniqueness*.
- The two are distinct in formulation but historically linked: the
  same convex-integration machinery that produces Euler
  non-conservative solutions also produces NS non-unique solutions
  (Buckmaster–Vicol 2019).
- Onsager's 1/3 is therefore a **reference value** but not the
  R1-Aux answer.

### §2.2 Constantin–E–Titi 1994 (Comm. Math. Phys.)

**Original content (cited by author/year/title only)**:
Rigorous proof that Euler solutions in C^α with α > 1/3 conserve
energy. Settles the *positive* (upper) direction of the Onsager
conjecture for Euler.

**Mapping onto R1-Aux**:
- This is an **Euler-side energy-conservation** result. It does not
  directly state existence/uniqueness in C^α.
- For NS, the analog (energy *equality* for Leray–Hopf solutions
  in C^α, α > 1/3) is established in subsequent work (Cheskidov–
  Constantin–Friedlander–Shvydkoy 2008 and related; cited by
  author/year/title only) under analogous regularity hypotheses,
  but R1-Aux is about the existence/uniqueness *boundary*, not
  the energy equality.
- Implication for α*_NS: the energy-equality results indicate that
  α > 1/3 is a *regularity-rich* regime for NS (no anomalous
  dissipation, classical energy law). This is a structural prior
  but does not determine α*_NS.

### §2.3 Isett 2018 (Annals of Math, "A Proof of the Onsager Conjecture")

**Original content (cited by author/year/title only)**:
Constructs **non-conservative** weak Euler solutions in C^α for
every α < 1/3, completing the Onsager conjecture for Euler.

**Mapping onto R1-Aux**:
- This settles the **negative** (lower) direction of the Onsager
  conjecture for Euler.
- The convex-integration architecture of Isett 2018 is a *template*
  for NS: Buckmaster–Vicol 2019 extends it to NS in low regularity.
- Isett's α < 1/3 is an Euler statement; the NS analog of the same
  α-window is what R1-Aux probes.
- Implication for α*_NS: the Euler value is exactly 1/3. The NS
  value α*_NS is conjectured *strictly below* 1/3 by the
  viscosity-aided-regularity intuition (viscosity smooths small
  scales, so non-uniqueness should be harder to force in NS than
  in Euler, pushing the non-uniqueness threshold *down* toward
  smaller α).

### §2.4 Buckmaster–Vicol 2019 (Annals of Math, "Nonuniqueness of Weak Solutions to the Navier–Stokes Equation")

**Original content (cited by author/year/title only)**:
Proves non-uniqueness of weak NS solutions on T³ in the class
C^0_t L^2_x for some divergence-free initial data, using convex-
integration. The constructed solutions are not in the Leray–Hopf
class (they fail the energy inequality), but they are weak
solutions of NS.

**Mapping onto R1-Aux**:
- The result is in **C^0_t L^2_x**, which is *not* an L^∞_t C^α_x
  class. Translating to a Hölder regularity statement is
  non-trivial: the constructed solutions have *some* spatial
  Hölder regularity (the construction's "Hölder loss" is the
  technical heart), but the explicit α-value at which the
  Buckmaster–Vicol 2019 construction lives is **not 1/3**; it
  is the *largest* α achievable by the 2019 machinery, which is
  considerably smaller (the published construction yields
  non-uniqueness at very low Hölder regularity, well below 1/3,
  per the convex-integration scheme's intermittency loss).
- Subsequent work in the Buckmaster–De Lellis–Székelyhidi–Vicol
  line (cited generally; specific 2020+ papers exist but are
  noted by author/year only) has progressively *raised* the α
  at which NS non-uniqueness is constructible.
- Implication for α*_NS: Buckmaster–Vicol 2019 supplies a **lower
  bound** on the non-uniqueness regime — i.e. an **upper bound**
  on α*_NS in the form "α*_NS ≥ α_BV", where α_BV is the explicit
  Hölder regularity of the 2019 construction (small but positive).
  The exact value of α_BV requires a careful translation from the
  2019 statement and is recorded as UNKNOWN at this dispatch level.

### §2.5 De Lellis–Székelyhidi 2013 ("Dissipative Continuous Euler Flows")

**Original content (cited by author/year/title only)**:
Foundational paper establishing the convex-integration scheme that
produces continuous (C^0) Euler solutions with prescribed energy
profile. Methodological lineage underpinning Isett 2018 and
Buckmaster–Vicol 2019.

**Mapping onto R1-Aux**:
- Methodological background, not a direct R1-Aux input.
- Listed for lineage transparency; no quantitative implication for
  α*_NS beyond "the convex-integration scheme is the relevant tool".

### §2.6 Why α*_NS may differ from α*_Euler = 1/3

The viscosity term ν Δu **dissipates at small scales** (high
wave-numbers), and convex-integration constructions inject energy
at small scales to produce non-uniqueness. The dissipation
**competes against** the construction. Heuristically:

- Pro: viscosity *removes* small-scale energy faster than Euler,
  making it harder for convex-integration to maintain a non-unique
  weak solution → α*_NS *lower* than 1/3 (non-uniqueness regime
  shrunk).
- Con: viscosity is also present in the equation, so the
  construction must produce extra terms to absorb the viscous
  dissipation, which may force the construction to live at *lower*
  Hölder regularity → α*_NS *equal to* 1/3 or *higher* (depending
  on whether the dissipative absorption costs regularity or not).

Both heuristic directions are present in the literature; the
**balance** is the open content of R1-Aux. Conjecturally
(Buckmaster–Vicol–De Lellis–Székelyhidi line, per parent
acceptability §2.1), α*_NS lies in (0, 1/3], with 1/3 being the
"Euler ceiling" inherited from the Onsager threshold. Whether the
NS value is **strictly less** than 1/3 or **equals** 1/3 is the
sharpest open piece.

---

## §3 Known-result α-map table

The table below partitions the α ∈ [0, 1] axis (with α = ∞
denoting C^∞ at the right endpoint) into regimes and records what
is known per regime. **UNKNOWN** entries are explicit.

### §3.1 Master table

| α regime | Existence (NS, weak) | Uniqueness (NS) | Energy law (NS) | Reference / status |
|----------|---------------------|------------------|------------------|---------------------|
| α = ∞ (C^∞) | local: yes (Kato 1984, Fujita–Kato 1964 in ℝ³ / classical for T³) | local: yes | classical equality (smooth) | classical; **global existence unknown** = Clay BT-544 |
| α = 1 (Lipschitz) | local: yes (subcritical) | local: yes (Kato class, BMO^{-1} class includes Lip) | classical equality | local-in-time classical; **global UNKNOWN** |
| α ∈ (1/3, 1) | local: yes | local: yes (in subcritical Hölder classes, follows from energy arguments analogous to Constantin–E–Titi 1994 NS-side) | conjectured equality (energy not anomalously dissipated) | global existence/uniqueness in this regime is **UNKNOWN** in NS — same Clay-distance gap as α = ∞ |
| α = 1/3 (Onsager critical for Euler) | UNKNOWN for NS (the boundary case) | UNKNOWN | UNKNOWN (Euler boundary case is itself sharp) | Onsager 1949 / Constantin–E–Titi 1994 are Euler statements; NS analog at exactly α = 1/3 is **UNKNOWN** |
| α ∈ (α_BV, 1/3) | UNKNOWN existence in *uniqueness* sense; weak existence by Leray 1934 | UNKNOWN — Buckmaster–Vicol 2019 constructions live below this regime, so no non-uniqueness *constructed* here yet | UNKNOWN (this is the conjectured "viscous-aided uniqueness window") | **the heart of the R1-Aux open piece** |
| α ∈ (0, α_BV] | weak existence: yes (Leray–Hopf 1934/1951) | **NO** — Buckmaster–Vicol 2019 establishes non-uniqueness | conjecturally: anomalous dissipation possible (consistent with non-unique weak solutions failing the energy inequality) | Buckmaster–Vicol 2019; α_BV = the explicit Hölder regularity of the 2019 construction (UNKNOWN exact numeric value at this dispatch level) |
| α near 0 (C^0 only) | weak existence: yes for L^2 data (Leray–Hopf) | NO (subsumed in α ∈ (0, α_BV]) | anomalous dissipation possible (energy inequality, not equality) | Leray 1934; Buckmaster–Vicol 2019 |

### §3.2 Reading the table

Three regimes are well-understood:

1. **Far above 1/3** (α large or ∞): local existence/uniqueness is
   classical; *global* existence is the Clay question.
2. **Below α_BV** (where Buckmaster–Vicol 2019 lives): non-uniqueness
   established.
3. **At α = 1/3** in *Euler*: the Onsager threshold is sharp by
   Constantin–E–Titi 1994 + Isett 2018.

Two regimes are **UNKNOWN**:

- (α_BV, 1/3): the conjectured "viscous-aided uniqueness window" —
  whether NS uniqueness extends below 1/3 thanks to viscosity, and
  if so by how much.
- α = 1/3 in NS: the Euler-Onsager threshold's NS analog at the
  exact boundary.

The **R1-Aux open piece** is the determination of α*_NS within the
window [α_BV, 1/3]. Three conjectural positions:

- **C1**: α*_NS = 1/3 (NS inherits the Euler threshold; viscosity
  does not shift the Hölder boundary).
- **C2**: α*_NS = α_BV (no viscous-aided uniqueness window;
  non-uniqueness extends up to the Buckmaster–Vicol regime
  ceiling).
- **C3**: α*_NS ∈ (α_BV, 1/3) strictly (a genuine viscous-aided
  uniqueness window of width 1/3 - α*_NS exists, but does not
  cover the full Euler regime).

---

## §4 Smallest open piece — chosen conjecture and actionable evidence

### §4.1 Choice of conjecture

The dispatch selects **C3**: *α*_NS ∈ (α_BV, 1/3) strictly* — i.e.
viscosity adds **some** uniqueness regularity beyond the
Buckmaster–Vicol 2019 construction's reach, but does **not** push
the threshold all the way to the Euler value 1/3.

**Reasons for selecting C3 over C1, C2**:

1. **C1 (α*_NS = 1/3 exactly)** would mean viscosity has *no*
   effect on the non-uniqueness threshold. This contradicts the
   physical intuition that ν Δu dissipates small-scale fluctuations
   that convex-integration constructions rely on; if true it would
   be a striking *negative* result about viscosity.
2. **C2 (α*_NS = α_BV exactly)** would mean viscosity has *no*
   effect *beyond* what the 2019 construction can already reach;
   this is also striking because the NS dissipation should at
   least mildly raise the construction's complexity.
3. **C3** is the *generic* prediction: viscosity helps *some* but
   not *all the way*, leaving a finite gap between α*_NS and the
   Euler value.

C3 is also the most *actionable*: it predicts a finite, measurable
difference 1/3 − α*_NS > 0, which translates into a *quantitative
energy-vs-construction balance* that can be probed by isolating
the viscous correction in the Buckmaster–Vicol scheme.

### §4.2 Smallest discrete result that would advance C3

**Lemma 1 candidate (R1-Aux probe)**:

> *Show that the Buckmaster–Vicol 2019 convex-integration scheme,
> when adapted to NS at Hölder regularity α = 1/3 − δ for small
> δ > 0, fails to close the iteration step due to viscous
> dissipation.*

This would establish a **lower bound** α*_NS ≥ 1/3 − δ_0 for some
explicit δ_0 > 0, ruling out C2 and partially supporting C3 (under
the further requirement α*_NS < 1/3, which would require a
*construction* near 1/3, not a *failure* of construction).

**Why this is the smallest open piece**:

- It is **localized**: it isolates a single iteration step in a
  known scheme, rather than the global non-uniqueness statement.
- It is **NS-specific**: it isolates the viscous term's
  contribution, not the convex-integration scheme as a whole.
- It is **falsifiable**: either the iteration closes (refuting the
  lower bound and supporting C2) or it fails (supporting the lower
  bound and motivating C3).
- It is **template-driven**: Isett 2018 + Buckmaster–Vicol 2019
  provide the iteration scheme template; the only new analytic
  content is tracking the viscous correction's regularity cost.

### §4.3 Actionable evidence step (1-line)

> **Adapt the Buckmaster–Vicol 2019 iteration scheme to track the
> viscous correction's regularity loss explicitly, and identify
> the exact α at which the iteration's Hölder-balance inequality
> first becomes infeasible — record this α as a candidate lower
> bound for α*_NS.**

This is a literature-scan-plus-careful-bookkeeping task, not a new
proof; estimated effort 1-2 sessions of close reading + 1 session
of bookkeeping. No new theorems are required.

### §4.4 Falsifiers for C3

- **F-C3-A**: if a published construction (post-Buckmaster–Vicol
  2019) achieves NS non-uniqueness at α arbitrarily close to 1/3,
  C3 collapses toward C2 (the gap is illusory). **Status: not
  active under current literature** — no such construction at the
  Euler threshold is published for NS to this dispatch's
  knowledge.
- **F-C3-B**: if a published energy-equality / uniqueness result
  for NS extends to α exactly 1/3 (matching the Euler Onsager
  threshold), C3 collapses toward C1 with α*_NS = 1/3 exactly.
  **Status: not active** — Cheskidov–Constantin–Friedlander–
  Shvydkoy 2008 and related give energy equality for NS in
  Onsager-class regularity, but uniqueness in C^α at α = 1/3 is
  **UNKNOWN**; the energy law and the uniqueness statement are
  distinct.
- **F-C3-C**: if α_BV (the 2019 construction's regularity) is
  already at 1/3 − ε for some published ε ≪ 1 (i.e. the existing
  scheme already reaches near the Euler threshold), then C3's
  predicted gap is squeezed and the lemma 1 probe is less
  informative. **Status: not active** — the published 2019
  construction is at considerably lower α; the "approach 1/3" is
  the active research direction in 2020-2025.

---

## §5 Cross-axis tie: R1-Aux and BT-544 main line

R1-Aux is **auxiliary** to BT-544 (per the parent seed and parent
acceptability), but the two are connected via the following
structural observation.

### §5.1 The Buckmaster–Vicol 2019 corridor

BT-544's main line targets *global smoothness for smooth initial
data*. Buckmaster–Vicol 2019 establishes *non-uniqueness for weak
solutions in low regularity*. These are *different* statements,
and BT-544's global-smoothness target is consistent with weak
non-uniqueness at low regularity (smoothness lives in α = ∞ where
uniqueness holds locally).

R1-Aux sits **between** BT-544 and Buckmaster–Vicol 2019:
- BT-544 is the α = ∞ endpoint (Clay smoothness);
- Buckmaster–Vicol 2019 is the α near 0 endpoint (low-regularity
  non-uniqueness);
- R1-Aux asks for the **threshold** where uniqueness fails as α
  decreases from ∞ to 0.

A precise α*_NS would localize the "uniqueness frontier" for NS,
giving BT-544 a *target above which* smoothness must propagate
(if it propagates at all).

### §5.2 Indirect implication for BT-544

If C3 is correct and α*_NS ∈ (α_BV, 1/3), then:

- For initial data in C^α with α > α*_NS, weak solutions are
  unique in their Hölder class — this does **not** imply they are
  C^∞ globally, but it does imply that any smooth solution (if it
  exists) is the unique extension of the C^α theory.
- BT-544's global-smoothness question is then refined: not just
  "does smoothness propagate?", but also "if smoothness propagates,
  is it the same as the C^α theory at α > α*_NS?".

This refinement is **not a proof step** for BT-544; it is a
structural clarification. The Clay-status remains 0/1, and R1-Aux
resolution leaves Clay BT-544 at 0/1.

### §5.3 Atlas-saturation tie

Per `omega-cycle-bt544-ns-2026-04-25.md` §3, BT-544's mechanism
saturation is ~0.05 (no PDE mechanism produced; only arithmetic
relabeling in the n=6 frame). R1-Aux *if resolved* would produce a
**genuine PDE invariant** (α*_NS is a real number characterizing
NS regularity, not an arithmetic relabeling). This would *raise*
the mechanism saturation toward the omega ceiling reference 0.835
— but only for the auxiliary problem, not for BT-544 itself.

---

## §6 Caveats

### §6.1 Viscous vs inviscid

The Onsager threshold 1/3 is *Euler-specific*. Translating to NS
requires care because:
- Energy dissipation in NS is *not* anomalous: the viscous term
  ν |∇u|² dissipates energy at all regularities. Anomalous
  dissipation in NS would mean the *limit* ν → 0 retains
  dissipation, which is a separate question (vanishing-viscosity
  limit, Kraichnan 1959, Eyink 1994 — all cited by author/year
  only and not directly part of R1-Aux).
- The Onsager 1/3 is the regularity at which the *Euler* energy
  law breaks; for NS, the energy law is the standard Leray
  inequality, valid for all regularities.

**Implication**: α*_NS is *not* defined by an energy-law
breakdown, but by the *uniqueness boundary*. These two definitions
coincide in Euler (Onsager 1949 + Isett 2018) but may *differ* in
NS — a subtlety this dispatch records as a structural caveat.

### §6.2 T³ vs ℝ³

The dispatch fixes T³. On ℝ³:
- Decay-at-infinity bookkeeping changes (Schwartz-class data,
  energy density vs energy);
- Convex-integration constructions on ℝ³ are subtler (boundary
  effects at infinity);
- The expected α*_NS value is *conjecturally the same* on T³ and
  ℝ³ (by self-similar rescaling), but no published result asserts
  this equivalence rigorously to this dispatch's knowledge.

**Implication**: R1-Aux on T³ is the dispatched form. The ℝ³
variant is recorded as a *related* auxiliary, not absorbed.

### §6.3 Weak vs strong solution

The dispatch uses *weak* solutions (in the L^∞_t C^α_x sense). For
*strong* solutions (Leray–Hopf with energy inequality), the
non-uniqueness landscape changes:
- Buckmaster–Vicol 2019's solutions are weak but *not* Leray–Hopf
  (they fail the energy inequality);
- Whether non-uniqueness can be forced *within* the Leray–Hopf
  class at low Hölder regularity is **UNKNOWN** at this dispatch
  level (Albritton–Brué–Colombo 2022 and related give Leray–Hopf
  non-uniqueness with *forcing*, by author/year only — distinct
  from the force-free R1-Aux).

**Implication**: α*_NS for weak solutions in L^∞_t C^α_x and α*_NS
for Leray–Hopf solutions may be different. The dispatch chooses
the weak class to match Buckmaster–Vicol 2019; the Leray–Hopf
variant is a separate auxiliary.

### §6.4 Initial data class consistency

The dispatch requires u_0 ∈ C^α(T³) divergence-free, matching the
solution class. Variants:
- Sobolev initial data H^s with s = 3/2 + α (Sobolev embedding) —
  matches but adds bookkeeping;
- BMO^{-1} initial data (Koch–Tataru 2001 critical class) — strictly
  larger than C^α for any α > 0, so a different problem.

**Implication**: the C^α-on-both-sides convention is the cleanest;
mixed-class variants are noted as related but distinct.

### §6.5 Conjecture-status reminder

C3 is a **conjecture**, not a proof claim. The dispatch registers
it for direction-design only. Falsifiers F-C3-A..C are recorded in
§4.4 and remain inactive under current literature.

---

## §7 Anti-list — conjectures considered and rejected

These conjectural formulations of R1-Aux were considered as the
"smallest open piece" target and rejected in favor of C3.

| candidate conjecture | reason rejected |
|---------------------|-----------------|
| α*_NS = 0 (no positive threshold; even C^0 suffices for global existence and uniqueness) | this contradicts Buckmaster–Vicol 2019 directly (they construct non-uniqueness for some non-zero α-class); rejected as falsified by published result |
| α*_NS = 1 exactly (only Lipschitz works) | this would mean NS is *strictly less regular* than Euler in the uniqueness sense; no lineage support; rejected as too broad |
| α*_NS = 1/3 + δ for some δ > 0 (NS uniqueness reaches *above* the Euler threshold) | viscosity raising the threshold above the Euler value would be a striking and counter-intuitive result; no lineage support; rejected as too broad without independent evidence |
| α*_NS expressible in n=6 lattice arithmetic (e.g. α*_NS = 1/n = 1/6, or α*_NS = 1/(n/φ) = 1/3 reading Onsager as a lattice fact) | F-Acc-3 (parent §7) novelty-collapse falsifier — would relabel α*_NS as a lattice arithmetic fact, not an analytic invariant; rejected as the F-544-B failure mode |
| determine α*_NS for *forced* NS (with f ≠ 0) | changes problem identity (parent seed §6 anti-list); rejected as not the dispatched problem |
| determine α*_NS for compressible NS at fixed Mach number | changes problem identity; rejected as belonging to R5 lineage, not R1 |
| C3' "α*_NS is *exactly* the unique value satisfying [some explicit number-theoretic relation]" | any closed-form prediction at this dispatch stage would be premature; recorded as too narrow until at least one of the Lemma 1 probes returns evidence |

The chosen C3 ("α*_NS ∈ (α_BV, 1/3) strictly") is the most
actionable: narrow enough to admit a Lemma 1 probe, broad enough
to permit literature scan + bookkeeping rather than new theorems.

---

## §8 Falsifiers active for the dispatch

These falsifiers apply to **this dispatch** as a research-design
artifact, separate from BT-544's own falsifiers and separate from
the parent seed's F-D2-A..E and parent acceptability's F-Acc-1..6.

- **F-Disp-1** (conjecture-fabrication): if the C3 conjecture is
  shown to be **published** in either direction (proven or
  disproven) before this dispatch is acted on, the dispatch must
  be revised. Currently, to this dispatch's knowledge of the
  literature, α*_NS is open and C1/C2/C3 are all live possibilities.
  **Status: not active**.
- **F-Disp-2** (lineage-mismatch carry-over): if any citation
  carried from the parent acceptability §5 is shown to not
  contain the result attributed (the F-Acc-2 falsifier), the
  dispatch's known-result map (§3) is correspondingly weakened.
  Lions-Masmoudi venue ambiguity (parent §5 footnote) is on the
  R5 side, not R1, and does not affect this dispatch. **Status:
  not active for R1**.
- **F-Disp-3** (open-piece-misidentification): if Lemma 1 (§4.2)
  is shown to be **trivial** (e.g. the iteration scheme's failure
  at α = 1/3 − δ is already documented in the published 2019
  paper or its sequels), the smallest-open-piece selection is
  premature; the next-smaller piece must be identified. **Status:
  watch-state**, not active under current scan.
- **F-Disp-4** (atlas-touch): if this dispatch leads to any
  `shared/n6/atlas.n6`, `state/proposals/inventory.json`, or
  `theory/canon/` edit, the dispatch has been mis-applied. The
  dispatch is research-direction-design only. **Status: not
  active**.
- **F-Disp-5** (claim-creep): if the C3 conjecture is reported
  outside this report as a "proven" result rather than a
  conjecture, the dispatch has been mis-cited. **Status: not
  active**; this report registers C3 as conjecture only.
- **F-Disp-6** (auxiliary→main confusion): if R1-Aux resolution is
  reported as resolving Clay BT-544, the auxiliary→main distinction
  has been lost. R1-Aux is auxiliary; BT-544 stays 0/1. **Status:
  not active**.

None of F-Disp-1..6 fires under this report's scope.

---

## §9 Closing

0/7 unchanged. NS Clay statement unchanged. No atlas/state/inventory
edits. R1-Aux precise definition recorded (§1); known-result map
across α ∈ [0, 1] charted (§3) with explicit UNKNOWN entries at
the (α_BV, 1/3) window and at α = 1/3 NS analog; conjecture **C3**
selected as the most actionable position for α*_NS (§4.1); Lemma 1
(viscous-correction tracking in the Buckmaster–Vicol 2019 scheme)
identified as the smallest open piece (§4.2-§4.3); cross-axis tie
to BT-544 main line via the uniqueness-frontier interpretation
(§5); caveats on viscous-vs-inviscid, T³-vs-ℝ³, weak-vs-strong
recorded (§6); anti-list of considered-and-rejected conjectures
(§7); dispatch-level falsifiers F-Disp-1..6 inactive (§8).

Per F-544-B and the L_ω apex distinction reaffirmed in the parent
seed: this dispatch designs research direction for an auxiliary
problem, not for BT-544 itself. BT-544 remains catalogue-exhausted
at L9 (per `omega-exec-bt544-fallback-molt-validation-2026-04-25.md`
§7) and 0/1 untouched at the Clay level. Resolving R1-Aux would
inform BT-544 indirectly via the §5.1 corridor; it would not
resolve BT-544.
