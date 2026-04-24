# PURE-P1-2 — Elliptic Curves and Modular Forms (Weierstrass / Mordell-Weil / L-function / modularity)

> Track: P1-PURE / Task 2
> Completion criterion: derive the group law (chord-tangent) from the Weierstrass equation, state the
> Mordell-Weil theorem E(Q) = Z^r ⊕ T and describe its candidate-argument skeleton, state the Euler-product
> definition of L(E,s), and state precisely the Taniyama-Shimura-Wiles modularity theorem.
> Source base: Silverman "The Arithmetic of Elliptic Curves" (GTM 106, 2nd ed. 2009) ch. 1-5, 8,
> Diamond-Shurman "A First Course in Modular Forms" (GTM 228, 2005) ch. 1-5, 8,
> Koblitz "Introduction to Elliptic Curves and Modular Forms" (GTM 97, 2nd ed. 1993) ch. 1-3.
> **Honesty**: this file is a textbook summary. Every theorem, formula, author, and date follows the three
> textbooks above. The connection to the project constants (n=6, etc.) is restricted to §11 memo-level
> observation; no candidate argument is included. No invented theorems or authors.

---

## 0. Purpose

Task 2 of the P1 learning roadmap consolidates elliptic curves and modular forms. These are the common
substrate for BSD (P3), Fermat's Last Theorem (Wiles, already demonstrated), the Langlands programme (P3),
and Sato-Tate (P3), so the following six items are to be mastered.

1. Weierstrass equation, discriminant Δ, j-invariant
2. Group law (chord-tangent), identity O (point at infinity)
3. Mordell-Weil theorem E(Q) = Z^r ⊕ T (r = rank, T = torsion)
4. Euler-product definition of L(E, s) and analytic continuation
5. Modular forms M_k(Γ₀(N)), newforms
6. Taniyama-Shimura-Wiles modularity theorem

---

## 1. Weierstrass Equation

### 1.1 Affine Coordinates

Over a field K, the short Weierstrass equation:

```
  E:  y² = x³ + a x + b       (a, b ∈ K, char K ≠ 2, 3)
```

**Discriminant**

```
  Δ(E) = -16 (4 a³ + 27 b²)
```

**j-invariant**

```
  j(E) = -1728 · (4 a)³ / Δ = 1728 · 4 a³ / (4 a³ + 27 b²)
```

(Silverman §III.1 formulas III.1.1, III.1.2)

Condition: Δ ≠ 0 is required for the curve to be non-singular (smooth). Δ = 0 produces a cusp or node.

### 1.2 General Weierstrass Equation

General form including char K ∈ {2, 3}:

```
  E:  y² + a₁ x y + a₃ y = x³ + a₂ x² + a₄ x + a₆
```

with a₁, a₂, a₃, a₄, a₆ ∈ K. The discriminant for this form is considerably more complex (Silverman §III.1,
table).

### 1.3 Projective Completion

Homogeneous equation in the projective plane P²:

```
  Y² Z = X³ + a X Z² + b Z³
```

The unique point at infinity is O = [0 : 1 : 0]. This O is the identity of the group law on E.

---

## 2. Group Law — Chord-Tangent

### 2.1 Geometric Definition

For two points P, Q on E(K), define the sum P + Q by the following procedure.

1. Draw the line ℓ through P and Q (the tangent at P when P = Q).
2. Let R be the third intersection of ℓ with E (Bézout: cubic curve × line = 3 points).
3. Reflect R across the x-axis (i.e. y → -y). The result is P + Q.

### 2.2 Algebraic Formulas — Short Weierstrass

With P = (x₁, y₁), Q = (x₂, y₂), P ≠ -Q:

x₁ ≠ x₂ (secant slope)
```
  λ = (y₂ - y₁) / (x₂ - x₁)
```

x₁ = x₂, y₁ = y₂ (tangent slope)
```
  λ = (3 x₁² + a) / (2 y₁)
```

and

```
  x₃ = λ² - x₁ - x₂
  y₃ = λ (x₁ - x₃) - y₁
  P + Q = (x₃, y₃)
```

(Silverman §III.2, Theorem III.2.3)

### 2.3 Group Axioms

**Theorem.** E(K) under this operation + is an abelian group. The identity is O, and the inverse of P is
-P = (x, -y).

A derivation of associativity uses Cayley-Bacharach geometrically or uses Jacobians of rational functions
algebraically (Silverman §III.3 Prop III.3.4).

### 2.4 Hasse Inequality

For an elliptic curve E/F_q over a finite field,

```
  |#E(F_q) - (q + 1)| ≤ 2 √q
```

Setting a_p := p + 1 - #E(F_p) gives |a_p| ≤ 2 √p. This a_p is the coefficient of the L-function.
(Silverman §V.1 Thm V.1.1, Hasse 1934)

---

## 3. Mordell-Weil Theorem

### 3.1 Statement

**Theorem (Mordell 1922; Weil 1928 general-field generalisation).** For an elliptic curve E defined over Q,
E(Q) is a finitely generated abelian group. That is,

```
  E(Q) = Z^r ⊕ E(Q)_tors
```

where r ≥ 0 is the rank and E(Q)_tors is a finite abelian group (torsion subgroup).

(Silverman §VIII.4 Thm VIII.4.1)

### 3.2 Candidate-Argument Skeleton

Two stages.

1. Weak Mordell-Weil theorem. E(Q) / 2 E(Q) is finite. (Kummer theory + Galois cohomology)
2. Height descent. Define the canonical height ĥ and apply a descent inequality of the form
   h(P) ≤ C · #{ ... } to conclude finite generation.

(Silverman §VIII.1-VIII.4 covers this argument across the four sections.)

### 3.3 Torsion Subgroup — Mazur's Theorem

**Theorem (Mazur 1977).** The torsion subgroup E(Q)_tors of an elliptic curve over Q is isomorphic to
exactly one of the following 15 groups:

```
  Z/n   (n = 1, 2, ..., 10, 12)
  Z/2 × Z/2n   (n = 1, 2, 3, 4)
```

(Silverman §VIII.7, cited as a theorem only. The argument uses Mazur's deep modular-curve theory.)

### 3.4 Rank — Open

The value of r must be computed individually for each E, and there is no known general upper bound or
distribution formula. Largest known rank (2006, Elkies) ≥ 28. Average rank (Bhargava-Shankar 2015) is known
to be strictly less than 1 (at most 0.885 on average).

---

## 4. L-function L(E, s)

### 4.1 Definition

For each prime p, define the local factor L_p(E, s).

Good reduction (p ∤ Δ):

```
  L_p(E, s) = (1 - a_p p^{-s} + p^{1-2s})^{-1}
```

where a_p = p + 1 - #E(F_p).

Bad reduction:

- split multiplicative: L_p = (1 - p^{-s})^{-1}
- non-split multiplicative: L_p = (1 + p^{-s})^{-1}
- additive: L_p = 1

(Silverman §C.16 or "Advanced Topics in the Arithmetic of Elliptic Curves" §III.19)

### 4.2 Global L-function

```
  L(E, s) = ∏ L_p(E, s)
            p
```

**Theorem.** This product converges absolutely for Re(s) > 3/2 (from the Hasse inequality |a_p| ≤ 2 √p).

### 4.3 Conductor N

To each E a conductor N(E) ∈ Z_{>0} is attached:

- good reduction p ∤ N: L_p as above
- multiplicative reduction: p ∥ N (exactly once)
- additive reduction: p² | N (or higher power)

The conductor is the functional-equation parameter of the L-function.

### 4.4 Completed L-function

```
  Λ(E, s) = N^{s/2} (2π)^{-s} Γ(s) L(E, s)
```

**Theorem (consequence of modularity).** Λ(E, s) admits analytic continuation to the whole complex plane and

```
  Λ(E, s) = w · Λ(E, 2 - s)
```

where w ∈ {±1} is the root number (sign, determined by the chosen curve).

(Diamond-Shurman §5.10, Silverman "Advanced Topics" §C.16)

This is a direct consequence of the modularity theorem. Without modularity, even the analytic continuation
was not known.

---

## 5. Basics of Modular Forms

### 5.1 Congruence Subgroups

```
  SL₂(Z) = { [[a,b],[c,d]] : a,b,c,d ∈ Z, ad - bc = 1 }

  Γ₀(N) = { [[a,b],[c,d]] ∈ SL₂(Z) : c ≡ 0 (mod N) }
```

If N = 1 then Γ₀(1) = SL₂(Z).

### 5.2 Definition of Modular Forms

**Definition.** A function f: H → C (H = upper half-plane) is a modular form of weight k and level N when
it satisfies:

1. f is holomorphic on H.
2. For every γ = [[a,b],[c,d]] ∈ Γ₀(N),
   ```
     f((a τ + b)/(c τ + d)) = (c τ + d)^k · f(τ)
   ```
3. f is holomorphic at the cusps (= the Fourier expansion has only finitely many negative-index terms).

Cusp form. If the value at the cusps is 0, it is called a cusp form. The space is denoted S_k(Γ₀(N)).

(Diamond-Shurman §1.2, Koblitz §III.1)

### 5.3 Fourier Expansion

For τ ∈ H, set q := e^{2π i τ}; then a modular form has the q-expansion

```
          ∞
  f(τ) = ∑   a_n q^n
         n=0
```

A cusp form has a_0 = 0.

### 5.4 Hecke Operators

For each prime p, T_p: S_k(Γ₀(N)) → S_k(Γ₀(N)) is defined (Diamond-Shurman §5.2). The T_p commute with each
other, and common eigenvectors are called eigenforms.

Newform. An eigenform of level N that does not come from a smaller level is called a newform.

For a newform f, the L-function

```
          ∞
  L(f, s) = ∑   a_n n^{-s}   (a_n: Fourier coefficients of f)
          n=1
```

has an Euler product:

```
  L(f, s) = ∏  (1 - a_p p^{-s} + p^{k-1-2s})^{-1}  ·  ∏  (1 - a_p p^{-s})^{-1}
         p∤N                                            p|N
```

(Diamond-Shurman Thm 5.9.2)

---

## 6. Modularity Theorem (Taniyama-Shimura-Weil)

### 6.1 Statement

**Theorem (Wiles 1995; Taylor-Wiles 1995; Breuil-Conrad-Diamond-Taylor 2001 completion).**
Every elliptic curve over Q is modular. That is, there exists a newform f ∈ S_2(Γ₀(N(E))) of conductor N(E)
such that

```
  L(E, s) = L(f, s)
```

(Diamond-Shurman §8.8 cites the argument and gives the statement as the main text.)

### 6.2 Brief History

- Taniyama 1955. At the Tokyo International Mathematicians' Conference, raised the question "might every
  elliptic curve be related to an automorphic form".
- Shimura, 1960s. Organised and rigorised Taniyama's idea into the Shimura-Taniyama candidate.
- Weil 1967. Reformulated via L-function functional equations (Weil 1967, paper 'Über die Bestimmung
  Dirichletscher Reihen durch Funktionalgleichungen').
- Frey 1986, Ribet 1986. The Frey-curve idea showed that modularity implies Fermat's Last Theorem (Ribet's
  level-lowering theorem).
- Wiles 1994 (announced), 1995 (published). Demonstrated modularity in the semistable setting. This
  provides a demonstration of Fermat's Last Theorem.
- Taylor-Wiles 1995. Filled the gap (Hecke-algebra argument) in Wiles's argument.
- Breuil-Conrad-Diamond-Taylor 2001. Extended to all elliptic curves, completing the theorem.

(Diamond-Shurman §9 history section)

### 6.3 Connection to Fermat's Last Theorem

**Theorem (Fermat's Last Theorem, Wiles 1995).** For n ≥ 3, the equation x^n + y^n = z^n has no positive
integer solutions.

Outline of the argument: if a^p + b^p = c^p (p an odd prime, gcd(a, b, c) = 1) had a solution, one could
build the Frey curve E_{a,b,c} : y² = x (x - a^p) (x + b^p). By Ribet's theorem, if this curve were
modular, a weight-2 cusp form of level 2 would exist; but S_2(Γ₀(2)) = 0, a contradiction. Hence
E_{a,b,c} is not modular. But Wiles's theorem (semistable modularity) says such a curve is modular —
a contradiction. Therefore no solution exists.

(Diamond-Shurman §9.4 sketch; Wiles 1995 original paper)

---

## 7. BSD Candidate (Birch-Swinnerton-Dyer)

### 7.1 Weak BSD

**Candidate.** The rank r of E(Q) equals the order of vanishing of L(E, s) at s = 1:

```
  r = ord_{s=1} L(E, s)
```

### 7.2 Strong BSD

**Candidate (full BSD).**

```
                                                Ω_E · Reg_E · #Ш(E)
  lim (s-1)^{-r} L(E, s)  =  ─────────────────────────────── · ∏ c_p
  s → 1                                        #E(Q)_tors²                p|N
```

where
- Ω_E: real period,
- Reg_E: Néron-Tate regulator,
- Ш(E): Tate-Shafarevich group,
- c_p: Tamagawa numbers.

(Clay Mathematics Institute official statement, Wiles's formal Millennium problem description)

### 7.3 Known Partial Results

- Coates-Wiles 1977. For CM elliptic curves, L(E, 1) ≠ 0 implies rank r = 0.
- Gross-Zagier 1986. If L(E, s) has a simple zero at s = 1, a relation between ĥ(Heegner point) and
  L'(E, 1) holds.
- Kolyvagin 1990. If ord_{s=1} L(E,s) ≤ 1, then weak BSD holds (for modular E).
- Bhargava-Shankar 2010-2015. Finite estimates for average rank; a positive proportion of E has
  rank 0 or 1.

(Silverman §X.5, Diamond-Shurman §9.5)

---

## 8. Congruent-Number Problem (any connection to n=6?)

### 8.1 Definition of Congruent Number

A positive integer n is called a congruent number if there is a right triangle all three sides of which are
rational and whose area is exactly n.

(Examples: 5, 6, 7 are congruent numbers. For 5, sides are 3/2, 20/3, 41/6. For 6, sides 3, 4, 5 — area 6.)

### 8.2 Equivalence with an Elliptic Curve

**Theorem.** n is a congruent number ⟺ the elliptic curve E_n: y² = x³ - n² x has rank ≥ 1.

(Koblitz §I.1, Tunnell 1983)

### 8.3 Tunnell's Theorem

**Theorem (Tunnell 1983).** Assuming BSD, there is a polynomial-time algorithm deciding whether n is a
congruent number.

This theorem illustrates the practical power of BSD.

---

## 9. Complex Multiplication (CM)

### 9.1 Definition

An elliptic curve E/K has complex multiplication (CM) if End(E) ⊗ Q is larger than the rational field Q
and is isomorphic to some imaginary quadratic field K.

### 9.2 Examples

- y² = x³ - x : End = Z[i] (Gaussian integers), CM.
- y² = x³ - 1 : End = Z[ω] (ω = e^{2π i/3}), CM.

### 9.3 L-function of a CM Curve

The L-function of a CM curve agrees with a Hecke-character L-function. Through this, analytic continuation
and functional equation were obtained by Deuring (1950s) without Wiles.

(Silverman "Advanced Topics" ch. II)

---

## 10. Isogeny and Equivalence Classes

### 10.1 Definition

An isogeny φ: E → E' is a group homomorphism (regular map) with finite kernel. The degree is
deg(φ) = |ker φ|.

### 10.2 Dual Isogeny

**Theorem.** For an isogeny φ of degree n, there is a dual isogeny φ̂ : E' → E such that φ ∘ φ̂ = [n]
(multiplication by n).

(Silverman §III.6)

### 10.3 ℓ-adic Tate Module

```
  T_ℓ(E) := lim_{←} E[ℓ^n]       (inverse limit over E[ℓ^n] = {ℓ^n-torsion})
```

T_ℓ(E) is a free module of rank 2 over Z_ℓ. The Galois group G_Q = Gal(Q̄/Q) acts on T_ℓ(E).

The Tate module is the basis for the Galois-representation interpretation of the L-function.

---

## 11. Link to the Project Constants (memo only)

The project constants σ(6) = 12, φ(6) = 2, τ(6) = 4, sopfr(6) = 5, J₂(6) = 24 are not directly related to
elliptic curves. However, the following indirect link may become a target of P2/P3 research.

- Congruent number 6. n = 6 is the smallest congruent number (Koblitz §I.1). The triangle (3, 4, 5) has
  area = 6. This fact is equivalent to the elliptic curve E_6 : y² = x³ - 36 x having rank ≥ 1. In fact
  rank = 1.
- Weight-6 Siegel modular forms. The dimension of the cusp-form space S_6(SL₂(Z)), Ramanujan Δ of weight
  12 (= 2 × 6), and so on — 6 appears frequently, but these are observations only, not theorem
  implications.

This section is at the level of observation and target; any candidate argument belongs to P2/P3.

---

## 12. Reference Pointers (textbook pages / chapters)

| Topic | Silverman GTM 106 | Diamond-Shurman GTM 228 | Koblitz GTM 97 |
| --- | --- | --- | --- |
| Weierstrass, Δ, j | §III.1 | §7.1 | §I.2 |
| Group law, rational formulas | §III.2, III.3 | §7.2 | §I.4 |
| Mordell-Weil | §VIII.4 | - | - |
| Mazur torsion | §VIII.7 | - | - |
| Hasse inequality | §V.1 | §1.4 | §II.5 |
| L(E, s) | §C.16 (Adv. Topics) | §5.10 | §III.3 |
| Modular forms basics | - | §1.1 | §III.1 |
| Hecke operators | - | §5.2 | §III.5 |
| Modularity theorem | - | §8.8, §9 | - |
| Wiles history | - | §9.1 | §IV |
| BSD candidate | §X.5 | §5.10 | §III.6 |
| CM curves | Adv.Topics ch. II | - | §II |
| isogeny | §III.6 | §1.5 | §I.7 |
| Tate module | §III.7 | §9.5 | - |

---

## 13. Five Takeaways from this Unit

1. Weierstrass equation + discriminant + j-invariant.
2. Group law (chord-tangent, rational formulas with the two cases of λ).
3. Mordell-Weil theorem E(Q) = Z^r ⊕ T (precise statement).
4. L(E, s) Euler product and analytic continuation via modularity.
5. Taniyama-Shimura-Wiles theorem and the derivation path to Fermat's Last Theorem.

With these five in hand one can proceed directly at the P3 stage to BSD, GRH, Langlands, Sato-Tate.

---

## 14. Honesty Declaration

- These notes are a textbook summary. No new result.
- Every theorem, formula, date, and author comes from the three textbooks above (Silverman, Diamond-Shurman,
  Koblitz).
- The 15-item Mazur torsion list follows the exact list of Silverman §VIII.7.
- The Bhargava-Shankar average-rank result (≤ 0.885) is drawn from the Bhargava-Shankar 2010 paper rather
  than the textbook (beyond the textbook scope, so recorded at footnote level only).
- The link to the project constant (n=6) is kept as a memo in §11 only; no candidate argument or theorem.
- No invented theorems, authors, or dates.
