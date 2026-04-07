# H-CX-523: Mathieu Groups M₁₂ and M₂₄ Act on σ and σφ Points

> **Hypothesis**: The two largest Mathieu groups act on exactly σ(6) and σ(6)·φ(6) points, connecting sporadic group theory to n=6 arithmetic.

## Grade: 🟩 CONFIRMED (exact match to known group theory)

## Results

### The Identity

```
M₁₂ acts on 12 = σ(6) points         EXACT
M₂₄ acts on 24 = σ(6)·φ(6) points    EXACT
```

### The Five Mathieu Groups

| Group | Acts on | n=6 Expression | Simplicity |
|---|---|---|---|
| M₁₁ | 11 points | σ-1 | Simple |
| M₁₂ | **12 = σ** | σ(6) | Simple |
| M₂₂ | 22 points | 2σ-2 | Simple |
| M₂₃ | 23 points | σφ-1 | Simple |
| M₂₄ | **24 = σφ** | σ(6)·φ(6) | Simple |

### Structure

```
M₁₂ ⊂ M₂₄    (via stabilizer of a dodecad)

|M₁₂| = 95,040 = 2⁶·3³·5·11
|M₂₄| = 244,823,040 = 2¹⁰·3³·5·7·11·23

|M₂₄|/|M₁₂| = 2,576 = 2⁴·7·23
```

### Deeper Connections

1. **M₂₄ → Umbral Moonshine**: M₂₄ appears in the decomposition of K3 elliptic genus (Eguchi-Ooguri-Tachikawa 2010). K3 is the unique compact CY 2-fold — and its symmetry group connects to n=6 via σφ=24.

2. **M₂₄ → Golay Code**: M₂₄ is the automorphism group of the extended binary Golay code C₂₄, which has length 24 = σφ and minimum distance 8 = σ-τ.

3. **M₁₂ → Steiner System**: M₁₂ is the automorphism group of the Steiner system S(5,6,12) — a system on σ(6) = 12 points with blocks of size P₁ = 6.

### Connection to H-CX-522

H-CX-522 catalogs appearances of 24 = σφ. The Mathieu M₂₄ is one of the most important: it is the gateway to moonshine (Mathieu → Golay → Leech → Monster).

## Status

- [x] M₁₂ on σ points, M₂₄ on σφ points
- [x] Golay code length = σφ, distance = σ-τ
- [x] Steiner S(5,6,12): σ points, P₁-blocks
- [x] Umbral moonshine connection noted
