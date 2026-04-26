---
id: omega-exec-bt544-d3-Bprime-axis-discriminator
date: 2026-04-25
scope: research-only molt-validation via literature import (NO NS claim, NO atlas promotion)
target: BT-544 D3 B' axis -- vortex-stretching merged with intermittency, formal F-D3-B' test
parent_reports:
  - reports/sessions/omega-decide-bt544-d3-strategy-postC-2026-04-25.md (MIXED decision)
  - reports/sessions/omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d3-C-axis-discriminator-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: discriminator evaluation, no claim
---

# Omega Exec -- BT-544 D3.B' Axis Discriminator (2026-04-25)

## §0 Non-claim disclaimer

This report executes the B' confirmatory literature-import
discriminator authorized by the MIXED decision in
`omega-decide-bt544-d3-strategy-postC-2026-04-25.md` (§4, §6.1). It
does **NOT**:

- claim 3D Navier-Stokes regularity (smoothness or blow-up);
- promote any atlas entry, modify `state/proposals/inventory.json`,
  modify `theory/canon/`, or alter the `BT-544 = 0/1 untouched`
  Clay status;
- supersede the L9 catalogue exhaustion verdict in
  `omega-exec-bt544-fallback-molt-validation-2026-04-25.md` (the
  rank-1/2/3 candidates remain FAILed);
- supersede the D3.A PASS_LITERATURE verdict (axis A remains
  clean, independent of compositional context);
- supersede the D3.C FAIL_INTERMITTENCY + FAIL_RELABELING verdict
  (axis C remains refuted, independent of compositional context);
- claim a new theorem on vortex-stretching, multifractal analysis,
  or NS regularity. Where I cite a result, I cite the *attestable
  classical reference* by author / year / title, not a new proof
  or a fabricated theorem.

D3.B' is a **confirmatory discriminator**. PASS_LITERATURE means a
clean uniform B' bound exists in the literature; the four other
verdict strings (FAIL_NO_LITERATURE_PATH, FAIL_BLOWUP_PRECEDENT,
PARTIAL, INCONCLUSIVE) are listed in the parent decision report
§6.1 and the B' execution prompt. Either way, **0/7 unchanged**.

The expected outcome (per the parent decision §2.3, §5, §6.1) is
**FAIL_NO_LITERATURE_PATH** -- F-D3-B' fires, D3 program closes
officially.

**Millennium tally**: 0/7 unchanged. BT-544 = 0/1 untouched.

---

## §1 B' axis definition

### §1.1 Recap of B' construction

Per the parent decision report §2.1, the merged axis B' is
constructed by replacing the seed's §2.2 "axisymmetric-with-swirl
3D Euler / inviscid" simplification of axis B with a multifractal
Hölder-class regime that absorbs axis C's intermittency content:

- velocity u in a multifractal class {C^{h(x)} : h(x) is a
  multifractal exponent field with spectrum f(h) consistent with
  She-Leveque 1994};
- vorticity ω = ∇×u inheriting spatial-structure from h(x);
- the discriminator question: does the stretching term (ω·∇)u
  produce or suppress finite-time singularities under a
  multifractal Hölder regime *that already accommodates anomalous
  dissipation*?

In symbolic form: B' = "vortex-stretching kinematics on a velocity
field that is Hölder C^{1/3 - δ} with δ tied to the She-Leveque
intermittency exponent at p = 6". The cascade content (axis C) is
absorbed as a Hölder constraint rather than a separate stand-alone
mechanism.

### §1.2 The B' discriminator (formal)

A clean B' would look like one of:

- **Positive direction**: a uniform enstrophy-growth bound
  ‖ω(t)‖_{L²}² ≤ G(t, ν, ‖ω_0‖_{L²}, h-spectrum) with G finite
  for all t > 0, derived under the She-Leveque multifractal
  regime. Such a bound, *if* it survived the multifractal
  correction to ζ_p, would localize the obstruction onto a
  residual A-coupling term.
- **Negative direction (still classifying as "clean B'")**: a
  Hou-Luo-style or Elgindi-style blow-up scenario whose singular-
  set parabolic Hausdorff dimension lies in [0, 1] = [0, μ(6)]
  AND whose Hölder exponent at the singular set is exactly
  1/3 - δ_SL (consistent with the She-Leveque prediction).

Either direction would be the "B' clean" outcome. The literature-
import test asks: do the four cited candidates (Constantin-
Fefferman 1993, Bernard-Gawedzki-Kupiainen 1998, Hou-Luo 2014,
Elgindi 2021) -- *or any standard companion result* -- furnish
either of these in a closed form?

### §1.3 Falsifier F-D3-B' (registered for this discriminator)

**F-D3-B'** (per parent decision §6.1):

> No literature-import path closes "vortex-stretching kinematics
> on a multifractal Hölder class with She-Leveque ζ_6 ≈ 1.778
> spectrum" into either a uniform enstrophy bound or a Hou-Luo /
> Elgindi-style blow-up with dim_P ≤ 1 + Hölder regularity
> matching She-Leveque. Equivalently: B' is the original Clay
> obstruction repackaged.

F-D3-B' fires iff every cited result either (a) gives a partial
bound that does not close under intermittency coupling, (b) proves
blowup or non-uniqueness which contradicts uniform regularity, or
(c) treats a different geometric setting (axisymmetric, 2.5D,
passive scalar) that does not generalize to B'.

---

## §2 Literature scan -- four cited results

### §2.1 Constantin-Fefferman 1993

**Citation**: P. Constantin, C. Fefferman, "Direction of vorticity
and the problem of global regularity for the Navier-Stokes
equations", Indiana Univ. Math. J. 42 (1993), 775-789.

**Statement (paraphrased)**: if the direction of vorticity
ξ(x, t) := ω(x, t) / |ω(x, t)| is *Lipschitz / Hölder coherent*
on a region where |ω| is large -- specifically, if ξ has a
smoothness modulus controlled by a small constant times the
inverse of |ω| -- then the vortex-stretching term (ω·∇)u is
*depleted* (reduced in magnitude relative to its naive bound),
and the L²-enstrophy growth is controlled.

**B'-bound contribution**: the result furnishes a *partial*
enstrophy bound under a **direction-of-vorticity smoothness
hypothesis**. The hypothesis is a Lipschitz / Hölder regularity
assumption on ξ, *not* on u or ω themselves.

**Closure under multifractal regime**: under a She-Leveque /
multifractal Hölder regime, the vorticity field ω = ∇×u inherits
a spatially-varying Hölder exponent h(x) - 1; the direction
ξ = ω/|ω| inherits **at best** the *minimum* exponent in the
multifractal spectrum f(h), and at multifractal generic points
ξ is **not Lipschitz**. Concretely: She-Leveque predicts a
spectrum supported on h ∈ [h_min, h_max] with h_min ≈ 1/3 - δ_SL,
and the geometric depletion hypothesis requires a uniform
Lipschitz modulus on ξ that the multifractal field does not
possess.

**Verdict for B'**: Constantin-Fefferman 1993 gives a **partial
enstrophy bound** but the **closure hypothesis fails under
intermittency**. Result classified as **(a) partial bound that
does not close under intermittency coupling**. F-D3-B' criterion
(a) **satisfied** for this citation -- contributes to F-D3-B'
firing.

**Companion result -- Constantin 1994**: P. Constantin, "Geometric
statistics in turbulence", SIAM Rev. 36 (1994), 73-98. Reviews
geometric depletion and active-scalar machinery; same conclusion
applies (depletion requires direction-of-vorticity smoothness
that multifractal fields lack). Not a separate vehicle for B'
closure.

### §2.2 Bernard-Gawedzki-Kupiainen 1998

**Citation**: D. Bernard, K. Gawedzki, A. Kupiainen, "Slow modes
in passive advection", J. Stat. Phys. 90 (1998), 519-569.

**Statement (paraphrased)**: in the Kraichnan model of passive
scalar advection by a delta-correlated-in-time Gaussian velocity
ensemble, the anomalous scaling exponents ζ_p^{scalar} − p/3 ≠ 0
arise from **zero modes** of the n-point correlation operator --
slow modes that decouple from the forcing and dominate the
inertial-range scaling. The mechanism rigorously couples
field-geometry features (the velocity ensemble's Hölder exponent)
to the scalar's anomalous exponents.

**B'-bound contribution**: BGK 1998 is a *passive scalar* result.
The advecting velocity is **prescribed** (a Gaussian ensemble
with given roughness); it does not solve a momentum equation, and
in particular does **not** include vortex-stretching dynamics.

**Closure under B'**: B' requires the multifractal Hölder regime
to apply to the **active velocity field** in the NS equation,
where vortex-stretching (ω·∇)u feeds back on the velocity
geometry. BGK 1998's zero-mode mechanism is for a **linear**
advection equation; transplanting it to the nonlinear NS setting
would require a (currently unavailable) extension that handles
the back-reaction of the scalar on the carrier.

**Verdict for B'**: BGK 1998 explains *why* intermittency couples
to field geometry, which is the structural observation feeding
F-D3-META-A in the C-discriminator. It does **not** furnish a
B'-bound. Result classified as **(c) treats a different setting
(passive scalar, not active velocity) that does not generalize**.
F-D3-B' criterion (c) **satisfied** for this citation --
contributes to F-D3-B' firing.

**Companion result -- Falkovich-Gawedzki-Vergassola 2001**: G.
Falkovich, K. Gawedzki, M. Vergassola, "Particles and fields in
fluid turbulence", Rev. Mod. Phys. 73 (2001), 913-975. Review of
the Kraichnan-model machinery; same passive-scalar limitation.

### §2.3 Hou-Luo 2014

**Citation**: T. Y. Hou, G. Luo, "Toward the finite-time blowup
of the 3D axisymmetric Euler equations: a numerical investigation",
Multiscale Model. Simul. 12 (2014), 1722-1776 (also Proc. Natl.
Acad. Sci. 111 (2014), 12968-12973).

**Statement (paraphrased)**: high-resolution numerical computation
of axisymmetric 3D incompressible Euler with swirl, on a solid
boundary, exhibits self-similar blow-up at a ring of points on the
boundary. The maximum vorticity ‖ω(t)‖_{L^∞} appears to diverge
in finite time T*, with rates consistent with a self-similar
profile. **Numerical, not rigorous**.

**B'-bound contribution**: Hou-Luo 2014 gives **numerical
evidence for finite-time blow-up** in axisymmetric-with-swirl
Euler. Two implications for B':

1. The **positive direction** of B' (uniform enstrophy bound) is
   contradicted by Hou-Luo's blow-up scenario: ‖ω(t)‖_{L²}² grows
   without bound as t → T* in their computation, so no uniform
   G(t, ...) finite for all t exists in this geometric class.
2. The **negative direction** of B' (B'-compliant blow-up
   scenario) is *partially* available in Hou-Luo, but with
   important caveats:
   - the geometry is **axisymmetric-with-swirl**, not a generic
     multifractal Hölder field;
   - the regularity at the singular set is *not analyzed* in
     Hou-Luo 2014 in terms of a She-Leveque / multifractal
     spectrum -- the computation is in C^∞ initial data with
     numerically-resolved structure, not in a multifractal class;
   - the singular set is a **ring** (1-dimensional) at the
     boundary, but the *parabolic* Hausdorff dimension and the
     Hölder exponent at the singular set are not extracted in
     forms compatible with the B' criterion (dim_P ≤ μ(6),
     Hölder = 1/3 - δ_SL).

**Closure under B'**: Hou-Luo 2014 lives in axisymmetric-with-
swirl geometry, which is **not** the multifractal Hölder regime
B' specifies. The blow-up scenario suggests B' *positive direction*
is structurally false in this geometry but does not establish a
B'-compliant blow-up.

**Verdict for B'**: Hou-Luo 2014 contributes to F-D3-B' via
criterion **(b) blowup precedent that contradicts uniform
regularity** (for the positive direction) and criterion **(c)
different geometric setting (axisymmetric-with-swirl, not
multifractal)** (for the negative direction). Both criteria
**satisfied**.

### §2.4 Elgindi 2021

**Citation**: T. M. Elgindi, "Finite-time singularity formation
for C^{1,α} solutions to the incompressible Euler equations on
ℝ³", Annals of Mathematics 194 (2021), 647-727.

**Statement (paraphrased)**: there exists a class of C^{1,α}
initial data for the 3D incompressible Euler equations (for some
small α > 0) such that the corresponding solution develops a
finite-time singularity. The construction is **rigorous** (a
proof, not numerics) and uses a self-similar blow-up profile in
a specific geometric class (axisymmetric, no swirl, with a
fundamental modification to enable the finite-time blow-up).

**B'-bound contribution**: Elgindi 2021 is a **rigorous blow-up
result for Euler in C^{1,α}**. Two implications for B':

1. The **positive direction** of B' (uniform enstrophy bound for
   *Euler*) is **rigorously refuted** in C^{1,α} for some α: the
   solution blows up in finite time, so no uniform G(t, ...)
   finite for all t > 0 can exist in this regularity class.
2. The Hölder regularity of the blow-up profile is **C^{1,α}**
   for some small α > 0, which is *adjacent to* but **not the
   same as** the multifractal C^{1/3 - δ_SL} regime B' requires:
   - C^{1,α} = "u is C^1 with derivative Hölder-α", so
     u ∈ C^{1+α} ⊂ C^1, much smoother than C^{1/3} typical of
     turbulent fields;
   - the multifractal regime has *spatially-varying* exponent
     h(x) ∈ [h_min, h_max] with h_min ≈ 1/3 - δ_SL, ranging
     **below** 1; the Elgindi class is **above** 1.

**Closure under B'**: Elgindi 2021 demonstrates that in a
*specific* C^{1,α} geometric class (related to but smoother than
the B' multifractal regime, and in Euler not NS), uniform
regularity fails. It is the strongest *blow-up precedent* in the
literature for incompressible Euler. It does **not** establish
either: (i) the B' positive direction (which it refutes for Euler
in C^{1,α}), or (ii) a B'-compliant blow-up in the multifractal
class (it operates in a different Hölder class).

The takeaway: blow-up *lives* in the C^{1,α} adjacent class. The
multifractal class B' specifies is *rougher* than C^{1,α} and at
least as susceptible to blow-up phenomena -- there is no
reasonable expectation that uniform regularity is *easier* in a
rougher class.

**Verdict for B'**: Elgindi 2021 contributes to F-D3-B' via
criterion **(b) blow-up precedent that contradicts uniform
regularity**. The result is for **Euler**, not NS, but the
B' setup is essentially inviscid-leaning (multifractal Hölder is
the inviscid regime; viscosity at fixed ν > 0 regularizes but
does not change the inertial-range structure that B' is asking
about).

### §2.5 Synthesis table

| reference | result | B'-bound contribution | F-D3-B' criterion |
|-----------|--------|------------------------|-------------------|
| Constantin-Fefferman 1993 | geometric depletion under direction-of-vorticity smoothness | **partial bound; closure hypothesis fails under multifractal Hölder** | (a) partial bound, does not close under intermittency |
| Bernard-Gawedzki-Kupiainen 1998 | zero-mode mechanism for anomalous scaling in Kraichnan passive scalar | passive scalar, not active velocity; does not include vortex-stretching | (c) different setting (passive, not active) |
| Hou-Luo 2014 | numerical evidence for axisymmetric-with-swirl Euler blow-up | refutes B' positive direction in axisymmetric geometry; does not match multifractal class | (b) blow-up precedent + (c) different geometry |
| Elgindi 2021 | rigorous finite-time blow-up for Euler in C^{1,α} | rigorously refutes uniform regularity in adjacent class; not multifractal class | (b) blow-up precedent in adjacent class |

**Aggregate verdict from the four citations**: every one fails
the B' closure criterion. None furnishes either (i) a uniform
enstrophy bound under the multifractal regime, or (ii) a B'-
compliant blow-up scenario with the She-Leveque Hölder match.

The *combined* picture: Constantin-Fefferman gives partial
control under hypotheses that fail multifractally; BGK 1998
explains why intermittency mechanically couples to field geometry
but only in the passive regime; Hou-Luo 2014 numerically shows
axisymmetric-with-swirl blow-up in a non-multifractal class;
Elgindi 2021 rigorously shows blow-up in a non-multifractal but
adjacent class. **No clean B' bound is producible from this
literature.**

---

## §3 Verdict

### §3.1 Verdict string

**FAIL_NO_LITERATURE_PATH** (primary), with **FAIL_BLOWUP_PRECEDENT**
(secondary) co-active.

The primary mode is FAIL_NO_LITERATURE_PATH because all four cited
results (and their immediate companions) fail to close: every one
either gives a partial bound that does not extend, treats a
different setting, or contradicts uniform regularity in an
adjacent class. The secondary mode FAIL_BLOWUP_PRECEDENT is co-
active because Hou-Luo 2014 (numerical) and Elgindi 2021
(rigorous) jointly establish that **finite-time blow-up is the
empirical / structural expectation** in Hölder classes near the
B' regime; uniform regularity for B' is therefore not just
"unproven" but "structurally false in adjacent classes".

### §3.2 Why not PARTIAL or INCONCLUSIVE

- **Not PARTIAL**: a PARTIAL verdict would require some cited
  result to give *partial B' control* in a form that *could in
  principle* be extended. Constantin-Fefferman 1993 looks like a
  partial bound at first glance, but its closure hypothesis
  (Lipschitz / Hölder smoothness of ξ = ω/|ω|) **fails generically
  under the multifractal regime** -- it is not a partial bound
  with extension potential, it is a bound whose hypothesis is
  not even approximately met. PARTIAL is therefore **not the
  honest verdict**.

- **Not INCONCLUSIVE**: an INCONCLUSIVE verdict would require the
  literature scan to be underdetermined -- e.g. plausible
  unscanned results. The four cited candidates plus the C-axis
  literature already scanned (BGK 1998, FGV 2001) cover the
  relevant landscape: geometric depletion (Constantin-Fefferman),
  intermittency coupling (BGK / FGV), axisymmetric blow-up
  numerics (Hou-Luo), rigorous Euler blow-up in adjacent Hölder
  class (Elgindi). No major recent (2020-2026) result has bridged
  multifractal regularity and uniform NS bounds; the parent
  decision report's external-scan suggestion (§6.1) is consistent
  with this assessment. INCONCLUSIVE is therefore **not the
  honest verdict**.

- **Not PASS_LITERATURE**: this would require an explicit
  uniform-bound result for B' (vortex-stretching with multifractal
  intermittency). No such result exists in the cited literature
  or its standard companions.

### §3.3 Verdict string (final)

**FAIL_NO_LITERATURE_PATH** (with FAIL_BLOWUP_PRECEDENT co-
active).

---

## §4 F-D3-B' status

**F-D3-B' fires.**

Per the formal definition in §1.3:

> No literature-import path closes "vortex-stretching kinematics
> on a multifractal Hölder class with She-Leveque ζ_6 ≈ 1.778
> spectrum" into either a uniform enstrophy bound or a Hou-Luo /
> Elgindi-style blow-up with dim_P ≤ 1 + Hölder regularity
> matching She-Leveque.

The §2 literature scan establishes:

1. **No uniform enstrophy bound** is producible -- Constantin-
   Fefferman's bound has a hypothesis that fails multifractally;
   BGK 1998 / FGV 2001 are passive scalar.
2. **No B'-compliant blow-up** is producible -- Hou-Luo 2014 is
   axisymmetric-with-swirl in a non-multifractal class; Elgindi
   2021 is in C^{1,α} (smoother than B' multifractal class) and
   for Euler, not NS, with no She-Leveque exponent match.

Both clean-B' branches (positive enstrophy bound; negative
blow-up scenario) are unavailable in the literature. **F-D3-B'
fires.**

This matches the parent decision report §6.1 expected outcome
("registered as expected-firing").

---

## §5 D3 program closure status

**D3 program officially closed.**

### §5.1 Closure justification chain

1. **D3.A** (axis A) -- PASS_LITERATURE (clean axis), preserved.
2. **D3.C** (axis C) -- FAIL_INTERMITTENCY + FAIL_RELABELING,
   preserved.
3. **D3.C → F-D3-META-A activation** (axes-not-independent,
   intermittency couples B ↔ C) -- preserved.
4. **D3.B'** (this report) -- FAIL_NO_LITERATURE_PATH +
   FAIL_BLOWUP_PRECEDENT, F-D3-B' fires.
5. **D3 compositional 2-gate strategy**: only 1 of 2 required
   clean-axis gates passed (A passed, C failed, B' merger does
   not rescue). **Strategy dead.**

### §5.2 What "officially closed" means here

- The D3 mechanism-decouple program -- as defined in
  `omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md` -- is
  **retired** as a closure-strategy for BT-544.
- F-D3-META-A (axes-not-independent, registered retirement
  condition in seed §7) **fired**, in two specific forms: (i)
  intermittency couples B ↔ C (recorded in C-discriminator §5.4);
  (ii) merging B and C into B' inherits both component
  obstructions (recorded in this report §2-§4).
- D3 is **not revived** by any path enumerated in the parent
  decision §7 anti-list (no pure-B', no axis-C reformulation in
  D3 frame, no axis-B standalone in D3 frame).

### §5.3 What "officially closed" does NOT mean

- Does **not** retract A PASS_LITERATURE (axis A remains a clean
  recorded result, independent of compositional context).
- Does **not** retract C FAIL (axis C's intermittency obstruction
  remains a real structural finding, independent of compositional
  context).
- Does **not** retract F-D3-META-A activation (the
  axes-not-independent observation stands as a permanent log
  entry).
- Does **not** affect millennium tally (0/7, unchanged either
  way).
- Does **not** modify atlas / state / inventory.

### §5.4 Decision-level falsifiers (parent decision §8)

- **F-DECIDE-A** (F-D3-B' does not fire / B' literature path
  exists): **NOT TRIGGERED** -- F-D3-B' fires per §4 above.
- **F-DECIDE-B** (D-tier candidate dominates B' before
  execution): not triggered (B' executed in this session).
- **F-DECIDE-C** (D1.4 PASS supersedes D3 framework): not
  applicable (D1.4 not yet dispatched).
- **F-DECIDE-D** (F-D3-META-A shown to not fire): not triggered
  (F-D3-META-A confirmed firing by §4 and §5.1 above).

The MIXED decision is **confirmed** by this discriminator.

---

## §6 Implications for BT-544 next steps

### §6.1 Per parent decision §6.3 redirect priorities

After D3 program closure, BT-544 returns to L9-catalogue-
exhausted state with the additional observation that "decouple-
strategy meta-falsifier F-D3-META-A fired and B' merger does not
rescue". Next-omega-cycle priorities (in order, from parent
decision §6.3):

1. **D1.4 She-Leveque dispatch** -- discriminator on whether the
   She-Leveque exponent ζ_6 ≈ 1.778 has any direct mechanism
   interpretation in 3D NS (not via Kraichnan-passive-scalar).
   Cost: low-to-medium.
2. **D1.3 NS↔MHD duality dispatch** -- catalogued in
   `omega-seed-bt544-d1-atlas-scan-2026-04-25.md` (assumed); MHD
   regularity literature (e.g. Lin-Xu-Zhang 2015) might offer a
   coupling-shift candidate. Cost: medium.
3. **D-tier extension research seed** -- identify a new molt-path
   candidate that does not factor through {A, B, C}. Examples:
   dimensional-reduction (Tao 2013 averaged-equation
   supercriticality), self-similar-profile (Hou-Luo follow-ups),
   non-tensor frame-shift (move off n=6 lattice entirely). Cost:
   high (research seed, not a discriminator).

### §6.2 What this report does NOT trigger

- Does **not** schedule any of (1)-(3) for immediate execution --
  only reaffirms the parent decision's redirect priorities.
- Does **not** propose Clay-class work.
- Does **not** propose a new D3 revival path -- D3 is retired.
- Does **not** modify atlas / state / inventory.

### §6.3 Standing record after this report

- L9 catalogue exhausted: KdV / Q5 / KPZ-d=7 (top 3) -- FAILed
  (recorded in
  `omega-exec-bt544-fallback-molt-validation-2026-04-25.md`).
- D1 atlas-scan: D1.1 Hou-Luo C^{1,α} -- FAILed at HVC validation
  (`omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md`);
  D1.3 / D1.4 untested.
- D2 R1+R5 axiom rewrite -- ACCEPTABLE auxiliary status only.
- D3 mechanism-decouple -- **closed** (this report).
- D-tier extension -- open research task.

BT-544 status: **L9-catalogue-exhausted with D3 closure
appended**. 0/1 untouched. 0/7 unchanged.

---

## §7 Anti-list -- candidate references rejected

Several plausible-sounding references were considered as B'
vehicles but rejected because they do not generalize to the
B' multifractal regime or do not provide a uniform bound. The
anti-list is logged here for honesty.

### §7.1 Caffarelli-Kohn-Nirenberg 1982

**Citation**: L. Caffarelli, R. Kohn, L. Nirenberg, "Partial
regularity of suitable weak solutions of the Navier-Stokes
equations", Comm. Pure Appl. Math. 35 (1982), 771-831.

**Result**: the 1-dimensional parabolic Hausdorff measure of the
singular set of a suitable weak solution to 3D NS is zero
(P^1(S) = 0).

**Rejected because**: CKN bounds the *singular set size* but does
**not** give a uniform regularity bound -- the result is
compatible with a singular set being non-empty (just
1-dimensional-small in the parabolic sense), so it does not
deliver the B' positive-direction enstrophy bound. CKN is also a
**suitable weak solution** result, which is an existence-
regularity result, not a uniform-bound result keyed to a
multifractal Hölder class. Not a B' vehicle.

### §7.2 Tao 2013 averaged-equation supercriticality

**Citation**: T. Tao, "Finite time blowup for an averaged three-
dimensional Navier-Stokes equation", J. Amer. Math. Soc. 29
(2016), 601-674.

**Rejected because**: Tao 2013/2016 is an **averaged** NS
equation, not 3D NS itself. The averaging breaks the very
structure (vortex-stretching in its full nonlocal form) that B'
is asking about. A blow-up for the averaged equation does not
imply blow-up for B'; conversely, a regularity result for the
averaged equation does not deliver a B' bound. Tao 2013 is
listed in the parent decision §6.3 as a *D-tier extension seed
candidate*, not a B' literature-import vehicle. Not a B' vehicle.

### §7.3 Robinson-Rodrigo-Sadowski 2016 (textbook)

**Citation**: J. C. Robinson, J. L. Rodrigo, W. Sadowski, *The
Three-Dimensional Navier-Stokes Equations* (Cambridge Studies in
Advanced Mathematics 157, 2016).

**Rejected because**: comprehensive textbook on 3D NS; covers
Leray-Hopf weak solutions, Kato-Fujita strong solutions, partial
regularity, and BKM. Does **not** contain a uniform regularity
bound under multifractal Hölder regimes (the open Clay problem
is precisely the gap the textbook documents). Useful as a
companion reference, not a B' vehicle.

### §7.4 Beale-Kato-Majda 1984

**Citation**: J. T. Beale, T. Kato, A. Majda, "Remarks on the
breakdown of smooth solutions for the 3-D Euler equations",
Comm. Math. Phys. 94 (1984), 61-66.

**Rejected because**: BKM gives a **conditional** continuation
criterion: smoothness on [0, T] iff ∫₀^T ‖ω‖_∞ dt < ∞. BKM does
not deliver a B' uniform bound -- it *converts* one bound
(L²-enstrophy growth) to another (L^∞ vorticity time-integral),
and the L^∞-time-integral bound is *exactly the open question*
in the multifractal regime. Useful as a structural input (used
in D3.A §4 step 4), not a B' vehicle.

### §7.5 Axisymmetric-without-swirl results

**Citation**: O. A. Ladyzhenskaya 1968 / M. R. Uchovskii, B. I.
Yudovich 1968 / Chen-Strain-Tsai-Yau 2008/2009 (axisymmetric NS
without swirl, global smoothness).

**Rejected because**: these are **axisymmetric-without-swirl**
results, where vortex-stretching vanishes by ansatz (analogue to
the 2.5D system in axis A). They deliver a clean uniform bound
*precisely because vortex-stretching is absent*, which is the
opposite of what B' is asking (B' is about vortex-stretching
under intermittency). Used as the axis-A literature anchor, not
a B' vehicle.

### §7.6 Buckmaster-Vicol 2019 / Buckmaster-Masmoudi-Novack-Vicol 2023

**Citation**: T. Buckmaster, V. Vicol, "Nonuniqueness of weak
solutions to the Navier-Stokes equation", Annals of Mathematics
189 (2019), 101-144; T. Buckmaster, N. Masmoudi, M. Novack, V.
Vicol, "Intermittent convex-integration for the 3D Euler
equations", Ann. PDE (2023).

**Rejected because**: convex-integration constructs **non-unique
weak solutions** at low regularity. Non-uniqueness is *adjacent
to* but **distinct from** the B' question: B' asks about *uniform
regularity bounds* for solutions that exist, not about
uniqueness or existence in low-regularity spaces. The intermittent
convex-integration of BMNV 2023 does involve intermittent
solutions, but the construction is **adversarial** -- it produces
pathological solutions to demonstrate non-uniqueness, not a
uniform regularity bound for "good" solutions in the multifractal
regime. Cited in C-discriminator §2.6, not a B' vehicle.

### §7.7 Onsager 1949 / Constantin-E-Titi 1994 / Isett 2018

**Citation chain**: Onsager 1949 conjecture; Constantin-E-Titi
1994 positive direction (α > 1/3 ⇒ conservation); Isett 2018
negative direction (α < 1/3 ⇒ pathological weak solutions).

**Rejected because**: the Onsager program addresses **energy
conservation** for weak Euler solutions, not **uniform
regularity** for NS. The Onsager threshold α = 1/3 is the
*Hölder critical exponent* for energy conservation, but
conservation is a *one-sided* property (it does not preclude
loss of regularity at a singular set). A C^{1/3+} solution
conserves energy but might still develop singularities (the
Onsager program does not rule this out). Cited in C-discriminator
§2.4-§2.5, not a B' vehicle.

### §7.8 Aggregate anti-list rationale

The anti-list demonstrates that the literature-scan is **not
incidentally underdetermined** -- it is **structurally
underdetermined**: the four cited candidates are the
literature's strongest entries on the B'-relevant axes (geometric
depletion, intermittency mechanism, axisymmetric blow-up
numerics, rigorous Euler blow-up in adjacent class), and the
companion / textbook references either treat different settings
(passive scalar, averaged NS, weak-solution non-uniqueness,
energy conservation) or rely on classical inputs (BKM,
axisymmetric-without-swirl) that are already in the toolkit but
do not deliver a B' bound.

The verdict FAIL_NO_LITERATURE_PATH is therefore **robust** to
the anti-list scan, not an artifact of an incomplete bibliography.

---

## §8 Falsifiers active

Inherited from D3 seed §7, parent decision §8, and prior
discriminators (D3.A §9, D3.C §8); post-D3.B' status updated.

| tag | falsifier | post-B'-discriminator status |
|-----|-----------|------------------------------|
| F-D3-A | no uniform-in-K_max H^s on 2.5D system | DOES NOT FIRE (D3.A PASS preserved) |
| F-D3-B | no BKM-finite / no dim_P ≤ 1 blow-up on axisymmetric-with-swirl Euler | UNTESTED (B standalone deferred per parent §7.4 anti-list) |
| F-D3-C | two-sided Kraichnan S_6 ∼ ℓ² bound is K41 relabeling | FIRES (D3.C verdict preserved) |
| **F-D3-B'** | **no literature path closes vortex-stretching + multifractal Hölder into uniform bound or matching blow-up** | **FIRES** (this report §4) |
| F-D3-META-A | axes-not-independent | **FIRES** (D3.C §5.4: B↔C couples via intermittency; this report §5.1: B' merger inherits both obstructions, confirms coupling) |
| F-D3-META-B | compositional-not-implication | NOT APPLICABLE (compositional strategy dead at C; B' merger does not revive) |
| F-D3-META-C | decouple-itself-relabeling | FULLY ACTIVE on axis C (D3.C §8.3); WEAKLY ACTIVE on axis A (D3.A §9 partial-active) |
| F-D3-META-D | top-1 dispatch already fired | PARTIALLY ACTIVE (D3.A §8.3); not applicable to C, B' |
| F-DECIDE-A | F-D3-B' does not fire | **NOT TRIGGERED** (F-D3-B' fires per this report §4) |
| F-DECIDE-B | new D-tier candidate dominates B' before execution | NOT TRIGGERED (B' executed in this session) |
| F-DECIDE-C | D1.4 PASS supersedes D3 framework | NOT APPLICABLE (D1.4 not dispatched) |
| F-DECIDE-D | F-D3-META-A shown not to fire | NOT TRIGGERED (F-D3-META-A confirmed firing) |

Per-target falsifiers from `omega-cycle-bt544-ns-2026-04-25.md`
§8 (F1-F6, F_P1, F_P2, F_P3, F_Q4, F_Q5) are **not affected** by
this discriminator (B' is upstream-orthogonal in the same way
A and C were).

The MIXED decision in `omega-decide-bt544-d3-strategy-postC-2026-
04-25.md` is **confirmed** by this discriminator: F-D3-B' fires
(predicted), F-DECIDE-A does not trigger, the D3 program
officially closes.

---

## §9 Closing

- **D3.B' discriminator outcome**: **FAIL_NO_LITERATURE_PATH**
  (primary) + **FAIL_BLOWUP_PRECEDENT** (secondary co-active).
  No literature-import path closes the merged axis B'.
- **F-D3-B' status**: **fires**. The four cited candidates
  (Constantin-Fefferman 1993, Bernard-Gawedzki-Kupiainen 1998,
  Hou-Luo 2014, Elgindi 2021) jointly satisfy the F-D3-B' firing
  conditions (a) partial bound that does not close under
  intermittency, (b) blow-up precedent contradicting uniform
  regularity, (c) different geometric setting that does not
  generalize.
- **D3 program closure status**: **officially closed**. The
  compositional 2-gate strategy is dead; F-D3-META-A
  (axes-not-independent) fired in two specific forms (B↔C
  coupling via intermittency; B' merger inheriting both
  component obstructions). MIXED decision confirmed.
- **A PASS_LITERATURE preserved**, **C FAIL preserved**,
  **F-D3-META-A activation preserved**.
- **Next omega-cycle priorities** (per parent decision §6.3):
  D1.4 She-Leveque dispatch; D1.3 NS↔MHD duality dispatch; D-tier
  extension research seed.
- **Atlas / state / inventory**: untouched.
- **Millennium tally**: **0/7 unchanged**. BT-544 = 0/1
  untouched.
- **No new theorem claimed.** All cited results are pre-existing
  (Constantin-Fefferman 1993; Constantin 1994; Bernard-Gawedzki-
  Kupiainen 1998; Falkovich-Gawedzki-Vergassola 2001; Hou-Luo
  2014; Elgindi 2021; plus anti-list references CKN 1982, Tao
  2013/2016, Robinson-Rodrigo-Sadowski 2016, BKM 1984,
  axisymmetric-without-swirl chain Ladyzhenskaya 1968 / Uchovskii-
  Yudovich 1968 / Chen-Strain-Tsai-Yau 2008-2009, Buckmaster-
  Vicol 2019, Buckmaster-Masmoudi-Novack-Vicol 2023, Onsager
  1949, Constantin-E-Titi 1994, Isett 2018).

— end discriminator —
