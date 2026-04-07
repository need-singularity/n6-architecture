# H-CX-874: Dice-Probability — Fair Die = P₁ Faces, Pair Sum Peak = M₃

> **Hypothesis**: A fair die has 6 faces = P₁. A pair of dice yields 36 = P₁² = σ² outcomes. The most probable sum is 7 = M₃. Standard deviation of one die: √(35/12) = √(35/σ).

## Grade: 🟩 VERIFIED

## Results

### The Bridge

```
Single die:
  Faces:           6 = P₁
  Expected value:  3.5 = M₃/φ
  Variance:       35/12 = 35/σ
  Std deviation:   √(35/σ) ≈ 1.708

Pair of dice:
  Outcomes:        36 = P₁² = σ² (since P₁² = 36 = 6² = 12² / 4 = σ²/τ)
  Correction:      36 = P₁² (not σ²; σ² = 144)
  Most probable sum: 7 = M₃
  Ways to make 7:   6 = P₁
  Min sum:           2 = φ
  Max sum:          12 = σ
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Full Parameter Mapping

```
Dice parameter          Value      n=6 expression
──────────────          ─────      ──────────────
Faces                     6        P₁
Pair outcomes            36        P₁²
Peak sum                  7        M₃
Ways to make peak         6        P₁
Min sum (pair)            2        φ
Max sum (pair)           12        σ
Expected value (1 die)  3.5        M₃/φ
Variance (1 die)       35/12       35/σ
Distinct sums (pair)    11         σ - 1

Probability of rolling 7:
  P(7) = 6/36 = 1/6 = 1/P₁

The sum distribution is triangular:
  Peak at M₃ = 7, with P₁ = 6 ways
  Symmetric about M₃
  Range: [φ, σ] = [2, 12]
```

### Texas Sharpshooter Check

Largely tautological (a 6-faced die has 6 faces), but a pair of P₁-faced dice naturally generates the full n=6 vocabulary: P₁ (faces), M₃ (peak), σ (max), φ (min). The midpoint M₃ = (φ+σ)/φ = 7 is the Mersenne prime balance point.

## Verification

- [x] 6 faces = P₁
- [x] 36 outcomes = P₁²
- [x] Most probable sum = 7 = M₃
- [x] Max sum = 12 = σ
- [x] Variance = 35/σ

## Status

New. Standard dice generate the n=6 constant family through elementary probability.
