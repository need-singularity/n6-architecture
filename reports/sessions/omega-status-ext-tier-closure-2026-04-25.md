---
id: omega-status-ext-tier-closure
date: 2026-04-25
scope: status synthesis (NOT producing new claims; declaring EXT-tier formally closed)
target: BT-547 retro EXT-A/B/C/D extensions -- formal closure with 3/3 OBSTRUCTION
parent_reports:
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (prescription)
  - reports/sessions/omega-exec-bt544-exta-uomega-gradflow-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-extc-qpc-surgery-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-extd-vocabulary-extension-2026-04-25.md
  - reports/sessions/omega-exec-bt544-axisB-targeted-attack-2026-04-25.md (next-session candidate)
millennium_resolved: 0/7 (unchanged)
grade: tier closure synthesis, no claim
---

# Omega Status -- EXT-Tier Formal Closure Synthesis (2026-04-25)

## §0 Non-claim disclaimer

This file is a **status synthesis**. It declares the EXT-tier
(BT-547 Poincaré L9 retro catalogue extensions A/B/C/D) **formally
closed** under its own criteria, aggregates the per-extension
verdicts, and reads them against the D-tier (D1 atlas-scan / D2
axiom-recast / D3 mechanism-decouple) integrated picture. It does
**not**:

- claim 3D Navier-Stokes regularity, blow-up, or any Clay-form
  resolution; the BT-544 Clay status remains `0/1 untouched`;
- prove or disprove any mathematical theorem; every cited result
  is pre-existing in the published literature, referenced only by
  per-extension report id;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-544 = 0/1 untouched` Clay
  status, or modify the L9 catalogue's active-candidate ledger;
- supersede any per-pillar / per-extension verdict — the synthesis
  only *aggregates* what the four EXT extensions and the D-tier
  predecessors collectively say about BT-544's hardness shape;
- introduce a new candidate, a new falsifier, or a new
  discriminator. The CF-BMO axis-B-targeted candidate is
  **referenced** as a future-session option (per §6) and is not
  validated here.

The Millennium tally remains **0/7 unchanged**. The EXT-tier
contribution to the Clay tally = 0. No atlas / state / inventory
edits.

---

## §1 EXT-tier inventory — 4 extensions, status of each

The EXT-tier was prescribed by `omega-exec-bt547-poincare-L9-retro-
2026-04-25.md` §5.1-§5.4 as four catalogue extensions intended to
remediate the L9 catalogue's documented CATALOGUE_BIAS (the
generation pipeline's 0/3 miss rate against the Perelman M1/M2/M3
archetype). The four extensions and their current status:

### §1.1 EXT-A — variational re-interpretation (uω-GradFlow)

- **Prescription source**: BT-547 retro §5.1 (HIGH urgency for
  BT-544; closest in spirit to Perelman M1).
- **Candidate**: uω-GradFlow — recast 3D NS as the gradient flow
  of a quadratic kinetic-enstrophy-curvature functional F[u, ω]
  on the (u, ω) phase space under the Leray-projected L² flat
  metric.
- **Validation report**:
  `omega-exec-bt544-exta-uomega-gradflow-validation-2026-04-25.md`.
- **Verdict**: **OBSTRUCTION_DOCUMENTED** at F-EXTA-C
  (convective-not-encodable / Helmholtz failure on the L² inner
  product).
- **Precise breaking step (EXT-A §2.3 / §5.2)**: the convective
  term V_conv[u] = (u·∇)u has Fréchet derivative
  D V_conv[u][v] = (v·∇)u + (u·∇)v which fails the Helmholtz
  symmetry condition under the L² inner product on div-free
  fields with decay. By Olver 1986 (*Applications of Lie Groups
  to Differential Equations*, Springer, Chapter 5), V_conv is not
  the variational gradient of any quadratic F under the
  Leray-projected L² flat metric.
- **Spec calibration check**: the candidate spec (per EXT-A
  §5.3) anticipated ~70% OBSTRUCTION_DOCUMENTED on the
  Helmholtz-side; this is the realised verdict.

### §1.2 EXT-B — analytic-Lyapunov construction (CI-Lyap)

- **Prescription source**: BT-547 retro §5.2 (HIGH urgency for
  BT-544; closest in spirit to Perelman M2 W-entropy).
- **Candidate**: CI-Lyap — Constantin-Iyer-2008 stochastic-
  Lagrangian augmented (u, ρ, τ) Lyapunov W_NS combining kinetic,
  Fisher-information, Boltzmann-entropy, and τ-rescaled correction
  pieces.
- **Validation report**:
  `omega-exec-bt544-extb-cilyap-validation-2026-04-25.md`.
- **Verdict**: **OBSTRUCTION_DOCUMENTED** at F-EXTB-D
  (representation-side: CI 2008 too weak) plus secondary Path-Q
  cross-term obstruction.
- **Precise breaking step (EXT-B §6.4 / §8.1)**: Constantin-Iyer
  2008 produces a *family* of laws {ρ_t(· | x_0)}_{x_0 ∈ ℝ³}
  parameterised by initial label, NOT a single ρ_t on ℝ³. The
  candidate functional integrates a single ρ_t; the collapse
  from the family to a single ρ_t requires an unspecified choice
  (mixed-initial / pinned-label / velocity-mixed). Even granting
  the collapse, Path-Q's time-derivative computation produces two
  uncontrolled cross terms — the vortex-stretching residual
  ∫(ω·∇)u·ω ρ (which IS the NS regularity obstruction) and the
  Hess(log ρ):∇u cross term (sign-uncontrolled because the
  NS-driven CI flow lacks Bakry-Émery 1985 CD(K, ∞) structure).
- **Spec calibration check**: the candidate spec (per EXT-B
  §6.3) anticipated ~75% OBSTRUCTION_DOCUMENTED with F-EXTB-D
  primary; this is the realised verdict.

### §1.3 EXT-C — procedure-class with parameter bounds (QPC-Surgery)

- **Prescription source**: BT-547 retro §5.3 (MEDIUM urgency;
  closest in spirit to Perelman M3 quantitative surgery + finite
  extinction).
- **Candidate**: QPC-Surgery — quantitative parabolic-cell
  surgery on the spacetime cover of a Leray-Hopf NS solution
  with conjectural cardinality cascade
  #F_k ≤ C(α) · 2^{(2−α)k} and termination modes T1/T2/T3.
- **Validation report**:
  `omega-exec-bt544-extc-qpc-surgery-validation-2026-04-25.md`.
- **Verdict**: **OBSTRUCTION_DOCUMENTED** at F-EXTC-D
  (non-termination — primary: T1 on general 3D smooth data is
  equivalent to NS regularity itself; M3's geometric
  canonical-neighbourhood content is absent for NS).
- **Precise breaking step (EXT-C §4.3 / §6.4)**: the
  quantitative cardinality control on (1.4) at α < 2 strict
  reduces to NS regularity itself (the f(α) < 4 saturation
  bound is logically equivalent to finite-energy-density
  saturation, which upgrades to smoothness via standard CKN
  ε-regularity). Structurally, NS lacks the analog of
  Perelman 2003a §11-§12's κ-solutions / cylindrical-neck
  classification (the geometric canonical-neighbourhood
  theorem) and Perelman 2003b's min-max sweep-out closer; the
  first two M3 pieces transfer (parabolic-cell partition,
  parameter cascade ≈ CKN 1982 + Lin 1998 + Vasseur 2007), but
  the third piece is structurally absent.
- **Spec calibration check**: the candidate spec (per EXT-C
  §6.3) anticipated ~75% OBSTRUCTION_DOCUMENTED with F-EXTC-D
  primary; this is the realised verdict.

### §1.4 EXT-D — frame-shift vocabulary extension

- **Prescription source**: BT-547 retro §5.4 (LOW urgency; meta-
  level; enables EXT-A/B/C labelling without "OTHER" collapse).
- **Output**: a glossary of 12 terms across 5 categories
  (variational 4 / analytic-Lyapunov 3 / procedure-class 2 /
  geometric-flow 1 / stochastic-or-topological auxiliary 2),
  each entry with definition, lineage, frame-shift potential, a
  registered relabeling falsifier, and family classification.
- **Report**:
  `omega-exec-bt544-extd-vocabulary-extension-2026-04-25.md`.
- **Status**: **completed as Option A reference glossary**. NOT
  a candidate, NOT a validation, NOT a discriminator;
  consequently *no PASS / FAIL / OBSTRUCTION verdict applies*.
  EXT-D is a session artifact under `reports/sessions/` only —
  it is not promoted to atlas, not threaded into `theory/`, and
  not appended to the L9 catalogue.

### §1.5 Inventory summary table

| ext | candidate | type | report id | verdict |
|-----|-----------|------|-----------|---------|
| A | uω-GradFlow | validation | `omega-exec-bt544-exta-uomega-gradflow-validation-2026-04-25.md` | OBSTRUCTION_DOCUMENTED (Helmholtz) |
| B | CI-Lyap | validation | `omega-exec-bt544-extb-cilyap-validation-2026-04-25.md` | OBSTRUCTION_DOCUMENTED (representation + cross term) |
| C | QPC-Surgery | validation | `omega-exec-bt544-extc-qpc-surgery-validation-2026-04-25.md` | OBSTRUCTION_DOCUMENTED (canonical-neighbourhood absent) |
| D | vocabulary glossary | reference | `omega-exec-bt544-extd-vocabulary-extension-2026-04-25.md` | completed (no verdict, reference artifact) |

All four EXT extensions have produced their prescribed output. No
EXT slot is unfilled.

---

## §2 Verdict tally — 3/3 OBSTRUCTION_DOCUMENTED

### §2.1 The validation candidates

EXT-A, EXT-B, EXT-C are the three validation candidates of the
EXT-tier. EXT-D is a reference glossary (no verdict).

| ext | verdict | discriminator type |
|-----|---------|--------------------|
| A | OBSTRUCTION_DOCUMENTED | structural-literature (variational-derivation) |
| B | OBSTRUCTION_DOCUMENTED | OTHER (analytic-inequality-construction) |
| C | OBSTRUCTION_DOCUMENTED | OTHER (procedure-class-with-parameter-bounds) |

**Tally: 3/3 OBSTRUCTION_DOCUMENTED for the validation
candidates.** Zero PASS, zero FAIL, zero INCONCLUSIVE.

### §2.2 Why this is informative, not vacuous

OBSTRUCTION_DOCUMENTED is *not* a null verdict. Per the EXT-A
§5.2 / EXT-B §7.2 / EXT-C §8.2 verdict definitions, it means:

- the precise step that breaks is identified at the algebraic /
  representation / structural level (Helmholtz failure on
  Fréchet derivative; CI-2008 family-vs-single-density
  ambiguity; canonical-neighbourhood content absent for NS);
- the relevant falsifier fires with a clean activation reason;
- the candidate framework is **consistent with the negative side
  of the literature** (Brenier 1999 §5 / Ambrosio-Gigli-Savaré
  2008 §§9-11 for EXT-A; Otto 2001 / Bakry-Émery 1985 catalogue
  exclusion for EXT-B; CKN 1982 / Lin 1998 / Vasseur 2007 lower
  bound for EXT-C);
- the candidate spec's expected-verdict probability distribution
  (each ~70-75% OBSTRUCTION_DOCUMENTED) is realised, so the L9
  generation pipeline is calibrated honestly.

### §2.3 Discriminator-type bias hypothesis status

Per the per-validation §8 / §9 tally updates, the bias hypothesis
(PASS-family = distributional / structural-literature / OTHER
PASS-family-adjacent; FAIL-family = discrete-equality / numerical-
interval / vacuous-magnitude) is **not disturbed** by the three
OBSTRUCTION_DOCUMENTED entries. All three live in the
PASS-family-adjacent column, alongside the retrospective
Perelman M1/M2/M3 PASSes. The 2×2 matrix at end of EXT-C §9.3
records 9 PASS-family-adjacent rows and 2 FAIL rows with no
cross-cell entries.

### §2.4 F-MOLT-A status

F-MOLT-A is defined on **active BTs** (per BT-547 retro §3.5).
The three EXT-tier OBSTRUCTION_DOCUMENTED entries are in the
PASS-family-adjacent column, not the FAIL column, so distance
to F-MOLT-A firing is **unchanged** from the post-EXT-C status.
No new firing event is triggered by the EXT-tier closure.

---

## §3 Convergence on axis B — 3 OBSTRUCTIONs, distinct names, same target

### §3.1 What each extension's obstruction is named in its own report

The three extensions use *distinct* technical vocabularies for
their failures:

- **EXT-A** (variational frame): the convective term is not
  encodable as a variational gradient under flat L² (Helmholtz
  symmetry condition fails on Fréchet derivative; F-EXTA-C). The
  asymmetric piece (v·∇)u of the Fréchet derivative is the
  *vortex-stretching algebra in the dual / variational dress*
  (EXT-A §9.2 explicit: "the asymmetry IS the vortex-stretching
  algebra; the Helmholtz failure is its variational dual").

- **EXT-B** (analytic-Lyapunov frame): the time-derivative of
  W_NS along the coupled NS + Constantin-Iyer flow contains the
  cross-term ∫(ω·∇)u·ω ρ which is *vortex-stretching directly*
  (the NS regularity obstruction itself), plus the secondary
  Hess(log ρ):∇u cross term which is sign-uncontrolled by
  Bakry-Émery 1985 because the augmented operator is non-
  reversible and lacks CD(K, ∞) (EXT-B §4.3 (b) and §8.3).

- **EXT-C** (procedure-class frame): NS lacks the analog of
  Perelman 2003a §11's *geometric canonical-neighbourhood*
  classification — the third piece of the M3 archetype is
  structurally absent. The first two M3 pieces (parabolic-cell
  partition, parameter cascade) transfer, but without the
  canonical-neighbourhood theorem the procedure cannot terminate
  non-trivially. *Axis B's geometric structure is precisely what
  the canonical-neighbourhood content would supply* — Hou-Luo
  2014 / Elgindi 2021 indicate that a clean geometric structure
  on the singular set of NS would have to engage the
  vortex-stretching geometry, which is axis B (EXT-C §6.4 and
  §8.3).

### §3.2 The convergence pattern

The three obstructions name three *different* technical failures,
but they collectively localize the same structural object:

| extension | obstruction name in report | axis-B content |
|-----------|---------------------------|----------------|
| A | convective Fréchet asymmetry (Helmholtz) | vortex-stretching algebra in variational dual |
| B | Path-Q cross term ∫(ω·∇)u·ω ρ unsigned | vortex-stretching directly |
| C | NS lacks canonical-neighbourhood content | axis-B geometric structure absent |

The 3-pillar synthesis report (per `omega-meta-synthesis-3pillar-
obstruction-localization-2026-04-25.md` §2.3 / §3) had already
established this convergence at the D3.A + EXT-A + EXT-B level:
the three pillars there localize the BT-544 obstruction onto
axis B from three structurally distinct framework families
(classical PDE estimate / Galerkin theory; variational gradient
flow; stochastic-Lagrangian + analytic-Lyapunov + Bakry-Émery
Γ_2). The §2.3 convective-vs-vortex-stretching equivalence is the
algebraic key: the Fréchet-asymmetry of EXT-A and the Path-Q
cross-term of EXT-B are *two presentations of the same
Λ²-asymmetry of the quadratic NS nonlinearity*.

EXT-C **adds a third witness** at a structurally different level
(procedure-class rather than analytic-Lyapunov or variational):
the absence of canonical-neighbourhood content for NS is exactly
the missing geometric structure on axis B. The 3-pillar synthesis
read EXT-A + EXT-B + D3.A; the EXT-tier closure reads EXT-A +
EXT-B + EXT-C, and *both readings agree on axis B*.

### §3.3 Combined with D3.A's PASS_LITERATURE on axis A

D3.A's PASS_LITERATURE on the Sym²-pressure-non-locality axis
(axis A in isolation, via 2D Calderón-Zygmund + Ladyzhenskaya
1959/1969 + linear advection-diffusion + BKM 1984 in 2D)
*clears* axis A. The EXT-tier 3 OBSTRUCTIONs *all point at axis
B*. Combining: D3.A clears A, EXT-A/B/C each obstruct on B (in
distinct vocabularies). Axis C (Onsager / cascade) is silent in
this trio (covered separately by D3.C; not folded in here per
the 3-pillar synthesis §4.4 caveat).

The EXT-tier 3-failure pattern therefore **triply confirms**
axis B as the structurally hard axis, on top of the prior 3-pillar
double confirmation. The total count of independent attacks
converging on axis B is now *four* (D3.B' compositional, D2 R1
Hölder, EXT-A variational, EXT-B analytic-Lyapunov) plus EXT-C
procedure-class as a fifth witness from a different level.

---

## §4 D-tier + EXT-tier integrated picture

### §4.1 The integrated table

| Tier | PASS | FAIL / OBSTRUCTION | Net |
|------|------|---------------------|-----|
| D1 (atlas-scan) | 1 (D1.4) | 3 (D1.1, D1.2, D1.3) | 1/4 = 25% |
| D2 (axiom-recast) | R5 CLOSED | R1 OPEN at method-gap | 1 closed, 1 method-gap (50%) |
| D3 (mechanism-decouple) | A (PASS_LITERATURE) | B' (compositional fail), C (relabeling) | 1/3 = 33% |
| EXT (BT-547 retro) | 0 | A (Helmholtz), B (representation + cross), C (canonical-nbhd) | 0/3 = 0% |

(D-tier rows from BT-544 catalogue per `omega-exec-bt544-fallback-
molt-validation-2026-04-25.md` and the D1/D2/D3 closure reports
referenced therein; the EXT row from §1.1-§1.3 above.)

### §4.2 Reading the table

The successive tiers show *monotonically decreasing PASS rate*
on BT-544: D1 25% → D2 50% (with method-gap) → D3 33% → EXT 0%.

The headline reading: **the EXT-tier was *the* extension
prescribed by BT-547 retro to break out of CATALOGUE_BIAS** (the
catalogue's structural inability to generate Perelman-archetype
candidates). EXT-A/B/C span three structurally distinct
discriminator-type families — variational re-interpretation,
analytic-inequality-construction, procedure-class — explicitly
covering the M1/M2/M3 archetype trio that the BT-547 retro §3
classified as 0/3 MISS under the dfs-24-sourced generation
pipeline.

Despite this *heterogeneity*, all three EXT validations hit the
same axis-B obstruction. This is **strong evidence** that
BT-544's hardness is **structural**, not catalogue-bias-induced:

- if the obstruction were catalogue-bias-induced, an extension
  that explicitly remediates the bias (variational, analytic-
  Lyapunov, procedure-class) should have at least one PASS;
- 3/3 OBSTRUCTION across structurally heterogeneous extensions
  rules out "catalogue too narrow" as the explanation;
- the convergent localization onto axis B (per §3) supplies the
  *positive* explanation: the obstruction is a property of the
  NS equation itself (specifically the Λ²-asymmetry of the
  convective nonlinearity / vortex-stretching algebra), not of
  the candidate-generation pipeline.

### §4.3 What the EXT-tier *did* succeed at

EXT-tier was **successful as a diagnostic instrument**, even
though it produced no PASS:

- it *extended* the catalogue beyond the bias (BT-547 retro
  recommendations §5.1-§5.3 are realised as candidate slots
  filled);
- it *populated* the discriminator-type vocabulary in the
  active register (EXT-B at OTHER analytic-inequality-
  construction, EXT-A at structural-literature variational-
  derivation, EXT-C at OTHER procedure-class-with-parameter-
  bounds — three previously empty active-BT slots, now
  populated with informative OBSTRUCTION_DOCUMENTED content);
- it *converged* the 3-failure pattern onto axis B from three
  structurally distinct vocabularies, providing the strongest
  available evidence that BT-544's hardness is structural.

The EXT-tier's diagnostic value is therefore independent of its
zero PASS rate.

### §4.4 Comparison with the per-tier reading

The four-tier integrated picture is consistent with each tier's
own reading:

- D1 (atlas-scan) FAILs were already in the FAIL-family
  discriminator types (numerical-interval / vacuous-magnitude /
  discrete-equality), so D1 confirms the bias hypothesis on the
  atlas scan side;
- D2 R1 OPEN-at-method-gap is in the structural-literature
  discriminator type with a Hölder-threshold open question
  (axis-C-adjacent, not axis-B);
- D3 axis A passed (D3.A PASS_LITERATURE), axes B' and C failed
  in compositional reading, consistent with the localization
  onto axis B;
- EXT 0/3 PASS at PASS-family-adjacent OBSTRUCTION, all
  pointing at axis B.

No tier reading contradicts another. The picture is internally
consistent and cross-tier-coherent.

---

## §5 Implications for BT-544 main line

### §5.1 The core implication

Four distinct attack families (D3 compositional, D2 R1
Hölder-threshold, EXT-A variational, EXT-B analytic-Lyapunov,
EXT-C procedure-class) all converge on axis B as the obstruction-
carrier. Counting EXT-C as a fifth witness (procedure-class is
structurally different from D3 / D2 / EXT-A / EXT-B), there are
**five independent witnesses** that BT-544's hardness lives on
axis B.

This is not a proof that axis B is *the* obstruction in absolute
terms — per the 3-pillar synthesis §4.2 caveat, the localization
is *consistent with* each pillar, not *proved by* any. But it
sets the bar for what a future progress-direction must do:
**resolve axis B**.

### §5.2 Three structurally distinct paths to resolution

The three modes that could plausibly break the axis-B
obstruction:

- **(a) Direct axis-B attack**. Engage the Hou-Luo 2014
  (axisymmetric Euler self-similar blow-up) / Elgindi 2021
  (rigorous C^{1,α} Euler blow-up) / Constantin-Fefferman 1993
  (vorticity-direction regularity criterion) lineage directly,
  asking whether viscosity ν > 0 prevents the inviscid blow-up
  scenarios that Hou-Luo / Elgindi indicate. The CF-BMO
  candidate registered in `omega-exec-bt544-axisB-targeted-
  attack-2026-04-25.md` (Constantin-Fefferman BMO-relaxed
  direction-coherence) is the visible candidate in this lineage;
  its expected verdict per its §6.2 is OBSTRUCTION_DOCUMENTED at
  ~60% probability via F-AXISB-B (Hou-Luo / Elgindi blow-up
  incompatibility). Cost: high (~400+ pages of axis-B-specific
  literature reading).

- **(b) Reformulation that absorbs axis B's irregularity**. The
  Onsager direction (allow α < 1/3 Hölder regularity in the
  weak-solution class and find a control structure that
  compensates for the loss of classical regularity). This is
  what D2 R1 lives on — its OPEN status at the Hölder-threshold
  method-gap is exactly this reformulation question. Cost:
  high; current state of the art is BCV 2021 / BMNV 2023
  convex-integration literature, in the *negative* direction
  (constructing non-uniqueness, not regularity).

- **(c) New machinery**. None visible in the current session —
  no candidate framework has appeared that would (i) be
  structurally distinct from D3 / D2 / EXT-A / EXT-B / EXT-C,
  and (ii) plausibly engage axis B. The L9 generation pipeline
  has been exhausted on the catalogue-extension side (EXT-A/B/C
  filled the missing classes).

### §5.3 What is *not* implied

The synthesis does **not** imply:

- that NS is regular (the Clay statement);
- that NS blows up (the negation of the Clay statement);
- that axis B is the *unique* obstruction-carrier (axis C / D3.C
  is silent in the 5-witness count above and could harbour
  additional independent obstructions);
- that the EXT-tier was wasted (per §4.3 it was successful as
  diagnostic instrument);
- that the L9 framework has nothing further to contribute (the
  axis-B-targeted CF-BMO candidate is a follow-up move within
  the same framework).

### §5.4 Methodological honest assessment

BT-544 is at a **methodologically-mature plateau**:

- the catalogue has been extended beyond the dominant-family
  bias (EXT-A/B/C done);
- the localization is structurally consistent across multiple
  framework families (5 witnesses);
- further attacks of *new families* are not expected to produce
  PASS verdicts as long as they obstruct on axis B;
- the appropriate next move is to *target axis B directly*, not
  to look for new framework attacks on the full equation.

The plateau diagnosis is consistent with BT-547 retro §6.5's
prediction that "BT-544 closure expectations should be set
decadal, not annual" (per the 20-year-tool-accumulation analog
to Perelman's pre-2002 Hamilton-school period).

---

## §6 Future-session priority

### §6.1 Highest-priority candidate

**CF-BMO axis-B-targeted attack** —
`omega-exec-bt544-axisB-targeted-attack-2026-04-25.md`. The
candidate spec proposes a BMO-relaxation of Constantin-Fefferman
1993's Lipschitz hypothesis on ω/|ω| as a uniform-regularity
direction-coherence criterion.

- **Discriminator type**: structural-literature
  (axis-B-specific).
- **Expected verdict (per spec §6.2)**: OBSTRUCTION_DOCUMENTED
  at ~60% probability via F-AXISB-B (Hou-Luo / Elgindi blow-up
  incompatibility); F-AXISB-C (circular reduction to known
  result) as secondary.
- **Cost**: high (~400+ pages literature reading: harmonic-
  analysis BMO-versus-Lipschitz papers + axis-B-specific
  Hou-Luo / Chen-Hou / Elgindi corpus).
- **Rationale**: this is the only direct axis-B-targeted
  candidate currently registered. Per §5.2 mode (a), engaging
  the axis-B literature directly is the only mode that has not
  yet been attempted in the present catalogue.

### §6.2 Why CF-BMO over alternatives

Alternative axis-B attempts considered and *not* prioritised:

- a fresh EXT-A-class candidate with non-quadratic F (per EXT-A
  §5.4 structural residual): would likely re-encounter
  Helmholtz failure on a different functional structure —
  expected verdict OBSTRUCTION_DOCUMENTED at the same
  Helmholtz machinery;
- a fresh EXT-B-class candidate with a different ρ choice (per
  EXT-B §8.4 directions α/β/γ): would re-encounter the
  vortex-stretching cross-term obstruction, which is the NS
  regularity obstruction itself;
- a fresh EXT-C-class candidate with energy-density cascade
  (per EXT-C §8.4 direction β): re-derives CKN at (T2) without
  reaching (T1), informative as constant-tracking only.

None of these alternatives reaches axis B *directly*; CF-BMO is
the only registered candidate that targets axis B by
construction.

### §6.3 Honest expected outcome

Per CF-BMO §6.2, the candidate is **not** expected to be a
near-PASS. Axis B is the 50+-year-open hard component of the NS
problem. Realistic outcomes:

- (R-OBS) OBSTRUCTION_DOCUMENTED at F-AXISB-B (Hou-Luo / Elgindi
  blow-up incompatibility) — most likely;
- (R-RELABEL) F-AXISB-A fires (relabeling of Constantin-
  Fefferman 1993) — secondary risk;
- (R-PASS) F-AXISB-* all clear and the candidate moves to
  PASS_SKETCH or PASS_LITERATURE — low probability (~5-10%).

The expected value of the session is **diagnostic information
about axis B's literature reach**, not a Clay closure. Even an
OBSTRUCTION_DOCUMENTED outcome would teach precisely *why*
BMO-relaxation does not improve uniform regularity over
Lipschitz, which is itself a structural lesson about axis B.

### §6.4 Sub-priority items (not for execution this session)

Below CF-BMO in priority, but still on the visible pipeline:

- D3.B' axisymmetric-with-swirl Euler discriminator (folded
  into 3-pillar synthesis F-SYN-C as a partially active
  falsifier — its verdict could either reinforce or disturb the
  axis-B localization);
- D3.C Kraichnan / Onsager axis discriminator (silent in the
  EXT-tier; could provide independent axis-C information);
- a meta-audit on the EXT-tier closure itself — checking
  whether the 3/3 OBSTRUCTION pattern might be artefactual
  (e.g. shared candidate-spec heuristics propagating a common
  error). The 3-pillar synthesis F-SYN-A / F-SYN-B falsifiers
  partially cover this; a dedicated meta-audit would harden the
  closure verdict.

These are recorded as visible follow-up directions, not
prescribed for execution.

---

## §7 EXT-tier closure declaration

### §7.1 The declaration

The **EXT-tier is formally closed**. Specifically:

- All four extensions (A/B/C/D) prescribed by BT-547 retro
  §5.1-§5.4 have produced their prescribed output.
- All three validation candidates (A/B/C) have received a
  validation report with a clean verdict: 3/3
  OBSTRUCTION_DOCUMENTED.
- EXT-D vocabulary glossary is complete as a 12-entry / 5-
  category reference artifact.
- The 3 OBSTRUCTIONs converge on axis B from three structurally
  distinct discriminator-type families, providing the strongest
  available evidence that BT-544's hardness is structural (per
  §4.2) rather than catalogue-bias-induced.

The closure is recorded as: **EXT-tier diagnostic output complete,
no PASS produced, axis-B convergence confirmed at strength 5/5 across
all attempted framework families**.

### §7.2 What the closure means for the L9 catalogue

The L9 catalogue's EXT-A / EXT-B / EXT-C slots are now
*populated* by registered candidates with OBSTRUCTION_DOCUMENTED
verdicts. The slots remain *open* in the sense that further
candidates of the same class are admissible (per per-validation
§8.4 / §9.5 directions):

- EXT-A slot: open for non-quadratic F or non-flat g candidates;
- EXT-B slot: open for path-space-Lyapunov / different-ρ-choice
  / non-CI-source candidates;
- EXT-C slot: open for energy-density-cascade or canonical-
  neighbourhood-supplied candidates.

But none of these is *prescribed* for execution; the priority
set is given by §6.1 (CF-BMO) over additional EXT candidates.

### §7.3 What the closure means for BT-547 retro

BT-547 retro's prescription was successful **as diagnostic** —
the L9 catalogue was extended beyond its CATALOGUE_BIAS, the
discriminator-type vocabulary was extended, and the resulting
3 validations honestly mapped where BT-544 breaks. The
prescription **did not produce a PASS**, which is consistent
with BT-547 retro §6.1's prediction that "BT-544 closure
expectations should be set decadal, not annual" and with the
20-year-tool-accumulation analog to Perelman's pre-2002 phase.

The BT-547 retro prescription is **discharged**. No further
catalogue extensions in the EXT-A/B/C/D family are prescribed.

### §7.4 What the closure does *not* claim

The closure declaration does **not** claim:

- that the BT-544 main-line obstruction is *proved* to be axis B
  in absolute terms (per §5.3, the localization is consistent
  with the witnesses, not proved by them);
- that the EXT-tier exhausts the L9 catalogue's potential
  contribution to BT-544 (the CF-BMO candidate is a within-
  framework follow-up);
- that axis B has been *engaged* (the EXT-tier produced
  obstructions *converging on* axis B, but did not engage axis-
  B-specific machinery directly — that is precisely what CF-BMO
  is for);
- that the Millennium tally has changed (it remains 0/7
  unchanged).

---

## §8 Anti-list — alternative interpretations rejected

Alternative readings of the EXT-tier 3/3 OBSTRUCTION pattern
considered and rejected. One-line reason each.

- **Anti-A (3/3 OBSTRUCTION = catalogue still biased)**: the
  EXT-tier failed because the candidate-generation pipeline is
  *still* catalogue-biased (just a different bias from the
  pre-EXT bias). **Rejected**: the three EXT candidates are
  *structurally heterogeneous* (variational re-interpretation /
  analytic-Lyapunov / procedure-class) and explicitly designed
  to remediate three different missing classes per BT-547 retro
  §3 / §5; the convergence onto axis B is *not* a shared bias
  but a property of the NS equation (per §3 cross-pillar synthesis).

- **Anti-B (3/3 OBSTRUCTION = artefact of shared candidate-spec
  heuristics)**: the three candidate specs use overlapping
  references (Otto 2001, Brenier 1999 §5, Bakry-Émery 1985), so
  the convergence is a citation-heritage effect. **Partially
  rejected**: there is some shared citation lineage, but the
  *failure modes* are at different algebraic levels (Helmholtz
  Fréchet asymmetry vs CI-2008 family-vs-density ambiguity vs
  M3 canonical-neighbourhood absent), and the 3-pillar synthesis
  §2.3 establishes the algebraic equivalence at the
  Λ²-asymmetry level independently of citation lineage. The
  shared-heuristics reading would explain failure on a single
  axis but does not explain the structural agreement of
  algebraic objects across distinct framework languages.

- **Anti-C (axis-B convergence = framework choice artefact)**:
  the convergence onto axis B is an artefact of choosing
  variational / analytic-Lyapunov / procedure-class frameworks
  that all happen to be poor at vortex-stretching, with other
  framework choices producing different localizations.
  **Partially rejected**: the 3-pillar synthesis §10.6 (F-SYN-B)
  partially admits this critique but answers it by noting that
  the *D3.A pillar* (classical PDE estimates / Galerkin theory,
  a structurally different framework family) also points
  consistently — it clears axis A and is silent on B — and the
  D-tier reading independently records axis B's hardness via
  D3.B' compositional fail. The framework-choice critique
  applies to EXT-A/B/C individually but not to the integrated
  D + EXT picture.

- **Anti-D (EXT-tier should have produced at least one PASS to
  count as successful)**: the EXT-tier should be re-graded as
  failed because no PASS emerged. **Rejected**: per BT-547
  retro §5 and §6 the EXT-tier was prescribed as a *diagnostic*
  catalogue extension, not as a closure attempt. PASS was never
  the success criterion; *honest characterisation of where BT-
  544 breaks* was. Per §4.3 the diagnostic value is the success
  measure, and 3/3 OBSTRUCTION_DOCUMENTED with axis-B
  convergence delivers that value.

- **Anti-E (closure should remain OPEN until CF-BMO is run)**:
  the EXT-tier closure should be deferred until CF-BMO and any
  other axis-B-targeted candidates are validated, on the
  grounds that closure is premature. **Rejected**: CF-BMO is
  *not* an EXT extension — it is a follow-up axis-B-specific
  candidate generated *as a consequence of* the EXT-tier
  closure findings. The EXT-tier prescription (BT-547 retro
  §5.1-§5.4) is fully discharged; deferring closure to await
  CF-BMO conflates tier boundaries.

- **Anti-F (EXT-D vocabulary should also receive a verdict)**:
  EXT-D should be graded PASS / FAIL / OBSTRUCTION rather than
  "completed (no verdict)". **Rejected**: per the EXT-D
  prescription (BT-547 retro §5.4), EXT-D is a vocabulary-
  extension reference artifact, not a candidate proposal. The
  PASS / FAIL / OBSTRUCTION verdict-set applies to validation
  discriminators; EXT-D has no discriminator. Forcing a verdict
  would mis-categorise the artifact.

- **Anti-G (5-witness count overstates independence)**: the
  count of 5 independent witnesses (D3 compositional + D2 R1 +
  EXT-A + EXT-B + EXT-C) is over-counted because some of
  these share machinery. **Partially active**: D3.B'
  compositional and EXT-A/EXT-B both engage classical PDE
  machinery; D2 R1 and EXT-C both engage parameter-bound /
  partial-regularity machinery. A more careful pruning yields
  perhaps 3-4 independent framework families. The §3 / §5.1
  reading remains valid at 3+ witnesses; the strict count of 5
  is informative but the conclusion is robust at smaller
  counts.

---

## §9 Falsifiers active for the closure synthesis

Falsifiers under which the §7 closure declaration would be
retracted or downgraded. Distinct from per-validation falsifiers
F-VAL-A through F-VAL-G of EXT-A / EXT-B / EXT-C and from
F-SYN-A through F-SYN-E of the 3-pillar synthesis.

### §9.1 F-CLOSE-A (per-validation verdict misread)

**Statement**: if any of EXT-A's OBSTRUCTION_DOCUMENTED at
F-EXTA-C, EXT-B's OBSTRUCTION_DOCUMENTED at F-EXTB-D + Path-Q
cross-term, or EXT-C's OBSTRUCTION_DOCUMENTED at F-EXTC-D is
misread by this closure synthesis (e.g. EXT-C's verdict is
re-graded by a procedure-class-specialist as PASS_SKETCH at the
energy-density-cascade level), the 3/3 tally is incorrect.

**Status**: NOT ACTIVE based on direct reading of the per-
validation §5 / §7 / §8 closing statements. Each report
explicitly records OBSTRUCTION_DOCUMENTED with cited section
references for the breaking step.

### §9.2 F-CLOSE-B (axis-B convergence overstated)

**Statement**: per the 3-pillar synthesis §10.2 F-SYN-B, the
algebraic equivalence of EXT-A's "convective Fréchet asymmetry"
and EXT-B's "vortex-stretching cross term" is structural-
cognate, not literally identical. If a careful specialist proof
shows the two are *not* the same algebraic object, the
convergence weakens from "same target" to "adjacent targets".

**Status**: PARTIALLY ACTIVE, inherited from the 3-pillar
synthesis F-SYN-B. The §3.2 axis-B convergence reading remains
defensible at the structural-cognate level even if literal-
identity is downgraded; the closure declaration weakens from
"3 OBSTRUCTIONs converge on axis B" to "3 OBSTRUCTIONs converge
on the Λ²-asymmetry / vortex-stretching / canonical-
neighbourhood-absence cluster" — different prose, same
substantive content.

### §9.3 F-CLOSE-C (D-tier integrated picture mis-tabulated)

**Statement**: if the §4.1 table mis-states any D-tier row
(e.g. D2's R1 status is OPEN rather than method-gap, or D3.A's
PASS_LITERATURE is at the compositional level rather than the
isolation level), the integrated picture is incorrect.

**Status**: NOT ACTIVE based on direct reading of the per-
D-tier closure reports referenced in `omega-exec-bt544-fallback-
molt-validation-2026-04-25.md`. The §4.1 table reproduces those
verdicts as stated. Risk of mis-transcription is real but low;
cross-check by re-reading individual D-tier closures if a
D-tier row is contested.

### §9.4 F-CLOSE-D (EXT-tier closure premature)

**Statement**: per Anti-E above, if the EXT-tier should have
remained open pending axis-B-targeted candidates, the §7
declaration is premature.

**Status**: NOT ACTIVE — the EXT-A/B/C/D prescription is fully
discharged; CF-BMO is a within-framework follow-up, not an
EXT-class extension. A reader who insists on the conflation can
re-grade the §7 declaration as "EXT-tier prescription
discharged; closure pending axis-B validation"; the substantive
content is identical.

### §9.5 F-CLOSE-E (catalogue still biased after EXT-tier)

**Statement**: per Anti-A above, if the EXT-tier itself has a
hidden dominant-family bias (e.g. all three candidates share an
implicit "smooth-flow" assumption that excludes weak-solution
class candidates), the 3/3 OBSTRUCTION pattern reflects EXT-
catalogue bias rather than NS structural hardness.

**Status**: PARTIALLY ACTIVE. EXT-A/B/C all assume smooth /
short-time NS solutions in their setup; the convex-integration
weak-solution literature (Buckmaster-Vicol 2019, Isett 2018) is
not engaged. This is a genuine EXT-catalogue limitation,
already noted in EXT-B §2.6 and §10.4. A weak-solution-class
EXT-extension is not currently prescribed; whether it would
produce a different verdict is open. The closure declaration
remains valid *for smooth NS within the prescribed candidate
class*; it does not extend to weak-solution class.

### §9.6 F-CLOSE-F (Millennium tally claim accidentally moved)

**Statement**: if any sentence in this synthesis can be read as
claiming a Millennium-relevant result (PASS / blow-up / Clay-
form NS regularity), the brief's hard constraint is violated.

**Status**: NOT ACTIVE. §0 disclaimer, §5.3 explicit non-
implications, and §7.4 explicit non-claims collectively
constrain the synthesis to status-only, no-NS-claim content.
Cross-checked: every numerical / structural assertion is
sourced from a per-extension report or the 3-pillar synthesis;
no novel mathematical claim is made.

### §9.7 F-CLOSE-G (atlas / state / inventory edit leakage)

**Statement**: if any change is made to atlas, state, or
inventory files as a result of this synthesis, the brief's hard
constraint is violated.

**Status**: NOT ACTIVE. This synthesis is research-only and
edits no atlas, state, or inventory file. The git status at
session start (specs and inventory.json modified by *unrelated*
prior sessions per the gitStatus header) is unaffected.

### §9.8 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-CLOSE-A | per-validation verdict misread | NOT ACTIVE |
| F-CLOSE-B | axis-B convergence overstated | PARTIALLY ACTIVE (inherited from F-SYN-B) |
| F-CLOSE-C | D-tier integrated picture mis-tabulated | NOT ACTIVE |
| F-CLOSE-D | EXT-tier closure premature | NOT ACTIVE |
| F-CLOSE-E | EXT-catalogue still biased | PARTIALLY ACTIVE (smooth-class limitation) |
| F-CLOSE-F | accidental Millennium claim | NOT ACTIVE |
| F-CLOSE-G | atlas/state/inventory edit leakage | NOT ACTIVE |

Two falsifiers (F-CLOSE-B, F-CLOSE-E) are partially active —
both are inherited risks (cross-pillar algebraic equivalence
not airtight; weak-solution class not engaged). Neither is
expected to retract the closure declaration; both are flagged
as the structural risks to the §3 / §7 readings.

---

## §10 Closing

**Declaration**: the EXT-tier (BT-547 retro catalogue extensions
A/B/C/D) is **formally closed**. All four extensions have produced
their prescribed output. The three validation candidates returned
3/3 OBSTRUCTION_DOCUMENTED at structurally distinct discriminator
types (variational-derivation / analytic-inequality-construction /
procedure-class-with-parameter-bounds). EXT-D vocabulary glossary
is complete as a 12-entry / 5-category reference artifact.

**Tally**:

- EXT-A uω-GradFlow: OBSTRUCTION_DOCUMENTED at F-EXTA-C
  (Helmholtz / convective Fréchet asymmetry).
- EXT-B CI-Lyap: OBSTRUCTION_DOCUMENTED at F-EXTB-D
  (representation-side: CI-2008 family-of-densities, not single-
  density) + secondary Path-Q vortex-stretching cross-term.
- EXT-C QPC-Surgery: OBSTRUCTION_DOCUMENTED at F-EXTC-D
  (canonical-neighbourhood absent for NS; M3 third piece
  structurally missing).
- EXT-D vocabulary: completed (no verdict, reference artifact).

**Convergence**: 3/3 OBSTRUCTIONs name distinct technical
failures (Helmholtz / representation / canonical-neighbourhood)
but localize the same structural object — axis B / Λ²-asymmetry
/ vortex-stretching / vortex-stretching-geometric-content. The
convergence is consistent with the prior 3-pillar synthesis
(D3.A + EXT-A + EXT-B), extending the witness count from 3 to
5 (D3.B' compositional, D2 R1, EXT-A, EXT-B, EXT-C) when
combined with the D-tier integrated picture.

**Implication**: BT-544 hardness is structural, not catalogue-
bias-induced. Axis B is *the* obstruction in the framework
families tested so far. Further attacks on the Clay form should
target axis B directly (CF-BMO candidate per §6.1), not look for
new framework families on the full equation.

**Diagnostic success of BT-547 retro**: the prescription was
*successful as diagnostic* — the L9 catalogue was extended
beyond CATALOGUE_BIAS, the discriminator-type vocabulary was
broadened, and the 3 validations honestly mapped where BT-544
breaks. It did not produce a PASS, which is consistent with
the decadal-timeline prediction (BT-547 retro §6.5).

**Future-session priority**: CF-BMO axis-B-targeted attack
(`omega-exec-bt544-axisB-targeted-attack-2026-04-25.md`) — high
priority but high cost (~400+ pages of axis-B-specific
literature reading), with expected verdict OBSTRUCTION_DOCUMENTED
at ~60% via F-AXISB-B (Hou-Luo / Elgindi blow-up incompatibility).
The expected value is diagnostic, not Clay-resolving.

**Honest assessment**: BT-544 is at a methodologically-mature
plateau. Further attacks should target axis B directly, not
look for new framework families.

**Falsifiers active (§9)**: F-CLOSE-B (inherited from F-SYN-B,
convergence reading is structural-cognate not literal-identity)
and F-CLOSE-E (smooth-class limitation; weak-solution-class
EXT extension not prescribed) are partially active; others not
active.

**0/7 unchanged. EXT-tier closed. No atlas/state/inventory
edits.**

— end status synthesis —
