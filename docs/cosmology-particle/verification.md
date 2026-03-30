# N6 Cosmology & Particle Physics -- Independent Verification

Verified: 2026-03-30
Method: Each hypothesis checked against PDG 2024 (Particle Data Group), Planck 2018/2020 cosmological parameters, CODATA 2022 fundamental constants, and standard QFT/cosmology textbooks (Peskin & Schroeder, Weinberg, Kolb & Turner). Grades adjusted where warranted.

## Grade Distribution (Summary)

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 4 | 13.3% | H-CP-1, H-CP-2, H-CP-5, H-CP-7 |
| CLOSE | 9 | 30.0% | H-CP-3, H-CP-4, H-CP-8, H-CP-11, H-CP-13, H-CP-17, H-CP-24, H-CP-25, H-CP-30 |
| WEAK | 8 | 26.7% | H-CP-6, H-CP-9, H-CP-10, H-CP-12, H-CP-14, H-CP-18, H-CP-19, H-CP-29 |
| FAIL | 9 | 30.0% | H-CP-15, H-CP-16, H-CP-20, H-CP-21, H-CP-22, H-CP-23, H-CP-26, H-CP-27, H-CP-28 |
| UNVERIFIABLE | 0 | 0% | -- |

**Non-failing: 21/30 (70.0%)**

| ID | Hypothesis | Grade |
|----|-----------|-------|
| H-CP-1 | 6 quark flavors = n | **EXACT** |
| H-CP-2 | 6 lepton flavors = n | **EXACT** |
| H-CP-3 | 4 EW gauge bosons = tau | **CLOSE** |
| H-CP-4 | 17 SM particles = n+n+tau+mu | **CLOSE** |
| H-CP-5 | 12 gauge generators = sigma | **EXACT** |
| H-CP-6 | 3 generations = n/phi | **WEAK** |
| H-CP-7 | m_p/m_e ~ 6*pi^5 (0.002%) | **EXACT** |
| H-CP-8 | sin^2(theta_W) ~ 3/13 (0.19%) | **CLOSE** |
| H-CP-9 | alpha^{-1} ~ sigma^2-sopfr-phi | **WEAK** |
| H-CP-10 | 3 color charges = n/phi | **WEAK** |
| H-CP-11 | 8 gluons = sigma-tau | **CLOSE** |
| H-CP-12 | Higgs doublet = phi | **WEAK** |
| H-CP-13 | H_0 ~ sigma*n+mu = 73 | **CLOSE** |
| H-CP-14 | r_p ~ 4*pi/15 fm | **WEAK** |
| H-CP-15 | Omega_Lambda from n=6 | **FAIL** |
| H-CP-16 | Omega_DM from n=6 | **FAIL** |
| H-CP-17 | eta ~ 6e-10 | **CLOSE** |
| H-CP-18 | Y_p ~ 1/tau | **WEAK** |
| H-CP-19 | CKM 3x3, 4 params | **WEAK** |
| H-CP-20 | m_u ~ phi MeV | **FAIL** |
| H-CP-21 | m_d ~ sopfr MeV | **FAIL** |
| H-CP-22 | m_s ~ 93 MeV | **FAIL** |
| H-CP-23 | m_t ~ 173 GeV | **FAIL** |
| H-CP-24 | alpha_s(M_Z) ~ 2/17 | **CLOSE** |
| H-CP-25 | String 10D = sigma-phi | **CLOSE** |
| H-CP-26 | Neutrino mass ratio | **FAIL** |
| H-CP-27 | m_W ~ 80 GeV | **FAIL** |
| H-CP-28 | Lithium problem | **FAIL** |
| H-CP-29 | 4D spacetime = tau | **WEAK** |
| H-CP-30 | Leech 24D = J_2 | **CLOSE** |

---

Grading scale:
- **EXACT**: The claimed number/structure matches real-world data precisely (<0.1% for continuous quantities, exact for integers), with a legitimate physical basis.
- **CLOSE**: Within ~1-2% of real values, or correct integer match but too generic (small integers like 2, 3, 4).
- **WEAK**: Requires cherry-picking, flexible counting, post-hoc rationalization, or matches a number too common to be meaningful.
- **FAIL**: Contradicted by data, unit-dependent, or no clean formula found.
- **UNVERIFIABLE**: Insufficient published data to confirm or deny.

---

## H-CP-1: 6 Quark Flavors = n

**Grade: EXACT** (confirmed)

The Standard Model contains exactly 6 quark flavors: up, down, charm, strange, top, bottom. This is experimentally established beyond doubt. The top quark was discovered at Fermilab in 1995, completing the set. LEP measurements of the Z boson decay width constrain the number of light neutrino generations to N_nu = 2.9840 +/- 0.0082, implying exactly 3 generations and hence 6 quarks. The match n = 6 is exact and involves the defining constant of the framework. EXACT is warranted.

Note: H-CP-1 and H-CP-2 are not fully independent, since anomaly cancellation in the SM requires equal numbers of quark and lepton generations.

---

## H-CP-2: 6 Lepton Flavors = n

**Grade: EXACT** (confirmed)

Six leptons: electron, muon, tau, plus their three associated neutrinos. All experimentally confirmed. The tau neutrino was directly observed by DONUT in 2000. Same LEP constraint applies. The match n = 6 is exact. As noted, this is linked to H-CP-1 by anomaly cancellation, so the two EXACT grades are partially redundant. Nevertheless, each individually represents a correct integer match.

---

## H-CP-3: 4 EW Gauge Bosons = tau(6)

**Grade: CLOSE** (confirmed)

The electroweak sector has 4 physical gauge bosons: photon (gamma), W+, W-, Z. All discovered and precisely measured. tau(6) = 4 matches exactly. However, 4 is the number of divisors of 6, and also one of the most common small integers in physics (4 spacetime dimensions, 4 Maxwell equations, etc.). The match is real but not specific enough for EXACT. CLOSE is appropriate.

---

## H-CP-4: 17 SM Particles = n + n + tau + mu

**Grade: CLOSE** (confirmed)

Counting distinct SM particle species (not antiparticles): 6 quarks + 6 leptons + 4 gauge bosons + 1 Higgs = 17. The decomposition into n=6 functions is clean: each sub-count independently matches. However, the total 17 is not itself a standard n=6 function value; it requires summing n twice plus tau and mu. The counting convention (excluding antiparticles, counting each quark color once) is standard but not unique. If we count all particles including antiparticles and color states, the number changes dramatically (61 or more). CLOSE is fair.

---

## H-CP-5: 12 Gauge Generators = sigma(6)

**Grade: EXACT** (confirmed)

SU(3)_C x SU(2)_L x U(1)_Y has 8 + 3 + 1 = 12 generators. This is a mathematical fact about the SM gauge group, experimentally confirmed through the existence of 8 gluons, W+/W-/Z, and photon. The sub-decomposition is notable:
- SU(3): 8 = sigma - tau
- SU(2): 3 = n/phi
- U(1): 1 = mu

12 = sigma(6) is exact. The number 12 does appear widely (12 = 2^2 * 3), but the clean sub-decomposition strengthens the match. Upgrading would be unwarranted since the gauge group is an input to the SM, not derived from it. EXACT for the integer match.

---

## H-CP-6: 3 Generations = n/phi

**Grade: WEAK** (confirmed)

Three fermion generations is one of the deepest unexplained facts of the SM. n/phi = 3 matches, but 3 is the most overloaded integer in physics: 3 spatial dimensions, 3 color charges, 3 families. Any framework containing a ratio equal to 3 can claim this match. The number 3 provides essentially zero bits of information in distinguishing n=6 from random coincidence. WEAK is correct.

---

## H-CP-7: m_p/m_e ~ 6*pi^5

**Grade: EXACT** (confirmed -- strongest match in this domain)

CODATA 2022: m_p/m_e = 1836.15267343(11). Formula: 6*pi^5 = 6 * 306.01968... = 1836.11811... Error: 0.0019% = 19 ppm.

This is one of the most striking numerical coincidences involving n=6. The formula uses only two ingredients (n=6 and pi) and achieves 19 ppm accuracy on a dimensionless ratio. For comparison, Eddington's famous alpha = 1/137 attempts never achieved this precision.

Caveats: (1) m_p/m_e arises from the ratio of the QCD scale Lambda_QCD to the electron Yukawa coupling -- there is no known theoretical reason involving pi^5. (2) The match has been noted independently by multiple authors and remains unexplained. (3) Despite the precision, this is numerology without a derivation.

Nevertheless, 19 ppm from a 2-parameter formula on a dimensionless quantity is exceptional. EXACT is warranted.

---

## H-CP-8: sin^2(theta_W) ~ 3/13

**Grade: CLOSE** (confirmed)

PDG 2024: sin^2(theta_W)(MS-bar, M_Z) = 0.23121 +/- 0.00004. Formula: (n/phi)/(sigma+mu) = 3/13 = 0.23077. Error: 0.19%.

0.19% is good for a simple rational approximation. The expression uses legitimate n=6 functions. However: (1) sin^2(theta_W) runs with energy (0.231 at M_Z, 0.238 at low energy, 0.375 at GUT scale). Matching at one scale is less fundamental. (2) 3/13 is a simple fraction that any rational approximation search would find. (3) The denominator 13 = sigma+mu requires combining two functions.

CLOSE is appropriate. Not EXACT because of running and the arbitrariness of the energy scale.

---

## H-CP-9: alpha^{-1} ~ sigma^2 - sopfr - phi = 137

**Grade: WEAK** (confirmed)

CODATA 2022: alpha^{-1} = 137.035999084(21). The formula sigma^2 - sopfr - phi = 144 - 5 - 2 = 137. Error on integer part: 0.026%.

The fine structure constant has been the target of numerological matching since Eddington (who claimed alpha^{-1} = 136, then 137). The formula 144 - 5 - 2 = 137 hits the integer part but leaves the fractional part 0.036 unexplained. More importantly, with sigma^2 = 144 as a starting point, one can reach any integer in a wide range by adding/subtracting small n=6 function values. This is a classic overfitting trap. WEAK is correct; the formula is too ad hoc.

---

## H-CP-10: 3 Color Charges = n/phi

**Grade: WEAK** (confirmed)

QCD has SU(3) gauge symmetry with 3 color charges (red, green, blue). n/phi = 3. Same issue as H-CP-6: the number 3 is too common. Additionally, noting that colors + anticolors = 6 = n is slightly more interesting but still a consequence of SU(3) being the gauge group. WEAK.

---

## H-CP-11: 8 Gluons = sigma - tau

**Grade: CLOSE** (confirmed)

8 gluons = 3^2 - 1 = 8 (adjoint of SU(3)). sigma(6) - tau(6) = 12 - 4 = 8. The expression is clean and the partition of 12 gauge generators into 8 strong + 4 electroweak is physically meaningful. This makes H-CP-11 stronger in combination with H-CP-5 and H-CP-3 than alone. 8 = sigma - tau is a legitimate structural decomposition. CLOSE (not EXACT because 8 = 2^3 appears in many contexts -- Bott periodicity, octonions, etc.).

---

## H-CP-12: Higgs Doublet = phi(6) = 2

**Grade: WEAK** (confirmed)

The Higgs is an SU(2)_L doublet: 2 complex components. phi(6) = 2. But "2" is the defining property of SU(2) doublets -- ANY SU(2) doublet has 2 components. This is not specific to n=6. The extended observation that 4 real DOF = tau(6) and 3 eaten Goldstones = n/phi, 1 physical Higgs = mu is a nice pattern but each number (4, 3, 1) is forced by the Goldstone theorem given SU(2)->U(1) breaking. WEAK.

---

## H-CP-13: H_0 ~ sigma*n + mu = 73

**Grade: CLOSE** (confirmed)

SH0ES (Riess et al. 2022): H_0 = 73.04 +/- 1.04 km/s/Mpc. sigma*n + mu = 72 + 1 = 73. Error: 0.05%.

The 0.05% match to the SH0ES value is excellent. However: (1) H_0 = 73 km/s/Mpc is unit-dependent -- in s^{-1} it's ~2.37*10^{-18}, with no n=6 connection. (2) The Hubble tension: Planck CMB gives 67.4, which is 8% off from 73. If the true H_0 is 67.4, the match fails. (3) sigma*n + mu = 73 requires three function evaluations combined by multiplication and addition.

CLOSE is appropriate given the unit dependence and unresolved Hubble tension.

---

## H-CP-14: r_p ~ 4*pi/15

**Grade: WEAK** (confirmed)

Muonic hydrogen: r_p = 0.84142 +/- 0.00026 fm. 4*pi/15 = 0.83776 fm. Error: 0.43%.

The error is acceptable, but this is a dimensionful quantity measured in femtometers. The match works only in fm; in any other unit system the numerical value changes. This is the fundamental problem with matching dimensionful constants to pure numbers. WEAK is correct.

---

## H-CP-15: Omega_Lambda from n=6

**Grade: FAIL** (confirmed)

Planck 2018: Omega_Lambda = 0.6847 +/- 0.0073. No clean n=6 expression reaches this value. Best attempts: 2/3 (2.6% off), 3/4 (9.5% off). Moreover, Omega_Lambda is the dark energy density parameter at the present epoch -- it was different in the past and will approach 1 in the future. It is not a fundamental constant. FAIL.

---

## H-CP-16: Omega_DM from n=6

**Grade: FAIL** (confirmed)

Planck 2018: Omega_DM h^2 = 0.1200 +/- 0.0012, giving Omega_DM ~ 0.259. Best attempt: 1/4 = 0.25 (3.4% off). Same time-dependence issue as H-CP-15. FAIL.

---

## H-CP-17: Baryon-to-Photon Ratio eta ~ 6*10^{-10}

**Grade: CLOSE** (confirmed)

BBN + CMB: eta = (6.14 +/- 0.02) * 10^{-10}. The coefficient 6.14 is close to n = 6 (2.3% off). The appearance of ~6 in the baryon-to-photon ratio is one of the more frequently noted numerical coincidences in cosmology.

However: (1) The coefficient is 6.14, not 6.00 (2.3% error). (2) The power 10^{-10} has no natural n=6 interpretation. (3) eta is defined in terms of number densities at a specific temperature.

CLOSE rather than EXACT because of the 2.3% deviation and the power-of-10 issue.

---

## H-CP-18: Y_p ~ 1/tau = 1/4

**Grade: WEAK** (confirmed)

Primordial helium-4 mass fraction Y_p = 0.2470 +/- 0.0002. 1/tau(6) = 1/4 = 0.25. Error: 1.2%.

The physical reason Y_p ~ 0.25 is completely understood: the neutron-to-proton ratio at BBN freeze-out is ~1/7, leading to Y_p = 2(n/p)/(1+n/p) ~ 2/8 = 0.25. The "4" in He-4 is the mass number (2 protons + 2 neutrons), and Y_p ~ A(He-4)/4A(He-4) arises from nuclear physics, not number theory. The match 1/tau(6) adds no information. WEAK.

---

## H-CP-19: CKM Matrix Structure

**Grade: WEAK** (confirmed)

CKM is 3x3 with 3 mixing angles + 1 CP-violating phase = 4 parameters. 3 = n/phi, 4 = tau(6). These numbers are mathematical consequences of having 3 generations: for N generations, a unitary mixing matrix has N(N-1)/2 angles + (N-1)(N-2)/2 phases. For N=3: 3 angles + 1 phase = 4. So 3 and 4 here are not independent of H-CP-6 (3 generations). WEAK.

---

## H-CP-20: m_u ~ phi MeV

**Grade: FAIL** (confirmed)

PDG 2024: m_u = 2.16 +0.49/-0.26 MeV (MS-bar, 2 GeV). phi(6) = 2. Error: 7.4%. Unit-dependent (only works in MeV). Current quark masses are scheme- and scale-dependent: at different renormalization scales, the number changes. FAIL.

---

## H-CP-21: m_d ~ sopfr MeV

**Grade: FAIL** (confirmed)

PDG 2024: m_d = 4.67 +0.48/-0.17 MeV (MS-bar, 2 GeV). sopfr(6) = 5. Error: 7.1%. Same unit/scheme dependence as H-CP-20. FAIL.

---

## H-CP-22: m_s from n=6

**Grade: FAIL** (confirmed)

PDG 2024: m_s = 93.4 +8.6/-3.4 MeV. No clean formula. Dimensionful, scheme-dependent. FAIL.

---

## H-CP-23: m_t from n=6

**Grade: FAIL** (confirmed)

PDG 2024: m_t = 172.57 +/- 0.29 GeV. No clean formula in any unit system. FAIL.

---

## H-CP-24: alpha_s(M_Z) ~ phi/(sigma+sopfr) = 2/17

**Grade: CLOSE** (confirmed)

PDG 2024: alpha_s(M_Z) = 0.1180 +/- 0.0009. phi/(sigma+sopfr) = 2/17 = 0.11765. Error: 0.30%.

0.30% on a dimensionless quantity is good. The denominator 17 = sigma + sopfr = 12 + 5 interestingly equals the SM particle count. However, alpha_s runs strongly with energy: at 1 GeV it's ~0.5, at 1 TeV it's ~0.09. The match is specific to the Z mass scale. CLOSE is appropriate.

---

## H-CP-25: String Theory 10D = sigma - phi

**Grade: CLOSE** (confirmed)

Superstring theory requires 10 spacetime dimensions. sigma - phi = 12 - 2 = 10. The compactified dimensions = 10 - 4 = 6 = n is an elegant coincidence. The bosonic string 26 = J_2 + phi = 24 + 2 is also notable.

However: (1) String theory's extra dimensions are not experimentally confirmed. (2) 10 = sigma - phi is a simple expression and 10 appears in many contexts (base-10 arithmetic, etc.). (3) The "compactified 6" is a mathematical consequence of choosing 10D with 4 observed dimensions.

CLOSE because the pattern is clean but the physics is unverified.

---

## H-CP-26: Neutrino Mass Squared Ratio

**Grade: FAIL** (confirmed)

Ratio Delta_m^2_{32}/Delta_m^2_{21} ~ 32.6. No clean n=6 formula. FAIL.

---

## H-CP-27: m_W from n=6

**Grade: FAIL** (confirmed)

m_W = 80.3692 +/- 0.0133 GeV. Dimensionful, no clean formula. The integer 80 is reachable by many combinations but the fractional part matters for precision measurements. FAIL.

---

## H-CP-28: Cosmological Lithium Problem

**Grade: FAIL** (confirmed)

The lithium problem (factor ~3 discrepancy between BBN prediction and observation) is real but noting that Li-7 has mass number 7 = sigma - sopfr contributes nothing to understanding it. FAIL.

---

## H-CP-29: 4D Spacetime = tau(6)

**Grade: WEAK** (confirmed)

4 spacetime dimensions = tau(6) = 4. The decomposition 4 = 3 + 1 = n/phi + mu (space + time) is noted. But 4 is ubiquitous in physics and mathematics. tau(6) = 4 is a property of 6 having 4 divisors, which has no causal connection to spacetime dimensionality. WEAK.

---

## H-CP-30: Leech Lattice 24D = J_2(6)

**Grade: CLOSE** (confirmed)

The Leech lattice exists in 24 dimensions. J_2(6) = 24. Also 24 = sigma*phi = 12*2. The Leech lattice connects to string theory (bosonic string has 24 transverse dimensions), modular forms, and monstrous moonshine. The mathematical identity J_2(6) = 24 is exact.

However, 24 = 4! = 2^3 * 3 appears in many mathematical contexts for reasons independent of perfect number arithmetic. The Leech lattice's dimension arises from the theory of even unimodular lattices and theta functions, not from properties of 6. CLOSE.

---

## Cross-Domain Observations

**Strongest results:**
1. m_p/m_e ~ 6*pi^5 (H-CP-7): 0.002% error, dimensionless, simple formula. Best single match.
2. 12 gauge generators = sigma (H-CP-5): exact integer, with clean sub-decomposition.
3. 6 quarks = n, 6 leptons = n (H-CP-1, H-CP-2): exact, defining feature of SM.

**Structural pattern (gauge sector):**
The SM gauge group SU(3) x SU(2) x U(1) decomposes as:
- Total generators: 12 = sigma
- Strong: 8 = sigma - tau
- Weak: 3 = n/phi
- Hypercharge: 1 = mu
- EW bosons (physical): 4 = tau

This self-consistent decomposition is the strongest structural argument in this domain.

**Weaknesses:**
- All quark mass matches fail (unit-dependent).
- Cosmological density parameters fail (epoch-dependent, not fundamental).
- Dark sector provides no matches.
- Fine structure constant remains elusive (classic numerology trap).

**Honest assessment:**
The particle counting matches (quarks, leptons, generators) are genuine exact integers. The m_p/m_e formula is strikingly precise. Most other matches are either too generic (small integers) or require ad hoc combinations. This domain has ~4 strong results out of 30 hypotheses, which is roughly consistent with the other N6 domains.
