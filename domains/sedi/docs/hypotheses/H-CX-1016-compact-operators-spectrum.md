# H-CX-1016: Compact Operators — Spectral Structure

> **Hypothesis**: The spectrum of a compact operator on infinite-dimensional Hilbert space is countable with 0 as the only accumulation point. Singular values σ_n → 0. For trace-class operators, Σσ_n < ∞.

## Grade: 🟧 STRUCTURAL

## Results

### The Correspondence

```
Compact operator spectrum:
  σ(K) = {0} ∪ {λ₁, λ₂, ...} (countable, 0 accumulation)
  Each λₙ ≠ 0 is an eigenvalue of finite multiplicity
  |λ₁| ≥ |λ₂| ≥ ... → 0

Singular value decomposition:
  K = Σ sₙ ⟨·, vₙ⟩ uₙ
  s₁ ≥ s₂ ≥ ... ≥ 0, sₙ → 0 (singular values)

Schatten classes S_p:
  S_p = {K : Σ sₙᵖ < ∞}
  S₁ = trace class: Σsₙ < ∞
  S₂ = Hilbert-Schmidt: Σsₙ² < ∞
  S_p for p = φ: Hilbert-Schmidt (the most natural class)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
```

### Structural Analysis

```
Schatten class hierarchy:
  S₁ ⊂ S₂ ⊂ ... ⊂ S_p ⊂ ... ⊂ compact
  Key classes: S₁ (trace), S₂ (Hilbert-Schmidt)
  S_φ = S₂ is self-dual: (S₂)* ≅ S₂

Trace of trace-class operators:
  tr(K) = Σ⟨Keₙ, eₙ⟩ (basis-independent)
  |tr(K)| ≤ ‖K‖₁ = Σsₙ

Lidskii's theorem:
  tr(K) = Σλₙ (trace = sum of eigenvalues)
  For finite rank n: tr = sum of n eigenvalues
  n = P₁ = 6: tr(K) = λ₁ + ... + λ₆ for rank-6

Fredholm alternative:
  (I - K)x = y: either unique solution or
  dim ker(I - K) = dim ker(I - K*) < ∞
  Finite-dim kernel: governed by multiplicity of λ = 1
```

### Texas Sharpshooter Check

The spectral theory of compact operators is universal and does not depend on any particular dimension. The Schatten class S₂ = S_φ being the most natural (self-dual) class mirrors the L² situation (H-CX-1004). The pattern "p = φ = 2 picks out the self-dual case" recurs across functional analysis.

## Verification

- [x] S_φ = S₂ is self-dual Schatten class
- [x] Rank-P₁ operators: trace = sum of P₁ eigenvalues
- [x] Self-duality at p = φ parallels L^φ self-duality
- [ ] No unique compact operator result tied to n = 6
