# H-CX-784: Character Table Dimension of S₆

> **Hypothesis**: For the symmetric group Sₙ, the sum of squared dimensions of irreducible representations equals n!. For S₆: 6! = 720 = P₁! = σ·60 = σ·sopfr·σ = 12·60.

## Grade: 🟩 EXACT

## Results

### The Formula

```
Σ dᵢ² = |G| = n!   (sum of squared irrep dimensions = group order)

For S₆:
  6! = 720
  P₁! = 720
  σ · sopfr · σ = 12 · 5 · 12 = 720  (not quite — 12·60=720, 60=sopfr·σ=5·12)
  σ² · sopfr = 144 · 5 = 720
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Predicted:  Σdᵢ² = P₁! = σ² · sopfr = 720
Observed:   |S₆| = 720
Error:      0.00%

Irrep dimensions of S₆: 1, 5, 9, 5, 10, 16, 5, 10, 9, 5, 1
Check: 1+25+81+25+100+256+25+100+81+25+1 = 720 ✓
```

### Why This Works

```
This is a fundamental theorem in representation theory: for any
finite group G, the sum of squares of irreducible character degrees
equals |G|. Applied to S₆:

  |S₆| = 6! = 720

The TECS-L decomposition 720 = σ² · sopfr expresses the group
order as a product of n=6 arithmetic invariants. Since P₁ = 6,
the factorial P₁! naturally decomposes through σ and sopfr.
```

### Texas Sharpshooter Check

The identity Σdᵢ² = n! is a universal theorem, not specific to n=6. The TECS-L content is the factorization 720 = σ²·sopfr, which is a valid arithmetic identity for n=6 constants.

## Verification

- [x] 6! = 720 confirmed
- [x] σ²·sopfr = 144·5 = 720 confirmed
- [x] Exact match, 0% error

## Status

New. The group order of S₆ factors cleanly through TECS-L constants.
