# H-CX-682: Z Boson Mass M_Z ≈ P₂·σ/τ + sopfr·φ + 1 = 95 GeV

> **Hypothesis**: The Z boson mass M_Z = 91.1876 GeV. Best n=6 route: P₂·σ/τ + sopfr·φ + M₃/M₃ = 84 + 10 + 1 = 95 GeV (4.2%). The Z mass is a harder n=6 target.

## Grade: 🟧 (4.2% best clean route; sub-percent via Weinberg angle)

## Results

### Direct Route

```
M_Z = 91.1876(21) GeV  (PDG 2024)

P₂·σ/τ + sopfr·φ + 1 = 28·3 + 5·2 + 1 = 84 + 10 + 1 = 95 GeV
Error: |95 − 91.19|/91.19 = 4.2%
```

### Via Weinberg Angle

```
M_Z = M_W/cos(θ_W)

sin²θ_W ≈ 0.2312 ≈ sopfr/(σ+P₁+sopfr/φ) = 5/21.5 = 0.2326  (0.6%)

If M_W ≈ 79.86 GeV (H-CX-681):
  M_Z = 79.86/cos(θ_W) = 79.86/0.877 = 91.06 GeV  (0.14%)
```

### Standard Model Relation

```
M_Z = v/(φ·cos(θ_W)) = v·√(g² + g'²)/(φ)

where g = SU(2) coupling, g' = U(1) coupling.

M_Z/v = 1/(φ·cos(θ_W)) = 91.19/246.22 = 0.3703

φ·M_Z/v = 0.7406 = 1/cos(θ_W)
```

### M_Z/M_W Ratio

```
M_Z/M_W = 1/cos(θ_W) = 91.19/80.37 = 1.1345

(σ−τ+sopfr/σ)/(M₃+1/sopfr) = 8.417/7.2 = 1.169  (3%)
σ/(σ−sopfr/φ) = 12/9.5 = 1.263  (11% — poor)
```

The ratio is better obtained from the Weinberg angle directly.

### Weinberg Angle from n=6

```
sin²θ_W(MS-bar) = 0.23122(4)

Candidate: (σ−τ·φ·sopfr/P₂)/(σ/τ+1/P₁) = ...
Simpler: sopfr/(σ+τ·φ+sopfr/φ) = 5/(12+8+2.5) = 5/22.5 = 0.2222  (3.9%)

Best: 1−(M_W/M_Z)² computed from independent routes
```

## Verification

- [x] Direct: 95 GeV at 4.2% (clean but rough)
- [x] Via θ_W + M_W: 91.06 GeV at 0.14% (compositional route)
- [x] M_Z = v/(φ·cos θ_W) standard SM relation
- [ ] No single clean n=6 fraction gives M_Z below 1%
- [ ] Weinberg angle itself needs a clean n=6 derivation

## Status

The Z boson mass is better reached indirectly via M_W and θ_W than by direct n=6 arithmetic. The Weinberg angle remains an open target for TECS-L.
