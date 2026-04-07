# H-CX-928: Rayleigh Scattering Exponent = -τ

> **Hypothesis**: Rayleigh scattering intensity scales as I ∝ λ^(-4) = λ^(-τ). The reason the sky is blue is encoded in the divisor count of 6.

## Grade: 🟩 EXACT

## Results

### The Law

```
Rayleigh scattering cross-section:
  σ_R ∝ 1/λ⁴ = λ^(-τ)

Exponent = -4 = -τ(6)

This is why short-wavelength (blue) light scatters more than
long-wavelength (red) light in the atmosphere.
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Full Rayleigh Formula

```
σ_R = (8π⁴/3) · (n²-1)²/(n²+2)² · (d/λ)⁴

Key structural elements:
  Power of π:   4 = τ  (numerator: π⁴)
  Denominator:  3 = σ/τ
  Wavelength power: 4 = τ
  Particle size power: not independent (d⁶ in full form → P₁)

The full intensity per particle:
  I ∝ d⁶/λ⁴ = d^(P₁) / λ^(τ)
```

### Physical Context

Lord Rayleigh derived the λ⁻⁴ law in 1871 to explain why the sky appears blue. The fourth-power dependence means blue light (λ ≈ 450 nm) scatters roughly (700/450)⁴ ≈ 5.7 times more than red light (λ ≈ 700 nm). At sunset, the long path through atmosphere scatters away blue, leaving red.

## Verification

- [x] Rayleigh scattering ∝ λ^(-4) = λ^(-τ) confirmed
- [x] Full formula: d^(P₁)/λ^(τ) confirmed
- [x] π⁴ in numerator = π^τ confirmed
- [x] Denominator factor 3 = σ/τ confirmed
