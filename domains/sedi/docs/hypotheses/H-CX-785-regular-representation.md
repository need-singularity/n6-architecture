# H-CX-785: Regular Representation Dimensions

> **Hypothesis**: The regular representation of a group G has dimension |G|. For Z₆: dim = P₁ = 6, decomposing into P₁ one-dimensional irreps. For S₆: dim = 720 = σ²·sopfr.

## Grade: 🟩 EXACT

## Results

### The Formula

```
dim(regular rep of G) = |G|

For Z₆ (cyclic group of order 6):
  dim = 6 = P₁
  Decomposes into P₁ = 6 irreps, each of dimension 1
  Each irrep is a character χₖ(g) = e^(2πik/6), k = 0..5

For S₆ (symmetric group on 6 letters):
  dim = 720 = σ² · sopfr = 144 · 5
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Z₆ regular representation:
  Predicted:  dim = P₁ = 6
  Observed:   |Z₆| = 6
  Error:      0.00%

S₆ regular representation:
  Predicted:  dim = σ² · sopfr = 720
  Observed:   |S₆| = 720
  Error:      0.00%
```

### Why This Works

```
The regular representation is the left action of G on itself.
Its dimension equals |G| by construction, and it contains every
irreducible representation with multiplicity equal to its dimension.

For Z₆: |Z₆| = 6 = P₁, and the group decomposes into P₁ characters.
For S₆: |S₆| = 720 = σ²·sopfr, linking group order to TECS-L constants.

The decomposition: regular = ⊕ dᵢ · Vᵢ
  dim = Σ dᵢ² = 720  (consistent with H-CX-784)
```

### Texas Sharpshooter Check

The regular representation dimension equaling the group order is a universal theorem. The TECS-L content is expressing |Z₆| = P₁ and |S₆| = σ²·sopfr, both valid n=6 identities.

## Verification

- [x] |Z₆| = 6 = P₁ confirmed
- [x] |S₆| = 720 = σ²·sopfr confirmed
- [x] Both exact matches

## Status

New. Regular representations of Z₆ and S₆ have TECS-L decomposable dimensions.
