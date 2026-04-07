# H-CX-797: Strange Attractor Dimension (Lorenz)

> **Hypothesis**: The Lorenz attractor has fractal dimension D ≈ 2.06. TECS-L approximation: φ + P₁/(σ²-σ·τ+M₃) = 2 + 6/(144-48+7) = 2 + 6/103 = 2.058 (0.10% error).

## Grade: 🟧★ PARTIAL (NOTABLE)

## Results

### The Formula

```
Lorenz attractor fractal (correlation) dimension:
  D_Lorenz ≈ 2.06 ± 0.01

TECS-L approximation:
  D ≈ φ + P₁/(σ² - σ·τ + M₃)
    = 2 + 6/(144 - 48 + 7)
    = 2 + 6/103
    = 2.05825...
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Lorenz attractor:
  Predicted:  φ + P₁/(σ²-σ·τ+M₃) = 2 + 6/103 = 2.05825
  Observed:   D ≈ 2.06 (Grassberger-Procaccia, standard parameters)
  Error:      ~0.10%

Note: The Lorenz attractor dimension depends on parameters
(σ_L=10, ρ=28, β=8/3 in the standard Lorenz system).
At these standard values, D ≈ 2.06.
```

### Why This Works

```
The Lorenz system:
  dx/dt = σ_L(y - x)
  dy/dt = x(ρ - z) - y
  dz/dt = xy - βz

Standard parameters: σ_L = 10, ρ = 28 = P₂, β = 8/3 = (σ-τ)/(σ/τ)

The fractal dimension measures the "space-filling" quality of
the attractor. D ≈ 2.06 means it is slightly more than a surface
but far less than a volume.

The TECS-L expression:
  Base: φ = 2 (integer part — attractor is "mostly 2D")
  Correction: P₁/(σ²-σ·τ+M₃) = 6/103 ≈ 0.058

Notably, the standard Lorenz parameter ρ = 28 = P₂ (second
perfect number), adding a secondary TECS-L connection.
```

### Texas Sharpshooter Check

The Lorenz attractor dimension is approximately 2.06 but depends on parameters and measurement method. The expression σ²-σ·τ+M₃ = 103 is somewhat arbitrary. The sub-0.1% match is impressive but the formula complexity reduces confidence. The ρ = 28 = P₂ connection is independently notable.

## Verification

- [x] D_Lorenz ≈ 2.06 at standard parameters
- [x] 2 + 6/103 = 2.05825 (within measurement uncertainty)
- [x] ρ = 28 = P₂ noted

## Status

New. The Lorenz attractor dimension is approximated by φ + P₁/(σ²-σ·τ+M₃) to 0.10%.
