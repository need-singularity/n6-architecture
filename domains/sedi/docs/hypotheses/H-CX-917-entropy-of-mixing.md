# H-CX-917: Entropy of Mixing = -nR·Σxᵢ·ln(xᵢ), Maximum at φ/τ Ratio

> **Hypothesis**: The entropy of mixing for ideal solutions ΔS = -nR·Σxᵢ·ln(xᵢ) achieves maximum for equal binary mixing at x = φ/τ : φ/τ = 1/2 : 1/2, yielding ΔS_max = nR·ln(φ).

## Grade: 🟩 CONFIRMED (exact identity)

## Results

### The Formula

```
Entropy of mixing (ideal binary):
  ΔS = -nR·[x·ln(x) + (1-x)·ln(1-x)]

Maximum at equal composition:
  x = 1/2 = φ/τ                              EXACT
  ΔS_max = nR·ln(2) = nR·ln(φ)              EXACT

The ln(2) = ln(φ) connection:
  φ = 2 (Euler totient of 6)
  Maximum mixing entropy = nR·ln(φ)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
ln(2) = ln(φ) ≈ 0.6931
```

### Physical Context

The entropy of mixing quantifies the disorder increase when different species are combined. For an ideal mixture:

- Maximum disorder occurs at equal proportions (50-50)
- This is the φ/τ : φ/τ ratio in TECS-L
- The maximum value nR·ln(2) = nR·ln(φ) connects to information theory: 1 bit = ln(2) nats

### Information-Theoretic Connection

```
Shannon entropy (binary):
  H = -p·log₂(p) - (1-p)·log₂(1-p)
  H_max = 1 bit at p = 1/2 = φ/τ

Boltzmann mixing entropy:
  ΔS_max = nR·ln(2) = nR·ln(φ) at x = φ/τ

Both maximize at the same TECS-L ratio φ/τ.
The bit (information unit) and the mixing entropy share φ.
```

### P₂ Generalization Check

```
P₂ = 28: φ(28) = 12, τ(28) = 6
  φ/τ = 12/6 = 2 (not a valid mole fraction)
  The binary mixing formula requires x ∈ [0,1], so φ/τ = 1/2 is n=6 specific.
  At n = 6: φ/τ = 2/4 = 1/2 ∈ [0,1] ✓
```

## Verification

- [x] Maximum at x = φ/τ = 1/2: exact
- [x] ΔS_max = nR·ln(φ): exact
- [x] Information-theoretic parallel: 1 bit at φ/τ
- [x] P₂ generalization: n=6 specific (φ/τ must be ≤ 1)
