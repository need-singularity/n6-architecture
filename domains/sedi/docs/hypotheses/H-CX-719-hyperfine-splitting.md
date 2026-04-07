# H-CX-719: Hyperfine Splitting — Hydrogen 21 cm Line

> **Hypothesis**: The hydrogen ground-state hyperfine splitting 1420.405 MHz resists clean n=6 encoding. All attempted formulas exceed 10% error. A hard constant.

## Grade: 🟧 SPECULATIVE (no clean match found)

## Results

### The Observable

```
ν_HF = 1420.405751768(1) MHz   (hydrogen 21 cm line)

Hyperfine splitting of hydrogen ground state.
One of the most precisely known frequencies in nature.
Basis of the 21 cm radio astronomy line.
```

### n=6 Attempts

```
Attempt 1: σ³ − σ² + σ·τ − σ + τ
  = 1728 − 144 + 48 − 12 + 4 = 1624
  Error: |1624 − 1420.4| / 1420.4 = 14.3%

Attempt 2: σ²·(τ(P₃) − 1) − σ·sopfr − P₂ − τ
  = 144·9 − 60 − 28 − 4 = 1296 − 92 = 1204
  Error: 15.2%

Attempt 3: P₃·(σ/τ) − σ·sopfr − P₂
  = 496·3 − 60 − 28 = 1488 − 88 = 1400
  Error: 1.4%

Attempt 4: P₃·(σ/τ) − σ·sopfr − τ
  = 1488 − 60 − 4 = 1424
  Error: 0.25%

Attempt 5: P₃·(σ/τ) − σ·sopfr − τ + φ/(σ/τ)
  = 1424 − 0.667 = 1423.33
  Error: 0.21%
```

### Best Result

```
ν_HF ≈ P₃·(σ/τ) − σ·sopfr − τ
      = 496·3 − 60 − 4
      = 1488 − 64
      = 1424 MHz

Predicted:  1424 MHz
Observed:   1420.406 MHz
Error:      0.25%

This is decent but the formula was found by search.
```

### Texas Sharpshooter Check

Could 7 random constants in [1, 500] produce 1420 within 0.25%?
- Target window: 1420.4 ± 3.6 (width 7.1)
- Expressions a·(b/c) − d·e − f: ~500 combinations
- Range: ~[−250000, 250000]; window fraction: 7.1/500000 ~ 1.4×10⁻⁵
- 500 trials: P ~ 0.007
- p-value ~ 0.007 (significant, but found by exhaustive search)
- Adjusted for look-elsewhere (5 attempts): p ~ 0.035

### P₂=28 Generalization

```
At P₂: P₃·(σ(P₂)/τ(P₂)) − σ(P₂)·sopfr(P₂) − τ(P₂)
      = 496·(56/6) − 56·11 − 6
      = 496·9.333 − 616 − 6
      = 4629.3 − 622
      = 4007.3

No known hyperfine splitting near 4007 MHz.

P₂ generalization: DOES NOT EXTEND
```

## Verification

- [x] Best fit: P₃·(σ/τ) − σ·sopfr − τ = 1424 at 0.25%
- [ ] Formula found by search after multiple failures
- [ ] 1420.405 MHz is a hard constant for TECS-L

## Status

New. Hydrogen hyperfine splitting is a hard constant. Best fit P₃·(σ/τ) − σ·sopfr − τ = 1424 achieves 0.25% but required exhaustive search. Low structural confidence.
