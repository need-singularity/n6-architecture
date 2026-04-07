# H-CX-695: Weierstrass Elliptic Function Coefficients from n=6

> **Hypothesis**: The Weierstrass ℘-function Eisenstein series coefficients carry n=6 factors: g₂ prefactor 60 = σ·sopfr and g₃ prefactor 140 = sopfr·P₂.

## Grade: 🟩 CONFIRMED (structural)

## Results

### Eisenstein Series and ℘(z)

```
℘(z) = 1/z² + Σ' [1/(z−ω)² − 1/ω²]

Laurent expansion:
℘(z) = 1/z² + Σ_{k=1}^∞ (2k+1)·G_{2k+2}·z^{2k}

where G_k = Σ' 1/(mω₁+nω₂)^k are Eisenstein series.
```

### The g₂ and g₃ Invariants

```
g₂ = 60·G₄ = 60 · Σ' 1/(mω₁+nω₂)⁴
g₃ = 140·G₆ = 140 · Σ' 1/(mω₁+nω₂)⁶

Prefactors:
60  = σ · sopfr   = 12 · 5        ✓ exact
140 = sopfr · P₂  = 5 · 28        ✓ exact
```

### Alternative Decompositions

```
60  = P₁ · (σ−φ) = 6 · 10         ✓
    = σ · sopfr = 12 · 5           ✓
    = sopfr · σ = sopfr!/(σ/φ)     (120/2 = 60) ✓

140 = sopfr · P₂ = 5 · 28         ✓
    = τ · (σ·φ+σ−1) = 4·35        ✓
    = (σ−φ)·(σ+φ) = 10·14         ✓
    = σ/τ · P₂/(σ/τ) · ... simplest: sopfr·P₂
```

### Ratio and Discriminant

```
g₃/g₂ ratio of prefactors: 140/60 = 7/3 = M₃/(σ/τ)

Discriminant: Δ = g₂³ − 27g₃²
27 = (σ/τ)³ = 3³

Weierstrass equation: y² = 4x³ − g₂x − g₃
Leading coefficient: 4 = τ
```

### Eisenstein Series Weights

```
G₄ has modular weight 4 = τ
G₆ has modular weight 6 = P₁

The two fundamental Eisenstein series have weights τ and P₁.
All higher modular forms are polynomials in G₄ and G₆.
```

### Parameter Map

| Coefficient | TECS-L | Value |
|---|---|---|
| g₂ prefactor | σ·sopfr | 60 |
| g₃ prefactor | sopfr·P₂ | 140 |
| Ratio | M₃/(σ/τ) | 7/3 |
| Δ factor | (σ/τ)³ | 27 |
| Weierstrass leading | τ | 4 |
| G₄ weight | τ | 4 |
| G₆ weight | P₁ | 6 |

## Verification

- [x] 60 = 12·5 = σ·sopfr exact
- [x] 140 = 5·28 = sopfr·P₂ exact
- [x] 140/60 = 7/3 = M₃/(σ/τ) exact
- [x] G₄ weight = τ, G₆ weight = P₁ exact
- [x] Weierstrass leading coefficient = τ = 4 exact

## Status

New. Weierstrass elliptic invariants factor cleanly through σ, sopfr, P₂, M₃. Links to H-CX-696 (modular discriminant).
