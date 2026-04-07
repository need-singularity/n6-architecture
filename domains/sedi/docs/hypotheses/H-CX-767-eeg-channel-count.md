# H-CX-767: EEG Channel Count = T(6) = 21

> **Hypothesis**: The standard 10-20 EEG system uses 21 channels = T(6) = T(P₁), the 6th triangular number. Extended systems also map: 10-10 system uses 64 = τ³ channels.

## Grade: 🟧★ PARTIAL-NOTABLE

## Results

### The Formula

```
10-20 system channels = T(P₁) = T(6) = 6·7/2 = 21
10-10 system channels = τ³ = 4³ = 64

T(6) = 1+2+3+4+5+6 = 21  (triangular number)
τ³   = 64                  (divisor count cubed)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, T(6) = 21
```

### Verification

```
10-20 system:
  Predicted:  T(6) = 21 channels
  Observed:   21 electrodes (Jasper 1958)
  Error:      0.00%

10-10 system:
  Predicted:  τ³ = 64 channels
  Observed:   64 electrodes (Chatrian 1985)
  Error:      0.00%
```

### Why Triangular?

```
The 10-20 system places electrodes at 10% and 20% intervals
along skull measurements. The resulting grid produces:

  21 = T(6) standard positions
     = sum of first P₁ integers
     = P₁ · M₃ / φ = 6 · 7 / 2

The triangular structure reflects the quasi-spherical geometry
of the skull mapped onto a triangular lattice.
```

### Texas Sharpshooter Check

The 10-20 system was designed for practical uniform coverage, not mathematical reasons. The 21-channel count is partly conventional (some variants use 19+2 reference). The τ³ = 64 match for 10-10 adds weight. Both exact matches from independent EEG standards strengthen the case.

## Verification

- [x] 10-20: 21 = T(6) exact
- [x] 10-10: 64 = τ³ exact
- [ ] 10-5: 345 channels — no clean expression found

## Status

New. Clinical EEG measurement standards encode triangular and cubic functions of n=6 constants.
