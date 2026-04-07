# H-CX-832: Turbo Code Rate and Shannon Limit

> **Hypothesis**: The canonical turbo code rate 1/3 = τ/σ connects turbo coding to n=6 constants, and the Shannon limit at rate φ/σ = 1/6 yields a computable Eb/N0 threshold.

## Grade: 🟧 SUGGESTIVE

## Results

### The Formula

```
Turbo code (Berrou et al., 1993):
  Standard rate: R = 1/3
  1/3 = τ/σ = 4/12

Near-capacity performance:
  At rate 1/3, Shannon limit Eb/N0 = -0.495 dB (BER → 0)
  Turbo codes achieve within ~0.5 dB of capacity

Shannon limit at rate R = φ/σ = 1/6:
  C = R → Eb/N0 = (2^R - 1)/R
  For R = 1/6:
    Eb/N0 = (2^(1/6) - 1) · 6
    = (1.1225 - 1) · 6
    = 0.1225 · 6
    = 0.735 (linear)
    = -1.34 dB

Rate hierarchy in TECS-L:
  1/P₁ = 1/6 = φ/σ     (very low rate)
  1/τ  = 1/4            (quarter rate)
  τ/σ  = 1/3            (standard turbo)
  1/φ  = 1/2            (half rate)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Turbo code rate:
  R = 1/3 = τ/σ = 4/12 ✓

Shannon limit at R = 1/3:
  Eb/N0_min = (2^(1/3) - 1)/(1/3) = 0.2599/0.333 = 0.780 → -1.07 dB ✓

Shannon limit at R = 1/6:
  Eb/N0_min = (2^(1/6) - 1)/(1/6) = 0.1225/0.167 = 0.735 → -1.34 dB ✓

Original turbo code (1993):
  Rate 1/2 with puncturing from rate 1/3 constituent codes ✓
  Constituent encoders: rate 1/φ RSC codes
```

### Texas Sharpshooter Check

The rate 1/3 is a common engineering choice for concatenated codes, not unique to turbo codes. The identification τ/σ = 1/3 is exact but 1/3 is a simple fraction. The Shannon limit calculation at rate 1/6 is mathematically valid but R=1/6 is not a standard operating point.

## Verification

- [x] Turbo code rate 1/3 = τ/σ
- [x] Shannon limit at rate 1/6 computable
- [x] Rate hierarchy through TECS-L ratios
- [x] Constituent codes at rate 1/φ

## Status

New. Turbo code standard rate τ/σ = 1/3 and the Shannon limit at rate φ/σ connect coding theory to n=6.
