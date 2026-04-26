---
id: omega-meta-synthesis-extb-representation-wall-n3
date: 2026-04-25
scope: cross-BT pattern synthesis (NOT producing new claims; consolidating 3 EXT-B obstructions)
target: 3 EXT-B candidates -- representation-wall pattern n=3
parent_reports:
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md
  - reports/sessions/omega-exec-n20-p9-bt543-extb-chromo-2026-04-25.md
  - reports/sessions/omega-exec-n20-p10-bt542-extb-meta-2026-04-25.md
  - reports/sessions/omega-status-ext-tier-closure-2026-04-25.md (BT-544 EXT-tier closure)
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (EXT family origin)
millennium_resolved: 0/7 (unchanged)
grade: cross-BT synthesis, no claim
---

# Omega Meta-Synthesis -- EXT-B Representation-Wall Pattern (n=3, 2026-04-25)

## §0 Non-claim disclaimer

This file is a **cross-BT pattern synthesis**. It aggregates the
three OBSTRUCTION_DOCUMENTED verdicts produced on 2026-04-25 by the
EXT-B-class (analytic-Lyapunov / variational analytic-inequality)
candidates against three Millennium targets — BT-544 (3D
Navier-Stokes), BT-543 (Yang-Mills mass gap), BT-542 (P vs NP) —
and reads them as a single structural pattern: **EXT-B candidates
hit a representation-side wall whenever the underlying configuration
space is non-standard (path-space hybrid for NS, gauge-equivalence
quotient for YM, complexity / circuit configuration space for
P=NP)**.

This file does **not**:

- claim resolution (positive or negative) of NS regularity, YM mass
  gap, or P vs NP;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-544 / BT-543 / BT-542 = 0/1
  untouched` Clay status, or modify the L9 catalogue ledger;
- supersede any per-BT EXT-B verdict — the synthesis only
  *aggregates* what the three reports already say;
- introduce a new candidate, a new falsifier, a new theorem;
- claim that no EXT-B-style attack can ever succeed (Perelman's
  Ricci-flow W-entropy is the BT-547 proof-of-concept that the
  family CAN close in some cases; per
  `omega-exec-bt547-poincare-L9-retro-2026-04-25.md` §5.2).

The Millennium tally remains **0/7 unchanged**. No atlas / state /
inventory edits.

---

## §1 The three EXT-B candidates

### §1.1 Summary table

| BT  | discipline | candidate name | falsifier fired (primary) | source report id |
|-----|-----------|----------------|---------------------------|------------------|
| 544 | NS (PDE)  | CI-Lyap (Constantin-Iyer + Otto + Bakry-Émery synthesis) | F-EXTB-D (CI 2008 representation does not extend to single ρ on ℝ³) | `omega-exec-bt544-extb-cilyap-validation-2026-04-25.md` |
| 543 | YM (PDE)  | chromo-Lyapunov (Wilson flow + chromomagnetic energy) | F-P9-B (no gauge-invariant Bakry-Émery / CD(K,∞) on A/G) | `omega-exec-n20-p9-bt543-extb-chromo-2026-04-25.md` |
| 542 | P vs NP (combinatorial) | meta-Lyapunov (Razborov-Smolensky / KW / Hirahara / Williams / GCT synthesis) | F-P10-B (no candidate closes at NP / P/poly) plus F-P10-C (NEXP / partial-MCSP / VNP-cap reached, NP not) | `omega-exec-n20-p10-bt542-extb-meta-2026-04-25.md` |

### §1.2 Common origin in BT-547 retro

All three are instances of the **EXT-B class** prescribed in the
BT-547 Poincaré L9-retro report
(`omega-exec-bt547-poincare-L9-retro-2026-04-25.md` §5.2 "EXT-B:
analytic-Lyapunov-construction candidate class"). The class was
introduced as a remediation against the L9 catalogue's documented
CATALOGUE_BIAS (the generation pipeline's 0/3 miss rate against the
Perelman M1/M2/M3 archetype, op. cit. §3-§4). EXT-B was given
**HIGH urgency for BT-544**; BT-543 and BT-542 placements followed
in the n=20 native-pairing plan
(`omega-meta-design-n20-native-pairing-resample-2026-04-25.md` §5,
rows P1 / P9 / P10).

### §1.3 Verdict count

3/3 OBSTRUCTION_DOCUMENTED. 0/3 PASS. No FAIL.

The verdict family is **PASS-family-adjacent** (per BT-544 EXT-B
template §9.3 placement of OBSTRUCTION_DOCUMENTED in the
PASS-family-adjacent column for the bias-hypothesis 2×2 accounting),
but for closure-event accounting, 0/3 closure events occurred —
each verdict is informational, not closure.

---

## §2 Common form -- the four-step pattern

Each of the three reports follows the same four-step structure at
the discriminator level. This is not a coincidence: the EXT-B class
*is* defined as "Lyapunov-style functional with Path-P / Path-Q
discriminator on analytic inequality" (per BT-544 EXT-B candidate
spec template, op. cit. §3-§4).

### §2.1 Step 1 -- Lyapunov-style functional W defined

| BT  | functional W | structural pieces |
|-----|--------------|-------------------|
| 544 | W_NS(u, ρ, τ) = ∫_{ℝ³} [τ(ν|∇u|² + |∇log ρ|²) + log ρ] ρ dx + γνt | T_kin (viscous) + T_Fish (Fisher info) + T_ent (Boltzmann entropy) + T_corr (linear) |
| 543 | W_chromo(A) = ∫_{T³} tr(B²) + κ ∫_{T³} tr(F_{0i}²) | T_mag (chromomagnetic) + κ T_el (chromoelectric) |
| 542 | Φ_meta = α|C_n|_size + β KW_depth + γ deg_p + δ K^t + ε mult_λ_* | T_size + T_depth + T_apx + T_K + T_GCT |

In each case, W is a **synthesis** of standard-literature pieces
(none of the three Ws is a published theorem object).

### §2.2 Step 2 -- Path-Q sketch attempt

Each report attempts to derive dW/dτ ≥ 0 (or ≤ 0) by sketch using
standard tools:

| BT  | tools attempted | dynamics |
|-----|-----------------|----------|
| 544 | Constantin-Iyer 2008 + Otto 2001 + Bakry-Émery 1985 + CV 2012 | NS coupled with stochastic-Lagrangian flow |
| 543 | Lüscher 2010 Wilson flow + Bakry-Émery + chromomagnetic decomposition | Wilson flow gradient descent on action |
| 542 | KW 1990 + Razborov-Smolensky 1987 + Hirahara 2018-2022 + Williams 2014 + Mulmuley-Sohoni 2001 | hardness-amplification (XOR / Yao 1982-style) |

### §2.3 Step 3 -- cross-term / representation gap fail

In each case Path-Q breaks at a specific cross term or
representation-gap step:

| BT  | breaking step | cross-term / gap |
|-----|---------------|------------------|
| 544 | F-EXTB-D §6.4 + Path-Q §4.5 | (a) CI 2008 produces family {ρ_t(·\|x_0)} not single ρ_t on ℝ³ (representation); (b) Hess(log ρ):∇u cross-term + vortex-stretching residual (sketch) |
| 543 | F-P9-B §6.2 Step 3 | A/G has no published gauge-invariant CD(K,∞); B-E mixing cross terms uncontrolled without CD condition |
| 542 | F-P10-B §5.4 + Path-Q §5.2 rate-gap | each summand confined to monotone / AC⁰[p] / partial-MCSP / NEXP / VNP-cap; rate-gap on amplification dynamics is itself the NP-vs-P/poly question |

### §2.4 Step 4 -- the breaking step coincides with the underlying open problem

In each case, the *fail* is structurally **identical** to the
underlying Millennium open problem itself (per
`omega-exec-n20-p10-bt542-extb-meta-2026-04-25.md` §7.3):

| BT  | breaking step | underlying open problem |
|-----|---------------|-------------------------|
| 544 | vortex-stretching cross term unbounded | NS global regularity (Clay BT-544) |
| 543 | gauge-invariant CD condition unestablished | YM mass gap + reflection-positivity (Clay BT-543) |
| 542 | rate-gap = separation itself; NP target unreached | NP-vs-P/poly (Clay BT-542) |

This is the structural signature of EXT-B: **the candidate
synthesis encodes the open problem at the breaking step**. A
Lyapunov that *would* close the underlying open problem if it
worked is exactly the candidate that *fails* at the open problem's
standard obstruction.

---

## §3 Configuration-space comparison -- standardness analysis

### §3.1 The three configuration spaces

| BT  | configuration space (state) | standard? | non-standard feature |
|-----|------------------------------|-----------|----------------------|
| 544 | (u, ρ, τ) ∈ {div-free L²(ℝ³)} × {prob. densities on ℝ³} × ℝ_{>0} | NO | path-space / Eulerian-density hybrid: ρ on ℝ³ is not canonically produced from CI 2008's path-space process |
| 543 | A / G with A = SU(N) connections on T⁴, G = gauge group | NO | infinite-dim, stratified (orbits of different stabilizer types form different strata), no published gauge-invariant Riemannian metric satisfying CD(K, ∞) |
| 542 | C = {(C_n, μ_n, t)}_n: Boolean / arithmetic circuits with input distributions and amplification-level parameter | NO | discrete, combinatorial; no notion of curvature-dimension; rate-gap on amplification is itself the target separation question |

### §3.2 The standard-space template that would have worked

Bakry-Émery 1985 / Otto 2001 / Otto-Villani 2000 calculus —
the analytic input that all three Path-Q sketches rely on — is
established for:

- **(M, g, μ) Riemannian manifold with reference measure μ** under
  a curvature-dimension condition CD(K, ∞), defined by
  Γ_2(f, f) ≥ K Γ(f, f) on the carré-du-champ Γ of a *reversible*
  diffusion semigroup;
- equivalently, Wasserstein-2 gradient flow on **P_2(ℝ^n)** for
  scalar parabolic PDE (Otto 2001 PME / heat equation; AGS 2008
  catalogue).

The three EXT-B configuration spaces are **none** of these
templates:

- BT-544 is *path-space hybrid* (CI 2008 produces a measure on
  path-space, not on ℝ³); the candidate's collapse to ρ_t on ℝ³
  is unspecified (mixed-initial / pinned-label / velocity-mixed —
  three different choices, each producing a different functional);
- BT-543 is *gauge-equivalence quotient* (A / G); gauge-fixed
  Langevin (Dell'Antonio-Zwanziger 1989-91) recovers reversibility
  but breaks manifest gauge invariance — the resulting Bakry-Émery
  is on the gauge-fixed slice, not on the gauge-invariant quotient;
- BT-542 is *combinatorial / discrete*; no diffusion semigroup, no
  carré-du-champ, no curvature; the closest analog (information-
  theoretic monotonicity, Bar-Yossef et al. 2004) certifies
  communication lower bounds via direct-sum, not circuit lower
  bounds at NP.

### §3.3 Standardness verdict

In all three cases the configuration space **fails the standard
Bakry-Émery / Otto input form**:

- BT-544: no canonical measure space (P_2(ℝ³, μ_canonical)) the
  candidate functional integrates against;
- BT-543: no canonical gauge-invariant Riemannian metric on A/G
  satisfying CD(K, ∞);
- BT-542: no carré-du-champ at all — the configuration is discrete.

The three reports independently identified this: BT-544 §10.1
("choice-of-ρ ambiguity is structural"), BT-543 §10.1 ("κ choice
is structurally undetermined"), BT-542 §3.4 + §5.3 (naturalization
on summand projections).

---

## §4 Structural fragility diagnosis

### §4.1 The Bakry-Émery requirement

The Path-Q sketch in all three reports leans on Bakry-Émery 1985
Γ_2 calculus for the cross-term sign control. Bakry-Émery 1985
Theorem (per BT-544 §2.3) establishes:

**CD(K, ∞) on a reversible diffusion ⇒ logarithmic Sobolev
inequality with constant 1/(2K) ⇒ exponential decay of relative
entropy**.

The chain requires:

(i) a **reversible** diffusion semigroup;
(ii) a well-defined **carré du champ** Γ;
(iii) a **curvature-dimension condition** CD(K, ∞) for some K > 0;
(iv) a **symmetric generator** with respect to a reference measure.

### §4.2 None of the three EXT-B configurations satisfies the input form

| condition | BT-544 (NS-CI flow) | BT-543 (A/G + Wilson flow) | BT-542 (circuits + amplification) |
|-----------|---------------------|----------------------------|-------------------------------------|
| (i) reversible diffusion | NO (NS advection by time-dependent u; CI flow is transport + Brownian noise, not reversible) | NO without gauge fixing (gauge-fixed Langevin is reversible but breaks invariance) | N/A (discrete dynamics; "amplification" is not a diffusion) |
| (ii) carré du champ | partial (Γ(f) = ν\|∇f\|² for the spatial Laplacian piece, but augmented operator on (u, ρ, τ) lacks a clean Γ) | partial (L² metric on A; not gauge-invariant in the CD sense) | NO (no diffusion) |
| (iii) CD(K, ∞) | NO (per BT-544 §4.5; non-symmetric extra terms in Γ_2 are not sign-controllable by Bakry-Émery alone) | NO (per BT-543 §6.2 Step 3; no published CD(K, ∞) on A/G; Babelon-Viallet 1981 L²-metric does not satisfy CD; Donaldson 1985 restricted to 4-manifolds with self-dual splittings) | N/A |
| (iv) symmetric generator | NO (NS advection is non-symmetric in ρ-weighted inner product) | NO without gauge fixing | N/A |

The diagnosis: **the Bakry-Émery framework's input form is
violated on all four conditions across all three EXT-B
configurations**. This is not a technical-fix issue (a non-symmetric
extension via Bakry-Gentil-Ledoux 2014 §C.6 might patch (i)+(iv) on
one of the cases) — it is a **structural fragility** of the EXT-B
class as instantiated against non-standard configuration spaces.

### §4.3 The path-Q sketch reduces to the open problem

In each case, the Path-Q sketch's failure mode is *informative* —
it identifies the precise place where the synthesis would have to
be replaced by an axis-changing new theorem:

- BT-544: the missing piece is *control of the
  vortex-stretching residual* via *some quantity* — but this is the
  NS regularity question itself. Bakry-Émery cannot help because
  the augmented operator is non-symmetric and lacks CD(K, ∞).
- BT-543: the missing piece is *gauge-invariant CD(K, ∞) on A/G*
  — but no published mathematical-gauge result establishes this.
  Gauge-fixing breaks the very invariance needed; moduli-space
  results (Driver-Hall 1999) are restricted to 2D and to flat
  connections.
- BT-542: the missing piece is *rate-gap separation between
  NP-hard and P-easy languages along amplification* — but this is
  the NP-vs-P/poly separation itself. Each summand reaches its
  confined target class (monotone / AC⁰[p] / partial-MCSP / NEXP /
  VNP-cap); the union does not include NP, and the rate-gap that
  would distinguish languages at NP is the open problem.

This *reducibility-to-open-problem* pattern is documented at
BT-542 §7.3 of the source report. The synthesis here generalises
it across BT-544 and BT-543 explicitly.

### §4.4 Structural fragility, not BT-specific failure

A skeptic might argue: each of the three EXT-B failures is
BT-specific (NS-specific, YM-specific, P=NP-specific) and the
"common pattern" is post-hoc pattern matching on small n.

The structural argument against this skepticism:

1. **The breaking step is not BT-specific** — it is
   *configuration-space-specific*. The same Bakry-Émery / Otto
   framework that works on (P_2(ℝ^n), W_2) (PME, heat equation, Otto
   2001) **does not transfer** to any of the three EXT-B
   configuration spaces. The transfer obstacle is the
   non-standardness of the configuration space, not the BT.

2. **The fix would require a non-Bakry-Émery Lyapunov technique**
   uniformly across the three cases — e.g., for BT-544 a path-space
   Lyapunov (BT-544 §8.4 Direction α), for BT-543 a tool other than
   Bakry-Émery (BT-543 §11.1 F-VAL9-C), for BT-542 a path-space-on-
   circuits / partial-MCSP-target Lyapunov (BT-542 §7.5 Direction α
   / Direction β). All three require **leaving the standard
   Bakry-Émery framework**.

3. **The pattern was anticipated upfront**, not retrofitted. The
   BT-544 EXT-B candidate spec §6.3 anticipated ~75% F-EXTB-D
   firing; the BT-543 P9 candidate spec anticipated ~70% F-P9-B
   firing; the BT-542 P10 candidate spec anticipated ~70% F-P10-B
   firing. All three priors were **realised**. This is calibration
   evidence, not pattern-matching evidence.

The conclusion: EXT-B's structural fragility is **a property of
the framework's input form (Bakry-Émery requires standard measure
spaces with CD(K, ∞))**, not a property of the individual BTs. The
three Millennium open problems live on configuration spaces that
the framework *cannot describe*.

---

## §5 Connection to EXT-tier closure / 3-pillar / BKM

### §5.1 EXT-tier closure (BT-544 perspective)

Per `omega-status-ext-tier-closure-2026-04-25.md` §1.1-§1.4: the
BT-544 EXT-tier prescribed by BT-547 retro consists of four
extensions A/B/C/D, of which three (A/B/C) are validation-bearing
and one (D) is meta-level glossary. The validations:

| extension | candidate | verdict | precise breaking step |
|-----------|-----------|---------|------------------------|
| EXT-A | uω-GradFlow | OBSTRUCTION_DOCUMENTED | F-EXTA-C: convective term fails Helmholtz symmetry on Leray-projected L² (Olver 1986 ch 5) |
| EXT-B | CI-Lyap | OBSTRUCTION_DOCUMENTED | F-EXTB-D: CI 2008 produces family {ρ_t(·\|x_0)}, not single ρ_t on ℝ³ |
| EXT-C | QPC-Surgery | OBSTRUCTION_DOCUMENTED | F-EXTC-D: T1 termination on general 3D smooth data ≡ NS regularity itself |

**3/3 OBSTRUCTION on BT-544 EXT-tier**, all on **axis B
(vortex-stretching Λ²)** per the closure synthesis §3-§4. The
EXT-tier closure thus localised BT-544's obstruction to axis B.

The current cross-BT EXT-B n=3 synthesis adds a complementary
reading: **EXT-B is structurally fragile across BTs**, not just
BT-544-specific. The BT-544 EXT-tier closure showed
"EXT-A/B/C all blocked at BT-544's axis B"; this synthesis shows
"EXT-B blocked at BT-544 *and* BT-543 *and* BT-542", i.e., EXT-B's
failure is not axis-B-specific to NS — it is **framework-specific
to non-standard configuration spaces**.

### §5.2 3-pillar obstruction localization (BT-544)

Per `omega-meta-synthesis-3pillar-obstruction-localization-2026-04-25.md`
§1-§3: the BT-544 obstruction localizes to **axis B
(vortex-stretching / Λ²)** under the three pillars
(D3.A PASS_LITERATURE; EXT-A OBSTRUCTION at Helmholtz; EXT-B
OBSTRUCTION at representation + cross-term).

The current synthesis is a **generalisation** of that
localisation: where the 3-pillar work localised BT-544's obstruction
to axis B *within* BT-544, this work localises EXT-B's obstruction
to *non-standard configuration spaces* across BT-544 / BT-543 /
BT-542. The two readings are consistent and complementary:

- 3-pillar BT-544 says: "the NS obstruction is at axis B";
- EXT-B n=3 says: "the EXT-B framework breaks on non-standard
  configuration spaces, of which NS-augmented (axis-B-relevant) is
  one instance".

### §5.3 BKM connection (BT-544 + BT-543)

The Beale-Kato-Majda 1984 criterion for NS regularity is a
*cross-BT structural template* that EXT-B parallels. BKM works in
**axis-B form** (∫_0^T ‖ω‖_{L^∞} dt < ∞ ⇒ regularity), reducing the
NS regularity question to a Λ² / vortex control question. The
EXT-B CI-Lyap candidate's breaking step (vortex-stretching
residual + Hess(log ρ):∇u cross term, BT-544 §4.5) is the
**Lyapunov-form analog** of BKM's failure mode — both end at the
same axis-B obstruction.

For BT-543, the BKM-analog is reflection-positivity / mass-gap
discharge via a Wilson-flow-monotone quantity. The EXT-B
chromo-Lyapunov's breaking step (no gauge-invariant CD(K, ∞) on
A/G, BT-543 §6.2 Step 3) is the **gauge-quotient analog** of
BKM-style failure — the framework cannot reach the configuration
space the question lives on.

For BT-542, no BKM-style criterion exists in classical
combinatorial complexity. The EXT-B meta-Lyapunov hits the
**rate-gap analog** — the synthesis cannot distinguish NP-hard
from P-easy along a flow because that distinction *is* the
NP-vs-P/poly question (BT-542 §5.2 / §6.3).

---

## §6 Implications for future EXT-B-style attacks

### §6.1 H1 + H2 mitigation alone is insufficient

The L9 generation-pipeline diagnostic
(`omega-meta-audit-l9-generation-pipeline-2026-04-25.md`)
identified two structural hypotheses (H1: catalogue bias against
PASS-family-adjacent verdicts; H2: discriminator-type clustering)
and proposed mitigations including category diversification
(EXT-A/B/C/D explicit slots) and discriminator-type rebalancing.

The current synthesis suggests these mitigations are **necessary
but not sufficient**:

- **Category diversification (H1 mitigation)** correctly placed
  EXT-B in the candidate pipeline (3 candidates ran). But all 3
  produced OBSTRUCTION_DOCUMENTED on the same structural grounds.
  Category diversification gives EXT-B a fair shot but cannot
  cure framework-input-form failure.
- **Discriminator-type rebalancing (H2 mitigation)** correctly
  identified analytic-inequality as a PASS-family-adjacent
  discriminator. But the analytic-inequality discriminator
  *requires* configuration spaces compatible with the analytic
  framework. The three Millennium targets exhibit configuration
  spaces incompatible with Bakry-Émery — this is not a
  discriminator-type problem but a **framework foundation**
  problem.

### §6.2 Future EXT-B-style attacks: framework foundation review

The synthesis suggests three directions for future EXT-B-style
work, each documented in the parent reports' "follow-up
directions" sections:

#### Direction A (path-space reformulation)

Replace the EXT-B candidate's single-density-on-ℝ³ (or
single-functional-on-A/G, or single-Φ-on-circuits) with a
**path-space measure** preserving the natural Lagrangian / orbit
/ trajectory structure of the underlying dynamics. Per BT-544 §8.4
Direction α; analogous for BT-543 (Wilson-flow path-space) and
BT-542 (BT-542 §7.5 Direction α: path-space-on-circuits).

This direction abandons Bakry-Émery on the augmented Eulerian /
gauge-quotient / circuit space and works directly on path-space.
Whether path-space Lyapunov is feasible is open.

#### Direction B (target downgrade)

Replace the NP / NS-regularity / mass-gap target with a
**meta-complexity / partial-target / sector-target** where the
configuration space is more standard. Per BT-544 §8.4 Direction β
(fix the choice A/B/C explicitly); BT-542 §7.5 Direction β (target
partial-MCSP rather than full MCSP / NP); BT-543 §11.1 F-VAL9-D
(target instanton sectors with κ = ±1 self/anti-self-dual).

This direction sacrifices the Clay-target cleanness for sketch
feasibility. None of the three reports endorses this as a closure
direction; it is a *non-trivial-but-non-Clay* direction.

#### Direction C (non-Bakry-Émery Lyapunov technique)

Replace Bakry-Émery / Otto with a different analytic framework
that handles the relevant non-standard configuration space. Per
BT-543 §11.1 F-VAL9-C (lattice-spectral arguments à la
Kogut-Susskind 1975, twisted YM à la Witten 1982, Hamiltonian-
lattice spectral-gap arguments); BT-544 §11.1 F-VAL-D
(Constantin-Fefferman 1993 geometric depletion with ρ-weighting);
BT-542 §7.5 Direction γ (drop Lyapunov, return to L9 frame-shift
where Hirahara molt is PASS_LITERATURE).

This direction is the most ambitious — it requires inventing or
imported a Lyapunov framework outside the Bakry-Émery canon. It
is also the direction that, if successful, would close the
underlying open problem (since the open problem is *defined* by
the absence of such a framework).

### §6.3 Predictive update for remaining BTs

The L9 generation pipeline §F-DIAG-E noted: "Risk: low for BT-544
and BT-541; medium for BT-543". The risk reading post-validation:

| BT | EXT-B candidate status | predictive risk for EXT-B closure |
|----|-------------------------|-----------------------------------|
| 541 (Riemann) | not yet attempted as EXT-B | LOW (per pipeline §F-DIAG-E); but EXT-B-on-ξ-flow (BT-547 retro Table §5 row 541) still on the deck |
| 542 (P=NP) | OBSTRUCTION (this report) | HIGH (realised) |
| 543 (YM)  | OBSTRUCTION (this report) | HIGH (realised) |
| 544 (NS)  | OBSTRUCTION (this report) | HIGH (realised) |
| 545 (Hodge) | not yet attempted | UNKNOWN (different configuration space — algebraic / Hodge structure) |
| 546 (BSD) | not yet attempted | UNKNOWN (number-theoretic / L-function) |
| 547 (Poincaré) | CLOSED (Perelman 2002-3) | N/A (this is the proof-of-concept) |

The three EXT-B OBSTRUCTIONs do **not** predict obstructions on
BT-541 / BT-545 / BT-546: those targets have configuration spaces
that may or may not be Bakry-Émery-compatible (Riemann zeta
ξ-flow is a one-parameter dynamical system on the critical strip,
which *is* close to a standard measure-space template; Hodge / BSD
configurations are algebraic, not analytic). The synthesis predicts
that **wherever the configuration space lacks standard
Bakry-Émery structure**, EXT-B will hit a representation-side
wall; otherwise, EXT-B is competitive.

### §6.4 Generation-pipeline diagnostic update

The L9 pipeline diagnostic
(`omega-meta-audit-l9-generation-pipeline-2026-04-25.md` §6.2 +
§F-DIAG-E) anticipated EXT-B-class candidates as the
"Lyapunov-style construction" axis. The 3/3 OBSTRUCTION result
suggests the diagnostic should add a **framework-foundation
sub-check** to the pipeline:

- before running an EXT-B candidate, classify the target's
  configuration space as **standard / non-standard** w.r.t.
  Bakry-Émery;
- if non-standard, explicitly note that the candidate is
  **framework-fragile** and downgrade prior probability of
  PASS_SKETCH;
- pipeline can still run the candidate (informational verdict has
  value: it identifies the precise breaking step, which seeds
  future work), but the closure prior should reflect the
  framework-foundation fragility.

This is a **process recommendation**, not a candidate or a
closure event.

---

## §7 Verdict

### §7.1 Three options on the table

Per the brief, the three verdicts on the cross-BT pattern are:

- **STRUCTURAL_FRAGILITY_CONFIRMED**: 3/3 EXT-B all hit
  representation-side walls on non-standard configuration spaces;
  EXT-B family is structurally fragile, not BT-specific.
- **PARTIAL_PATTERN**: 3/3 OBSTRUCTION but the obstructions are
  individually distinct (representation-wall takes a different
  form in each BT); pattern is suggestive but not unified.
- **ARTIFACT**: 3 cases coincidentally produce similar-looking
  walls; small-n sample, no real pattern.

### §7.2 Verdict assessment

**STRUCTURAL_FRAGILITY_CONFIRMED.**

The reasons (aggregated from §2-§4 above):

1. **Common form**: all three reports follow the same four-step
   pattern (W defined as synthesis → Path-Q sketch → cross-term /
   representation gap → fail coincides with the open problem),
   per §2.1-§2.4.

2. **Non-standard configuration spaces in all three**: per §3.1-§3.3,
   each of the three configurations (path-space hybrid for NS,
   gauge-equivalence quotient for YM, discrete combinatorial for
   P=NP) violates the standard Bakry-Émery / Otto input form. None
   of the three is in the catalogued (P_2(ℝ^n), W_2) gradient-flow
   archetype.

3. **Bakry-Émery input form violated on all four conditions**:
   per §4.1-§4.2 table, reversibility / carré-du-champ / CD(K, ∞) /
   symmetric-generator all fail or are absent in all three cases.
   This is a **framework-input-form** failure, not a technicality.

4. **Calibration evidence**: per §4.4 point 3, the three candidate
   spec priors (75% / 70% / 70%) for OBSTRUCTION were each
   realised. This is not pattern-matching evidence but
   anticipatory-prior evidence — the pattern was predicted upfront
   and then realised, three times independently.

5. **Reduction to open problem in all three**: per §2.4 and §4.3,
   each Path-Q sketch fails at *exactly* the underlying Millennium
   open problem. This is the canonical structural-fragility
   signature of EXT-B.

The verdict is *not* PARTIAL_PATTERN because the obstructions
*are* unified at the framework level (standard Bakry-Émery input
form fails uniformly across all three). The differences in
*surface form* (CI 2008 representation gap vs gauge-invariant CD
absence vs rate-gap on amplification) are different *symptoms* of
the same root cause: **non-standard configuration space →
Bakry-Émery framework cannot describe the dynamics**.

The verdict is *not* ARTIFACT because the pattern was
*anticipated*, *not* discovered post-hoc; the structural argument
(§4) is independent of the numerical pattern (§3).

### §7.3 Verdict scope and non-claim repeat

This STRUCTURAL_FRAGILITY_CONFIRMED verdict speaks **only** to the
EXT-B family's reliance on Bakry-Émery / Otto-style frameworks
applied to non-standard configuration spaces. It is **NOT**:

- a claim that no Lyapunov ever closes any Millennium problem
  (Perelman's W-entropy closed BT-547 Poincaré per
  `omega-exec-bt547-poincare-L9-retro-2026-04-25.md`);
- a claim that BT-541 / BT-545 / BT-546 are also EXT-B-fragile
  (their configuration spaces are different and may admit
  standard-framework reformulations);
- a claim that the 3/3 OBSTRUCTION across BT-544 / BT-543 / BT-542
  was inevitable (different EXT-B candidate choices may have
  produced different verdicts; the synthesis only addresses the
  three specific candidates run);
- a closure event, a tally update, a Clay-status modification, or
  an atlas / state / inventory edit.

---

## §8 Anti-list -- alternative interpretations

### §8.1 Alternative A: 3 cases is too small

**Statement**: n = 3 is statistically insufficient to claim a
structural pattern; the apparent uniformity could be a small-n
fluke.

**Response**: per §4.4 point 3, the pattern was anticipated
*before* the validations ran, with three independent priors
(75% / 70% / 70%). Anticipated + realised pattern is structurally
informative regardless of n. The verdict is conditional on the
*structural* argument (§4), not on n.

**Status**: this alternative is partially active in the sense that
n=4 (BT-541, BT-545, or BT-546 EXT-B) would sharpen the pattern;
but it is not active as an objection to the current verdict.

### §8.2 Alternative B: surface obstructions are different

**Statement**: BT-544 fires on F-EXTB-D (CI 2008 representation),
BT-543 fires on F-P9-B (gauge-invariant Bakry-Émery), BT-542 fires
on F-P10-B (no NP closure). These are different falsifiers, not a
unified obstruction.

**Response**: per §3.3 + §4.2, all three falsifiers reduce to the
same root cause — non-standard configuration space →
Bakry-Émery / Otto input form violated. The surface differences
(which falsifier fires, which step fails) are different
*symptoms*; the framework-input-form failure is the
*disease*. Per BT-547 retro §3-§4 catalogue terminology, this is
the same "frame-bias" diagnosis at the framework level, not at the
BT level.

**Status**: this alternative is rejected — the structural argument
(§4) explicitly unifies the three surface falsifiers under one
framework-input-form failure.

### §8.3 Alternative C: EXT-B is not "fragile" — it correctly
identifies hard problems

**Statement**: the 3/3 OBSTRUCTION is not a fragility of EXT-B
but a property of the targets. EXT-B's job is to expose the
underlying open problem at the breaking step; it does this
correctly. "Fragility" is a misleading framing.

**Response**: this is partially valid. Per BT-547 retro §3-§4 the
EXT-tier was designed for *informational* outputs (precise
breaking step identification), not closure events. The
3/3 OBSTRUCTION is the expected deliverable of the EXT-B class —
identification of the breaking step. Calling this "fragility" is
indeed framework-loaded; a more neutral framing would be "EXT-B
delivers informational obstructions on non-standard configuration
spaces; these obstructions are not closure events but they do
seed follow-up directions" (per §6.2 Directions A/B/C).

**Status**: this alternative is partially active. The
"fragility" terminology in the synthesis is shorthand for "EXT-B
in its standard Bakry-Émery instantiation cannot close on
non-standard configurations". A future synthesis might prefer
"EXT-B-standard is informational on non-standard configurations".
The current report retains "fragility" for consistency with the
brief, but flags the framing issue.

### §8.4 Alternative D: a future EXT-B candidate may break the pattern

**Statement**: future EXT-B candidates with different functional
forms (e.g., path-space Lyapunov, non-Bakry-Émery technique) may
close one of the three. The pattern is conditional on the specific
synthesis attempts run on 2026-04-25.

**Response**: this is the explicit recognition in §6.2 Directions
A/B/C and in §6.3. The pattern is conditional on
*standard-Bakry-Émery EXT-B*; non-standard EXT-B candidates are
not addressed by the current verdict. This alternative is *fully
active* as a recognition of the verdict's scope, not as an
objection.

**Status**: this alternative is **active and acknowledged** —
the verdict explicitly addresses standard-Bakry-Émery EXT-B and
flags non-standard variants as future work.

---

## §9 Falsifiers active for this synthesis

### F-SYN-A (one of the three Path-Q breaks is fixable by a missed tool)

**Statement**: if a published paper exists that controls one of
the three Path-Q breaking steps via a specific known tool —
e.g., a Bakry-Gentil-Ledoux 2014 §C.6 non-symmetric-CD result that
applies to the NS-driven CI flow, or a gauge-invariant CD result
on A/G obscure in mathematical-gauge literature, or a
meta-complexity Lyapunov in a niche complexity sub-literature —
then one of the three OBSTRUCTIONs softens and the n=3 pattern
becomes n=2.

**Status**: PARTIALLY ACTIVE. Each parent report flags 1-2
"specialist-re-reading" risks (BT-544 F-VAL-B / F-VAL-E; BT-543
F-VAL9-B / F-VAL9-D; BT-542's literature search is current as of
2026-04 but cannot preclude a missed paper). Cumulative risk of at
least one fix is non-trivial; current synthesis assumes none of
the three softens.

### F-SYN-B (a fourth EXT-B candidate produces PASS_SKETCH)

**Statement**: if BT-541 or BT-545 or BT-546 EXT-B produces
PASS_SKETCH or PASS_LITERATURE, the n=3 OBSTRUCTION pattern is
broken at n=4 and the structural-fragility verdict softens to
"BT-544/543/542-specific OBSTRUCTION cluster, not EXT-B-wide".

**Status**: NOT YET TESTED. BT-541 EXT-B (Lyapunov on ξ-flow) is
on the BT-547 retro deck (§5 Table row 541) but not yet executed.
BT-545 / BT-546 EXT-B are not on any current execution plan. The
risk that one of these produces PASS is material; the synthesis
explicitly limits its claim to the n=3 sample (§7.3).

### F-SYN-C (the framework-foundation diagnosis is wrong)

**Statement**: the §4 diagnosis attributes failure to
Bakry-Émery input-form violation. If the actual cause of failure
is BT-specific (e.g., NS specifically, YM specifically, P=NP
specifically) rather than framework-specific, then the
"structural fragility" framing is misleading.

**Status**: PARTIALLY ACTIVE per §8.2 Alternative B. The
synthesis argues the framework-input-form failure is the unified
cause; but a critic could argue each BT has BT-specific reasons
for OBSTRUCTION that happen to reduce to the same surface form.
The §4 argument is structural; this falsifier is rejected by the
synthesis but flagged as a possible objection.

### F-SYN-D (atlas/state/inventory edit leakage)

**Statement**: if any change is made to atlas, state, or
inventory files as a result of this synthesis, the brief's hard
constraint is violated.

**Status**: NOT ACTIVE. This synthesis is research-only and edits
no atlas, state, or inventory file. The git status at session
start (specs and inventory.json modified by *unrelated* prior
sessions per the gitStatus header) is unaffected.

### F-SYN-E (verdict overreach beyond scope)

**Statement**: if the synthesis claims more than "3 specific
EXT-B candidates produced OBSTRUCTION via a unified
framework-input-form failure" — e.g., if it claims "EXT-B can
never close any Millennium problem" or "Bakry-Émery is uniformly
inapplicable to PDE Lyapunov questions" — then the verdict
overreaches.

**Status**: NOT ACTIVE based on §7.3 explicit scope statement.
The synthesis explicitly disclaims:
(i) claims about future EXT-B candidates;
(ii) claims about BT-541 / BT-545 / BT-546;
(iii) claims about Lyapunov closure in general (Perelman BT-547
counter-example explicitly noted).

### Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-SYN-A | one Path-Q break fixable by missed tool | PARTIALLY ACTIVE |
| F-SYN-B | fourth EXT-B candidate PASSes | NOT YET TESTED |
| F-SYN-C | framework-foundation diagnosis wrong | PARTIALLY ACTIVE |
| F-SYN-D | atlas / state / inventory edit leakage | NOT ACTIVE |
| F-SYN-E | verdict overreach | NOT ACTIVE |

Two falsifiers (F-SYN-A, F-SYN-C) are partially active —
re-checking them would require specialist re-reading of
Bakry-Gentil-Ledoux 2014 §C.6, mathematical-gauge literature on
A/G CD conditions, and meta-complexity sub-literatures. Neither is
*expected* to overturn the verdict, but both are flagged as
structural risks.

One falsifier (F-SYN-B) awaits future executions on BT-541 /
BT-545 / BT-546 EXT-B and is the canonical sharpening test for the
n=3 → n=4+ pattern.

---

## §10 Closing

**Verdict**: **STRUCTURAL_FRAGILITY_CONFIRMED.**

3/3 EXT-B candidates (BT-544 CI-Lyap, BT-543 chromo-Lyapunov,
BT-542 meta-Lyapunov) produced OBSTRUCTION_DOCUMENTED on
2026-04-25, all three at representation-side walls on non-standard
configuration spaces (path-space hybrid for NS, gauge-equivalence
quotient for YM, discrete combinatorial for P=NP). The shared
framework-input-form failure is **Bakry-Émery / Otto requires a
standard measure space with a curvature-dimension condition
CD(K, ∞) on a reversible diffusion semigroup with symmetric
generator; none of the three EXT-B configurations satisfies any of
the four input-form conditions**. The breaking step in each Path-Q
sketch coincides with the underlying Millennium open problem
itself, which is the canonical structural-fragility signature.

**Common form (4-step)**: synthesis-W defined → Path-Q sketch
attempt → cross-term / representation-gap fail → fail = open
problem.

**Configuration-space comparison**: all three non-standard;
none in the catalogued (P_2(ℝ^n), W_2) gradient-flow archetype.

**Structural fragility diagnosis**: the standard
Bakry-Émery / Otto framework is not transferable to the three
EXT-B configurations; future EXT-B-style attacks need either
path-space reformulation (Direction A), target downgrade
(Direction B), or non-Bakry-Émery Lyapunov technique
(Direction C). The L9 generation-pipeline H1+H2 mitigations
(category diversification, discriminator-type rebalancing) are
necessary but not sufficient — framework-foundation review is also
required.

**Connections**: complementary to BT-544 EXT-tier closure (3/3
OBSTRUCTION on axis B, BT-544-specific) and to 3-pillar
obstruction localization (BT-544 obstruction localised to axis B);
parallel to BKM cross-BT structural template (axis-B for NS,
reflection-positivity for YM, rate-gap for P=NP).

**Anti-list responses (§8)**: alternatives A (small n), B
(surface differences), C (fragility framing), D (future
candidates) addressed. Alternative D explicitly active and
acknowledged in §6.2 / §6.3 / §7.3.

**Falsifiers active (§9)**: F-SYN-A and F-SYN-C partially active
(specialist re-reading risks); F-SYN-B awaits BT-541 / BT-545 /
BT-546 EXT-B executions; F-SYN-D and F-SYN-E not active.

**0/7 unchanged. NS / YM / P=NP regularity / mass-gap / separation
status open. No atlas / state / inventory edits.** All cited
content is by per-BT report id; no new theorems, no new
citations beyond what the parent reports establish.

— end synthesis —
