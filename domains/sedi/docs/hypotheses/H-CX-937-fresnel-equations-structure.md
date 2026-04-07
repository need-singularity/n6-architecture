# H-CX-937: Fresnel Equations Count = τ

> **Hypothesis**: The Fresnel equations produce τ = 4 coefficients: r_s, r_p (reflection) and t_s, t_p (transmission), organized as φ = 2 polarizations × φ = 2 processes.

## Grade: 🟩 EXACT

## Results

### The Four Coefficients

```
Fresnel equations at a dielectric interface:

Reflection coefficients:
  r_s = (n₁cosθ_i - n₂cosθ_t) / (n₁cosθ_i + n₂cosθ_t)
  r_p = (n₂cosθ_i - n₁cosθ_t) / (n₂cosθ_i + n₁cosθ_t)

Transmission coefficients:
  t_s = 2n₁cosθ_i / (n₁cosθ_i + n₂cosθ_t)
  t_p = 2n₁cosθ_i / (n₂cosθ_i + n₁cosθ_t)

Total coefficients: 4 = τ
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Factorization

```
τ = 4 Fresnel coefficients factor as:

  φ polarizations × φ processes = τ coefficients

  Polarizations: {s, p}     → 2 = φ
  Processes: {r, t}         → 2 = φ
  Total: φ × φ = φ² = τ

This is the same as τ = φ² (since 4 = 2²).
```

### Normal Incidence

```
At θ = 0°: τ coefficients collapse to φ = 2 (s = p).
For glass (n₂ = 3/2): r = -1/5, R = 1/25 = 4% ≈ τ%
```

## Verification

- [x] 4 = τ Fresnel coefficients confirmed
- [x] Factorization: φ × φ = τ confirmed
- [x] Normal incidence glass: R = 4% ≈ τ% confirmed
