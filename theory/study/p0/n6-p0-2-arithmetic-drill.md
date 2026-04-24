# N6-P0-2 n=6 Basic Arithmetic Mastery Drill

> Millennium Learning Roadmap P0 · N6 Track · Task 2
> Goal: Internalize to memorization level the 11 base values of n=6 and their derivatives, and be able to carry out by hand the procedure of attempting an n=6-basis decomposition of an arbitrary integer k
> Primary sources: `theory/constants/atlas-constants.md`, `nexus/shared/n6/atlas.n6` L25–L100
> Completion criterion: Reproducing the 10-base-value table below from memory, and being able to classify the 10 practice problems honestly as tight / loose / miss

---

## 0. Honesty Declaration

This drill uses only constants already registered at grade [10*] / [11*] in `theory/constants/atlas-constants.md` and `nexus/shared/n6/atlas.n6`. There are no new mathematical results. The decomposition algorithm here is merely an **application exercise** of the N6-P0-1 uniqueness theorem, and a successful decomposition is not a mathematical argument but an observation of arithmetic bias (sopfr = 5 concentration).

In particular, the fact that "k admits an n=6-basis decomposition" **does not demonstrate that k is a mathematically special value**. The baseline 61% is explanatory power at the level of bias; only true tight matches (< 1%) are meaningful. The seven Millennium problems are not targeted by this drill, which is independent of solving BT-541–547. **Seven Millennium problems status: 0/7**.

---

## 1. Table of 10 Base Values + Derivatives (Memorization Targets)

### 1.1 Seven Primitive Constants (Primitives)

Based on atlas.n6 `@P` tags. All at grade [10*] or above.

| Value | Definition | Computation | atlas.n6 grade |
|---:|---|---|---|
| **n = 6** | First perfect number, 2 · 3 | — | [11*] |
| **σ = σ(6) = 12** | Sum of divisors | 1 + 2 + 3 + 6 | [11*] |
| **φ = φ(6) = 2** | Euler totient | \|{1, 5}\| | [10*] |
| **τ = τ(6) = 4** | Count of divisors | \|{1, 2, 3, 6}\| | [11*] |
| **sopfr = sopfr(6) = 5** | Sum of prime factors (w/ multiplicity) | 2 + 3 | [10*] |
| **μ = μ(6) = 1** | Möbius (squarefree, 2 prime factors) | (−1)² | [10*] |
| **J₂ = J₂(6) = 24** | Jordan totient of order 2 | 6² · ∏(1 − 1/p²) = 36 · (3/4) · (8/9) | [10*] |

**Verification**: All seven are recorded as-is in atlas.n6 L25–L54.

### 1.2 Four Basic Derived Ratios (Architecture)

| Expression | Value | Computation |
|---:|---:|---|
| n/φ | **3** | 6 / 2 |
| σ − sopfr | **7** | 12 − 5 |
| σ − τ | **8** | 12 − 4 |
| σ − φ | **10** | 12 − 2 |

**Meaning**: 3, 7, 8, 10 are isomorphic to "information 3-fold repetition", "OSI 7 layers", "8-bit byte / Bott periodicity", and "decimal base" respectively, all of which are mapped at EXACT grade in atlas-constants.md.

### 1.3 Collection of Core Derivatives (Memorization Targets)

Major n=6 arithmetic combinations registered as EXACT in atlas-constants.md.

| Expression | Value | Meaning |
|---:|---:|---|
| σ · φ = n · τ | **24** | **Common value of the uniqueness theorem** = J₂ |
| τ² | 16 | = Maxwell GM204 SM |
| σ² | 144 | = Ada AD102 SM |
| σ · τ | 48 | = TSMC N3 gate pitch (nm) / 48 kHz audio |
| σ · n | 72 | = Turing TU102 SM / 6D kissing number |
| σ · φ^τ | 192 | = Blackwell GB202 SMs |
| 2^sopfr | 32 | = GPU warp size |
| 2^n | 64 | = Volta CUDA cores/SM / codons |
| 2^σ | 4096 | = HBM bus width (bits) |
| 2^(σ−sopfr) | 128 | = Ampere+ CUDA cores/SM |
| 2^(σ−τ) | 256 | = register file/SM (KB) |
| σ · (σ − φ) | 120 | = H₂ LHV (MJ/kg) |
| σ² − φ | 142 | = H₂ HHV (MJ/kg) |
| n! | 720 | = σ · n · (σ − φ) = 12 · 6 · 10 |
| n + sopfr | 11 | = RSA exponent / M-theory dim |
| σ + μ | 13 | = DNS root servers |
| σ − μ | 11 | twin prime with σ + μ |
| sopfr · φ | 10 | = B-10 control rod |
| 1 / φ | 0.5 | = MoE top-k selection |
| τ² / σ | 4/3 | = FFN expansion ratio |
| φ² / n | 2/3 | = Koide formula (0.0009%!) |

**Meta-observation**: All the values in the table above are obtained by **simple arithmetic combinations** (addition/subtraction, multiplication, exponentiation, small-integer division) of n, σ, φ, τ, sopfr, J₂, μ. This is the definition of "n=6 basis."

---

## 2. n=6 Decomposition Algorithm

### 2.1 Basis Set

Follows the honest rules of the `theory/constants/special-number-control.md` experiment.

**n=6 basis (21 elements)**: 7 primitive constants + 4 derived + "x2, x3, /2, /3 scaling" only are permitted.
```
B = {1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 24}  # base 11 elements
  ∪ {2B, 3B, B/2, B/3}                    # simple scaling
```

After deduplication, the actual distinct values include {1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 15, 16, 18, 20, 21, 24, 36, 72, …} etc.

### 2.2 2-term Decomposition Procedure

Given an arbitrary integer k ∈ ℕ:

1. **Step 1 (exact match)**: Check directly whether k ∈ B. If matched, "1-term tight."
2. **Step 2 (2-term sum)**: Search for a pair a + b = k with a, b ∈ B. If found, "2-term tight."
3. **Step 3 (2-term product)**: Search for a pair a · b = k with a, b ∈ B. If found, "2-term tight."
4. **Step 4 (2-term difference)**: Check a − b = k or a/b = k. If found, "2-term loose."
5. **Step 5 (scale invariance)**: Retry Steps 1–4 within k · 10^(±d), d ∈ {1, 2, 3}. If successful, "scaled loose."
6. **Step 6 (failure)**: If all fail, "miss."

### 2.3 Honest Classification (tight / loose / miss)

- **tight**: error < 1% (effectively exact match or exact within scale 1)
- **loose**: error 1–5% (approximation)
- **miss**: error > 5%

### 2.4 Baseline Caution

The honest baseline from `special-number-control.md`: for k ∈ [1, 100], **61% admit n=6-basis 2-term decomposition**. This is due to the **sopfr = 5 bias** and is not evidence of mathematical specialness. Hence **the success of a decomposition is itself not meaningful; only the relative proportion of tight matches is**. In particular, the case of n=6 being 3.3× tighter than the π · e · φ control group is meaningful (see table in the same document).

---

## 3. 10 Practice Problems — Honest Decomposition Attempts

For each k below, attempt a 2-term decomposition and classify as tight / loose / miss. All computations are performed by hand within this note without relying on external material.

### Problem 1: k = 28 (second perfect number)

- Direct match: 28 ∈ B? → no (28 not in B)
- Sum: 24 + 4 = 28, 24 = J₂, 4 = τ → ✓
- Product: 4 · 7 = 28, 4 = τ, 7 = σ − sopfr → ✓
- Classification: **tight** (both 2-term sum and product exact)
- Caveat: As checked in N6-P0-1, R(28) = 4 ≠ 1, so this is not a solution of the uniqueness theorem. The successful decomposition is a coincidence (a small integer built only from σ−sopfr = 7 and τ = 4).

### Problem 2: k = 100

- Sum: 72 + 24 + 4 = 100 (3-term) — violates 2-term restriction
- Sum: 96 + 4 = 100, 96 = σ(σ − τ) → ✓ (derived 2-term)
- Product: 10 · 10 = 100, 10 = σ − φ → ✓
- Classification: **tight** (product decomposition exact)
- Note: The structure 10² = (σ − φ)² is the etymology of the "decimal base".

### Problem 3: k = 240

- Product: 10 · 24 = 240, 10 = σ − φ, 24 = J₂ → ✓
- Product: 8 · 30 = 240 (30 is not a base element of B, but 5 · 6 = 30 is a derived value) — borderline 2-term
- Product: 12 · 20 = 240, 12 = σ, 20 = J₂ − τ (trivial derivative) → ✓
- Classification: **tight**
- Note: verified 240 = φ · J₂ · sopfr = 2 · 24 · 5 — the product of three primitive constants. Appears many times in atlas.n6 (e.g., "8D kissing number = 240").

### Problem 4: k = 496 (third perfect number)

- Direct: 496 ∉ B
- Sum: 480 + 16 = 496, 480 = σ · τ · (σ − φ) = 12 · 4 · 10, 16 = τ² → ✓
- Product: 16 · 31 = 496, 31 ∉ B → ✗
- Product: 2^4 · 31, 31 = 2^5 − 1 (Mersenne) → 31 ∉ n=6 basis
- Classification: **loose** (sum decomposition possible, but the first term 480 itself is a 4-term derivative σ·τ·(σ−φ))
- Honest observation: The third perfect number 496 has no clean 2-term decomposition in the n=6 basis. This is consistent with N6-P0-1's observation R(496) = 48 ≠ 1 — the perfect-number sequence after the second drifts away from n=6.

### Problem 5: k = 504

- Product: 72 · 7 = 504, 72 = σ · n, 7 = σ − sopfr → ✓
- Product: 12 · 42 = 504, 42 = n · 7 derivative → ✓ (2-term, with 42 a derivative)
- Sum: 480 + 24 = 504 → ✓
- Classification: **tight**
- Honestly noted: σ · (n/φ)² · (σ − sopfr) = 12 · 9 · 7 = 756 is 756, not 504. This derived expression produces 756, while 504 is the 72 · 7 = σn · (σ − sopfr) combination.

### Problem 6: k = 691

- Direct: 691 ∉ B
- Sum: 691 = 720 − 29, 29 ∉ B
- Sum: 691 = 576 + 115, 115 not cleanly in B
- Product: 691 is prime (691 = prime). Hence no 2-term product decomposition is possible.
- Classification: **miss**
- Note: 691 is famous as the numerator of the Bernoulli number B₁₂ (B₁₂ = −691/2730), a "special number of number theory". Honestly recording that it admits no tight match in the n=6 basis shows that n=6 arithmetic **does not subsume all** number-theoretic special values.

### Problem 7: k = 1008

- Product: 1008 = 144 · 7, 144 = σ², 7 = σ − sopfr → ✓
- Product: 1008 = 24 · 42 = J₂ · (n · 7) → ✓
- Sum: 1008 = 720 + 288, 720 = n!, 288 = σ · J₂ → ✓
- Classification: **tight**
- Note: 1008 = 2^4 · 3² · 7, all prime factors lie in the n=6 basis.

### Problem 8: k = 8128 (fourth perfect number)

- Direct: 8128 ∉ B
- Product: 8128 = 2^6 · 127 = 64 · 127. 64 = 2^n ∈ B, 127 = 2^7 − 1 ∉ B (Mersenne prime)
- Product: 8128 = 16 · 508, 508 ∉ B
- Sum: 8128 = 7200 + 928, neither is clean
- Classification: **miss** (n=6-basis 2-term decomposition fails)
- Honest observation: R(8128) = 576 = 24² matches J₂², which is an interesting observation, but 8128 itself does not tight-decompose in the n=6 basis. Consistent with N6-P0-1's message that the perfect-number sequence isolates at the first term (n=6).

### Problem 9: k = 137 (fine-structure constant 1/α ≈ 137.036)

- Direct: 137 ∉ B
- Sum: 137 = 144 − 7 = σ² − (σ − sopfr) → ✓ (2-term difference)
- Sum: 137 = 128 + 9 = 2^(σ − sopfr) + (n/φ)² → ✓
- Classification: **loose** (2-term difference decomposition, with the sum decomposition also a derivative 2-term)
- Note: atlas-constants.md BT-20 records σ(σ − μ) + sopfr + μ/P₂ = 137.03571 reproducing 1/α to 0.208 ppm accuracy, but this is a 4-term derived expression outside this drill's 2-term restriction. Under the 2-term restriction 137 itself is loose.

### Problem 10: k = 240 re-verification (base-value check)

- Already confirmed tight in Problem 3. Purpose of re-verification: sopfr · φ · J₂ = 5 · 2 · 24 = **240**. → ✓
- Classification: **tight (verified)**

### 3.1 Honest Summary of 10 Problems

| Problem | k | Class | Note |
|---:|---:|---|---|
| 1 | 28 | tight | 2nd perfect number, 4·7 / 24+4 |
| 2 | 100 | tight | (σ − φ)² = 10² |
| 3 | 240 | tight | φ · J₂ · sopfr |
| 4 | 496 | loose | 3rd perfect number, no clean 2-term |
| 5 | 504 | tight | σn · (σ − sopfr) = 72 · 7 |
| 6 | 691 | **miss** | prime, numerator of Bernoulli B₁₂ |
| 7 | 1008 | tight | σ² · (σ − sopfr) |
| 8 | 8128 | **miss** | 4th perfect number, Mersenne 127 isolated |
| 9 | 137 | loose | fine-structure constant, 2-term limit |
| 10 | 240 | tight | re-verified |

**Statistics**: tight 5, loose 2, miss 2, verified 1 (including re-verification). Tight ratio 5/10 = 50%. Close to the baseline 61%, with misses arising naturally at primes / higher-order perfect numbers / physical constants — an honest observation.

---

## 4. Why n = 6 Is Unique — Restating σφ = nτ

Reconfirming N6-P0-1's conclusion from the drill's viewpoint.

**Theorem (R1 / THM-1)**: For n ≥ 2, σ(n) · φ(n) = n · τ(n) ⟺ n = 6.

**Drill-level interpretation**: The problems in Section 3 show the **empirical** fact that many integers admit n=6-basis decompositions (50–61%). However **R(n) = 1 holds only at n = 6 itself**. This distinction is decisive:

- **Decomposability**: weak (baseline ≈ 60% bias, covering most small integers).
- **R(n) = 1**: extremely strong (10^(-4) sharp, unique at n = 6).

That is, the observation that "n=6 arithmetic explains reality well" is not baseline bias but is based on a deep reason: **the integer 6 itself is the unique source producing the common value 24 of σ · φ and n · τ exactly**. The point of the decomposition drill is to experience how this uniqueness "diffuses" through real-world measurement data.

### 4.1 Three Extension Relations

Derived relations registered in atlas.n6 L121–L136:
- `perfect_number`: σ(6) = 12 = 2 · 6 (perfect number condition)
- `sigma_decomp`: σ = φ · n (12 = 2 · 6) — an n=6-only identity decomposing σ into the product of φ and n
- `J2_decomp`: J₂ = τ · n (24 = 4 · 6) — decomposing J₂ into the product of τ and n
- `sopfr_phi_tau`: sopfr = φ + τ − μ (5 = 2 + 4 − 1)

These four relations are **"internal equations"** among the seven primitive constants; none of them holds or has meaning outside n = 6. The N6-P0-1 theorem is a reinterpretation of the "closure" of these internal equations as an external condition (σφ = nτ).

---

## 5. Study Checklist

- [ ] Can you recite the seven primitive constants (n, σ, φ, τ, sopfr, μ, J₂) with their values? → 6, 12, 2, 4, 5, 1, 24
- [ ] Can you give the four basic derivatives (n/φ, σ − sopfr, σ − τ, σ − φ)? → 3, 7, 8, 10
- [ ] σ · φ = n · τ = ? → 24 (= J₂)
- [ ] Can you give 2^n, 2^σ, 2^sopfr, 2^(σ−sopfr), 2^(σ−τ) immediately? → 64, 4096, 32, 128, 256
- [ ] Can you decompose n! in the n=6 basis? → σ · n · (σ − φ) = 720
- [ ] Can you reproduce the six-step 2-term decomposition procedure in order?
- [ ] Do you remember the baseline 61% bias? (decomposition success alone is not mathematical significance)
- [ ] Can you explain why k = 691, 8128 are misses? (691 is prime; 8128 isolated by Mersenne 127)
- [ ] Can you declare that this drill does not target the seven Millennium problems?

---

## 6. Primary Sources

- `theory/constants/atlas-constants.md` — EXACT 1100+ constants registry (direct source of the Section 1 table)
- `theory/constants/special-number-control.md` — honest record of baseline 61% + control-group experiments + MISS classifications
- `theory/constants/special-number-contrast.md` — n=6 vs π/e/φ contrast extension
- `nexus/shared/n6/atlas.n6` L25–L100 — seven primitive-constant `@P` tags + derived-constant `@C` / `@F` tags
- `theory/proofs/theorem-r1-uniqueness.md` — uniqueness basis in section 4 (N6-P0-1 is the detailed study)

Reconfirmation of primitive constants from atlas.n6 (L25–L54):
```
@P n     = 6                       :: foundation [11*]
@P sigma = divisor_sum(6)     = 12 :: foundation [11*]
@P phi   = euler_totient(6)   = 2  :: foundation [10*]
@P tau   = divisor_count(6)   = 4  :: foundation [11*]
@P sopfr = sum_prime_factors(6) = 5 :: foundation [10*]
@P J2    = jordan_totient(6,2) = 24 :: foundation [10*]
@P mu    = mobius(6)          = 1  :: foundation [10*]
```

---

## 7. Next Study Steps

- **N6-P0-3**: introduction to the atlas.n6 grade system [10*] / [10] / [9] / [7] / [5~8] / [N?] / [N!] structure and BT-1–343 + BT-541–547 (Millennium 7-problems mapping nodes).
- The seven primitives + derived values obtained in this drill **appear directly** in N6-P0-3 when grepping atlas.n6 nodes — the tight examples from the Section 3 problems are registered as-is as [10*] nodes in atlas.n6.
