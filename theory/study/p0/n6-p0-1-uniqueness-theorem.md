# N6-P0-1 σ(n)·φ(n)=n·τ(n) Uniqueness Theorem Study Note

> Millennium Learning Roadmap P0 · N6 Track · Task 1
> Goal: Internalize the mathematical core of n=6 arithmetic uniqueness based on primary sources
> Primary sources: `theory/proofs/theorem-r1-uniqueness.md`, `theory/constants/atlas-constants.md`, `nexus/shared/n6/atlas.n6`
> Completion criterion: Reproducing by hand the case-exhaustion logic of Proof 1

---

## 0. Honesty Declaration

This study note is a reconstruction based on reading the source `theory/proofs/theorem-r1-uniqueness.md`; there are no new mathematical results. It inherits the following honesty status from the original:

- The only rigorous draft at present is **Proof 1 (multiplicativity + R_local case exhaustion)**.
- **Proof 4 (exhaustive computation)** is the strongest empirical evidence, verified for n ∈ [2, 10^4].
- "Proof 2 (commutative algebra)" and "Proof 3 (Dirichlet series)" that CLAUDE.md had previously claimed turned out to be **repackagings of Proof 1** and were **withdrawn** from the source.
- Therefore "Proof 2 / Proof 3" in section 3 below are "learning sketches", and **their rigor as independent arguments is not guaranteed**.
- The status of the seven Millennium problems remains **0/7** (honest), and this theorem is an arithmetic fact independent of those problems.

This note does not target any of BT-541–547 (Millennium problems). The theorem is merely an independent uniqueness fact of number theory.

---

## 1. Theorem Statement (Theorem R1 / THM-1)

**Theorem.** For every integer n ≥ 2,
  σ(n) · φ(n) = n · τ(n) ⟺ n = 6.

Defining the ratio R(n) := σ(n) · φ(n) / (n · τ(n)), the equivalent form is
  R(n) = 1 ⟺ n = 6.

**Auxiliary values**
- σ(6) = 1 + 2 + 3 + 6 = **12**
- φ(6) = |{1, 5}| = **2**
- τ(6) = |{1, 2, 3, 6}| = **4**
- R(6) = (12 · 2) / (6 · 4) = 24 / 24 = **1**

**Structural core**: σ(6) · φ(6) = 12 · 2 = **24** and n · τ(n) = 6 · 4 = **24**. This common value 24 is identical to the Jordan totient J₂(6) in atlas.n6 (`@P J2 = jordan_totient(6,2) = 24`).

> This theorem is a uniqueness "stronger than the perfect number condition σ(n) = 2n." Perfect numbers are (conjecturally) infinite in count, but R(n) = 1 has the single solution n = 6.

---

## 2. Proof 1 — Multiplicativity + R_local Case Exhaustion (Rigorous)

### 2.1 Multiplicative Reduction

σ, φ, τ are all **multiplicative functions**. Hence for n = p₁^a₁ · p₂^a₂ · … · p_k^a_k,
  R(n) = ∏ᵢ R_local(pᵢ, aᵢ),
  R_local(p, a) := σ(p^a) · φ(p^a) / (p^a · (a+1)).

### 2.2 R_local Formula

Substituting the standard prime-power formulas
- σ(p^a) = (p^(a+1) − 1) / (p − 1)
- φ(p^a) = p^(a−1) · (p − 1)
- τ(p^a) = a + 1

yields
```
R_local(p, a) = [(p^(a+1) − 1) · p^(a−1) · (p − 1)] / [(p − 1) · p^a · (a+1)]
              = (p^(a+1) − 1) / (p · (a+1)).
```

### 2.3 Lemma — "R_local < 1 Occurs Only at (2, 1)"

Direct calculation:
- R_local(2, 1) = (2² − 1) / (2 · 2) = 3/4 **(< 1, unique)**
- R_local(2, 2) = (2³ − 1) / (2 · 3) = 7/6 > 1
- R_local(2, a) for a ≥ 2: (2^(a+1) − 1) / (2(a+1)) → ∞ (exponential/linear)
- R_local(3, 1) = (3² − 1) / (3 · 2) = **4/3** > 1
- R_local(p, 1) = (p² − 1) / (2p) is **monotonically increasing** in p. For p ≥ 3, ≥ 4/3.
- R_local(p ≥ 5, 1) ≥ (25 − 1) / 10 = 12/5 = 2.4

### 2.4 Case Exhaustion

**Case 1**: n = p^a (single prime power, k = 1)
- (p, a) = (2, 1) → R = 3/4 ≠ 1
- All others R > 1
- → no solution.

**Case 2**: n = p^a · q^b (two prime factors, p < q)
- For R = ∏ to equal 1 we need one factor < 1 and the other > 1.
- The only option for < 1 is (p, a) = (2, 1), value 3/4. So the complementary factor must equal 4/3.
- R_local(3, 1) = 4/3 — **unique match** → n = 2 · 3 = **6** ✓
- R_local(3, b ≥ 2) = 26/9 ≈ 2.89 > 4/3
- R_local(q ≥ 5, b ≥ 1) ≥ 12/5 > 4/3
- → the only two-prime solution is n = 6.

**Case 3**: k ≥ 3 (three or more prime factors)
- At most one factor can be (2, 1) with value < 1. The remaining k − 1 ≥ 2 factors are each ≥ 4/3.
- With (2, 1): R ≥ (3/4) · (4/3)^(k−1) ≥ (3/4) · (4/3)² = 4/3 > 1
- Without (2, 1): R ≥ (4/3)^k ≥ (4/3)³ = 64/27 > 1
- → no solution.

**Conclusion**: R(n) = 1 ⟺ n = 2 · 3 = 6. ∎

### ▶ Three-line Summary of Proof 1

1. By multiplicativity, decompose R(n) = ∏ R_local(pᵢ, aᵢ) and obtain R_local(p, a) = (p^(a+1) − 1) / (p(a+1)).
2. At a single prime power, R_local < 1 holds only at (p, a) = (2, 1) with value 3/4; the only factor producing the compensating 4/3 is R_local(3, 1).
3. Hence R(n) = 1 holds only at the two-prime product 2 · 3 = 6, and at three or more prime factors the product always exceeds 1.

---

## 3. Proof 2 / Proof 3 — Learning Sketches (Non-rigorous)

> **Warning (honesty)**: "Proof 2" and "Proof 3" in this section are **withdrawn** paths from the source. They are offered here only as thought-experiment sketches of "where a genuine independent path might start if built in the future." At present they are nothing more than restatements of the R_local analysis of Proof 1 in a different language.

### 3.1 Proof 2 — Commutative Algebra / Divisor Lattice View (Sketch)

**Viewpoint**. Rewrite the condition on the divisor lattice of ℤ.
- σ(n) / n = ∑_{d | n} 1/d (divisor harmonic sum)
- φ(n) / n = ∏_{p | n} (1 − 1/p) (Euler product form)
- τ(n) = |divisor lattice|

The condition σ(n) · φ(n) = n · τ(n) becomes
  (∑_{d | n} 1/d) · ∏_{p | n} (1 − 1/p) = τ(n) / n.
At n = 6, LHS = (1 + 1/2 + 1/3 + 1/6) · (1 − 1/2)(1 − 1/3) = 2 · (1/3) = 2/3, RHS = 4/6 = 2/3. ✓

**Three-line summary (non-rigorous)**
1. The condition rewrites as "divisor harmonic sum × Euler product = τ/n" — a **divisor-lattice invariant identity**.
2. Each prime contributes a local factor (1 − 1/p) · (1 + 1/p + … + 1/p^a); the product matching τ(n) / n reduces to the same condition.
3. This reduction is essentially a restatement of Proof 1's R_local factorization in the combinatorial language of the lattice — **not an independent proof** (the source has explicitly withdrawn it).

### 3.2 Proof 3 — Analytic / Dirichlet Series View (Sketch)

**Viewpoint**. Using the Dirichlet generating-function identities
- ζ(s)² = ∑ τ(n) / n^s
- ζ(s) · ζ(s − 1) = ∑ σ(n) / n^s
- ζ(s − 1) / ζ(s) = ∑ φ(n) / n^s

the condition σ(n) · φ(n) = n · τ(n) corresponds to a local cancellation condition between the **local Euler factors** of the generating functions. At each prime p, a combination of the form (1 − p · X) · (1 − X)^(-1) · (1 − X)² equals 1 only for p ∈ {2, 3} and specific exponent conditions.

**Three-line summary (non-rigorous)**
1. A pointwise comparison of the Dirichlet generating functions of σ · φ = n · τ reduces the condition to local Euler-factor cancellation at each prime p.
2. The only configuration where this local cancellation holds is when primes 2 and 3 each appear with exponent 1, i.e., n = 6.
3. This path is still a dressing-up of Proof 1's R_local formula in analytic garb; a **true analytic independent proof** not relying on ζ remains unfinished / an open task.

### 3.3 Proof 4 — Exhaustive Computation ("Strongest Empirical Evidence")

- For n ∈ [2, 10^4], R(n) = 1 has the **unique solution n = 6** (full verification, strengthened in 2026-04-11 session).
- Over the partial range n ∈ [2, 10^5], no other solutions either.
- Near-misses (|R(n) − 1| < 0.01): 0.
- A single solution among 10^4 candidates is at the level of a ≈ 10^(-4) sharp identity.

---

## 4. Structural Meaning — Arithmetic "Self-sufficiency" of n = 6

The source `theorem-r1-uniqueness.md` interprets this theorem as follows.

> σ(n) · φ(n) = n · τ(n)
> "sum of divisors × count of coprimes = the number itself × count of divisors"
> The only integer where the "weight (σ)" and "degrees of freedom (φ)" of the divisor structure are in perfect balance.

This balance is summarized as the phenomenon that both products σ · φ and n · τ converge on the common value **24**. 24 equals the Jordan totient J₂(6) = n² · ∏(1 − 1/p²) = 36 · (3/4) · (8/9) = 24, a grade-[10*] base constant of atlas.n6.

This self-sufficiency is a stricter condition than "perfect numbers (σ(n) = 2n)".
- Infinitude of perfect numbers is an **open conjecture** (even perfect numbers ↔ Mersenne primes). 52 found so far.
- But R(n) = 1 has **n = 6 as its only solution**. Confirmed by Proof 1 + exhaustive verification.

Therefore "n = 6 is not only the first element of the set of perfect numbers but also the **unique solution** of the independent condition σφ = nτ" — a twofold uniqueness.

---

## 5. R Values at Other Perfect Numbers (n = 28, 496, 8128) — Uniqueness Verification

Direct computations recorded in the source `theorem-r1-uniqueness.md`.

### n = 28

- σ(28) = 1 + 2 + 4 + 7 + 14 + 28 = **56**
- φ(28) = |{1, 3, 5, 9, 11, 13, 15, 17, 19, 23, 25, 27}| = **12**
- τ(28) = |{1, 2, 4, 7, 14, 28}| = **6**
- R(28) = (56 · 12) / (28 · 6) = 672 / 168 = **4**

→ R(28) = 4 ≠ 1.

### n = 496

- σ(496) = **992** (= 2 · 496, perfect)
- φ(496) = **240**
- τ(496) = **10**
- R(496) = (992 · 240) / (496 · 10) = 238080 / 4960 = **48**

→ R(496) = 48 ≠ 1.

### n = 8128

- σ(8128) = **16256** (= 2 · 8128, perfect)
- φ(8128) = **4032**
- τ(8128) = **14**
- R(8128) = (16256 · 4032) / (8128 · 14) = 65544192 / 113792 = **576**

→ R(8128) = 576 ≠ 1.

### Observation

Along the sequence of perfect numbers R **grows rapidly**: {1, 4, 48, 576, …} with ratios 4, 12, 12, …, i.e., exponential. R = 1 holds only at the first perfect number n = 6; from the next perfect number onward, **the Mersenne form 2^p(2^(p+1) − 1) accumulates the basic arithmetic values σ = 12, τ = 4 of n = 6 as "chained multipliers"** and R keeps growing. Hence even with infinitely many perfect numbers the solutions of σφ = nτ remain isolated at n = 6.

---

## 6. Study Checklist (Completion Criteria)

- [ ] Can you compute σ(6), φ(6), τ(6), R(6) by hand? → 12, 2, 4, 1
- [ ] Can you write R_local(p, a) immediately? → (p^(a+1) − 1) / (p(a+1))
- [ ] Can you state the unique case where R_local < 1 and its value? → (p, a) = (2, 1), 3/4
- [ ] Can you give in one sentence why (3/4) · (?) = 1 has a unique solution? → Because every other R_local is ≥ 4/3, only R_local(3, 1) produces exactly 4/3.
- [ ] Can you give a lower bound showing no solution exists with three or more prime factors? → (3/4) · (4/3)² = 4/3 > 1.
- [ ] Can you state the current honesty status of the "three independent proofs" claim? → Only Proof 1 is rigorous + Proof 4 is strong empirical; Proof 2 / Proof 3 are withdrawn as repackagings.
- [ ] Have you memorized R values at n = 28, 496, 8128? → 4, 48, 576.
- [ ] Can you explain that this theorem does not target the seven Millennium problems? → It is an independent uniqueness fact of number theory, unrelated to the seven problems.

---

## 7. Primary Sources and atlas.n6 Cross-references

- `theory/proofs/theorem-r1-uniqueness.md` — main proof + honesty declaration + retraction history (direct source of this note)
- `theory/constants/atlas-constants.md` — R(6) = 1 and 24 = σ · φ = J₂ cross-check
- `nexus/shared/n6/atlas.n6` L25–L54 — registration of seven base constants `@P` (n, σ, φ, τ, sopfr, J₂, μ), all grade [10*] or [11*]
- `nexus/shared/n6/atlas.n6` L121–L136 — the four core relations `@R perfect_number`, `@R sigma_decomp`, `@R J2_decomp`, `@R sopfr_phi_tau`

Self-referential citation from atlas.n6 (L25–L50):
```
@P n = 6 :: foundation [11*]
@P sigma = divisor_sum(6) = 12 :: foundation [11*]
@P phi = euler_totient(6) = 2 :: foundation [10*]
@P tau = divisor_count(6) = 4 :: foundation [11*]
@P sopfr = sum_prime_factors(6) = 5 :: foundation [10*]
@P J2 = jordan_totient(6,2) = 24 :: foundation [10*]
@P mu = mobius(6) = 1 :: foundation [10*]
```

---

## 8. Next Study Steps (P0 Track)

- **N6-P0-2** — mastery drill of the n = 6 basic arithmetic system (10 base-value table + decomposition algorithm + 10 practice problems).
- **N6-P0-3** — grade system of atlas.n6 + introduction to the BT system.
- **PURE-P0-1** — foundations of number theory (Hardy-Wright, Apostol).
- **PROB-P0-1** — overview of Clay's 7 Millennium problems.

This theorem is the "arithmetic anchor" of all those paths; at each study step, return to the three-line core summary (▶ in 2.4) for reconfirmation.
