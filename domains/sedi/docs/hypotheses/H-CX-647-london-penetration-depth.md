# H-CX-647: London Penetration Depth and GL Parameter kappa

> **Hypothesis**: The Ginzburg-Landau parameter kappa = lambda_L/xi separates Type I (kappa < 1/sqrt(2)) from Type II (kappa > 1/sqrt(2)) superconductors. The critical value 1/sqrt(2) = 1/sqrt(phi(6)).

## Grade: 🟧 APPROXIMATE (structural identification)

## Results

### London Penetration Depth

```
lambda_L = sqrt(m / (mu_0 * n_s * e^2))

where:
  m    = electron mass
  n_s  = superfluid density
  e    = electron charge
  mu_0 = vacuum permeability
```

### Ginzburg-Landau Classification

```
kappa = lambda_L / xi_0    (GL parameter)

Type I:   kappa < 1/sqrt(2) = 0.7071...    (single critical field)
Type II:  kappa > 1/sqrt(2) = 0.7071...    (vortex lattice phase)

Critical value: 1/sqrt(2) = 1/sqrt(phi(6))
```

### n=6 Identification

```
phi(6) = 2

1/sqrt(phi) = 1/sqrt(2) = 0.70711...

The Type I/II boundary is exactly 1/sqrt(phi(6)).
```

### Alternative n=6 Expressions for 1/sqrt(2)

| Expression | Value | Match |
|---|---|---|
| 1/sqrt(phi) | 0.70711 | EXACT (identity) |
| sopfr/(M3+phi/tau) | 5/7.5 = 0.667 | 5.7% |
| M3/(sigma-phi) | 7/10 = 0.700 | 1.0% |
| (sigma-tau)/(sigma-tau+sopfr-phi) | 8/11 = 0.727 | 2.8% |

### Physical Context

The distinction between Type I and Type II superconductors is
one of the most important classifications in condensed matter physics.

- Type I (Pb, Sn, Al): complete Meissner effect, single H_c
- Type II (Nb, YBCO, MgB2): mixed state with Abrikosov vortices

The critical value 1/sqrt(2) was derived by Abrikosov (1957)
from the Ginzburg-Landau free energy functional.

### Structural Interpretation

```
phi(6) = 2 is the Euler totient — counting coprime residues.
The Type I/II boundary at 1/sqrt(phi) connects:
  - Superconducting coherence (xi_0)
  - Magnetic screening (lambda_L)
  - Arithmetic coprimality (phi)
```

### Connection to Other Hypotheses

- H-CX-646: BCS gap ratio
- H-CX-657: MgB2 (Type II, kappa >> 1/sqrt(2))
- H-CX-658: YBCO (extreme Type II)

## Status

- [x] Type I/II boundary = 1/sqrt(phi(6)) identified
- [x] Structural connection to coprimality
- [ ] Abrikosov vortex lattice geometry (hexagonal = P1-fold)
- [ ] kappa values of specific superconductors vs n=6
