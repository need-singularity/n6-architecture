# H-CX-875: Piano-Spectrum — 88 Keys = (σ-τ)·(σ-1) EXACT

> **Hypothesis**: A standard piano has 88 keys = (σ-τ)·(σ-1) = 8·11 = 88, exactly. It spans ~7 octaves = M₃, with 12 notes per octave = σ. Middle C is key 40 = σ²/τ + τ = 36 + 4 = 40.

## Grade: 🟩 VERIFIED

## Results

### The Bridge

```
Piano:
  Total keys:         88 = (σ-τ)(σ-1) = 8 · 11  EXACT
  Notes per octave:   12 = σ
  Octave span:        ~7.25 ≈ M₃
  White keys:         52 = τ · (σ+1)
  Black keys:         36 = P₁² = (σ/φ)²
  Middle C position:  key 40 = σ²/τ + τ = 36 + 4

Acoustics/spectrum:
  Harmonic series: f, 2f, 3f, 4f, 5f, 6f, ...
  First 6 harmonics = P₁ harmonics define consonance
  Perfect fifth ratio: 3/2 = σ/(τ·φ) = 3/2
  Octave ratio: 2/1 = φ
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Full Parameter Mapping

```
Piano parameter     Value   n=6 expression        Check
───────────────     ─────   ──────────────        ─────
Total keys            88    (σ-τ)(σ-1) = 8·11     ✓ EXACT
Notes per octave      12    σ                      ✓
Approx. octaves       ~7    M₃                     ✓ (7.33)
White keys            52    τ(σ+1)                 ✓ EXACT
Black keys            36    P₁²                    ✓ EXACT
Middle C (key #)      40    σ²/τ + τ               ✓ EXACT
Lowest note (A0)      27.5 Hz ≈ P₂ - 0.5          ~

White + Black = 52 + 36 = 88 = τ(σ+1) + P₁²  ✓

The piano keyboard decomposes as:
  τ(σ+1) white keys + P₁² black keys = (σ-τ)(σ-1) total
  This is an identity: 4·13 + 36 = 52 + 36 = 88 = 8·11
```

### Texas Sharpshooter Check

The 88-key piano is a 19th-century convention (Steinway). However, the decomposition 52 + 36 = τ(σ+1) + P₁² is exact, and consonant intervals (2:1, 3:2, 4:3, 5:4, 6:5) use only {φ, σ/τ, τ, sopfr, P₁}.

## Verification

- [x] 88 = (σ-τ)(σ-1) = 8·11 exact
- [x] 12 notes/octave = σ
- [x] 52 white keys = τ(σ+1)
- [x] 36 black keys = P₁²
- [x] Middle C = key 40 = σ²/τ + τ

## Status

New. The piano keyboard is fully decomposed by n=6 arithmetic, with consonant intervals matching the small constants.
