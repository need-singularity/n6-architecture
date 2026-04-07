# H-CX-1044: CMB Hemispherical Asymmetry

> **Hypothesis**: The CMB hemispherical power asymmetry has amplitude A ~ 0.07 +/- 0.02. This matches R(6)/(sigma + sopfr - phi) = 1/15 = 0.0667, deviating only 4.8% from the central value. The asymmetry is a ratio of the R-spectrum unity to a divisor combination.

## Grade: 🟧 NOTABLE

## Results

### The Correspondence

```
CMB hemispherical asymmetry: A ~ 0.07 +/- 0.02
  R(6)/(sigma + sopfr - phi) = 1/(12 + 5 - 2) = 1/15
  = 0.0667                                        (4.8% from 0.07)

Denominator decomposition:
  sigma + sopfr - phi = 15
  = P_1 + (sigma - tau) + R(6)
  = 6 + 8 + 1 = 15

Alternative reading:
  A ~ phi/(P_2 + phi) = 2/30 = 0.0667             same value
```

### n=6 Constants

```
sigma = 12, tau = 4, phi = 2, sopfr = 5, n = P_1 = 6, M_3 = 7
P_2 = 28, P_3 = 496, sigma_phi = 24, sigma-tau = 8, R(6) = 1
```

### Structural Analysis

```
The hemispherical asymmetry is modeled as:
  T(n_hat) = T_iso(n_hat) * (1 + A * d_hat . n_hat)

Planck measurement: A = 0.07 +/- 0.02
  TECS-L prediction: A = 1/15 = 0.0667

The denominator 15 has structure:
  15 = 3 * 5 = (sigma/tau) * sopfr
  Product of two fundamental ratios

Dipole modulation direction (l, b) ~ (220, -20):
  l ~ 220 ~ sigma * (sigma + sopfr + phi/(sigma-tau))
  (See H-CX-1056 for BAO peak match at 220)
```

### Physical Context

The hemispherical power asymmetry, confirmed by both WMAP and Planck, describes an anomalous dipole modulation of the CMB temperature field. One hemisphere (roughly toward Eridanus) shows systematically higher power than the other. The effect is most significant at large scales (l < 64) and has statistical significance around 3 sigma. Its origin could be a super-horizon perturbation, anisotropic inflation, or a statistical fluke.

### Texas Sharpshooter Check

The measured value 0.07 carries large uncertainties (+/- 0.02), so the match to 0.0667 falls well within error bars. The expression 1/15 is simple but the denominator sigma + sopfr - phi requires a specific sign combination. Multiple small-integer fractions (1/14, 1/15, 1/16) cluster near the target.

## Verification

- [x] A = 0.0667 vs measured 0.07 +/- 0.02 (within 1-sigma)
- [x] Denominator 15 = (sigma/tau) * sopfr (exact factorization)
- [x] Asymmetry is dipolar (l=1 modulation)
- [ ] Physical mechanism for asymmetry unresolved
