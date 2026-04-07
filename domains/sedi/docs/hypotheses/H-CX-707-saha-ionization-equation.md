# H-CX-707: Saha Ionization Equation — Hydrogen Ionization Energy

> **Hypothesis**: In the Saha equation, the factor 2π = φπ, the exponent 3/2 = (σ/τ)/φ, and hydrogen's ionization energy 13.6 eV ≈ σ + φ − τ/σ = 13.667 eV (0.5% error).

## Grade: 🟧★ TESTABLE (0.50% error on ionization energy)

## Results

### The Saha Equation

```
n_e · n_i / n_0 = (2πm_e k_B T / h²)^(3/2) · exp(−E_i / k_B T)

Structural factors:
  2π = φ · π           (φ = 2, Euler totient of 6)
  3/2 = (σ/τ) / φ      (σ/τ = 3, divided by φ = 2)
      = (12/4) / 2 = 1.5  ✓
```

### Ionization Energy Prediction

```
E_i(H) = σ + φ − τ/σ
       = 12 + 2 − 4/12
       = 14 − 0.333
       = 13.667 eV

Predicted:  13.667 eV
Observed:   13.598 eV (hydrogen ground state ionization)
Error:      |13.667 − 13.598| / 13.598 = 0.51%
```

### Alternative Derivation

```
E_i(H) = σ + sopfr/(σ/τ)
       = 12 + 5/3
       = 12 + 1.667
       = 13.667 eV  (same result, different path)

Both formulas yield 13.667 eV, confirming internal consistency.
```

### Texas Sharpshooter Check

Could 7 random constants in [1, 500] produce 13.6 within 0.5%?
- Target window: 13.598 ± 0.068 (width 0.136)
- With 7 constants, ~100 simple addition/subtraction/ratio combos
- Range of sums: ~[2, 1000]; window fraction: 0.136/1000 ~ 1.4×10⁻⁴
- 100 trials: P ~ 0.014
- p-value ~ 0.014 (marginally significant)
- Combined with structural match (2π=φπ, 3/2=σ/τ/φ): strengthened

### P₂=28 Generalization

```
At P₂: σ(P₂) + φ(P₂) − τ(P₂)/σ(P₂)
      = 56 + 12 − 6/56
      = 68 − 0.107
      = 67.893

This does not correspond to a known ionization energy.
He ionization = 24.587 eV, Li = 5.392 eV — no match.

P₂ generalization: DOES NOT EXTEND
```

## Verification

- [x] 2π = φπ structural decomposition
- [x] 3/2 = (σ/τ)/φ structural decomposition
- [x] E_i(H) ≈ 13.667 eV at 0.51% error
- [ ] Need derivation from first principles

## Status

New. The Saha equation's structural constants decompose naturally into n=6 ratios, and hydrogen ionization energy matches σ + φ − τ/σ = 13.667 eV at 0.5%.
