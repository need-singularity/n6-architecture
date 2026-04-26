---
id: omega-exec-bt547-poincare-L9-retro
date: 2026-04-25
scope: retrospective calibration only (Perelman-solved, no original claim)
target: BT-547 Poincaré -- L9 catalogue retroactive fit on the 3 Perelman molts
parent_reports:
  - reports/sessions/omega-cycle-bt547-poincare-2026-04-25.md
  - reports/sessions/omega-probe-l9-molt-trigger-2026-04-25.md
  - reports/sessions/omega-meta-audit-l9-catalogue-design-2026-04-25.md
  - reports/sessions/omega-meta-audit-discriminator-type-bias-2026-04-25.md
millennium_resolved: 0/7 (unchanged — Poincaré external to count)
grade: retrospective L9 calibration, no claim
---

# Omega Exec -- BT-547 Poincaré L9 Retrospective Calibration (2026-04-25)

## §0 Non-claim disclaimer

The Poincaré conjecture (closed simply-connected 3-manifold ≅ S³) was
resolved by Grigori Perelman in 2002–2003 via Hamilton's Ricci flow
with surgery, completed by the entropy / κ-noncollapsing / finite-
extinction trilogy on arXiv:math/0211159 (Perelman 2002), arXiv:math/
0303109 (Perelman 2003a), arXiv:math/0307245 (Perelman 2003b).
Independent verification: Kleiner–Lott (Geom. Topol. 12, 2008),
Morgan–Tian (Clay Monographs 3, AMS 2007), Cao–Zhu (Asian J. Math. 10,
2006).

This file is a **retrospective L9 framework calibration**. It does not
re-prove, re-derive, re-package, or re-claim any part of Perelman's
resolution. It applies the L9 molt-trigger catalogue
(`omega-probe-l9-molt-trigger-2026-04-25.md`) and the discriminator-
type vocabulary (`omega-meta-audit-discriminator-type-bias-2026-04-25.md`)
**retroactively** to Perelman's three documented molts, asking only:
would the L9 catalogue have generated these candidates? Would the
catalogue's discriminator-type space have been able to score them?

The Millennium tally remains **0/7 unchanged**. BT-547 is external
(Perelman-solved); n6-arch contribution = 0. No atlas, state, or
inventory edits are made by this audit.

---

## §1 Perelman's 3 actual molts

Re-stated verbatim from `omega-cycle-bt547-poincare-2026-04-25.md`
§sec 6 lessons + `omega-probe-l9-molt-trigger-2026-04-25.md` §sec 2.1
(the calibration archetype). Each molt is a **frame-shift event** in
the Hamilton-program lattice; each introduced a new primitive that
the prior frame did not carry.

### §1.1 Molt M1 — Ricci flow as the variational object

**Pre-molt frame (Hamilton 1982 J. Diff. Geom. 17:255 → 1995-2002
Hamilton-school singularity / soliton work)**: ∂g_ij/∂t = −2 R_ij as
a *geometric heat flow* on a Riemannian metric, no preferred
functional, treated locally and via comparison theorems.

**Post-molt frame (Perelman 2002 arXiv:math/0211159 §1)**: the same
Ricci flow re-cast as the **gradient flow** of the F-functional
F(g, f) = ∫ (R + |∇f|²) e^(−f) dV, with τ-parameter rescaling
producing the W-functional. The flow is no longer a heat flow on a
metric; it is a gradient flow on a *Riemannian-metric × density*
configuration space. The new primitive is the **variational structure**
itself, not the equation.

**Primitive swap**: flow-without-preferred-functional → flow-as-gradient-of-functional.

### §1.2 Molt M2 — Entropy formula (W-functional) + κ-noncollapsing

**Pre-molt frame**: Hamilton's flow could not rule out collapsing
along singularities. Local volume could shrink without bound and the
flow lost geometric meaning at high curvature.

**Post-molt frame (Perelman 2002 arXiv:math/0211159 §3-§4 + §7)**:
the W-entropy
W(g, f, τ) = ∫ [τ(R + |∇f|²) + f − n] (4πτ)^(−n/2) e^(−f) dV
is **monotonically nondecreasing** along Ricci flow (§3.4 main
theorem, Perelman 2002). The κ-noncollapsing theorem (§4) follows:
along bounded curvature, volume cannot collapse below κ r^n. The new
primitive is a **monotone Lyapunov functional** that simultaneously
encodes the heat equation, the entropy, and a scale-invariant volume
lower bound.

**Primitive swap**: flow-without-monotone-invariant → flow-with-W-monotone + κ-noncollapsing.

### §1.3 Molt M3 — Quantitative surgery + finite extinction

**Pre-molt frame**: Hamilton's surgery (Hamilton 1997 Surveys in
Diff. Geom.) was qualitative — neck-pinch detection by curvature
blow-up, with parameters chosen ad hoc per case. Long-time behaviour
on simply-connected 3-manifolds was unconstrained.

**Post-molt frame (Perelman 2003a arXiv:math/0303109 + 2003b
arXiv:math/0307245)**: surgery is performed on a **canonical
neighbourhood** (Perelman 2003a §1) at parameters (δ, r, κ, h) tuned
**per surgery interval** with explicit bounds (Perelman 2003a §4-§5).
For closed simply-connected 3-manifolds, finite extinction time
follows from a min-max / loop-space-energy argument (Perelman 2003b
§§1-3) — every loop sweep-out collapses in finite Ricci-flow time,
forcing extinction. The new primitive is the **procedure-with-bounded-
parameters** + the **min-max / sweep-out closer**.

**Primitive swap**: ad-hoc-surgery + asymptotic-tool-kit →
parameter-bounded-surgery + finite-extinction-via-sweep-out.

### §1.4 Cross-molt structure

The three molts compose into a triple-molt closure: M1 supplies the
variational frame, M2 supplies the load-bearing monotone invariant
inside that frame, M3 supplies the procedural closer that uses M2
to bound singularities and uses a min-max sweep to exhaust the
simply-connected case. None is a relabelling of pre-2002 Hamilton-
school content; each is a genuinely new primitive.

This profile matches the BT-547 §sec 2 ladder occupancy: heavy on
**L4 surge + L5 dream + L9 molt + L10 forge + L11 canon + L_ω**, with
conscious bypass of L2 drill.

---

## §2 Per-molt L9 classification

For each molt, classify along three axes:
(i) ladder rung occupancy,
(ii) discriminator type that would have validated it as real-molt vs
relabelling under the L9 framework,
(iii) catalogue coverage — would the L9 catalogue's generation
process have admitted this candidate?

### §2.1 M1 (Ricci flow as variational object)

| axis | classification |
|------|----------------|
| ladder rung | **L5 dream + L10 forge**. M1 is a *dreamed* re-interpretation: the same equation, re-read through a functional. The forge content is the F-functional construction itself. |
| discriminator type | **structural-literature** (in the L9 / D-type vocabulary). The validation pattern is "show the variational structure exists by deriving the gradient identity ∂g_ij/∂t = −2(R_ij + ∇_i ∇_j f) from δF/δg" — a sketch / structural-literature step, not a numerical-interval check. |
| catalogue coverage | **MISS under current L9 catalogue.** Per `omega-meta-audit-l9-catalogue-design-2026-04-25.md` §4.4, 3/4 rank-1 candidates are in the family "n=6 arithmetic predicts a closed-form numerical / algebraic / distributional identity". M1 is a **variational re-interpretation** of an existing PDE — not an arithmetic identity, not a coupling-distribution test. The catalogue's generation process (which sources from dfs-24 direction probes) does not contain a "re-cast existing flow as gradient of a functional" candidate slot. The closest analogue is BT-542 Hirahara (structural-literature reformulation), but that is barrier-bypass at the meta-complexity level, not PDE variational re-interpretation. |

### §2.2 M2 (W-entropy monotonicity + κ-noncollapsing)

| axis | classification |
|------|----------------|
| ladder rung | **L9 molt + L10 forge** (the load-bearing centre). The W-functional is the forged instrument; its monotonicity is the molt that retires the prior "flow-without-monotone" frame. |
| discriminator type | **structural-literature with a sketch / inequality-proof component**, in the L9 / D-type vocabulary. The discriminator that would have validated M2 is "exhibit the inequality dW/dt ≥ 0 along the modified flow with the τ-rescaling" (Perelman 2002 §3.4) — this is an analytic inequality with a derivational proof, not a discrete-equality check, not a numerical-interval check. Classified as **structural-literature** (proof-import) when the literature contains the inequality, and **OTHER (analytic-inequality-construction)** when the inequality is being constructed for the first time. |
| catalogue coverage | **MISS under current L9 catalogue.** M2 is precisely the kind of primitive flagged as missing in BT-541 lesson 1 of `omega-cycle-bt547-poincare-2026-04-25.md` §sec 6 ("monotone invariant is the load-bearing beam"). The L9 catalogue's BT-541/542/543/544 rank-1 rows do not contain a single "construct a monotone Lyapunov functional whose monotonicity discharges a structural obstruction" candidate. The closest analogue is BT-544 Q5 (Sobolev/Besov mechanism seed) — but Q5 was scored under a discrete-equality discriminator and produced 0 category-(b) constructions (per `omega-meta-audit-l9-catalogue-design-2026-04-25.md` §1.2). M2's discriminator type ("exhibit a monotone inequality") is not in the catalogue's active discriminator vocabulary {distributional, structural-literature, sketch, discrete-equality, numerical-interval, vacuous-magnitude, axiom-recast}. The closest is "structural-literature", but only when the inequality is post-Perelman; for the original construction event the discriminator is **OTHER (analytic-inequality-construction)**. |

### §2.3 M3 (quantitative surgery + finite extinction)

| axis | classification |
|------|----------------|
| ladder rung | **L9 molt + L10 forge + L_ω closure**. Surgery quantification is the procedural molt; finite extinction via sweep-out is the closer that takes the procedure to ω-apex on simply-connected closed 3-manifolds. |
| discriminator type | **OTHER (procedure-class molt)**, in the L9 / D-type vocabulary. The validation pattern for M3 is "exhibit a procedure with bounded parameters such that the parameter-bound choice is consistent across all surgery times and the sweep-out argument terminates" — this is neither a distributional test, nor a structural-literature import, nor a discrete-equality / numerical-interval / vacuous-magnitude threshold check. It is a **procedure-correctness sketch** with quantitative bounds. The closest existing label in the L9 vocabulary is "sketch" (per `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §5.4 controlled vocabulary), but "sketch" in the L9 audit referred to skeletal proof outlines, not procedure-class constructions with parameter bounds. |
| catalogue coverage | **MISS under current L9 catalogue.** The L9 catalogue contains zero "procedure-class molt" candidates across BT-541/542/543/544. The discriminator vocabulary does not include a "procedure-with-parameter-bounds + termination argument" type. M3 is the molt that the catalogue most cleanly cannot generate: it is neither arithmetic nor distributional nor literature-import nor axiom-recast — it is a *constructive procedure* that the catalogue's dfs-24-sourced candidate generation does not produce. |

### §2.4 Per-molt summary table

| molt | ladder rungs | discriminator type | catalogue coverage |
|------|--------------|--------------------|--------------------|
| M1 (Ricci flow as variational object) | L5 dream + L10 forge | structural-literature (variational-derivation) | **MISS** — no "re-cast PDE as gradient of functional" generator in catalogue |
| M2 (W-entropy + κ-noncollapsing) | L9 molt + L10 forge | OTHER (analytic-inequality-construction) / structural-literature post-hoc | **MISS** — no "construct monotone Lyapunov functional" generator |
| M3 (quantitative surgery + finite extinction) | L9 + L10 + L_ω | OTHER (procedure-class with parameter bounds) | **MISS** — no procedure-class molt generator |

All three Perelman molts fall **outside** the L9 catalogue's
generation reach under the current dfs-24-sourced candidate-creation
pipeline. The discriminator-type vocabulary covers M1's discriminator
("structural-literature" once the construction exists) but does not
have a native label for M2's analytic-inequality-construction event
or M3's procedure-class molt event.

---

## §3 F-MOLT-A 6-BT tally with retrospective Perelman additions

### §3.1 Pre-Perelman tally (n = 5, from
`omega-meta-audit-discriminator-type-bias-2026-04-25.md` rows 1, 5, 6, 7
and lead-B leg)

| BT | candidate | verdict | discriminator type |
|----|-----------|---------|--------------------|
| 541 | Lead-B SLE_6 × GUE | PASS | distributional |
| 542 | Hirahara MCSP non-natural | PASS_LITERATURE | structural-literature |
| 543 | A4-ratio-only | FAIL | numerical-interval |
| 544 | Q1 KdV Gram-lattice | FAIL | discrete-equality |
| 545 | IHC (Hodge / structural-literature, per parent reports) | PASS | structural-literature |

(BT-545 IHC PASS is taken from the brief's stated tally; the parent
reports' 9-row sample primarily covers BT-541/542/543/544. The IHC
row is included only because the brief states it; this audit does
not re-derive the IHC verdict.)

### §3.2 Adding Perelman's 3 molts retrospectively

Perelman's resolution is a known PASS event; the three molts each
introduced a real new primitive (per §1) and were independently
verified (Kleiner–Lott 2008, Morgan–Tian 2007, Cao–Zhu 2006). Treated
as retrospective PASSes at the calibration grid:

| BT | candidate | verdict | discriminator type |
|----|-----------|---------|--------------------|
| 547 | M1 Ricci-flow-as-variational | PASS (retrospective) | structural-literature (variational-derivation) — closest fit |
| 547 | M2 W-entropy + κ-noncollapsing | PASS (retrospective) | OTHER (analytic-inequality-construction) — no clean L9 vocabulary match |
| 547 | M3 quantitative surgery + finite extinction | PASS (retrospective) | OTHER (procedure-class molt) — no clean L9 vocabulary match |

### §3.3 Combined 6-BT tally (n = 8 candidate-rows)

| # | BT | candidate | verdict | discriminator type |
|---|----|-----------|---------|--------------------|
| 1 | 541 | Lead-B SLE_6 × GUE | PASS | distributional |
| 2 | 542 | Hirahara MCSP | PASS | structural-literature |
| 3 | 543 | A4-ratio-only | FAIL | numerical-interval |
| 4 | 544 | Q1 KdV Gram | FAIL | discrete-equality |
| 5 | 545 | IHC | PASS | structural-literature |
| 6 | 547 | M1 (variational) | PASS | structural-literature (variational-derivation) |
| 7 | 547 | M2 (W-entropy) | PASS | OTHER (analytic-inequality-construction) |
| 8 | 547 | M3 (surgery + extinction) | PASS | OTHER (procedure-class) |

### §3.4 Discriminator-type 2×2 (collapsed) — does Perelman confirm or disturb the bias hypothesis?

Hypothesis under test (per `omega-meta-audit-discriminator-type-bias-
2026-04-25.md`):
- PASS family = distributional / structural-literature
- FAIL family = discrete-equality / numerical-interval / vacuous-magnitude

Adding Perelman:

|                                             | PASS | FAIL |
|---------------------------------------------|------|------|
| distributional / structural-literature       | 4 (rows 1, 2, 5, 6) | 0 |
| discrete-equality / numerical-interval / vacuous | 0 | 2 (rows 3, 4) |
| **OTHER (analytic-inequality / procedure-class)** | 2 (rows 7, 8) | 0 |

If OTHER is collapsed into "PASS-family adjacent" (since the two
Perelman OTHER molts both PASSed):

|                                             | PASS | FAIL |
|---------------------------------------------|------|------|
| distrib / struct-lit / OTHER (PASS-adjacent) | 6   | 0    |
| discrete-equality / numerical-interval       | 0   | 2    |

The hypothesis is **not disturbed**: no cross-cell entries.
The hypothesis is **broadened, not refuted**: the Perelman PASSes
are in two discriminator types (structural-literature, OTHER) that
were already in or compatible with the PASS-family. Critically,
**the OTHER label is necessary** because two of the three Perelman
discriminators are not in the L9 vocabulary {distributional,
structural-literature, sketch, discrete-equality, numerical-interval,
vacuous-magnitude, axiom-recast}.

### §3.5 Family-coverage observation (the central retrospective finding)

The bias hypothesis (PASS-family = distrib / struct-lit) is
**retrospectively stable** — adding Perelman does not produce
cross-cell entries. But the more important retrospective observation
is this:

**The L9 catalogue's candidate-generation reach does not include
the family of moves Perelman actually made.** All three Perelman
molts (M1 variational re-interpretation, M2 monotone-Lyapunov
construction, M3 procedure-class with parameter bounds) require
generators that the dfs-24-sourced catalogue does not contain. The
catalogue's dominant-family bias (per CATALOGUE_BIAS verdict) toward
"n=6 arithmetic predicts closed-form identity" is the **opposite
type** of generator from what Perelman's resolution required.

This is not a falsification of the bias hypothesis on the
discrimination axis — Perelman's PASSes do live in PASS-family-adjacent
discriminator types — but it is a **structural finding** about the
catalogue's generation axis: even with Perelman's archetype
explicitly cited as the calibration baseline (per §sec 2 of the
L9 probe design), the catalogue did not implement candidate
generators that could produce Perelman-like molts.

---

## §4 Calibration verdict

**L9_CATALOGUE_PARTIAL_FIT.**

### §4.1 Why not L9_CATALOGUE_FITS

A full FIT verdict would require the L9 catalogue to plausibly
generate Ricci-flow-as-variational, W-entropy-monotone, and
quantitative-surgery-with-finite-extinction as candidate frame-
shifts. The §2 per-molt table records all three as MISS under the
current generation pipeline:

- M1 has no "re-cast existing PDE as gradient of a functional"
  candidate slot in the catalogue;
- M2 has no "construct a monotone Lyapunov functional that
  discharges a structural obstruction" candidate slot;
- M3 has no procedure-class molt slot.

Three MISS rows out of three Perelman molts is too strong a
miss-rate for FITS. The L9 catalogue, **as currently generated**,
does not reach Perelman's family.

### §4.2 Why not L9_CATALOGUE_MISSES

A full MISSES verdict would require Perelman's molts to be entirely
outside the discriminator-type vocabulary as well as the candidate-
generation reach. The discriminator-type axis is partially fit:

- M1's discriminator (variational-derivation sketch) maps cleanly
  onto **structural-literature** in the L9 vocabulary;
- M2's discriminator (analytic-inequality-construction) is
  **adjacent** to structural-literature (post-construction) and
  to "sketch" (proof-outline form); the L9 vocabulary lacks a
  precise label but the type is in the same neighbourhood;
- M3's discriminator (procedure-class with parameter bounds) is
  the strongest miss — the L9 vocabulary has no native label.

The PASS-family of the bias hypothesis (distrib / struct-lit) is
**broadened** by Perelman, not violated. This is a partial fit:
discrimination axis ≈ 2/3 fit, generation axis 0/3 fit.

### §4.3 Why not INCONCLUSIVE

The retrospective grid is small (3 Perelman molts, n ≤ 16 even with
the 6-BT inclusion). The verdict could be INCONCLUSIVE if the data
left the question open. They do not: the generation-axis 0/3 miss
is unambiguous (no catalogue generator targets PDE-variational,
analytic-Lyapunov, or procedure-class molts), and the discrimination-
axis 2/3 fit is also unambiguous. The mixed reading is precisely
what PARTIAL_FIT names.

### §4.4 Verdict statement

**L9_CATALOGUE_PARTIAL_FIT.** The L9 catalogue's discriminator-type
vocabulary captures roughly 2/3 of Perelman's molt discriminators
(M1 cleanly, M2 by adjacency, M3 not at all). The catalogue's
candidate-generation pipeline captures **0/3** of Perelman's molts:
no dfs-24-sourced generator produces variational re-interpretation,
analytic-Lyapunov construction, or procedure-class with parameter
bounds. The framework is internally consistent and discriminator-
adequate for a subset of molt families, but it does not reach
Perelman's archetype on the generation axis.

This calibration verdict is consistent with the parent
CATALOGUE_BIAS verdict (`omega-meta-audit-l9-catalogue-design-
2026-04-25.md`): both audits identify the **generation axis** as
the binding constraint. The discrimination axis, per the
discriminator-type-bias audit, is CONFOUNDED but not actively
broken.

---

## §5 Catalogue extension recommendations

Per the PARTIAL_FIT verdict, three classes of catalogue extension
are recommended. Each is read-only relative to the current L9
file (no in-place edits per the parent reports' hard constraints);
each is implementable in a follow-up session.

### §5.1 EXT-A: variational-re-interpretation candidate class (HIGH)

**Action**: add a candidate-generator class for "re-cast an existing
PDE / dynamical system as the gradient flow of a functional on an
extended configuration space". Prototype seeds (per existing repo
material):

- BT-541: re-cast the Riemann ξ-functional flow under the family of
  Mellin-pulled distributions as a gradient flow of a free-energy
  on a moduli space of zero-distributions. (Targets the same
  monotone-invariant lacuna identified in BT-547 §sec 6 lesson 1.)
- BT-543: re-cast Yang-Mills heat flow as gradient of a Yang-Mills
  + Lagrangian-multiplier functional on connection × Higgs space
  (precedent: Donaldson 1985, Uhlenbeck-Yau 1986).
- BT-544: re-cast 3D Navier-Stokes as gradient of a relative-
  entropy functional on the (u, ω) phase space — in the spirit of
  Perelman's F-functional re-cast. (This is closest in spirit to
  Perelman's M1.)

**Discriminator type**: structural-literature with variational-
derivation sub-test.

**Cost**: medium (each candidate requires 2-3 hours of literature
threading + variational-derivation sketch).

**Signal**: high — populates the L5 dream / L10 forge rungs that
the §sec 2 BT-547 ladder profile flagged as under-resourced.

### §5.2 EXT-B: analytic-Lyapunov-construction candidate class (HIGH)

**Action**: add a candidate-generator class for "construct a
monotone Lyapunov functional W on the BT's natural state space such
that dW/dt has a definite sign along the BT's natural dynamics, and
W discharges a known structural obstruction".

Prototype seeds (per `omega-cycle-bt547-poincare-2026-04-25.md`
§sec 6 lesson 1, "for each active BT, the omega cycle should
explicitly ask 'what is the candidate monotone invariant?' and
admit 'none yet' rather than substitute static identities"):

- BT-541: prime-counting / zero-density-dependent Lyapunov on the
  ξ-flow.
- BT-542: meta-complexity-resource-monotone (e.g. minimum-circuit-
  size as a Lyapunov along Hirahara reductions).
- BT-543: chromomagnetic-energy Lyapunov on lattice configurations.
- BT-544: *the* missing piece — an enstrophy-coupled-with-pressure
  Lyapunov; explicitly the BT-544 lacuna from BT-547 §sec 6 lesson 1
  (BT-544 carries Y6 NS energy / BKM as the *only* partial monotone
  analogue across the active 6 BTs).

**Discriminator type**: OTHER (analytic-inequality-construction)
when the inequality is being newly constructed; structural-
literature when the inequality is post-Perelman / post-Hamilton.

**Cost**: high (each candidate is a research-paper-scale proposal,
not a session-scale derivation). This is the Perelman-archetype
class.

**Signal**: very high — this is the specific lacuna the BT-547
lesson 1 calls out, and is the load-bearing molt class of Perelman's
resolution.

### §5.3 EXT-C: procedure-class molt with parameter bounds (MEDIUM)

**Action**: add a candidate-generator class for "specify a
procedure (e.g. surgery, Galois descent step, regulator
construction) with parameters (δ_i, r_i, …) such that the parameter
choices are consistent across all stages, and a termination /
exhaustion argument closes the procedure".

Prototype seeds:

- BT-541: explicit-formula descent procedure with bounded error
  terms across explicit-formula intervals.
- BT-543: surgical-quantization scheme on lattice gauge with
  parameter (a, β, κ) bounds.
- BT-544: weak-strong-uniqueness procedure with bounded measure-
  theoretic parameters along Buckmaster-Vicol-type families.
- BT-546: Kolyvagin-derivative procedure with bounded twists.

**Discriminator type**: OTHER (procedure-class with parameter
bounds). The L9 vocabulary should be extended to admit this label
explicitly.

**Cost**: high (procedure-class molts are the most ambitious).

**Signal**: medium-to-high — populates the L9 + L10 + L_ω
conjunction that BT-547 §sec 2 found Perelman strong on.

### §5.4 EXT-D: extend L9 discriminator vocabulary (LOW)

**Action**: extend the controlled discriminator-type vocabulary
from the current 7 labels {distributional, structural-literature,
sketch, discrete-equality, numerical-interval, vacuous-magnitude,
axiom-recast} to include:

- **analytic-inequality-construction** (for M2-class molts);
- **procedure-class-with-parameter-bounds** (for M3-class molts);
- **variational-re-interpretation** (for M1-class molts; possibly
  a sub-type of structural-literature).

**Cost**: very low (vocabulary extension in a 1-paragraph note).

**Signal**: low directly, but enables EXT-A/B/C labelling without
collapsing into "OTHER".

---

## §6 Predictive implications for unsolved BTs (especially BT-544)

The PARTIAL_FIT verdict + EXT-A/B/C recommendations have specific
predictive consequences for the active 6 BTs.

### §6.1 BT-544 NS — closest structural neighbour to Perelman / Ricci flow

Among the 6 active BTs, BT-544 NS is the geometric-PDE-evolutionary
nearest neighbour to the Ricci flow:
- Both are second-order parabolic / parabolic-like flows on a
  geometric / fluid configuration space.
- Both have a 20-year-plus tool-accumulation phase (Hamilton 1982-
  2002 ↔ NS Beale-Kato-Majda 1984 / Buckmaster-Vicol 2019 / Isett
  2018, ≈ 35 years of partial-result corpus).
- Both have a known-monotone partial analogue (BKM criterion ↔
  Hamilton's Riemannian-curvature monotonicity along Ric > 0
  initial data).
- Neither has a load-bearing Lyapunov functional that discharges
  the *full* obstruction (NS lacks the analogue of Perelman's
  W-entropy; Hamilton lacked it pre-Perelman).

**Prediction**: a closure of BT-544 will require an EXT-B-class
analytic-Lyapunov construction. The current L9 catalogue's BT-544
rows (Q1 algebraic-lattice, Q5 mechanism-Sobolev seed, KPZ d=7
scaling) are all in the dominant-family bias (per CATALOGUE_BIAS)
and **not** in the EXT-B class. The catalogue, as written, is
structurally unable to license the molt class that the Perelman-
archetype calibration says is required for a closure on this BT.

**Action implication**: BT-544 should be among the first to receive
EXT-A and EXT-B candidate generators. The HVC Bekenstein cap (D1.1
FAIL) and the 2.5D non-local-pressure axis A (D3.A PASS_LITERATURE)
already give partial coverage of EXT-A; an explicit EXT-B Lyapunov
candidate is missing.

### §6.2 BT-541 RH — partial neighbour via explicit-formula descent

BT-541 has a partial-monotone analogue (the Hadamard / De La Vallée-
Poussin explicit formulas) and an extensive 120-year tool-
accumulation. The Lead-B SLE_6 × GUE PASS (distributional
discriminator) is a real-molt verdict per the parent reports. Lead-B
is **not** in the EXT-B class — it is a coupling-statistics
identity, in the dominant-family. A full closure on BT-541 likely
requires an EXT-B candidate analogous to "construct a monotone
along the ξ-flow that discharges the off-line zero obstruction".

**Prediction**: BT-541 closure (if it occurs in this framework)
needs an EXT-B candidate, not just Lead-B. The current PASS is a
gateway, not a closure.

### §6.3 BT-543 YM — neighbour via lattice + variational

BT-543 has multiple partial-monotone candidates (lattice action,
energy density) and an extensive 50-year tool-accumulation. The A4-
ratio-only FAIL (numerical-interval discriminator) is in the
FAIL-family per the bias hypothesis. A full closure on BT-543
likely requires an EXT-A candidate (variational re-cast of the
Yang-Mills heat flow) + EXT-B candidate (mass-gap-discharging
Lyapunov).

**Prediction**: BT-543's catalogue should be diversified per
EXT-A/B before further FAIL-family candidates are run.

### §6.4 BT-542 P vs NP — outside the geometric-PDE neighbourhood

BT-542 Hirahara PASS (structural-literature) is structurally
different from M1/M2/M3. The 4-barriers wall is not a geometric-PDE
obstruction; it is a meta-complexity barrier. EXT-B (analytic-
Lyapunov) does not directly transfer; the BT-542-native EXT-class
would be "meta-complexity-monotone construction" (Hirahara 2018-
class). The Hirahara PASS is in this native class.

**Prediction**: BT-542 closure requires extension of the meta-
complexity-monotone class beyond Hirahara 2018, not EXT-A/B/C in
the geometric form.

### §6.5 Summary predictive table

| BT | nearest Perelman-archetype class | required catalogue extension | expected timeline |
|----|----------------------------------|------------------------------|--------------------|
| 541 | EXT-B (Lyapunov on ξ-flow) | EXT-B priority | decadal |
| 542 | meta-complexity-monotone (BT-542-native) | EXT-class for meta-complexity | decadal |
| 543 | EXT-A + EXT-B | EXT-A + EXT-B priority | decadal |
| 544 | EXT-A + EXT-B (closest to Perelman) | EXT-A + EXT-B priority — highest urgency | decadal-to-annual |
| 545 | structural-literature (IHC partial) | EXT-A possibly | decadal |
| 546 | EXT-C (Kolyvagin-derivative procedure) | EXT-C priority | decadal |

This is consistent with `omega-cycle-bt547-poincare-2026-04-25.md`
§sec 6 lesson 2 ("a 20-year tool-accumulation phase preceded
Perelman's 0.7-year surge … closure expectations should be set
decadal, not annual").

---

## §7 Anti-list — alternative classifications considered

Alternative readings of the §2 per-molt classification that were
considered and rejected. One-line reason each.

- **Anti-A (M1 = relabelling, not molt)**: read M1 as a notational
  re-naming of Hamilton's heat flow rather than a real frame-shift.
  Rejected: Perelman 2002 §1 explicitly introduces a *new
  configuration space* (metric × density) and a *new flow* (gradient
  of F, not Ricci flow proper) that *coincides with* Ricci flow up
  to diffeomorphism after a τ-rescaling. This is more than a
  re-naming — it is a re-frame with computable consequences (W-
  monotonicity, κ-noncollapsing).

- **Anti-B (M2 discriminator = sketch)**: classify M2's
  discriminator as "sketch" in the L9 vocabulary rather than
  OTHER. Rejected because "sketch" in the L9 audit refers to
  proof-outline level discriminators (e.g. dispatch-table-of-
  proof-steps); M2's discriminator is a published analytic
  inequality with an explicit derivation. Closer to structural-
  literature post-publication, OTHER for the construction event.

- **Anti-C (M3 = sub-molt of M2)**: collapse M3 into M2 since
  surgery is implemented using W-entropy bounds. Rejected: surgery
  uses the κ-noncollapsing consequence of W-monotonicity but adds
  the *parameter-bounded procedure* and the *finite-extinction
  sweep-out* — these are independent primitives. Cleaner to count
  as a separate molt.

- **Anti-D (catalogue actually fits Perelman via L_ω escape
  hatch)**: argue that L_ω axiom-recast (per `omega-meta-audit-
  discriminator-type-bias-2026-04-25.md` §6 D2 R1/R5 ACCEPTABLE
  rows) covers all three Perelman molts as "axiom recasts". Rejected:
  axiom-recast in the L9 framework refers to L_ω axiom-level frame
  changes (recasting Clay-form NS as Onsager-Hölder threshold),
  not to mid-frame molts within an established PDE programme.
  Perelman's molts are within the established Ricci-flow / Hamilton
  programme, not axiom-level recasts.

- **Anti-E (Perelman PASSes count toward F-MOLT-A)**: include
  Perelman's 3 PASSes in the F-MOLT-A 6-BT tally as if they were
  active-BT validations, lowering F-MOLT-A's distance-to-firing.
  Rejected: F-MOLT-A is defined on **active** BTs (541-546), not
  retrospective external solved BTs. Adding Perelman would
  conflate retrospective calibration with active-validation, and
  is explicitly out of scope per the brief's hard constraint
  "DO NOT make new claims about Poincaré".

- **Anti-F (verdict = INCONCLUSIVE)**: argue n=3 Perelman molts
  is too small for any verdict. Rejected: the 0/3 generation-axis
  miss is unambiguous (no current catalogue generator produces
  any of M1/M2/M3); the ≈ 2/3 discrimination-axis fit is also
  unambiguous. PARTIAL_FIT is the honest characterization, not
  INCONCLUSIVE.

- **Anti-G (CATALOGUE_FITS via L9 §3 catalogue having
  "monotone-invariant" sub-axis)**: argue that BT-544 Q5
  (Sobolev/Besov mechanism seed) and BT-541 Lead-C (M-set noise
  floor on Bernoulli numerator) are EXT-B-class candidates already
  in the catalogue. Rejected: per `omega-meta-audit-l9-catalogue-
  design-2026-04-25.md` §1.2, Q5 produced 0 category-(b) estimates
  and reduces to "arithmetic without analysis"; Lead-C is a
  distributional bound on a specific arithmetic identity, not a
  Lyapunov functional construction. Neither qualifies as EXT-B.

---

## §8 Falsifiers active for the retrospective fit

Falsifiers under which **this very calibration's PARTIAL_FIT
verdict** would be retracted. Distinct from BT-level and gate-level
falsifiers.

- **F-RETRO-A** (Perelman-molt-mis-attribution): if any of M1, M2,
  M3 in §1 mis-attributes a primitive to Perelman that is in fact
  in pre-2002 Hamilton-school literature, the §2 catalogue-coverage
  miss is over-stated. Cross-check: the W-functional and its
  monotonicity (M2) appear first in Perelman 2002 §3.4 (no pre-2002
  Hamilton-school precedent for the *τ*-rescaled W-form is
  documented in the standard verification corpus); the quantitative
  surgery parameters (M3) appear first in Perelman 2003a §4-§5
  (Hamilton 1997 had qualitative surgery only). Risk: low. **Not
  active**.

- **F-RETRO-B** (M3 = pure-engineering-not-molt): if M3 is read as
  a *technical implementation* of M2 (Hamilton-style surgery using
  Perelman's bounds) rather than an independent molt, the §2 M3
  classification collapses into M2. The PARTIAL_FIT verdict still
  holds (M2 still misses on the generation axis), but the §5 EXT-C
  recommendation loses standing. Risk: medium. The independence of
  M3 rests on the finite-extinction / sweep-out argument
  (Perelman 2003b), which uses M2 as input but adds the loop-space
  energy primitive — independence is defensible but not airtight.
  **Partially active**: §5 EXT-C should be flagged as conditional
  on M3 independence.

- **F-RETRO-C** (discriminator-vocabulary-extension-already-implicit):
  if the L9 vocabulary's "OTHER" label is read as covering analytic-
  inequality-construction and procedure-class molts implicitly, then
  EXT-D (vocabulary extension) is redundant and the discrimination-
  axis fit is 3/3 rather than 2/3. The verdict shifts toward FITS.
  Counter-argument: per `omega-meta-audit-discriminator-type-bias-
  2026-04-25.md` §1, the OTHER label is mentioned as a residual
  category but the audit's hypothesis is stated on the 7-element
  vocabulary; "OTHER" is not a positively-defined PASS-family
  member. Risk: low. **Not active**, but EXT-D is correspondingly
  low-priority.

- **F-RETRO-D** (catalogue-extension-already-planned-in-D-seeds):
  if the user-prompt "open seeds D1, D2, D3" mentioned in
  `omega-meta-audit-l9-catalogue-design-2026-04-25.md` §4.4 already
  cover EXT-A/B/C, then the §5 recommendations are redundant. Per
  the parent audit's §5.1: D1 = atlas-side scan, D2 = L_ω axiom-
  level frame change, D3 = decouple BT-544 mechanism axis. None
  of D1/D2/D3 explicitly names variational-re-cast (EXT-A),
  analytic-Lyapunov (EXT-B), or procedure-class (EXT-C) generation.
  D2 is closest to EXT-C in spirit but operates at L_ω axiom-level,
  not L9 procedure-class. Risk: low. **Not active**.

- **F-RETRO-E** (Perelman-molt-not-the-only-Millennium-archetype):
  if a future Millennium resolution (e.g. on BT-541 RH) follows a
  family of molts disjoint from M1/M2/M3 (e.g. random-matrix-
  universality + L-function families + Selberg trace), the
  retrospective fit on Perelman alone is a single-data-point
  calibration. The PARTIAL_FIT verdict is correctly conditional on
  Perelman as the only Millennium-resolved archetype. **Not active
  in this audit; conditional on the next Millennium resolution.**

- **F-RETRO-F** (PARTIAL_FIT-verdict-set-strict): if the verdict
  set {FITS, PARTIAL_FIT, MISSES, INCONCLUSIVE} is interpreted
  narrowly such that PARTIAL_FIT requires fit on > 1/2 axes, then
  the §4 verdict (≈ 2/3 fit on discrimination, 0/3 fit on
  generation; net ≈ 1/3 average) edges toward MISSES. Counter-
  argument: PARTIAL_FIT is the appropriate label when at least one
  axis is partially captured — discrimination is. Risk: medium.
  A stricter reader may downgrade to MISSES; the recommendation
  set §5 is identical either way. **Partially active**: the
  verdict could be re-graded to MISSES under a stricter reading.

- **F-RETRO-G** (n6-architecture-perspective-confound): the L9
  catalogue is sourced from n6-architecture's dfs-24 direction
  probes, which by construction emphasize n=6 arithmetic. Perelman
  worked on n=3 (3-manifolds) in a non-n6 frame. Critics may argue
  the §2 catalogue-coverage miss is **expected** because n6 is the
  wrong frame for Perelman, not a deficit of the L9 architecture.
  Counter-argument: the L9 catalogue's design (per L9 §1) does not
  intrinsically require n=6 framing; the n=6 bias is inherited from
  the dfs-24 sourcing decision (per CATALOGUE_BIAS). Extensions
  EXT-A/B/C are explicitly frame-agnostic. Risk: low. **Not
  active**, but the n6-frame inheritance is the structural reason
  for the bias, per §3.5.

---

## §9 Closing

**Verdict**: L9_CATALOGUE_PARTIAL_FIT. The L9 catalogue's
discriminator-type vocabulary captures roughly 2/3 of Perelman's
three molt discriminators (M1 cleanly via structural-literature,
M2 by adjacency, M3 not at all). The catalogue's candidate-
generation pipeline captures **0/3** of Perelman's molts. The
framework is internally consistent and discriminator-adequate for
a subset of molt families, but it does not reach Perelman's
archetype on the generation axis.

**3-molt classification (rung / discriminator-type)**:
- M1 Ricci flow as variational object — L5 dream + L10 forge / structural-literature (variational-derivation)
- M2 W-entropy + κ-noncollapsing — L9 + L10 / OTHER (analytic-inequality-construction)
- M3 quantitative surgery + finite extinction — L9 + L10 + L_ω / OTHER (procedure-class with parameter bounds)

**F-MOLT-A 6-BT updated tally (with retrospective Perelman PASSes)**:
- BT-541 Lead-B: PASS (distributional)
- BT-542 Hirahara: PASS (structural-literature)
- BT-543 P3: FAIL (numerical-interval)
- BT-544 Q1: FAIL (discrete-equality)
- BT-545 IHC: PASS (structural-literature)
- BT-547 M1: PASS retrospective (structural-literature)
- BT-547 M2: PASS retrospective (OTHER, analytic-inequality)
- BT-547 M3: PASS retrospective (OTHER, procedure-class)

Active F-MOLT-A status (active BTs only, retrospective PASSes
excluded per Anti-E): NOT FIRED. Distance to firing unchanged.
Bias hypothesis (PASS-family = distrib / struct-lit) **not
disturbed** by Perelman additions; broadened by OTHER-class.

**Catalogue extension recommendations**: EXT-A (variational re-
interpretation), EXT-B (analytic-Lyapunov construction), EXT-C
(procedure-class with parameter bounds), EXT-D (vocabulary
extension). EXT-B has highest urgency for BT-544.

**0/7 unchanged. No atlas/state/inventory edits.**
