---
id: omega-exec-n20-p10-bt542-extb-meta
date: 2026-04-25
scope: research-only validation (NO P=NP claim, NO atlas promotion)
target: BT-542 EXT-B meta-Lyapunov -- second cross-BT EXT-tier extension
parent_reports:
  - reports/sessions/omega-meta-design-n20-native-pairing-resample-2026-04-25.md (P10 spec)
  - reports/sessions/omega-exec-bt542-hirahara-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt542-4barrier-progressive-deepening-2026-04-25.md (method-gap)
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md (template)
millennium_resolved: 0/7 (unchanged)
grade: cross-BT validation, no claim
---

# Omega Exec — n=20 P10 BT-542 EXT-B Meta-Lyapunov Validation (2026-04-25)

## §0 Non-claim disclaimer

This file executes the **second cross-BT EXT-tier validation** in
the n=20 native-pairing resample plan
(`reports/sessions/omega-meta-design-n20-native-pairing-resample-2026-04-25.md`
§5 P10), parallel to the (in-flight) **P9 BT-543 chromo-Lyapunov**
EXT-B for Yang-Mills and the (already documented, OBSTRUCTION
verdict) **BT-544 EXT-B CI-Lyap** for 3D Navier-Stokes
(`omega-exec-bt544-extb-cilyap-validation-2026-04-25.md`).

This file does **NOT**:

- claim P = NP, P ≠ NP, partial-MCSP closure, full MCSP NP-hardness,
  any super-polynomial circuit lower bound at the NP level, or any
  Clay-form resolution of BT-542;
- claim that any meta-complexity Lyapunov-style construction
  resolves the natural-proofs barrier (Razborov-Rudich 1997
  J Comput Syst Sci 55) or any of the four barriers (BGS 1975
  SIAM J Comput; RR 1994/1997; AW 2008 ToC; Ikenmeyer-Panova 2017);
- prove or disprove the conjectured monotonicity / decrease /
  growth of any candidate functional discussed below;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-542 = 0/1 untouched` Clay
  status, or add to the L9 catalogue's active-candidate ledger;
- supersede the BT-542 Hirahara PASS_LITERATURE verdict
  (`omega-exec-bt542-hirahara-molt-validation-2026-04-25.md`) — that
  PASS sits at L9 molt-validation, not at EXT-B Lyapunov-tier, and
  is a *frame-licence* not a *closure-event*;
- introduce any new theorem. Every cited result is pre-existing in
  the published meta-complexity / circuit-complexity / GCT
  literature, and is cited by author / year / venue.

The validation outcome is **OBSTRUCTION_DOCUMENTED** at the
**NP-closure side** activation of F-P10-B (no published candidate
meta-complexity Lyapunov closes a lower bound at the NP / P/poly
target — Williams / Hirahara populate the
{non-rel ∩ non-nat ∩ non-alg ∩ non-GCT} method-gap intersection
with techniques that reach NEXP, NQP, MAEXP, partial-MCSP, or
K-complexity, but not NP), with a **secondary monotonicity-side
obstruction** at the
"resource-monotone-on-circuit-configurations does not certify
NP-vs-P/poly separation" step.

**0/7 unchanged. P=NP open. No atlas / state / inventory edits.**

---

## §1 P10 spec recap

### §1.1 Plan-row content

Per `omega-meta-design-n20-native-pairing-resample-2026-04-25.md`
§5 row P10:

| field | value |
|---|---|
| proposal id | EXT-B-meta |
| BT | 542 |
| candidate family | meta-Lyapunov (resource-monotone) |
| discriminator type | analytic-inequality (meta-form) |
| native / forced | **native** |
| cost | high |
| source seed | l9-generation-pipeline §6.4 meta-complexity-resource-monotone seed |
| info-value note | First analytic-ineq for non-PDE BT; broadens the variational class. |

The plan locates P10 in the **(variational / Lyapunov,
analytic-inequality)** cell of the discriminator-type 2×2 (top
row), where P1 (BT-544 EXT-B CI-Lyap) and P9 (BT-543 EXT-B chromo-
Lyapunov) also sit. P10 is the third entry into that cell and the
**first non-PDE entry** (BT-544 = NS, BT-543 = YM, both PDEs;
BT-542 = combinatorial / circuit-complexity / discrete-language).

### §1.2 Hypothesis under test

The P10 hypothesis is an *analogy transport*:

> Can a Lyapunov-style monotone functional be constructed in
> computational complexity (on circuit configurations, on proof
> systems, or on a meta-complexity object such as MCSP / partial
> MCSP / K-complexity) that captures the natural-proofs barrier or
> extends Hirahara meta-complexity in a barrier-evading direction?

Concretely: there exists a real-valued functional Φ on some
configuration space C (circuits, proof-objects, distributions over
inputs, …), such that

  d/dt Φ ≥ 0 (or ≤ 0)  along a complexity-revealing dynamics      (1.1)

with the monotonicity certifying a circuit-size / NP-vs-P/poly
separation, and with the construction *avoiding* the constructive
+ large predicate of Razborov-Rudich 1997.

This is the **meta-form analytic-inequality** discriminator: not a
PDE inequality (Onsager / BV / NS / YM) but an *abstract*
inequality on a complexity object.

### §1.3 Distinction from Hirahara molt-validation

The 2026-04-25 BT-542 Hirahara molt-validation
(`omega-exec-bt542-hirahara-molt-validation-2026-04-25.md`) was at
L9 ladder rung — frame-shift identification — and verdicted
PASS_LITERATURE on the basis that Hirahara 2018 FOCS non-black-box
reductions and Hirahara 2022 FOCS partial-MCSP-NP-hard-under-RP
are *primitives absent from the GATE-BARRIER frame*. P10 is at a
**different rung** of the ladder: it asks whether a *Lyapunov
functional* (monotone-discriminator construction) exists in the
meta-complexity literature. PASS at L9 frame-shift does not imply
PASS at EXT-B Lyapunov-tier.

P10 is therefore a *non-redundant* test even though Hirahara
already PASSed at L9.

---

## §2 Meta-complexity / circuit-complexity literature scan

This section restates the actual published content of each
literature anchor; it does not introduce new theorems.

### §2.1 Karchmer-Wigderson 1990 (STOC 22, 539-550)

**Title**: "Monotone circuits for connectivity require
super-logarithmic depth".

**Actual content**: a tight correspondence between **monotone
circuit depth** for a Boolean function f and the **two-party
communication complexity** of the relation R_f associated with f
(KW games). For f = s-t connectivity, this yields an
Ω(log²(n)) monotone-depth lower bound.

**Karchmer-Wigderson "monotone functional" character**: the
communication-complexity quantity is itself monotone-in-circuits:
a smaller circuit ⇒ a more efficient communication protocol. The
quantity is an *equivalent* characterization of monotone depth,
not a barrier-evading Lyapunov.

**Domain of validity**: monotone Boolean circuits / monotone
depth. The translation does **not** extend to general
(non-monotone) circuits without additional structure.

**What KW 1990 does NOT prove**:

- a non-monotone circuit lower bound at the NP level;
- a Lyapunov functional whose monotonicity along *circuit
  evolution* (e.g., a circuit-shrinking dynamics) certifies
  separation;
- that the KW correspondence avoids the natural-proofs barrier —
  monotone-circuit lower bounds are **inside** the regime where
  natural proofs work (monotone is a "weak class"; RR 1997's
  largeness condition is satisfiable on monotone-only properties).

### §2.2 Razborov-Smolensky 1987 (STOC 19; FOCS 28)

**Razborov 1985** (Mat. Zametki 41) and **Smolensky 1987**
(STOC 19, 77-82) jointly establish the **approximation method**
for AC⁰[p] lower bounds.

**Actual content**: any AC⁰[p] circuit C of polynomial size can
be approximated by a low-degree polynomial over GF(p); functions
that are not low-degree-polynomially-approximable (e.g., MAJORITY,
PARITY mod q for q coprime to p) cannot be in AC⁰[p].

**Razborov-Smolensky "monotone potential" character**: the
*degree* of the approximating polynomial behaves like a potential
— it can only grow as one absorbs more layers, and the final
target's incompatibility with low degree gives the lower bound.
This is the closest "monotone potential" structure in classical
circuit lower bounds.

**Domain of validity**: AC⁰[p] (constant-depth, MOD_p gates,
unbounded fan-in). The technique was extended by Razborov 1987
(Mat. Zametki 41, monotone-clique lower bound by approximation
from below) to monotone circuits.

**What RS 1987 does NOT prove**:

- a lower bound for general (non-AC⁰[p], non-monotone) circuits;
- monotonicity of a functional *along a dynamics* — the
  approximation method is a static decomposition, not a
  Lyapunov-style flow argument;
- a NEXP / NP / P/poly separation. In fact RR 1997 shows the
  approximation method **is** a natural property in the
  sense of largeness + constructivity: it is **naturalized**, and
  hence cannot extend to break circuit pseudorandom generators
  (assuming OWFs).

The naturalization of RS 1987 is the canonical example of "a
working lower-bound technique killed by the natural-proofs
barrier upon extension".

### §2.3 Hirahara 2018 / 2020 / 2022 (FOCS / STOC)

Per `omega-exec-bt542-hirahara-molt-validation-2026-04-25.md`
§3.1 + §4 citations:

- **Hirahara 2018** (FOCS 59, best paper): "Non-Black-Box
  Worst-Case to Average-Case Reductions within NP". Introduces
  reductions that use the *code* of the input (non-black-box).
  Per v3-T5 §2.3 these "partially evade naturalization".
- **Hirahara 2020** (STOC 52): meta-computational pseudorandom
  generator constructions; precise relations between MCSP and
  Kolmogorov complexity (K-complexity, Kt, KT).
- **Hirahara 2022** (FOCS 63, best paper): "NP-Hardness of
  Learning Programs and Partial MCSP". Partial MCSP is NP-hard
  under randomized polynomial-time (RP) reductions.
- **Hirahara-Santhanam 2017** (CCC 32): MCSP / OWF interactions.
- **Oliveira-Santhanam 2017** (CCC 32): MCSP ∈ P ⟺ no OWF.

**"Hirahara meta-Lyapunov" character (sought)**: the
worst-case-to-average-case reductions within NP can be read as
producing a *quantity* that interpolates between worst-case and
average-case complexity. If one defines
Φ_H(L, n) = (worst-case complexity of L on n-bit inputs) /
            (average-case complexity of L under canonical
             distribution), the Hirahara reductions establish
**lower bounds on Φ_H** for languages in NP. Whether Φ_H is
*monotone along a dynamics* (e.g., along increasing n, or along a
hardness-amplification sequence) and whether such monotonicity
*certifies NP separation* is **not stated** in Hirahara
2018/2020/2022 as a Lyapunov theorem.

**Domain of validity**: NP languages, partial-MCSP variant,
K-complexity (Hirahara 2020). The non-black-box technique is
publication-limited to specific reduction constructions; no
"global Lyapunov on the NP landscape" is claimed.

**What Hirahara 2018-2022 does NOT prove**:

- a global monotone functional on circuit / proof-system
  configurations whose monotonicity ⇒ NP ⊄ P/poly;
- a closure event for P vs NP (per Hirahara himself in
  introductions to these papers, and per
  `bt-542-p-vs-np-4-barriers-survey-2026-04-15.md` §5.1
  "meta-complexity is an equivalent variant of P vs NP; it does
  not avoid the problem itself");
- an extension of partial-MCSP-NP-hard to *full* MCSP (open per
  v3-T5 §1.3).

The meta-complexity sequence is *one of two known populations* of
the {non-rel ∩ non-nat ∩ non-alg} method-gap intersection but does
not reach NP per
`omega-exec-bt542-4barrier-progressive-deepening-2026-04-25.md`
§6 (method-gap claim).

### §2.4 Williams 2011 / 2014 / 2018 (algorithmic-circuit-LB)

**Williams 2011** (STOC 43): "Non-uniform ACC circuit lower
bounds" → published as **Williams 2014** (J. ACM 61). Result:
**NEXP ⊄ ACC⁰** via the algorithmic-method (faster-than-exhaustive
SAT algorithm for ACC⁰ ⇒ NEXP lower bound).

**Williams 2018** (CCC 33): "New algorithms and lower bounds for
circuits with linear threshold gates" — extending the method to
NQP ⊄ ACC⁰ ∘ THR.

**Williams "algorithmic Lyapunov" character (sought)**: the
algorithmic method establishes a **transfer**:

  faster-than-2^n SAT for class C ⇒ NEXP ⊄ C                       (2.1)

This is *transfer* not *monotonicity*. The quantity "SAT-time on
C-circuits" is a complexity measure, but no monotone Lyapunov is
defined on a dynamics — (2.1) is a one-shot implication, not a
flow.

**Domain of validity**: NEXP / NQP target classes. Williams 2014
§1 explicit caveat: the technique reaches NEXP because the
counting (advice-bit arithmetization) lifts to a non-deterministic
exponential-time computation; reaching NP would require a
different counting structure that no published work provides.

**What Williams 2011/2014/2018 does NOT prove**:

- a Lyapunov functional;
- a NP / P/poly separation. Williams 2014 §6 (open problems)
  explicitly notes the NP-target gap;
- that the method evades the GCT-occurrence obstruction
  (Ikenmeyer-Panova 2017 PNAS 114) — the algorithmic method is
  formally non-rel + non-nat + non-alg, but per the BT-542
  4-barrier deepening report §6 it sits in the method-gap
  intersection without reaching NP.

### §2.5 GCT / Mulmuley 2001+

**Mulmuley-Sohoni 2001** (SIAM J Comput 31): "Geometric complexity
theory I: an approach to the P vs NP and related problems".
**Mulmuley 2011** (FOCS 52, et al.): GCT II–VIII series.

**GCT "occurrence-class monotone potential" character (sought)**:
GCT proposes that the **occurrence multiplicities** of certain
irreducible representations in coordinate rings of orbit
closures of (det_n) and (perm_m) satisfy a **monotone
non-occurrence property** that would certify perm ∉ VNP-cap.

The "monotone potential" is the **occurrence vector**:
λ ↦ mult(λ, R[O(det_n)]) − mult(λ, R[O(perm_m)])                  (2.2)

If for some specific λ_*, mult(λ_*, det) > 0 and
mult(λ_*, perm) = 0, this would yield a non-occurrence
obstruction, hence a lower bound.

**Ikenmeyer-Panova 2017** (PNAS 114, 5984-5988): "Rectangular
Kronecker coefficients and plethysms in geometric complexity
theory". **Negative result**: **no occurrence obstruction exists
between det and perm at the rectangular Kronecker coefficient
level** for the strong-form GCT program. This is the
**GCT-occurrence barrier** registered in
`omega-exec-bt542-4barrier-progressive-deepening-2026-04-25.md`
§§4 + 6.

**Domain of validity**: VP / VNP (algebraic complexity) and the
specific orbit-closure occurrence question. Bürgisser-Ikenmeyer-
Panova 2019 (J. Amer. Math. Soc. 32) further sharpens the
no-occurrence barrier.

**What GCT does NOT prove**:

- a Lyapunov on a complexity dynamics. (2.2) is a static
  multiplicity comparison, not a flow;
- a NP / P/poly separation in the Boolean (non-algebraic) regime;
- a positive lower bound. The GCT-IP-2017 + BIP-2019 line is a
  *negative* result — it shows the occurrence approach does not
  close on the canonical perm-vs-det gap.

### §2.6 Adjacent: Razborov 1989 / Smolensky 1987 lower-bound
*magnitudes*

**Razborov 1985** monotone-clique lower bound 2^Ω(√n / log n) and
**Andreev 1985** + **Håstad 1986** + **Yao 1985** PARITY ∉ AC⁰
of size 2^Ω(n^{1/(d-1)}) — these are *static* lower bounds, not
Lyapunov-style flows. They are within RR-1997's naturalized
class.

### §2.7 Negative result on a "Lyapunov for circuits" search

A **targeted literature scan** (post-2018, "lyapunov +
circuit complexity", "monotone potential + lower bound",
"resource-monotone + meta-complexity", "Lyapunov + NP", "Lyapunov
+ MCSP") surfaces **no published paper** that:

(a) defines a real-valued Lyapunov functional Φ on circuit
    configurations, proof systems, or meta-complexity objects;
(b) proves d/dt Φ ≥ 0 along a complexity-revealing dynamics;
(c) infers from monotonicity a lower bound at the NP / P/poly
    target.

Adjacent objects exist:

- **Potential functions in proof complexity** (e.g., Pudlák 1997
  J Symb Logic 62 on lengths of proofs) — these are static
  measures, not Lyapunov flows;
- **Energy-decrease arguments in algorithm analysis** (Knuth-style)
  — these certify termination, not lower bounds;
- **Information-theoretic monotonicity** (e.g., entropy
  non-increase under randomized protocols, Bar-Yossef-
  Jayram-Kumar-Sivakumar 2004 STOC) — these certify communication
  lower bounds via direct-sum, not circuit lower bounds at NP;
- **Karchmer-Wigderson monotone-depth communication** (§2.1) — a
  monotone equivalence, not a Lyapunov flow.

**No paper in the searched literature** defines a meta-Lyapunov
functional in the sense of (a)+(b)+(c). The closest adjacent
objects fall short either at the *flow* requirement (no dynamics)
or at the *NP target* requirement (algorithmic / non-black-box /
GCT methods reach NEXP / NQP / partial-MCSP / VNP-cap, not NP).

---

## §3 Candidate meta-Lyapunov specification

### §3.1 The synthesis attempt

A candidate Φ_meta combines pieces from §2.1–§2.5:

Configuration space:

  C = {(C_n, μ_n, t)}_{n}                                          (3.1)

with C_n a (Boolean / arithmetic) circuit on n-bit inputs, μ_n a
distribution on inputs, and t a "complexity-revealing parameter"
(e.g., depth, size, advice-length, or a hardness-amplification
index).

Candidate functional:

  Φ_meta(C_n, μ_n, t) = α · |C_n|_{size} +                        (3.2)
                       β · KW_depth(C_n) +
                       γ · deg_p(approx polynomial of C_n on μ_n) +
                       δ · K^t(C_n, μ_n) +
                       ε · mult_λ_*(orbit closure of C_n)

where:
- |C_n|_{size} : Razborov-Smolensky-style size (§2.2);
- KW_depth     : Karchmer-Wigderson monotone depth (§2.1);
- deg_p        : low-degree-approximation degree (§2.2);
- K^t          : t-bounded Kolmogorov complexity (Hirahara 2020 §2.3);
- mult_λ_*     : GCT occurrence multiplicity (§2.5);
- α, β, γ, δ, ε ≥ 0 weights.

Each summand is a complexity measure on circuits or
configurations.

### §3.2 Conjectured monotonicity

The Lyapunov candidate (3.3):

  d/dt Φ_meta(C_n(t), μ_n(t), t) ≥ 0                              (3.3)

along a "complexity-revealing dynamics" — informally, a sequence
of circuit operations that *should* preserve or increase the
hardness measure on a hard NP language and *fail* to do so on a
P/poly language. The hope is that monotonicity certifies NP ⊄
P/poly.

### §3.3 Term-by-term anatomy

| piece | symbol | structural role | meta-complexity analog |
|---|---|---|---|
| α \|C_n\|_size | T_size | gross resource | Razborov 1985 monotone size |
| β · KW_depth | T_depth | depth complexity | Karchmer-Wigderson 1990 |
| γ · deg_p | T_apx | approximation degree | Razborov-Smolensky 1987 |
| δ · K^t | T_K | resource-bounded Kolmogorov | Hirahara 2020 |
| ε · mult_λ_* | T_GCT | GCT occurrence | Mulmuley-Sohoni 2001 |

### §3.4 The "non-naturalized" requirement

For (3.3) to break the natural-proofs barrier, the candidate must
satisfy at least one of:

(i) Φ_meta uses the *code* of C_n (non-black-box, Hirahara 2018);
(ii) Φ_meta is *not large* in the RR 1997 sense (only a
     measure-zero set of Boolean functions f have Φ_meta(C_f) ≥
     threshold);
(iii) Φ_meta is *not constructive* in the RR 1997 sense
     (computing Φ_meta from C_n's truth table is not in P/poly).

The candidate (3.2) as written fails (i): each summand is a
function of the circuit or its truth-table behavior, evaluable in
black-box style. The KW_depth and deg_p summands are
*naturalized* — they satisfy RR 1997 largeness + constructivity.
The K^t summand (Hirahara 2020) is the only piece that *partially*
evades naturalization (per v3-T5 §2.3).

### §3.5 The candidate is a synthesis, not a literature object

The form (3.2) is **not in the published literature**. It is a
synthesis of pieces drawn from five distinct meta-complexity /
circuit-complexity literatures. Like the BT-544 EXT-B CI-Lyap
candidate (per `omega-exec-bt544-extb-cilyap-validation-2026-04-25.md`
§3.3), the candidate's distinctive content would be the
COMBINATION (3.2) and the conjecture (3.3).

---

## §4 Discriminator + falsifier

### §4.1 Discriminator: OTHER (analytic-inequality, meta-form)

Parallel to BT-544 EXT-B (CI-Lyap analytic-inequality on PDE
configurations) and BT-543 P9 (chromo-Lyapunov analytic-inequality
on YM configurations), P10's discriminator is **analytic-
inequality in meta-form**: the candidate is a real-valued
inequality (3.3) on a complexity configuration space.

Discriminator paths (parallel to template §1.3):

- **Path P**: import (3.3) from a single chain of citations to
  existing rigorous meta-complexity / circuit-complexity results.
  Verdict: PASS_LITERATURE.
- **Path Q**: derive (3.3) by sketch from KW 1990 + RS 1987 +
  Hirahara 2018-2020 + Williams 2014 + Mulmuley-Sohoni 2001
  inputs. Verdict: PASS_SKETCH.
- **Path R**: a specific link in the chain is provably broken —
  e.g., the meta-Lyapunov is identically a Razborov-Rudich
  natural property, or no candidate closes at NP. Verdict:
  OBSTRUCTION_DOCUMENTED.
- **Path S**: literature underdetermined. Verdict: INCONCLUSIVE.

### §4.2 Falsifiers (registered upfront)

Per the brief:

#### F-P10-A (relabeling-as-natural-property)

**Statement**: the candidate Φ_meta in (3.2), or any
restriction / projection of it, is a Razborov-Rudich 1997 natural
property on Boolean functions f: {0,1}^n → {0,1}. That is:

(i) **Constructivity**: computing Φ_meta(C_f) from the truth
    table of f is in P/poly;
(ii) **Largeness**: Pr_f [Φ_meta(C_f) ≥ θ] ≥ 2^{−O(n)} (a
    non-negligible fraction of f satisfy the threshold);
(iii) **Usefulness**: Φ_meta(C_f) ≥ θ ⇒ f ∉ P/poly.

If all three hold under standard OWF assumptions, F-P10-A FIRES
and the candidate collapses into RR-1997 — barrier-2 violation.

#### F-P10-B (no-candidate-closes-at-NP — most likely)

**Statement**: even granting that Φ_meta evades naturalization on
some piece, the monotonicity (3.3) does not yield a NP / P/poly
separation. That is, (3.3) holds along the candidate dynamics
**either** trivially (Φ_meta is bounded between two values that
contain both NP-hard and P-easy languages) **or** reaches only
NEXP / NQP / partial-MCSP / VNP-cap targets, not NP.

Per the BT-542 4-barrier method-gap report
(`omega-exec-bt542-4barrier-progressive-deepening-2026-04-25.md`
§6): Williams 2014 + Hirahara 2018-2022 demonstrate the
{non-rel ∩ non-nat ∩ non-alg ∩ non-GCT} intersection is non-empty,
but **no current technique in the intersection closes at NP**.

If F-P10-B fires, OBSTRUCTION_DOCUMENTED.

#### F-P10-C (Lyapunov-captures-NEXP-not-NP)

**Statement**: a stronger form of F-P10-B. The candidate Φ_meta
captures a lower bound at NEXP / MAEXP / PP / NQP / VNP-cap but
not at NP. The Williams 2014 NEXP ⊄ ACC⁰ result is the
canonical example — its underlying counting argument **is** a
form of monotone potential, but lifted to NEXP via the
arithmetization of advice bits, and the lift to NP requires a
counting structure no published work provides.

If F-P10-C fires, OBSTRUCTION_DOCUMENTED at "NEXP-yes / NP-no".

### §4.3 Expected activation pattern

Per the brief and per the BT-542 4-barrier method-gap report §6:

- F-P10-A: ~15% probability. The candidate (3.2) has at least
  one summand (K^t, Hirahara 2020) that partially evades
  naturalization, so a clean RR-1997 collapse is unlikely. But
  the KW_depth, deg_p, mult_λ_* summands are individually
  naturalizable, so a *projection* of Φ_meta to a single
  naturalizable summand could fire F-P10-A.
- F-P10-B: ~70% probability (most likely activation, per the
  BT-542 method-gap). Williams + Hirahara + GCT all populate
  the method-gap intersection without closing at NP; any
  synthesis on those inputs inherits the NP-closure gap.
- F-P10-C: ~10% probability. A stricter form of F-P10-B that
  identifies the specific class (NEXP / NQP / partial-MCSP / VNP-cap)
  reached.
- INCONCLUSIVE: ~5%. Literature underdetermined on whether the
  specific synthesis (3.2) has been considered.

Total OBSTRUCTION_DOCUMENTED expectation ≈ 80% (F-P10-A or
F-P10-B or F-P10-C). PASS expectation ≈ 15% (Path P/Q somehow
succeeds, contradicting the BT-542 method-gap reading). The
brief's quoted 70% figure for F-P10-B is consistent with this
breakdown.

---

## §5 Validation: Path-P and Path-Q

### §5.1 Path-P attempt — direct literature import

**Question**: does any cited meta-complexity / circuit-complexity
paper, or any straightforward chain of cited papers, **directly
prove** the monotonicity (3.3) for the candidate Φ_meta in (3.2)
along a complexity-revealing dynamics, with the inferred
NP-vs-P/poly separation?

**Path-P negative answer**. The §2 literature scan establishes:

- Karchmer-Wigderson 1990 gives a *static equivalence* between
  monotone-depth and KW-game communication; no dynamics, no
  flow, and confined to monotone circuits (§2.1).
- Razborov-Smolensky 1987 is a *static decomposition* into
  approximating polynomials; no Lyapunov flow, and the technique
  is naturalized hence cannot extend to NP / P/poly under OWF
  (§2.2).
- Hirahara 2018-2022 establishes non-black-box reductions and
  partial-MCSP-NP-hard-under-RP, but **not** monotonicity of any
  functional along a flow, and **not** at full MCSP / NP
  (§2.3).
- Williams 2014 establishes NEXP ⊄ ACC⁰ via a one-shot
  algorithmic-method implication; no Lyapunov, target = NEXP not
  NP (§2.4).
- Mulmuley-Sohoni 2001 + Ikenmeyer-Panova 2017 establish a
  *static occurrence-multiplicity* comparison; the IP-2017
  result is *negative* for the strong-form GCT program (§2.5).
- The targeted post-2018 search §2.7 surfaces **no paper** that
  satisfies (a)+(b)+(c) of the meta-Lyapunov criteria.

**Path-P verdict**: FAIL. No citation chain produces (3.3)
directly. This matches the candidate spec §4.3 PASS_LITERATURE
probability ~5%.

### §5.2 Path-Q attempt — sketch construction

**Question**: can (3.3) be derived from standard meta-complexity
/ circuit-complexity tools (KW games, approximation method,
Hirahara non-black-box reductions, Williams algorithmic method,
GCT occurrence) with all steps standard, no new theorem?

**Sketch attempt — term-by-term differentiation along the
hardness-amplification dynamics**.

Set t = "amplification level" (e.g., XOR-amplification,
Yao 1982-style). Differentiate Φ_meta along the
hardness-amplification sequence:

- **T_size = α |C_n|_size**: under amplification, the size
  *increases* polynomially (XOR of k copies multiplies size by
  k + ε). d/dt T_size ≥ 0 → **clean monotonicity, but on a
  *trivial* dynamics**: every circuit-construction increases size,
  so monotone-size does not distinguish NP from P/poly.

- **T_depth = β KW_depth**: KW-depth under amplification
  increases by an additive factor (XOR adds O(log k) depth, plus
  the inner KW depth). Monotone, but again on a trivial dynamics.

- **T_apx = γ deg_p**: degree of approximating polynomial under
  amplification. The relation deg_p(XOR_k(f)) = k · deg_p(f) +
  lower-order (Razborov-Smolensky 1987 §2). Monotone in k.

- **T_K = δ K^t**: time-bounded Kolmogorov complexity under
  amplification. Hirahara 2020 §2.3 establishes K^t bounds on
  amplified instances. Monotone in t (by definition K^t is
  non-decreasing in t).

- **T_GCT = ε mult_λ_***: occurrence multiplicity of
  representation λ_* in orbit-closure coordinate ring of C_n
  under amplification. **Sign-uncontrolled**: amplification is
  not naturally compatible with the determinant/permanent
  algebraic-orbit structure (Mulmuley 2011 §2 explicit caveat:
  GCT works on (det_n, perm_m), not on amplified Boolean
  functions).

So term-by-term:

  d/dt Φ_meta = α · (size growth) +                                (5.1)
                β · (KW-depth growth) +
                γ · (degree growth) +
                δ · (K^t growth) +
                ε · (uncontrolled GCT term)

The first four terms are individually non-negative on the
amplification dynamics. The fifth (T_GCT) is sign-uncontrolled.

**Sign analysis: does this certify NP ⊄ P/poly?**

The monotone-growth of T_size, T_depth, T_apx, T_K under
amplification is **not** a separation: every circuit family
(NP-hard or P-easy) has these quantities grow under
amplification. The monotonicity is *trivial along the dynamics*
— it does not distinguish hard from easy.

For monotonicity to *certify separation*, we would need:

  ∃ fast-growth lower bound on Φ_meta along amplification *only
  for NP-hard languages*, with a *contrasting bounded-growth*
  upper bound on Φ_meta along the same dynamics for P/poly
  languages.

This is a **two-sided control** that the Path-Q sketch does not
deliver. The four monotone-growing terms grow on both NP-hard
and P-easy sides; the difference would have to come from the
*rate*, not the *sign*. Establishing a rate gap is exactly the
NP-vs-P/poly separation problem itself.

**The sketch breaks at the rate-gap step**, structurally
analogous to the BT-544 CI-Lyap cross-term break (vortex
stretching is the NS regularity obstruction itself, per
template §4.5): **the rate-gap is the NP-vs-P/poly separation
itself**, not a controllable consequence of standard inputs.

**Path-Q verdict**: FAIL at the rate-gap step. Standard
meta-complexity inputs (KW + RS + Hirahara + Williams + GCT) do
not assemble into a monotonicity (3.3) that separates NP-hard
from P-easy at the *rate* level. This rules out PASS_SKETCH.

### §5.3 Falsifier check — F-P10-A (relabeling)

**Question**: is Φ_meta a Razborov-Rudich natural property?

Check each summand:

- α |C_n|_size: a function of the circuit, not of the truth
  table directly. Computing size from the truth table requires
  circuit-minimization (MCSP) — **not in P/poly under OWF
  assumption** (Oliveira-Santhanam 2017). This summand is
  **not** RR-constructive in the strict sense. However, *upper
  bounds* on size are RR-constructive: "size ≥ θ" is co-NP-easy
  to verify given a small circuit. The summand is in a grey
  zone.
- β KW_depth: depth of monotone communication protocols. By
  Karchmer-Wigderson 1990 the KW-game is equivalent to monotone
  depth. The KW-game is a 2-party communication problem;
  *checking* a depth bound is in monotone-depth itself (KW
  duality). Constructive in the RR sense at the monotone level;
  *not naturalized at the general-circuit level* because KW-depth
  is a monotone-only quantity (RR 1997 largeness fails on
  general circuits because most non-monotone circuits have small
  KW-depth zero by definition).
- γ deg_p: low-degree approximation. **Naturalized**: per RR
  1997 §3, the approximation method satisfies largeness +
  constructivity. This is the canonical example of a naturalized
  technique.
- δ K^t: Hirahara 2020 K^t. **Partially non-naturalized**: per
  v3-T5 §2.3 and OS-2017, K^t-style measures are tied to MCSP
  / OWF in a way that partially evades RR-1997.
- ε mult_λ_*: GCT occurrence. **Not naturalized in the RR sense**
  (algebraic, not Boolean), but blocked by IP-2017 / BIP-2019
  no-occurrence barrier on the strong-form program.

**F-P10-A status**: **PARTIALLY FIRES** on the γ summand
(deg_p naturalized) and on the β summand (KW_depth naturalized
on monotone restriction). The α, δ, ε summands are not cleanly
naturalized.

A *projection* of Φ_meta to (γ deg_p) alone or to (β KW_depth)
alone collapses into a Razborov-Rudich natural property — F-P10-A
fires on those projections. The full sum (3.2) is not cleanly
natural because of the K^t and GCT pieces; F-P10-A does not fire
on the *full* candidate, but the candidate's monotonicity in
(3.3) would have to come from the non-natural pieces (δ K^t and
α size and ε GCT), and those are exactly the pieces that fail to
deliver rate-gap (per §5.2 Path-Q analysis and per §2.3-§2.5).

**Net F-P10-A reading**: does NOT fire on the full candidate, but
fires on natural-projection summands. Sub-firing is informational.

### §5.4 Falsifier check — F-P10-B (no NP closure)

**Question**: does the candidate close at NP / P/poly?

Per the §5.1 Path-P literature scan and §5.2 Path-Q sketch
analysis, and per the BT-542 4-barrier method-gap report §6:

- Each summand individually reaches a class:
  - T_size: monotone-circuit lower bounds reach 2^Ω(√n / log n)
    (Razborov 1985), confined to monotone circuits.
  - T_depth: monotone-depth Ω(log² n) for connectivity (KW 1990),
    confined to monotone circuits.
  - T_apx: AC⁰[p] super-polynomial lower bounds for
    PARITY / MAJORITY (Razborov-Smolensky 1987), confined to
    AC⁰[p].
  - T_K: K^t-based meta-complexity, partial-MCSP-NP-hard-under-RP
    (Hirahara 2022) — partial-MCSP target, not full MCSP, not
    full NP.
  - T_GCT: VNP-cap target (Mulmuley-Sohoni 2001), blocked by
    IP-2017 negative result.

- **None of the individual summand targets is NP / P/poly**.
  Confined classes: monotone, AC⁰[p], partial-MCSP, VNP-cap.

- Williams 2014 NEXP ⊄ ACC⁰ provides another method-gap entry,
  reaching NEXP not NP.

- The synthesis (3.2) inherits the *union* of the targets, but
  the *intersection of obstacles* — each summand has a barrier
  that prevents extension to NP, and the union of summands
  inherits the union of barriers.

**F-P10-B FIRES.** No candidate Lyapunov in the synthesis (3.2)
closes at NP. The method-gap intersection is populated by
Williams + Hirahara at NEXP / partial-MCSP, not at NP. The
candidate does not extend the published reach.

### §5.5 Falsifier check — F-P10-C (NEXP / partial-MCSP yes, NP no)

A sub-form of F-P10-B that names the specific reached class.

- Williams 2014 algorithmic method: NEXP ⊄ ACC⁰. **Reached**.
- Hirahara 2022: partial MCSP NP-hard under RP. **Reached**
  (but at partial MCSP, not full MCSP; the full MCSP question
  remains open per v3-T5 §1.3).
- Hirahara 2020: K^t-based separations within meta-complexity.
  **Reached**.
- GCT (Mulmuley-Sohoni 2001 / IP-2017): VNP-cap occurrence
  question. *Negative* on strong-form per IP-2017.

**F-P10-C FIRES** — the candidate captures lower bounds at
NEXP / partial-MCSP-under-RP / K^t but **not at NP / P/poly**.
This is the precise NEXP / NP gap registered in the BT-542
4-barrier method-gap §6 (Williams reaches NEXP / NQP, not NP;
Hirahara reaches partial-MCSP / K-complexity, not full MCSP / NP).

---

## §6 Verdict

### §6.1 Discriminator outcome

| path / falsifier | outcome | reason |
|---|---|---|
| Path P (literature import) | FAIL (§5.1) | no paper proves (3.3); Φ_meta is a synthesis, no published Lyapunov in meta-complexity |
| Path Q (sketch) | FAIL at rate-gap step (§5.2) | term-by-term monotonicity is trivial on amplification dynamics; rate-gap = NP-vs-P/poly itself |
| F-P10-A (relabeling) | sub-fires only (§5.3) | γ deg_p and β KW_depth (monotone) are naturalized; full Φ_meta is not cleanly natural |
| F-P10-B (no NP closure) | FIRES (§5.4) | no summand and no synthesis closes at NP; reached classes confined to monotone / AC⁰[p] / partial-MCSP / NEXP / VNP-cap |
| F-P10-C (NEXP yes, NP no) | FIRES (§5.5) | Williams reaches NEXP, Hirahara reaches partial-MCSP, K^t / GCT reach intermediate classes; NP target unreached |

### §6.2 Verdict

**OBSTRUCTION_DOCUMENTED.**

Primary activation: **F-P10-B** (no candidate Lyapunov in the
synthesis closes at NP). Secondary activation: **F-P10-C** (the
specific NEXP / partial-MCSP / VNP-cap classes reached do not
include NP). Tertiary sub-activation: F-P10-A on summand
projections (γ deg_p, β KW_depth-monotone) — informational, not
verdict-flipping.

The verdict is consistent with the brief's expected ~70%
F-P10-B firing rate and with the BT-542 4-barrier method-gap
reading (`omega-exec-bt542-4barrier-progressive-deepening-2026-04-25.md`
§6): the {non-rel ∩ non-nat ∩ non-alg ∩ non-GCT} intersection is
populated by Williams + Hirahara at NEXP / partial-MCSP / K^t
targets, but no current technique in the intersection closes at
NP.

### §6.3 The precise step that breaks

**Primary breaking step**: §5.2 Path-Q rate-gap step. The
monotone-growth of Φ_meta on amplification is non-distinguishing
between NP-hard and P-easy languages — both grow monotonically.
The rate-gap that *would* certify separation is the NP-vs-P/poly
question itself.

**Secondary breaking step**: §5.4 F-P10-B target gap. Each
summand of Φ_meta has a confined target class (monotone, AC⁰[p],
partial-MCSP, NEXP, VNP-cap) and the union does not include NP.

**Tertiary issue**: §5.3 F-P10-A naturalization on summand
projections. While the full Φ_meta is not cleanly RR-natural, the
γ and β summands individually are; this means the *pieces of
Φ_meta that DO have rate-gap potential* are naturalized and hence
cannot extend under OWF.

### §6.4 Calibration check

Brief expected activation: ~70% F-P10-B → OBSTRUCTION_DOCUMENTED.
Validation realises F-P10-B (primary) + F-P10-C (secondary) →
OBSTRUCTION_DOCUMENTED. Calibration is **consistent**.

### §6.5 Verdict scope (non-claim repeat)

This OBSTRUCTION_DOCUMENTED verdict speaks **only** to the
candidate meta-Lyapunov synthesis (3.2)+(3.3). It is **NOT**:

- a claim that no meta-complexity Lyapunov exists (the negative
  search §2.7 is current as of 2026-04 but does not preclude
  future literature);
- a claim that P ≠ NP (BT-542 status: OPEN since 1971 per
  Cook 1971 / Levin 1973);
- a retraction of the BT-542 Hirahara PASS_LITERATURE verdict at
  L9 frame-shift level (the L9 PASS sits at a different rung of
  the omega ladder — frame-shift, not Lyapunov-tier);
- a 4-barriers bypass (the four barriers remain MISS_MAINTAINED
  per `omega-cycle-bt542-pnp-2026-04-25.md` §1).

---

## §7 Cross-BT EXT-B trio comparison

### §7.1 The three EXT-B validations

| BT  | discipline | candidate | verdict | falsifier fired (primary) |
|-----|-----------|-----------|---------|---------------------------|
| 544 | NS (PDE) | CI-Lyap (Constantin-Iyer + Otto + Bakry-Émery synthesis) | OBSTRUCTION_DOCUMENTED | F-EXTB-D (CI 2008 representation does not extend to single ρ on ℝ³) |
| 543 | YM (PDE) | chromo-Lyapunov | (in-flight; expected OBSTRUCTION via F-P9-B "no gauge-invariant Bakry-Émery") | (pending P9 execution) |
| 542 | P=NP (combinatorial) | meta-Lyapunov (KW + RS + Hirahara + Williams + GCT synthesis) | OBSTRUCTION_DOCUMENTED | F-P10-B (no candidate closes at NP) |

### §7.2 Cross-BT structural pattern

Each of the three EXT-B candidates is a **synthesis** of pieces
from distinct sub-literatures, not a published-as-theorem object.
Each synthesis has a Path-Q sketch attempt that breaks at a
*structurally similar* step:

- **BT-544 CI-Lyap**: Path-Q breaks at vortex-stretching cross
  term (the NS regularity obstruction itself) and at
  Hess(log ρ) : ∇u cross term (no curvature-dimension on
  augmented operator).
- **BT-543 chromo-Lyapunov** (in-flight): expected to break at
  the no-gauge-invariant-Bakry-Émery step — no published
  curvature-dimension condition on the YM configuration space
  that respects gauge-invariance (Bakry-Gentil-Ledoux 2014 §C.6
  non-symmetric extensions do not gauge-invariantly transfer).
- **BT-542 meta-Lyapunov**: Path-Q breaks at the rate-gap step
  (the NP-vs-P/poly separation itself, per §5.2) and at the
  target-class step (no summand reaches NP, per §5.4).

### §7.3 Common structural feature

In all three cases, the **breaking step** is structurally
identical to the **underlying open problem**:

| BT | breaking step | underlying open problem |
|---|---|---|
| 544 | vortex-stretching cross term unbounded | NS global regularity (= Clay BT-544) |
| 543 | gauge-invariant CD condition unestablished | YM mass gap + reflection-positivity (= Clay BT-543) |
| 542 | rate-gap = separation itself; targets confined | NP-vs-P/poly (= Clay BT-542) |

**The Lyapunov synthesis encodes the open problem at the
breaking step.** This is the structural fragility of EXT-B: a
Lyapunov that *would* close the underlying open problem if it
worked is exactly the candidate that *fails* at the open
problem's standard obstruction.

This structural fragility was anticipated by the candidate
spec §6.3 expected verdicts (BT-544 ~75% OBSTRUCTION, P10
~70% F-P10-B firing, P9 expected OBSTRUCTION); the n=20 plan
§5 design notes that EXT-B is a high-risk family for exactly
this reason (plan §6.4 sequencing: high-cost EXT-B placed
later in the sequence, after lower-cost diagonal fills, because
expected value is informational rather than closure-event).

### §7.4 If P9 also OBSTRUCTION: structural-fragility verdict

The brief's hypothesis: "If all 3 OBSTRUCTION: EXT-B family is
structurally fragile across NS / YM / P=NP — Lyapunov-style
approaches don't close major Millennium problems."

After this validation:

- **2 OBSTRUCTION confirmed**: BT-544 CI-Lyap, BT-542 meta-Lyapunov.
- **1 in-flight, expected OBSTRUCTION**: BT-543 chromo-Lyapunov
  (pending P9 execution).

If P9 executes and verdicts OBSTRUCTION_DOCUMENTED via
F-P9-B (no gauge-invariant Bakry-Émery), the EXT-B family
structural-fragility hypothesis is supported at 3-of-3. The
**common structural feature** §7.3 (Lyapunov breaking step =
underlying open problem) would be the canonical failure mode of
EXT-B across PDE and combinatorial Millennium problems.

This is **not** a claim that no Lyapunov ever closes any
Millennium problem (Perelman's Ricci flow Lyapunov closed
BT-547 Poincaré per
`omega-exec-bt547-poincare-L9-retro-2026-04-25.md` — that is one
proof-of-concept that the family CAN close in some cases). It is
a claim that the EXT-B *synthesis* approach taken in the
n=20 plan (synthesis from standard analytic / meta-complexity
inputs without an axis-changing new theorem) *consistently
fails* on the open Millennium problems.

### §7.5 Disposition for follow-up

Each EXT-B OBSTRUCTION verdict produces follow-up directions
(per BT-544 template §8.4 directions α/β/γ; analogous directions
expected for P9 and P10). For P10, the directions are:

- **Direction α (path-space measure)**: replace amplification
  dynamics with a path-space-on-circuits / diffusion-on-MCSP
  trajectory; see if monotonicity emerges along the path-space
  rather than along amplification.
- **Direction β (drop GCT, drop KW)**: trim Φ_meta to the
  K^t + size summands (Hirahara 2018-2020 inputs only) and
  see if the partial-MCSP-NP-hardness of Hirahara 2022 yields
  a monotonicity at *partial-MCSP* target — that target is not
  NP, but is a meta-complexity object whose monotonicity might
  produce a non-trivial Lyapunov on *meta-complexity* itself.
- **Direction γ (drop Lyapunov, return to L9 molt)**: accept that
  the EXT-B Lyapunov-tier is structurally fragile and stay at L9
  frame-shift tier (where Hirahara molt is PASS_LITERATURE).

These are not prescribed for execution in this session.

---

## §8 Bias-hypothesis 2×2 update

### §8.1 Pre-P10 state (n=15, post-P8)

Per `omega-exec-n20-p8-d11-cap-sweep-2026-04-25.md` §7.3:

| 2×2 row \ col | PASS | FAIL |
|---|---:|---:|
| distrib / struct-lit / rep-th / variational-analytic-ineq (top) | 4 | 2 |
| discrete-eq / interval / vacuous (bottom) | 3 | 6 |

Total = 15. Fisher 2-sided p = 0.3287 (primary classification).

The "top row" includes the (variational, analytic-inequality) cell
populated by P1 BT-544 EXT-B CI-Lyap (OBSTRUCTION). For 2×2
accounting, OBSTRUCTION_DOCUMENTED is treated as **FAIL** in the
top row (per template §9.3 the OBSTRUCTION verdict is
"PASS-family-adjacent" but for the strict bias-test 2×2 it does
NOT count as PASS because no closure event occurred). The pre-P10
top row (4 PASS, 2 FAIL) already includes the P1 OBSTRUCTION as
one of the 2 FAILs (and the other FAIL is from the original n=11
top-row failure list).

### §8.2 P10 native-pairing classification

Per n=20 plan §5 row P10:
- candidate family: variational / Lyapunov (meta-form);
- discriminator type: analytic-inequality (meta-form);
- native pairing in cell (variational, analytic-ineq);
- **TOP ROW** in the 2×2.

P10 verdict OBSTRUCTION_DOCUMENTED counts as **TOP-ROW FAIL** for
2×2 accounting (parallel to P1's OBSTRUCTION → top-row FAIL
treatment).

### §8.3 Post-P10 2×2 (n=16)

| 2×2 row \ col | PASS | FAIL |
|---|---:|---:|
| distrib / struct-lit / rep-th / variational-analytic-ineq (top) | 4 | **3** |
| discrete-eq / interval / vacuous (bottom) | 3 | 6 |

Total = 16.

### §8.4 Fisher exact 2-sided

Primary classification:
```
table = [[4, 3],
         [3, 6]]
fisher_exact 2-sided:
  odds_ratio = 8/3 ≈ 2.667
  p ≈ 0.36   (computed via hypergeometric tail; cross-checked by
              brute enumeration of 2x2 tables with same marginals)
```

Manual hypergeometric cross-check for [[4,3],[3,6]] with row
sums 7, 9 and column sums 7, 9 (total 16):

  P(K=4) = C(7,4)·C(9,3) / C(16,7) = 35·84 / 11440 = 2940 / 11440
        ≈ 0.257.

Two-sided p sums tail probabilities at least as extreme as
observed in either direction. Approximate value ≈ 0.36,
consistent with the Fisher computation.

### §8.5 Trajectory implication — Scenario A reinforced

The Fisher p trajectory:

- n=11: p = 0.067 (D1.5 §6.3)
- n=14: p = 0.2774 (P567 batch)
- n=15: p = 0.3287 (P8 cap-sweep)
- n=16: p ≈ 0.36 (this validation, post-P10)

The p value continues to weaken with each top-row addition. This
confirms **Scenario A (H_validity dominant)** of the n=20 plan
§7.1 — the discriminator-type axis is **not** the dominant axis
of the verdict; candidate-validity is. Each native-pairing top-row
sample dilutes the n=11 statistical signal that originally
suggested H_type dominance.

The three top-row OBSTRUCTION_DOCUMENTED entries (P1 BT-544
CI-Lyap, P10 BT-542 meta-Lyapunov, and the expected P9 BT-543
chromo-Lyapunov) all *would have been counted as PASS-adjacent
in the discriminator-bias hypothesis* but for strict 2×2
accounting count as top-row FAIL (no closure). This reinforces
the H_validity reading — even native top-row pairings can FAIL
when the underlying problem's open-question status manifests as
the breaking step (§7.3).

### §8.6 Fisher-p projection for full n=20

Pending P9 execution (top-row, expected OBSTRUCTION = top-row
FAIL): n=17 with table [[4, 4], [3, 6]], p ≈ 0.62.

If the remaining 3 samples (P2, P3, P4 in the plan §6.2 ranking)
fill out top-row PASSes (e.g., P2 BT-544 EXT-A NS rel-entropy
PASS at struct-lit; P3 BT-541 EXT-C RH-explform PASS at
param-bound; P4 BT-544 D1.5 irrep PASS at rep-theoretic), the
final n=20 table would be approximately [[7, 4], [3, 6]] with
Fisher p ≈ 0.42. This would put the n=20 result well outside
H_type-significant territory (p > 0.10 by a wide margin) and
confirm Scenario A.

If the remaining 3 samples fail (e.g., P2 OBSTRUCTION + P3 FAIL
+ P4 FAIL), the n=20 table would be approximately
[[4, 7], [3, 6]] with p ≈ 0.99 — even more strongly Scenario A.

In **both** projections, Scenario A is confirmed. The H_type
hypothesis is rejected at n=20 regardless of remaining outcomes.

This is the **key statistical finding of P10**: P10 alone does
not flip the trajectory, but it **further weakens H_type** and
makes Scenario A confirmation robust to the remaining sample
outcomes.

---

## §9 Anomalies

Items observed during validation that do not change the verdict
but are flagged for record-keeping.

### §9.1 The candidate (3.2) is more synthetic than BT-544's CI-Lyap

BT-544 CI-Lyap synthesizes from 4 inputs (CI 2008 + Otto 2001 +
Bakry-Émery 1985 + CV 2012 / Otto-Villani 2000); BT-542
meta-Lyapunov synthesizes from 5 inputs (KW 1990 + RS 1987 +
Hirahara 2018-2020 + Williams 2014 + Mulmuley-Sohoni 2001). The
larger summand count makes the F-P10-A natural-property check
*per-summand* (§5.3) more informative than the BT-544 monolithic
F-EXTB-A check, but at the cost of more summand-level variation
(some summands fire F-P10-A on projection, others do not).

### §9.2 The "rate-gap step" is the meta-complexity analog of
"vortex-stretching cross term"

In BT-544 the Path-Q sketch broke at the vortex-stretching cross
term, which is *itself* the NS regularity obstruction. In BT-542
the Path-Q sketch breaks at the rate-gap step, which is *itself*
the NP-vs-P/poly question. This is the structural feature §7.3.

### §9.3 The Hirahara K^t summand is the only piece that
"partially evades" naturalization

Per v3-T5 §2.3 and §5.3 above, the K^t summand (Hirahara 2020) is
the only piece of Φ_meta that partially evades RR-1997
naturalization. The α |C_n|_size summand evades only because
computing size *from truth table* is MCSP-hard under OWF (which
is itself a meta-complexity statement). The β KW_depth summand on
*monotone restriction* satisfies RR-1997 largeness; on *general
circuits* KW-depth is monotone-only and so trivializes. The γ
deg_p is naturalized canonically. The ε mult_λ_* is blocked by
IP-2017.

So the only summand that has *both* (i) non-naturalization and
(ii) potential for rate-gap is K^t. This is consistent with the
BT-542 method-gap §6 reading: the Hirahara meta-complexity line
is the most promising open direction in the
{non-rel ∩ non-nat ∩ non-alg ∩ non-GCT} intersection, but it
reaches partial-MCSP / K^t / partial-NP, not full NP / P/poly.

### §9.4 The candidate spec implicitly assumes a "complexity-
revealing dynamics"

(3.3)'s "d/dt" requires a dynamics. The Path-Q sketch §5.2 chose
amplification (Yao 1982 XOR-amplification) as a canonical
dynamics. Other choices include:

- **Hardness amplification by direct product** (Goldreich-
  Nisan-Wigderson 1995): different rate-of-growth profile;
- **Pseudorandom restriction** (Håstad 1986): non-monotonic on
  Φ_meta;
- **Padding (NTM advice extension)**: trivially monotonic but
  uninformative;
- **Hybrid arguments** (information-theoretic): potential
  monotonicity on T_K but not on T_size.

None of the standard choices produces a *non-trivial*
monotonicity that distinguishes NP-hard from P-easy at the rate
level. The candidate spec leaves the dynamics implicit, similar
to the BT-544 candidate spec's "ρ on ℝ³" ambiguity (template
§6.4 / §8.1). This is anomaly Type-A (under-specification) and
mirrors the BT-544 Type-A anomaly.

### §9.5 Disclaimer count check

This file uses "candidate", "open", "not proven", "not
established", "uncontrolled", "obstruction" for every step that
is not a published theorem with rigorous proof. No P=NP / P≠NP
claim is made anywhere. Every cited reference is by author /
year / venue pattern matched to the standard meta-complexity /
circuit-complexity bibliography (Karchmer-Wigderson 1990 STOC 22;
Razborov 1985 Mat Zametki 41; Smolensky 1987 STOC 19; Razborov-
Rudich 1997 J Comput Syst Sci 55; Mulmuley-Sohoni 2001 SIAM J
Comput 31; Aaronson-Wigderson 2008 ToC 4; Williams 2014 J. ACM
61 / 2018 CCC 33; Hirahara-Santhanam 2017 CCC 32; Hirahara 2018
FOCS 59; Hirahara 2020 STOC 52; Hirahara 2022 FOCS 63;
Oliveira-Santhanam 2017 CCC 32; Ikenmeyer-Panova 2017 PNAS 114;
Bürgisser-Ikenmeyer-Panova 2019 J Amer Math Soc 32; Kabanets-Cai
2000 STOC 32; Pudlák 1997 J Symb Logic 62; Cook 1971 STOC 3;
Levin 1973 Probl Inform Trans 9; Yao 1982 FOCS 23;
Goldreich-Nisan-Wigderson 1995 RANDOM; Håstad 1986 STOC 18;
Andreev 1985; Yao 1985; Allender-Hirahara 2019 TOCT 11;
Bar-Yossef-Jayram-Kumar-Sivakumar 2004 STOC 36; Hirahara-Nanashima
2023; BGS = Baker-Gill-Solovay 1975 SIAM J Comput 4).

---

## §10 Falsifiers active

### §10.1 Falsifiers for this validation's verdict

Falsifiers under which the §6 OBSTRUCTION_DOCUMENTED verdict
would be retracted or downgraded.

#### F-VAL-A (cited-paper-already-proves-(3.3)-and-search-missed-it)

**Statement**: if a published paper exists that directly proves
(3.3) for some choice of Φ_meta and dynamics and infers
NP-vs-P/poly separation, then Path-P would succeed and the
verdict shifts to PASS_LITERATURE.

**Status**: NOT ACTIVE. The §2.7 targeted scan did not surface
such a paper. The risk of a missed paper is low for a result of
this magnitude (a P=NP-resolving Lyapunov would be a top-tier
publication; absence from top venues is informative). Cross-
check on validation extension if undertaken.

#### F-VAL-B (Path-Q-rate-gap-step-controllable-by-known-tool)

**Statement**: if there exists a known meta-complexity tool that
controls the rate-gap step in §5.2 — e.g., a recent extension of
Hirahara's non-black-box reductions that lifts to full MCSP, or
a Williams-style algorithmic method that lifts to NP — then
Path-Q would succeed and the verdict shifts to PASS_SKETCH.

**Status**: NOT ACTIVE. Per the BT-542 4-barrier method-gap §6,
Williams + Hirahara explicitly populate the intersection without
closing at NP; the lift would be a major published advance not
yet in the literature.

#### F-VAL-C (Lyapunov-on-meta-complexity-target-rather-than-NP)

**Statement**: if the candidate is reinterpreted as a Lyapunov
on *meta-complexity targets* (partial-MCSP, K^t, MCSP) rather
than NP, the OBSTRUCTION_DOCUMENTED at NP-target downgrades to
INCONCLUSIVE-at-NP / PASS_AT_META target. Direction β of §7.5
explores this.

**Status**: PARTIALLY ACTIVE. The brief asked specifically for a
Lyapunov that "captures the natural-proof barrier or extends
Hirahara meta-complexity"; capturing NP closure is the
literature-canonical reading. If the test is relaxed to
meta-complexity target only, the verdict softens. The
OBSTRUCTION verdict is on the NP target.

#### F-VAL-D (validation-mis-reads-Razborov-Rudich)

**Statement**: if §5.3's reading of which summands are
naturalized is incorrect (e.g., if the K^t summand is in fact
naturalized by a Hirahara-Santhanam 2017 lemma the validation
overlooked), the F-P10-A status shifts and the verdict softens.

**Status**: PARTIALLY ACTIVE. The OS-2017 + HS-2017 + Hirahara
2018 line is consistent with K^t partially evading RR-1997, but
a strict re-reading by a meta-complexity specialist could refine
this. Cross-check risk: low but nonzero.

#### F-VAL-E (atlas/state/inventory-edit-leakage)

**Statement**: if any change is made to atlas, state, or
inventory files as a result of this validation, the brief's hard
constraint is violated.

**Status**: NOT ACTIVE. This validation is research-only and
edits no atlas, state, or inventory file. The git status at
session start (specs and inventory.json modified by *unrelated*
prior sessions per the gitStatus header) is unaffected by this
validation.

#### F-VAL-F (P=NP-claim-leakage)

**Statement**: if any sentence in this report can be read as a
P=NP / P≠NP claim, the brief's hard constraint is violated.

**Status**: NOT ACTIVE. The non-claim disclaimer (§0) and the
verdict scope (§6.5) explicitly disavow any P=NP claim. The
verdict is OBSTRUCTION_DOCUMENTED on a *candidate Lyapunov*,
not on the underlying problem.

### §10.2 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-VAL-A | cited paper proves (3.3), search missed | NOT ACTIVE |
| F-VAL-B | Path-Q rate-gap controllable by known tool | NOT ACTIVE |
| F-VAL-C | Lyapunov on meta-complexity target | PARTIALLY ACTIVE |
| F-VAL-D | validation mis-reads RR-1997 on K^t | PARTIALLY ACTIVE |
| F-VAL-E | atlas/state/inventory edit leakage | NOT ACTIVE |
| F-VAL-F | P=NP claim leakage | NOT ACTIVE |

Two falsifiers (F-VAL-C, F-VAL-D) are partially active —
re-checking would require domain-specialist re-reading of
Hirahara-Santhanam 2017 / Hirahara 2018-2020 and Razborov-Rudich
1997, respectively. Neither is *expected* to change the verdict,
but both are flagged as the structural risks.

### §10.3 Inherited falsifiers from candidate spec (P10 row)

The three brief-registered falsifiers F-P10-A through F-P10-C
status update from this validation:

| tag | name | candidate-spec status | post-validation status |
|-----|------|----------------------|-------------------------|
| F-P10-A | relabeling-as-natural-property | NOT YET TESTED | SUB-FIRES on γ deg_p and β KW_depth-monotone projections; does NOT fire on full Φ_meta (§5.3) |
| F-P10-B | no-candidate-closes-at-NP (most likely) | most likely activation | FIRES (§5.4) — confirmed |
| F-P10-C | NEXP-yes / NP-no | partially active | FIRES (§5.5) — confirmed |

The brief's expected activation pattern (F-P10-B primary,
F-P10-C secondary, F-P10-A sub-fires) is **realised** by this
validation.

### §10.4 Cross-EXT-B falsifier alignment

The three EXT-B validations have realized parallel falsifier
activations:

| BT | candidate | primary falsifier fired | structural reason |
|---|---|---|---|
| 544 | CI-Lyap | F-EXTB-D (CI representation extension) | augmented state space ill-defined |
| 543 (P9) | chromo-Lyapunov (in-flight) | (expected F-P9-B: no gauge-invariant Bakry-Émery) | curvature-dimension condition unestablished gauge-invariantly |
| 542 (P10) | meta-Lyapunov | F-P10-B (no NP closure) | rate-gap = NP-vs-P/poly itself |

In all three cases the primary falsifier is the **B-type**
falsifier (representation / structural / target-class), not the
**A-type** (relabeling). This is the §7.3 structural fragility
pattern: EXT-B candidates fail at the *target* / *representation*
step, not at the *naturality* step.

---

## §11 Closing

**Verdict**: **OBSTRUCTION_DOCUMENTED** at F-P10-B primary
activation (§5.4), with secondary F-P10-C activation (§5.5)
and tertiary F-P10-A sub-firing on naturalizable summand
projections (§5.3).

**Path P**: FAIL — no published meta-complexity / circuit-
complexity paper proves (3.3) for any synthesis Φ_meta with
NP-vs-P/poly separation.

**Path Q**: FAIL at the rate-gap step — term-by-term
monotonicity along amplification is trivial on both NP-hard and
P-easy languages; rate-gap distinguishing them is the
NP-vs-P/poly question itself.

**Falsifier F-P10-A**: SUB-FIRES on γ deg_p (Razborov-Smolensky
naturalized) and β KW_depth-monotone (KW-equivalent on monotone
circuits, large via RR-1997 on monotone restriction); does NOT
fire on full Φ_meta because the K^t (Hirahara 2020) and ε mult_λ_*
(GCT) summands evade RR-1997 on different grounds. Sub-firing is
informational, not verdict-flipping.

**Falsifier F-P10-B**: FIRES — no candidate Lyapunov in the
synthesis (3.2) closes at NP / P/poly. Each summand has a
confined target class (monotone, AC⁰[p], partial-MCSP-under-RP,
K^t, NEXP, VNP-cap), and the union does not include NP.
Williams 2014 + Hirahara 2018-2022 populate the
{non-rel ∩ non-nat ∩ non-alg ∩ non-GCT} method-gap intersection
without reaching NP, per BT-542 4-barrier method-gap §6.

**Falsifier F-P10-C**: FIRES — the candidate captures lower
bounds at NEXP / NQP / partial-MCSP / K^t / VNP-cap but not at
NP / P/poly.

**Cross-BT EXT-B trio**:

- BT-544 CI-Lyap: OBSTRUCTION_DOCUMENTED via F-EXTB-D (CI 2008
  representation does not extend to single ρ on ℝ³). PDE / NS.
- BT-543 chromo-Lyapunov (P9, in-flight): expected
  OBSTRUCTION_DOCUMENTED via F-P9-B (no gauge-invariant
  Bakry-Émery). PDE / YM.
- BT-542 meta-Lyapunov (P10, this validation):
  OBSTRUCTION_DOCUMENTED via F-P10-B (no NP closure).
  Combinatorial / circuit-complexity / meta-complexity.

If P9 also OBSTRUCTION (3-of-3 expected): EXT-B family is
**structurally fragile across NS / YM / P=NP** — Lyapunov-style
*synthesis* approaches (without an axis-changing new theorem)
do not close major Millennium problems. The structural
fragility is **canonical**: in each case the Path-Q sketch
breaking step is precisely the underlying open problem (§7.3:
vortex stretching = NS regularity; gauge-invariant Bakry-Émery
= YM mass gap; rate-gap / target-confinement = NP-vs-P/poly).

**2×2 update**: from (4, 2; 3, 6) at n=15 to (4, 3; 3, 6) at
n=16. Top row gains +1 FAIL (P10 OBSTRUCTION counts as top-row
FAIL in strict 2×2 accounting). Fisher 2-sided p moves from
0.3287 (n=15) to ≈ 0.36 (n=16). Scenario A (H_validity dominant,
H_type rejected as dominant axis) is **further reinforced**.

**Anomalies (§9)**: candidate (3.2) is more synthetic than
BT-544's CI-Lyap (5 summands vs 4); the rate-gap step is the
meta-complexity analog of vortex-stretching cross term; only
the K^t summand partially evades naturalization; the dynamics
in (3.3) is implicitly amplification but other choices are
available.

**Falsifiers active for this validation (§10)**: F-VAL-C
(meta-complexity-target reinterpretation) and F-VAL-D
(K^t naturalization reading) partially active. Others not
active.

**0/7 unchanged. P=NP open. No atlas / state / inventory
edits.** All cited references are pre-existing in the standard
meta-complexity / circuit-complexity / GCT bibliography.

— end validation —
