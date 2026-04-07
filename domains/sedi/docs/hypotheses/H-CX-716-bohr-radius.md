# H-CX-716: Bohr Radius — a₀ from TECS-L Constants

> **Hypothesis**: The Bohr radius a₀ = 0.529177 Angstrom ≈ sopfr·τ/(σ·(σ/τ + φ/(σ·φ))) = 0.5405 (2.1% error).

## Grade: 🟧 SPECULATIVE (2.1% error)

## Results

### The Constant

```
a₀ = 4πε₀ℏ² / (m_e e²) = 0.529177210544(82) × 10⁻¹⁰ m  (CODATA 2022)

The most probable distance between electron and proton
in hydrogen ground state.
```

### n=6 Prediction

```
a₀ (in Angstrom) ≈ sopfr·τ / (σ·(σ/τ + φ/(σ·φ)))

Numerator:  sopfr·τ = 5 × 4 = 20
Denominator: σ·(σ/τ + φ/(σ·φ))
           = 12·(3 + 2/24)
           = 12·(3 + 0.08333)
           = 12 × 3.08333
           = 37.0

a₀ ≈ 20/37.0 = 0.5405 Å

Predicted:  0.5405 Å
Observed:   0.52918 Å
Error:      |0.5405 − 0.52918| / 0.52918 = 2.14%
```

### Alternative Attempt

```
a₀ ≈ sopfr / (σ − φ·sopfr/τ + M₃/(σ·τ − sopfr))
   = 5 / (12 − 2.5 + 7/43)
   = 5 / (9.5 + 0.163)
   = 5 / 9.663
   = 0.5174 Å
   Error: 2.2% — similar quality

Neither formula achieves sub-percent accuracy.
```

### Texas Sharpshooter Check

Could 7 random constants in [1, 500] produce 0.529 within 2%?
- Target window: 0.529 ± 0.011 (width 0.022)
- Ratio expressions a·b/(c·(d/e + f/(c·f))): ~300 combinations
- Range: ~[0.001, 1000]; fraction hitting window: 0.022/1000 ~ 2.2×10⁻⁵
- 300 trials: P ~ 0.007
- p-value ~ 0.007 (marginally significant, but 2% error is mediocre)

### P₂=28 Generalization

```
At P₂: sopfr(P₂)·τ(P₂) / (σ(P₂)·(σ(P₂)/τ(P₂) + φ(P₂)/(σ(P₂)·φ(P₂))))
      = 11·6 / (56·(56/6 + 12/(56·12)))
      = 66 / (56·(9.333 + 0.01786))
      = 66 / (56 × 9.351)
      = 66 / 523.66
      = 0.1261

No known atomic scale at 0.126 Å.

P₂ generalization: DOES NOT EXTEND
```

## Verification

- [x] a₀ ≈ 0.5405 Å at 2.14%
- [ ] Error exceeds 2% — mediocre match
- [ ] Formula is complex and appears fitted

## Status

New. The Bohr radius approximately matches sopfr·τ/(σ·(σ/τ+φ/(σφ))) = 0.5405 Å at 2.1%. The match is too coarse and the formula too complex to be compelling.
