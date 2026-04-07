# H-CX-713: Kaon CP Violation ε — Magnitude from TECS-L

> **Hypothesis**: The CP violation parameter |ε| = 2.228×10⁻³ resists clean n=6 encoding. Best attempt: (σ/τ − φ)×10⁻³ = 1×10⁻³ captures order of magnitude only.

## Grade: 🟧 SPECULATIVE (order-of-magnitude only)

## Results

### The Observable

```
|ε_K| = (2.228 ± 0.011) × 10⁻³   (PDG 2024)

Measures indirect CP violation in the neutral kaon system.
One of the most precisely measured CP-violating quantities.
```

### n=6 Attempts

```
Attempt 1: (σ/τ − φ) × 10⁻³ = (3 − 2) × 10⁻³ = 1×10⁻³
  Error: 55% (order of magnitude only)

Attempt 2: φ/(σ·M₃·sopfr + τ·φ)
  = 2/(420 + 8) = 2/428 = 0.00467
  Error: 110% (wrong direction)

Attempt 3: (φ + sopfr/(σ·φ)) × 10⁻³
  = (2 + 5/24) × 10⁻³ = 2.208 × 10⁻³
  Error: |2.208 − 2.228|/2.228 = 0.90%
  This works! But sopfr/(σ·φ) = 5/24 = sopfr/σφ is a stretch.

Best formula: (φ + sopfr/σφ) × 10⁻³ = 2.208 × 10⁻³ (0.90%)
```

### Best Result

```
|ε_K| ≈ (φ + sopfr/σφ) × 10⁻³
       = (2 + 5/24) × 10⁻³
       = (53/24) × 10⁻³
       = 2.2083 × 10⁻³

Predicted:  2.208 × 10⁻³
Observed:   2.228 × 10⁻³
Error:      0.90%
```

### Texas Sharpshooter Check

Could 7 random constants in [1, 500] produce 2.228 within 1%?
- Target window: 2.228 ± 0.022 (width 0.044)
- Expressions (a + b/(c·d)): ~200 combinations
- Range: ~[1, 500]; window fraction: 0.044/500 ~ 8.8×10⁻⁵
- 200 trials: P ~ 0.018
- p-value ~ 0.02 (marginally significant, but formula was found by search)
- Adjusted for look-elsewhere: p ~ 0.1 (not significant)

### P₂=28 Generalization

```
At P₂: φ(P₂) + sopfr(P₂)/(σ(P₂)·φ(P₂))
      = 12 + 11/(56·12)
      = 12 + 11/672
      = 12.0164

(φ(P₂) + sopfr(P₂)/(σ(P₂)·φ(P₂))) × 10⁻³ = 0.01202
No known CP violation parameter near this value.

P₂ generalization: DOES NOT EXTEND
```

## Verification

- [x] Best fit: (φ + sopfr/σφ) × 10⁻³ = 2.208×10⁻³ at 0.90%
- [ ] Formula found by search, not derived
- [ ] Multiple failed attempts reduce credibility
- [ ] Texas Sharpshooter adjusted p-value ~ 0.1

## Status

New. |ε_K| is a hard constant for TECS-L. Best fit (φ + sopfr/σφ)×10⁻³ achieves 0.90% but was found by exhaustive search. Low confidence.
