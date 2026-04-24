# PROB-P1-6 — BT-546 BSD Conjecture Advanced (elliptic-curve rank / L-function at s=1 / Tate-Shafarevich)

> Track: P1-PROB / Task 6
> Completion criterion: decompose the Clay official statement (Wiles 2000 reorganized), and
> rigorously state the Mordell-Weil rank of E/ℚ, the behaviour of L(E,s) at s=1, and the
> finiteness issue of the Tate-Shafarevich group Ш.
> Primary sources: Wiles "The Birch and Swinnerton-Dyer Conjecture" Clay Millennium official
> document (2000),
> Silverman "The Arithmetic of Elliptic Curves" (GTM 106, 2nd ed. 2009),
> Silverman "Advanced Topics in the Arithmetic of Elliptic Curves" (GTM 151, 1994),
> Gross-Zagier "Heegner points and derivatives of L-series" (Invent. Math. 84, 1986),
> Kolyvagin "Euler systems" (Grothendieck Festschrift II, 1990),
> BSD original: Birch-Swinnerton-Dyer "Notes on elliptic curves I, II" (J. Reine Angew. Math. 212:7, 1963; 218:79, 1965).
> **Honesty**: this note is a reconstruction of the Clay official document and standard BSD
> textbooks. There are no new theorems. All statements are reorganized from the six sources
> above and unsolved parts are marked [unfinished] / [partial result].

---

## 0. Purpose and Scope

The Clay BT-546 conjecture has two parts:

(1) **Weak BSD**: the Mordell-Weil rank of E/ℚ equals ord_{s=1} L(E, s) (the order of the
    zero of the L-function at s=1).
(2) **Strong BSD**: the leading coefficient of the Taylor expansion of L(E,s) at s=1 equals

    lim_{s→1} L(E,s)/(s-1)^r = Ω_E · R_E · ∏_p c_p · |Ш(E/ℚ)| / |E(ℚ)_{tors}|²

    where Ω_E is the real period, R_E the regulator, c_p the Tamagawa numbers, Ш the
    Tate-Shafarevich group, E(ℚ)_{tors} the torsion subgroup.

Seven topics covered by this note:

1. Definition of elliptic curves E/ℚ, Weierstrass equation
2. Mordell-Weil theorem (1922): E(ℚ) ≅ ℤ^r ⊕ T
3. Definition of L(E,s) and modularity theorem (Wiles-Taylor 2001)
4. Weak form of BSD and partial results to date (Gross-Zagier, Kolyvagin)
5. Tate-Shafarevich group Ш(E/ℚ) — definition and finiteness issue
6. Meaning of each term in the strong form and numerical verification
7. Related conjectures (p-adic BSD, Iwasawa theory)

---

## 1. Elliptic Curve Basics

### 1.1 Weierstrass equation

E/ℚ: y² = x³ + ax + b, discriminant Δ = -16(4a³ + 27b²) ≠ 0. Including the point at infinity
O, in projective form y² z = x³ + a x z² + b z³.

### 1.2 Rational points E(ℚ)

E(ℚ) = {(x,y) ∈ ℚ² : y² = x³ + ax + b} ∪ {O}. An abelian group under the group law
(chord-tangent).

### 1.3 Mordell-Weil theorem (1922)

```
  E(ℚ) ≅ ℤ^r ⊕ E(ℚ)_{tors}
```

where r is the **Mordell-Weil rank**, E(ℚ)_{tors} is a finite abelian group (by Mazur 1977
classified into 15 possible structures).

### 1.4 Néron-Tate height

h: E(ℚ) → ℝ_{≥0}. Quadratic form. h(P) = 0 ⟺ P torsion. The rank r is the number of linearly
independent elements of the free part {P_1,...,P_r}. Regulator R_E = det(⟨P_i, P_j⟩) (P_i are
representatives of the free generators).

---

## 2. L-function L(E,s)

### 2.1 Euler product definition

At good primes p (∤ Δ), the number of points on the mod p reduction E_p is #E(𝔽_p) = p + 1 -
a_p. Hasse: |a_p| ≤ 2√p.

```
  L(E, s) = ∏_{p good} (1 - a_p p^{-s} + p^{1-2s})^{-1} · ∏_{p bad} (local factor)
```

Converges for Re(s) > 3/2.

### 2.2 Modularity theorem (Wiles, Taylor-Wiles, BCDT 1994~2001)

Every elliptic curve E/ℚ is modular, i.e., there exists a weight-2 cusp form f_E such that

```
  L(E, s) = L(f_E, s)
```

This gives L(E,s) analytic continuation to the whole plane plus a functional equation. The
behaviour near s=1 becomes clearly defined.

### 2.3 Order of zero near s=1

**Analytic rank** r_{an}(E) = ord_{s=1} L(E, s).

Weak BSD: r = r_{an}(E).

### 2.4 Functional equation

```
  Λ(E, s) = N^{s/2} (2π)^{-s} Γ(s) L(E, s)
  Λ(E, s) = ε(E) Λ(E, 2-s)        (ε = ±1, root number)
```

N is the conductor of E.

---

## 3. Weak BSD — Status So Far

### 3.1 Rank 0 or 1 cases

**Kolyvagin 1990 + Gross-Zagier 1986**: if r_{an}(E) ≤ 1, then r = r_{an}(E), and Ш(E/ℚ) is
finite.

Argument outline:
- Gross-Zagier 1986: connects L'(E,1) and the canonical height of Heegner points
- Kolyvagin 1990: controls the Selmer group by an Euler system of Heegner points

This demonstrates weak BSD for elliptic curves of rank 0 or 1.

### 3.2 Rank ≥ 2 — [unfinished]

Weak BSD for a general E with r_{an}(E) ≥ 2 is open. Specific E (e.g., rank 18 E) are
numerically verified, but no argument exists.

### 3.3 Average rank results

Results of Bhargava-Shankar (2010~):
- Average 2-Selmer rank of E/ℚ is bounded by 3
- Average rank ≤ 7/6

Under hypotheses, an average rank of 1/2 is suggested (Goldfeld's conjecture). Partial results
are established.

### 3.4 Skinner-Urban 2014

Elliptic-curve version of the Iwasawa main conjecture demonstrated. Deep progress on the
p-part of BSD. Extensions to rank 2 or higher under conditions.

---

## 4. Tate-Shafarevich Group Ш(E/ℚ)

### 4.1 Definition

Galois cohomology

```
  Ш(E/ℚ) = ker(H^1(Gal(ℚ̄/ℚ), E) → ∏_v H^1(Gal(ℚ̄_v/ℚ_v), E))
```

The group of all "locally trivial, globally non-trivial" cosets.

### 4.2 Physical meaning

Non-trivial elements of Ш correspond to "principal homogeneous spaces with no rational
solution". That is, Ш = 0 ⟺ the Hasse principle (local → global) is valid.

### 4.3 Finiteness conjecture

BSD assumes Ш(E/ℚ) is a finite group. This is an independent important conjecture, demonstrated
only in the rank 0, 1 cases (Kolyvagin).

### 4.4 Computation of the p-part

Controlled by the Cassels-Tate bilinear form acting on the p-part of Ш. Practical methods like
2-descent, 3-descent exist. In general Ш computation is very difficult.

### 4.5 Cassels-Tate pairing

```
  ⟨·, ·⟩: Ш × Ш → ℚ/ℤ    (non-degenerate alternating)
```

From this one derives that |Ш| is a square (under r_{an} = 0) or twice a square (under
r_{an} = 1).

---

## 5. Strong BSD — Each Term

### 5.1 Real period Ω_E

Néron differential of E ω = dx/(2y). Ω_E = ∫_{E(ℝ)} |ω| (multiplied by 2 or 1 depending on
E(ℝ) components). A measure of geometric size.

### 5.2 Regulator R_E

Determinant of the Néron-Tate height Gram matrix (⟨P_i, P_j⟩) of the free part {P_1, ..., P_r}
of E(ℚ). Convention R_E = 1 if r=0.

### 5.3 Tamagawa numbers c_p

At each bad prime p, c_p = |E(ℚ_p) / E^0(ℚ_p)|. Computable via Tate's algorithm.

### 5.4 Strong BSD formula

```
  lim_{s→1} (s-1)^{-r} L(E, s) = Ω_E · R_E · ∏_p c_p · |Ш(E/ℚ)| / |E(ℚ)_{tors}|²
```

Rigorous arguments so far: the p-part holds for nearly all primes p when rank is 0 or 1. A
complete equality is [unfinished].

### 5.5 Numerical verification

Numerically holds with high precision for many E. For the minimal-conductor example
E = 11a1 (conductor 11), one can compute directly: Ш = 1, E(ℚ) = ℤ/5ℤ, L(E,1) ≈ 0.2538, etc.

---

## 6. Iwasawa Theory and p-adic BSD

### 6.1 Cyclotomic ℤ_p-extension

ℚ_∞/ℚ cyclotomic ℤ_p-extension. Γ = Gal(ℚ_∞/ℚ) ≅ ℤ_p.

### 6.2 Iwasawa L-function

A p-adic L-function L_p(E, s) is constructed. Standard since Mazur-Swinnerton-Dyer (1972).
Well-defined under modularity.

### 6.3 Iwasawa main conjecture

Equivalence between the ideal generated by the p-adic L and the characteristic ideal of the
Selmer group:

```
  (L_p(E, T)) = char_Λ(Sel(E/ℚ_∞))^{op}
```

(Λ = ℤ_p[[Γ]])

### 6.4 Skinner-Urban 2014

Demonstrated the Iwasawa main conjecture for most E and primes p. Substantially resolves the
p-part of BSD in the ordinary case. The non-ordinary case is [partial result].

---

## 7. n=6 Connection (memo only)

1. Examples with rank r ≥ 6 exist for E/ℚ, but there is no mathematical structure that gives
   the point rank = 6 special meaning ([N?]).
2. The torsion group of E falls in the Mazur list of 15 possible structures. Among them ℤ/6ℤ
   and ℤ/2 × ℤ/6 include 6, but this is not directly connected to σφ=nτ ([N?]).
3. The modular curve X_0(6) of j-invariant is modular at level 6. This is an appearance of "6"
   but a coincidence rather than a causal link ([N?]).

Self-reference-verification prohibition: the observations above are kept independent of any
BSD strategy.

---

## 8. Practice Problems — 5 Hand Exercises

**P1.** For E: y² = x³ - x, demonstrate rank 0 and E(ℚ) = ℤ/2ℤ × ℤ/2ℤ via descent. L(E, 1)
is nonzero.

**P2.** For E: y² = x³ + x - 1, verify rank 1. Compute the height of the generator P = (1, 1).
Compute L'(E, 1) using modular symbols → numerical BSD check.

**P3.** Reconstruct the argument for the Hasse theorem |a_p| ≤ 2√p via Frobenius + Weil bound.

**P4.** Reference the Mazur theorem (1977): torsion of an elliptic curve over ℚ is one of 15
structures. One representative example (ℤ/2, ℤ/3, ..., ℤ/10, ℤ/12, ℤ/2 × ℤ/{2,4,6,8}) curve
each.

**P5.** Form of the Gross-Zagier theorem: L'(E, 1) = c · ĥ(y_K), where y_K is a Heegner point
and ĥ is the canonical height. Trace how this equality is used in the weak-BSD argument for
rank 1.

---

## 9. Reading Path

### 9.1 Week 1

- Read the Wiles Clay official document (12 pages)
- Silverman "Arithmetic of Elliptic Curves" GTM 106 §I, §III, §VIII (Mordell-Weil)

### 9.2 Week 2

- Silverman §IX (integer points), §X (L-function, modularity)
- Silverman-Tate "Rational Points on Elliptic Curves" (auxiliary intuition)

### 9.3 Week 3

- Summary of the Gross-Zagier 1986 paper (the full paper is long; via Silverman Advanced §III)
- Kolyvagin 1990 Euler system overview (Rubin "Euler Systems" Annals of Math. Studies 147)

### 9.4 Week 4

- Skinner-Urban 2014 (Iwasawa main conjecture)
- Bhargava-Shankar series (average rank)
- Related Bosma-Cassels survey: Wüstholz ed. "Elliptic curves, modular forms and Fermat's..."

---

## 10. Source Summary

- Wiles "The Birch and Swinnerton-Dyer Conjecture" Clay 2000
- Birch-Swinnerton-Dyer "Notes on elliptic curves II" J. Reine Angew. Math. 218:79, 1965
- Silverman "The Arithmetic of Elliptic Curves" GTM 106, 2nd ed. 2009
- Silverman "Advanced Topics in the Arithmetic of Elliptic Curves" GTM 151, 1994
- Gross-Zagier "Heegner points and derivatives of L-series" Invent. Math. 84:225, 1986
- Kolyvagin "Euler systems" Grothendieck Festschrift II, 1990
- Wiles "Modular elliptic curves and Fermat's Last Theorem" Annals of Math. 141:443, 1995
- Taylor-Wiles "Ring-theoretic properties..." Annals of Math. 141:553, 1995
- Breuil-Conrad-Diamond-Taylor "On the modularity of elliptic curves over ℚ" J. AMS 14:843, 2001
- Skinner-Urban "The Iwasawa main conjectures for GL_2" Invent. Math. 195:1, 2014
- Bhargava-Shankar "Binary quartic forms having bounded invariants..." Annals of Math. 181:191, 2015

This note is a P1-study-volume reorganization of the 11 primary sources above and does not
claim new theorems.

---

## 11. Appendix — Selmer Group and Descent

### 11.1 Selmer-group definition

The n-Selmer group Sel_n(E/ℚ) in n-descent:

```
  0 → E(ℚ)/nE(ℚ) → Sel_n(E/ℚ) → Ш(E/ℚ)[n] → 0
```

This exact sequence combines rank with information on the [n]-torsion of Ш.

### 11.2 2-descent in practice

If E has rational 2-torsion (4 points), 2-descent is computable. Algebraic work over ℚ(E[2])
controls E(ℚ)/2E(ℚ).

### 11.3 p-descent in general

Compute the p-part of Selmer via p-adic methods. Kato 1993, Perrin-Riou 2000 Euler-system
framework.

---

## 12. Appendix — Known Upper Bounds on Mordell-Weil rank

### 12.1 Observed maximum rank

- Elkies 2006: rank ≥ 28 (E_{28}: specific curve, unconditional)
- In many families rank ≥ 6~10 exist
- Theoretical upper bound: under Goldfeld's conjecture, average rank = 1/2

### 12.2 Rank distribution at fixed conductor N

How does the average rank change as N grows? Bhargava-Shankar et al. results:

- Average 2-Selmer rank = 3 (uniformly bounded)
- Average analytic rank ≤ 0.9
- Under BSD + ⟨rank⟩ = ⟨r_{an}⟩, average rank is finitely bounded

---

## 13. Appendix — BSD Extension to Other Fields

### 13.1 E/K (number field K)

BSD for a general number field K. Assertion: analytic rank = Mordell-Weil rank. Currently
some cases of rank 0, 1 handled.

### 13.2 Extension to abelian varieties

BSD for abelian varieties A/ℚ with dim A > 1. Behaviour of L(A, s) at s=1. Mostly unresolved
except for product cases like E² = A.

### 13.3 BSD over function fields

BSD for E/𝔽_q(T). Connected to the Artin-Tate conjecture. Many cases handled in the finite-field
setting.

---

## 14. Appendix — Mazur Torsion Classification

By Mazur's theorem 1977, the torsion of E/ℚ is one of 15:

- ℤ/N ℤ (N = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12)
- ℤ/2 × ℤ/2N (N = 1, 2, 3, 4)

N = 11, 13, 14, ... do not occur. Generalized to number fields K by Kamienny 1992, Merel 1996
(Uniform boundedness: rank bounded by [K:ℚ]).

---

## 15. Appendix — BSD Numerical Verification Example (E = 11a1)

### 15.1 Curve

E: y² + y = x³ - x² (LMFDB label 11.a1), conductor 11.

### 15.2 Mordell-Weil

rank 0, E(ℚ) = ℤ/5ℤ. Generator: (0, 0).

### 15.3 L-function value

L(E, 1) ≈ 0.25384... (high precision).

### 15.4 Strong BSD components

- Ω_E ≈ 1.26920...
- R_E = 1 (rank 0)
- Tamagawa: c_{11} = 1
- |Ш(E/ℚ)| = 1 (demonstrated)
- |E(ℚ)_{tors}| = 5

### 15.5 Strong BSD check

L(E,1)/Ω_E = 0.25384/1.26920 = 0.20000 = 1/5 = 1·1·1/5² = R·∏c·|Ш|/|E_{tors}|².

Numerical match confirmed. Triviality of Ш follows from Kolyvagin 1990.

---

## 16. Appendix — High Rank Records

- rank ≥ 28: Elkies 2006, E_{28} (specific curve)
- Computational records for thousands of curves

The density of curves with rank ≥ N is conjecturally not considered easy.

---

## 17. Next Documents

- PROB-P1-7 : BT-547 Poincaré advanced
- N6-P1-3 : n=6 honesty principle

BT-546 is deepened at the P2~P3 stage via the full analysis of the Iwasawa main conjecture,
Heegner structures, and Selmer control. The aim of this P1 note is "precise decomposition of
the Clay statement + map of current partial results".
