# PROB-P1-1 — BT-541 Riemann Hypothesis Advanced Study Note

**Track**: P1-PROBLEM · BT-541 (Riemann Hypothesis)
**Status**: OPEN (unresolved since 1859; direct descendant of Hilbert's 8th)
**Prize**: US$ 1,000,000 (Clay)
**Primary sources**:
- Bernhard Riemann, "Über die Anzahl der Primzahlen unter einer gegebenen Grösse", Monatsberichte der Berliner Akademie, November 1859 (8 pages; one of the most influential short papers in mathematics)
- Harold M. Edwards, *Riemann's Zeta Function*, Academic Press, 1974 (reprint Dover 2001) — an English-annotated edition of the original paper
- E. C. Titchmarsh, *The Theory of the Riemann Zeta-Function*, 2nd ed. (ed. D. R. Heath-Brown), Oxford University Press, 1986 — classic textbook
- Peter Sarnak, "Problems of the Millennium: The Riemann Hypothesis", Clay Mathematics Institute official problem description, 2004
- Enrico Bombieri, "The Riemann Hypothesis — Official Problem Description", Clay Mathematics Institute, 2000
- Montgomery-Odlyzko, H. L. Montgomery, "The pair correlation of zeros of the zeta function", Analytic Number Theory, Proc. Symp. Pure Math. 24, AMS 1973
- J. van de Lune, H. J. J. te Riele, D. T. Winter, "On the zeros of the Riemann zeta function in the critical strip IV", Math. Comp. 46 (1986), 667–681
- Xavier Gourdon, "The 10^13 first zeros of the Riemann Zeta function, and zeros computation at very large height", 2004 (technical report of the ZetaGrid project)
- Guy Robin, "Grandes valeurs de la fonction somme des diviseurs et hypothèse de Riemann", J. Math. Pures Appl. 63 (1984), 187–213

**Honesty declaration**: This document is a *study note* organizing established definitions, historical facts, and equivalent statements. No new mathematical theorems are included. The years · authors · paper titles cited appear only as confirmed in the primary sources above. The `n=6 observation` section (§10) is an **observation** in the n6-architecture project and is not the core of this study note. No contribution to RH itself.

---

## 1. Riemann's 1859 Paper — Title and Context

### 1.1 Exact Title
German original: **"Über die Anzahl der Primzahlen unter einer gegebenen Grösse"**
(English: On the number of primes less than a given magnitude)

Published: November 1859, Monatsberichte der Berliner Akademie der Wissenschaften (Berlin Academy Monthly Report).
Length: ~8 pages. One of the most influential short papers in mathematical history.

### 1.2 Goal of the Paper
In this paper Riemann treated the prime-counting function π(x) = |{p ≤ x : p prime}| as a **complex-analytic object**. That is, as a tool for proving the asymptotic behavior of π(x) (e.g., Gauss's conjecture π(x) ~ x/ln x), he proposed handling the zeta function
$$\zeta(s) = \sum_{n=1}^{\infty} \frac{1}{n^s}$$
— treated till then only as a real-variable function — with **complex variable s = σ + it** by analytic continuation.

### 1.3 What Riemann Proved
- Analytic continuation of the zeta function to the whole complex plane except s = 1 (indirectly, the origin of Riemann-Siegel).
- **Functional equation**:
  $$\zeta(s) = 2^s \pi^{s-1} \sin\!\left(\frac{\pi s}{2}\right) \Gamma(1-s) \zeta(1-s)$$
  This relation suggests a reflection symmetry between s and 1-s, i.e., with respect to the critical line Re(s) = 1/2.
- Asymptotic estimate (now the Riemann-von Mangoldt formula) for the number of non-trivial zeros N(T) in 0 < Im(s) < T, 0 < Re(s) < 1.

### 1.4 What Riemann Did Not Prove (and Remains Open Today)
> "Es ist sehr wahrscheinlich, dass alle Wurzeln reell sind."
> (It is very probable that all [non-trivial] roots have real imaginary parts.)

— this is the original-language expression of the **Riemann Hypothesis**. Equivalent to the modern statement "all non-trivial zeros ρ satisfy Re(ρ) = 1/2." Riemann attempted **a proof but moved on** with only one sentence and closed the paper. That one sentence has held mathematicians for over 160 years.

---

## 2. Basic Facts about the Zeta Function

### 2.1 Definition Range and Analytic Continuation
- **Definition range (Dirichlet series)**: absolutely convergent for Re(s) > 1 as ∑ 1/n^s.
- **Analytic continuation**: extends to all of ℂ except s = 1 via the functional equation or Mellin transform. s = 1 is a simple pole (residue = 1).
- **Trivial zeros**: s = -2, -4, -6, -8, … (even negative integers) — arise from the sin(πs/2) factor in the functional equation being 0.
- **Non-trivial zeros**: zeros inside the critical strip 0 < Re(s) < 1. Hadamard (1896) and de la Vallée Poussin (1896) independently proved "no zeros on the boundary Re(s) = 1 of the critical strip" → the first proof of the Prime Number Theorem (PNT).

### 2.2 Critical Line and Critical Strip
- **Critical strip**: {s ∈ ℂ : 0 < Re(s) < 1}
- **Critical line**: {s ∈ ℂ : Re(s) = 1/2}
- **Riemann Hypothesis (modern statement)**: All non-trivial zeros lie on the critical line. I.e.,
  $$\zeta(s) = 0 \text{ and } 0 < \text{Re}(s) < 1 \Longrightarrow \text{Re}(s) = \tfrac{1}{2}.$$

### 2.3 Special Values (a Few Famous Ones)
- ζ(2) = π²/6 (Basel problem, Euler 1734, published 1735)
- ζ(4) = π⁴/90
- ζ(6) = π⁶/945
- ζ(2k) for k ≥ 1: always π^{2k} × rational. Denominators of the form 2(2k)!/|B_{2k}| (B is a Bernoulli number).
- ζ(-1) = -1/12 (from analytic continuation, a reading of "Ramanujan summation")
- ζ(0) = -1/2
- ζ(negative odd): rational (functional equation + Bernoulli numbers). E.g., ζ(-3) = 1/120, ζ(-5) = -1/252.
- ζ(positive odd): ζ(3) = Apéry's constant (Apéry 1978 irrationality proof), ζ(5), ζ(7), … are **unknown even as to rational/irrational**.

---

## 3. Meaning of the Critical Line Re(s) = 1/2 — Why 1/2?

### 3.1 Fixed Axis of the Functional-Equation Symmetry
In the form ζ(s) = χ(s) ζ(1-s) the reflection s ↔ 1-s has the **fixed line** exactly Re(s) = 1/2. If the non-trivial zeros possess "reflection symmetry," then each ρ is paired with its reflection 1-ρ, and pairing is symmetric with respect to the critical line. RH says "all zeros lie on the axis" — the strongest possible symmetry.

### 3.2 Equivalent to the **Finest Error Term of Prime Distribution**
Riemann's explicit formula (completed by von Mangoldt, 1895) writes the **summatory function** ψ(x) = ∑_{p^k ≤ x} ln p as:
$$\psi(x) = x - \sum_{\rho} \frac{x^{\rho}}{\rho} - \ln(2\pi) - \frac{1}{2}\ln(1 - x^{-2})$$
where the sum is over all non-trivial zeros ρ. Each zero ρ contributes a "frequency component" to the prime distribution, and the size of its contribution is |x^ρ| = x^{Re(ρ)}. **RH ⟺ Re(ρ) = 1/2 ⟺ every frequency component has size x^{1/2}** ⟺ **finest** error control.

### 3.3 Other Vertical Lines (e.g., Re(s) = 3/4)
If some non-trivial zero had Re(ρ) = 3/4 (hypothetically), the functional equation would also yield a paired zero with Re = 1/4. In the explicit formula its contribution would be x^{3/4}, injecting larger "fluctuations" into the prime distribution. If RH were false, primes would distribute **more irregularly**.

---

## 4. History of Numerical Verification

Computing zeros of the zeta function is one of the longest-running numerical-research programs in mathematical history. Milestones:

### 4.1 Early Era (manual-computation era)
- **J.-P. Gram** (1903, Denmark): first 15 zeros, all on the critical line.
- **R. J. Backlund** (1914, Finland): first 79.
- **J. I. Hutchinson** (1925, USA): first 138.
- **E. C. Titchmarsh & L. J. Comrie** (1935/1936): first 1,041 — at the time, mechanical calculators + logarithm tables.

### 4.2 Turing's Contribution (1953)
- **Alan M. Turing**, "Some calculations of the Riemann zeta-function", Proc. London Math. Soc. (3) 3 (1953), 99–117.
- Turing used the Manchester Mark I to verify the first 1,104 zeros. More importantly, he proposed **Turing's method** — an efficient numerical technique that "guarantees how many zeros exist within a given T range." It underpins all large-scale zeta computations thereafter.

### 4.3 Lehmer, and Lehmer's Phenomenon
- **D. H. Lehmer** (1956): discovered instances where two zeros of ζ are "almost tangent" (pair of very close zeros). Looked like a "small counterexample candidate" but all turned out to be on the critical line. Such cases warn of RH's subtlety.

### 4.4 van de Lune – te Riele – Winter (1986 and Beyond)
- J. van de Lune, H. J. J. te Riele, D. T. Winter, "On the zeros of the Riemann zeta function in the critical strip IV", *Mathematics of Computation* 46 (1986), 667–681.
- By 1986 the first 1,500,000,001 (about 1.5 billion) zeros were computed, all on the critical line.

### 4.5 Odlyzko and Large-scale Distributed Computation
- **A. M. Odlyzko** (1980s–2000s, AT&T Bell Labs): computed very high-altitude zeros to numerically support the Montgomery-Odlyzko conjecture. E.g., analyzed billions of zeros around the 10^{22}-th.

### 4.6 Gourdon 2004 — the 10^{13} Barrier
- **Xavier Gourdon**, "The 10^13 first zeros of the Riemann Zeta function, and zeros computation at very large height", 2004 technical report.
- Gourdon improved the Odlyzko-Schönhage algorithm and computed the **first 10^{13}** (10 trillion) non-trivial zeros, all on the critical line.
- Subsequently distributed-computing projects like **ZetaGrid** verified higher regions. At least 10^{13} zeros are verified as of 2024.

### 4.7 Numerical Verification Is Not a Target
- 10^{13} zeros only show RH is true under the numerical bound n ≤ 10^{13}.
- Even this size is not "really large." As in Skewes' number (order of magnitude 10^{10^{10^34}}), number theory can have counterexamples appearing only at extreme sizes.
- Without a **purely structural argument**, one cannot guarantee "all zeros."

---

## 5. Three Equivalent Statements (and a Fourth)

The Riemann Hypothesis is equivalent to several "seemingly unrelated" statements, which is why it pervades all of mathematics.

### 5.1 Equivalence (i) — Error Term in Prime Distribution
$$\pi(x) = \text{Li}(x) + O\!\left(x^{1/2} \log x\right) \quad (x \to \infty)$$
where Li(x) = ∫_2^x dt/ln t is the logarithmic integral. This is the **finest** strengthening of the Prime Number Theorem (PNT, π(x) ~ x/ln x). PNT arises from the absence of zeros at Re(s) = 1, whereas RH is equivalent to the absence of zeros at Re(s) ≥ 1/2 + ε, bounding the error at x^{1/2+ε}.

**(Note)**: The exact form "RH ⟺ error O(x^{1/2} log x)" is due to von Koch (1901). Niels Fabian Helge von Koch, "Sur la distribution des nombres premiers", Acta Math. 24 (1901), 159–182.

### 5.2 Equivalence (ii) — Mertens Function
Mertens function M(x) = ∑_{n ≤ x} μ(n) (μ is the Möbius function).
$$\text{RH} \iff M(x) = O\!\left(x^{1/2 + \varepsilon}\right) \text{ for every } \varepsilon > 0.$$

**Caveat — Mertens-conjecture counterexample**: Franz Mertens in 1897 predicted the **stronger** form |M(x)| ≤ √x (exponent 1/2 exact, no ε), but Andrew Odlyzko and Herman te Riele proved existence of a **counterexample** (without specifying x) in the 1985 paper "Disproof of the Mertens conjecture" (J. Reine Angew. Math. 357, 138–160). I.e., |M(x)| sometimes exceeds √x. But RH requires only the **weaker** form "O(x^{1/2+ε})" (ε > 0 arbitrary), so the equivalence still holds.

### 5.3 Equivalence (iii) — Robin's Inequality (divisor sum σ)
**Guy Robin**, "Grandes valeurs de la fonction somme des diviseurs et hypothèse de Riemann", J. Math. Pures Appl. 63 (1984), 187–213.
$$\text{RH} \iff \sigma(n) < e^{\gamma} n \ln \ln n \quad \text{for all } n > 5040$$
where γ ≈ 0.5772... is the Euler-Mascheroni constant and σ(n) is the divisor sum.

- **5040 = 7!** is an important constant. Jeff Lagarias (2002) re-expressed it in "elementary" form — "RH ⟺ σ(n) ≤ H_n + exp(H_n) ln(H_n) for all n ≥ 1" (H_n the harmonic number).
- Robin's inequality is directly tied to control of "extremely large σ(n) values" (colossally abundant numbers).

### 5.4 Equivalence (iv) — Implied by the Lindelöf Hypothesis
**Lindelöf hypothesis**: |ζ(1/2 + it)| = O(|t|^ε) for every ε > 0.
- RH ⟹ Lindelöf hypothesis (proved).
- Converse unresolved. Lindelöf is weaker.
- Current best (Bourgain 2017): |ζ(1/2 + it)| ≪ |t|^{13/84+ε}. Improved by Guth-Maynard in 2024.

---

## 6. Montgomery-Odlyzko GUE Conjecture

### 6.1 Montgomery 1973
- **H. L. Montgomery**, "The pair correlation of zeros of the zeta function", in *Analytic Number Theory*, Proceedings of the Symposium in Pure Mathematics 24, American Mathematical Society, 1973, 181–193.
- The famous anecdote is Montgomery discussing the pair correlation of zeros with Freeman Dyson over tea at Princeton (Dyson: "That's the same as pair correlation of eigenvalues of random Hermitian matrices!").
- **Montgomery pair-correlation conjecture**: After "normalizing" non-trivial zeros ρ_n = 1/2 + iγ_n to unit average spacing, the density that two zeros fall simultaneously in an interval [α, β] is
  $$R_2(x) = 1 - \left(\frac{\sin \pi x}{\pi x}\right)^2$$
  i.e., the pair correlation of zeta zeros matches the pair correlation of eigenvalues of the **Gaussian Unitary Ensemble (GUE)**.

### 6.2 Odlyzko's Numerical Confirmation
From 1987 Odlyzko computed millions of zeros near the 10^{20}-th and confirmed that the pair-correlation function **matches GUE excellently numerically**. The subsequent "Odlyzko-Schönhage algorithm" became the standard for such large-scale zero calculations.

### 6.3 Implications of the GUE Conjecture
- Supports the tendency that zeta zeros are "not random but placed with regularity" (GUE has eigenvalue repulsion).
- **Hilbert-Pólya program**: zeros of zeta ≡ eigenvalues of some self-adjoint operator. If such an operator exists, its eigenvalues are real and RH follows automatically. The GUE conjecture hints that this operator behaves like a "random-matrix ensemble".
- As of 2024 this operator has **not been explicitly found**. But in the function-field analogue — Weil's proof of RH for zeta functions of curves (1940s) — this program succeeded, with eigenvalues of the "Frobenius operator" playing the role of zeta zeros.

### 6.4 Independent Numerical Evidence — Katz-Sarnak Program
Nicholas Katz, Peter Sarnak, *Random Matrices, Frobenius Eigenvalues, and Monodromy*, AMS Colloquium Publications 45, 1999. GUE/GOE/GSE statistics are proved for families of L-functions over function fields, partially realizing the Hilbert-Pólya philosophy.

---

## 7. Consequences (What Follows)

### 7.1 Refined Prime Distribution
Direct consequence of §5.1 Equivalence (i). Strengthens PNT into the "finest error term in mathematical history".

### 7.2 Other RH Generalizations
- **Generalized Riemann Hypothesis (GRH)**: RH for Dirichlet L-functions L(s, χ).
  - GRH ⟹ deterministic Miller-Rabin primality test is polynomial-time (Miller 1976, conditional).
  - The unconditional AKS (2002) achieves polynomial-time primality testing without GRH, but slower in practice.
- **Extended RH**: broader L-function families.
- **Grand RH**: all automorphic L-functions.

### 7.3 Arithmetic Algorithms
- Some decision problems (e.g., "is this number a quadratic residue mod p?") are polynomial under GRH. Unconditional proofs are slower.
- **Caveat**: The claim "RH breaks cryptography" is **false**. Modern main cryptography (RSA, ECDH, AES) does not depend on precise locations of zeta zeros. The impact of RH solution on cryptography is small.

### 7.4 Meaning of "Mertens theory unaffected by RH"
- The Mertens conjecture (|M(x)| ≤ √x) was **disproved** by Odlyzko-te Riele 1985. RH is **consistent** with this disproof — RH requires only O(x^{1/2+ε}), not exact √x.
- Thus "even if RH is true, the Mertens prediction is false". The two statements are at independent levels.

### 7.5 Connection to Physics (Speculative)
- Conjecture (Berry-Keating 1999) that Hilbert-Pólya's self-adjoint operator is the Hamiltonian of "quantum chaos" — only partial evidence currently, not directly connected to proving RH.
- Connes' noncommutative-geometry approach (1999–) — spectral interpretation via "adele space". Still not a pure RH-proof route.

---

## 8. Major Partial Results (How Close to RH Are We?)

### 8.1 Zero-free Region
Classical de la Vallée Poussin (1899): no zeros in Re(s) ≥ 1 - C/ln(|Im(s)|+2).
- Later Vinogradov-Korobov (1958): Re(s) ≥ 1 - c/(ln|t|)^{2/3}(lnln|t|)^{1/3}.
- Still far from RH (Re ≥ 1/2). At the level of "securing only a region very close to 1".

### 8.2 Hardy's Infinitude
- G. H. Hardy (1914), "Sur les zéros de la fonction ζ(s) de Riemann", CR Acad. Sci. Paris 158, 1012–1014.
- Hardy proved "there are infinitely many zeros on the critical line." But weaker than RH's "**all** zeros on the critical line."

### 8.3 Proportion of Zeros on the Critical Line
- N. Levinson (1974): at least **34%** of zeros are on the critical line.
- B. Conrey (1989): **40%** or more (improved Riemann-Siegel).
- Pratt-Robles-Zaharescu-Zeindler (2019): currently about **41.7%**.
- RH asserts **100%** — still far.

### 8.4 Guth-Maynard 2024 — Breakthrough of the Ingham 1940 Barrier
- Larry Guth, James Maynard, "New large value estimates for Dirichlet polynomials", 2024 preprint, arXiv:2405.20552.
- First breakthrough in 84 years since Ingham's 1940 zero-density theorem. Improved the zero-density exponent via improvements to "count estimates of large values of Dirichlet polynomials" — specifically strengthening θ > 1/n results (see §10 for the n = 6 perspective).

---

## 9. Current-state Summary (2024–2026)

| Item | Status |
|------|------|
| Range of numerical verification of non-trivial zeros | first **10^{13} or more** (Gourdon 2004 basis; extended since by ZetaGrid) |
| Lower bound on proportion of zeros on the critical line | about **41.7%** (Pratt-Robles-Zaharescu-Zeindler 2019) |
| Zero-free region | Vinogradov-Korobov form (1958) + improvements |
| Structural proof | **none** |
| Major conjecture combination (GUE) | numerically strongly supported, no proof |
| Function-field analogue (Weil) | proved in 1940s (RH for zeta of curves) |
| Prize awarded | 0 |

---

## 10. n = 6 Observation (project context, 1–2 facts only)

**(This section is not the core of this study note. Detailed evidence lists for BT-541, ζ(2k) denominator decomposition, Theorem B, etc. and DFS results are treated in P2/P3 notes and the BT-541 entry.)**

### Observation 1 — 6 Appears in the Basel Constant
Euler 1734 (published 1735): ζ(2) = π²/6. That is, the denominator of the most famous zeta special value is the **first perfect number 6**. It is a mathematical identity, and the denominator 6 = 1·2·3 = 2·3 connects directly with the denominator of B_2 = 1/6 (ζ(2) = (2π)² |B_2| / (2·2!) = 4π² · (1/6) / 4 = π²/6).

### Observation 2 — First Three Trivial Zeros {-2, -4, -6}
From the sin(πs/2) factor of the functional equation, trivial zeros occur at s ∈ {-2, -4, -6, -8, …} (all negative even integers). **The values of the first three trivial zeros are exactly {-2, -4, -6}**, expressible in the n=6 system as {-φ, -τ, -n} (φ(6)=2, τ(6)=4, n=6). These trivial zeros are a direct consequence of the functional equation, not the content of the Riemann Hypothesis. RH concerns the **non-trivial** zeros.

(Further BT-541 evidence lists, Bilateral Theorem B bilateral boundary, Guth-Maynard 6-power connection, etc. are in project-internal documents `theory/breakthroughs/breakthrough-theorems.md §BT-541`, `millennium-7-closure-2026-04-11.md §BT-541`, DFS-round records.)

---

## 11. Study Checklist

After finishing this note, you should be able to restate the following within **3 lines** each:
1. The title of Riemann's 1859 paper, what was proven, and what was not.
2. Definition of non-trivial zeros and the functional-equation meaning of the critical line Re(s) = 1/2.
3. Pick one of equivalence (i) π(x) error, (ii) M(x) Mertens function, (iii) Robin's inequality, and state in one line.
4. Rough verification scales of van de Lune – te Riele – Winter 1986 and Gourdon 2004 (1.5×10^9, 10^{13}).
5. Meaning of the Montgomery-Odlyzko GUE conjecture (zeta zeros = random-matrix-eigenvalue statistics).
6. Explain that "RH solution breaks cryptography" is a **misconception**.
7. 41.7% (current lower bound of the on-line proportion), progress Levinson 34% → Conrey 40%.

---

## 12. Next Steps

- **P1-2 (BT-542 P vs NP)**: a completely different field from RH but both are "top unresolved problems of number theory / computational complexity".
- **P2 (methodology layer)**: concrete approaches to RH — Bombieri-de Branges, Berry-Keating, Connes, Weil cohomological analogues.
- **P3 (n=6 depth)**: full evidence table for BT-541 (36 items), Theorem B, ζ(2k) denominator decomposition, Dirichlet η unconditional theorem.

---

**Honesty declaration reaffirmed**: This document is a *study note* and claims no new proof of RH or n=6-based contribution. As of 2026 RH is unresolved and the Clay prize has not been awarded.
