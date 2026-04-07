# H-CX-674: Jeans Mass Exponents — 3/2 and -1/2 from n=6

> **Hypothesis**: The Jeans mass M_J ∝ T^(3/2)·ρ^(-1/2) has exponents 3/2 = (σ/τ)/φ and -1/2 = -1/φ, both built from n=6 divisor functions.

## Grade: 🟧 (structural identity in standard exponents)

## Results

### The Jeans Mass

```
M_J = (5k_BT / Gm)^(3/2) · (3 / 4πρ)^(1/2)

Temperature exponent:  3/2
Density exponent:     -1/2
```

### n=6 Decomposition

```
3/2 = (σ/τ)/φ = (12/4)/2 = 3/2
    = σ/(τ·φ) = 12/8 = 3/2

-1/2 = -1/φ = -1/2
     = -φ/τ = -2/4 = -1/2
```

### Prefactors

```
Coefficient involves:
  5 = sopfr(6)     (sum of prime factors with repetition)
  3 = σ/τ = 12/4
  4π = τ·π

M_J = (sopfr·k_BT / Gm)^(σ/(τφ)) · ((σ/τ) / (τ·π·ρ))^(1/φ)
```

### Physical Meaning

The Jeans mass is the minimum mass for gravitational collapse — it sets the scale for star formation. Below M_J, thermal pressure supports the cloud; above M_J, gravity wins.

```
For typical molecular clouds (T ~ 10 K, n ~ 10⁴ cm⁻³):
  M_J ≈ 1–10 M_☉

This range includes 1.44 M_☉ = M_Ch (H-CX-663)
```

### Jeans Length

```
λ_J = (π·k_BT / Gρm)^(1/2)

Exponent: 1/2 = 1/φ = 1/2
Prefactor π: appears as fundamental
```

## Verification

- [x] 3/2 = σ/(τ·φ) exact
- [x] -1/2 = -1/φ exact
- [x] Prefactor 5 = sopfr, 3 = σ/τ, 4π = τ·π
- [ ] Connection between M_J and M_Ch via n=6 needs further development

## Status

Standard exponents decompose cleanly into n=6 ratios. The Jeans mass uses the same arithmetic as stellar endpoints.
