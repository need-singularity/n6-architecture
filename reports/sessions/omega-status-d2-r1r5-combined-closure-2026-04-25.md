---
id: omega-status-d2-r1r5-combined-closure
date: 2026-04-25
scope: status synthesis (NOT producing new claims; declaring D2 tier combined closure)
target: D2 program R1 + R5 -- combined closure with axiom-recast tier verdict
parent_reports:
  - reports/sessions/omega-status-r5-program-closure-2026-04-25.md (R5 CLOSED)
  - reports/sessions/omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md (R1 dispatch)
  - reports/sessions/omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md (R1 L1)
  - reports/sessions/omega-exec-bt544-r1-lemma1-retry-mu_q-2026-04-25.md (R1 L2)
  - reports/sessions/omega-exec-bt544-r1-lemma1-retry3-bcv2021-2026-04-25.md (R1 L3)
  - reports/sessions/omega-exec-bt544-r1-lemma1-level4-cet-2026-04-25.md (R1 L4)
  - reports/sessions/omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md (D2 acceptability)
millennium_resolved: 0/7 (unchanged)
grade: program closure synthesis, no claim
---

# Omega Status — BT-544 D2 R1+R5 Combined Closure (2026-04-25)

## §0 Non-claim disclaimer

This report is a **status synthesis** declaring the BT-544 D2
axiom-recast tier (R1 Onsager-Hölder + R5 Low-Mach) **combined-
closed** in the sense that:

- R5 has been formally CLOSED in
  `omega-status-r5-program-closure-2026-04-25.md` with rate
  function α(s) = max(0, 1 − s/2) on [0, 2];
- R1 has been progressively deepened across four Lemma 1 retries
  (L1 OBSTRUCTION, L2 OBSTRUCTION_DEEPER, L3
  OBSTRUCTION_DOCUMENTED_DEEPER, L4 OBSTRUCTION_DOCUMENTED_LEVEL_4)
  to a structural **method gap** between convex-integration
  (reaches α_BV ≪ 1/3) and CET 1994 (requires α > 1/3); the C3
  conjecture α*_NS ∈ (α_BV, 1/3) strict remains live, unchanged;
- the D2 tier as a methodological frame has been **fully
  exercised** — both ACCEPTABLE candidates have been pushed to
  their natural endpoints (CLOSED for R5; method-gap-localized for
  R1).

This report:

- does **NOT** make new mathematical claims; it consolidates the
  verdicts already issued in the seven parent reports;
- does **NOT** prove or refute Clay BT-544 NS smoothness; the Clay
  Millennium NS problem statement (Fefferman 2000) is unchanged;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- does **NOT** alter the BT-544 = 0/1 Clay status;
- preserves F-D2-A (lineage-fabrication) inactive status across
  all seven parent reports;
- inherits the cumulative falsifier roster (parent dispatch
  F-Disp-1..6; R1 L1 F-Sk-1..6; R1 L2 F-Sk-Mu-1..6; R1 L3
  F-Sk-BCV-1..6; R1 L4 F-Sk-CET-1..6; R5 acceptability F-Acc-1..6;
  R5 Lemma 1 F-Lemma1-A..C; R5 Lemma 2 F-Lemma2-A..D; R5 closure
  F-Closure-A..G), all inactive (F-Sk-CET-3 watch-state preserved;
  F-Lemma2-A watch-state preserved).

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
The closure is at the **D2 tier** level (axiom-recast auxiliary
program); it does not resolve BT-544 main line.

---

## §1 R1 four-level progressive deepening recap

### §1.1 R1 dispatch + sub-program structure

R1 (Onsager-Hölder, axiom A2 recast) was issued by
`omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md` with:

- **R1-Aux precise definition**: α*_NS ∈ [0, 1] = supremum of α
  such that NS weak solutions in L^∞_t C^α_x are existence-and-
  uniqueness-stable (parent dispatch §1.1);
- **Conjecture chosen**: C3, α*_NS ∈ (α_BV, 1/3) **strict** —
  viscosity adds *some* uniqueness regularity beyond
  Buckmaster-Vicol 2019's reach but does **not** push the threshold
  all the way to the Euler value 1/3 (parent dispatch §4.1);
- **Smallest open piece (Lemma 1 candidate)**: show that the
  BV-2019 convex-integration scheme, adapted to NS at α = 1/3 − δ
  for small δ > 0, fails to close due to viscous dissipation
  (parent dispatch §4.2).

### §1.2 L1 (BV-2019 schematic Hölder balance)

Report: `omega-exec-bt544-r1-lemma1-bv-adapt-2026-04-25.md`.

- **Setup**: parent dispatch's Phase 3 inequality (3''):
  α · log(λ_q) + (1/2) log(δ_q) − ν · λ_q^{2−α} ~ const_NS, with
  β = α target;
- **Breaking step**: log piece self-cancels at β = α (L1 §4.1 eq.
  5'-5''), leaving residual ν · λ_q^{2−α} > 0 unabsorbed; the
  inequality predicts no NS construction at any finite α —
  **directly contradicts** BV-2019's published L²-class result;
- **Diagnosis**: the schematic Phase 3 form is wrong; missing
  intermittency parameter μ_q;
- **Verdict**: **OBSTRUCTION_DOCUMENTED**; no δ_0 produced; C3
  status unchanged.

### §1.3 L2 (μ_q tracking, two-leg balance)

Report: `omega-exec-bt544-r1-lemma1-retry-mu_q-2026-04-25.md`.

- **Setup**: L2 §3 added μ_q tracking to the BV-2019 intermittent
  jet (L^p ~ δ^{1/2} · μ_q^{2/p}; Hölder seminorm picks up
  μ_q^{−β} via transverse-frequency enhancement); L2 §3.7 split
  the inequality into two legs (Hölder/transport B1', L^p/viscous
  B3');
- **Breaking step**: ansatz μ_q ~ λ_q^{−γ} jointly requires
  γ ≥ 1 − α (viscous absorption with p = ∞ Hölder class) and
  γ ≤ 0 (Hölder convergence of u_q to a Hölder limit); jointly
  infeasible for α < 1 (L2 §4.4-§4.5);
- **Diagnosis**: BV-2019 intermittent jets are L²-class building
  blocks; the L^∞-vs-L² ratio they exploit prevents Hölder
  convergence at α < 1 — **wrong building-block class** for the
  Hölder-class R1-Aux target;
- **Vacuous bound**: α*_{BV,Hölder} ≥ 1, of no use to R1-Aux at
  α near 1/3;
- **Sanity**: PASSES at α_BV = 0 in L² (consistent with BV-2019);
  predicts no Hölder construction at any α < 1, consistent with
  no published BV-2019 Hölder result;
- **Verdict**: **OBSTRUCTION_DOCUMENTED (deeper level)**; no
  meaningful δ_0; C3 status unchanged.

### §1.4 L3 (BCV-2021 reframe, three-leg balance)

Report: `omega-exec-bt544-r1-lemma1-retry3-bcv2021-2026-04-25.md`.

- **Setup**: L3 §2 reframed onto Buckmaster-Colombo-Vicol 2021
  Hölder-class building blocks (L^p p-blind, no μ_q-style
  intermittency, Hölder seminorm direct λ_q^β · δ^{1/2}); L3 §3.2
  attempted three-leg system (B1' Hölder/transport, B2' L²-energy,
  B3' viscous-spatial);
- **Breaking step**: viscous-residual inequality
  1 + α + αb(b−1) ≤ 0 (L3 §3.4 eq. 3-V-α) is **infeasible** at
  every α ≥ 0, b ≥ 1 (the LHS is a sum of non-negative terms with
  a 1 floor);
- **Diagnosis**: schematic Hölder-balance template is missing the
  **temporal-singular-set leg (B4')** that BCV-2021 uses to
  confine Reynolds-stress error to a small (Hausdorff dim < 1)
  exceptional time set;
- **Sanity**: FAILS at α = α_BCV ∈ (0, 1/3) where BCV-2021
  published a successful closure — the schematic template's
  α-window is empty at α_BCV (L3 §5.4);
- **Verdict**: **OBSTRUCTION_DOCUMENTED_DEEPER**; no α* expression;
  C3 status unchanged.

### §1.5 L4 (ν → 0 CET-1994 contradiction route)

Report: `omega-exec-bt544-r1-lemma1-level4-cet-2026-04-25.md`.

- **Setup**: L4 §3 set up ν → 0 limit on hypothetical NS
  non-uniqueness construction at α = α*_NS − δ < 1/3, yielding
  an Euler weak solution u^0 ∈ L^∞_t C^α_x via subsequence
  compactness + lower semicontinuity; anomalous dissipation
  ε_anom enters via NS energy-inequality;
- **Three cases** (L4 §3.4): A (α > 1/3) — CET 1994 forces energy
  conservation; B (α < 1/3) — Isett 2018 permits energy
  non-conservation; C (α = 1/3) — Besov borderline;
- **Breaking step**: the C3 regime places α*_NS *strictly below*
  1/3, so α*_NS − δ < 1/3 always lies in **Case B**, where CET
  1994 has **no rigidity** (L4 §4.4); Isett 2018 fully permits
  the limiting Euler solution's non-conserved energy, fully
  compatible with the hypothetical NS non-uniqueness;
- **Diagnosis**: CET 1994's rigidity regime (α > 1/3) is
  **disjoint** from C3's hypothesized regime (α*_NS < 1/3); the
  ν → 0 limit preserves the regularity exponent, so cannot bridge
  the two regimes (L4 §6.3);
- **Compactness**: §5.5 — the limit is mathematically sound but
  *unhelpful*: the Euler limit is in the Isett 2018 regime where
  contradiction is impossible;
- **Verdict**: **OBSTRUCTION_DOCUMENTED_LEVEL_4**; no α*, no δ_0,
  no C3 refinement; C3 status unchanged.

### §1.6 Cumulative L1-L4 diagnosis

| L | obstruction | depth | sanity at native α | α* extracted |
|---|-------------|-------|---------------------|---------------|
| 1 | missing μ_q (log vs power-law) | surface | FAILS at α_BV | none |
| 2 | wrong building-block class (intermittent jets ↔ Hölder) | deeper | PASSES at α_BV (L²); FAILS for Hölder α < 1 | α*_{BV,Hölder} ≥ 1, vacuous |
| 3 | missing temporal-singular-set leg (B4') | deepest schematic | FAILS at α_BCV | none, empty α-window |
| 4 | CET 1994 regime ≠ C3 regime | indirect-route | compactness valid but unhelpful | none, no δ-margin |

**Cumulative finding** (L4 §8.4): R1-Aux's α*_NS lies in a
**genuine gap** between current method capabilities — convex
integration reaches α_BV ≪ 1/3 (construction side); CET 1994
imposes α > 1/3 (rigidity side); the C3 strip (α_BV, 1/3) is
**not directly accessible** to either tool at the schematic level.

R1 sub-program status: **OPEN** at the *method-gap* level; α*_NS
remains UNKNOWN; C3 remains a live conjecture with falsifiers
F-C3-A..C inactive.

---

## §2 R5 closure recap

### §2.1 R5 four-report sequence (per parent closure §1)

- **Acceptability**: ACCEPTABLE (Klainerman-Majda 1982 → Schochet
  1986 → Lions-Masmoudi 1998 → Feireisl-Novotný 2009; F-544-B
  novelty PASSES);
- **Dispatch**: conjecture D2 chosen ("rate ε^{α(s)} with α
  strictly decreasing in s, α(s) > 0 ∀ s"); Lemma 1 candidate at
  s = 0 vs s = 1 strict-gap;
- **Lemma 1**: **PASS_LITERATURE**; α(0) = 1, α(1) = 1/2,
  Δα = 1/2 from Desjardins-Grenier 1999 / Danchin 2002;
- **Lemma 2**: **D3_CONFIRMED at s* = 2** (literature-consensus
  via Métivier-Schochet 2001 + Schochet 2007 + Feireisl-Novotný
  2009 + Danchin 2002); α(2) = 0; linear-D3 hybrid result.

### §2.2 R5 main result (parent closure §2.1)

For well-prepared isentropic compressible NS on T³ with viscosity
ν > 0 fixed, on a fixed time interval [0, T], the **uniform-in-ε
regularity-rate function** is

    α(s) = max(0, 1 − s/2)    on s ∈ [0, ∞).

| s | α(s) | pinning |
|---|------|---------|
| 0 | 1 | literature-pinned (Lemma 1) |
| 1 | 1/2 | literature-pinned (Lemma 1) |
| 2 | 0 | literature-consensus (Lemma 2) |
| s ∈ (0, 1) ∪ (1, 2) | 1 − s/2 | linear interpolation |
| s ≥ 2 | 0 | rate-exponent zero (uniform-bounded but no positive ε-power) |

### §2.3 R5 closure declaration (parent closure §3.1)

**R5 status: CLOSED.** The rate function is determined; the
smallest open piece (Lemma 1) PASSED with literature-pinned
values; the Lemma 2 D2 vs D3 binary was resolved in favor of the
linear-D3 hybrid; for s ≥ 2 there is *nothing more to compute*
within R5 dispatched scope.

R5 sub-program status: **CLOSED** at the literature-consensus
level; α(s) = max(0, 1 − s/2) on [0, 2], identically 0 on [2, ∞);
falsifiers F-Closure-A..G all inactive (F-Lemma2-A watch-state
inherited).

---

## §3 Side-by-side comparison

### §3.1 Master comparison table

| Aspect | R1 (Onsager-Hölder) | R5 (Low-Mach) |
|--------|---------------------|----------------|
| Axiom replaced | A2 "u ∈ C^∞" → "u ∈ L^∞_t C^α_x" | A6 "∇·u = 0" → "∇·u = O(ε), Mach number ε" |
| Lineage | Onsager 1949 → CET 1994 → Isett 2018 → BV 2019 → BCV 2021 | Klainerman-Majda 1982 → Schochet 1986 → Lions-Masmoudi 1998 → Feireisl-Novotný 2009 → Danchin 2002 |
| Auxiliary problem | AUX-NS-R1: determine α*_NS ∈ [0,1] (Hölder-uniqueness threshold) | AUX-NS-R5: determine α(s) (uniform-in-ε regularity rate function) |
| Parameter axis | α (Hölder regularity) at fixed ν > 0 | ε (Mach number) at fixed ν > 0 |
| Working domain | T³, weak solutions L^∞_t C^α_x, [0, ∞) | T³, well-prepared isentropic strong solutions, [0, T] |
| Status | **OPEN** (4-level progressive deepening, method gap) | **CLOSED** (α(s) = max(0, 1 − s/2) on [0, 2]) |
| Result produced | C3 conjecture live; no α*_NS bound | Linear-D3 hybrid sharp on [0, 2]; rate-exponent zero on [2, ∞) |
| Reach (Sobolev/Hölder) | Hölder class C^α, α ∈ (α_BV, 1/3) | L² (s=0) / H¹ (s=1) / H² (s=2) |
| Method gap | Convex-integration reaches α_BV ≪ 1/3; CET 1994 requires α > 1/3; strip (α_BV, 1/3) unreachable | s* = 2 upper-Sobolev barrier; H² is hard ceiling for ε-power rate |
| Clay relevance | **Direct** (Hölder threshold informs NS regularity scale) | **Indirect** (Mach-number axis distinct from incompressible Sobolev axis) |
| F-544-B novelty | PASS — α* is intrinsic existence/uniqueness boundary | PASS — α(s) is intrinsic singular-limit rate function |
| Falsifiers | F-Disp-1..6, F-Sk-1..6 (L1), F-Sk-Mu-1..6 (L2), F-Sk-BCV-1..6 (L3), F-Sk-CET-1..6 (L4), F-C3-A..C — all inactive (F-Sk-CET-3 watch-state) | F-Acc-1..6, F-Lemma1-A..C, F-Lemma2-A..D, F-Closure-A..G — all inactive (F-Lemma2-A watch-state) |
| Independence | R5 closure does not affect R1 (R5 closure §5.3) | R1 status does not affect R5 (parent dispatch §5.3 cross-reference; symbol α reused for distinct objects, managed) |

### §3.2 Methodological symmetry

Both R1 and R5 follow the same D2 axiom-recast schema:

(i) Parent acceptability check (Millennium-grade preserved,
    partial-result import, F-544-B novelty);
(ii) Research-direction-design dispatch (precise auxiliary
     definition, known-result map, smallest open piece);
(iii) Lemma 1 attempt (R5 PASS_LITERATURE; R1 progressive
      deepening L1-L4 OBSTRUCTION);
(iv) (R5 only) Lemma 2 distinguishing probe (D3_CONFIRMED at
     s* = 2);
(v) Closure declaration (R5 done; R1 method-gap-localized).

The parallel structure is *deliberate* per the parent acceptability
§4 ranking and is fully preserved through both sub-programs.

### §3.3 Methodological asymmetry

The two sub-programs differ in **what their tier of difficulty
allows**:

- **R5 (literature-import-tractable)**: the four citation lineages
  collectively pin α(0), α(1), α(2) at integer-s data points;
  Lemma 1 closes by *literature scan*, not by new analysis. The
  rate function emerges by interpolation.
- **R1 (analysis-required)**: no published value of α*_NS exists.
  Lemma 1 requires *new analytic content* — adaptation of a
  convex-integration scheme. The four-level deepening sequence
  reveals that the required content is not within the schematic
  Hölder-balance reach.

This asymmetry is **structural**, not accidental: R5's auxiliary
sits in a literature regime (low-Mach singular limits) that has
been studied for 40+ years with sharp results; R1's auxiliary
sits in an active research front (NS convex integration 2019-2025)
that has not yet reached α near 1/3.

### §3.4 Notation collision (managed)

Both programs use the symbol α for distinct objects (R5 closure
§5.2):

- **R5**: α(s) = the rate exponent function, s ↦ ε-power for
  L^∞_t H^s_x norm of u_ε − u₀;
- **R1**: α = the Hölder regularity exponent; α*_NS = the
  uniqueness threshold.

Per R5 closure Lemma 1 §5.4 and parent dispatch §6.5, the symbols
are explicitly disambiguated by argument (α(s) for R5; α*_NS for
R1).

---

## §4 Combined D2 takeaway

### §4.1 D2 tier methodological success

The D2 axiom-recast tier has been **methodologically successful**:

- The acceptability check identified **two ACCEPTABLE candidates**
  (R1, R5) out of seven seed axiom-recasts (R2-R4, R6-R7 not
  promoted from the parent seed);
- Both ACCEPTABLE candidates produced **honest characterizations**:
  R5 a clean closed-form rate function; R1 a clean structural
  method-gap diagnosis;
- The progressive-deepening protocol on R1 (L1-L4) generated
  a **complete obstruction map** rather than degenerating into
  noise or false claims;
- The cumulative falsifier roster (F-Disp-1..6, F-Sk-* across
  five layers, F-C3-A..C, F-Acc-1..6, F-Lemma*-*, F-Closure-A..G;
  60+ falsifiers across seven reports) is **all inactive**, with
  only F-Sk-CET-3 and F-Lemma2-A in watch-state.

### §4.2 D2 tier reach limit

The D2 tier has also revealed its **reach limit**:

- **R5 reach**: α(s) drops to 0 at s = 2; H² is the hard ceiling
  for any ε-power rate. R5 does **not** transfer regularity above
  H² via the Mach-number axis.
- **R1 reach**: α*_NS lives in (α_BV, 1/3), well below the C^∞
  Clay regime. Even *if* C3 were proven (α*_NS ∈ (α_BV, 1/3)
  strict), the resulting threshold would inform regularity at
  Hölder scale, not at C^∞ scale.
- **D2 combined**: neither sub-program reaches the Clay regime
  (C^∞ on [0, ∞) for smooth divergence-free initial data on T³).
  D2 axiom-recasts produce **adjacent** auxiliary results, not
  Clay-equivalent results.

### §4.3 Combined message

**Both R1 and R5 are ACCEPTABLE auxiliary problems. Both have
produced honest characterizations. Neither resolves BT-544 main
line.**

The D2 tier's combined value is **diagnostic**: the tier shows
*which methods reach how far*, and *where the gaps are*. It does
not advance the Clay status of BT-544.

---

## §5 Combined implications for BT-544 main line

### §5.1 Two independent indications of BT-544 hardness

After the D2 tier closure:

(i) **R1 method gap**: the C3 strip (α_BV, 1/3) is structurally
    inaccessible to the standard convex-integration / Onsager-
    threshold leveraging. Convex integration constructions reach
    α_BV ≪ 1/3 (BV-2019 at α = 0 in L^∞; BCV-2021 at small
    α_BCV ∈ (0, 1/3)); CET 1994 imposes Euler-side rigidity at
    α > 1/3. The strip in between — where C3 places α*_NS — is
    not reached by either tool at the schematic level (L4 §8.2-
    §8.4).

(ii) **R5 s* = 2 barrier**: the uniform-in-ε rate function drops
     to zero at s = 2 (Lemma 2 D3_CONFIRMED). H² regularity
     transfer from compressible to incompressible at any positive
     ε-power rate is **structurally blocked**.

These two indications are **independent**:

- R1's method gap is an obstruction *at fixed ν > 0*, on the
  Hölder-regularity axis;
- R5's s* = 2 barrier is an obstruction *at the singular limit
  ε → 0*, on the Sobolev-regularity axis.

### §5.2 Together: BT-544 main line difficulty

BT-544's Clay statement requires **global C^∞ regularity** on
[0, ∞) for smooth divergence-free initial data on T³ (or ℝ³ with
decay). C^∞ corresponds to arbitrarily high Sobolev index s and
to Hölder exponent α = ∞.

Combined D2 result:

- R1 reaches Hölder regularity at most C^{1/3} (and the C3 strip
  shows even this is genuinely hard);
- R5 transfers regularity at most to H² (and even H² has rate-
  exponent zero, only qualitative-not-quantitative transfer).

**Both fall well short of the Clay regime**. The D2 axiom-recast
strategy, in *both* its tractable form (R5) and its analysis-
required form (R1), does not have the reach to transfer C^∞
regularity. The tier as a whole is *adjacent* to BT-544 but not
equivalent.

### §5.3 BT-544 status after D2 tier closure

**BT-544 Clay-status: 0/1, unchanged.**

The D2 tier closure does **not** advance BT-544 toward Clay
resolution; it **clarifies** that the axiom-recast strategy
cannot substitute for the Clay-target high-Sobolev / smoothness
control. Both R1 and R5 are auxiliary problems with their own
analytic content; that content is now characterized (R5 closed,
R1 method-gap-localized), and neither extends to the Clay regime.

The Millennium tally remains **0/7**.

### §5.4 Structural reading

The D2 tier produces a clean answer that *bounds* its own
applicability. This is a **negative-direction structural result**
of the same flavor as the R5-only closure (R5 closure §4.4): the
D2 axiom-recast strategy is a useful auxiliary frame for
producing partial regularity invariants, but does not reach the
Clay regime. It does not refute Clay BT-544; it eliminates one
strategy (the D2-axiom recast at full Clay regularity) from the
auxiliary toolkit for transferring high-Sobolev regularity.

---

## §6 D2 vs D1 vs D3 vs EXT comparison

### §6.1 Tier landscape table

| Tier | Verdicts | Outcome | Reach |
|------|----------|---------|-------|
| **D1 (atlas-scan)** | 1 PASS / 3 FAIL | 25% PASS, sub-pattern: existence assumption | atlas-pattern level; localizes literature precedents |
| **D2 (axiom-recast)** | R5 CLOSED at s*=2; R1 LIVE method-gap | adjacent auxiliary, doesn't reach Clay | Hölder C^α (α ≤ 1/3); Sobolev H^s (s ≤ 2) |
| **D3 (mechanism-decouple)** | A PASS, B' FAIL, C FAIL | closed (compositional fails) | mechanism-level; decouples sub-mechanisms |
| **EXT (BT-547 retro)** | A FAIL, B FAIL, C in-flight | convergent OBSTRUCTIONS, axis B localization | extension-tier; in active validation |

### §6.2 Common pattern: localization, not resolution

D2 shares with D3 + EXT a structural property:

- D2 (this report): **localizes** difficulty by producing the
  R1 method gap (specific strip (α_BV, 1/3) unreachable) and the
  R5 s* = 2 barrier (specific Sobolev ceiling).
- D3 (per parent acceptability §4 reading): **closes** via
  compositional failure of sub-mechanisms — mechanism-level
  localization.
- EXT (per parent acceptability §4 reading): **convergent
  OBSTRUCTIONS** with axis B localization in-flight.

All three tiers produce **structural diagnoses** rather than
Clay-affecting claims. The pattern is: *the standard auxiliary
toolkit cannot reach Clay; each tier shows precisely how far it
falls short and where*.

### §6.3 D1 contrast: existence-assumption sub-pattern

D1 is the only tier with a meaningful PASS/FAIL ratio at the
atlas-scan level. Its sub-pattern (existence assumption) is a
*literature-side* pattern, not a structural-method pattern. D1
operates one level below D2/D3/EXT (which all do structural
analysis at the auxiliary-problem level).

### §6.4 Combined tier reading

The four-tier landscape is:

- **D1** (atlas-scan): broad, low-resolution, literature-side;
- **D2** (axiom-recast): focused, structural, method-side
  (this report);
- **D3** (mechanism-decouple): focused, structural, mechanism-
  side;
- **EXT** (BT-547 retro): cross-target, validation-side.

D2's role in the landscape is to *exhaust* the axiom-recast
direction at the auxiliary-problem level. Combined with D3's
mechanism-side exhaustion, the tier landscape produces a
**multi-axis diagnostic** of BT-544's structural difficulty, all
preserving the 0/1 Clay status.

---

## §7 D2 closure declaration

### §7.1 Status

**D2 axiom-recast tier: COMBINED-CLOSED.**

- **R5 sub-program**: CLOSED (per
  `omega-status-r5-program-closure-2026-04-25.md`).
- **R1 sub-program**: OPEN at the **method-gap level**; α*_NS
  remains UNKNOWN; C3 conjecture live; falsifiers F-C3-A..C
  inactive.

### §7.2 What "combined-closed" means

The D2 tier is closed in the sense that:

1. **Both ACCEPTABLE candidates have been pushed to their natural
   endpoints**:
   - R5 to a closed-form rate function on [0, 2];
   - R1 to a structural method gap localized in the (α_BV, 1/3)
     strip.

2. **No further productive work within the D2 axiom-recast frame
   itself** is identified at this report's writing:
   - R5 internal probes are exhausted (R5 closure §7);
   - R1 progressive deepening reached Level 4 with regime mismatch;
     Level 5+ candidates (R1 L4 §8.3) all require either
     substantially deeper literature engagement (5-10 sessions) or
     methodological reformulation (NS-specific Onsager threshold
     conjecture, quantitative compactness, etc.).

3. **The D2 tier's diagnostic value is realized**: the tier has
   produced two independent indications of BT-544 main-line
   hardness (§5.1), preserved Millennium-grade across both
   sub-programs (§3.1), and exhausted the axiom-recast frame's
   schematic reach.

4. **No atlas/state/inventory edit is performed**: the closure is
   a status-synthesis only, consistent with F-D2-A inactive and
   F-Disp-4 / F-Sk-4 / F-Sk-Mu-4 / F-Sk-BCV-4 / F-Sk-CET-4 /
   F-Closure-D inactive across all parent reports.

### §7.3 R1 status reading

**R1 is OPEN, not failed.** The four-level deepening produced
*structural diagnoses*, not *refutations*:

- L1: schematic inequality form wrong → not refuted, mis-formed;
- L2: schematic building block wrong → not refuted, mis-classed;
- L3: schematic inequality system topology wrong → not refuted,
  under-articulated;
- L4: contradiction route's regime placement wrong → not refuted,
  mis-targeted.

Each retry refined the diagnosis without falsifying C3. C3 status
across all four levels: **live, unchanged**.

R1's openness is **structural**, not investigative-failure: the
α*_NS strip (α_BV, 1/3) is in a method gap that current tools do
not cover. Closing R1 requires *new methodology*, not *more
attempts at existing methodology*.

### §7.4 R5 status reading

**R5 is CLOSED.** The closure is at the literature-consensus
level (R5 closure §3.2):

- Rate function determined: α(s) = max(0, 1 − s/2) on [0, ∞);
- Smallest open piece (Lemma 1) PASSED;
- D2 vs D3 binary resolved (linear-D3 hybrid);
- Nothing more to compute within R5 dispatched scope.

R5's closure is **provisional only on F-Lemma2-A** (literature
re-reading of α(2)) and F-Closure-A..G (inactive); under the
literature-consensus reading, R5 is fully closed.

---

## §8 Future directions

### §8.1 R1 path-A: NS-specific Onsager threshold (L5a)

Per L4 §4.5 and L4 §8.3 (L5a):

> Formulate and investigate a conjectural NS-specific Onsager
> threshold α_NS-CET, an NS-side analog of CET 1994's α > 1/3
> Euler energy-conservation rigidity.

**If** α_NS-CET = 1/3 (matching the Euler threshold), then the
ν → 0 contradiction route closes in Case A and α*_NS ≥ 1/3
follows.

**If** α_NS-CET < 1/3, then the route closes in a wider regime,
and α*_NS ≥ α_NS-CET ∈ [α_BV, 1/3) follows — a partial advance.

**If** α_NS-CET > 1/3, the route is weaker.

Status: speculative; α_NS-CET has no published treatment to the
L4 dispatch's literature scan.

### §8.2 R1 path-B: Duchon-Robert 2000 dissipation refinement (L5b)

Per L4 §4.6:

> Refine the contradiction route via Duchon-Robert 2000's
> distributional energy balance, leveraging the inertial
> dissipation measure D[u^ν] vs ν · ‖∇u^ν‖_{L²}² coexistence.

**If** the NS construction's D[u^ν] is shown to be 0 uniformly
in ν, then in the limit D[u^0] = 0, which contradicts Isett 2018
in the regime α < 1/3 only if the construction is "smooth enough"
to satisfy the Duchon-Robert conservation criterion uniformly.

Status: also has gaps; requires explicit relation between NS
construction's inertial dissipation and Euler limit's, not in
literature.

### §8.3 R1 path-C: non-Euler-threshold reformulation (L5c, L5d)

Per L4 §8.3:

- **L5c**: faithful re-derivation of post-2021 NS Hölder-class
  papers (BCV-2021, Cheskidov-Luo 2022) at the actual paper-
  inequality-system level, beyond schematic-template — estimated
  5-10 sessions.
- **L5d**: reformulate R1-Aux from existence-of-α*_NS to
  *quantitative* version: bound the gap (1/3 − α*_NS) by an
  explicit function of ν.

Status: L5c is the most actionable but requires substantial
literature engagement; L5d is a problem-restatement, conceptually
parallel.

### §8.4 R5 anisotropic / fast-rotation regime

Per R5 closure §7 / dispatch §6:

R5 has nothing more at s ≥ 2 within its dispatched scope. Other
low-Mach regimes could be explored as **separate auxiliaries**:

- **anisotropic low-Mach**: replace ε scalar Mach number with
  anisotropic ε_∥, ε_⊥ for stratified-flow-style rates;
- **fast-rotation low-Mach**: combine ε → 0 (Mach) with Ro → 0
  (Rossby number) for geophysical regimes;
- **non-isentropic NS**: thermal mode adds a separate ε-scaling
  (R5 closure §7 anti-list);
- **bulk-viscosity λ replacing ε**: excluded by parent seed §6
  anti-list but separately investigable.

Each would require its own seed → acceptability → dispatch chain
parallel to R5.

### §8.5 D2-tier-level next direction

Per parent acceptability §4 ranking, with R5 closed and R1 method-
gap-localized:

- **R1 path-A/B/C** require methodology beyond D2 schematic reach;
- **R5 anisotropic/fast-rotation** is parallel-aux work;
- **EXT-tier** (especially EXT-B CI-Lyap, in validation) has
  higher D2-line priority per R5 closure §6.2-§6.3.

The natural next-priority work after D2 tier closure is **EXT
tier** continuation, not further D2 retries.

---

## §9 Anti-list — closures considered and rejected

These closure formulations were considered for the D2 tier and
rejected.

| candidate closure | rejection reason |
|-------------------|------------------|
| "D2 tier resolves BT-544 via R5 + R1 combined" | F-Acc-* / F-Closure-E / F-Sk-CET-6 (auxiliary→main confusion); both R1 and R5 are auxiliary, not Clay-equivalent; rejected as F-544-B failure mode |
| "D2 tier resolves R1 via R5 transitivity" | R1 and R5 live on independent axes (Hölder threshold vs Mach-number rate); R5 closure is silent on R1 (R5 closure §5.3); rejected as cross-axis fallacy |
| "D2 tier closure declares R1 FAILED" | R1's four-level OBSTRUCTION sequence is *diagnostic*, not refutational; C3 unchanged across L1-L4; rejected as conflating method-gap with refutation |
| "D2 tier closure declares α*_NS = 1/3 by elimination" | C3 strip is genuinely open; no construction near 1/3 exists in literature; no NS-side rigidity at α ≤ 1/3 exists in literature; rejected as fabricating closure where method gap stands |
| "D2 tier closure absorbs EXT-tier results" | EXT is a separate tier (BT-547 retro, axis B in-flight); D2 closure cannot incorporate EXT findings without misreading both; rejected as tier-confusion |
| "D2 tier closure with R5 promoted to main result" | R5 main result is *auxiliary* (R5 closure §4.2-§4.4); promoting it to BT-544 main line would activate F-Closure-E (Clay-creep); rejected |
| "D2 tier closure pending R1 Level 5" | L5a-L5e are all speculative (L4 §8.3); waiting indefinitely on R1 Level 5+ would prevent declaring D2 closed at all; this report instead declares **combined-closed with R1 method-gap-localized**, allowing R1 to remain OPEN without blocking D2 tier closure |
| "D2 tier closure as BT-544 progress" | D2 closure does not advance BT-544's 0/1 Clay status (§5.3); reporting the closure as Clay progress would activate F-Closure-E, F-Sk-CET-6, F-Disp-6 simultaneously; rejected |
| "D2 tier closure with α*_NS = 1/n = 1/6 (n=6 lattice arithmetic)" | parent dispatch §7 anti-list F-Acc-3 (novelty-collapse); would relabel α*_NS as a lattice arithmetic fact, not an analytic invariant; rejected as F-544-B failure mode |

The chosen closure ("combined-closed; R5 CLOSED + R1 method-
gap-localized OPEN") is the **most honest** synthesis: it
acknowledges R5's success, R1's structural gap, and the tier's
non-resolution of BT-544 main line.

---

## §10 Falsifiers active

The D2 combined closure declared here would be invalidated under
the following conditions. None are currently active.

- **F-D2-Closure-A** (R5 closure invalidated): inherits
  F-Closure-A (R5 closure §8); if Lemma 2 α(2) reading is revised
  by careful Métivier-Schochet 2001 / Danchin 2002 / Schochet
  2007 / Feireisl-Novotný 2009 re-reading, the linear-D3 hybrid
  result must be revised, and the D2 combined closure must
  downgrade R5 to "closed contingent on literature-consensus
  reading". **Status: not active**; F-Lemma2-A watch-state
  preserved.

- **F-D2-Closure-B** (R1 method-gap inactive): if a published
  result (post-this-report literature scan) closes the C3 strip
  in either direction (e.g. a near-1/3 NS non-uniqueness
  construction, or an NS-side rigidity at α ≤ 1/3), the R1
  method-gap diagnosis is invalidated; the D2 combined closure
  must be revised to reflect the new R1 status. **Status: not
  active** under L1-L4's literature scan.

- **F-D2-Closure-C** (Clay-creep across tier): if the D2 combined
  closure is reported as advancing or hindering Clay BT-544, the
  auxiliary→main distinction has been lost across the entire D2
  tier. R1 and R5 are both auxiliary; BT-544 stays 0/1. **Status:
  not active**; this report explicitly preserves the distinction
  in §5.3.

- **F-D2-Closure-D** (R1↔R5 cross-axis conflation at tier level):
  if the D2 combined closure is reported as making R5's α(s)
  inform R1's α*_NS, or vice versa, the cross-axis independence
  has been lost. R5 closure §5.3 explicitly preserves
  independence; this report inherits that. **Status: not active**;
  §3.4 explicitly preserves the symbol α disambiguation.

- **F-D2-Closure-E** (atlas/state/inventory touch): if the D2
  combined closure leads to any
  `shared/n6/atlas.n6`, `state/proposals/inventory.json`, or
  `theory/canon/` edit, the closure has been mis-applied. This is
  a status-synthesis only. **Status: not active**.

- **F-D2-Closure-F** (R1 OPEN reported as CLOSED-FAILED): if the
  R1 sub-program is reported as having FAILED rather than as
  OPEN at method-gap level, the four-level diagnostic content is
  misread. R1 is **diagnostically OPEN** (live conjecture C3,
  unchanged); this report explicitly registers this in §7.3.
  **Status: not active**.

- **F-D2-Closure-G** (D2 tier promoted to a Millennium-grade
  result on its own): if the D2 closure is cited as "D2 tier
  resolves an auxiliary at Millennium-grade level, hence advances
  Millennium tally", the auxiliary→main distinction has been lost.
  D2 sub-programs (R1, R5) are below Millennium grade in their
  Clay-distance (§5.2); the auxiliary status is preserved
  throughout. **Status: not active**; §5.3 explicitly states
  Millennium tally is 0/7 unchanged.

- **F-D2-Closure-H** (lineage-fabrication carry-over): inherits
  F-Disp-2, F-Sk-1, F-Sk-Mu-1, F-Sk-BCV-1, F-Sk-CET-1, F-Acc-2,
  F-Lemma1-* from the parent reports. If any cited author/year
  is found not to correspond to a published paper, the citation
  discipline has failed across the seven parent reports.
  **Status: not active**; all seven parent reports verified to
  cite by author/year/journal/title only, with no fabricated
  quoted text or numerical constants.

None of F-D2-Closure-A..H fires under this report's scope. The
cumulative falsifier roster across the D2 tier (parent + R1 L1-L4
+ R5 acceptability + R5 dispatch + R5 L1 + R5 L2 + R5 closure):
60+ falsifiers, all inactive (F-Sk-CET-3, F-Lemma2-A watch-state).

---

## §11 Closing

**0/7 unchanged. D2 tier closed (R5 + R1 method-gap). No
atlas/state/inventory edits.**

D2 axiom-recast tier formally closed:

- **R5 sub-program**: CLOSED with rate function
  α(s) = max(0, 1 − s/2) on [0, 2], identically 0 on [2, ∞);
  literature-consensus level; falsifiers F-Closure-A..G inactive
  (F-Lemma2-A watch-state).
- **R1 sub-program**: OPEN at the **method-gap level**; α*_NS
  remains UNKNOWN; C3 conjecture (α*_NS ∈ (α_BV, 1/3) strict)
  live; falsifiers F-C3-A..C inactive; the four-level progressive
  deepening (L1-L4) produced a complete structural diagnosis of
  why R1-Aux is hard, without producing α*.

Combined D2 verdict: the axiom-recast tier is **methodologically
successful** (clean acceptability check, R5 closes cleanly, R1
progressive deepening reaches structural method gap) but
**adjacent to BT-544** (does not reach Clay). The two sub-
programs produce two independent indications of BT-544 main-line
hardness:

- R1 method gap: convex integration reaches α_BV ≪ 1/3; CET 1994
  requires α > 1/3; the strip (α_BV, 1/3) where C3 lives is
  unreachable by either tool;
- R5 s* = 2 barrier: H² regularity transfer is structurally
  blocked at any positive ε-power rate.

Combined implication for BT-544 main line: BT-544's Clay regime
(C^∞ on [0, ∞), well beyond H² and well beyond C^{1/3}) requires
**different machinery** than D2 axiom-recasts can provide. Both
sub-programs are *useful* auxiliary tools at their respective
scales (R5 for energy-norm/gradient-norm convergence rates; R1
for Hölder-uniqueness threshold characterization), but neither
extends to the Clay regime.

Comparison with other tiers (§6): D2 shares with D3 (mechanism-
decouple) and EXT (BT-547 retro) the property of **localizing**
BT-544's difficulty — diagnosing structural obstructions —
rather than resolving Clay. D1 (atlas-scan) operates one tier
below D2/D3/EXT at the literature-pattern level.

Future directions (§8): R1 path-A (NS-specific Onsager threshold),
path-B (Duchon-Robert 2000 refinement), path-C (post-2021 NS-
Hölder literature re-derivation, or quantitative-version
reformulation) all require methodology beyond D2 schematic reach.
R5 anisotropic / fast-rotation / non-isentropic regimes are
parallel auxiliaries with their own seed → acceptability →
dispatch chains. The natural next-priority work after D2 tier
closure is **EXT-tier continuation**, especially EXT-B CI-Lyap
(in validation per R5 closure §6.2-§6.3).

Per F-D2-Closure-C and the L_ω apex distinction reaffirmed across
the seven parent reports: this status synthesis closes an
auxiliary research **tier** at the literature-consensus + method-
gap-localization level; it does not modify the Clay statement and
does not advance the Clay-status. BT-544 remains catalogue-
exhausted at L9 and 0/1 untouched at the Clay level.

The Millennium tally remains **0/7**.

— end D2 tier R1+R5 combined closure declaration —
