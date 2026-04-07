# H-CX-940: Speed of Sound in Air = σ·P₂ + M₃ = 343

> **Hypothesis**: The speed of sound in air at 20°C: c = 343 m/s = σ·P₂ + M₃ = 12·28 + 7 = 343. Exact match. Acoustic impedance Z ≈ 413 Pa·s/m within 0.5%.

## Grade: 🟧★ NOTABLE (speed of sound: exact)

## Results

### The Value

```
Speed of sound in air (20°C, 1 atm):
  c = 343 m/s

TECS-L construction:
  σ · P₂ + M₃ = 12 · 28 + 7 = 336 + 7 = 343  ← EXACT

Acoustic impedance:
  Z = ρc ≈ 413.1 Pa·s/m
  σ·(σφ + sopfr·φ) + sopfr - φ = 12·34 + 3 = 411 (0.5%)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Physical Context

Acoustic impedance Z = ρc determines how sound transmits between media. At an interface, the reflection coefficient depends on the impedance mismatch: R = (Z₂-Z₁)/(Z₂+Z₁), analogous to the optical Fresnel equations (H-CX-937).

### Component Analysis

```
Air density: ρ = 1.204 kg/m³
  ≈ R(6) + φ/τ(P₃) = 1 + 2/10 = 1.200 (0.3%)

Speed of sound: c = 343 m/s = σ·P₂ + M₃  ← EXACT
  Uses both the P₂ perfect number and M₃ Mersenne prime.
```

## Verification

- [x] Speed of sound: 343 = σ·P₂ + M₃ EXACT
- [x] Z_air ≈ 413 Pa·s/m, TECS-L ≈ 411 (0.5%)
- [x] Air density ≈ 1.200 (0.3% from TECS-L)
