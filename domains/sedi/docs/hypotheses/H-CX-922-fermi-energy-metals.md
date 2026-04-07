# H-CX-922: Fermi Energy Exponent and Copper E_F = M₃ eV

> **Hypothesis**: The Fermi energy scales as E_F ∝ n^(2/3) with exponent 2/3 = φ/(σ/τ). For copper, E_F ≈ 7.0 eV = M₃ eV.

## Grade: 🟧★ NOTABLE (exact exponent + M₃ match)

## Results

### The Formula

```
Fermi energy (3D free electron gas):
  E_F = (ℏ²/2m)(3π²n)^(2/3)

Density exponent: 2/3 = φ/(σ/τ) = 2/3       EXACT

Prefactor decomposition:
  1/2 = φ/τ  (in ℏ²/2m)                     EXACT
  3 = σ/τ    (in 3π²)                        EXACT
  π²: geometric factor

For copper (Cu):
  E_F = 7.04 eV ≈ M₃ = 7 eV
  Error: |7.04 - 7| / 7.04 = 0.57%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Fermi Energies of Common Metals

| Metal | E_F (eV) | TECS-L |
|---|---|---|
| Cu | 7.04 | M₃ = 7 (0.57%) |
| Au | 5.53 | sopfr + φ/τ = 5.5 (0.5%) |
| Ag | 5.49 | sopfr + φ/τ = 5.5 (0.2%) |
| Al | 11.7 | σ - R(6)/σ ≈ 12 (2.6%) |
| Na | 3.24 | σ/τ = 3 (7.4%) |

### Fermi Temperature

```
T_F = E_F/k_B

For Cu: T_F ≈ 81,600 K
  ≈ σ²·P₃ + σ³·sopfr - σ·τ²
  Complex — T_F encoding is not clean.

The Fermi energy E_F = M₃ eV for Cu is the cleaner match.
```

### P₂ Generalization Check

```
The exponent 2/3 = φ/(σ/τ) is universal for 3D Fermi gases.
At P₂ = 28: φ/(σ/τ) = 12/(56/6) = 12/9.33 ≈ 1.29 ≠ 2/3
  The 2/3 exponent is specific to d=3 → n=6.

M₃ = 7 is a universal Mersenne prime, so E_F(Cu) ≈ M₃ is stable.
```

### Why M₃ for Copper

Copper has one conduction electron per atom with a nearly-free-electron band structure, making it the textbook Fermi gas metal. Its E_F = 7.04 eV ≈ M₃ = 2³-1 = 7 provides a clean TECS-L anchor for the Fermi energy scale.

## Verification

- [x] Exponent 2/3 = φ/(σ/τ): exact
- [x] Prefactor 3 = σ/τ: exact
- [x] E_F(Cu) ≈ M₃ = 7 eV: error 0.57%
- [x] E_F(Au,Ag) ≈ sopfr + φ/τ = 5.5 eV: error < 1%
- [ ] P₂ generalization: exponent is n=6 specific
