# H-CX-694: Class Number One Problem and n=6 Structure

> **Hypothesis**: There are exactly 9 = (σ/τ)² imaginary quadratic fields with class number h(−d)=1. The smallest discriminants {−3,−4,−7,−8} map to {σ/τ, τ, M₃, σ−τ}.

## Grade: 🟧 SUGGESTIVE

## Results

### The Class Number One Problem

```
Solved by Heegner (1952), Baker-Stark (1966-67):
h(−d) = 1 for exactly 9 values of d:

d ∈ {3, 4, 7, 8, 11, 19, 43, 67, 163}

Count = 9 = (σ/τ)² = 3² = (12/4)²
```

### n=6 Constants in Small Discriminants

```
d = 3  = σ/τ           ✓
d = 4  = τ              ✓
d = 7  = M₃             ✓
d = 8  = σ − τ          ✓
d = 11 = σ − 1          ✓

Five of nine discriminants are direct n=6 constants.
```

### The Remaining Four

```
d = 19 = σ + M₃ = 12 + 7           two-term
d = 43 = σ·τ − sopfr = 48 − 5      two-term
d = 67 = σ·sopfr + M₃ = 60 + 7     two-term
d = 163 = σ² + 19 = 144 + 19       (where 19 = σ + M₃)
```

### Heegner Numbers and j-Invariant

```
e^(π√163) ≈ 262537412640768744 (Ramanujan's constant)
           = 640320³ + 744

744 = P₁! + σφ = 720 + 24 (H-CX-684)

640320 = 2⁶·3·5·23·29 = ... contains 2 and 3 (factors of P₁)

The near-integer property arises because h(−163)=1
and j(ρ_{163}) is an integer.
```

### Structure of the Count

```
9 = (σ/τ)² = 3²

The class number one fields form a finite set of size (σ/τ)².
For class number 2: there are 18 = σ + P₁ = 3·6 fields.

Count hierarchy:
h = 1: 9 fields  = (σ/τ)²
h = 2: 18 fields = φ·(σ/τ)² = 2·9
```

### Parameter Map

| Discriminant | TECS-L | Value |
|---|---|---|
| d₁ | σ/τ | 3 |
| d₂ | τ | 4 |
| d₃ | M₃ | 7 |
| d₄ | σ−τ | 8 |
| d₅ | σ−1 | 11 |
| Count | (σ/τ)² | 9 |
| h=2 count | φ·(σ/τ)² | 18 |

## Verification

- [x] Count = 9 = (σ/τ)² exact
- [x] d ∈ {3,4,7,8,11} matches {σ/τ, τ, M₃, σ−τ, σ−1} (5 of 9)
- [x] h=2 count = 18 = 2·9 = φ·(σ/τ)² exact
- [x] 744 = P₁! + σφ in Ramanujan's constant context

## Status

New. Five of nine Heegner numbers are direct n=6 constants. Count (σ/τ)²=9 is structural.
