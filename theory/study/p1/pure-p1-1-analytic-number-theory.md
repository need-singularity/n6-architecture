# PURE-P1-1 — Analytic Number Theory (zeta function / functional equation / Perron-Mellin / explicit formula)

> Track: P1-PURE / Task 1
> Completion criterion: be able to derive the ξ(s) functional equation ξ(s) = ξ(1-s) via the θ-series / Mellin route,
> and to walk line by line through the explicit formula for ψ(x):
> ψ(x) = x - Σ_ρ x^ρ/ρ - ln(2π) - ½ ln(1 - x^{-2}).
> Source base: Iwaniec-Kowalski "Analytic Number Theory" (AMS Colloquium Publ. 53, 2004) ch. 5,
> Edwards "Riemann's Zeta Function" (Academic Press, 1974) ch. 1-3,
> Titchmarsh-Heath-Brown "The Theory of the Riemann Zeta-Function" (2nd ed., 1986) ch. 2, ch. 3.
> **Honesty**: this file is a textbook summary. It contains no new results. Every theorem and formula is
> reconstructed from the three textbooks above, and the derivation paths (including Riemann's 1859 paper)
> and page numbers follow each textbook edition. No invented theorems, authors, or dates are included.

---

## 0. Purpose and Scope

The first task of the Millennium P1 learning roadmap is to gain hands-on familiarity with the core tools of
analytic number theory. To then handle ζ-based problems such as RH, BSD, Langlands, GRH in stages P2-P3,
the following six foundations must be in place first.

1. Extension of the domain of definition of Riemann ζ(s) (analytic continuation)
2. Derivation of the functional equation ξ(s) = ξ(1-s) (θ-series + Mellin + reflection)
3. Meaning of the trivial zeros (s = -2, -4, -6, ...)
4. Non-trivial zeros (critical strip 0 < Re(s) < 1) and the RH hypothesis
5. Chebyshev ψ(x) and the explicit formula
6. Perron formula, Mellin transform — a bridge between sums and contour integrals

These notes are not directly related to the project constants (n=6, σ=12, φ=2, τ=4, sopfr=5, J₂=24).
They are purely a summary of analytic number theory. However, since the σ·φ = n·τ ⟺ n=6 candidate itself
connects to ζ-zero distributions at stages P2 and P3 via Dirichlet series such as
∑ σ(n)/n^s = ζ(s)ζ(s-1), a brief memo is left in §9 only (the argument itself is deferred to the P2
document).

---

## 1. The ζ Function and Analytic Continuation

### 1.1 Dirichlet Series Definition

For Re(s) > 1,

```
          ∞   1
  ζ(s) = ∑  ─────   (s = σ + it, σ > 1)
         n=1  n^s
```

This series is absolutely convergent, uniformly convergent on compact subsets, and analytic on each compact
subset of the half-plane σ = Re(s) > 1 (Weierstrass M-test).

### 1.2 Euler Product (Re(s) > 1)

```
           ┌       1     ┐
  ζ(s) = ∏ │ ──────────  │      (p over all primes)
         p └  1 - p^{-s} ┘
```

Derivation: (1 - p^{-s})^{-1} = ∑_{k=0}^∞ p^{-ks}. Multiply over all primes p and use the fundamental
theorem of arithmetic (unique factorisation of every positive integer) to obtain ∑ 1/n^s.

### 1.3 From σ > 1 to σ > 0 (first extension)

(1 - 2^{1-s}) ζ(s) = ∑_{n=1}^∞ (-1)^{n-1} / n^s = η(s) (Dirichlet η, alternating series) converges for
σ > 0. Hence

```
  ζ(s) = η(s) / (1 - 2^{1-s})
```

extends ζ analytically to σ > 0, s ≠ 1 + 2π i k / ln 2. This is the first-step extension in Hardy's style,
insufficient to obtain the full functional equation, but meaningful since the right-hand side η converges.

(Source: Titchmarsh §2.1)

### 1.4 Simple Pole at s = 1

ζ(s) has a simple pole of residue 1 at s = 1:

```
  ζ(s) = 1/(s-1) + γ + O(s-1)       (s → 1)
```

where γ = 0.5772156... is the Euler-Mascheroni constant. Derivation: write ζ(s) - 1/(s-1) as an integral
representation and take the limit s → 1 (Edwards §1.3).

---

## 2. θ-series and Mellin Transform

### 2.1 Jacobi θ Function

```
          ∞
  θ(x) = ∑  e^{-π n² x}       (x > 0)
        n=-∞
```

i.e. θ(x) = 1 + 2 ∑_{n=1}^∞ e^{-π n² x}.

### 2.2 θ Transformation Formula (Poisson Summation)

**Theorem (Jacobi).** For x > 0,

```
  θ(x) = x^{-1/2} · θ(1/x)
```

**Proof sketch.** The Fourier transform of f(y) = e^{-π y² x} is f̂(ξ) = x^{-1/2} e^{-π ξ² / x}.
Substitute into the Poisson summation formula ∑_{n∈Z} f(n) = ∑_{k∈Z} f̂(k).

(Source: Iwaniec-Kowalski §5.2, Edwards §1.5)

### 2.3 Mellin Transform

For f: (0,∞) → C satisfying suitable decay conditions,

```
                  ∞
  M[f](s) := ∫    f(x) x^{s-1} dx
                 0
```

is called the Mellin transform.

Example. f(x) = e^{-x} gives M[f](s) = Γ(s) (Re s > 0).

### 2.4 Integral Representation of ζ

From Γ(s/2) π^{-s/2} n^{-s} = ∫₀^∞ x^{s/2 - 1} e^{-π n² x} dx, summing over n (σ > 1),

```
                                      ∞
  π^{-s/2} Γ(s/2) ζ(s) = ∫         x^{s/2-1} · ψ(x) dx
                                      0
```

where ψ(x) := ∑_{n=1}^∞ e^{-π n² x} = (θ(x) - 1)/2 (notation local to this chapter; this ψ clashes with
Chebyshev ψ in the next chapter, but here it comes from θ).

---

## 3. Functional Equation ξ(s) = ξ(1-s)

### 3.1 Riemann's Method (1859)

(Reconstruction following Edwards §1.6, Iwaniec-Kowalski §5.3)

Split the integral into x ∈ (0,1) and x ∈ (1,∞).

```
                                         1                      ∞
  π^{-s/2} Γ(s/2) ζ(s) = ∫ x^{s/2-1} ψ(x) dx + ∫ x^{s/2-1} ψ(x) dx
                                        0                      1
```

In the first integral, substitute x → 1/x and apply the θ transformation formula θ(x) = x^{-1/2} θ(1/x),
i.e. 2 ψ(x) + 1 = x^{-1/2} (2 ψ(1/x) + 1), to obtain

```
  ∫₀¹ x^{s/2-1} ψ(x) dx
     = ∫₁^∞ x^{-s/2} · (x^{1/2} ψ(x) + ½ x^{1/2} - ½) · dx/x
     = ∫₁^∞ x^{(1-s)/2 - 1} ψ(x) dx + ½ · (1/(s-1) - 1/s)
```

Therefore

```
  π^{-s/2} Γ(s/2) ζ(s) = 1/(s(s-1))
                        + ∫₁^∞ [x^{s/2-1} + x^{(1-s)/2 - 1}] ψ(x) dx
```

The second term on the right is symmetric in s and 1-s. The left-hand side ζ(s) is analytically defined by
this formula for all s (except s=0,1).

### 3.2 Definition of ξ(s)

```
  ξ(s) := ½ s(s-1) π^{-s/2} Γ(s/2) ζ(s)
```

Setting this up makes s(s-1) cancel the poles, so ξ(s) becomes entire. Using the symmetric form of §3.1 we
immediately obtain the following.

**Theorem (functional equation).**

```
  ξ(s) = ξ(1-s)
```

(Source: Iwaniec-Kowalski Thm 5.3, Edwards §1.6, Titchmarsh §2.6)

### 3.3 Geometric Meaning of the Symmetry

s ↦ 1-s is the reflection across the line Re(s) = 1/2. The zeros of ξ(s) are distributed symmetrically with
respect to this axis. RH is the claim that all non-trivial zeros lie on this axis.

### 3.4 Alternative Form of the Functional Equation

Using the reflection and duplication formulas for Γ,

```
  ζ(s) = 2^s π^{s-1} sin(πs/2) Γ(1-s) ζ(1-s)
```

(this form: Edwards §1.7, Titchmarsh §2.1). Substituting s = -2k (k=1,2,3,...) into this form, sin(-kπ) = 0
meets the pole of Γ(1-s) and what remains is ζ(-2k) = 0 (k ≥ 1). These are the trivial zeros.

---

## 4. Distribution of Zeros

### 4.1 Trivial Zeros

```
  ζ(-2) = ζ(-4) = ζ(-6) = ... = 0
```

follows immediately from the functional equation. Note that ζ(0) = -1/2, not 0 (since at s=0 the pole of
Γ(s/2) is cancelled by the factor s(s-1) of ξ, giving ξ(0) a finite value rather than 0).

In fact Γ(s/2) has a pole at s=0, and s(s-1) cancels it exactly.

### 4.2 Non-Trivial Zeros — Critical Strip

Definition. The critical strip is 0 < Re(s) < 1.

**Theorem (Hadamard, de la Vallée Poussin 1896).** There are no zeros of ζ(s) on Re(s) = 1.

By the functional equation there are also no zeros on Re(s) = 0. Hence all non-trivial zeros lie in
0 < Re(s) < 1.

**Riemann Hypothesis (RH, 1859).** Every non-trivial zero ρ satisfies Re(ρ) = 1/2.

(Open; Clay Millennium problem.)

### 4.3 Number of Zeros on the Critical Line

**Theorem (Hardy 1914).** There are infinitely many zeros of ζ on Re(s) = 1/2.

**Theorem (Selberg 1942).** The number of zeros on this line accounts for a positive proportion.

**Theorem (Levinson 1974; Conrey 1989 improvement).** At least 40% of the non-trivial zeros lie on the
critical line (Conrey reaches 40.58%).

These results are not candidate resolutions of RH but quantify the "many" statement.

### 4.4 Approximation of the Zero Count — Riemann-von Mangoldt

The number N(T) of non-trivial zeros with 0 < Im(ρ) < T satisfies

```
  N(T) = (T/2π) ln(T/2π) - T/2π + O(ln T)
```

(Source: Titchmarsh §9.4, Edwards §6.7)

---

## 5. Chebyshev ψ(x) and Its Identity

### 5.1 Definition

```
                          ∞
  ψ(x) := ∑  Λ(n) = ∑    ∑   ln p · [p^k ≤ x]
         n≤x         p   k=1
```

where the von Mangoldt function is

```
  Λ(n) = { ln p,  n = p^k (k ≥ 1)
         { 0,     otherwise
```

### 5.2 Equivalence with PNT

**Theorem.** The following three are equivalent.

```
  π(x) ~ x/ln x   ⟺   θ(x) ~ x   ⟺   ψ(x) ~ x
```

where θ(x) = ∑_{p≤x} ln p (Chebyshev θ).

(Source: Apostol §4.5, Iwaniec-Kowalski §2.4)

---

## 6. Perron Formula

### 6.1 Statement

**Theorem (Perron).** If a(n) is an arithmetic function and f(s) = ∑ a(n)/n^s is absolutely convergent at
σ = σ₀, then, for non-integer x,

```
                                 c + i∞
                         1
  ∑  a(n) = ───────── · ∫       f(s) · x^s / s · ds
 n ≤ x        2π i
                                c - i∞
```

where c > σ₀ is a real number greater than the abscissa of convergence. (Source: Iwaniec-Kowalski Thm 5.1,
Titchmarsh §3.12)

### 6.2 Truncated Version

For numerical use, truncate at height T and attach an error term:

```
                       c + iT
                1
  ∑ a(n) = ────── · ∫       f(s) · x^s / s · ds + O(error)
 n ≤ x       2π i
                      c - iT
```

The error term depends on the sizes of a(n) and f(s), on T, and on x.

### 6.3 Application to ζ

Taking a(n) = Λ(n), f(s) = -ζ'(s)/ζ(s) in Perron,

```
                          c + i∞
                   1                       x^s
  ψ(x) = ──── ∫      (-ζ'(s)/ζ(s)) · ─── ds
                  2πi                        s
                         c - i∞
```

with c > 1.

### 6.4 Contour Shift — Residue Harvesting

Move the integration contour from c = 1 + ε to the left (say Re(s) = -A), collecting residues at the
singular points traversed.

- s = 1 (pole of ζ): residue = x^1 · 1 = x
- s = 0 (pole of 1/s): residue = -ζ'(0)/ζ(0) · x^0 = -ζ'(0)/ζ(0) = ln(2π)
- s = ρ (non-trivial zero of ζ, critical strip): residue = -x^ρ/ρ
- s = -2k (trivial zero, k = 1, 2, ...): residue = -x^{-2k} / (-2k) = x^{-2k}/(2k)

Summing the trivial-zero contributions gives ½ ∑_{k=1}^∞ x^{-2k}/k = -½ ln(1 - x^{-2}).

---

## 7. Explicit Formula

### 7.1 Final Form

Summing all harvested residues,

```
  ψ(x) = x - ∑  x^ρ/ρ - ln(2π) - ½ ln(1 - x^{-2})
             ρ
```

where ρ ranges over all non-trivial zeros of ζ, and x > 1 (midpoint convention if x is a prime power).

**Source.** Iwaniec-Kowalski §5.5, Edwards §3.2-3.3, Titchmarsh §3.5. This identity is called the
von Mangoldt explicit formula (a ψ-version of Riemann's 1859 formula for π(x), rigorised by von Mangoldt
1895).

### 7.2 Interpretation

- The main term x is the backbone of PNT: "on average, ψ(x) ≈ x".
- ∑_ρ x^ρ/ρ is a corrective oscillation term. Each zero ρ = β + iγ contributes a wave with amplitude
  x^β/|ρ| and period (2π/ln x · γ).
- -ln(2π) is a constant correction.
- -½ ln(1 - x^{-2}) is the trivial-zero contribution (→ 0 as x → ∞).

Under RH, every ρ has β = 1/2, so |x^ρ/ρ| ≤ x^{1/2}/|ρ|, hence

```
  ψ(x) = x + O(x^{1/2} · ln² x)   (conditional on RH)
```

### 7.3 Unconditional Error Known Without RH

Unconditionally (without RH),

```
  ψ(x) = x + O(x exp(-c √(ln x)))   (some c > 0)
```

follows from the zero-free region of Hadamard-de la Vallée Poussin 1896.

(Source: Iwaniec-Kowalski §5.7, Titchmarsh §3.9)

### 7.4 Riemann's Original 1859 Form

Riemann himself stated a π(x) (prime-counting function) version:

```
                        ∞
  J(x) := ∑  (1/k) π(x^{1/k})
              k=1

  J(x) = Li(x) - ∑ Li(x^ρ) + ∫_x^∞ dt/(t(t²-1) ln t) - ln 2
                   ρ
```

Later π(x) is recovered via π(x) = ∑_{k=1}^∞ (μ(k)/k) J(x^{1/k}). (Edwards §1.17)

---

## 8. Mellin Transform and Multiplicative Structure

### 8.1 Mellin Transform Properties

| Operation | Transform |
| --------- | --------- |
| f(ax) | a^{-s} · M[f](s) |
| x^a · f(x) | M[f](s + a) |
| f'(x) | -(s-1) · M[f](s-1) (conditional on integration by parts) |
| ∫₀^x f(t) dt | -(1/s) · M[f](s+1) (conditional on convergence) |

### 8.2 Convolution Theorem

```
  M[f ⋆ g](s) = M[f](s) · M[g](s)      (⋆: multiplicative convolution)
```

where (f ⋆ g)(x) := ∫₀^∞ f(x/t) g(t) dt/t.

### 8.3 Example — Dirichlet Series as Mellin

For f(x) = ∑ a_n · [0,1]_n(x) (step function),

```
  M[f](s) = ∑ a_n / n^s · (...)
```

emerges, so the Perron formula is naturally interpreted as an inverse Mellin transform.

---

## 9. Link to the n=6 Candidate (assigned to P2 and P3)

These are P1 study notes; the candidate argument for σ·φ=n·τ ⟺ n=6 is handled in P0 (foundations) and
again at P2 (analytic approach). Only the connection points are noted here.

- Dirichlet series. For Re(s) > 2, ∑ σ(n)/n^s = ζ(s)ζ(s-1), ∑ φ(n)/n^s = ζ(s-1)/ζ(s),
  ∑ τ(n)/n^s = ζ(s)². (Apostol Thm 11.5, 11.7)
- n and τ. ∑ n τ(n)/n^s = ∑ τ(n)/n^{s-1} = ζ(s-1)² (s ↦ s-1).
- Multiplicative structure. The Dirichlet series of σ·φ generally has no simple closed form, but whether
  the uniqueness of n=6 can be reconstructed as an analytic candidate argument by comparing values at
  special points s is a target of P2 research. (Only the statement is left here; the argument is not
  included.)

---

## 10. Reference Pointers (textbook pages / chapters)

| Topic | Iwaniec-Kowalski | Edwards | Titchmarsh |
| --- | --- | --- | --- |
| ζ Euler product | Thm 1.1 | §1.3 | §1.1 |
| Analytic continuation (first extension) | §5.1 | §1.4 | §2.1 |
| θ transformation | §5.2 | §1.5 | §2.5 |
| Functional equation ξ(s)=ξ(1-s) | Thm 5.3 | §1.6 | §2.6 |
| Trivial zeros | §5.3 | §1.9 | §2.12 |
| Perron formula | Thm 5.1 | §3.1 | §3.12 |
| Explicit formula | §5.5 | §3.2-3.3 | §3.5 |
| Re(s)=1 zero-free | §5.4 | §3.5 | §3.6 |
| Riemann-von Mangoldt N(T) | §5.4 | §6.7 | §9.4 |

---

## 11. Five Takeaways from this Unit

1. Define ζ(s) for σ > 1 — Dirichlet series ∑ n^{-s}
2. θ-series transformation formula — θ(x) = x^{-1/2} θ(1/x)
3. ξ(s) = ξ(1-s) functional equation — ½ s(s-1) π^{-s/2} Γ(s/2) ζ(s) is symmetric under s↔1-s
4. Three classes of zeros — trivial (ζ(-2k)=0) / non-trivial (critical strip) / RH location (Re=1/2)
5. ψ(x) explicit formula — ψ(x) = x - ∑_ρ x^ρ/ρ - ln(2π) - ½ ln(1-x^{-2})

With these five in hand one can proceed to the next P1 stage — elliptic curve L-functions, NS energy
inequality, Hodge-de Rham, Yang-Mills path integrals, exotic spheres, NP-completeness.

---

## 12. Further Reading (within the textbooks)

- Appendix of Edwards "Riemann's Zeta Function": English translation of Riemann's 1859 paper
- Iwaniec-Kowalski ch. 6: zero-density theorems, prime distribution in short intervals
- Titchmarsh ch. 10: Riemann-Siegel formula (for numerical ζ)
- Bombieri "The Riemann Hypothesis — official Clay statement" (Clay Institute web page)

---

## 13. Honesty Declaration

- These notes are a textbook summary. No new result.
- Formulas and theorems are taken from the three textbooks above. Page and chapter numbers follow each
  edition (year).
- Numerical constants (Euler γ ≈ 0.5772..., Conrey 40.58%, etc.) are quoted as in the textbooks.
- Project constants (n=6, etc.) appear only as link memos in §9; no candidate argument is included.
- No invented theorems, authors, or dates. Anything uncertain is explicitly marked (open) or (conditional).
