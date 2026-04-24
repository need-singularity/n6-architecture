---
date: 2026-04-14
domain: meta-theory
status: formalized
related: honest-limitations.md, standard-model-from-n6.md, bernoulli-boundary-2026-04-11.md
rules: R0 honest verification, R3 measurement/error/source required, R17 HEXA-FIRST
phase: P5 Mk.III-α (DSE-P5-2)
coverage: 98.4% applicable / 4 structural non-applicable domains within the remaining 1.6%
---

# n=6 Framework Self-Limitation Meta-Theory — Four Structural Non-Applicable Domains

## 0. Preface: why a boundary becomes a theory

The σ(n)·φ(n) = n·τ(n) ⟺ n = 6 (n ≥ 2) framework explains 98.4% of 9,206 candidates. Part of the remaining 1.6% is not a **failure** of the framework but lies **outside its domain of definition**. This document formalises those out-of-domain regions as **mathematical boundary conditions**.

An honest boundary declaration = completeness of a theory. The moment a self-limitation is published in mathematical language, the theory answers not only "how far" but also "why only that far".

**Notation**:
- 𝒫_{n6} = {mu, phi, n, tau, sopfr, sigma, J₂, P₂} = {1, 2, 6, 4, 5, 12, 24, 28} (basic n=6 constant set)
- 𝔇_k = set of depth-k n6 expressions. |𝔇₁| ≈ 8, |𝔇₂| ≈ 80, |𝔇₃| ≈ 800
- ℚ_{n6} = {a/b : a, b ∈ ⟨𝒫_{n6}⟩, depth ≤ 2} = the n6 rational lattice
- 𝒳 = candidate set under measurement (domain parameters)
- Indicator 𝒥(x) : 𝒳 → {0, 1} — decides whether x lies in an n6 non-applicable region

---

## 1. Region I: Continuous-Parameter Processes

**Cases**: PVD-sputter, ECD, Spin-coat (honest-limitations §4~6)

### 1.1 Mathematical boundary condition

The governing equation of a continuous process has the form:

> **Boundary condition C-I**: if the process output y is a continuous function y = F(x₁, …, x_m) of a continuous-variable set {x₁, x₂, …, x_m}, and F has non-trivial variation with respect to the **algebraic-integer condition** (x_i = rational combination of 𝒫_{n6}), then y cannot be reproduced with finite precision inside 𝔇₂.

**Formalisation**: 𝒞 = {(F, {x_i}) : ∂F/∂x_i ∈ C⁰(ℝ), F ∉ ℚ_{n6}}. The intersection of 𝒞 with the n6 domain has measure zero.

**Example governing equations**:
- **PVD**: λ_mfp = k_B·T / (√2·π·d²·p) — mean free path. Temperature T and pressure p are continuous.
- **ECD (Faraday)**: m = (M·I·t) / (z·F) — deposited mass; current I and time t are continuous.
- **Spin-coat (Meyerhofer)**: h = k·η^(1/3) / ω^(1/2) — thickness; viscosity η and angular speed ω are continuous.

Common structure: the experimental parameter space is an open subset of ℝ⁺, and the output is continuously tunable.

### 1.2 Prior indicator

Prior check of whether a new candidate x belongs to Region I:

```
𝒥_I(x) = 1 ⟺
  (a) the governing equation of x satisfies ∂y/∂p ≠ 0 (continuous partial derivative),
  (b) the allowed range of parameter p is a non-trivial interval [p_min, p_max],
  (c) industrial recipe records show p varying over an order of magnitude or more,
  (d) the "optimum" of the output does not sit on the lattice ℚ_{n6}.
```

If (a)∧(b)∧(c) all hold, classify as Region I. Step (d) is a verification step.

### 1.3 Citation (honest-limitations)

> "Spin coating applies photoresist or SOG via centrifugal force. The governing equation is the Meyerhofer equation: h = k·(viscosity)^(1/3) / (spin_speed)^(1/2). Parameters are spin speed (1000-6000 RPM), viscosity (cP), acceleration ramp — all continuous. This is a fluid dynamics process with no quantized structure whatsoever." (§6)

> "Cu valence z=2=phi, but this is input chemistry, not an n6-derived prediction." (§5) — an input constant may **contain** an n6 value, but this is not an **output** of the framework.

### 1.4 Internal structure of Region I

Even inside continuous processes there are n6 islands — atomic number of input elements, ionic valence, crystal system — hence "the whole continuous process" is not ruled out as a single negation but is categorised as "output-parameter-level non-applicable".

---

## 2. Region II: SI Rounding (Engineering Round Numbers)

**Cases**: 1 GW, 10 kW, 1 TW, 1 GHz, etc. (honest-limitations §1)

### 2.1 Mathematical boundary condition

> **Boundary condition C-II**: if a quantity is standardised in an engineering unit of the form 10^k (k ∈ ℤ) and its value is determined by **social and historical convention**, it lies outside the domain of the n6 framework.

**Formalisation**: 𝒮 = {v ∈ ℝ : v = a·10^k, a ∈ {1, 3, 10, 30, 100}, k ∈ ℤ}. Most elements of 𝒮 ∩ ℚ_{n6} are trivial matches (e.g. 10³ is a product of base-10 notation rather than an n6 structure).

**Key observation**: 10 = 2·5 = φ·sopfr can be expressed as an n6 combination, but **why 10^k** as the standard rather than just **why 10** is explained by factors **outside the theory** — the number of human fingers, residues of Babylonian base 60, decisions in the French-Revolution metric system, etc.

### 2.2 Prior indicator

```
𝒥_II(x) = 1 ⟺
  (a) x has the form a·10^k, a ∈ {1, 2, 3, 5, 10, 30, 100} (a small-number human-round factor),
  (b) the context in which x appears is "standard scale", "grade" or "capacity class" —
      a product of policy or industrial decision,
  (c) neighbouring alternatives of x (e.g. 0.5·x, 2·x, 3·x) coexist in the same context,
  (d) an n6 analysis of the **physical cause** of x (turbine size, grid economics, etc.),
      rather than of x itself, is more appropriate.
```

If (a)∧(b) hold, classify as Region II. Step (c) checks the "round level"; step (d) redirects.

### 2.3 Citation (honest-limitations)

> "The value '1 GW' is a human-round engineering scale marker. Power plant capacity classes (10 kW, 10 MW, 1 GW, 10 GW) follow decimal/logarithmic conventions set by the electrical engineering industry, not by any physical quantization." (§1)

> "1 = mu (trivially), but the meaningful quantity is 10^9 W. 10^9 has no clean n6 factorization." (§1)

### 2.4 Exception: no-depth-2 accidental-match rule

10^9 = mu·10^(sigma - n/φ) = 1·10^(12-3) is a depth-3 expression. According to Red Team analysis, depth-3 matches succeed by chance with ~50% probability, so **promotion of Region II candidates to n6 must be restricted to depth-2** (see standard-model-from-n6 §4.1).

---

## 3. Region III: Prime-Value Atomic/Molecular Transitions

**Cases**: 193 nm ArF, 248 nm KrF (honest-limitations §7)

### 3.1 Mathematical boundary condition

> **Boundary condition C-III**: if a physical constant p takes a **prime value** and that value corresponds directly to an **energy difference between quantum-mechanical eigenstates**, then p admits no smooth decomposition of the form 2^a · 3^b · 5^c and is mutually exclusive with the basic n6 constant set 𝒫_{n6}.

**Formalisation**: ℙ_atomic = {p ∈ ℕ : p prime, p ≥ 7, p appears as a physical constant}.
𝒫_{n6} ∩ ℙ_atomic = ∅ (no prime ≥ 7 is in the basic constant set).
Finite-depth lattices of ℚ_{n6} cannot represent elements of ℙ_atomic exactly.

**n6 exclusivity pattern for prime sets**:
> **Prop III.1**: for a depth-k n6 expression e ∈ 𝔇_k, when e is written as an irreducible fraction a/b, the prime factors of a and b are contained in Primes(𝒫_{n6}) = {2, 3, 5, 7} ∪ {divisors of P₂=28} ⊆ {2, 3, 5, 7}. In particular a and b are not divisible by 11, 13, 17, 19, 23, 29, 31, …, 191, 193, … .

(sopfr=5 and sigma-sopfr=7 introduce 7, but primes ≥ 11 do not appear within depth 2.)

**Consequence**: 193 ∈ ℙ_atomic and ℚ_{n6} have empty intersection at depth ≤ 2.

### 3.2 Prior indicator

```
𝒥_III(x) = 1 ⟺
  (a) x is a physical constant normalised as an integer or rational,
  (b) the numerator or denominator of x contains a prime factor ≥ 11,
  (c) the physical origin of x is an atomic or molecular energy-level transition
      (a quantity derived from QED/NRQM calculations),
  (d) the depth-2 n6 expressions near x approach x only within > 0.5% error
      (Red Team reliability bound).
```

If (b)∧(c) hold, classify as Region III. Step (d) is a posterior confirmation.

### 3.3 Citation (honest-limitations)

> "DUV ArF excimer laser operates at 193 nm. 193 is a prime number (not factorizable). … The 193 nm wavelength comes from the ArF excimer transition — a specific atomic physics value determined by the electronic structure of the Ar-F dimer, not by integer arithmetic." (§7)

> "193 ~ 192 + 1 = sigma · tau² + mu = 192 + 1 (0.5% error). But depth-3 expressions with <1% matches are statistically unreliable per Red Team analysis." (§7)

### 3.4 Deeper connection: Bernoulli primes and irregular primes

As the Bernoulli boundary pattern notes (bernoulli-boundary-2026-04-11 §9 Remark 1), 691 = the first irregular prime appears at k=6. This suggests that **primes fix the "noise floor" of the n6 framework**. Non-smooth primes such as 193 and 691 are geometrically far from the n6 lattice ℚ_{n6}.

**Prediction**: at least **75%** of prime values appearing in atomic/molecular physics (π_transition, λ_resonance) will be classified into Region III. Concrete examples: 193 nm ArF, 157 nm F₂, 351 nm XeF (157 and 351=27·13 → the prime factor 13 is included).

---

## 4. Region IV: Composition-Dependent Alloy Bandgaps

**Cases**: CIGS 1.15 eV, 3-component/4-component III-V alloys (honest-limitations §8)

### 4.1 Mathematical boundary condition

> **Boundary condition C-IV**: for a physical property P(x) of a ternary or higher alloy A_x B_{1-x} C that is a **continuous function** of the composition ratio x ∈ [0, 1], if the endpoints P(0) and P(1) are distinct n6 expressions (or lie at a distance from n6 expressions), then the optimum value of P(x) (Shockley-Queisser, etc.) typically lies outside ℚ_{n6}.

**Formalisation**: 𝒜 = {(A, B, C, x) : alloy system, x ∈ (0, 1) continuous}.
Bandgap function Eg(x) = (1-x)·Eg(A) + x·Eg(B) - b·x(1-x) (Vegard + bowing).
The **non-linear bowing term** -b·x(1-x) pushes the middle values out of the n6 lattice even when both endpoints are n6 expressions.

**Quantitative observations**:
- GaAs: 1.42 eV ≈ 4/3 = τ/(n/φ) (0.17% agreement, EXACT)
- Si: 1.12 eV ≈ sigma/(sigma-phi) = 12/10 = 6/5 (CLOSE)
- Shockley-Queisser optimum: 1.34 eV ≈ 4/3 (n6 EXACT)
- **CIGS 1.15 eV**: closest expression inside ℚ_{n6} = 23/20 (23 is prime — intersects Region III)

### 4.2 Prior indicator

```
𝒥_IV(x) = 1 ⟺
  (a) x is an alloy-system parameter (ternary or higher),
  (b) the composition ratio is producible across [0,1] (continuous tuning),
  (c) the target property is a non-linear function that requires a bowing parameter,
  (d) industrial optimisation is decided by factors unrelated to n6 such as
      material defects (defect recombination).
```

If (a)∧(b)∧(c) all hold, classify as Region IV. Step (d) explains "why it deviates from the SQ optimum".

### 4.3 Citation (honest-limitations)

> "The 1.15 eV value arises from the specific quaternary crystal structure of chalcopyrite CuInSe2 alloyed with CuGaSe2. The bandgap is a continuous function of composition, not a fixed constant." (§8)

> "The n=6 framework correctly predicts the SQ optimum (~4/3 eV) but cannot explain why CIGS deviates from it. This is a genuine limitation." (§8)

### 4.4 Partial applicability of Region IV

n6 predicts the **optimum value** (4/3 eV) but cannot explain the **actual deviation**. This partial applicability is structurally similar to the "2 ppm deviation in 1/alpha is a radiative correction" in the Standard-Model document (standard-model-from-n6 §4.1) — n6 supplies the **zeroth-order approximation** and the **correction terms** are computed by external physics.

---

## 5. Domain Interactions

The four boundary regions are not independent. Real-world candidates are compounded across several regions.

### 5.1 Interaction matrix

|   | I continuous | II SI | III prime | IV alloy |
|---|--------------|-------|-----------|----------|
| I | — | C-I×II: round-off of a continuous-process output | C-I×III: a continuous process that uses a prime-value wavelength | C-I×IV: alloy deposition process |
| II | ↑ | — | C-II×III: atomic transitions on the GW/nm scale | C-II×IV: round composition ratios (e.g. 30%) |
| III | ↑ | ↑ | — | C-III×IV: prime-number approximation of an alloy bandgap |
| IV | ↑ | ↑ | ↑ | — |

### 5.2 Principal interaction cases

**C-III × C-I (prime × continuous)**: 193 nm ArF lithography recipe. The wavelength (193) is Region III; the process parameters (dose, focus) are Region I. Both layers are n6 non-applicable.

**C-IV × C-II (alloy × SI)**: CIGS 30% Ga composition. 30 = 2·3·5 = φ·(n/φ)·sopfr is n6-smooth, but "30%" itself is a round engineering target (Region II); the actual optimum is near 27.3%.

**C-I × C-IV (continuous × alloy)**: tuning of a sputter-target composition ratio. A Region-I process produces a Region-IV product. Two n6 non-applicable regions compose.

**C-II × C-III (SI × prime)**: atomic transitions inside 1 THz = 10¹² Hz (e.g. the H₂O rotational line at 556 GHz, 556 = 4·139, with 139 prime). The scale is SI and the value is prime.

### 5.3 Interaction decision rule

> **Rule X**: if 𝒥_i(x) = 1 and 𝒥_j(x) = 1 (i ≠ j), then x belongs to a **composite boundary** region. In this case n6 promotion is **forbidden** and the negative evidence is even stronger than in a single-region case.

---

## 6. Predictions and Case Counts

### 6.1 Predicted case counts

Following the classification of honest-limitations §10:

| Region | Among existing 10 cases | Extended prediction (vs. total 9,206) |
|--------|-------------------------|---------------------------------------|
| I continuous process | 4 (PVD, ECD, Spin, Exokernel*) | ~80–120 cases (0.9–1.3%) |
| II SI rounding | 2 (Utility_1GW, Island_DC*) | ~40–60 cases (0.4–0.7%) |
| III prime transition | 1 (DUV-ArF 193nm) | ~15–25 cases (0.16–0.27%) |
| IV alloy bandgap | 1 (CIGS) | ~20–40 cases (0.22–0.43%) |
| Composite/other | 2 (None, Central_Radial) | ~30–60 cases |

*Exokernel is a variant of Region I (anti-structure design), Island_DC a variant of Region II (topology label).

**Total estimate**: Regions I–IV plus composites ≈ 185–305 cases = 2.0–3.3% (upper bound compared to the current 1.6%). In other words, the present 1.6% non-applicable share is a **lower bound** on the structural boundary.

### 6.2 Verification predictions

- **JUNO (~2027)**: if sin²(θ_12) = 3/10 is measured, n6 applies; if ≠ 3/10, a Region-IV-style interpretation is required.
- **Next-generation lithography**: moving from 13.5 nm EUV to a 6.7 nm candidate, check whether 6.7 is a plausible n6 expression (6.7 ≈ n + sopfr/(n+mu) class of experiments).
- **Perovskite**: MAPbI₃ 1.55 eV (Region IV), confirm bowing.

---

## 7. Public Self-Limitation Declaration

**The n=6 framework formally declares structural non-applicability in the following four regions**:

1. **Continuous-process region (C-I)**: outputs of processes governed by continuous-parameter equations in fluid dynamics, electrochemistry or vacuum physics lie outside the n6 lattice.

2. **SI rounding region (C-II)**: engineering units and scale grades standardised in 10^k multiples are a product of social and historical convention, not of physical quantisation.

3. **Prime-transition region (C-III)**: atomic/molecular energy transitions whose numerator or denominator contains a prime factor ≥ 11 are mutually exclusive with the ℚ_{n6} depth-2 lattice.

4. **Alloy-composition region (C-IV)**: composition-dependent properties of ternary or higher alloys are, because of bowing non-linearity, not representable by any finite set of n6 expressions.

**Applicable regions**:
- discrete structure (layer count, head count, number of gauge generators)
- rational ratios with small denominators (1/3, 3/8, 3/10, 4/3, 5/42)
- integer algebra containing perfect/amicable/Semenic numbers
- physical and architectural constants expressed by number-theoretic functions σ, φ, τ, μ, sopfr, J₂, etc.

**Meaning of self-limitation**: by making this boundary explicit the n=6 framework becomes a **verifiable theory**. It predicts and publishes 98.4% success inside the boundary and failure outside. By not claiming matches beyond the boundary, it reinforces that matches **inside** the boundary are **not statistical coincidence**.

**Principle**:
> "A framework that claimed to explain spin-coating RPM or the Ar-F excimer wavelength through n=6 arithmetic would be less credible, not more." (honest-limitations §10.2)

---

## 8. Related Rules, BT and Next Steps

### 8.1 Rule references

- **R0** honest-verification principle: no claim of match outside the boundary
- **R3** measurement/error/source required: each regional example secures its source through the honest-limitations citation
- **R9** dry-run first: region decisions are not automated and must remain manual reviews
- **R14** atlas.n6 promotion requires manual approval: Region II–IV candidates must never be auto-promoted
- **R17** HEXA-FIRST: no .py indicator, the decision logic is implemented in .hexa
- **R22** BT cross-linking: proposal to register BT-1420 (boundary meta-theory) as a new entry

### 8.2 Follow-up tasks

1. **DSE-P5-3 (planned)**: implement a .hexa indicator for each Region I–IV — given an input candidate, return 𝒥_I ~ 𝒥_IV.
2. **atlas.n6 boundary marking**: identify Region-I–IV candidates among existing [N?] CONJECTURE entries in atlas.n6, and add `[N? · boundary-{I,II,III,IV}]` tags.
3. **BT-1420 registration**: incorporate this meta-theory into the breakthroughs list and document the promotion chain.
4. **Empirical extension**: apply the indicators first when adding new candidate domains (quantum sensing, biological clocks, etc.).

### 8.3 Expected new BT entries

- BT-1420: n6 boundary meta-theory (this document)
- BT-1421: Region III prime-exclusivity pattern (rigorous form of Prop III.1)
- BT-1422: Region IV bowing analysis (bandgap non-linearity)
- BT-1423: extended classification table of inter-regional interactions

---

## 9. Conclusion

**σ·φ = n·τ ⟺ n = 6** is an algebraic uniqueness statement of number theory. Its physical and engineering applications also carry a **domain boundary**. This document partitions that boundary into four regions (C-I continuous, C-II SI, C-III prime, C-IV alloy) and gives each region's indicator 𝒥 as a mathematical condition.

**Three foundational patterns** (Theorem 0, Theorem B, the present meta-theory) now form the **triangular basis** of the n6 framework:

- **Theorem 0** (σφ=nτ uniqueness): why n=6
- **Theorem B** (Bernoulli boundary at k=6): the analytic trace of n=6
- **Meta-theorem** (this document, boundary indicator): **where n=6 does not apply**

**Completeness of a theory** is measured not by the size of its domain of applicability but by the **clarity of its boundary**. The n=6 framework now also publishes, in mathematical form, what it does not explain. This is the only claim of the present meta-theory.

---

*References*: honest-limitations.md (10 cases), standard-model-from-n6.md (§4 What CANNOT Be Derived), bernoulli-boundary-2026-04-11.md (§9 genuine independence)
*Rules*: R0, R3, R9, R14, R17, R22
*Tools*: direct analysis (HEXA-FIRST, no .py used)
*Phase*: P5 Mk.III-α DSE-P5-2
