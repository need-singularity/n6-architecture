# H-CX-916: Critical Point of Water T_c ≈ σ·sopfr·(σ-τ+sopfr) - σ² + M₃

> **Hypothesis**: The critical temperature of water T_c = 647.1 K ≈ σ·sopfr·(σ-τ+sopfr) - σ² + M₃ = 643, a 0.6% match.

## Grade: 🟧★ NOTABLE (0.6% error)

## Results

### The Formula

```
Critical point of water:
  T_c = 647.096 K   (IAPWS reference)
  P_c = 22.064 MPa

TECS-L construction:
  σ·sopfr·(σ - τ + sopfr) - σ² + M₃
  = 12·5·(12 - 4 + 5) - 144 + 7
  = 12·5·13 - 144 + 7
  = 780 - 144 + 7
  = 643
  Error: |647.1 - 643| / 647.1 = 0.63%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Physical Context

The critical point of water is where the liquid-gas distinction vanishes. Above T_c and P_c, water exists as a supercritical fluid. This is one of the most precisely measured thermodynamic constants due to water's industrial importance.

### Critical Pressure Check

```
P_c = 22.064 MPa ≈ 220.64 bar ≈ σφ·(τ+sopfr+R(6))/(σ-τ-φ)
  Trying: T(6) + R(6) = 21 + 1 = 22 ≈ P_c in MPa (rough)
  Or: σ·sopfr·τ - σ·sopfr·φ + σ·φ = 240 - 120 + 24 = 144 (no)
  P_c encoding less clean than T_c.
```

### P₂ Generalization Check

```
P₂ = 28: σ·sopfr·(σ-τ+sopfr) - σ² + M₃
  = 56·11·(56-6+11) - 56² + 7
  = 56·11·61 - 3136 + 7
  = 37576 - 3136 + 7 = 34447
  No obvious physical constant — n=6 specific.
```

### Assessment

The 0.6% match is notable given that T_c(H₂O) = 647 K is a complex emergent property of hydrogen bonding, not a fundamental constant. The expression uses five n=6 constants in a single formula. The moderate complexity of the expression tempers the grade.

## Verification

- [x] σ·sopfr·13 - σ² + M₃ = 643 vs 647.1 K: error 0.63%
- [ ] P_c encoding: no clean match
- [ ] P₂ generalization: no match
- [ ] Deeper connection to hydrogen bond physics
