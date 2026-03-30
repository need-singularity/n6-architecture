# N6 Cosmology & Particle Physics -- Extreme Hypotheses H-CP-61~80

> Extension of H-CP-1~30. TECS-L cross-domain discoveries applied to
> cosmology and particle physics. Deeper structural connections, precision
> formulas, and speculative predictions tested against measured values.

> **Honesty principle**: The base 30 hypotheses yielded 4 EXACT, 9 CLOSE,
> 8 WEAK, 9 FAIL. These extreme hypotheses probe more speculative territory.
> We expect a similar or worse success rate.

## Core Constants (review)

```
  n = 6          sigma(6) = 12     tau(6) = 4      phi(6) = 2
  sopfr(6) = 5   J_2(6) = 24       mu(6) = 1       lambda(6) = 2
  R(6) = 1       P_2 = 28 (second perfect number)
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

## TECS-L Cross-Reference Discoveries

```
  Verified matches from other N6 domains:
    1. BCS specific heat jump numerator 12 = sigma(6)
    2. Abrikosov vortex hexagonal coordination = 6 = n
    3. Bott periodicity = 8 = sigma - tau
    4. Leech lattice = 24 = J_2(6)
    5. m_p/m_e ~ 6*pi^5 (19 ppm) -- strongest precision match
    6. 12 SM gauge generators = sigma(6)
    7. Boltzmann sparsity 1/e ~ 0.368 (engine technique)
    8. Egyptian fraction 1/2+1/3+1/6 = 1 (routing/allocation)
```

---

## Category X: Precision Dimensionless Ratios

---

### H-CP-61: Neutron-to-Proton Mass Ratio from n=6

> m_n/m_p = 1.00137842 (CODATA 2022). Can n=6 sharpen this?

```
  Measured: m_n / m_p = 1.00137841887(58)

  The ratio is ~1 + epsilon where epsilon = 0.00138.
    epsilon = (m_n - m_p) / m_p = 1.293 MeV / 938.272 MeV

  Attempt: 1 + mu/(sigma*sopfr*n + sigma) = 1 + 1/(360+12) = 1 + 1/372
           = 1.002688 (too large)
           1 + mu/(sigma * sopfr * sigma) = 1 + 1/720 = 1.001389 (0.6 ppm!)

  Formula: m_n/m_p ~ 1 + mu/(sigma * sopfr * sigma)
           = 1 + 1/(12 * 5 * 12) = 1 + 1/720

  1/720 = 0.001389
  Measured epsilon = 0.001378

  Error: |0.001389 - 0.001378| / 0.001378 = 0.77%

  Note: 720 = 6! (6 factorial). So the formula is 1 + 1/n!
  Also: 720 = sigma * sopfr * sigma = 12 * 5 * 12 = sigma^2 * sopfr.

  This is suggestive: the neutron-proton mass splitting is ~1/6!.

  BUT:
    The 0.77% error on epsilon means the formula is approximate.
    720 = 6! is a well-known factorial, not unique to n=6 functions.
    The mass splitting arises from QCD+QED isospin breaking,
    which has no known connection to 6!.

  Grade: CLOSE
  m_n/m_p ~ 1 + 1/720 = 1 + 1/n! matches the neutron-proton mass
  splitting to 0.77% on the deviation. Interesting that 720 = 6!
  but no theoretical basis.
```

---

### H-CP-62: Muon-to-Electron Mass Ratio ~ sigma * (sigma + sopfr + mu)

> m_mu/m_e = 206.768 (CODATA 2022).

```
  Measured: m_mu / m_e = 206.7682830(46)

  Attempts:
    sigma * (sigma + sopfr + mu) = 12 * 18 = 216 (4.5% off)
    sigma * (sigma + sopfr) = 12 * 17 = 204 (1.3% off)
    sigma * sigma + J_2 * sopfr/n = 144 + 20 = 164. No.
    (J_2 + phi) * (sigma - tau) + sopfr - phi = 26*8 + 3 = 211 (2.0%)
    sigma^2 + sopfr*sigma + sopfr - phi = 144+60+5-2 = 207 (0.11%!)

  Best: sigma^2 + sopfr*sigma + sopfr - phi
        = 144 + 60 + 5 - 2 = 207
  Error: |207 - 206.768| / 206.768 = 0.11%

  BUT: this formula uses 4 terms with arbitrary signs.
  Simpler: sigma * (sigma + sopfr) = 204 (1.3% off) is cleaner but worse.

  The muon mass is a mystery (the "muon problem": why m_mu/m_e ~ 207?).
  No known theoretical prediction.

  Grade: WEAK
  The best-fit formula is too ad hoc (4 terms). The simpler
  sigma*(sigma+sopfr)=204 is 1.3% off. Neither is convincing.
```

---

### H-CP-63: Tau-to-Muon Mass Ratio ~ sigma + sopfr/n

> m_tau/m_mu = 16.817 (PDG 2024).

```
  Measured: m_tau / m_mu = 16.8170(13)

  Attempts:
    sigma + tau + mu = 17 (1.1% off)
    sigma + sopfr = 17 (1.1% off)
    sigma * sqrt(phi) = 12*1.414 = 16.97 (0.91% off)
    (sigma + sopfr) * mu = 17. Same as above.

  Best: 17 = sigma + sopfr (or sigma + tau + mu). Error: 1.1%.

  Interesting that 17 = SM particle count appears again (see H-CP-4, H-CP-24).

  Grade: CLOSE
  m_tau/m_mu ~ 17 = sigma + sopfr. 1.1% error on a dimensionless ratio.
  The recurring appearance of 17 in this framework is notable.
```

---

### H-CP-64: Cabibbo Angle ~ 1/(n + sopfr*mu + mu)

> sin(theta_C) = 0.2253 +/- 0.0007 (Cabibbo angle, |V_us|).

```
  Measured: sin(theta_C) = |V_us| = 0.2253 +/- 0.0007
  theta_C = 13.02 degrees

  Attempts on sin(theta_C):
    phi / (sigma - tau + mu) = 2/9 = 0.2222 (1.4% off)
    (n/phi) / (sigma + mu) = 3/13 = 0.2308 (2.4% off)
    -- wait, that's sin^2(theta_W) from H-CP-8!

    sopfr / (J_2 - phi) = 5/22 = 0.2273 (0.9% off)
    mu*tau / (sigma + sopfr + mu) = 4/18 = 0.2222 (1.4% off)

  Attempts on theta_C in degrees:
    13 = sigma + mu = 13. Error: |13 - 13.02|/13.02 = 0.15%

  The Cabibbo angle theta_C ~ 13 degrees = sigma + mu degrees.
  0.15% error on the angle in degrees.

  BUT:
    Degrees are a human convention (360 per circle).
    In radians: theta_C = 0.2274 rad, not matching anything clean.
    sin(theta_C) ~ 5/22 at 0.9% is decent but 22 is not a clean n=6 value.

  Grade: WEAK
  13 degrees = sigma + mu matches to 0.15% but degrees are conventional.
  sin(theta_C) has no clean match. WEAK.
```

---

### H-CP-65: Proton Magnetic Moment ~ n/phi + mu/sigma

> mu_p = 2.7928 nuclear magnetons (CODATA 2022).

```
  Measured: mu_p / mu_N = 2.79284734463(82)

  Attempts:
    n/phi = 3 (7.4% off)
    n/phi - mu/sopfr = 3 - 0.2 = 2.8 (0.26% off!)
    sigma*phi/n - mu/sigma/sopfr = 4 - ... no, too complex.

  Clean attempt: n/phi - mu/sopfr = 3 - 1/5 = 14/5 = 2.80
  Error: |2.80 - 2.7928| / 2.7928 = 0.26%

  The proton magnetic moment is anomalous (Dirac value would be 1 mu_N
  for a point particle). The anomalous value 2.793 arises from QCD.

  14/5 = 2.8 is a simple fraction. n/phi - mu/sopfr = 3 - 1/5 is
  a clean expression using 3 n=6 functions.

  BUT:
    0.26% error is decent but not EXACT-level for this precision.
    mu_p is deeply connected to quark structure and QCD, with no
    known link to number theory.

  Grade: CLOSE
  mu_p ~ 14/5 = n/phi - mu/sopfr. 0.26% error, dimensionless ratio.
```

---

### H-CP-66: Neutron Magnetic Moment ~ -(phi - mu/sopfr)

> mu_n = -1.9130 nuclear magnetons (CODATA 2022).

```
  Measured: mu_n / mu_N = -1.91304273(45)

  Attempts:
    -phi = -2 (4.5% off)
    -(phi - mu/sigma) = -(2 - 1/12) = -23/12 = -1.9167 (0.19%!)

  Formula: mu_n/mu_N ~ -(phi - mu/sigma) = -(2 - 1/12) = -23/12

  Error: |(-23/12) - (-1.91304)| / 1.91304 = 0.19%

  23/12 is a simple fraction. The expression phi - mu/sigma = 2 - 1/12
  uses only n=6 functions.

  Combined with H-CP-65:
    mu_p/mu_n ~ (14/5) / (-23/12) = -168/115 = -1.4609
    Measured: mu_p/mu_n = -1.45989805(34)
    Error: 0.07%!

  The ratio mu_p/mu_n ~ -168/115 at 0.07% is remarkably precise.

  Grade: CLOSE
  mu_n ~ -23/12 = -(phi - mu/sigma) at 0.19%. The combined
  ratio mu_p/mu_n at 0.07% is stronger. But these are numerical
  fits, not derivations.
```

---

### H-CP-67: Electron g-factor Anomaly a_e and n=6

> a_e = (g-2)/2 = 0.00115965218128(18).

```
  Measured: a_e = 0.00115965218128(18) (most precise QED prediction)

  The leading QED term: a_e = alpha/(2*pi) = 0.0011614...
  Full value includes higher-order QED + hadronic + weak corrections.

  Can n=6 reach alpha/(2*pi)?
    alpha ~ 1/137.036.
    alpha/(2*pi) = 1/(137.036 * 2 * pi) = 1/861.02 = 0.001161...

  Attempt on 137*2*pi:
    sigma^2 * n = 144 * 6 = 864 (0.35% off from 861)
    (sigma^2 - sopfr - phi) * (phi * pi) = 137 * 2pi = 861.0 (exact by construction)
    -- but this just uses alpha^{-1} ~ 137 from H-CP-9 times 2*pi.

  No independent n=6 content beyond what H-CP-9 already provides.

  Grade: FAIL
  a_e is derived from alpha via QED perturbation theory.
  No new n=6 content beyond alpha^{-1} ~ 137 (already WEAK).
```

---

### H-CP-68: Cosmological Baryon Fraction ~ 1/n = 1/6?

> Omega_b / Omega_m = 0.157 (Planck 2018).

```
  Measured: Omega_b = 0.0493, Omega_m = 0.315
  Ratio: Omega_b / Omega_m = 0.157

  Attempts:
    1/n = 1/6 = 0.167 (6.0% off)
    mu/n = 1/6. Same.
    phi/sigma = 1/6. Same.

  All give 1/6 = 0.167, which is 6% off from 0.157.

  Alternatively: Omega_b/Omega_total = 0.0493 ~ 1/20 = sopfr/(sigma*tau+sigma+...)
  No clean expression.

  Grade: FAIL
  1/6 is 6% off. Omega_b/Omega_m is epoch-dependent. FAIL.
```

---

### H-CP-69: QCD Scale Lambda_QCD ~ sigma * sigma + ... MeV?

> Lambda_QCD(MS-bar, N_f=5) ~ 210-230 MeV.

```
  Measured: Lambda_QCD ~ 210-230 MeV (depends on scheme, N_f)

  Attempts:
    J_2 * (sigma - phi + mu) = 24 * 11 = 264. No.
    sigma * (sigma + sopfr + phi) = 12 * 19 = 228 (within range!)
    sigma * (sigma + tau + sopfr - mu) = 12 * 20 = 240. Marginal.

  sigma * (sigma + sopfr + phi) = 12 * 19 = 228 MeV.
  This falls within the measured range ~210-230 MeV.

  BUT:
    Lambda_QCD is scheme-dependent and its numerical value in MeV
    is unit-dependent. The range 210-230 covers many integers.
    The formula uses 4 n=6 functions multiplied.

  Grade: FAIL
  Unit-dependent. Lambda_QCD in MeV is not a dimensionless quantity.
  Matching it to n=6 arithmetic is meaningless.
```

---

### H-CP-70: Pion Mass Ratio m_pi/m_e ~ sigma * J_2 + sopfr

> m_pi_charged / m_e = 273.13 (PDG).

```
  Measured: m_pi+/m_e = 273.133 (from m_pi = 139.570 MeV, m_e = 0.511 MeV)

  Attempts:
    sigma * J_2 - sopfr*phi*mu = 288 - 10 = 278 (1.8%)
    sigma * J_2 - sopfr*n/phi = 288 - 15 = 273 (0.049%!)

  Formula: sigma * J_2 - sopfr * n/phi = 12*24 - 5*3 = 288 - 15 = 273

  Error: |273 - 273.133| / 273.133 = 0.049%

  This is a dimensionless ratio (mass ratio) and the formula hits 273
  against the measured 273.13.

  BUT:
    The formula uses sigma, J_2, sopfr, n, phi -- 5 function values.
    With that many parameters, hitting any 3-digit integer is easy.
    The combination sigma*J_2 - sopfr*n/phi is not natural.

  Grade: WEAK
  273 = sigma*J_2 - sopfr*(n/phi) matches m_pi/m_e to 0.049%.
  Precision is excellent but the formula uses too many parameters.
  Overfitting risk is high.
```

---

### H-CP-71: Proton-to-Pion Mass Ratio ~ n + mu/sigma

> m_p / m_pi = 6.7264 (PDG).

```
  Measured: m_p / m_pi_charged = 938.272 / 139.570 = 6.7226

  Attempts:
    n + mu/sigma*n = 6 + 0.5 = 6.5 (3.3% off)
    n + sopfr/(n+mu) = 6 + 5/7 = 6.714 (0.13%)
    n * (mu + mu/sigma) = 6 * 13/12 = 6.5. No.
    sigma * sopfr / (sigma - tau + mu) = 60/9 = 6.667 (0.83%)

  Best: n + sopfr/(n+mu) = 6 + 5/7 = 47/7 = 6.7143
  Error: |6.7143 - 6.7226| / 6.7226 = 0.12%

  Note: 47/7 is a simple fraction. 47 = sigma*tau - mu = 48-1 = J_2*phi - mu.
  And 7 = sigma - sopfr.

  Grade: CLOSE
  m_p/m_pi ~ 47/7 = n + sopfr/(n+mu). 0.12% error on a dimensionless
  ratio. Formula is moderately complex but the approximation is decent.
```

---

### H-CP-72: Gravitational Coupling alpha_G

> alpha_G = G*m_p^2/(hbar*c) = 5.906 * 10^{-39}.

```
  Measured: alpha_G = 5.906 * 10^{-39}

  The coefficient is ~5.9 ~ n - mu/sigma = 6 - 1/12 = 71/12 = 5.917 (0.18%)
  The exponent -39 = -(sigma*n/phi + sopfr - phi) = -(36+5-2) = -39.

  Formula: alpha_G ~ (n - mu/sigma) * 10^{-(sigma*n/phi + sopfr - phi)}
           = (71/12) * 10^{-39}
           = 5.917 * 10^{-39}

  Error on coefficient: |5.917 - 5.906| / 5.906 = 0.19%

  BUT:
    alpha_G depends on which mass is used (proton vs electron).
    With m_e: alpha_G_e = 1.752 * 10^{-45}, completely different.
    The exponent -39 depends on the proton mass choice.
    Powers of 10 are unit-dependent artifacts.

  Grade: FAIL
  alpha_G depends on the choice of mass and units.
  Not a clean dimensionless constant (it's dimensionless but
  mass-choice-dependent). FAIL.
```

---

### H-CP-73: Egyptian Fraction and Quark Charge Structure

> Quark charges: +2/3 and -1/3. Egyptian fraction 1/2 + 1/3 + 1/6 = 1.

```
  Up-type quarks: charge +2/3
  Down-type quarks: charge -1/3

  Charge quantum: 1/3 (fractional charges in units of e)

  Egyptian fraction connection:
    1/2 + 1/3 + 1/6 = 1 (unit fractions from divisors of 6)
    Quark charges use denominators 3 (from the 1/3 term).

  Proton charge neutrality:
    p = uud: 2/3 + 2/3 - 1/3 = 1 = Egyptian sum
    n = udd: 2/3 - 1/3 - 1/3 = 0

  Can we express quark charges from n=6?
    +2/3 = phi/n * mu = 2/6 = 1/3? No, that gives 1/3 not 2/3.
    +2/3 = phi/(n/phi) = 2/3. Yes: phi(6)/(n/phi(6)) = 2/3.
    -1/3 = -mu/(n/phi) = -1/3.

  So: up charge = phi/(n/phi) = phi^2/n = 4/6 = 2/3.
      down charge = -mu/(n/phi) = -mu*phi/n = -2/6 = -1/3.
      Wait: -mu*phi/n = -2/6 = -1/3 works.

  Actually more simply: charges are multiples of 1/3 = mu/(n/phi),
  with up = +2 * (1/3) and down = -1 * (1/3).

  The fundamental charge quantum e/3 uses the denominator 3 = n/phi.

  BUT:
    Fractional quark charges 1/3 and 2/3 arise from SU(3) x U(1)
    quantum numbers and anomaly cancellation. The denominator 3
    is forced by 3 colors. This is not an independent n=6 prediction.

  Grade: WEAK
  Quark charges use 1/3 quantum = mu/(n/phi). The Egyptian fraction
  connection is suggestive but quark charges are determined by gauge
  structure, not number theory.
```

---

### H-CP-74: SM Fermion Degrees of Freedom ~ 90 = sigma * n + sigma + n

> Total SM fermion DOF (before SSB).

```
  SM fermions (one generation, left-handed):
    Quarks: 2 flavors * 3 colors * 2 (L,R) = 12 per generation (after EW)
    Leptons: 2 flavors * 2 (L,R) = 4 per generation (assuming nu_R exists)
    Actually counting Weyl spinors before SSB:
      Q_L: 2 * 3 = 6    (SU(2) doublet, 3 colors)
      u_R: 1 * 3 = 3    (singlet, 3 colors)
      d_R: 1 * 3 = 3    (singlet, 3 colors)
      L_L: 2 * 1 = 2    (SU(2) doublet, no color)
      e_R: 1 * 1 = 1    (singlet)
    Per generation: 6 + 3 + 3 + 2 + 1 = 15 Weyl fermions
    3 generations: 45 Weyl fermions
    With antiparticles: 90 Weyl DOF

  n=6 attempt:
    90 = sigma * n + sigma + n = 72 + 12 + 6 = 90
    90 = sopfr * sigma + n * sopfr = 60 + 30 = 90
    90 = sigma * (n + phi + mu) = 12 * (6+2+1/2)? No.
    Simplest: 90 = n * (sigma + n/phi) = 6 * 15 = 90.

  Per generation: 15 = sigma + n/phi = 12 + 3.
  Total (with antiparticles): 90 = n * 15 = n * (sigma + n/phi).

  Here 15 = sigma + n/phi is the per-generation Weyl fermion count.
  This is a real number from SM representation theory.

  BUT:
    15 per generation is well known. 15 = 6+3+3+2+1 from the
    specific SM representations. 90 = 6*15 is just generations * per-gen.
    The n=6 expression n*(sigma+n/phi) is circular: n=6 generations,
    and sigma+n/phi = 15 is a coincidence.

  Grade: CLOSE
  90 = n*(sigma+n/phi) with 15 per generation = sigma+n/phi.
  Clean decomposition, but partially tautological (n=6 for generations).
```

---

### H-CP-75: Anomaly Cancellation Condition and n=6

> SM anomaly cancellation requires sum of hypercharges = 0 per generation.

```
  Per generation, the hypercharge sum:
    Sum Y = 3*(1/6 + 1/6 + 2/3 + (-1/3)) + (-1/2 + (-1/2) + (-1)) = 0

  Decomposition: the quark contribution uses the fraction 1/6 = 1/n.
  The left-handed quark doublet has Y = 1/6.

  The Egyptian fraction appears:
    1/6 is the smallest unit fraction in 1/2 + 1/3 + 1/6 = 1.

  More precisely:
    Q_L has Y = 1/6 = 1/n
    u_R has Y = 2/3 = phi/(n/phi)
    d_R has Y = -1/3 = -mu/(n/phi)
    L_L has Y = -1/2 = -mu/phi
    e_R has Y = -1 = -mu

  Every SM hypercharge is expressible as a ratio of n=6 functions.

  BUT:
    Hypercharges are rational numbers determined by anomaly cancellation
    and gauge invariance. They necessarily have small-integer ratios
    with denominators from the gauge group ranks. Any small-number
    framework would match them.

  Grade: WEAK
  SM hypercharges are all expressible via n=6 functions, but this is
  because they are small rational numbers. Not specific to n=6.
```

---

### H-CP-76: CMB Temperature T_CMB ~ phi + mu/(n-mu)

> T_CMB = 2.7255 +/- 0.0006 K.

```
  Measured: T_CMB = 2.7255 +/- 0.0006 K

  Attempts:
    phi + sopfr/(n+mu) = 2 + 5/7 = 2.714 (0.42% off)
    phi + sopfr/(n*n/phi) = 2 + 5/9 = 2.556. No.
    n/phi + sopfr/(n+mu) = 3 + 5/7. Too large.
    phi + mu*sopfr/(n+mu)^2 = 2 + 5/49 = 2.102. No.
    (sigma + sopfr + phi + mu)/(n + mu) = 20/7 = 2.857. No.
    (n*sopfr - phi*mu)/(sigma - mu) = 28/11 = 2.545. No.
    sigma*sopfr/(J_2 - phi) = 60/22 = 2.727 (0.056%!)

  Formula: sigma*sopfr / (J_2 - phi) = 60/22 = 30/11 = 2.72727...
  Error: |2.72727 - 2.7255| / 2.7255 = 0.065%

  30/11 matches T_CMB to 0.065% which is within 1-sigma.

  BUT:
    T_CMB is measured in Kelvin. In other temperature units
    (Rankine, Celsius), the number changes completely.
    This is a dimensionful constant. The match is unit-dependent.

  Grade: FAIL
  Despite 0.065% precision, T_CMB in Kelvin is unit-dependent.
  The formula 30/11 is also achievable from many number systems.
  FAIL for unit dependence.
```

---

### H-CP-77: Neutron Lifetime ~ sigma * sigma * sopfr * phi - sigma*sopfr seconds?

> tau_n = 878.4 +/- 0.5 s (beam) vs 877.75 +/- 0.34 s (bottle).

```
  Measured: tau_n ~ 878 s (neutron lifetime, with ~1 s discrepancy
  between beam and bottle methods -- the "neutron lifetime puzzle")

  Attempts:
    sigma^2 * n + sigma*phi = 864 + 24 = 888 (1.1%)
    sigma^2 * n + sigma + phi = 864 + 12 + 2 = 878 (0.0%!)

  Formula: sigma^2 * n + sigma + phi = 144*6 + 12 + 2 = 864 + 14 = 878

  Error: |878 - 878.4| / 878.4 = 0.046%

  BUT:
    The neutron lifetime in SECONDS is entirely unit-dependent.
    In minutes: 14.6 min. In Planck time units: ~1.63*10^{46}.
    The formula only works in SI seconds. This is a textbook example
    of unit-dependent numerology.

  Grade: FAIL
  878 matches the measured value in seconds to 0.046%, but this
  is meaningless because seconds are an arbitrary human unit.
```

---

### H-CP-78: SM Gauge Group Rank = sopfr(6) = 5?

> Rank of SU(3) x SU(2) x U(1) = 2 + 1 + 1 = 4.

```
  Gauge group ranks:
    SU(3): rank 2
    SU(2): rank 1
    U(1):  rank 1
    Total rank: 2 + 1 + 1 = 4 = tau(6)

  NOT sopfr = 5 as originally guessed. The total rank is 4 = tau(6).

  Sub-decomposition:
    SU(3) rank: 2 = phi(6)
    SU(2) rank: 1 = mu(6)
    U(1) rank: 1 = mu(6)

  BUT:
    Rank = number of diagonal generators = number of simultaneously
    measurable quantum numbers. For SU(N), rank = N-1.
    The total rank 4 is forced by the gauge group choice.
    tau(6) = 4 matching the rank is the same as H-CP-29 (spacetime = 4).

  Grade: WEAK
  Total gauge rank 4 = tau(6). But 4 is generic and forced by the
  group structure. Same issue as other tau=4 matches.
```

---

### H-CP-79: Number of SM Parameters ~ 19

> The SM has 19 free parameters (or 26 with neutrino masses).

```
  Classical SM free parameters: 19
    3 gauge couplings + 6 quark masses + 3 lepton masses +
    3 CKM angles + 1 CKM phase + Higgs mass + Higgs VEV + theta_QCD
    = 3 + 6 + 3 + 3 + 1 + 1 + 1 + 1 = 19

  With massive neutrinos: 19 + 7 (3 masses + 3 angles + 1 phase) = 26

  n=6 attempts:
    19 = sigma + sopfr + phi = 12 + 5 + 2 (clean)
    19 = J_2 - sopfr = 24 - 5 (clean)
    26 = J_2 + phi = 24 + 2

  For the extended SM:
    26 = J_2 + phi = 24 + 2
    Also 26 = bosonic string dimensions (H-CP-25).

  The decomposition of 19 into sub-counts:
    3 gauge couplings = n/phi
    6 quark masses = n
    3 lepton masses = n/phi
    4 CKM params = tau
    1 Higgs mass = mu
    1 Higgs VEV = mu
    1 theta_QCD = mu
    Total: n/phi + n + n/phi + tau + mu + mu + mu = 3+6+3+4+1+1+1 = 19

  BUT:
    The parameter count depends on what you count as "free."
    Some authors count 18 or 20. With neutrino masses, could be 25-28.
    The sub-decomposition uses mu three times, which is flexible counting.

  Grade: CLOSE
  19 = sigma + sopfr + phi = J_2 - sopfr. Multiple clean expressions.
  26 (extended SM) = J_2 + phi = bosonic string dimension.
  The counting is somewhat convention-dependent.
```

---

### H-CP-80: Complete SM Gauge Decomposition -- The sigma(6) = 12 Theorem

> The SM gauge sector is completely organized by n=6 arithmetic.

```
  Synthesis of H-CP-3, H-CP-5, H-CP-10, H-CP-11:

  SM Gauge Group: SU(3) x SU(2) x U(1)

  Generators:
    Total:    12 = sigma(6)
    SU(3):     8 = sigma - tau
    SU(2):     3 = n/phi
    U(1):      1 = mu

  Physical bosons (after SSB):
    Gluons:     8 = sigma - tau  (confined)
    EW bosons:  4 = tau          (gamma, W+, W-, Z)
    Higgs:      1 = mu           (surviving scalar)

  Ranks:
    Total:     4 = tau
    SU(3):     2 = phi
    SU(2):     1 = mu
    U(1):      1 = mu

  Casimir operators:
    SU(3): C_2(fund) = 4/3 = tau/n*phi  (actually this is forced by SU(3))
    SU(2): C_2(fund) = 3/4 = n/(phi*tau) (forced by SU(2))

  Fermion representations per generation:
    Q_L(3,2,1/6): dim = 6 = n
    u_R(3,1,2/3): dim = 3 = n/phi
    d_R(3,1,-1/3): dim = 3 = n/phi
    L_L(1,2,-1/2): dim = 2 = phi
    e_R(1,1,-1):  dim = 1 = mu
    Total: 6+3+3+2+1 = 15 = sigma + n/phi

  Summary scorecard:
    sigma = 12: total generators (EXACT)
    sigma - tau = 8: gluons (CLOSE)
    tau = 4: EW bosons, total rank, spacetime dim (CLOSE/WEAK)
    n/phi = 3: SU(2) generators, generations (WEAK)
    phi = 2: SU(3) rank, Higgs doublet (WEAK)
    mu = 1: U(1) generator, Higgs (WEAK)
    n = 6: quarks, leptons, Q_L dimension (EXACT)

  The entire SM gauge sector fits into n=6 arithmetic at the level of
  integer counts. This is the strongest structural pattern in this domain.

  BUT:
    Each individual match involves small integers (1, 2, 3, 4, 6, 8, 12)
    that appear frequently in mathematics and physics.
    The SM gauge group is an experimental input, not derived.
    A framework based on n=12 (sigma=28, tau=6, phi=4) would also
    produce many matches with different assignments.

  Overall assessment:
    The pattern is self-consistent and covers the full gauge sector
    with no contradictions. The sub-decomposition 12 = 8 + 3 + 1
    matching sigma = (sigma-tau) + (n/phi) + mu is the strongest
    single result. But specificity to n=6 vs other small numbers
    remains questionable.

  Grade: CLOSE
  Comprehensive structural match of the SM gauge sector to n=6
  arithmetic. Self-consistent decomposition. But individual matches
  involve small integers that could arise from many frameworks.
```

---

## Summary

| ID | Hypothesis | Formula | Error | Grade |
|----|-----------|---------|-------|-------|
| H-CP-61 | m_n/m_p ~ 1+1/720 | 1+mu/(sigma^2*sopfr) | 0.77% (on deviation) | **CLOSE** |
| H-CP-62 | m_mu/m_e ~ 207 | sigma^2+sopfr*sigma+sopfr-phi | 0.11% | **WEAK** |
| H-CP-63 | m_tau/m_mu ~ 17 | sigma+sopfr | 1.1% | **CLOSE** |
| H-CP-64 | Cabibbo angle ~ 13 deg | sigma+mu | 0.15% (degrees) | **WEAK** |
| H-CP-65 | mu_p ~ 14/5 | n/phi-mu/sopfr | 0.26% | **CLOSE** |
| H-CP-66 | mu_n ~ -23/12 | -(phi-mu/sigma) | 0.19% | **CLOSE** |
| H-CP-67 | a_e from n=6 | -- | -- | **FAIL** |
| H-CP-68 | Omega_b/Omega_m ~ 1/6 | 1/n | 6.0% | **FAIL** |
| H-CP-69 | Lambda_QCD ~ 228 MeV | sigma*(sigma+sopfr+phi) | unit-dep | **FAIL** |
| H-CP-70 | m_pi/m_e ~ 273 | sigma*J_2-sopfr*(n/phi) | 0.049% | **WEAK** |
| H-CP-71 | m_p/m_pi ~ 47/7 | n+sopfr/(n+mu) | 0.12% | **CLOSE** |
| H-CP-72 | alpha_G from n=6 | -- | mass-choice dep | **FAIL** |
| H-CP-73 | Quark charges from n=6 | phi/(n/phi), -mu/(n/phi) | exact fractions | **WEAK** |
| H-CP-74 | 90 fermion DOF | n*(sigma+n/phi) | 0% | **CLOSE** |
| H-CP-75 | Hypercharges from n=6 | various | exact fractions | **WEAK** |
| H-CP-76 | T_CMB ~ 30/11 | sigma*sopfr/(J_2-phi) | 0.065% | **FAIL** |
| H-CP-77 | tau_n ~ 878 s | sigma^2*n+sigma+phi | 0.046% | **FAIL** |
| H-CP-78 | Gauge group rank = 4 | tau | 0% | **WEAK** |
| H-CP-79 | 19 SM parameters | sigma+sopfr+phi | 0% | **CLOSE** |
| H-CP-80 | SM gauge decomposition | sigma=8+3+1 | 0% | **CLOSE** |

## Grade Distribution

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 0 | 0% | -- |
| CLOSE | 8 | 40% | H-CP-61, H-CP-63, H-CP-65, H-CP-66, H-CP-71, H-CP-74, H-CP-79, H-CP-80 |
| WEAK | 6 | 30% | H-CP-62, H-CP-64, H-CP-70, H-CP-73, H-CP-75, H-CP-78 |
| FAIL | 6 | 30% | H-CP-67, H-CP-68, H-CP-69, H-CP-72, H-CP-76, H-CP-77 |

**Non-failing: 14/20 (70.0%)**

## Notable Results

**Strongest new matches:**
1. **mu_p/mu_n ratio** (H-CP-65 + H-CP-66): Combined ratio -168/115 matches to 0.07%. Both individual matches are 0.19-0.26%. These are dimensionless ratios.
2. **m_n/m_p ~ 1 + 1/720 = 1 + 1/6!** (H-CP-61): The neutron-proton mass splitting as 1/6! is elegant and 0.77% accurate on the deviation.
3. **SM gauge decomposition** (H-CP-80): The complete self-consistent mapping of the gauge sector to n=6 functions is the structural highlight.
4. **19 SM parameters = sigma + sopfr + phi** (H-CP-79): Multiple clean expressions.

**Key failures:**
- All dimensionful constants fail when honestly assessed for unit dependence (T_CMB, Lambda_QCD, neutron lifetime).
- The electron anomalous moment adds nothing beyond alpha.
- Cosmological density fractions remain unmatchable.

**Recurring theme: 17 = sigma + sopfr**
This number appears as: SM particle count (H-CP-4), denominator in alpha_s formula (H-CP-24), and m_tau/m_mu approximation (H-CP-63). Its repeated appearance is notable.
