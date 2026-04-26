---
id: omega-exec-bt544-d3-C-axis-discriminator
date: 2026-04-25
scope: research-only molt-validation (NO NS claim, NO atlas promotion)
target: BT-544 D3.C axis -- Onsager/Kraichnan two-sided S_6∼ℓ² bound
parent_reports:
  - reports/sessions/omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md (§ C axis)
  - reports/sessions/omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md (compositional 1st gate ✓)
millennium_resolved: 0/7 (unchanged)
grade: discriminator evaluation, no claim
---

# Omega Exec — BT-544 D3.C Axis Discriminator (2026-04-25)

## §0 Non-claim disclaimer

This report **evaluates** the D3.C discriminator of `omega-seed-
bt544-d3-mechanism-decouple-2026-04-25.md` (§2.3 + §4.1 axis-C
ranking). It does **NOT**:

- claim 3D Navier-Stokes regularity (smoothness or blow-up);
- promote any atlas entry, modify `state/proposals/inventory.json`,
  modify `theory/canon/`, or alter the `BT-544 = 0/1 untouched`
  Clay status;
- replace the L9 catalogue exhaustion verdict in
  `omega-exec-bt544-fallback-molt-validation-2026-04-25.md` (the
  rank-1/2/3 candidates remain FAILed);
- supersede the D3.A PASS_LITERATURE verdict in
  `omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md` (axis A
  remains clean);
- claim a new theorem on the Kraichnan model. Where I cite a result,
  I cite the *attestable classical reference* known to the
  Onsager / Kraichnan / convex-integration literature, not a new
  proof.

D3.C is a **discriminator**, not a Clay attempt. PASS means "axis C
in isolation supports a non-relabeling two-sided S_6 ∼ ℓ² bound";
FAIL_INTERMITTENCY means "the bound is empirically/theoretically
wrong because intermittency forces ζ_6 < 2"; FAIL_RELABELING means
"the bound is K41 dimensional analysis with n=6 labels applied
post-hoc". Either way, **0/7 unchanged**.

---

## §1 C axis spec extracted (the Onsager / Kraichnan two-sided bound)

Per `omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md` §2.3 +
§4.1, the C-axis discriminator asks for a sharp two-sided
structure-function bound at p = n = 6 in the Kraichnan-passive-
scalar model with a chosen 2-point velocity statistic.

**Structure functions (definitions, classical)**:
For an isotropic homogeneous turbulent (or model-turbulent) field
u, the longitudinal p-th order structure function is

  S_p(ℓ) := ⟨ |δu_∥(ℓ)|^p ⟩,    δu_∥(ℓ) := [u(x+ℓê) − u(x)] · ê,   (1)

with ê an arbitrary unit direction and ⟨·⟩ the spatial / ensemble
average. The inertial-range scaling is

  S_p(ℓ) ∼ ℓ^{ζ_p}  for η ≪ ℓ ≪ L,                                 (2)

with η the dissipative scale (Kolmogorov scale) and L the integral
scale.

**K41 prediction (Kolmogorov 1941)**: ζ_p^{K41} = p/3, hence
ζ_6^{K41} = 2 and S_6 ∼ ℓ^2. This is the prediction the n=6 lattice
encodes via α_c = φ/n = 1/3 and S_6 ∼ ℓ^{6·(1/3)} = ℓ^2.

**Onsager 1949 critical exponent**: α_c = 1/3. Above α = 1/3
(Hölder C^{1/3+}), Euler conserves energy; below α = 1/3 (Hölder
C^{1/3−}), anomalous dissipation can occur. This is the Onsager
conjecture.

**The discriminator (verbatim, D3 seed §2.3)**: produce a sharp
two-sided bound

  c_− · ℓ^{p/3} ≤ S_p(ℓ) ≤ c_+ · ℓ^{p/3}                          (★)

at p = 6, with explicit c_± and explicit ℓ-range, *derived from
the Kraichnan-model equations alone* (i.e. not by post-hoc
relabeling of K41 dimensional analysis as an n=6-lattice
prediction). The bound (★) at p=6 reads:

  c_− · ℓ^2 ≤ S_6(ℓ) ≤ c_+ · ℓ^2.                                (★_6)

**Key requirement**: BOTH bounds must hold. The upper bound
S_6 ≤ c_+ ℓ^2 is compatible with K41; the lower bound S_6 ≥ c_− ℓ^2
**forbids strict intermittency excess**, i.e. forbids ζ_6 < 2.

---

## §2 Literature scan (Onsager / Kraichnan / She-Leveque /
       Constantin-E-Titi / Isett / Buckmaster-Vicol)

### §2.1 Onsager 1949 critical exponent

- **Onsager 1949** ("Statistical hydrodynamics", Nuovo Cimento
  Supp. 6, 279). Conjectured α_c = 1/3: above this Hölder
  threshold, weak Euler solutions conserve energy; below, anomalous
  dissipation can occur. *Conjecture, not proof.*

### §2.2 K41 / K62 (the linear and lognormal predictions)

- **Kolmogorov 1941** ("Local structure of turbulence in
  incompressible viscous fluid for very large Reynolds numbers",
  Dokl. Akad. Nauk SSSR 30). Linear scaling ζ_p = p/3. **Statistical
  / dimensional argument**, not a rigorous theorem from NS.
- **Kolmogorov 1962** (J. Fluid Mech. 13, refined similarity
  hypothesis). Log-normal correction; predicted nonlinear
  ζ_p^{K62} = p/3 − μ p(p−3)/18 with μ ≈ 0.25 the intermittency
  parameter. Gives ζ_6^{K62} ≈ 1.75. **Phenomenological**.

### §2.3 She-Leveque 1994 intermittency model

- **She-Leveque 1994** ("Universal scaling laws in fully developed
  turbulence", Phys. Rev. Lett. 72, 336). Hierarchical model
  prediction
  ζ_p^{SL} = p/9 + 2 [1 − (2/3)^{p/3}].
  Plugging p = 6: ζ_6^{SL} = 6/9 + 2[1 − (2/3)^2] = 2/3 + 2·(5/9) =
  2/3 + 10/9 = 16/9 ≈ **1.778**. Empirical fit to wind-tunnel,
  jet, and pipe-flow data is **excellent across p = 1, …, 18**;
  the SL model is the standard intermittency benchmark.
- **Anselmet-Gagne-Hopfinger-Antonia 1984** (J. Fluid Mech. 140):
  experimental measurements of ζ_p for p ≤ 18 in turbulent jets,
  showing systematic departure from K41 at p ≥ 4. Reported
  ζ_6 ≈ 1.7–1.8, well below 2.
- **Frisch 1995** (*Turbulence: The Legacy of A. N. Kolmogorov*,
  Cambridge University Press), §8 and §9 review of multifractal
  models and experimental ζ_p data. ζ_6 < 2 is the consensus.

### §2.4 Constantin-E-Titi 1994 (Onsager rigorous, positive direction)

- **Constantin-E-Titi 1994** ("Onsager's conjecture on the energy
  conservation for solutions of Euler's equation", Comm. Math.
  Phys. 165, 207). **Theorem**: weak solutions of 3D Euler in
  L^3([0, T]; B^{α}_{3, ∞}(T^3)) with α > 1/3 conserve energy.
  This is the *positive* (regularity-implies-conservation)
  direction of the Onsager conjecture.

### §2.5 Isett 2018 (Onsager rigorous, negative direction)

- **Isett 2018** ("A proof of Onsager's conjecture", Annals of
  Mathematics 188, 871). **Theorem**: for any α < 1/3, there
  exist weak Euler solutions in C^α(T^3 × [0, T]) that do **not**
  conserve energy. This is the *negative* (exact-α_c = 1/3)
  direction. Combined with Constantin-E-Titi 1994, the Onsager
  conjecture is fully resolved at α = 1/3 for Euler.

### §2.6 Buckmaster-Vicol 2019 / 2023 (NS non-uniqueness)

- **Buckmaster-Vicol 2019** ("Nonuniqueness of weak solutions to
  the Navier-Stokes equation", Annals of Mathematics 189, 101).
  Convex-integration construction of non-unique weak solutions to
  3D NS in low-regularity classes. Demonstrates that the Onsager-
  type pathology survives the addition of viscosity at low
  regularity.
- **Buckmaster-Masmoudi-Novack-Vicol 2023** ("Intermittent
  convex-integration for the 3D Euler equations", Ann. PDE).
  Constructs intermittent (anomalous-scaling) Euler weak
  solutions with controlled regularity.

### §2.7 Kraichnan model (passive scalar with white-in-time velocity)

- **Kraichnan 1968** ("Small-scale structure of a scalar field
  convected by turbulence", Phys. Fluids 11, 945). Introduced the
  delta-correlated-in-time Gaussian velocity ensemble; gave a
  closed equation for the scalar 2-point function.
- **Kraichnan 1994** (Phys. Rev. Lett. 72, 1016). Conjecture that
  passive scalar in the Kraichnan velocity displays anomalous
  scaling (ζ_p^{scalar} ≠ p/3 even for the linear advection).
- **Gawedzki-Kupiainen 1995** ("Anomalous scaling of the passive
  scalar", Phys. Rev. Lett. 75, 3834). **Theorem**: in the
  Kraichnan model, the scalar 4-point function has ζ_4^{scalar} <
  2·ζ_2^{scalar} — *strict* intermittency at finite renormalization,
  rigorously derived from the closed model equations.
- **Bernard-Gawedzki-Kupiainen 1998** ("Slow modes in passive
  advection", J. Stat. Phys. 90, 519). Zero-mode mechanism
  identified as the source of anomalous scaling exponents in the
  Kraichnan model.
- **Falkovich-Gawedzki-Vergassola 2001** ("Particles and fields
  in fluid turbulence", Rev. Mod. Phys. 73, 913). Comprehensive
  review: rigorous demonstration that the Kraichnan model exhibits
  non-Kolmogorov scaling **at all even orders p ≥ 4**, with
  explicit perturbative computation of ζ_p − p/3 ≠ 0.

### §2.8 Synthesis of the literature scan

| reference | result | bearing on (★_6) |
|-----------|--------|------------------|
| K41 1941 | ζ_6 = 2 (dimensional) | upper bound only, no proof |
| K62 1962 | ζ_6 ≈ 1.75 | predicts ζ_6 < 2, contradicts (★_6) |
| She-Leveque 1994 | ζ_6 ≈ 1.778 | predicts ζ_6 < 2, contradicts (★_6) |
| Anselmet et al 1984 | ζ_6 ∈ [1.7, 1.8] experimentally | empirically refutes (★_6) lower bound |
| Constantin-E-Titi 1994 | α > 1/3 ⇒ conservation | tangential (regularity, not S_p) |
| Isett 2018 | α < 1/3 weak Euler exists with dissipation | tangential (existence, not S_p bound) |
| Buckmaster-Vicol 2019 | NS non-unique at low regularity | tangential (uniqueness, not S_p) |
| Kraichnan 1968 / 1994 | introduces model | sets framework |
| Gawedzki-Kupiainen 1995 | ζ_4 ≠ 2 ζ_2 in Kraichnan model (rigorous) | refutes K41 in Kraichnan |
| Bernard-Gawedzki-Kupiainen 1998 | zero-mode mechanism for anomalous ζ_p | mechanism for ζ_p ≠ p/3 |
| Falkovich-Gawedzki-Vergassola 2001 | ζ_p ≠ p/3 at p ≥ 4 in Kraichnan | rigorous-perturbative refutation of (★_6) |

The literature uniformly says: **ζ_6 < 2 in the Kraichnan model
(rigorously, perturbatively); ζ_6 < 2 in real turbulence
(empirically, via She-Leveque / Anselmet et al)**.

---

## §3 Discriminator path chosen

**Path Q (FAIL_INTERMITTENCY)**.

Justification: the discriminator (★_6) requires the **lower bound**
S_6(ℓ) ≥ c_− ℓ^2. By §2.3 (She-Leveque 1994; Anselmet et al 1984)
and §2.7 (Gawedzki-Kupiainen 1995; Falkovich-Gawedzki-Vergassola
2001), the actual scaling in the inertial range is
ζ_6 ≈ 1.778 < 2 — strictly less than 2. Therefore for ℓ → 0 within
the inertial range,

  S_6(ℓ) ∼ ℓ^{1.778}  =  ℓ^2 · ℓ^{−0.222}  ≫  c_− · ℓ^2,        (3)

so the lower bound c_− ℓ^2 is **trivially satisfied** (any positive
constant works, since ℓ^{1.778} dominates ℓ^2 as ℓ → 0). But the
**upper bound** is the problem: for ℓ → 0,

  S_6(ℓ) ∼ ℓ^{1.778}  ≫  c_+ · ℓ^2,                              (4)

so no finite c_+ can satisfy S_6 ≤ c_+ ℓ^2 uniformly in the
inertial range as ℓ → 0. The two-sided bound (★_6) **fails on the
upper-bound side**, due to intermittency.

(Alternative reading: if (★_6) is interpreted as ζ_6 = 2 *exactly*
— i.e. the symbol ∼ in S_6 ∼ ℓ^2 means "exactly the same exponent"
rather than asymptotic equivalence — then it is refuted on **both**
sides simultaneously: ζ_6 ≠ 2 means neither "S_6 ≥ c_− ℓ^2" nor
"S_6 ≤ c_+ ℓ^2" can hold uniformly with non-trivial finite c_±
across the inertial range. Either reading produces FAIL_INTERMITTENCY.)

**This places the discriminator outcome firmly in Path Q
(FAIL_INTERMITTENCY)**, not Path P (PASS) or Path R
(OBSTRUCTION-but-true). Path S (relabeling-FAIL) is also active
secondarily — see §4.

---

## §4 Verdict

**FAIL_INTERMITTENCY** (primary), with **FAIL_RELABELING**
(secondary) co-active.

### §4.1 Primary verdict: FAIL_INTERMITTENCY

The two-sided S_6 ∼ ℓ^2 bound (★_6) is **empirically and
theoretically wrong**. Specifically:

- **Empirically**: Anselmet-Gagne-Hopfinger-Antonia 1984 measured
  ζ_6 ∈ [1.7, 1.8] in jet, pipe, and wind-tunnel flows; the
  consensus value is ζ_6 ≈ 1.778 (She-Leveque 1994 hierarchical
  model). This is **strictly less than 2**, so the upper bound
  S_6(ℓ) ≤ c_+ ℓ^2 cannot hold for finite c_+ as ℓ → 0 in the
  inertial range.
- **Theoretically (Kraichnan model)**: Gawedzki-Kupiainen 1995 and
  Bernard-Gawedzki-Kupiainen 1998 rigorously derived
  ζ_p^{Kraichnan} ≠ p/3 for p ≥ 4 in the Kraichnan model. The
  zero-mode mechanism produces anomalous scaling intrinsically
  from the model equations. Hence even in the *simplest* setting
  the D3 seed proposed (Kraichnan-passive-scalar with B and A
  switched off), the K41 prediction ζ_6 = 2 fails.

The bound (★_6) is the wrong target. The Kraichnan model — chosen
in the D3 seed §2.3 precisely as the vehicle for axis-C
isolation — is the **canonical** model in which intermittency was
*first proven rigorously*. Asking for a two-sided ℓ^2 bound is
asking the model to violate its own established theorems.

### §4.2 Secondary verdict: FAIL_RELABELING

Even ignoring the intermittency obstruction, the upper bound
S_6(ℓ) ≤ c_+ ℓ^2 — if interpreted as the *K41 dimensional-analysis
prediction* — is exactly the relabeling failure mode F-D3-C
flagged in the D3 seed §2.3:

> "no two-sided Kraichnan-model bound at S_6 ∼ ℓ² is producible
> from the model equations alone (i.e. every candidate bound is a
> known classical result with the n=6 label applied post-hoc,
> exactly the §3.2 Q5-style relabeling failure)"

The K41 prediction ζ_6 = 2 is *not derived from the Kraichnan
model equations* — it is the dimensional-analysis prediction
ζ_p = p/3 evaluated at p = n = 6. The n=6 label adds nothing: the
exponent 2 is "p/3 at p=6", which is "6/3 = 2", a tautology of the
K41 ansatz, and the Kraichnan model has been shown (Gawedzki-
Kupiainen 1995) to *violate* this ansatz. So even if (★_6) were
not refuted by intermittency, it would be refuted by relabeling:
the bound is not Kraichnan-derived, it is K41-relabeled.

### §4.3 Composite verdict

**FAIL_INTERMITTENCY** is the *primary* failure mode (the bound is
physically wrong); **FAIL_RELABELING** is the *secondary* failure
mode (even if the bound were physically right, its derivation would
be relabeling). The two failure modes are independent and
mutually-reinforcing.

The discriminator C **fires F-D3-C** with extra force: not only is
the bound a relabeling, it is a relabeling **of an empirically
false ansatz**.

---

## §5 Compositional 2-gate status

### §5.1 Per-axis status (post-A, post-C)

| axis | discriminator | post-A status | post-C status | next role |
|------|---------------|---------------|---------------|-----------|
| A    | uniform H^s on 2.5D non-local-pressure | **PASS_LITERATURE** | (unchanged) PASS | clean, retired |
| B    | BKM-finite or dim_P ≤ 1 on axisymmetric-with-swirl Euler | UNTOUCHED | UNTOUCHED | predicted obstruction-carrier |
| C    | Kraichnan two-sided S_6 ∼ ℓ² bound | UNTOUCHED | **FAIL_INTERMITTENCY + FAIL_RELABELING** | refuted, exhausted |

### §5.2 Compositional strategy: status

The compositional strategy (D3 seed §3.2) requires **two clean
axes** for the obstruction to localize on the third. Status:

- A: clean (PASS_LITERATURE).
- C: **NOT clean** (FAIL_INTERMITTENCY + FAIL_RELABELING).
- B: untested, predicted hardest.

**The compositional strategy fails its 2nd gate.** Only one of the
two required clean-axis gates passed (A). C does not pass: it is
empirically and theoretically refuted, and even if it were not,
its derivation would be relabeling.

### §5.3 What does C's failure mean for the strategy?

The D3 seed §4.1 anticipated that C would *likely* fire F-D3-C
(relabeling exhaustion). The seed estimated:

> "Expected difficulty: low for the negative direction (the bound
> at S_6 ∼ ℓ² is *known to fail* in the Kraichnan model — the
> intermittency corrections are present), and moderate-to-hard for
> the positive direction"

The realized outcome **confirms the seed's prediction in the
strongest possible form**:

1. The negative direction fired **with rigorous theorem support**
   (Gawedzki-Kupiainen 1995; Falkovich-Gawedzki-Vergassola 2001).
2. The empirical She-Leveque 1994 / Anselmet et al 1984 data
   independently confirms the failure outside the Kraichnan model.
3. Both FAIL modes (intermittency and relabeling) are active, not
   just one.

**The compositional 2-gate strategy is dead.** Two clean gates
are not available within the {A, B, C} triple under the D3 seed's
discriminator definitions.

### §5.4 Residual information from C's failure

The failure is **informative**, not vacuous:

- The bound (★_6) is wrong because **intermittency** (ζ_p ≠ p/3
  at p ≥ 4) is a real physical phenomenon, not a labeling
  artifact. This means the n=6 lattice's prediction
  S_p ∼ ℓ^{p/3} (= ℓ^{p·φ/n}) is **empirically falsified at
  p ≥ 4**, not just at p = 6.
- More structurally: intermittency couples the energy cascade
  (axis C) to vortex stretching (axis B) via the multifractal /
  zero-mode mechanism. Bernard-Gawedzki-Kupiainen 1998 zero-mode
  analysis shows that the anomalous scaling is *driven by the
  geometry of the advecting velocity field* — i.e. by structure
  in the field that, in real NS, would correspond to vortex-
  stretching geometry.
- **Hence: axes B and C are NOT independent**. The decoupled-
  axis hypothesis (D3 seed §1.2: "the three axes are coupled
  non-trivially in the full NS equation … but switching off the
  others lets each be probed alone") **breaks down on C**: even
  in the simplest decoupled model (Kraichnan passive scalar), the
  anomalous-scaling phenomenon is entangled with field-geometry
  features that, in NS, are vortex-stretching features.

This is **F-D3-META-A active** (axes-not-independent), in a
specific form: B and C are structurally coupled via intermittency
/ multifractality, not just via the explicit feedback-triangle of
D3 seed §3.1.

---

## §6 Implications for the obstruction-carrier hypothesis

### §6.1 Original hypothesis (D3 seed §4.2)

> "Axis B (vortex-stretching) carries the proof-block. … Under the
> §3.2 strategy, A and C should be dispatched first. If A is clean
> and C exhausts by relabeling, the obstruction is **localized**
> to B."

### §6.2 Realized situation after C's discriminator

- A: clean as predicted. ✓
- C: **does not "exhaust by relabeling cleanly"**; it fails on a
  *deeper* level. It fails because intermittency is real, AND it
  fails because the ansatz is K41-relabeled. The double failure
  reveals that **C is not separable from B**: intermittency
  couples C ↔ B in a way that the D3 seed's decouple did not
  account for.

### §6.3 Two interpretive paths

**Path 1: B remains the sole obstruction-carrier (charitable
reading)**. Under this reading, C's failure simply means C is
"empty" as a separate axis — its content is *all* in B (vortex-
stretching), and the K41 prediction at C was just a numerological
proxy for vortex-stretching geometry. This collapses the triple
{A, B, C} to a doublet {A, B} with B carrying the obstruction.
Compositional strategy reformulated: "axis A is clean; the entire
NS obstruction lives on axis B (which absorbs the cascade content
of axis C via intermittency)". This is consistent with the
textbook view that vortex-stretching is the 3D obstruction.

**Path 2: the decouple itself breaks (strict reading)**. Under
this reading, C's failure invalidates the *clean-decouple
hypothesis* of the D3 seed: the three axes do not separate cleanly
even in the simplifications of §2.1-§2.3 of the seed. F-D3-META-A
fires, the decouple program is retired, and BT-544 returns to the
L9-catalogue-exhausted state (no per-axis decoupling available).

### §6.4 Recommendation

**Path 1 is the more useful reading**, but it is *not implied* by
the literature alone — it requires an additional argument that C's
content is fully absorbed into B, which itself is not in the repo.
**Path 2 is the more honest reading**: the decouple breaks, and
the compositional strategy is dead.

The honest verdict: the D3 compositional strategy — as defined in
the D3 seed — does not survive the C discriminator. **Axis B
remains predicted-hardest, but the path to localize the
obstruction onto B via "two clean axes" is no longer available.**

If a future report wants to revive the strategy, it would need
to either (a) redefine axis C with a different (non-K41) ansatz,
e.g. asking for the She-Leveque or multifractal exponent
ζ_6 ≈ 1.778 to be derivable from the Kraichnan model — but this
is a *different discriminator*, not the (★_6) bound the D3 seed
actually proposed; or (b) accept Path 1 (C absorbed into B) and
reformulate the compositional strategy as a doublet {A, B}.

Neither is performed in this report. The discriminator as defined
in the D3 seed has been evaluated and produces FAIL.

---

## §7 Anomalies

1. **Seed prediction strength vs realized strength**. The D3 seed
   §4.1 estimated C's discriminator at "low difficulty for the
   negative direction" and predicted F-D3-C would likely fire.
   The realized outcome is **stronger than predicted**: F-D3-C
   fires *with two independent confirmations* (intermittency
   theorem in Kraichnan + empirical She-Leveque), and the failure
   reveals an unanticipated structural coupling B ↔ C.

2. **F-D3-META-A activation**. The D3 seed §7 listed F-D3-META-A
   ("axes-not-independent") as "currently not active" because the
   2.5D ansatz (axis A) and axisymmetric-without-swirl (axis B)
   are standard separable simplifications. The seed did not
   anticipate that F-D3-META-A could fire **on the C axis** via
   intermittency-induced B ↔ C coupling. **F-D3-META-A is now
   active** (in this specific form), not retired.

3. **F-D3-META-C reactivation**. The D3 seed §7 flagged
   F-D3-META-C ("decouple-itself-relabeling") as "partially
   active" because per-axis estimates borrow from classical PDE
   machinery. The C discriminator's FAIL_RELABELING verdict
   strengthens this: the (★_6) bound is *literally* a K41
   relabeling, so the C-axis content of the decouple **is**
   relabeling, exactly the F-D3-META-C failure mode. F-D3-META-C
   on axis C is **fully active**, not partially.

4. **Inversion in dispatch logic**. The D3 seed §5 wrote: "Why not
   C first: C's discriminator is more likely to fire F-D3-C
   (relabeling exhaustion) than to produce new content. Firing
   F-D3-C is informative but non-progressive in the compositional
   sense; it merely confirms the Q5 verdict on a third axis."
   Realized: firing F-D3-C is informative *and* mildly
   progressive — it reveals B ↔ C coupling, which is a structural
   observation about the decouple that was not in the seed. So
   the dispatch order (A, then C, then B) was justified after all,
   even though the seed argued for "A first because progressive".

---

## §8 Falsifiers active

Inherited from D3 seed §2.3 (F-D3-C) and §7 (meta-falsifiers),
post-discriminator status updated:

| tag | falsifier | post-C-discriminator status |
|-----|-----------|------------------------------|
| F-D3-A | no uniform-in-K_max H^s on 2.5D system | DOES NOT FIRE (D3.A PASS) |
| F-D3-B | no BKM-finite / no dim_P ≤ 1 blow-up on axisymmetric-with-swirl Euler | UNTESTED (B deferred) |
| **F-D3-C** | **two-sided Kraichnan S_6 ∼ ℓ² bound is K41 relabeling** | **FIRES** — both intermittency-fail and relabeling-fail |
| F-D3-META-A | axes-not-independent | **FIRES on C axis** — intermittency couples B ↔ C |
| F-D3-META-B | compositional-not-implication | NOT YET TESTED (compositional strategy died at C) |
| F-D3-META-C | decouple-itself-relabeling | **FULLY ACTIVE on C axis** — (★_6) is K41 relabeling |
| F-D3-META-D | top-1 dispatch already fired | DOES NOT APPLY to C (C is 2nd dispatch, not top-1) |

Per-target falsifiers from `omega-cycle-bt544-ns-2026-04-25.md`
§8 (F1–F6, F_P1, F_P2, F_P3, F_Q4, F_Q5) are **not affected** by
this C discriminator (axis C is independent of those falsifiers
by construction of the decouple, except via F_Q5 which is
strengthened: Q5-style relabeling fires here on axis C).

---

## §9 Closing

- **D3.C discriminator outcome**: FAIL_INTERMITTENCY (primary) +
  FAIL_RELABELING (secondary) (Path Q + Path S co-active).
- **Bound (★_6)**: refuted by She-Leveque 1994 / Anselmet et al
  1984 / Gawedzki-Kupiainen 1995 / Bernard-Gawedzki-Kupiainen
  1998 / Falkovich-Gawedzki-Vergassola 2001. ζ_6 ≈ 1.778 < 2.
- **Compositional 2-gate status**: A ✓ (PASS_LITERATURE), C ✗
  (FAIL). Strategy fails its 2nd required clean gate. The
  decouple as defined in D3 seed does not yield a "two clean axes
  → localize to third" reduction.
- **Axis-B implication**: B remains the predicted obstruction-
  carrier (D3 seed §4.2 standing prediction), but the path to
  *reduce* BT-544 to "axis B alone" via the compositional
  strategy is no longer available because C is not cleanly
  separable from B (intermittency couples them via the multifractal /
  zero-mode mechanism). Either (a) reformulate as the doublet
  {A, B} with B absorbing C's content, or (b) accept that the
  decouple breaks and BT-544 returns to the L9-catalogue-
  exhausted state.
- **Atlas / state / inventory**: untouched.
- **Millennium tally**: **0/7 unchanged**. BT-544 = 0/1 untouched.
- **No new theorem claimed.** All cited results are pre-existing
  (Onsager 1949; Kolmogorov 1941/1962; Anselmet-Gagne-Hopfinger-
  Antonia 1984; Kraichnan 1968/1994; She-Leveque 1994;
  Constantin-E-Titi 1994; Gawedzki-Kupiainen 1995; Bernard-
  Gawedzki-Kupiainen 1998; Frisch 1995 monograph; Falkovich-
  Gawedzki-Vergassola 2001; Isett 2018; Buckmaster-Vicol 2019;
  Buckmaster-Masmoudi-Novack-Vicol 2023).

— end discriminator —
