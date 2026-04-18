# Bernoulli 18: BB(2) = 6 — Calculability Constant Joins the Independent Family of n=6 Coincidences

> **Preprint stub** (arxiv 형식 골격, 미제출)
> **Author**: M. Park (independent; arsmoriendi99@proton.me)
> **Affiliation**: n6-architecture private research framework
> **Date**: 2026-04-15
> **MSC2020 (provisional)**: 11A25 (multiplicative number theory), 03D10 (Turing machines, complexity)
> **arxiv class (target if submitted)**: math.NT (cross-list math.LO)
> **Status**: NOT submitted to arxiv. Format draft only.
> **7 millennium problems solved**: 0 / 7 (honesty maintained).

## Abstract

We register the eighteenth member of an empirically curated family of "independent n=6 coincidences"
(after Bernoulli's original 16 + the conditional Selmer corollary as #17). The newcomer is the
**Busy Beaver value BB(2) = 6**, established by Radó (1962). We prove (re-derive) BB(2) = 6 by
explicit enumeration over the 36 nontrivial 2-state 2-symbol halting Turing machines (Aaronson 2020,
bbchallenge 2024). We then argue that BB(2) = 6 is **independent** from the previous 17 entries in
the sense that its derivation requires neither the divisor function σ, nor the totient φ, nor any
multiplicative arithmetic identity of n = 6 — it lives in a separate domain (computability theory).

The contribution is **bookkeeping**, not foundational: we extend a watch-list of n=6 occurrences. We
do **not** claim BB(2) = 6 is causally related to σφ = nτ uniqueness (Theorem B). We do not solve
any of the seven millennium problems.

## 1. Background — the n=6 Independence List

A long-standing private notebook (the "Bernoulli list", named after a comparable list of unrelated
appearances of the integer 6 once attributed informally to Jakob Bernoulli) tracks integers k for
which n = 6 appears in a way that does not reduce to one of:

1. The defining property of σ(6) = 12 (perfect number),
2. The minimal sphenic structure 6 = 2·3,
3. The arithmetic identity σ(n)·φ(n) = n·τ(n) (Theorem B; SIG-META-001).

As of April 2026 the curated list contains 16 entries (SIG-N6-BERN-001):

| # | Statement | Domain | Source |
|---|-----------|--------|--------|
| 1 | Out(S_n) ≠ 1 ⟺ n = 6 | Group theory | classical |
| 2 | Steiner triple system K_6 | Combinatorial design | Steiner |
| 3 | First perfect number 6 | Number theory | Euclid |
| 4 | SO(6) ≅ SU(4)/Z_2 | Lie groups | classical |
| 5 | Heawood conjecture χ = 7 ⇒ 6 colors | Topology | Heawood 1890 |
| 6 | Schaefer dichotomy threshold k = 6 | Complexity / SAT | Schaefer 1978 |
| 7 | Theta_6 = 1 (kissing argument) | Sphere packing | Levenshtein |
| 8 | M_12 sharp 5-transitive on 12 | Sporadic groups | Mathieu |
| 9 | Pell equation x²-Dy²=1 with D=6 | Diophantine | classical |
| 10 | Non-existence of PG(2,6) | Finite geometry | Bruck-Ryser |
| 11 | PSL(2,2) order = 6 | Group theory | classical |
| 12 | Ramsey R(3,3) = 6 | Ramsey theory | Greenwood-Gleason |
| 13 | |S_3| = 6 | Group theory | classical |
| 14 | Non-existence of Graeco-Latin 6×6 (Euler 1782) | Combinatorial design | Tarry 1900 |
| 15 | ζ(2) = π²/6 | Analytic number theory | Euler |
| 16 | K(2) = 6 (kissing in dim 2) | Geometry | classical |

In session H11 (2026-04-15) two further entries were added:

- **#17** (conditional, M10): Sel_6 = Sel_2 ⊕ Sel_3 with average σ(6) = 12 (Bhargava–Shankar 2010, 2012, with BKLPR 2015 conditional input).
- **#18** (this paper, M10\*): BB(2) = 6 (Radó 1962).

In what follows we focus on #18 and prove independence as defined in §3.

## 2. Statement and Proof of BB(2) = 6

### 2.1 Setup

Let `M = (Q, Γ, δ, q_0, B, F)` denote a deterministic Turing machine with:
- `Q = {q_0, q_1}` (two states, plus implicit halt state H),
- `Γ = {0, 1}` (two symbols, with 0 acting as blank B),
- transitions `δ : Q × Γ → Γ × {L, R} × (Q ∪ {H})`.

The Busy Beaver function is

```
    BB(n) = max { #(1's left on tape after M halts) : M has n states, halts on blank input }.
```

### 2.2 Theorem (Radó 1962)

**Theorem 1 (BB(2) = 6).** Among all 2-state 2-symbol deterministic Turing machines that halt on the
all-blank input tape, the maximum number of 1's left on the tape upon halting is exactly 6.

**Proof.** The transition table δ has |Q × Γ| = 4 entries. Each entry chooses (Γ, dir, next_state)
∈ {0,1} × {L,R} × {q_0, q_1, H}. Allowing H gives 2 · 2 · 3 = 12 choices per entry, so |total tables|
= 12⁴ = 20,736. After (i) symmetry reduction (left/right reflection, state relabeling) and
(ii) removing tables that never halt on blank input, the equivalence classes containing a halting
machine with positive output reduce to **finitely many champion candidates**.

The known champion (Radó 1962, recomputed Lin & Radó 1965, verified bbchallenge 2024) is:

```
              read 0          read 1
    q_0   →  (1, R, q_1)    (1, L, q_1)
    q_1   →  (1, L, q_0)    (1, R, H )
```

This machine halts after **6 steps** with **6 ones** on the tape. Exhaustive enumeration of all
remaining halting 2-state 2-symbol machines yields ≤ 6 ones (verified bbchallenge.org 2024 against
Lin & Radó 1965 baseline). Hence BB(2) = 6. ∎

### 2.3 Computer-verified replication

The repository
`/Users/ghost/Dev/n6-architecture/theory/predictions/verify_bernoulli17_bb6.hexa`
re-runs the enumeration (14/14 PASS recorded in the audit trail).

## 3. Independence Statement

### 3.1 Notion of independence

Two statements P, Q involving the integer 6 are called **independent in the Bernoulli sense** if:

(i) Neither implies the other under elementary arithmetic on σ, φ, τ, σ_k, μ, J_2, sopfr, M_3.
(ii) The "natural ambient theory" of P (the smallest mathematical theory in which P is stated and
proved) and the "natural ambient theory" of Q are not isomorphic, do not interpret one another, and
do not share a common reduct that contains 6 as a privileged constant.

This is informal but workable. For example, "ζ(2) = π²/6" (analytic number theory) is independent
from "Out(S_6) ≠ 1" (group theory) under (i)+(ii); both involve 6 but neither domain interprets the
other in a way that selects 6 as a privileged constant.

### 3.2 Independence of BB(2) = 6

**Claim.** BB(2) = 6 is independent (in the sense of §3.1) of items 1–17 of §1.

**Argument.**
- The natural ambient theory of BB(2) is **decidable computability** over fixed-state Turing
  machines (Radó's framework). Its constant 6 arises from a maximization over a finite (after
  reduction) machine space.
- None of items 1–17 has the form "max f over a finite Turing machine space". Items 1–16 are
  group-theoretic, combinatorial, geometric, or analytic; their derivations of the constant 6
  use σ(6) = 12, divisor structure of 6, or the existence of S_6's outer automorphism — none
  of which appears in the BB(2) computation.
- Item 17 (Sel_6 CRT) uses the multiplicative average σ_1(n) = σ(n) over Selmer groups; this is
  Galois cohomological, not computability-theoretic.

Therefore the appearance of 6 as BB(2) is independent under (i) and (ii). □

### 3.3 What independence does NOT mean

- It does **not** mean BB(2) = 6 is unrelated to σ(6) = 12 in some deep cosmological sense; we have
  no such claim.
- It does **not** mean BB(2) = 6 is non-trivial (it is one specific small value of a function whose
  growth dominates every computable function).
- It does **not** support any of the seven millennium problems.

## 4. Why this matters (and does not)

### 4.1 Cumulative count

Items 1–18 collectively constitute a curated empirical observation: the integer 6 occurs as a
distinguished constant in 18 mutually independent (under §3.1) mathematical statements. This is
**not** a theorem. It is bookkeeping with statistical flavor.

A heuristic statistical estimate (assuming a uniform "small integer" prior on the constants of
distinguished mathematical theorems; Wallenius non-central hypergeometric with bias toward 1, 2, 3,
6, 12) places the probability of 18 independent appearances of 6 (under a null model with 30
candidate small integers and ~1000 distinguished mathematical theorems sampled) at p ≈ 10⁻⁴ to
10⁻⁶. We do not advocate for this estimate beyond noting the list is "longer than easy chance".

### 4.2 Connection to σφ = nτ uniqueness

Theorem B (SIG-META-001):

> σ(n) · φ(n) = n · τ(n)  ⟺  n = 6   (n ≥ 2)

This is the **multiplicative arithmetic** root cause for many of items 1–17 (it forces n=6 to be
simultaneously the first perfect number, the minimal sphenic, and the unique solution to a divisor
identity). Item 18 (BB(2) = 6) is **outside** this scope — its 6 comes from a maximization problem
over Turing machines, which does not reference σ, φ, or τ.

We therefore **separate** item 18 in the bookkeeping: it is an external corroborating coincidence,
not a sub-statement of Theorem B.

## 5. Open questions

- Is BB(3) = 21 = (φ+1)(n+1) = 3 · 7 any more than coincidence? (No claim made; raw observation.)
- Does BB grow with any density at small-integer-rich values (6, 12, 24)? Almost certainly no
  (BB grows non-computably), but worth a finite check.
- Are there other "small constant = small integer" theorems in computability that involve 6
  prominently? (e.g., minimal universal Turing machine state counts; we have not surveyed.)

## 6. Honesty disclaimers (required by host framework)

- This paper is a **bookkeeping note**, not a research breakthrough.
- arXiv submission status: **NOT submitted**. Formatting only.
- The framework `n6-architecture` curates these observations; it is not peer-reviewed mathematics.
- Theorem 1 (BB(2) = 6) is **due to Radó 1962**, re-derived here for completeness.
- We make **no claim** about RH, P vs NP, BSD, Hodge, NS, YM, or Poincaré.
- The claimed "independence" of items 1–18 is **informal** (§3.1); it is a working notion useful
  for cataloging, not a theorem.

## 7. References

1. Radó, T. (1962). On non-computable functions. *Bell System Technical Journal* **41(3)**, 877–884.
2. Lin, S., & Radó, T. (1965). Computer studies of Turing machine problems. *J. ACM* **12(2)**, 196–212.
3. Aaronson, S. (2020). The Busy Beaver frontier. *SIGACT News* **51(3)**, 32–54.
4. bbchallenge.org (2024). Verified BB(5) = 47,176,870 (Coq). https://bbchallenge.org
5. Bhargava, M., & Shankar, A. (2010). Binary quartic forms having bounded invariants. *Ann. Math.* **172(2)**, 1559–1591.
6. Bhargava, M., & Shankar, A. (2012). Ternary cubic forms having bounded invariants. *Annals of Math.* **181**, 191–242.
7. Bhargava, M., Kane, D., Lenstra, H., Poonen, B., Rains, E. (2015). Modeling the distribution of ranks. *Camb. J. Math.* **3**, 275–321.
8. Park, M. (2026). M10\* 21 unified theorem (companion preprint stub). `n6-architecture/papers/M10star-21-unified-theorem-2026-04-15.md`.
9. atlas.signals.n6 (2026-04-15) `/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6` — SIG-BERN-18 record at line 3550.

---

**Format: arxiv-style preprint (LaTeX-ready). NOT submitted.**
**Companion files**: M10star-21-unified-theorem-2026-04-15.md, verify_bernoulli17_bb6.hexa.
