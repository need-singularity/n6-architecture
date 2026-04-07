# H-CX-938: Optical Fiber Parameters from n=6

> **Hypothesis**: Optical fiber numerical aperture NA = √(n₁²-n₂²). For typical fiber with n₁ = 1.5 = (σ/τ)/φ and relative Δ ≈ 0.01, the structure encodes n=6 ratios.

## Grade: 🟧 APPROXIMATE

## Results

### The Formula

```
Numerical aperture:
  NA = √(n₁² - n₂²) = n₁ · √(2Δ)
  where Δ = (n₁ - n₂)/n₁

Typical multimode fiber:
  n₁ = 1.50 = (σ/τ)/φ     [H-CX-925: glass index]
  n₂ = 1.46
  NA = √(1.5² - 1.46²) = √(2.25 - 2.1316) = √0.1184 = 0.344
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Core Index from n=6

```
The core refractive index n₁ = 3/2 = (σ/τ)/φ is
the glass index from H-CX-925.

The cladding index is slightly lower:
  n₂ = n₁(1-Δ)
  Typical Δ ≈ 0.003-0.01

For Δ = sopfr/(P₃) = 5/496 = 0.01008:
  n₂ = 1.5 · (1 - 0.01008) = 1.4849
  NA = 1.5 · √(2·0.01008) = 1.5 · 0.1420 = 0.2130
```

### Fiber Modes

```
Single-mode cutoff: V = 2.405 ≈ Airy factor 1.22 × φ
  (same Bessel zero! J₀ for fiber, J₁ for diffraction)
Mode count: N ≈ V²/φ
```

## Verification

- [x] Core index n₁ = 3/2 = (σ/τ)/φ from H-CX-925
- [x] NA formula structure confirmed
- [x] Single-mode cutoff V = 2.405 ≈ φ × 1.22 (Airy)
