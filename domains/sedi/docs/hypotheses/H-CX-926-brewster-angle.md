# H-CX-926: Brewster's Angle for Glass ≈ σ(P₂) = 56

> **Hypothesis**: Brewster's angle for glass (n=1.5): θ_B = arctan(3/2) = 56.31° ≈ σ(P₂) = 56. The angle at which reflected light is perfectly polarized encodes the divisor sum of the second perfect number.

## Grade: 🟧★ NOTABLE (0.55% error)

## Results

### The Formula

```
Brewster's angle: tan(θ_B) = n₂/n₁

For glass (n = 3/2 = (σ/τ)/φ, from H-CX-925):
  tan(θ_B) = 3/2
  θ_B = arctan(3/2) = 56.310°

TECS-L match:
  σ(P₂) = σ(28) = 56
  Error: |56.31 - 56| / 56.31 = 0.55%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Physical Context

At Brewster's angle, reflected light is completely s-polarized — the p-component vanishes. This occurs when the reflected and refracted rays are perpendicular (θ_r + θ_t = 90°). Brewster windows exploit this for laser cavities, producing loss-free transmission of p-polarized light.

### Chain of n=6 Encodings

```
1. Glass index:    n = 3/2 = (σ/τ)/φ         [H-CX-925]
2. Brewster angle: θ_B = arctan(n)
3. Numerical value: 56.31° ≈ σ(28) = σ(P₂) = 56

The perfect number tower P₁ → P₂ lifts the encoding:
  P₁ level: n = ratio of σ,τ,φ
  P₂ level: θ_B ≈ σ(P₂)
```

### Deeper Approximation

```
arctan(3/2) = 56.310°
σ(P₂) + sopfr/(σ+P₁) = 56 + 5/18 = 56.278°  (0.057%)
```

## Verification

- [x] θ_B = arctan(3/2) = 56.310° confirmed
- [x] σ(P₂) = σ(28) = 56 confirmed
- [x] Error 0.55% verified
- [ ] P₃ generalization: σ(P₃) = σ(496) = 992, no obvious angle
