---
id: omega-exec-bt544-r1-lemma1-level4-cet
date: 2026-04-25
scope: research-only Lemma 1 Level 4 attempt (NOT a theorem; sketch via CET 1994)
target: BT-544 R1 Lemma 1 Level 4 -- ν→0 Onsager-contradiction route
parent_reports:
  - reports/sessions/omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md (L1)
  - reports/sessions/omega-exec-bt544-r1-lemma1-retry-mu_q-2026-04-25.md (L2)
  - reports/sessions/omega-exec-bt544-r1-lemma1-retry3-bcv2021-2026-04-25.md (L3)
  - reports/sessions/omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md (C3 conjecture)
millennium_resolved: 0/7 (unchanged)
grade: lemma sketch retry-4 via contradiction route, no claim
---

# Omega Exec — BT-544 R1 Lemma 1 Level 4 ν→0 Onsager-Contradiction Route (2026-04-25)

## §0 Non-claim disclaimer

This report is the **fourth attempt** (Level 4) of the R1-Aux
Lemma 1 sketch. The prior three attempts all returned
**OBSTRUCTION_DOCUMENTED** at successively deeper levels:

- L1 (`omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md`):
  surface — Phase 3 inequality (3'') self-cancels at β = α; missing
  intermittency parameter μ_q.
- L2 (`omega-exec-bt544-r1-lemma1-retry-mu_q-2026-04-25.md`):
  deeper — μ_q tracked, but BV-2019 intermittent jets are L²-class,
  wrong building block for Hölder regularity (forces α ≥ 1, vacuous).
- L3 (`omega-exec-bt544-r1-lemma1-retry3-bcv2021-2026-04-25.md`):
  deepest schematic — BCV-2021 reframe also infeasible at α_BCV;
  schematic Hölder-balance template missing the temporal-singular-
  set leg (B4').

The recommended Level-4 unblocking technique was L3 §8.2 #3:

> "ν → 0 Onsager-contradiction (preserved from attempt 1, §7.2 #3):
> prove Lemma 1 by *indirect* contradiction — show that any C^α NS
> non-uniqueness at α near 1/3 would, in the ν → 0 limit,
> contradict Constantin–E–Titi 1994's Euler energy-conservation
> threshold."

This Level 4 attempt executes that route. It is **a research-
direction sketch retry-4**, not a theorem. It:

- does **NOT** prove Lemma 1 — verdict in §6;
- does **NOT** prove or refute any direction of α*_NS;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- does **NOT** claim Constantin–E–Titi 1994 says anything beyond
  what is published; the CET 1994 statement is recalled by
  author/year/journal/theorem only, with no quoted text or
  fabricated constants;
- does **NOT** claim Isett 2018 says anything beyond what is
  published;
- does **NOT** make any anomalous-dissipation / Eyink 1994 /
  Kraichnan 1959 / Duchon–Robert 2000 claims beyond their published
  scope;
- preserves F-Disp-1..6 (parent), F-Sk-1..6 (L1), F-Sk-Mu-1..6 (L2),
  F-Sk-BCV-1..6 (L3) inactive, plus introduces F-Sk-CET-1..6
  specific to this Level 4 attempt (§10).

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
This Level 4 attempt is auxiliary to the auxiliary problem R1-Aux;
even a PASS_SKETCH_REFINES_C3 verdict would not move BT-544.

---

## §1 L1–L3 cumulative obstruction recap

### §1.1 Three-layer schematic-template inadequacy

The three prior attempts have established a coherent diagnosis of
**why the schematic Hölder-balance approach fails for R1-Aux**:

| L | obstruction | depth | sanity at native α | α* expression |
|---|-------------|-------|---------------------|----------------|
| 1 | missing μ_q (log vs power-law mismatch) | surface | FAILS at α_BV = 0 | none |
| 2 | wrong building-block class (intermittent jets ↔ Hölder) | deeper | PASSES at α_BV (consistent with BV-2019 L²); FAILS for Hölder α < 1 | α*_{BV,Hölder} ≥ 1, vacuous |
| 3 | missing temporal-singular-set leg (B4') | deepest schematic | FAILS at α_BCV | none, empty α-window |

The cumulative diagnosis: **the convex-integration re-derivation
route, at the schematic Hölder-balance level, is structurally
inadequate**. Each retry has refined the diagnosis (μ_q absent →
wrong building block → wrong inequality system topology), but no
useful α* lower bound has emerged.

### §1.2 Why a different route is warranted

L3 §8.2 #3 explicitly recommended dropping the convex-integration
re-derivation in favor of a **contradiction-by-limit** strategy
that leverages CET 1994's Euler-side rigidity. The motivation:

- The Euler threshold α = 1/3 is **rigorously established** (CET
  1994 above; Isett 2018 below). It is *not* a conjecture; both
  bounds are theorems.
- If any NS non-uniqueness construction at α < 1/3 (say α =
  α*_NS − δ with α*_NS conjecturally < 1/3 per C3) exists *and*
  is uniform in ν, the limit ν → 0 should give an Euler weak
  solution with the same Hölder regularity α*_NS − δ.
- For α*_NS − δ < 1/3, the Euler limit is in the Isett 2018 regime
  (where energy non-conservation is possible). For α*_NS − δ >
  1/3, the Euler limit is in the CET 1994 regime (where energy
  *must* be conserved).

The contradiction route asks: **can the ν → 0 limit be controlled
tightly enough to invoke either CET 1994 (forcing α*_NS ≥ 1/3) or
Isett 2018 (giving compatibility with C3)?** The Level 4 attempt
attempts to set this up rigorously.

---

## §2 CET 1994 statement + Onsager threshold sharpness

### §2.1 Constantin–E–Titi 1994 (recall, schematic)

**Constantin–E–Titi 1994** ("Onsager's conjecture on the energy
conservation for solutions of Euler's equation", *Comm. Math.
Phys.* 165 (1994), 207–209). The published theorem (recalled
schematically, not quoted):

> *Let u be a weak solution of the incompressible Euler equations
> on T³ × [0, T]. If u ∈ L³(0, T; B^{1/3}_{3, c(N)}) (the standard
> Besov refinement), then u conserves energy on [0, T].*

The Hölder corollary (used in the Onsager-conjecture exposition,
e.g. Eyink 1994, Duchon–Robert 2000, Cheskidov–Constantin–Friedlander–
Shvydkoy 2008):

> *If u ∈ L³(0, T; C^α(T³)) for some α > 1/3, then u conserves
> energy on [0, T].*

(The Besov B^{1/3}_{3, c(N)} class is slightly weaker than C^α for
α > 1/3 but stronger than C^{1/3}; the corollary is the standard
Hölder version.)

### §2.2 Isett 2018 (recall, schematic)

**Isett 2018** ("A Proof of the Onsager Conjecture", *Annals of
Math.* 188 (2018), 871–963). The published theorem (recalled
schematically):

> *For any α < 1/3, there exist weak solutions of the incompressible
> Euler equations on T³ × [0, T] in L^∞(0, T; C^α(T³)) that do
> not conserve energy.*

(Formally: u ∈ C^0_t C^α_x with strict energy decay; the
construction is via convex integration with Mikado-flow building
blocks, building on De Lellis–Székelyhidi 2013–2014 and
Daneri–Székelyhidi 2017.)

### §2.3 Sharpness of α = 1/3 at the Euler level

The two theorems together establish that **α = 1/3 is the sharp
threshold for Euler energy conservation**:

- α > 1/3: energy **must** be conserved (CET 1994).
- α < 1/3: energy **can** fail to conserve (Isett 2018, by
  explicit construction).
- α = 1/3 exactly: borderline; CET 1994 covers a slightly wider
  Besov class B^{1/3}_{3, c(N)} which includes some α = 1/3
  velocity fields, but not all C^{1/3} velocity fields are in this
  class. The exact behavior at α = 1/3 is regime-dependent.

This sharpness is the **Onsager 1949 conjecture**, fully resolved
on the Euler side as of 2018.

### §2.4 NS-side analog: open

For NS (ν > 0), the analogous question:

> *At what spatial Hölder exponent α is energy conservation
> guaranteed?*

is **open**. Three known facts:

(i) For smooth solutions (α = 1, Lipschitz, or higher with the
necessary integrability), NS conserves energy *up to* viscous
dissipation: dE/dt = −ν · ‖∇u‖_{L²}² ≤ 0.

(ii) For Leray-Hopf weak solutions (u ∈ L^∞_t L²_x ∩ L²_t H¹_x),
the *energy inequality* dE/dt ≤ −ν · ‖∇u‖_{L²}² holds; equality
is open and may fail (anomalous dissipation; Eyink 1994).

(iii) For α near 1/3 in C^α, neither CET-style energy conservation
nor Isett-style energy non-conservation is established for NS.

The R1-Aux question (α*_NS) sits in this open gap.

---

## §3 ν → 0 limit argument — formal setup

### §3.1 Hypothetical NS construction at α < 1/3

Hypothesis (R1-Aux conjecture C3 strict version, hypothetical):
suppose there exist NS weak solutions u^ν at spatial Hölder
exponent α = α*_NS − δ for some δ > 0, with the construction
**uniform in ν** in the sense:

(H1) u^ν ∈ L^∞(0, T; C^α(T³)) for all ν ∈ (0, ν_0].

(H2) ‖u^ν‖_{L^∞_t C^α_x} ≤ C uniformly in ν (independent of ν).

(H3) u^ν solves the NS equations with viscosity ν on [0, T] × T³
in the appropriate weak sense.

(H4) The construction is **non-trivial** in the sense that u^ν is
*not* the unique smooth Leray-Hopf solution (i.e. there exist at
least two distinct u^ν, u^{ν,'} with the same initial data,
realizing C3-style non-uniqueness).

Hypothesis (H1)–(H4) is what an "α*_NS − δ NS non-uniqueness
construction" would look like if it existed. Its existence is
exactly what C3 (strict) conjectures, and what Lemma 1 was
attempting to constrain.

### §3.2 The ν → 0 limit (formal)

By (H2), the family {u^ν}_{ν ∈ (0, ν_0]} is uniformly bounded in
L^∞_t C^α_x. By Arzelà–Ascoli (in space; for fixed t) and the
Aubin–Lions lemma (in time, with appropriate weak compactness
arguments), there exists a subsequence ν_k → 0 such that

    u^{ν_k} → u^0   in   L^∞(0, T; C^{α'}(T³))   for any α' < α    (3.1)

(strong in space at lower regularity α' < α, weak-* at the full
α by lower semicontinuity).

The limit u^0 solves the **incompressible Euler equations** on
[0, T] × T³ in the weak sense, **provided the nonlinear term
(u^ν · ∇)u^ν passes to the limit**. This passage is the delicate
step (it requires either strong L²-convergence of u^ν, or
sufficient regularity to interpret (u^0 · ∇)u^0 as a distribution).

For α > 1/3 (in the limit Hölder class), the nonlinear term is
classically well-defined, and (3.1) gives a bona fide Euler weak
solution u^0.

For α ≤ 1/3, the nonlinear term requires additional argument
(commutator estimates á la Duchon–Robert 2000); the limit u^0
exists but its status as an Euler weak solution depends on the
class.

### §3.3 The Euler limit's Hölder regularity

By lower semicontinuity of the C^α norm under weak-* convergence,

    ‖u^0‖_{L^∞_t C^α_x} ≤ liminf_{ν_k → 0} ‖u^{ν_k}‖_{L^∞_t C^α_x} ≤ C    (3.2)

So u^0 is a weak Euler solution in L^∞_t C^α_x with α =
α*_NS − δ.

### §3.4 The CET 1994 / Isett 2018 dichotomy on u^0

Two cases for the Euler limit's energy behavior:

**Case A**: α = α*_NS − δ > 1/3. By CET 1994 (applied to u^0,
provided u^0 is a bona fide Euler weak solution), u^0 **conserves
energy**: E(u^0(t)) = E(u^0(0)) for a.e. t ∈ [0, T].

**Case B**: α = α*_NS − δ < 1/3. By Isett 2018, there *exist*
Euler weak solutions in L^∞_t C^α_x with non-conserved energy. So
u^0 *may* have non-conserved energy, but is not *required* to.

**Case C**: α = α*_NS − δ = 1/3 exactly. Borderline; CET 1994's
Besov refinement may or may not apply depending on the specific
Besov class membership.

### §3.5 The NS-side energy bound

From (H3), u^ν solves NS with the *energy inequality* (for
Leray-Hopf weak solutions; the inequality is preserved for many
weaker classes including the convex-integration constructions):

    E(u^ν(t)) ≤ E(u^ν(0)) − 2ν ∫_0^t ‖∇u^ν(s)‖_{L²}² ds    (3.3)

In the limit ν → 0, the dissipation term 2ν ∫ ‖∇u‖² ds **may
remain non-zero** even as ν → 0 (this is the *anomalous
dissipation* phenomenon, Eyink 1994):

    lim sup_{ν → 0} 2ν ∫_0^t ‖∇u^ν(s)‖_{L²}² ds = ε_anom(t) ≥ 0    (3.4)

Anomalous dissipation is a **physical possibility**, not a
mathematical impossibility, and is widely conjectured to hold for
turbulent flows.

If ε_anom(t) > 0 for some t, then the limit energy E(u^0(t)) is
*strictly less than* E(u^0(0)):

    E(u^0(t)) ≤ E(u^0(0)) − ε_anom(t) < E(u^0(0))    (3.5)

i.e. the Euler limit u^0 has **non-conserved energy** purely from
the NS-side anomalous dissipation, regardless of u^0's intrinsic
energy behavior.

---

## §4 Contradiction construction with gap analysis

### §4.1 Case A contradiction attempt

Suppose α = α*_NS − δ > 1/3 (Case A in §3.4). The hypothetical NS
non-uniqueness gives u^ν, u^{ν,'} with same initial data, both
satisfying (H1)–(H4).

By §3.2, both have ν → 0 limits u^0, u^{0,'} that are Euler weak
solutions in L^∞_t C^α_x.

By CET 1994 (§3.4 Case A), both u^0 and u^{0,'} **conserve
energy**: E(u^0(t)) = E(u^0(0)) and E(u^{0,'}(t)) = E(u^{0,'}(0))
for a.e. t.

By (3.5), if anomalous dissipation occurs in the NS sequence
(ε_anom > 0), then E(u^0(t)) < E(u^0(0)) — contradicting CET 1994.

**So in Case A, anomalous dissipation cannot occur**: the NS
sequence must have ε_anom = 0. This is a **constraint**, not a
contradiction, on the hypothetical NS construction.

For a contradiction, we need to show that anomalous dissipation
*must* occur for the hypothetical NS sequence — equivalently, that
the limit u^0 *cannot* have ε_anom = 0. This is **not derivable
from (H1)–(H4) alone**: the NS sequence may very well satisfy
ε_anom = 0 (no anomalous dissipation), in which case Case A is
self-consistent and **no contradiction arises**.

**Gap A**: the contradiction in Case A relies on showing that the
hypothetical NS non-uniqueness *forces* ε_anom > 0. This is itself
a deep open question (the relation between NS non-uniqueness and
anomalous dissipation is conjectural; cf. Cheskidov–Shvydkoy 2014,
Drivas–Eyink 2019, by author/year only).

### §4.2 Case B contradiction attempt

Suppose α = α*_NS − δ < 1/3 (Case B in §3.4). The hypothetical NS
non-uniqueness gives u^ν, u^{ν,'}; their ν → 0 limits u^0, u^{0,'}
are Euler weak solutions in L^∞_t C^α_x.

By Isett 2018 (§3.4 Case B), Euler weak solutions in this class
*may* have non-conserved energy. There is **no constraint from
CET 1994** in this regime.

**No contradiction possible in Case B from CET 1994 alone**: the
Euler limit can absorb anomalous dissipation (or not), and the
NS-side hypothetical construction is consistent with both
possibilities.

**Gap B**: the contradiction route as posed has no Euler-side
rigidity to exploit when α < 1/3. The case α = α*_NS − δ < 1/3 is
*precisely* the regime where C3 places α*_NS, and it is precisely
the regime where CET 1994 does **not** apply.

### §4.3 Case C borderline

For α = α*_NS − δ = 1/3 exactly, the Besov refinement of CET 1994
may or may not apply. The contradiction is **regime-dependent**
and not derivable from the schematic bounds alone.

### §4.4 The structural problem

The contradiction route, as set up in §4.1–§4.3, yields:

- **Case A (α > 1/3)**: CET 1994 forces ε_anom = 0 (a constraint),
  not a contradiction. The constraint is consistent with the
  hypothetical NS construction (just a constraint on its
  dissipation behavior).
- **Case B (α < 1/3)**: no CET 1994 rigidity; hypothesis cannot be
  contradicted via this route.
- **Case C (α = 1/3)**: borderline; not decided.

**Net result**: the contradiction route does **not close**. The
key obstruction is that **CET 1994 only gives rigidity above
α = 1/3**, while C3 places α*_NS *below* 1/3. The route as
naively posed cannot constrain α*_NS to be ≥ 1/3.

### §4.5 Possible refinement: NS-specific energy rigidity

A potential refinement: rather than relying on CET 1994 (which is
an Euler theorem), use an NS-side energy rigidity. Specifically:

> *Conjecture (R1-Aux Lemma 1' candidate)*: for NS solutions u^ν
> at α > α_NS-CET in C^α, energy is conserved (modulo the explicit
> viscous dissipation term ν · ‖∇u‖_{L²}²), where α_NS-CET is some
> NS-specific Onsager-type threshold.

This conjecture has **no published treatment** to this dispatch's
literature scan. The quantity α_NS-CET is conjectural; if
α_NS-CET = 1/3 (matching the Euler threshold), the contradiction
route would close in Case A. If α_NS-CET < 1/3, the route would
close in a wider regime. If α_NS-CET > 1/3 (i.e. NS *requires
more regularity* than Euler for energy conservation), the route is
weaker.

Without an established α_NS-CET, the refinement is conditional on
an *additional* unproven theorem.

### §4.6 Possible refinement: Duchon–Robert 2000 distributional
energy balance

**Duchon–Robert 2000** ("Inertial energy dissipation for weak
solutions of incompressible Euler and Navier-Stokes equations",
*Nonlinearity* 13). The published distributional energy balance:

> *For weak solutions u of NS or Euler in L³_t L³_x, there exists a
> distribution D[u] (the "inertial dissipation measure") such that
> ∂_t (|u|²/2) + ∇·(...) = −D[u] in the distributional sense, and
> D[u] ≥ 0 weakly.*

For Euler at α > 1/3 in C^α, D[u] = 0 distributionally (recovers
CET 1994). For Euler at α < 1/3, D[u] can be a non-trivial
positive distribution (recovers Isett 2018-style anomalous
dissipation).

For NS, the inertial dissipation D[u^ν] coexists with the viscous
dissipation ν · ‖∇u^ν‖². As ν → 0, the *NS inertial dissipation
limit* D[u^0] should match the *Euler inertial dissipation* of
u^0.

If we could prove that the NS inertial dissipation D[u^ν] is **0
uniformly in ν** for the hypothetical α*_NS − δ < 1/3 construction
(perhaps because the construction is "smooth enough" to satisfy
the Duchon–Robert conservation criterion uniformly), then in the
limit D[u^0] = 0 — but this contradicts Isett 2018 in the regime
α < 1/3 only if the hypothetical construction's dissipation
behavior is incompatible with Isett 2018's.

This refinement also has gaps. Without an explicit relation
between the NS construction's inertial dissipation and the Euler
limit's, the contradiction is not closed.

---

## §5 Compactness check — does ν → 0 limit close?

### §5.1 What is required for compactness

For the ν → 0 limit argument in §3.2 to give a bona fide Euler
weak solution u^0 satisfying the energy considerations of §3.4,
we need:

(C1) **Strong L² convergence** u^{ν_k} → u^0 in L²_t L²_x: required
to pass the nonlinear term (u^ν · ∇)u^ν → (u^0 · ∇)u^0 in
distributions.

(C2) **Hölder norm preservation in the limit**: ‖u^0‖_{L^∞_t C^α_x}
≤ C (which follows from lower semicontinuity, §3.3).

(C3) **Energy passage**: the energy E(u^{ν_k}(t)) → E(u^0(t))
along the subsequence (so the §3.5 anomalous dissipation analysis
is valid).

### §5.2 Compactness in C^α — does it work?

Under (H2), {u^ν} is bounded in L^∞_t C^α_x. By Arzelà–Ascoli (in
space, fixed t), each u^ν(t, ·) has a subsequence converging in
C^{α'} for any α' < α. By a diagonal argument (in t, using
equicontinuity in time provided by the NS time-derivative bound),
we can extract a sequence u^{ν_k} → u^0 in C^0_t C^{α'}_x for any
α' < α.

This gives **strong convergence at lower regularity α' < α**, but
the **full α regularity is preserved only weakly-***, i.e. via
lower semicontinuity (3.2). The full C^α convergence is not
guaranteed.

For the Euler nonlinear term (u^0 · ∇)u^0 to be well-defined as
a distribution, we need at least α' > 1/3 (so the product u^0 ⊗
u^0 is in W^{2α', 1} which is in L^∞ if 2α' > 0, hence the
product is integrable, etc.). For α (and hence α') near 1/3, the
nonlinear term is *exactly at the borderline* of distributional
definability.

**This is the same regime where CET 1994 vs Isett 2018 dichotomy
is sharp**: at α slightly above 1/3, the nonlinear term is
distributionally well-defined (and CET 1994 applies); at α slightly
below 1/3, the nonlinear term requires a careful renormalization
(and Isett 2018-style energy non-conservation is possible).

### §5.3 The C3 regime: α*_NS − δ in (α_BV, 1/3)

C3 places α*_NS in (α_BV, 1/3) strictly, so α*_NS − δ is even
*more strictly* below 1/3 (for δ > 0). In this regime:

- (C1) strong L² convergence: holds (by lower-regularity strong
  convergence, since the NS construction has at least L²_t L²_x
  bound from the Leray-Hopf-style energy inequality).
- (C2) Hölder norm preservation: holds weakly (by lower
  semicontinuity).
- (C3) Energy passage: **may or may not hold**. The energy may
  drop in the limit due to anomalous dissipation (the §3.5 ε_anom
  term).

In the regime α < 1/3, (C3) is the borderline: the Euler limit's
energy may differ from the NS sequence's limit-energy by ε_anom.
If ε_anom > 0, the Euler limit has non-conserved energy
**purely from the NS sequence's dissipation accumulation**, not
from any Euler-side mechanism.

### §5.4 The compactness gap

The ν → 0 limit, applied to a hypothetical α*_NS − δ < 1/3 NS
non-uniqueness sequence, gives an Euler weak solution u^0 in
L^∞_t C^α_x. But:

- u^0's energy may already be non-conserved (consistent with Isett
  2018 in this regime).
- u^0's energy non-conservation may match the NS sequence's
  anomalous dissipation ε_anom (consistent with the Euler limit
  picking up the dissipation deficit).
- **No CET 1994 obstruction applies** because α < 1/3.

The compactness check **does not produce a contradiction**. The
ν → 0 limit is mathematically sound (assuming the regularity
hypotheses are uniform) but gives an Euler limit that is fully
compatible with Isett 2018, not contradictory to CET 1994.

### §5.5 Compactness verdict

**The compactness step is valid; the contradiction does not close.**

The core issue: CET 1994's rigidity is at α > 1/3, which is *not*
where C3 places α*_NS. The ν → 0 limit transports the NS
regularity α to the Euler regularity α (modulo lower
semicontinuity slack), but this same α value is in the
"non-conservation possible" regime of Isett 2018, not in CET
1994's "conservation forced" regime.

To close the contradiction, one would need either:

(i) An NS construction at α > 1/3 (which would invoke CET 1994
in Case A, but C3 doesn't predict this — C3 places α*_NS *below*
1/3); or

(ii) An NS-specific energy rigidity at α < 1/3 (the conjectural
α_NS-CET threshold §4.5; not established in literature); or

(iii) A reverse contradiction: assume α*_NS = 1/3 (i.e. C1 in the
parent dispatch), derive an Isett-style construction that
contradicts CET 1994 via ν → 0 — but this *also* doesn't work,
since CET 1994 applies only at α > 1/3, not at α = 1/3 exactly.

---

## §6 Verdict

**OBSTRUCTION_DOCUMENTED_LEVEL_4.**

The ν → 0 Onsager-contradiction route, as set up in §3–§5, **does
not close** the Lemma 1 contradiction. The structural reason: CET
1994's energy-conservation rigidity applies at Euler Hölder
exponents *strictly above 1/3*, while the C3 conjecture places
α*_NS *strictly below 1/3*. The two regimes are disjoint, and the
ν → 0 limit transports an NS Hölder exponent α to the same Euler
exponent α (modulo lower semicontinuity), so the limit lands in
**the same regime** where CET 1994 has nothing to say.

The contradiction route is therefore **incompatible with C3's
regime placement**: the route can only close at α > 1/3 (Case A),
but C3 hypothesizes α*_NS < 1/3 (Case B). At α < 1/3, Isett 2018
permits non-conserved energy in the Euler limit, fully compatible
with the hypothetical NS non-uniqueness.

### §6.1 Verdict-grade justification

| candidate verdict | applicability | rejected because |
|-------------------|---------------|------------------|
| PASS_SKETCH_REFINES_C3 (contradiction closes; α*_NS = 1/3 forced) | NO | the contradiction does not close in the C3 regime (α < 1/3) because CET 1994 has no rigidity there; closure would require α_NS-CET conjecture (unestablished) |
| PARTIAL (bound on δ-margin obtained but not closed) | NO | no δ-margin is produced; the route fails *categorically* in Case B (α < 1/3), not by a δ-tightening issue |
| INCONCLUSIVE (literature scan underdetermined) | NO | the literature is conclusive: CET 1994 applies only above 1/3, Isett 2018 covers below 1/3, and the ν → 0 limit preserves the regularity exponent. The route is well-posed and conclusively **does not close** |
| **OBSTRUCTION_DOCUMENTED_LEVEL_4** | **YES** | the route is set up rigorously (§3), the contradiction attempt is articulated (§4), the gap is identified at the case-analysis level (§4.4: CET 1994 has no rigidity in C3's regime), and the compactness check is valid but unhelpful (§5.5) |
| FAIL (Lemma 1 refuted at Level 4) | NO | the route's failure to close is *structural*, not a refutation of Lemma 1; Lemma 1 may still be establishable by other routes |
| C3_REFUTED | NO | the route's failure is *consistent* with C3 (it neither supports nor refutes C3) |

### §6.2 α* bound produced

**None.** The Level 4 attempt does not produce any α* lower bound,
upper bound, or δ-margin. The C3 strip (α_BV, 1/3) is **unconstrained**
by this attempt.

### §6.3 Gap identified

The gap is **structural**: CET 1994's rigidity regime (α > 1/3) is
disjoint from C3's hypothesized regime (α*_NS < 1/3). The ν → 0
limit, while mathematically sound (§5.5), transports the
hypothetical NS construction into the **same** regime, where Isett
2018 permits all the energy behaviors that the contradiction would
need to forbid.

To close the route, one of:

(i) An NS-specific Onsager threshold α_NS-CET — but if this exists
    and equals 1/3, it would itself be the answer to R1-Aux,
    obviating Lemma 1; if it differs from 1/3, it requires its own
    proof.

(ii) A *direct* construction at α > 1/3 (i.e. proving the C2
    branch of the parent dispatch, where α*_NS ≥ 1/3) — but this
    is exactly the *opposite* of what Lemma 1 sought.

(iii) A reformulation that doesn't rely on the Euler-side
    threshold sharpness — but the route's premise was *to* leverage
    that sharpness.

None of (i)–(iii) is currently within the schematic Level 4
analysis's scope.

---

## §7 C3 conjecture status post-Level 4

**C3 status: live, unchanged.**

Per parent dispatch §4.4 and L1–L3:

C3: *α*_NS ∈ (α_BV, 1/3) strictly*

Level 4's analysis:

- Does **not** narrow the upper end of the strip (α near 1/3): no
  near-1/3 construction is produced; CET 1994's rigidity is
  invoked but does not constrain the C3 regime.
- Does **not** narrow the lower end of the strip: no δ_0 lower
  bound of the form α*_NS ≥ 1/3 − δ_0 is produced.
- Does **not** falsify C3: the contradiction route's failure is
  *consistent* with C3 (and would have been consistent even if
  C1 or C2 were true).

**C3 status**: live, unchanged. Falsifiers F-C3-A..C (parent §4.4)
remain inactive.

The Level 4 attempt **strengthens the methodological case** that:

1. The convex-integration re-derivation route (L1–L3) is
   inadequate.
2. The ν → 0 contradiction route, **as posed**, is also
   inadequate — the regime mismatch between CET 1994 and C3 is
   *structural*.
3. R1-Aux's α*_NS in (α_BV, 1/3) is **genuinely outside the
   reach of the standard Onsager-threshold-leveraging strategy**.

Per F-Sk-CET-3 (§10): C3 is **not refuted** by Level 4's failure
to close; the failure is regime-mismatch, not C3-incompatibility.

---

## §8 Cumulative methodological synthesis (4 levels)

### §8.1 Four-level progressive deepening summary

| Level | Approach | Obstruction diagnosed | Sanity check | α* extracted |
|-------|----------|------------------------|---------------|---------------|
| L1 | BV-2019 schematic Hölder balance | missing μ_q, log-vs-power-law | FAILS at α_BV | none |
| L2 | μ_q tracking, two-leg balance | wrong building block (intermittent jets) | PASSES at α_BV in L²; FAILS for Hölder α < 1 | vacuous α*_{BV,Hölder} ≥ 1 |
| L3 | BCV-2021 reframe, three-leg balance | missing temporal-singular-set leg (B4') | FAILS at α_BCV | none, empty α-window |
| **L4** | **ν → 0 CET-contradiction** | **CET 1994 regime ≠ C3 regime** | **§5.5 compactness valid but unhelpful** | **none, no δ-margin** |

### §8.2 What the four levels collectively reveal

The four-level progression has revealed an **honest landscape** of
R1-Aux Lemma 1's difficulty:

1. **Convex-integration re-derivation** (L1–L3): structurally
   inadequate at the schematic level. Each successive refinement
   (μ_q, building-block class, singular-set leg) reveals a deeper
   missing piece. A **faithful** treatment requires close re-
   reading of the post-2021 NS Hölder-class papers (BCV-2021,
   Cheskidov–Luo 2022, and successors) at a level beyond
   schematic-template analysis. **Effort**: 5–10 sessions of
   literature work, with no guarantee of producing α* in C3's
   regime.

2. **ν → 0 Onsager-contradiction** (L4): structurally inadequate
   *because of regime mismatch*. CET 1994 is sharp at the Euler
   level (α > 1/3) but offers no rigidity in C3's hypothesized
   regime (α < 1/3). The contradiction route would be effective
   *if* C3 placed α*_NS *above* 1/3 (i.e. C2), but C3 does the
   opposite.

3. **Combined diagnosis**: the standard tools (convex integration
   + Onsager threshold leveraging) **do not directly access the
   C3 regime**. The C3 regime — α*_NS strictly between α_BV and
   1/3 — sits in a genuine gap that neither the construction side
   (which currently reaches only α_BV ≪ 1/3) nor the rigidity side
   (which requires α > 1/3) covers.

### §8.3 What Level 5+ might be

Possible Level 5 directions:

(L5a) **NS-specific Onsager threshold conjecture**: formulate and
investigate the conjectural α_NS-CET threshold (§4.5). This
becomes a meta-question: *is there an NS analog of CET 1994 at
some threshold α_NS-CET?* If yes, what is it?

(L5b) **Quantitative compactness**: rather than the qualitative
ν → 0 limit, extract a *quantitative* relation between the NS
construction's parameters at small ν and the Euler limit's
regularity. This is the realm of Vasseur–Yu, De Rosa–Park, and
related quantitative regularity work; cited by author/year only,
no claim to specific results.

(L5c) **Direct reading of post-2021 literature**: abandon the
schematic-template approach entirely, and re-derive Lemma 1 from
the actual published BCV-2021 / Cheskidov–Luo 2022 inequality
systems. This is L3 §8.2 #1 (option 1), estimated 5–10 sessions.

(L5d) **Reformulate R1-Aux entirely**: drop the α*_NS lower-bound
formulation, and instead seek a *quantitative* version of C3, e.g.
*the gap (1/3 − α*_NS) is bounded above by some explicit function
of ν*. This shifts the question from existence to magnitude.

(L5e) **Methodological pause**: declare R1-Aux "structurally hard"
and shift the BT-544 R1 axis to a different sub-question (e.g. R5
strict-gap line, or the C-axis discriminator, or a completely
different open piece in the parent dispatch's anti-list).

The honest assessment after four levels: **all five candidate
Level 5 directions are speculative**, and none is guaranteed to
produce a useful α*. Continued progressive deepening at Level 5+
may be productive, or may indicate a *fundamental obstruction*
that R1-Aux (in its current formulation) is genuinely beyond the
reach of current convex-integration / Onsager-threshold methods.

### §8.4 Honest synthesis

After four levels of progressive deepening, **R1-Aux Lemma 1 is
still open**. Each retry has identified a more specific blocker:

- L1: schematic inequality form wrong;
- L2: schematic building block wrong;
- L3: schematic inequality system topology wrong;
- L4: contradiction route's regime placement wrong.

The cumulative diagnosis is that **R1-Aux's α*_NS lies in a
genuine gap between current method capabilities**. Convex
integration reaches α_BV (well below 1/3) on the construction
side; CET 1994 imposes α > 1/3 on the rigidity side; the C3 strip
(α_BV, 1/3) is **not directly accessible** to either tool at the
schematic level.

Level 5+ continuation would require either a substantially deeper
literature engagement (L5c) or a methodological reformulation
(L5a, L5b, L5d). The four-level Lemma 1 sequence has, on the other
hand, fulfilled its registered purpose: **diagnose the structural
obstruction with increasing precision**. The progressive deepening
has not produced α*, but it has produced a clear map of *why*
α* is hard.

---

## §9 Anti-list — alternative routes considered (Level 4 update)

Routes considered at Level 4, beyond the ν → 0 CET-contradiction
attempted, and reasons not pursued.

| route | Level 4 status | reason not pursued in Level 4 |
|-------|----------------|-------------------------------|
| ν → 0 Onsager-contradiction via CET 1994 (the L4 primary subject) | **pursued** | produced OBSTRUCTION_DOCUMENTED_LEVEL_4 (regime mismatch) |
| Convex-integration re-derivation (L1–L3 retried) | already pursued | OBSTRUCTION at L1–L3; will not re-pursue at L4 |
| Duchon–Robert 2000 distributional energy balance refinement | not pursued | recorded in §4.6 as a possible refinement, but requires NS-Euler dissipation transport theorem, beyond L4 scope |
| Vasseur–Yu / De Rosa–Park quantitative compactness | not pursued | recorded as L5b candidate; no published treatment specific to R1-Aux |
| NS-specific Onsager threshold α_NS-CET conjecture | not pursued | recorded as L5a candidate; conjectural, requires its own proof |
| Reverse contradiction (assume α*_NS = 1/3, derive Isett-incompatibility) | not pursued | does not work: CET 1994 doesn't apply at α = 1/3 exactly (Besov refinement is borderline), and Isett 2018 doesn't apply at α = 1/3 either |
| Cheskidov–Constantin–Friedlander–Shvydkoy 2008 (Besov refinement of CET 1994) | not pursued | the Besov class B^{1/3}_{3, c(N)} is at α = 1/3 exactly; doesn't extend rigidity to α < 1/3, where C3 lives |
| Albritton–Brué–Colombo 2022 forced-NS counterexample to ν → 0 limit | not pursued | uses *forcing* — out of scope for force-free R1-Aux; also targets a different question (Leray-Hopf non-uniqueness, not Hölder α near 1/3) |
| Modulated convex-integration (Modena–Sattig, Modena–Székelyhidi) | not pursued | targets transport equations; out of scope |
| Eyink 1994 anomalous dissipation in C^α | not pursued | provides the ε_anom framework used in §3.5 but is not itself a Lemma-1-establishing tool |
| Drivas–Eyink 2019 anomalous dissipation–regularity correspondence | not pursued | recorded in §4.1 as relevant to Gap A; no published explicit treatment for R1-Aux |

Of these, the most actionable Level 5 candidates are L5a (NS-
specific α_NS-CET) and L5c (literature-faithful BCV-2021 re-
derivation), but both require effort substantially beyond the
"sketch attempt" scope of L1–L4.

---

## §10 Falsifiers active for Level 4

These falsifiers apply to **Level 4** specifically, in addition to
L1's F-Sk-1..6, L2's F-Sk-Mu-1..6, L3's F-Sk-BCV-1..6, and parent
F-Disp-1..6.

- **F-Sk-CET-1** (CET 1994 statement fabrication): if the §2.1
  recall of Constantin–E–Titi 1994 is found to differ from the
  published 1994 *Comm. Math. Phys.* paper (e.g. the Besov class
  is different, or the threshold is differently formulated), the
  Level 4 case-analysis (§3.4) and §4 contradiction attempts must
  be revised. **Status: not active** — §2.1 cites by author/year/
  journal/theorem only; the statement "α > 1/3 in C^α implies
  energy conservation" is the standard Hölder corollary widely
  reproduced in subsequent literature; no quoted text or specific
  numerical constant is attributed.

- **F-Sk-CET-2** (Isett 2018 statement fabrication): if the §2.2
  recall of Isett 2018 differs from the published *Annals* paper,
  the Case B analysis (§4.2) must be revised. **Status: not
  active** — §2.2 cites by author/year/journal only; the statement
  "α < 1/3 admits non-conservative C^α Euler weak solutions" is
  the headline theorem of the published paper.

- **F-Sk-CET-3** (regime-mismatch is artifact, not structural):
  if the regime mismatch (CET 1994 at α > 1/3 vs C3 at α*_NS <
  1/3) is found to be resolvable by a *technique* that hasn't
  been considered (e.g. an Euler-side rigidity at α ≤ 1/3 not
  derived from CET 1994), the OBSTRUCTION_DOCUMENTED_LEVEL_4
  verdict is premature. **Status: watch-state** — current
  literature scan does not provide such a technique, but the
  watch-state remains because the negation is not exhaustively
  verified.

- **F-Sk-CET-4** (atlas/state/inventory touch): if Level 4 leads
  to any modification of `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`, the Level 4 attempt
  has been mis-applied. **Status: not active** — Level 4 is
  research-direction only; no atlas/state/inventory edits
  performed.

- **F-Sk-CET-5** (claim-creep on δ_0 / α*): if Level 4 is reported
  as having produced a meaningful δ_0, α* lower bound, or refined
  C3, the attempt has been mis-cited. Level 4 produces **no** α*
  bound, **no** δ_0, and **no** C3 refinement. **Status: not
  active**; this report explicitly flags the absence of α* (§6.2)
  and the unchanged C3 status (§7).

- **F-Sk-CET-6** (auxiliary→main confusion, Level 4 layer): if
  Level 4's obstruction is reported as informing BT-544 directly,
  the auxiliary→main distinction has been lost across five layers
  (parent + L1 + L2 + L3 + L4). The Level 4 attempt is auxiliary
  to the auxiliary R1-Aux. **Status: not active**.

None of F-Sk-CET-1..6 fires under this report's scope.

The cumulative falsifier roster across L1–L4:

- F-Disp-1..6 (parent dispatch): inactive (verified L1).
- F-Sk-1..6 (L1): inactive (verified L1, L2, L3, L4).
- F-Sk-Mu-1..6 (L2): inactive (verified L2, L3, L4).
- F-Sk-BCV-1..6 (L3): inactive (verified L3, L4).
- F-Sk-CET-1..6 (L4, this report): inactive (verified §10).
- F-C3-A..C (parent §4.4): inactive (C3 unchanged, §7).

Total: 30+ falsifiers across five reports, all inactive. The
F-Sk-CET-3 watch-state is the only "soft" status; all others are
firmly inactive.

---

## §11 Closing

**0/7 unchanged. NS regularity status open. No atlas/state/inventory edits.**

**Summary of Level 4 attempt**:

- L1–L3 cumulative obstruction recapped (§1): three increasingly
  deep convex-integration template inadequacies.
- CET 1994 statement recalled schematically (§2.1): α > 1/3 in
  C^α implies Euler energy conservation. Isett 2018 recalled
  (§2.2): α < 1/3 admits non-conservative C^α Euler weak
  solutions. Onsager threshold α = 1/3 sharp at Euler level
  (§2.3). NS-side analog (α*_NS) open (§2.4).
- ν → 0 limit argument set up (§3): hypothetical NS non-uniqueness
  (H1)–(H4) at α = α*_NS − δ < 1/3 yields Euler limit u^0 in
  L^∞_t C^α_x via subsequence + lower semicontinuity. Anomalous
  dissipation ε_anom enters via NS energy-inequality.
- Contradiction construction with gap analysis (§4): three cases
  (α > 1/3, α < 1/3, α = 1/3 borderline). **Case A** (α > 1/3):
  CET 1994 forces ε_anom = 0 — a constraint, not a contradiction.
  **Case B** (α < 1/3, the C3 regime): no CET 1994 rigidity; route
  cannot constrain. **Case C** (α = 1/3): borderline.
- Compactness check (§5): ν → 0 limit is mathematically sound;
  produces a bona fide Euler weak solution u^0; energy passage
  may be non-trivial (anomalous dissipation), but in the C3
  regime this is fully consistent with Isett 2018. Compactness
  step is valid but does not produce contradiction.
- **Verdict: OBSTRUCTION_DOCUMENTED_LEVEL_4** (§6). The
  contradiction route's failure is *structural* — CET 1994's
  regime (α > 1/3) is disjoint from C3's regime (α*_NS < 1/3),
  and the ν → 0 limit preserves the regularity exponent, so
  cannot bridge the regimes. **No α* bound, no δ_0, no C3
  refinement** produced (§6.2).
- Gap identified (§6.3): structural regime mismatch. Closing the
  route would require an NS-specific Onsager threshold α_NS-CET
  (conjectural, §4.5), or a Duchon–Robert 2000 dissipation-
  transport refinement (§4.6), neither of which is in
  literature scope.
- C3 conjecture status (§7): **live, unchanged**. Level 4 neither
  supports nor refutes C3; the regime-mismatch failure is
  *consistent* with all of C1, C2, C3 simultaneously.
- Cumulative methodological synthesis (§8): four-level diagnosis
  reveals R1-Aux's α*_NS sits in a **genuine gap** between current
  method capabilities (convex integration α_BV ≪ 1/3 on
  construction side; CET 1994 α > 1/3 on rigidity side). C3 strip
  (α_BV, 1/3) is **not directly accessible** to schematic-level
  analysis. Level 5+ candidates (L5a–L5e, §8.3) recorded; all
  speculative.
- Honest synthesis (§8.4): four-level Lemma 1 sequence has
  **diagnosed the structural obstruction with increasing
  precision** but has not produced α*. R1-Aux Lemma 1 remains
  open. Continued progressive deepening at Level 5+ may be
  productive or may indicate a *fundamental obstruction*.
- Anti-list (§9): updated with Level 5 candidates.
- Falsifier roster (§10): F-Sk-CET-1..6 specific to Level 4,
  inactive. Cumulative roster across L1–L4 (30+ falsifiers):
  all inactive, F-Sk-CET-3 watch-state only.

**Comparative depth across four levels** (extending L3's table):

| Level | obstruction diagnosis | sanity check at native α | α* expression |
|-------|------------------------|---------------------------|----------------|
| 1 | missing μ_q | FAILS at α_BV | none |
| 2 | wrong building-block class | PASSES at α_BV (L²); FAILS for Hölder | α*_{BV,Hölder} ≥ 1, vacuous |
| 3 | missing temporal-singular-set leg | FAILS at α_BCV | none, empty α-window |
| **4** | **CET 1994 regime ≠ C3 regime** | **§5.5 compactness valid but unhelpful** | **none, no δ-margin** |

The schematic Hölder-balance template has been exhausted (L1–L3),
and the leading alternative route — ν → 0 Onsager-contradiction
via CET 1994 — has been shown to be regime-incompatible with C3
(L4). The progressive-deepening sequence has produced a complete
structural map of *why* R1-Aux Lemma 1 is hard, without producing
a useful α* lower bound.

R1-Aux remains open. α*_NS remains UNKNOWN. C3 remains a live
conjecture with falsifiers F-C3-A..C inactive. BT-544 remains 0/1
untouched. Clay status unchanged.

Per F-Disp-6 (parent §8), F-Sk-5 (L1 §10), F-Sk-Mu-6 (L2 §11),
F-Sk-BCV-6 (L3 §11), and F-Sk-CET-6 (§10): Level 4 does not
inform BT-544 directly. The auxiliary→main distinction is
preserved at all five layers (parent, L1, L2, L3, L4).

**0/7 unchanged. NS regularity status open. No atlas/state/inventory edits.**
