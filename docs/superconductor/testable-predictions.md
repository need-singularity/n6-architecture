# N6 Superconductor/Magnet Domain — Testable Predictions (v2)

> Falsifiable predictions derived from n=6 arithmetic applied to superconductor physics,
> high-Tc materials, vortex physics, and magnet engineering.
> Each prediction: specific numerical value, n=6 formula, verification method,
> falsification criterion, related BT, confidence rating.
>
> v2 reorganized: 27 predictions across 4 tiers, 3 honest negative/null predictions retained.
>
> Constants: n=6, sigma=12, phi=2, tau=4, J_2=24, sopfr=5, mu=1

---

## Tier 1: Today Verifiable (Literature Search / Database Check)

### TP-SC-01: Abrikosov Vortex Lattice = Hexagonal (CN=6=n)

**Prediction**: ALL Type-II superconductors in the mixed state (Hc1 < H < Hc2) form Abrikosov vortex lattices with coordination number exactly 6 = n.

| Item | Value |
|------|-------|
| n=6 formula | CN = n = 6 (2D kissing number) |
| Verification | Survey decoration experiments, Bitter patterns, neutron scattering, STM data across >20 Type-II materials (Nb, NbSe2, YBCO, MgB2, BSCCO, FeSe) |
| Expected result | Triangular (hexagonal) lattice in all clean, equilibrium cases |
| Falsification | Any clean Type-II SC shows equilibrium square lattice as ground state (note: square lattice near Hc2 in d-wave is a known exception at high fields) |
| Related BT | BT-122 (hexagonal geometry universality), H-SC-01 |
| Confidence | ★★★ (already confirmed -- Abrikosov 1957, Essmann & Trauble 1967) |

---

### TP-SC-02: YBCO Metal Atom Ratio {1,2,3} = Proper Divisors of 6

**Prediction**: YBa2Cu3O7 metal atom ratio Y:Ba:Cu = 1:2:3, sum = 6 = n, set = {1,2,3} = proper divisors of 6. No other high-Tc cuprate with comparable Tc has the same divisor-set property.

| Item | Value |
|------|-------|
| n=6 formula | {1,2,3} = div(6), 1+2+3 = n = 6 |
| Verification | X-ray crystallography databases (ICSD, COD). Check all cuprates with Tc > 77K for metal ratio patterns |
| Expected result | YBCO uniquely has sum=6 among major cuprate families |
| Falsification | Another high-Tc cuprate (Tc > 90K) also has metal ratio sum = 6 with same divisor set (would weaken uniqueness, not falsify the match) |
| Related BT | H-SC-02 |
| Confidence | ★★★ (crystallographic fact) |

---

### TP-SC-03: Nb3Sn Unit Cell Nb Atoms = 6 = n

**Prediction**: In the A15 (Cr3Si-type) structure of Nb3Sn, each unit cell contains exactly 6 Nb atoms and 2 = phi Sn atoms, total 8 = sigma-tau per cell.

| Item | Value |
|------|-------|
| n=6 formula | Nb = n = 6, Sn = phi = 2, total = sigma-tau = 8 |
| Verification | A15 structure Wyckoff positions: Nb at 6c (or 6d), Sn at 2a. Any crystallography reference (Pearson's handbook) |
| Expected result | 6 Nb + 2 Sn per unit cell, confirmed |
| Falsification | A15 Nb3Sn unit cell contains a different number of Nb atoms (would be a crystallographic error) |
| Related BT | H-SC-03 |
| Confidence | ★★★ (crystallographic fact, verified in Strukturbericht) |

---

### TP-SC-04: Cooper Pair Electron Count = phi(6) = 2

**Prediction**: The fundamental charge carrier in all superconductors is the Cooper pair with exactly 2 = phi(6) electrons. No triplet (3-electron) or quadruplet (4-electron) pairing has been confirmed in any SC.

| Item | Value |
|------|-------|
| n=6 formula | pair size = phi(6) = 2 |
| Verification | BCS theory (1957). Survey all known superconducting mechanisms: BCS, Eliashberg, spin-fluctuation, phonon-mediated |
| Expected result | All confirmed SCs use 2-electron pairing |
| Falsification | Discovery of a confirmed superconductor with 4-fermion condensation as ground state (charge-4e SC) |
| Related BT | H-SC-06 |
| Confidence | ★★★ (BCS fundamental; charge-4e proposals exist but unconfirmed) |

---

### TP-SC-05: Three Macroscopic Quantum Effects = n/phi = 3

**Prediction**: Superconductivity manifests exactly 3 = n/phi macroscopic quantum effects: (1) flux quantization, (2) Josephson effect, (3) Meissner effect. These three exhaust the independent macroscopic consequences of the SC order parameter Psi.

| Item | Value |
|------|-------|
| n=6 formula | macroscopic quantum effects = n/phi = 3 |
| Verification | Textbook survey: Tinkham, Rose-Innes & Rhoderick, de Gennes. Count independent macroscopic quantum phenomena derivable from Psi = \|Psi\|e^{i*theta} |
| Expected result | \|Psi\|^2 -> Meissner, single-valuedness of theta -> flux quantization, Delta(theta) across weak link -> Josephson. Three and only three |
| Falsification | Identification of a 4th independent macroscopic quantum effect not reducible to the above three |
| Related BT | H-SC-09 |
| Confidence | ★★ (well-established classification, but boundary cases like Andreev reflection could be argued) |

---

### TP-SC-06: Four Hallmark SC Phenomena = tau(6) = 4

**Prediction**: The superconducting transition is characterized by exactly 4 = tau(6) hallmark phenomena: (1) zero resistance, (2) Meissner effect, (3) specific heat jump, (4) energy gap.

| Item | Value |
|------|-------|
| n=6 formula | hallmarks = tau(6) = 4 |
| Verification | Textbook consensus: Tinkham Ch. 1-3, Ashcroft & Mermin Ch. 34. Count primary signatures of the SC phase transition |
| Expected result | The standard pedagogical four: R=0, Meissner, C_v jump, Delta gap |
| Falsification | A 5th equally fundamental hallmark is established (NMR relaxation, thermal conductivity are secondary) |
| Related BT | H-SC-08 |
| Confidence | ★★ (classification-dependent; robust across major textbooks) |

---

### TP-SC-07: Two Josephson Relations = phi(6) = 2

**Prediction**: The Josephson junction is completely described by exactly 2 = phi(6) fundamental equations: DC (I = I_c sin(Delta_phi)) and AC (V = (hbar/2e)(d(Delta_phi)/dt)).

| Item | Value |
|------|-------|
| n=6 formula | Josephson equations = phi(6) = 2 |
| Verification | Any Josephson junction textbook. Count independent fundamental relations |
| Expected result | Two and only two: DC + AC. All other junction phenomena derive from these |
| Falsification | A third independent Josephson equation is discovered (cos-phi term in some unconventional junctions?) |
| Related BT | H-SC-12 |
| Confidence | ★★ (well-established; phi-0 junctions add complexity but not a new fundamental equation) |

---

### TP-SC-08: MgB2 Atomic Numbers: Mg Z=12=sigma, B Z=5=sopfr

**Prediction**: In MgB2, the two constituent elements have atomic numbers Z(Mg)=12=sigma(6) and Z(B)=5=sopfr(6). Both are exact matches to distinct n=6 functions.

| Item | Value |
|------|-------|
| n=6 formula | Z(Mg) = sigma = 12, Z(B) = sopfr = 5 |
| Verification | Periodic table (trivial). Significance test: among all binary SC compounds, what fraction has both Z values matching n=6 functions? |
| Expected result | MgB2 is one of very few binary SCs where both Z match distinct n=6 functions |
| Falsification | >30% of binary SCs have both Z matching n=6 functions (would indicate the pattern is not specific) |
| Related BT | H-SC-04 |
| Confidence | ★★ (numerically exact, but causal connection absent) |

---

## Tier 2: Lab Verification (1-3 Years, Existing Equipment)

### TP-SC-09: ITER TF Coils = 3n = 18, CS Modules = n = 6

**Prediction**: ITER uses exactly 18 = 3n toroidal field coils and 6 = n central solenoid modules. This n=6 pattern will persist in next-generation tokamak designs.

| Item | Value |
|------|-------|
| n=6 formula | TF = 3n = 18, CS = n = 6 |
| Verification | ITER design documents (ITER Organization, public). Check SPARC, ARC, DEMO coil counts |
| Expected result | ITER: 18 TF + 6 CS confirmed. SPARC: 18 TF (same). DEMO: likely 16 or 18 TF |
| Falsification | Next-gen tokamak (DEMO or STEP) uses TF count not divisible by n=6 (e.g., 14 or 20 TF coils) |
| Related BT | BT-99 (tokamak q=1 as perfect number) |
| Confidence | ★★★ (ITER/SPARC confirmed; DEMO TBD) |

---

### TP-SC-10: Optimal CuO2 Plane Count = n/phi = 3 (Tc Maximum)

**Prediction**: Across all cuprate families (Bi, Tl, Hg, Y), the critical temperature Tc is maximized at exactly n_L = 3 = n/phi CuO2 planes per unit cell. No cuprate with n_L = 4+ surpasses the n_L = 3 record.

| Item | Value |
|------|-------|
| n=6 formula | optimal n_L = n/phi = 3 |
| Verification | Synthesize and characterize n_L = 3 vs n_L = 4 members in a NEW cuprate family (if discovered). Or survey comprehensive databases |
| Expected result | Tc(n_L=3) > Tc(n_L=4) in every cuprate family tested |
| Falsification | A cuprate with n_L = 4 or 5 achieves Tc > 134K (current Hg-1223 record at ambient pressure) |
| Related BT | H-SC-05 |
| Confidence | ★★★ (experimentally well-established across 4+ cuprate families) |

---

### TP-SC-11: WHH Coefficient = ln(2) = ln(phi) Universality

**Prediction**: The WHH (Werthamer-Helfand-Hohenberg) orbital limiting formula Hc2(0) = -0.6932 * Tc * (dHc2/dT)|_Tc applies universally with coefficient ln(2) = ln(phi(6)) = 0.6932 for all clean-limit s-wave superconductors.

| Item | Value |
|------|-------|
| n=6 formula | WHH coefficient = ln(phi) = ln(2) = 0.6932 |
| Verification | Measure dHc2/dT near Tc for 10+ clean s-wave SCs. Compare Hc2(0) with WHH prediction |
| Expected result | WHH with ln(2) coefficient accurate to <5% for clean, single-band, s-wave SCs |
| Falsification | Systematic deviation >10% in clean s-wave SCs (not attributable to multiband or paramagnetic effects) |
| Related BT | H-SC-07 |
| Confidence | ★★ (analytic result from Gorkov equations; deviations in multiband/d-wave materials are expected) |

---

### TP-SC-12: BCS Gap Ratio 3.528 -- No Clean n=6 Expression (Honest Null)

**Prediction**: The BCS universal gap ratio 2Delta(0)/(k_B*Tc) = 3.528 does NOT have a clean n=6 expression. Closest: (sopfr+n)/(n/phi) = 11/3 = 3.667 (3.9% off). The BCS ratio derives from pi and Euler's gamma, not number theory.

| Item | Value |
|------|-------|
| n=6 formula | **None found** (honest null) |
| Verification | Measure 2Delta(0)/(k_B*Tc) in 10+ weak-coupling BCS SCs (Al, In, Sn, Zn, V) |
| Expected result | Ratio clusters at 3.528 +/- 5% for weak-coupling SCs |
| Falsification | N/A -- this is a falsifiability anchor showing where n=6 does NOT apply |
| Related BT | -- |
| Confidence | ★ (retained for honesty: n=6 does not map to BCS gap ratio) |

**Note**: This is an honest null prediction. n=6 does NOT cleanly map to the BCS gap ratio. Retained as a falsifiability anchor.

---

### TP-SC-13: Hc2/Tc Ratio -- Does Not Match sigma-phi (Honest Negative)

**Prediction**: For Nb-based A15 superconductors (Nb3Sn, Nb3Al, Nb3Ge), the ratio Hc2(0)/Tc clusters near 1.6-1.8 T/K, NOT near any n=6 expression (sigma-phi=10, sopfr=5, etc.).

| Item | Value |
|------|-------|
| n=6 formula | **None** -- actual ratio ~1.7 T/K, not expressible as simple n=6 |
| Verification | Nb3Sn: Hc2~30T/Tc=18.3K = 1.64 T/K. Nb3Al: ~1.75 T/K. Nb3Ge: ~1.61 T/K |
| Expected result | Ratios cluster near 1.6-1.8 T/K |
| Falsification | Already essentially falsified as an n=6 mapping |
| Related BT | H-SC-03 |
| Confidence | ★ (honest: the data do not support this mapping) |

**Note**: Retained as negative evidence. Hc2/Tc ratio is determined by electronic mean free path and Fermi velocity, not n=6 arithmetic.

---

### TP-SC-14: SC Qubit Archetypes = n/phi = 3

**Prediction**: All superconducting qubits are variants of exactly 3 = n/phi fundamental archetypes: charge qubit, flux qubit, and phase qubit. Modern devices (transmon, fluxonium, 0-pi) are evolved versions of these three. No genuinely new 4th archetype will emerge.

| Item | Value |
|------|-------|
| n=6 formula | qubit archetypes = n/phi = 3 |
| Verification | Devoret & Schoelkopf (2013), Clarke & Wilhelm (2008). Map all SC qubit types to conjugate variable pairs (Q, Phi, phi) |
| Expected result | All new qubits map to E_C/E_J/E_L ratios of the three archetypes |
| Falsification | A 4th SC qubit archetype based on a genuinely new conjugate variable is demonstrated |
| Related BT | H-SC-11 |
| Confidence | ★★ (strong for now; topological qubits may eventually create a 4th category) |

---

### TP-SC-15: MgB2 Two-Gap Ratio Delta_sigma/Delta_pi ~ n/phi = 3

**Prediction**: MgB2 has two superconducting gaps. The ratio of the larger sigma-band gap to the smaller pi-band gap is approximately n/phi = 3.

| Item | Value |
|------|-------|
| n=6 formula | Delta_sigma / Delta_pi = n/phi = 3 |
| Verification | Point-contact spectroscopy, STM, specific heat fitting on MgB2 single crystals. Published values: Delta_sigma ~ 7.1 meV, Delta_pi ~ 2.3 meV |
| Expected result | Delta_sigma/Delta_pi ~ 7.1/2.3 ~ 3.09 |
| Falsification | Ratio is outside [2.5, 3.5] across multiple measurement techniques |
| Related BT | H-SC-04 |
| Confidence | ★★★ (experimentally measured ratio ~3.0-3.1, excellent match to n/phi=3) |

---

### TP-SC-16: Vortex Pinning Optimal Defect Size ~ xi * n/phi

**Prediction**: The optimal artificial pinning center size for maximum J_c enhancement in Type-II SCs is approximately n/phi = 3 times the coherence length xi.

| Item | Value |
|------|-------|
| n=6 formula | optimal defect diameter d_opt ~ (n/phi) * xi = 3*xi |
| Verification | Irradiation studies with controlled columnar defect diameters in YBCO and REBCO films. Vary d from 1*xi to 6*xi, measure J_c |
| Expected result | J_c maximum at d ~ 2-4 xi, with peak near 3*xi |
| Falsification | Peak J_c consistently at d = xi (1:1 matching) or d > 5*xi |
| Related BT | H-SC-01 |
| Confidence | ★★ (theoretical maximum pinning at d ~ few*xi is expected; exact factor TBD experimentally) |

---

## Tier 3: Future Verification (5-20 Years, New Materials/Facilities)

### TP-SC-17: Optimal Electron-Phonon Coupling lambda ~ phi = 2

**Prediction**: The optimal electron-phonon coupling constant for maximizing Tc in conventional (BCS/Eliashberg) superconductors is lambda_opt ~ phi = 2, balancing strong coupling against lattice instability.

| Item | Value |
|------|-------|
| n=6 formula | lambda_opt = phi = 2 |
| Verification | Systematic study of Tc vs lambda across new high-Tc conventional SCs. Current data: MgB2 (lambda~0.9, Tc=39K), Nb3Sn (lambda~1.8, Tc=18.3K), H3S (lambda~2.2, Tc=203K) |
| Expected result | Tc peaks near lambda ~ 2.0-2.5 before structural collapse |
| Falsification | Maximum Tc occurs at lambda > 4 or lambda < 1 in systematic study |
| Related BT | H-SC-04 |
| Confidence | ★★ (hydride data suggests lambda ~ 2 is indeed near optimal; structural instability limits higher lambda) |

---

### TP-SC-18: Room-Temperature SC Tc in n=6? (Speculative)

**Prediction**: If room-temperature superconductivity (Tc ~ 300K at ambient pressure) is achieved, the Tc may be expressible as an n=6 formula. However, too many n=6 expressions exist near 300K to make this predictive: sigma*(J_2+1)=300, sopfr*n*(sigma-phi)=300, etc.

| Item | Value |
|------|-------|
| n=6 formula | Tc(RT-SC) = sopfr*n*(sigma-phi) = 5*6*10 = 300K? or 12*25? (uncertain) |
| Verification | Wait for room-temperature SC discovery. Check if Tc matches any n=6 expression |
| Expected result | Tc ~ 250-350K, almost certainly n=6-expressible post hoc |
| Falsification | (Weak) RT-SC Tc that is NOT cleanly expressible as n=6 arithmetic (unlikely given density of n=6 expressions) |
| Related BT | -- |
| Confidence | ★ (highly speculative; too many n=6 expressions could fit any number post hoc) |

---

### TP-SC-19: Next-Gen Fusion Magnet Coil Count Follows n=6

**Prediction**: Next-generation fusion devices (DEMO, STEP, CFETR, ARC) will use TF coil counts from the n=6 vocabulary: {12=sigma, 16=2^tau, 18=3n, 24=J_2}.

| Item | Value |
|------|-------|
| n=6 formula | TF coils in {sigma, 2^tau, 3n, J_2} = {12, 16, 18, 24} |
| Verification | Monitor DEMO (EU), STEP (UK), CFETR (China), ARC (CFS) design publications |
| Expected result | ITER=18, SPARC=18. DEMO design currently 16 or 18. Compact stellarators may use 12 |
| Falsification | Next 3 fusion devices all use TF counts outside {12, 16, 18, 24} (e.g., 14, 20, 22) |
| Related BT | BT-99 |
| Confidence | ★★ (18 is common for engineering reasons -- toroidal ripple minimization with manageable port access) |

---

### TP-SC-20: Topological SC Protected Modes = n/phi = 3

**Prediction**: Topological superconductors in symmetry class DIII host exactly n/phi = 3 types of protected boundary modes (Majorana, helical Majorana, chiral Majorana) corresponding to spatial dimensions d = 1, 2, 3.

| Item | Value |
|------|-------|
| n=6 formula | protected mode types = n/phi = 3 |
| Verification | Check Altland-Zirnbauer class DIII in d = 1,2,3. Experimental: observe boundary modes in candidate TSCs (Sr2RuO4, CuxBi2Se3, UTe2) |
| Expected result | Three distinct topological boundary mode types in d <= 3 |
| Falsification | A 4th qualitatively distinct boundary mode is found in class DIII at d <= 3 |
| Related BT | H-SC-28 |
| Confidence | ★★ (topological classification is rigorous; experimental realization is challenging) |

---

### TP-SC-21: Optimal SMES Coil Cross-Section = Hexagonal

**Prediction**: For large-scale Superconducting Magnetic Energy Storage (SMES), a hexagonal conductor cross-section maximizes the packing fraction and minimizes AC loss compared to circular or rectangular cross-sections.

| Item | Value |
|------|-------|
| n=6 formula | optimal cross-section CN = n = 6 (hexagonal close-packing) |
| Verification | Finite-element simulation of SMES coil packing with hexagonal vs circular vs rectangular cross-sections |
| Expected result | Hexagonal achieves fill factor > 0.9 vs ~0.785 for circular, with lower AC loss from reduced void fraction |
| Falsification | Rectangular cross-section achieves higher fill factor AND lower AC loss than hexagonal |
| Related BT | BT-122 (hexagonal universality), H-SC-01 |
| Confidence | ★★ (hexagonal packing advantage is geometric; engineering constraints may favor rectangular in practice) |

---

### TP-SC-22: Compact Stellarator Coil Count = sigma = 12 or n = 6

**Prediction**: Optimized compact stellarators (W7-X successors) will use total modular coil counts that are multiples of n=6 or equal to sigma=12.

| Item | Value |
|------|-------|
| n=6 formula | coils = k * n, or sigma = 12 |
| Verification | W7-X: 50 non-planar + 20 planar = 70. HSX: 48 = sigma*tau coils. Monitor next designs |
| Expected result | Next stellarator designs may converge on simpler coil sets; 12 or 24 modular coils for reduced engineering complexity |
| Falsification | Next 2+ stellarator designs use coil counts not expressible as n=6 |
| Related BT | BT-99 |
| Confidence | ★ (stellarator coil counts are optimization-driven; n=6 match is uncertain) |

---

## Tier 4: Industrial/Long-Term Verification (20+ Years)

### TP-SC-23: HTS Cable Critical Current Target = sigma * J_2 = 288 A per Tape

**Prediction**: Next-generation REBCO HTS tapes will achieve engineering critical current benchmarks near 288 = sigma * J_2 A per standard-width tape as manufacturing matures.

| Item | Value |
|------|-------|
| n=6 formula | I_c target = sigma * J_2 = 288 A |
| Verification | Monitor SuperPower, AMSC, SuNam, Fujikura REBCO tape specifications over 2026-2040 |
| Expected result | Industry convergence toward standard I_c benchmarks |
| Falsification | Industry standard settles at a value clearly unrelated to 288 (e.g., 200 or 500 A as the benchmark) |
| Related BT | H-SC-14 |
| Confidence | ★ (speculative; I_c is materials-engineering driven) |

---

### TP-SC-24: Quantum Computer SC Qubit Count Milestone at 10^n = 10^6

**Prediction**: Full fault-tolerant quantum computing will require approximately 10^n = 10^6 = 1,000,000 physical SC qubits. Intermediate milestones: (sigma-phi)^(n/phi) = 10^3 = 1000 (current era), sigma^3 = 1728 (next era).

| Item | Value |
|------|-------|
| n=6 formula | milestones: (sigma-phi)^3 = 1000, sigma^3 = 1728, 10^n = 10^6 |
| Verification | Track IBM, Google, Amazon SC qubit roadmaps (2026-2045) |
| Expected result | Major announcements cluster near n=6-expressible numbers |
| Falsification | Next 5 major qubit milestones are all non-n=6-expressible (e.g., 1500, 3000, 7000, 15000, 50000) |
| Related BT | H-SC-11 |
| Confidence | ★ (qubit counts follow engineering/funding; many numbers are n=6-expressible) |

---

### TP-SC-25: SC MagLev Pole Pairs in {n/phi, tau, n} = {3, 4, 6}

**Prediction**: Optimized SC MagLev propulsion systems use pole pair counts from n=6 vocabulary: {n/phi=3, tau=4, n=6} for linear synchronous motors.

| Item | Value |
|------|-------|
| n=6 formula | pole pairs in {n/phi, tau, n} = {3, 4, 6} |
| Verification | JR Central L0 Series (Japan): SC coils with specific pole geometry. Chinese 600 km/h MagLev designs |
| Expected result | JR L0 uses SC coils with 4 poles per bogie. Chinese designs TBD |
| Falsification | Next MagLev system uses pole counts outside n=6 vocabulary |
| Related BT | BT-123 (SE(3) robotics universality) |
| Confidence | ★ (limited data; engineering optimization dominates) |

---

### TP-SC-26: FCC-hh Dipole Count -- Null Prediction (Honest)

**Prediction**: The FCC-hh (Future Circular Collider) dipole magnet count will likely NOT match a clean n=6 expression. FCC-hh circumference ~91 km, dipole length ~14-16m, giving ~5700-6500 dipoles. No simple n=6 formula yields this range.

| Item | Value |
|------|-------|
| n=6 formula | **None predicted** (null anchor) |
| Verification | FCC-hh CDR. Count dipoles and check n=6 expressibility |
| Expected result | Dipole count determined by tunnel radius and field strength, unlikely to match n=6 cleanly |
| Falsification | (Inverse) If it DOES cleanly match n=6, that would be surprising and noteworthy |
| Related BT | -- |
| Confidence | ★ (honest null anchor: we predict NO clean match here) |

---

### TP-SC-27: SPARC Toroidal Field ~ sigma(6) = 12 T

**Prediction**: The SPARC compact tokamak achieves peak on-axis toroidal field near sigma = 12 T, enabled by HTS REBCO magnets. Design target: 12.2 T.

| Item | Value |
|------|-------|
| n=6 formula | B_T(SPARC) ~ sigma = 12 T |
| Verification | SPARC magnet test results (MIT/CFS). Published design: ~12.2 T on-axis |
| Expected result | 12.0-12.5 T on-axis field |
| Falsification | SPARC achieves <10 T or >15 T on-axis |
| Related BT | H-SC-25, BT-99 |
| Confidence | ★★ (design spec is 12.2T; whether sigma=12 match is coincidence or engineering optimum is debatable) |

---

## Summary Table

```
+----------+----------------------------------------------+--------+--------+--------+
|  ID      |  Prediction                                  | n=6    | Tier   | Conf.  |
+----------+----------------------------------------------+--------+--------+--------+
| TP-SC-01 | Abrikosov vortex CN = 6                      | n=6    | T1     | ***    |
| TP-SC-02 | YBCO ratio {1,2,3} = div(6)                  | div(6) | T1     | ***    |
| TP-SC-03 | Nb3Sn unit cell Nb = 6                       | n=6    | T1     | ***    |
| TP-SC-04 | Cooper pair = 2 = phi                        | phi    | T1     | ***    |
| TP-SC-05 | 3 macroscopic quantum effects                | n/phi  | T1     | **     |
| TP-SC-06 | 4 hallmark SC phenomena                      | tau    | T1     | **     |
| TP-SC-07 | 2 Josephson relations                        | phi    | T1     | **     |
| TP-SC-08 | MgB2: Mg Z=12=sigma, B Z=5=sopfr            | sigma  | T1     | **     |
+----------+----------------------------------------------+--------+--------+--------+
| TP-SC-09 | ITER TF=18=3n, CS=6=n                        | 3n, n  | T2     | ***    |
| TP-SC-10 | Optimal CuO2 planes = 3 = n/phi             | n/phi  | T2     | ***    |
| TP-SC-11 | WHH coefficient ln(2) = ln(phi)              | ln(phi)| T2     | **     |
| TP-SC-12 | BCS gap ratio 3.528 (NO n=6 match)           | ---    | T2     | *      |
| TP-SC-13 | Hc2/Tc mapping (FALSIFIED)                   | ---    | T2     | *      |
| TP-SC-14 | SC qubit archetypes = 3 = n/phi              | n/phi  | T2     | **     |
| TP-SC-15 | MgB2 gap ratio ~3 = n/phi                    | n/phi  | T2     | ***    |
| TP-SC-16 | Vortex pinning optimal ~ 3*xi               | n/phi  | T2     | **     |
+----------+----------------------------------------------+--------+--------+--------+
| TP-SC-17 | Optimal e-ph coupling lambda ~ 2 = phi       | phi    | T3     | **     |
| TP-SC-18 | Room-temp SC Tc in n=6? (speculative)        | ?      | T3     | *      |
| TP-SC-19 | Next fusion TF coils in {12,16,18,24}        | n=6    | T3     | **     |
| TP-SC-20 | Topological SC modes = 3 = n/phi             | n/phi  | T3     | **     |
| TP-SC-21 | SMES hexagonal cross-section optimal         | n=6    | T3     | **     |
| TP-SC-22 | Stellarator coils = 12 or 6k                 | sigma  | T3     | *      |
+----------+----------------------------------------------+--------+--------+--------+
| TP-SC-23 | HTS tape I_c target 288 A = sigma*J2         | sJ2    | T4     | *      |
| TP-SC-24 | SC qubit milestones at 10^n                  | 10^n   | T4     | *      |
| TP-SC-25 | MagLev pole pairs in {3,4,6}                 | n=6    | T4     | *      |
| TP-SC-26 | FCC-hh dipoles (NULL prediction)             | null   | T4     | *      |
| TP-SC-27 | SPARC B_T ~ 12 T = sigma                     | sigma  | T4     | **     |
+----------+----------------------------------------------+--------+--------+--------+
```

---

## Statistics

| Tier | Count | Timeframe | High Conf (***) | Negative/Null |
|------|-------|-----------|-----------------|---------------|
| **Tier 1** (Today) | 8 | Immediate | 4 | 0 |
| **Tier 2** (Lab) | 8 | 1-3 years | 3 | 2 |
| **Tier 3** (Future) | 6 | 5-20 years | 0 | 1 |
| **Tier 4** (Industry) | 5 | 20+ years | 0 | 1 |
| **Total** | **27** | | **7** | **4** |

---

## Honest Assessment

### Where n=6 works strongly in superconductors:

| Domain | Strength | Examples |
|--------|----------|---------|
| Crystal geometry (CN, lattice) | EXACT | Abrikosov CN=6, Nb3Sn 6 atoms per cell |
| Chemical stoichiometry | EXACT | YBCO {1,2,3}=div(6), MgB2 Z=(12,5) |
| Standard classifications | CLOSE | 3 quantum effects, 4 hallmarks, 2 Josephson eqs, 3 qubit types |
| Specific ratios | CLOSE-EXACT | MgB2 gap ratio ~3.0 = n/phi, CuO2 optimal planes = 3 |
| Magnet engineering | CLOSE | ITER 18 TF coils = 3n, SPARC ~12T = sigma |

### Where n=6 fails in superconductors:

| Domain | Problem | Examples |
|--------|---------|---------|
| BCS material constants | No clean n=6 | Gap ratio 3.528, critical exponents |
| Hc2/Tc ratios | Does not match | ~1.7 T/K for A15s, no n=6 expression |
| Large engineering counts | Optimization-driven | Stellarator coils, accelerator dipoles |
| Future material Tc | Post hoc fitting | Too many n=6 expressions near any number |

### Most impactful predictions:

1. **TP-SC-15** (MgB2 gap ratio ~3) -- already confirmed, strongest quantitative match in this domain
2. **TP-SC-09** (ITER 18+6 coils) -- confirmed, extends to future fusion devices
3. **TP-SC-01** (Abrikosov CN=6) -- textbook result, foundational geometric inevitability

### Falsifiability anchors (honest negatives):

- **TP-SC-12**: BCS gap ratio has no n=6 match (retained for honesty)
- **TP-SC-13**: Hc2/Tc mapping fails (retained as negative evidence)
- **TP-SC-18**: RT-SC Tc prediction is speculative (acknowledged as post hoc risk)
- **TP-SC-26**: FCC-hh null prediction (we explicitly expect NO match)

---

## References

1. Abrikosov, A.A. (1957). JETP 5, 1174. -- Vortex lattice theory
2. Essmann, U. & Trauble, H. (1967). Phys. Lett. A 24, 526. -- Vortex decoration
3. Bardeen, J., Cooper, L.N., Schrieffer, J.R. (1957). Phys. Rev. 108, 1175. -- BCS theory
4. Nagamatsu, J. et al. (2001). Nature 410, 63. -- MgB2 discovery
5. Hazen, R.M. et al. (1987). Phys. Rev. Lett. 58, 1118. -- YBCO structure
6. Deaver, B.S. & Fairbank, W.M. (1961). Phys. Rev. Lett. 7, 43. -- Flux quantization
7. Tinkham, M. (2004). Introduction to Superconductivity, 2nd ed.
8. Devoret, M.H. & Schoelkopf, R.J. (2013). Science 339, 1169. -- SC qubits
9. Weger, M. & Goldberg, I.B. (1973). Solid State Physics 28, 1. -- A15 compounds
10. Clarke, J. & Wilhelm, F.K. (2008). Nature 453, 1031. -- SC qubits review

---

*Derived from H-SC-01~30 hypotheses and BT-99, BT-122, BT-85.*
*27 predictions: 8 Tier 1 + 8 Tier 2 + 6 Tier 3 + 5 Tier 4.*
*Honest: 4 negative/null predictions retained for falsifiability.*
