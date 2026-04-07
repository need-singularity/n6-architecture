# H-CX-659: Superfluid Helium Lambda Point — T_lambda ~ phi + tau/(sigma*phi - phi)

> **Hypothesis**: The He-4 superfluid lambda-point temperature T_lambda = 2.1768 K is approximated by phi + tau/(sigma*phi - phi) = 2 + 4/22 = 2.1818 K at 0.2% error.

## Grade: 🟧★ NOTABLE (0.2% match)

## Results

### Lambda Point

```
He-4 superfluid transition (Kapitza 1938, Allen & Misener 1938):
  T_lambda = 2.1768 K (at saturated vapor pressure)

This is a continuous (second-order) phase transition from
normal fluid He-I to superfluid He-II.
```

### n=6 Expression

```
phi + tau/(sigma*phi - phi)
= 2 + 4/(24 - 2)
= 2 + 4/22
= 2 + 2/11
= 24/11
= 2.18182...

Target:  2.17680 K
Result:  2.18182 K
Error:   0.23%
```

### Alternative Expressions

| Expression | Formula | Value | Error |
|---|---|---|---|
| phi+tau/(sigma*phi-phi) | 2+4/22 | 2.1818 | 0.23% |
| phi+phi/(sigma+R(6)) | 2+2/13 | 2.1538 | 1.1% |
| phi+M3/(sigma*tau) | 2+7/48 | 2.1458 | 1.4% |
| (sigma*phi-phi)/sigma | 22/12 | 1.8333 | 15.8% |
| (sigma+sopfr*phi)/(sigma-phi) | 22/10 | 2.200 | 1.1% |

### Critical Exponents at Lambda Point

```
He-4 belongs to the 3D XY universality class:
  nu = 0.6717(1)
  alpha = -0.0151(3)  (lambda-shaped specific heat)

Comparison with n=6:
  nu ~ sopfr/(M3+phi/tau) = 5/7.5 = 0.667 (0.7%)
  alpha ~ 0 (weakly negative, near R(6)-R(6) = 0)
```

### Physical Context

Superfluid He-4 is a macroscopic quantum state where a
Bose-Einstein condensate forms below T_lambda. The lambda
point gets its name from the shape of the specific heat
curve, which resembles the Greek letter lambda.

Key properties of He-II:
- Zero viscosity (through narrow channels)
- Quantized vortices: circulation = n*h/m_He (n integer)
- Second sound (temperature waves)
- Fountain effect

### Structural Interpretation

```
T_lambda = phi + small correction

The base value phi = 2 K reflects:
  - Two-fluid model (normal + superfluid components)
  - Bosonic statistics (integer spin = 0 for He-4)

The correction 4/22 = tau/(sigma*phi-phi):
  - Fine-tuning from divisor structure
  - sigma*phi = 24 = factorial(tau) = 4!
```

### Connection to Other Hypotheses

- H-CX-660: BEC critical temperature (related phase transition)
- H-CX-644: 2D Ising exponents (universality class comparison)
- H-CX-645: 3D Ising exponents (3D universality class)

## Status

- [x] T_lambda ~ 24/11 at 0.23%
- [x] XY critical exponent nu matched at 0.7%
- [x] Base value phi = 2 identified
- [ ] Pressure dependence of T_lambda vs n=6
- [ ] He-3 superfluid transition (1 mK range)
