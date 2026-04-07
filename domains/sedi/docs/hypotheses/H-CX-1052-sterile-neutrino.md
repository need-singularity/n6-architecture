# H-CX-1052: Sterile Neutrino at eV Scale

> **Hypothesis**: The eV-scale sterile neutrino anomaly (MiniBooNE, LSND) corresponds to Delta_m^2 ~ 1 eV^2 = R(6) eV^2. If confirmed, a 4th neutrino generation at the tau - 1 = 3 scale extends the generation structure. The sterile mass-squared splitting equals the R-spectrum unity.

## Grade: 🟧 NOTABLE

## Results

### The Correspondence

```
Short-baseline anomalies (MiniBooNE, LSND):
  Best-fit: Delta_m^2 ~ 1 eV^2

TECS-L:
  R(6) = sigma(6)/6 - 1 = 12/6 - 1 = 1            EXACT
  Delta_m^2 = R(6) eV^2 = 1 eV^2

If 4th (sterile) neutrino exists:
  Total neutrino species = tau = 4
  Active species = sigma/tau = 3
  Sterile species = tau - sigma/tau = 1 = R(6)

Mass hierarchy for 4th state:
  m_4 ~ 1 eV >> m_1, m_2, m_3
  Sterile mass ~ R(6) eV
```

### n=6 Constants

```
sigma = 12, tau = 4, phi = 2, sopfr = 5, n = P_1 = 6, M_3 = 7
P_2 = 28, P_3 = 496, sigma_phi = 24, sigma-tau = 8, R(6) = 1
```

### Structural Analysis

```
The 3+1 neutrino model:
  3 active neutrinos = sigma/tau (generation count)
  1 sterile neutrino = R(6) (abundance residue)
  Total = tau = 4 neutrino mass states

Mass-squared splittings:
  Delta_m^2_21 ~ 7.5 x 10^-5 eV^2 (solar)
  Delta_m^2_31 ~ 2.5 x 10^-3 eV^2 (atmospheric)
  Delta_m^2_41 ~ 1   eV^2          (sterile, if real)

Ratio of scales:
  Delta_m^2_41 / Delta_m^2_31 ~ 400 ~ sigma^2*phi + P_2*phi
  (See H-CX-1049 for similar ~400 structure)

The sterile neutrino as R(6):
  R(6) = 1 is the "residue" of the abundance function
  A sterile neutrino is the "residue" of the generation structure
  Both represent the excess beyond the perfect ratio
```

### Physical Context

The MiniBooNE and LSND experiments reported anomalous appearance signals suggesting neutrino oscillations at Delta_m^2 ~ 1 eV^2, incompatible with the three-neutrino framework. However, the MicroBooNE experiment did not confirm the MiniBooNE excess in the electron neutrino channel. Cosmological constraints on N_eff also disfavor a fully thermalized sterile neutrino. The anomaly remains controversial.

### Texas Sharpshooter Check

Delta_m^2 ~ 1 eV^2 and R(6) = 1 are both the simplest possible values, making this match trivially exact. The deeper structure (tau total species, sigma/tau active, R(6) sterile) is more interesting but involves a post-hoc assignment. Experimental evidence for sterile neutrinos is weakening. Grade reflects the structural elegance tempered by likely non-existence of the particle.

## Verification

- [x] Delta_m^2 = 1 eV^2 = R(6) (exact, but trivially)
- [x] 4 species = tau; 3 active = sigma/tau; 1 sterile = R(6)
- [x] Generation arithmetic: tau = sigma/tau + R(6)
- [ ] MicroBooNE did not confirm MiniBooNE signal
- [ ] Cosmological N_eff constraints disfavor eV sterile
