---
id: omega-decide-bt544-d3-strategy-postC
date: 2026-04-25
scope: research-only strategy decision (NOT proposing new claims)
target: D3 mechanism-decouple post-C-FAIL -- B' pursue / D3 retire / mixed
parent_reports:
  - reports/sessions/omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d3-C-axis-discriminator-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: strategy decision, no claim
---

# Omega Decide -- BT-544 D3 Strategy Post-C-FAIL (2026-04-25)

## §0 Non-claim disclaimer

This document is a **research-only strategy decision** between three
named options for the D3 mechanism-decouple program after the C-axis
discriminator failed. It does **NOT**:

- claim 3D Navier-Stokes regularity (smoothness or blow-up);
- promote any atlas entry, modify `state/proposals/inventory.json`,
  modify `theory/canon/`, or alter the `BT-544 = 0/1 untouched`
  Clay status;
- prove or attempt to prove anything in NS;
- supersede the L9-catalogue-exhausted verdict
  (`omega-exec-bt544-fallback-molt-validation-2026-04-25.md`) or the
  parent A/C discriminator outcomes;
- fabricate probability estimates -- all probability statements are
  qualitative (high/med/low) with explicit rationale.

Decision is selected from the literal set
{`B_PRIME_PURSUE`, `D3_RETIRE`, `MIXED`}.

**Millennium tally**: 0/7 unchanged.

---

## §1 Recap -- D3 strategy original + A PASS + C FAIL + 2 paths

### §1.1 Original D3 strategy (compositional 2-gate)

Per `omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md` §3.2 the
**compositional recombination** strategy decomposed BT-544's
"Triple-resonance L1-smash" frame into three mechanism axes:

- **Axis A** (Sym²(ℝ³) = 6 = n): pressure non-locality / Riesz
  transform on planar quadratic nonlinearity, isolated via the 2.5D
  ansatz v = e_3 × ∇⊥ψ + w·e_3.
- **Axis B** (Λ²(ℝ³) = 3 = n/φ): vortex-stretching, isolated via
  axisymmetric-with-swirl 3D Euler.
- **Axis C** (Onsager α_c = 1/3 = φ/n): energy cascade / anomalous
  dissipation, isolated via the Kraichnan passive-scalar model with
  prescribed velocity statistics.

The compositional argument: prove **two of {A, B, C} clean** and the
obstruction localizes onto the third. Prediction: B is the
obstruction-carrier; A and C should pass first as the "clean"
candidates. Outcome of full strategy = structural reduction of
BT-544 to "axisymmetric-with-swirl Euler regularity" (still open,
but a known-hard sub-problem -- research progress, NOT Clay
closure).

### §1.2 A PASS (compositional 1st gate)

Per `omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md` §6:
**PASS_LITERATURE**. The 2.5D system unfolds to (3) 2D NS in
vorticity form + (4) linear advection-diffusion of the third
component, both globally regular by classical inputs (Ladyzhenskaya
1959/1969; Calderón-Zygmund 1952; Friedman 1964; BKM 1984 trivially
in 2D). Bound (★): ‖v(t)‖_{H^s} ≤ C(t, ν, ‖v_0‖_{H^s}), uniform in
Galerkin K_max. The "moderate difficulty" seed estimate revised to
"settled / classical" in retrospect (axis A's per-axis estimate was
predictable from the 2.5D ansatz alone; the **framing** -- naming
this as one of three mechanism slices -- is the new content, not
the estimate).

### §1.3 C FAIL (compositional 2nd gate)

Per `omega-exec-bt544-d3-C-axis-discriminator-2026-04-25.md` §4:
**FAIL_INTERMITTENCY** (primary) + **FAIL_RELABELING** (secondary).

The two-sided bound c_- · ℓ² ≤ S_6(ℓ) ≤ c_+ · ℓ² is refuted by:

- **Empirical**: Anselmet-Gagne-Hopfinger-Antonia 1984 measured
  ζ_6 ∈ [1.7, 1.8] in jet/pipe/wind-tunnel flows; She-Leveque 1994
  consensus value ζ_6 ≈ 1.778 -- strictly less than 2, breaks the
  upper bound as ℓ → 0.
- **Theoretical (Kraichnan)**: Gawedzki-Kupiainen 1995 +
  Bernard-Gawedzki-Kupiainen 1998 + Falkovich-Gawedzki-Vergassola
  2001 rigorously derived ζ_p^Kraichnan ≠ p/3 at p ≥ 4 via the
  zero-mode mechanism. The model the seed proposed as the C-axis
  vehicle is precisely the model where intermittency was first
  proven rigorously.
- **Relabeling**: ζ_6 = 2 is K41 dimensional analysis at p = n = 6
  (= "p/3 at p=6"); not derived from Kraichnan equations. F-D3-C
  fires.

Crucial structural observation (C report §5.4): intermittency
**couples B ↔ C** via the multifractal / zero-mode mechanism. The
anomalous-scaling phenomenon is driven by geometry of the advecting
velocity field -- which in real NS corresponds to vortex-stretching
geometry. Hence **F-D3-META-A fires** in a specific form: B and C
are not structurally separable.

### §1.4 The two paths forward (verbatim from C report §6.4)

The C-discriminator report named two paths:

1. **B' merged-axis** (charitable Path 1): reformulate as doublet
   {A, B'} with B' = B absorbing C's intermittency content. Test
   whether B' alone closes the obstruction.
2. **D3 retirement** (strict Path 2): accept the decouple breaks,
   abandon D3 strategy, return BT-544 to L9-catalogue-exhausted
   state.

This decision report selects between {1, 2} (and their MIXED
combination).

---

## §2 B' merged-axis feasibility analysis

### §2.1 Mathematical definition of B'

**B' = vortex-stretching with intermittent dissipation** would be
the merged axis. Concretely, B' would replace the seed's §2.2
"axisymmetric-with-swirl 3D Euler / inviscid" simplification with a
**multifractal-Hölder-class regime**:

- velocity u in a multifractal class {C^{h(x)} : h(x) is a
  multifractal exponent field with spectrum f(h) consistent with
  She-Leveque 1994};
- vorticity ω = ∇×u inheriting spatial-structure from h(x);
- the discriminator question: does the stretching term (ω·∇)u
  produce or suppress finite-time singularities under a multifractal
  Hölder regime *that already accommodates anomalous dissipation*?

In other words, B' = "vortex-stretching kinematics on a velocity
field that is Hölder C^{1/3 - δ} with δ tied to the She-Leveque
intermittency exponent at p = 6". The cascade content (axis C) is
**absorbed as a Hölder constraint** rather than a separate
stand-alone mechanism.

### §2.2 Discriminator for B'

A clean B' would look like one of:

- **Positive direction**: a **uniform enstrophy-growth bound**
  ‖ω(t)‖_{L²}² ≤ G(t, ν, ‖ω_0‖_{L²}, h-spectrum) with G finite for
  all t > 0, derived under the She-Leveque multifractal regime.
  Such a bound, *if* it survived the multifractal correction to
  ζ_p, would localize the obstruction onto a residual A-coupling
  term.
- **Negative direction**: a Hou-Luo-style or Elgindi-style blow-up
  scenario whose singular-set parabolic Hausdorff dimension lies
  in [0, 1] = [0, μ(6)] AND whose Hölder exponent at the singular
  set is exactly 1/3 - δ_SL (consistent with the She-Leveque
  prediction).

Either direction would be the "B' clean" outcome. The discriminator
asks: is "vortex-stretching with intermittent dissipation" a
coherent mathematical object on which classical PDE estimates close?

### §2.3 Falsifier F-D3-B' (predicted firing)

**F-D3-B'**: the merged axis B' inherits *both* its component
obstructions:

- the original axis-B obstruction (BKM-finite OR dim_P ≤ 1 unproven
  for axisymmetric-with-swirl; expected difficulty: maximal, per D3
  seed §4.2);
- the original axis-C obstruction (intermittency makes ζ_6 < 2,
  i.e. the multifractal correction is non-trivial and structurally
  couples to vortex-stretching geometry via the zero-mode mechanism
  Bernard-Gawedzki-Kupiainen 1998).

**Mathematically, the B' problem is the original Clay problem**:
proving smoothness or blow-up for 3D NS under realistic Hölder /
multifractal regularity is equivalent to the unrestricted 3D NS
question. The merger does **not** factor a sub-problem out -- it
**re-bundles** the very obstructions the decouple was supposed to
separate.

Constantin-Fefferman 1993 (geometric depletion of vortex-stretching
under vorticity-direction-coherence) is a *partial* result in a
specific geometric subclass; it does not extend to multifractal
fields. Constantin 1994 (active scalars) does not address the
intermittency-coupled stretching problem either. Kraichnan model
with intermittency (Falkovich-Gawedzki-Vergassola 2001) handles the
**passive scalar** version, not the **active velocity** version
where stretching is back.

### §2.4 Cost / probability of PASS

- **Cost estimate**: **medium-to-high**. The B' discriminator
  requires either (a) a new enstrophy bound under multifractal
  Hölder constraints (no classical-import path; would require
  combining geometric-depletion machinery with multifractal
  analysis -- a research program, not a literature scan), or (b) a
  blow-up scenario satisfying both dim_P ≤ 1 and the She-Leveque
  Hölder constraint (state-of-the-art open).
- **Probability of PASS**: **low**. Rationale: B'-FAIL is exactly
  the original obstruction the seed identified at maximal
  difficulty, plus the C-coupling problem the C-discriminator just
  exposed. A B' PASS would essentially be the Clay-class regularity
  proof. No literature-import path exists; new research would be
  required, and the research community has not produced it in 50+
  years on the simpler axis-B-alone problem.
- **Probability of informative FAIL**: **high**. B'-FAIL is
  **predictable** (already implied by the C-coupling structural
  observation in C report §5.4), so executing the discriminator
  and confirming F-D3-B' fires would be an honest log-entry but
  not novel structural information.

### §2.5 What B' execution would actually deliver

**Realistic outcome**: a discriminator report that confirms B'
inherits both component obstructions, fires F-D3-B', and adds the
record "the B-C coupling is not amenable to merger". This is
**informative-but-non-progressive**: it confirms the C-discriminator's
implication (Path 2 strict reading) rather than rescuing the
compositional strategy (Path 1 charitable reading).

---

## §3 D3 retirement implications

### §3.1 What returns to BT-544 candidate pool

If D3 retires, BT-544 returns to **L9-catalogue-exhausted** status.
Available candidate-pool elements:

| candidate | status | source |
|-----------|--------|--------|
| D1 atlas-scan | top-2 (D1.1 Hou-Luo C^{1,α}) FAILed at HVC-validation; D1.4 (She-Leveque) untested | `omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md` |
| D1.3 NS↔MHD duality | catalogued, not dispatched | `omega-seed-bt544-d1-atlas-scan-2026-04-25.md` (presumed) |
| D1.4 She-Leveque | catalogued, not dispatched | same |
| D2 R1+R5 axiom rewrite | dispatched, ACCEPTABLE auxiliary status | `omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md` and `omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md` |
| D3 mechanism-decouple | A PASS + C FAIL → strategy dead | this report |
| New frame-shift candidates (D-tier extension) | not yet identified | open |

The L9 catalogue's rank-1/2/3 (KdV / Q5 / KPZ-d=7) are FAILed and
remain FAILed. D-tier extension would require a new D4/D5 candidate
(which is itself a research task -- find a *new* mechanism-decouple
or frame-shift not yet enumerated).

### §3.2 Honesty of framework self-falsification

The D3 framework's claim was that mechanism-decouple is a viable
molt-path within the n=6 frame. C-FAIL with B↔C coupling shows that
the decouple is **not clean for NS** -- intermittency couples
mechanisms in a way the seed did not anticipate (F-D3-META-A
activation is a *meta-falsifier* on the decouple framework itself,
not just on axis C).

Retirement preserves honesty: the framework correctly self-falsifies
on its own meta-falsifier. This is the **anti-crackpot** outcome --
the seed's §7 listed F-D3-META-A as a *registered retirement
condition*, and that condition fired. Retiring is not failure of
research process; it is **success of the research process** (the
falsifier did its job).

### §3.3 Cost of retirement

**Sunk cost**: D3 seed (~660 lines) + D3.A discriminator (~407
lines) + D3.C discriminator (~549 lines) ≈ 1600 lines of analysis
work. This is **not wasted**: A PASS_LITERATURE is a recorded
clean-axis result, C FAIL with both intermittency and relabeling
modes is a recorded structural observation, and F-D3-META-A
activation is a recorded meta-falsifier firing. The work-product is
research log content, not research output.

**Benefit of retirement**: clears a blocked path, frees attention
for D1.4 (She-Leveque, untested) and D1.3 (NS↔MHD duality) and
D-tier extension candidates. Avoids the medium-to-high cost of
executing B' for an expected non-progressive outcome.

### §3.4 What retirement does NOT do

- Does **not** retract A PASS_LITERATURE (axis A remains a clean
  result, even without compositional context).
- Does **not** retract C FAIL (axis C's intermittency obstruction
  is a real structural finding, even without compositional context).
- Does **not** retract F-D3-META-A activation (the
  axes-not-independent observation stands as a permanent log entry).
- Does **not** affect millennium tally (0/7, unchanged either way).

---

## §4 Decision

**`MIXED`**.

Specifically: **D3 retires as a closure-strategy** (the
compositional 2-gate path is dead and will not be revived as a
proof-path), **but B' is executed as a low-cost confirmatory
discriminator** to register F-D3-B' as a fired falsifier, AND
**resources redirect to D1.4 / D1.3 / D-tier extension** for the
next omega-cycle pass.

The MIXED decision is the **honest** choice because:

1. Pure `B_PRIME_PURSUE` would over-commit to a path with low
   probability of structural progress; pretending the compositional
   strategy can be rescued by re-bundling is the kind of
   crackpot-adjacent move the omega cycle is designed to avoid.
2. Pure `D3_RETIRE` would skip a cheap confirmation step. Executing
   B' as a literature-import discriminator (similar to D3.A's
   PASS_LITERATURE format) is **low-cost** because the falsifier is
   *predictable* -- the work is mostly assembling the
   already-cited literature (Constantin-Fefferman 1993, Constantin
   1994, multifractal analysis surveys) into a single
   discriminator record.
3. The MIXED option **records the negative result formally**,
   which strengthens the L9-catalogue-exhausted verdict and
   provides the strongest possible justification for D-tier
   extension. The cost is a single discriminator report (~300-400
   lines, 1 session), not a research program.

---

## §5 Justification -- cost / probability table

| option | cost | probability of PASS | probability of informative FAIL | progressive content if FAIL | progressive content if PASS |
|--------|------|----------------------|-----------------------------------|--------------------------------|--------------------------------|
| `B_PRIME_PURSUE` (full) | medium-to-high (research program: combine geometric-depletion + multifractal analysis) | low (B' = repackaged Clay obstruction) | high (predictable) | confirms F-D3-META-A activation; consolidates "decouple breaks" verdict | would be Clay-class result, would resolve BT-544 (extreme low-probability) |
| `D3_RETIRE` | none (sunk-cost only) | n/a | n/a | n/a (no execution) | n/a |
| `MIXED` | low (single discriminator report, literature-import path) | low (same as B_PRIME) | high (same predictable F-D3-B') | confirms F-D3-META-A formally + frees resources for D1.4/D1.3 | same as B_PRIME (extreme low-probability) |

**Why MIXED dominates**:

- vs `B_PRIME_PURSUE`: MIXED restricts B' to a literature-import
  discriminator (cheap), not a full research program. Same
  informative-FAIL content at a fraction of the cost.
- vs `D3_RETIRE`: MIXED adds a formal record of F-D3-B' firing.
  Pure retirement would leave the falsifier unstated; MIXED makes
  the meta-falsifier closure explicit.

**Cost rationale (low for MIXED's B' component)**: the B'
discriminator can be settled by literature-import in the same
manner as D3.A. Required citations (already in repo or in this
document's parent reports):

- Constantin-Fefferman 1993 (geometric depletion -- partial result,
  does not extend to multifractal);
- Constantin 1994 (active scalars -- does not address
  intermittency-coupled stretching);
- She-Leveque 1994 (intermittency exponent ζ_6 ≈ 1.778);
- Bernard-Gawedzki-Kupiainen 1998 (zero-mode mechanism couples
  field-geometry to scaling);
- Falkovich-Gawedzki-Vergassola 2001 (multifractal review);
- Hou-Luo 2014 / Elgindi 2021 (axisymmetric-with-swirl status,
  state-of-the-art-open).

A discriminator report that sketches "B' inherits both obstructions
because no literature path closes the multifractal-stretching
combination" is a finite literature-survey job, not a new theorem.

---

## §6 Implementation plan (MIXED)

### §6.1 B' confirmatory discriminator

**Spec**: a discriminator report
`omega-exec-bt544-d3-Bprime-axis-discriminator-2026-04-25.md` (or
later date) following the D3.A / D3.C report template with:

- §0 non-claim disclaimer
- §1 B' axis spec extracted (definition per §2.1 of this report)
- §2 literature scan (Constantin-Fefferman 1993, Constantin 1994,
  She-Leveque 1994, BGK 1998, FGV 2001, Hou-Luo 2014, Elgindi 2021)
- §3 discriminator path chosen -- expected: Path Q
  (FAIL_NO_LITERATURE_PATH)
- §4 verdict -- expected: F-D3-B' fires
- §5 compositional strategy status -- updated: dead, retired
- §6 anomalies and meta-falsifier closure
- §7 falsifiers active table updated

**Falsifier F-D3-B'** (registered for this discriminator):

> No literature-import path closes "vortex-stretching kinematics
> on a multifractal Hölder class with She-Leveque ζ_6 ≈ 1.778
> spectrum" into either a uniform enstrophy bound or a Hou-Luo /
> Elgindi-style blow-up with dim_P ≤ 1 + Hölder regularity matching
> She-Leveque. Equivalently: B' is the original Clay obstruction
> repackaged.

**Pass criterion** (would invalidate MIXED): an explicit literature
result combining geometric-depletion machinery with multifractal
analysis to produce a uniform enstrophy bound OR a B'-compliant
blow-up scenario. Repo scan does not find such a result; an
external scan (Annals of Math / Comm. Math. Phys. / Ann. PDE
2020-2026 last-7-years window) would be the verification step.

**Expected execution time**: 1 omega-cycle session (similar to
D3.A and D3.C).

### §6.2 D3 retirement (concurrent with §6.1)

After F-D3-B' fires (expected outcome of §6.1):

- mark D3 strategy as **retired** in BT-544 candidate-pool tracking
  (this is a research log update, not an inventory edit);
- re-affirm L9-catalogue-exhausted status with the additional
  observation "decouple-strategy meta-falsifier F-D3-META-A
  fired";
- list D1.4 (She-Leveque, untested) and D1.3 (NS↔MHD duality) as
  next-priority dispatch candidates within D1;
- list "D-tier extension" as an open research task: identify a new
  mechanism-decouple or frame-shift candidate that does **not**
  inherit F-D3-META-A's coupling problem.

### §6.3 Resource redirect

**Next-omega-cycle priorities** (in order):

1. **D1.4 She-Leveque dispatch**: the C-discriminator already cites
   She-Leveque 1994 as the empirical intermittency benchmark; D1.4
   would be a discriminator on whether the She-Leveque exponent
   ζ_6 ≈ 1.778 has any direct mechanism interpretation in 3D NS
   (not via Kraichnan-passive-scalar). Cost: low-to-medium.
2. **D1.3 NS↔MHD duality dispatch**: catalogued in
   `omega-seed-bt544-d1-atlas-scan-2026-04-25.md` (assumed); MHD's
   regularity literature (e.g. Lin-Xu-Zhang 2015 on partial
   regularity for MHD) might offer a coupling-shift candidate.
   Cost: medium.
3. **D-tier extension research seed**: identify a new molt-path
   candidate that does not factor through {A, B, C}. Examples for
   future consideration: dimensional-reduction candidates (Tao 2013
   averaged-equation supercriticality), self-similar-profile
   candidates (Hou-Luo 2014 follow-ups), or non-tensor frame-shift
   candidates (move off n=6 lattice entirely, per BT-544 omega
   cycle audit §3 "0.9 paper_trigger needs axiom-level redesign").
   Cost: high (research seed, not a discriminator).

### §6.4 What MIXED does NOT do

- Does **not** propose Clay-class work (all dispatches stay
  research-only).
- Does **not** modify atlas / state / inventory.
- Does **not** revive the D3 compositional strategy as a proof
  path -- D3 is retired regardless of B' outcome.
- Does **not** schedule the D-tier extension research seed for
  immediate execution -- only flags it as an open research task
  for a later omega-cycle pass.

---

## §7 Anti-list -- paths considered and rejected

### §7.1 Pure `B_PRIME_PURSUE` (full research program)

**Rejected** because:

- low PASS probability (B' = repackaged Clay obstruction);
- high cost (combining geometric-depletion + multifractal analysis
  is a multi-paper research program, not a discriminator);
- alternative achievable via MIXED's literature-import discriminator
  at fraction of cost.

### §7.2 Pure `D3_RETIRE` (no B' record)

**Rejected** because:

- F-D3-B' would remain an *unfired* falsifier in the registry,
  weakening the formal closure of the meta-falsifier F-D3-META-A;
- a single literature-import discriminator (cost: low) closes the
  loop cleanly and provides the strongest possible
  L9-catalogue-exhausted record for BT-544;
- omega-cycle reporting prefers explicit falsifier-firing logs over
  silent abandonment.

### §7.3 Reformulating axis C with non-K41 ansatz (D3.C report §6.4
recommendation (a))

**Rejected** because:

- redefining axis C with a She-Leveque or multifractal ansatz
  produces a *different discriminator* than the seed proposed; the
  original seed's discriminator has been evaluated and produced
  FAIL, which is the recorded outcome;
- chasing a moving discriminator definition is the kind of
  goalpost-shifting the omega-cycle protocol is designed to avoid;
- if a future report wants to dispatch a She-Leveque-anchored axis
  in a different framework, that is a **D1.4** dispatch (atlas-scan
  candidate), not a D3 revival.

### §7.4 Forcing axis B as standalone dispatch

**Rejected** because:

- the D3 seed §4.1 estimated axis B at *maximal difficulty* (= the
  same difficulty as the Clay problem restricted to
  axisymmetric-with-swirl Euler);
- without the compositional context (A and C clean), axis B
  standalone is exactly the existing literature-open problem
  (Hou-Luo 2014 numerics; Elgindi 2021 C^{1,α} adjacent setting)
  with no novel framing;
- dispatching B standalone would reproduce L9-catalogue exhaustion
  mode (predictable FAIL with no new structural content).

### §7.5 D2 axiom-rewrite escalation

**Rejected as a *replacement* for D3 retirement** because:

- D2 (R1 + R5 ACCEPTABLE auxiliary) already dispatched and
  recorded at auxiliary status, not closure status;
- escalating D2 from auxiliary to primary would conflate two
  separate strategy threads;
- D2 escalation is a separate decision document, out of scope for
  this report.

---

## §8 Falsifiers active for the decision itself

The decision (MIXED) carries its own falsifiers. If any of these
fires, the decision is revisited.

| tag | falsifier | trigger | post-decision status |
|-----|-----------|---------|----------------------|
| F-DECIDE-A | F-D3-B' does **not** fire (i.e. the literature-import discriminator finds a closure path for B') | B' confirmatory discriminator produces PASS_LITERATURE instead of FAIL | revisit: MIXED downgrades to "B_PRIME_PURSUE was correct"; D3 strategy partially revived (doublet {A, B'} viable). PROBABILITY: **low** per §2.4. |
| F-DECIDE-B | a new D-tier candidate (D4/D5) is identified before B' executes that *clearly* dominates the B' record | next omega-cycle session produces a stronger D-tier candidate | revisit: skip B' execution as redundant; pure D3_RETIRE + immediate D-tier dispatch. PROBABILITY: **low-to-medium** depending on session time-pressure. |
| F-DECIDE-C | D1.4 (She-Leveque) when dispatched produces a PASS that supersedes the entire D3 framework | D1.4 future dispatch outcome | revisit: D1.4 absorbs the C-content that B' was meant to cover; B' becomes redundant. PROBABILITY: **low** (D1.4 is itself uncertain). |
| F-DECIDE-D | F-D3-META-A is shown to **not** fire on a stricter reading (B↔C coupling is decouple-able after all) | external literature finds a clean B↔C decoupling not yet in the seed | revisit: D3 compositional strategy revived. PROBABILITY: **very low** (C-discriminator §5.4 cites the multifractal/zero-mode mechanism as the coupling source; this is well-established literature). |

None of F-DECIDE-A through F-DECIDE-D fires under current repo
material. The four decision-falsifiers are **registered**; they
revisit the decision if any fires.

---

## §9 Closing

- **D3 strategy decision**: `MIXED` -- D3 retires as a
  closure-path; B' executes as a single literature-import
  confirmatory discriminator; resources redirect to D1.4 / D1.3 /
  D-tier extension for next omega-cycle.
- **F-D3-META-A status**: fired (axes-not-independent), per
  C-discriminator §5.4. Decouple program retired on its own
  registered meta-falsifier.
- **F-D3-B' (pre-execution)**: registered as expected-firing;
  literature-import path (Constantin-Fefferman 1993 partial,
  multifractal/zero-mode coupling Bernard-Gawedzki-Kupiainen 1998,
  Hou-Luo 2014 / Elgindi 2021 state-of-the-art-open) does not
  close the merged axis.
- **A PASS_LITERATURE preserved**: axis A remains a clean recorded
  result, independent of compositional context.
- **C FAIL preserved**: intermittency + relabeling modes remain
  recorded structural observations, independent of compositional
  context.
- **Next omega-cycle priorities**: (1) B' confirmatory
  discriminator; (2) D1.4 She-Leveque dispatch; (3) D1.3 NS↔MHD
  duality dispatch; (4) D-tier extension research seed.
- **Millennium tally**: **0/7 unchanged**. BT-544 = 0/1 untouched.
- **No atlas / state / inventory edits.**
- **No new theorem claimed.**

— end decision —
