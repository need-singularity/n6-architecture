# PURE-P0-3 — Introduction to Complex Analysis (Analytic Continuation / Residues / Gamma / ζ(2)=π²/6)

> Track: P0-PURE / Task 3
> Completion criterion: able to reconstruct one proof of ζ(2) = π²/6
> Source basis: Stein-Shakarchi "Complex Analysis" (2003), Ahlfors "Complex Analysis" 3rd ed. (1979), Conway "Functions of One Complex Variable I" 2nd ed. (1978), Titchmarsh "The Theory of the Riemann Zeta-Function" 2nd ed. (Heath-Brown 1986)
> **Honesty**: this file is a textbook summary. The ζ(2) = π²/6 proof is a modern reconstruction of Euler's 1735 original argument.

---

## 0. Goal and Scope

The goals of this P0 complex-analysis introduction are:

1. Analytic continuation principle
2. Residue theorem
3. Definition of the gamma function Γ(s) and its functional equation
4. Definition of the Riemann zeta function ζ(s) and an overview of its functional equation
5. **Proof of ζ(2) = π²/6** (reconstruction of Euler's sinc-product method, plus two alternatives)

(5) is the core of this note. Connections with n = 6 are recorded only as **observations** without causal claims.

---

## 1. Analytic Continuation Principle

### 1.1 Holomorphic Function

**Definition** (Stein-Shakarchi Ch. 1).

A complex function f : Ω → ℂ defined on an open set Ω ⊆ ℂ is **holomorphic** ⟺ at each point z₀ ∈ Ω the complex derivative

```
          f(z₀ + h) - f(z₀)
f'(z₀) = lim ─────────────────
        h→0        h
```

exists (h ∈ ℂ going to 0 from every direction with the same limit).

**Equivalent formulation (Cauchy-Riemann)**: For f(x+iy) = u(x,y) + iv(x,y),
- ∂u/∂x = ∂v/∂y
- ∂u/∂y = -∂v/∂x

### 1.2 Holomorphic ⟺ Power-series Expansion

**Theorem (Stein-Shakarchi Thm 2.4.4)**.

f is holomorphic on Ω ⟺ around each point z₀ f is expressible as a convergent power series f(z) = Σ_{n≥0} a_n (z - z₀)^n.

### 1.3 Identity Theorem

**Theorem** (Stein-Shakarchi Thm 2.4.6).

If f, g are holomorphic on a domain (connected open set) Ω, and f = g on a set with an **accumulation point** inside Ω, then f ≡ g on all of Ω.

**Corollary**: Analytic continuations are **unique**. If two continuations share a nonempty common domain, they agree there, forcing agreement beyond.

### 1.4 Examples of Analytic Continuation

- **Geometric series**: 1/(1-z) = Σ_{n≥0} z^n (convergent for |z| < 1) continues to all of ℂ \ {1} (same explicit formula).
- **ζ(s) = Σ n^{-s}**: defined for Re(s) > 1. Extends to the whole complex plane except s = 1 — see Section 5.

---

## 2. Residue Theorem

### 2.1 Isolated Singularities and Residues

If f is holomorphic on a punctured neighborhood of z₀ except at z₀, then z₀ is an **isolated singularity**. The Laurent expansion

```
f(z) = Σ_{n=-∞}^{∞} a_n (z - z₀)^n
```

has **residue** := a_{-1}. Notation Res(f, z₀).

### 2.2 Residue Theorem

**Theorem (Stein-Shakarchi Thm 3.2.1)**.

Let Ω ⊆ ℂ be simply connected and γ a positively oriented simple closed curve inside Ω. If f is holomorphic on Ω except for isolated singularities z₁, …, z_k that lie inside γ, then

```
∮_γ f(z) dz  =  2πi · Σ_{j=1}^{k} Res(f, z_j)
```

### 2.3 Applications to Real Analysis

**Example 1**: ∫_{-∞}^{∞} dx/(1+x²) = π. (f(z) = 1/(1+z²), upper half-plane semicircular contour, pole at z=i, Res = 1/(2i).)

**Example 2**: ∫_0^∞ (sin x)/x dx = π/2.

**Example 3 (P1 preview)**: The functional equation of the Riemann zeta function is derived via contour deformation and residues.

---

## 3. Gamma Function Γ(s)

### 3.1 Integral Definition

**Definition** (Stein-Shakarchi Ch. 6).

For Re(s) > 0,

```
          ∞
Γ(s) :=  ∫   t^{s-1} e^{-t} dt
          0
```

Absolute convergence verified for Re(s) > 0.

### 3.2 Functional Equation

**Theorem**. For Re(s) > 0,

```
Γ(s + 1) = s · Γ(s)
```

**Proof**. Integration by parts:
∫_0^∞ t^s e^{-t} dt = [-t^s e^{-t}]_0^∞ + s ∫_0^∞ t^{s-1} e^{-t} dt = 0 + s·Γ(s). □

### 3.3 At Positive Integers

Γ(1) = ∫_0^∞ e^{-t} dt = 1.
⟹ Γ(n) = (n-1)! (n ∈ ℕ, n ≥ 1).

### 3.4 Analytic Continuation

Γ(s+1) = s·Γ(s) ⟹ Γ(s) = Γ(s+1)/s extends to Re(s) > -1.
Iterating, Γ(s) extends to ℂ \ {0, -1, -2, -3, …}, with simple poles at 0, -1, -2, … having residue (-1)^n/n!.

### 3.5 Reflection Formula (Euler's)

**Theorem** (Stein-Shakarchi Thm 6.1.4).

```
Γ(s) · Γ(1 - s) = π / sin(πs)
```

**Result**: Γ(1/2) = √π.

### 3.6 Product Formula (Weierstrass)

```
1/Γ(s) = s · e^{γs} · ∏_{n≥1} (1 + s/n)·e^{-s/n}
```

(γ = Euler-Mascheroni constant.)

---

## 4. Riemann Zeta Function ζ(s)

### 4.1 Dirichlet-Series Definition

For Re(s) > 1,

```
         ∞
ζ(s) =   Σ   n^{-s}
         n=1
```

This series absolutely converges and is holomorphic for Re(s) > 1.

### 4.2 Euler Product (PURE-P0-1 §5)

```
ζ(s) = ∏_p (1 - p^{-s})^{-1}   (Re(s) > 1)
```

### 4.3 Analytic Continuation

**Theorem (Riemann 1859)**. ζ(s) continues analytically to **ℂ \ {1}**, with a simple pole at s=1 (residue 1).

**Sketch**: Relation with Γ(s)
```
Γ(s) ζ(s) = ∫_0^∞  t^{s-1} / (e^t - 1)  dt  (Re(s) > 1)
```
contour deformation (Hankel contour) continues to the left half-plane.

### 4.4 Functional Equation

**Theorem (Riemann 1859)**.

```
ξ(s) := (1/2) s (s-1) π^{-s/2} Γ(s/2) ζ(s)
```

defines an entire ξ satisfying ξ(s) = ξ(1 - s).

**Consequence**:
```
ζ(1 - s) = 2 (2π)^{-s} cos(πs/2) Γ(s) ζ(s)
```

### 4.5 Zeros and RH

- **Trivial zeros**: s = -2, -4, -6, … (negative even integers).
- **Non-trivial zeros**: inside the "critical strip" 0 < Re(s) < 1.
- **Riemann Hypothesis**: all non-trivial zeros lie on Re(s) = 1/2. **Unresolved**.

---

## 5. ζ(2) = π²/6: Reconstruction of Euler's Sinc-Product Proof

### 5.1 Goal

```
         ∞
ζ(2) =   Σ   1/n²   =   π²/6
         n=1
```

### 5.2 Preparation: Weierstrass / Hadamard Factorization

**Theorem** (Ahlfors Thm 5.8).

sin(z) is entire and its zeros are z = kπ (k ∈ ℤ). Hence the factorization

```
           ∞
sin(z) = z · ∏ (1 - z²/(kπ)²)
           k=1
```

**Note**: A plain ∏ (1 - z/(kπ)) does not converge. Pairing as **(1 - z²/(kπ)²) = (1 - z/(kπ))(1 + z/(kπ))** gives absolute convergence.

Scaling z → πx:

```
             ∞
sin(πx) = πx · ∏ (1 - x²/n²)     (Euler 1735)
             n=1
```

### 5.3 Divide Both Sides by x

```
                  ∞
sin(πx)/(πx) = ∏  (1 - x²/n²)
                n=1
```

**Taylor expansion** of the left side (for small |x|):

```
sin(πx) = πx - (πx)³/6 + (πx)⁵/120 - ⋯
sin(πx)/(πx) = 1 - (πx)²/6 + (πx)⁴/120 - ⋯
             = 1 - (π²/6) x² + (π⁴/120) x⁴ - ⋯
```

### 5.4 Right-hand-side Expansion (x² Coefficient)

```
∞
∏ (1 - x²/n²) = 1 - (Σ_{n=1}^∞ 1/n²) · x² + (higher x⁴, x⁶, …)
n=1
```

(The x² coefficient of the infinite product equals the sum of picking exactly one x²/n² from each factor = -Σ 1/n². Higher terms come from picking pairs.)

### 5.5 Coefficient Comparison

Compare x² coefficients:

**Left-hand coefficient**: -π²/6.
**Right-hand coefficient**: -Σ_{n=1}^∞ 1/n² = -ζ(2).

Two holomorphic functions equal as power series in x ⟹ every coefficient matches (identity theorem, §1.3):

```
-π²/6 = -ζ(2)
⟹ ζ(2) = π²/6   □
```

### 5.6 Rigor Check

1. **Absolute convergence of the product for sin(πx)?** — Yes. |x²/n²| < 1/2 for n ≥ N, and Σ log(1 - x²/n²) converges absolutely via log expansion. (Ahlfors §5.2)

2. **Is coefficient comparison allowed?** — Yes. Both sides are holomorphic near x = 0 (removable singularity at zero), and Taylor coefficients are unique.

3. **Proof of Weierstrass factorization itself** is left to the reference texts cited in §5.2.

### 5.7 Getting ζ(4) from the x⁴ Coefficient

**Bonus** (same method):

LHS x⁴ coefficient: π⁴/120.
RHS x⁴ coefficient: Σ_{m<n} 1/(m²n²) = (1/2)[(Σ 1/n²)² - Σ 1/n⁴] = (1/2)[ζ(2)² - ζ(4)].

Hence π⁴/120 = (1/2)(ζ(2)² - ζ(4)) = (1/2)(π⁴/36 - ζ(4))
⟹ ζ(4) = π⁴/36 - π⁴/60 = π⁴ (5/180 - 3/180) = π⁴·(2/180)·... computing

Exact: ζ(4) = **π⁴/90**.

(Result recorded in Euler's original 1735 paper.)

---

## 6. Alternative Proofs (Mentioned)

### 6.1 Cauchy Cotangent-expansion Method

**Idea**: cot(πz) has poles at z = k (k ∈ ℤ) with residue 1/π at each. The contour integral

```
∮_C (π cot(πz)) · f(z) dz
```

over a large rectangle C computes Σ_{k} f(k) via residues.

**Application**: With f(z) = 1/z², z=0 is a double pole; computing the residue produces ζ(2). Conclusion ζ(2) = π²/6.

**Strength**: Fully complex-analytic, uses only the residue theorem.
**Reference**: Ahlfors §5.2.3, Titchmarsh §4.5.

### 6.2 Fourier-series Method

**Idea**: Fourier series of f(x) = x on [−π, π]:

```
x = 2 Σ_{n≥1} ((-1)^{n+1}/n) sin(nx)
```

By Parseval's identity,

```
(1/π) ∫_{-π}^{π} x² dx = Σ_{n≥1} (2/n)²
⟹ 2π²/3 = 4 · ζ(2)
⟹ ζ(2) = π²/6
```

**Strength**: Purely real-analytic, no complex analysis needed.
**Reference**: Stein-Shakarchi *Fourier Analysis*, §3.

### 6.3 Double-integral Method (Apostol / Beukers)

```
∫∫_{[0,1]²}  dx dy / (1 - xy)  =  ζ(2) = π²/6
```

Direct computation of the left side gives π²/6. Beukers used this method to prove the irrationality of ζ(3).

**Reference**: Apostol *Mathematical Analysis*, 2nd ed.

---

## 7. n = 6 Observation (Honest Independence)

### 7.1 The 6 in ζ(2) = π²/6

ζ(2) = π²/**6**. A 6 appears in the denominator. It arises

- in the proof (§5) directly from the fact that the Taylor expansion of sinc has **x³ coefficient = -1/6**;
- hence a structural consequence of "3! = 6", **different** from the number-theoretic n = 6 (completeness in R1 theorem).

**Honest claim**: The 6 in ζ(2) and the n = 6 of the PURE-P0-1 R1 theorem **do not come from the same structure**. The 6 in R1 comes from multiplicativity of σ, φ, τ; the 6 in ζ(2) comes from the Taylor expansion of sin. Asserting that the two 6s share a cause requires a separate argument (currently absent).

### 7.2 Analogous Observations

- ζ(4) = π⁴/**90**. Here 90 = 9·10 = 2·3²·5. No particular 6 structure.
- ζ(6) = π⁶/**945**. 945 = 3³·5·7.
- ζ(2k) = (-1)^{k+1} · (2π)^{2k} · B_{2k} / (2·(2k)!) (Euler).

These formulas are determined by the Bernoulli numbers B_{2k} and (2k)!. The appearance of 6 comes from 2! = 2 at k=1 and the overall normalization (2π)²/(2·2) = π², yielding π²/6.

### 7.3 Caution (feedback_honest_verification)

Attempts to derive the 6 of the R1 theorem from the 6 of ζ(2) are **pattern-matching bias**. The fact that both mention 6 is an observation; a structural connection is a **conjecture** until proven.

---

## 8. Honesty Check

### 8.1 Things Not Claimed by This Note

- **Strict proof** of the Weierstrass factorization is left to reference texts.
- **Full proof** of the Riemann functional equation is left to reference texts (Titchmarsh §2.6).
- A closed form for ζ(2k+1) (odd indices) — **currently unknown**. Apéry 1979 proved only the **irrationality** of ζ(3).

### 8.2 sopfr=5 Bias Caution

ζ(2) = π²/6 and sopfr observations are unrelated. This note does not use sopfr.

### 8.3 Seven-Millennium-problem Solved Count

Number of Millennium problems solved by this note: **0 / 7**.

(In particular RH is explicitly marked **unresolved** in §4.5.)

---

## 9. References

1. E. M. Stein, R. Shakarchi, *Complex Analysis*, Princeton Univ. Press, 2003. (Ch. 1-3, 5-6)
2. L. V. Ahlfors, *Complex Analysis*, 3rd ed., McGraw-Hill, 1979. (Ch. 5)
3. J. B. Conway, *Functions of One Complex Variable I*, 2nd ed., Springer, 1978.
4. E. C. Titchmarsh, *The Theory of the Riemann Zeta-Function*, 2nd ed. (ed. D. R. Heath-Brown), Oxford, 1986.
5. L. Euler, "De summis serierum reciprocarum", *Comm. Acad. Sci. Petrop.* 7 (1735), 123–134. (Original paper for ζ(2) = π²/6)
6. B. Riemann, "Über die Anzahl der Primzahlen unter einer gegebenen Grösse", *Monatsber. Berliner Akad.* (1859).
7. R. Apéry, "Irrationalité de ζ(2) et ζ(3)", *Astérisque* 61 (1979).
8. F. Beukers, "A note on the irrationality of ζ(2) and ζ(3)", *Bull. London Math. Soc.* 11 (1979).

---

**Written for**: P0-PURE track / Task 3
**Status**: textbook summary complete; ζ(2) = π²/6 via Euler sinc reconstruction included (3 alternatives mentioned)
**Next**: P0-PURE comprehensive review → P1 (serious entry into analytic number theory)
