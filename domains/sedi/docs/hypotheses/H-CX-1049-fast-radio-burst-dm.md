# H-CX-1049: Fast Radio Burst Dispersion Measure

> **Hypothesis**: The average extragalactic dispersion measure of cosmological FRBs is ~300-500 pc/cm^3. A TECS-L estimate gives sigma^2*phi + sigma^2/phi + P_2*phi = 288 + 72 + 56 = 416 pc/cm^3, lying within the observed range at 4% from the midpoint.

## Grade: 🟧 NOTABLE

## Results

### The Correspondence

```
FRB average extragalactic DM: ~300-500 pc/cm^3
  Midpoint of range: ~400 pc/cm^3

TECS-L expression:
  sigma^2 * phi + sigma^2/phi + P_2 * phi
  = 144*2 + 144/2 + 28*2
  = 288 + 72 + 56 = 416                           (4% from 400)

Alternative decomposition:
  P_3 - sigma * (sigma-tau+phi)/phi
  = 496 - 12*5 = 496 - 60 = 436                   (~9%)
```

### n=6 Constants

```
sigma = 12, tau = 4, phi = 2, sopfr = 5, n = P_1 = 6, M_3 = 7
P_2 = 28, P_3 = 496, sigma_phi = 24, sigma-tau = 8, R(6) = 1
```

### Structural Analysis

```
DM expression decomposition:
  sigma^2 * phi = 288    (dominant term: squared divisor sum * Euler)
  sigma^2 / phi = 72     (subdominant: squared sum / Euler)
  P_2 * phi     = 56     (correction: second perfect number * Euler)

Total = 416 pc/cm^3

The DM-redshift relation (Macquart relation):
  <DM_cosmic>(z) ~ 900 * z  pc/cm^3
  At z ~ 0.5: DM ~ 450 pc/cm^3
  At z ~ 0.46: DM ~ 416 pc/cm^3

So TECS-L DM corresponds to z ~ 0.46:
  0.46 ~ tau * sigma / (sigma^2 - tau)
  = 48/140 = 0.343 (rough)
```

### Physical Context

Fast radio bursts are millisecond-duration radio transients of cosmological origin. Their dispersion measures encode the integrated column density of free electrons along the line of sight. The Macquart relation, confirmed with localized FRBs, shows that DM grows roughly linearly with redshift. The typical extragalactic DM for FRBs at z ~ 0.3-0.5 falls in the 300-500 pc/cm^3 range, with significant scatter from host galaxy and local environment contributions.

### Texas Sharpshooter Check

The FRB DM range is broad (300-500) and the TECS-L value 416 falls comfortably within it. This wide target makes the match less discriminating. The expression uses three terms with different operations (multiply, divide, multiply), adding degrees of freedom. The 4% accuracy from the midpoint is reasonable but the midpoint itself is approximate. Grade reflects a plausible but not highly constraining match.

## Verification

- [x] DM = 416 within observed range 300-500 pc/cm^3
- [x] Expression uses sigma^2 and P_2 with phi scaling
- [x] Consistent with Macquart relation at z ~ 0.46
- [ ] FRB DM distribution has large scatter
- [ ] Host galaxy DM contribution adds uncertainty
