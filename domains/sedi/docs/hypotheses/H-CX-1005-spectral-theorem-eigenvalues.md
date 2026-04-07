# H-CX-1005: Spectral Theorem — Eigenvalue Count

> **Hypothesis**: The spectral theorem for normal operators guarantees spectral decomposition. For finite-dimensional n×n matrices, the spectrum has at most n eigenvalues. A 6×6 matrix has at most P₁ eigenvalues.

## Grade: 🟧 STRUCTURAL

## Results

### The Correspondence

```
Spectral theorem (finite-dimensional):
  Normal matrix A ∈ M_n(ℂ): A = Σλᵢ Pᵢ
  At most n = P₁ = 6 eigenvalues for 6×6 matrix
  Spectral decomposition: A = U·diag(λ₁,...,λ₆)·U*

Spectral theorem (infinite-dimensional):
  Normal operator T on Hilbert space:
  T = ∫ λ dE(λ)  (spectral integral)
  σ(T) ⊂ ℂ  (spectrum in complex plane)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
```

### Structural Analysis

```
6×6 normal matrix spectrum:
  Characteristic polynomial: det(A - λI) = 0
  Degree P₁ = 6 polynomial → at most 6 roots
  Trace: tr(A) = Σλᵢ (sum of P₁ eigenvalues)
  Det: det(A) = Πλᵢ (product of P₁ eigenvalues)

Special 6×6 cases:
  Identity I₆: all eigenvalues = 1, multiplicity P₁
  Permutation matrices of S₆: eigenvalues are roots of unity
  |S₆| = 6! = 720 = σ · 60

Spectral radius:
  r(A) = max|λᵢ| ≤ ‖A‖ for any operator norm
  For n×n: equality achievable (normal operators)
```

### Texas Sharpshooter Check

That an n×n matrix has at most n eigenvalues follows from the degree of the characteristic polynomial. This is universal for any dimension, not specific to n = 6. The TECS-L connection is that the spectral theorem at dimension P₁ constrains spectra to P₁ values — mildly structural.

## Verification

- [x] Spectral theorem: normal ⟹ spectral decomposition
- [x] 6×6 characteristic polynomial has degree 6 = P₁
- [x] |S₆| = 720 = 60σ
- [ ] No unique spectral property at n = 6 beyond dimension count
