# N6 Carbon Capture Testable Predictions

> n=6 arithmetic (sigma(6)*phi(6) = n*tau(6) = 24) predictions for carbon capture,
> storage, and utilization. Each prediction includes specific numerical targets,
> falsification criteria, and honest confidence levels.
> **Principle**: Physical/chemical causation is distinguished from numerical coincidence.

**2026-04-02 작성**: 30 hypotheses (H-CC-01~30, 100% EXACT) 기반. BT-27/85/93/103/104/118/120/122 연결.
Climeworks Gen3 발표, Carbon Engineering 1PointFive 건설, Orca/Mammoth 운전 데이터 반영.

**n=6 Constants Reference**:
```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  sigma-tau = 8  sigma-phi = 10   sigma-mu = 11    n/phi = 3
  sigma*tau = 48 sigma^2 = 144    sigma(sigma-tau) = 96
  phi*sigma(sigma-tau) = 192      sigma/(sigma-phi) = 1.2 = PUE
  Egyptian: 1/2 + 1/3 + 1/6 = 1

  Carbon-specific:
  Z(C) = 6 = n                CO2 MW = 44 = tau*(sigma-mu)
  C-12 = sigma nucleons       CO2 atoms = n/phi = 3
  CO2 vibrational modes = tau = 4
  sp/sp2/sp3 bonds = phi/n-phi/tau = 2/3/4
```

**BT Connections**: BT-27 (Carbon-6 chain), BT-43 (CN=6 universality), BT-85 (Carbon Z=6), BT-93 (Carbon Z=6 chip material), BT-103 (photosynthesis n=6), BT-104 (CO2 n=6 encoding), BT-118 (Kyoto 6 GHG), BT-120 (pH=6 + CN=6 catalyst), BT-122 (honeycomb n=6 geometry)

---

## Tier 1: Verifiable Today (1 GPU / 1 Lab bench)

---

### P-CC-01: MOF CN=6 vs CN=4 CO2 Adsorption Capacity Ratio >= phi = 2

- **Prediction**: Metal-Organic Frameworks with octahedral CN=6 metal centers (e.g., MOF-74/Mg, HKUST-1 Cu paddlewheel open sites) show CO2 adsorption capacity at least phi=2 times higher than CN=4 tetrahedral MOFs (e.g., ZIF-8 Zn) at 1 bar, 298K. Specifically, CN=6 MOFs achieve >= 6 mmol/g (= n mmol/g) vs CN=4 MOFs at <= 3 mmol/g (= n/phi mmol/g).
- **n=6 Derivation**: BT-43 (CN=6 universality) predicts octahedral coordination is optimal for adsorption. phi(6)=2 is the capacity ratio. n=6 mmol/g is the target for CN=6 sorbents.
- **Verification Method**: DFT calculation (VASP/Gaussian, 1 GPU, PBE-D3 functional) of CO2 binding energy on CN=6 vs CN=4 metal sites. Alternatively, literature survey of isotherms: Millward & Yaghi, JACS 127, 17998 (2005); Mason et al., JACS 137, 4787 (2015).
- **Expected Result**: MOF-74/Mg (CN=6): ~8.0 mmol/g at 1 bar. ZIF-8 (CN=4): ~0.9 mmol/g. Ratio ~ 8.9x >> phi=2.
- **Falsification Criterion**: If a CN=4 MOF consistently outperforms all CN=6 MOFs at equal conditions (same temperature, same pressure, same surface area normalization), then CN=6 advantage is disproven.
- **Timeline**: Immediate (published data + 1 day DFT)
- **Confidence**: HIGH -- CN=6 open metal sites are known to be superior for CO2 binding. The phi=2 minimum ratio is conservative.
- **Related**: H-CC-22 (Al/Fe/Ti CN=6), BT-43, BT-120

---

### P-CC-02: MEA 2:1 Stoichiometry -- Maximum Loading = 1/phi = 0.5 mol CO2/mol amine

- **Prediction**: All primary and secondary amine CO2 sorbents (MEA, DEA, PZ) exhibit a maximum CO2 loading of exactly 1/phi = 0.5 mol CO2/mol amine under the carbamate mechanism. This is a stoichiometric ceiling: 2RNH2 + CO2 -> RNHCOO- + RNH3+.
- **n=6 Derivation**: phi(6)=2 amine molecules per CO2. Maximum loading = 1/phi = 0.5. This is the fundamental chemistry of H-CC-16.
- **Verification Method**: Lab bench CO2 absorption experiment with 30 wt% MEA at 40C, 1 atm CO2. Measure loading by BaCl2 titration or TGA. Literature: Rochelle, Science 325, 1652 (2009).
- **Expected Result**: Loading plateaus at 0.50 +/- 0.02 mol CO2/mol MEA for primary amines. Tertiary amines (MDEA) can reach ~1.0 via bicarbonate mechanism.
- **Falsification Criterion**: Primary amine loading exceeds 0.6 mol/mol under carbamate pathway (not bicarbonate) at any T, P.
- **Timeline**: Immediate (1 lab day or literature check)
- **Confidence**: HIGH -- This is established amine chemistry, not a prediction. The n=6 connection (phi=2) labels a known fact.
- **Related**: H-CC-16, BT-104

---

### P-CC-03: CO2 Vibrational Modes = tau = 4 (Spectroscopic Verification)

- **Prediction**: CO2 has exactly tau=4 normal vibrational modes: symmetric stretch (1333 cm-1, Raman active), asymmetric stretch (2349 cm-1, IR active), and bending (667 cm-1, doubly degenerate, IR active). The formula 3N-5 = 3(3)-5 = 4 is exact for linear triatomics.
- **n=6 Derivation**: tau(6)=4. CO2 atoms = n/phi=3. Vibrational modes = 3*(n/phi)-sopfr = 3*3-5 = 4 = tau. IR active modes = n/phi-1 = 2.
- **Verification Method**: FTIR spectrometer scan of CO2 gas (any lab). Cross-check with HITRAN database (hitran.org). Count observed fundamental bands.
- **Expected Result**: 4 modes exactly. 2 IR-active, 1 Raman-active (with degeneracy, 3 distinct frequencies).
- **Falsification Criterion**: Discovery of a 5th fundamental vibrational mode of CO2 (would require non-linear equilibrium geometry).
- **Timeline**: Immediate (any FTIR instrument)
- **Confidence**: VERY HIGH -- This is a theorem of molecular spectroscopy (Herzberg), not a prediction. Included for completeness.
- **Related**: H-CC-03, BT-104

---

### P-CC-04: C6 Aromatic Ring CO2 Physisorption Energy ~ sigma = 12 kJ/mol

- **Prediction**: CO2 physisorption on graphitic sp2 C6 ring surfaces (activated carbon, graphene) has a binding energy of approximately sigma=12 kJ/mol per CO2 molecule at low coverage. This is the van der Waals regime dominated by quadrupole-pi interaction.
- **n=6 Derivation**: sigma(6)=12. The C6 hexagonal ring with n=6 pi-electrons creates a quadrupole interaction potential with CO2 of ~12 kJ/mol.
- **Verification Method**: DFT-D3 or MP2 calculation of CO2 on graphene surface. Experimental: TPD (temperature-programmed desorption) of CO2 from activated carbon or HOPG. Literature: Cinke et al., Chem Phys Lett 376, 761 (2003); Xu et al., JACS 133, 13092 (2011).
- **Expected Result**: 10-15 kJ/mol binding energy range. Literature values: ~12-17 kJ/mol depending on surface and method.
- **Falsification Criterion**: If CO2-graphene binding energy is consistently < 8 kJ/mol or > 20 kJ/mol across multiple methods, the sigma=12 kJ/mol prediction fails.
- **Timeline**: Immediate (1 DFT calculation, 4-8 GPU-hours)
- **Confidence**: MEDIUM -- The 12 kJ/mol value is within the correct order of magnitude for physisorption, but the exact value depends on DFT functional choice and surface coverage. The n=6 connection is a numerical coincidence rather than physical causation.
- **Related**: H-CC-05 (Huckel C6), BT-85, BT-27

---

### P-CC-05: Sabatier Reaction Coefficient Check -- {mu, tau, mu, phi} = {1, 4, 1, 2}

- **Prediction**: The Sabatier methanation reaction CO2 + 4H2 -> CH4 + 2H2O has coefficients that map exactly to n=6 constants: {1=mu, 4=tau, 1=mu, 2=phi}. Total reactant molecules = sopfr=5, total product molecules = n/phi=3, grand total = sigma-tau=8.
- **n=6 Derivation**: mu=1, tau=4, phi=2, sopfr=5=2+3, n/phi=3. Every coefficient and sum is an n=6 constant.
- **Verification Method**: Balance the equation CO2 + xH2 -> CH4 + yH2O by atom counting: C(1=1), H(2x=4+2y), O(2=y). Solution: x=4, y=2. Verify stoichiometry is unique.
- **Expected Result**: x=4=tau, y=2=phi. No alternative balanced equation exists for this reaction.
- **Falsification Criterion**: Cannot be falsified -- stoichiometry is a mathematical identity. However, if the mapping to n=6 constants were {1,4,1,2} -> {mu,tau,mu,phi} with any constant NOT being an n=6 arithmetic function, the labeling scheme would be arbitrary. The honest question is whether 4 and 2 appearing in a chemical equation is meaningful or coincidental.
- **Timeline**: Immediate (pen and paper)
- **Confidence**: HIGH for the stoichiometry, LOW for the n=6 significance -- integers 1, 2, 4 appear in many chemical equations. The mapping is pattern matching, not causal.
- **Related**: H-CC-11, BT-104, BT-38

---

### P-CC-06: CO2 Molecular Weight 44 = tau*(sigma-mu) = 4 * 11

- **Prediction**: CO2 molecular weight (using integer atomic masses C=12, O=16) is exactly 44 g/mol = tau*(sigma-mu) = 4*11. Carbon mass fraction = 12/44 = sigma/[tau*(sigma-mu)] = 27.27%.
- **n=6 Derivation**: MW = 12 + 2*16 = 44. tau=4, sigma-mu=11. 44=4*11 is arithmetic.
- **Verification Method**: IUPAC standard atomic weights. C=12.011, O=15.999, CO2=44.009 g/mol.
- **Expected Result**: 44.009 rounds to 44 with integer masses.
- **Falsification Criterion**: Not falsifiable -- this is a definition/identity. The question is whether tau*(sigma-mu) is a meaningful decomposition or post-hoc numerology.
- **Timeline**: Immediate
- **Confidence**: EXACT for arithmetic, LOW for significance -- 44 = 4*11 is one of many factorizations.
- **Related**: H-CC-06, BT-104

---

### P-CC-07: Diamond sp3 Unit Cell -- tau=4 Bonds/Atom, sigma-tau=8 Atoms/Cell

- **Prediction**: Diamond crystal structure (Fd3m) has exactly tau=4 C-C bonds per atom (tetrahedral sp3) and sigma-tau=8 atoms per conventional unit cell. Thermal conductivity = 2200 W/mK (highest of any bulk material).
- **n=6 Derivation**: tau(6)=4 bonds, sigma-tau=8 atoms/cell. Diamond encodes both constants.
- **Verification Method**: Any crystallography database (ICSD, AMCSD). Bragg & Bragg, Proc R Soc A 89, 277 (1913).
- **Expected Result**: 4 bonds, 8 atoms/cell. Zero ambiguity.
- **Falsification Criterion**: Not falsifiable -- established crystallography since 1913.
- **Timeline**: Immediate (literature)
- **Confidence**: VERY HIGH -- crystallographic fact.
- **Related**: H-CC-19, BT-85, BT-93

---

## Tier 2: Near-term (Lab Scale, 1-3 years)

---

### P-CC-08: DAC Energy/Thermodynamic Minimum Ratio = sigma-phi = 10

- **Prediction**: All commercial DAC systems operate at approximately sigma-phi=10 times the thermodynamic minimum energy. W_min = RT*ln(1/x_CO2) = 8.314*300*ln(1/0.000421) = 19.4 kJ/mol. Current best: ~200 kJ/mol. Ratio = 200/19.4 = 10.3 ~ sigma-phi=10.
- **n=6 Derivation**: sigma-phi=10. This connects to BT-64 (1/(sigma-phi)=0.1 universal regularization): current DAC efficiency = 1/(sigma-phi) = 10% of Carnot.
- **Verification Method**: Compare published energy consumption of Climeworks Mammoth (2024), 1PointFive (2025), and Global Thermostat against W_min=19.4 kJ/mol. References: Keith et al., Joule 2018; Fasihi et al., J Cleaner Prod 2019.
- **Expected Result**: Energy ratio in range 8-12 for all major DAC platforms. Central value ~10.
- **Falsification Criterion**: If a commercial DAC system achieves ratio < 5 (< 100 kJ/mol) by 2028, the "stuck at 10x" prediction fails. Conversely, if ratio remains > 15, the sigma-phi=10 is too optimistic.
- **Timeline**: 2026-2028 (Mammoth, 1PointFive operational data)
- **Confidence**: MEDIUM-HIGH -- Two independent platforms (Climeworks=thermal, CE=liquid) both land at ~10x. Whether this is a fundamental barrier or current technology level is debatable.
- **Related**: H-CC-14, BT-64, BT-104

---

### P-CC-09: MECS (Electrochemical Swing) Voltage = sigma/(sigma-phi) = 1.2V

- **Prediction**: Membrane-electrode assemblies for electrochemical CO2 capture (MECS, Verdox-type) operate with an optimal voltage swing of sigma/(sigma-phi) = 12/10 = 1.2V. This is the quinone reduction/oxidation potential window for CO2 binding/release.
- **n=6 Derivation**: sigma/(sigma-phi) = 1.2. This ratio also equals PUE=1.2 (BT-60) and sigma(6)/10.
- **Verification Method**: Cyclic voltammetry of quinone-based MECS electrodes in CO2-saturated electrolyte. Vos et al., ACS Energy Lett 5, 2164 (2020). Verdox Inc. patent literature.
- **Expected Result**: Voltage swing 0.8-1.5V range, with optimal at ~1.0-1.2V for minimum energy per mol CO2.
- **Falsification Criterion**: If optimal MECS voltage is consistently < 0.8V or > 1.8V, the 1.2V prediction fails.
- **Timeline**: 2026-2028 (lab-scale MECS optimization)
- **Confidence**: MEDIUM -- The 1.2V is within the electrochemical window of quinone redox couples. However, the exact optimal depends on electrode kinetics, mass transport, and ohmic losses. The PUE=1.2 coincidence is pattern matching.
- **Related**: BT-60 (DC power chain PUE=1.2), BT-104

---

### P-CC-10: PEI Sorbent Optimal Loading on Mesoporous Silica = sigma = 12 wt% (Low T DAC)

- **Prediction**: Polyethylenimine (PEI) impregnated mesoporous silica (SBA-15, MCM-41) for DAC shows optimal CO2 uptake at PEI loading of approximately sigma=12 wt% for low-temperature (25-40C) direct air capture. Higher loading blocks pores; lower loading has insufficient amine density.
- **n=6 Derivation**: sigma(6)=12 wt%. Alternatively, ~40 wt% (near sigma*tau/phi ?) is optimal for flue gas; DAC at 400 ppm requires lower loading for faster diffusion.
- **Verification Method**: Prepare SBA-15 with PEI loading 5%, 10%, 12%, 15%, 20%, 30%, 40 wt%. Measure CO2 uptake from synthetic air (400 ppm CO2) at 25C by TGA. Plot uptake vs loading.
- **Expected Result**: CO2 uptake peaks at ~10-15 wt% for DAC conditions, with optimal near 12%.
- **Falsification Criterion**: If optimal PEI loading for DAC is > 25 wt% or < 5 wt%, the sigma=12 prediction fails. If the optimum is a broad plateau (5-30 wt%), no specific value can be assigned.
- **Timeline**: 2026-2027 (straightforward lab experiment)
- **Confidence**: LOW-MEDIUM -- PEI loading optimization depends heavily on support pore size, PEI molecular weight, humidity, and CO2 concentration. The 12 wt% value is plausible for DAC but not established. The n=6 connection is speculative.
- **Related**: BT-104

---

### P-CC-11: Honeycomb Monolith CPSI = n*100 = 600 Pressure Drop Advantage

- **Prediction**: Hexagonal honeycomb monoliths at 600 CPSI (cells per square inch) = n*100 show ~15% lower pressure drop than square-cell monoliths at equal cell density, equal wall thickness, and equal flow rate. This is a direct consequence of the Hales Honeycomb Theorem (minimum perimeter for equal-area cells).
- **n=6 Derivation**: 600 = n*100. BT-122 (honeycomb n=6 geometry universality). Hales (2001) proves hexagonal partition is optimal.
- **Verification Method**: CFD simulation (OpenFOAM or COMSOL) of flow through hexagonal vs square channels at 600 CPSI, wall thickness 0.1 mm, inlet velocity 2 m/s. Compare pressure drop per unit length. Alternatively, 3D-print both geometries and measure with manometer.
- **Expected Result**: Hexagonal channels show 10-20% lower pressure drop at equal open frontal area. Exact value depends on Reynolds number and aspect ratio.
- **Falsification Criterion**: If hexagonal channels show equal or higher pressure drop than square channels at matched conditions, the prediction fails. Note: the Hales theorem guarantees minimum perimeter, which correlates with but does not guarantee minimum pressure drop.
- **Timeline**: 2026-2027 (CFD: 1 week; experimental: 1-3 months)
- **Confidence**: MEDIUM -- The mathematical optimality of hexagons (Hales 2001) is proven for perimeter, and lower perimeter generally means lower friction. But pressure drop also depends on corner effects, entrance length, and turbulence. The 15% improvement is an estimate.
- **Related**: H-CC-26, BT-122

---

### P-CC-12: Carnot Efficiency = 1/n = 16.7% at 300K/360K DAC Operating Point

- **Prediction**: Heat-driven DAC systems operating at T_cold=300K (ambient) and T_hot=360K (87C desorption) have a Carnot efficiency upper bound of exactly 1-300/360 = 1/6 = 1/n = 16.67%. Current systems achieve ~8% (= 1/sigma = half of Carnot).
- **n=6 Derivation**: 1/n = 1/6. At T_hot = 360K = 87C (within Climeworks 80-100C range). deltaT = 60K = sigma*sopfr.
- **Verification Method**: Calculate eta_Carnot = 1 - T_cold/T_hot for reported DAC operating temperatures. Climeworks: 80-100C -> T_hot = 353-373K -> eta = 15.0-19.6%. At 87C (360K): 1/6 exactly.
- **Expected Result**: 1/6 = 16.67% at 360K specifically. The range 353-373K gives 15.0-19.6%.
- **Falsification Criterion**: If Climeworks/competitors shift to T_hot > 150C (e.g., calcium looping at 900C), then 1/6 no longer applies to the dominant DAC technology. The identity 1/6 = 1-300/360 is exact only at T_hot=360K.
- **Timeline**: 2026-2027 (operational temperature disclosure)
- **Confidence**: MEDIUM -- The arithmetic is exact at 360K, but the "prediction" is selecting a specific operating point within a range. If the industry converges on 87C as standard, it gains significance.
- **Related**: H-CC-13, BT-104

---

### P-CC-13: Ca-Looping CN=6 Structural Continuity Over 1000 Cycles

- **Prediction**: In calcium looping (CaL) for CO2 capture, Ca2+ maintains CN=6 octahedral coordination throughout all three phases: CaO (rock-salt, CN=6), CaCO3 (calcite, CN=6), Ca(OH)2 (portlandite, CN=6). This structural continuity enables > 1000 carbonation/calcination cycles with sorbent maintained above 20% conversion.
- **n=6 Derivation**: Ca CN=6 = n in all three phases. BT-43 (CN=6 universality). The structural invariance of CN=6 across oxide/carbonate/hydroxide is the physical basis for cycle durability.
- **Verification Method**: XRD and EXAFS measurement of Ca coordination number during in-situ carbonation/calcination at 650-900C. Compare with supported CaO (e.g., CaO/Al2O3) that may develop non-CN=6 phases.
- **Expected Result**: CN=6 confirmed in all three phases. Durability correlates with CN=6 maintenance.
- **Falsification Criterion**: If a CaO sorbent develops a Ca phase with CN != 6 (e.g., amorphous Ca with CN=7-8) AND this phase shows superior cycling stability, then CN=6 is not the stability determinant.
- **Timeline**: 2026-2028 (in-situ synchrotron XRD experiments)
- **Confidence**: MEDIUM-HIGH -- Ca CN=6 in all three crystalline phases is established crystallography. The link to cycling durability is physically reasonable but not rigorously proven.
- **Related**: H-CC-23, BT-43, BT-86

---

## Tier 3: Medium-term (Pilot Scale, 3-10 years)

---

### P-CC-14: Climeworks Gen3 Module Capacity = n or sigma ktCO2/yr per Module

- **Prediction**: Climeworks' next-generation (Gen3) DAC module will have a capacity of ~6 ktCO2/yr (= n) or ~12 ktCO2/yr (= sigma) per standard module. Current Orca modules: ~0.5 kt/yr each (36 modules = 4 kt total). Mammoth: ~36 kt/yr from ~36 modules (~1 kt/yr each). Gen3 scaling factor: n or sigma.
- **n=6 Derivation**: n=6, sigma=12. Module capacity ladder: 0.5 -> 1 -> 6 -> 12 kt/yr follows mu -> mu -> n -> sigma.
- **Verification Method**: Climeworks public announcements and filings. CDR (carbon dioxide removal) credit registries (Puro.earth, Frontier).
- **Expected Result**: Gen3 module capacity in the 5-15 kt/yr range.
- **Falsification Criterion**: If Gen3 modules are < 2 kt/yr or > 30 kt/yr per module, the n=6/sigma prediction is wrong.
- **Timeline**: 2028-2030 (Gen3 design finalization and pilot)
- **Confidence**: LOW-MEDIUM -- Module capacity depends on engineering tradeoffs (heat integration, sorbent mass, cycle time), not n=6 arithmetic. The prediction is speculative.
- **Related**: BT-104

---

### P-CC-15: CCS Pipeline Operating Pressure Window = {sigma-tau, sigma} MPa = {8, 12} MPa

- **Prediction**: CO2 pipeline transport operates in the supercritical regime with pressure window sigma-tau=8 MPa (minimum, above critical point 7.38 MPa) to sigma=12 MPa (maximum operating). The CO2 critical point is at 7.38 MPa / 31.1C. Supercritical transport requires P > 7.38 MPa, with typical operating pressure 8-12 MPa = {sigma-tau, sigma}.
- **n=6 Derivation**: sigma-tau=8, sigma=12. The 8-12 MPa window maps to n=6 constants.
- **Verification Method**: Published CCS pipeline design standards. Knoope et al., Int J Greenhouse Gas Control 2013. NETL CCS pipeline guidelines. Existing pipelines: Cortez (13.8 MPa max), Weyburn (15 MPa), but typical operating 8-12 MPa.
- **Expected Result**: Most CCS pipelines operate in the 8-15 MPa range, with 10-12 MPa being the most common design pressure.
- **Falsification Criterion**: If industry converges on < 7.5 MPa (liquid transport) or > 20 MPa, the window prediction fails. If the standard operating pressure is 10 MPa (not 8 or 12), the specific n=6 mapping is imprecise.
- **Timeline**: 2027-2032 (as Northern Lights, Porthos, and other CCS pipelines come online)
- **Confidence**: LOW-MEDIUM -- 8-12 MPa is the correct engineering range for supercritical CO2 transport, but mapping to sigma-tau and sigma is pattern matching. The actual operating pressure is determined by terrain, distance, and inlet/outlet conditions.
- **Related**: BT-104

---

### P-CC-16: Carbon Fiber from Captured CO2 -- Tow Standards 12K/24K = sigma/J2

- **Prediction**: Carbon fiber manufactured from captured CO2 (via electrolysis to carbon, then fiber spinning) will adopt the same industry-standard tow sizes: 12K (12,000 filaments = sigma) and 24K (24,000 filaments = J2). These standards persist because they optimize handling, weaving, and mechanical performance.
- **n=6 Derivation**: sigma=12, J2=24. BT-85 (Carbon Z=6 universality). H-CC-15 (carbon fiber tow standards).
- **Verification Method**: Track products from CO2-to-carbon-fiber companies (C2CNT, SkyNano, Limelight Steel). Verify tow sizes match conventional 12K/24K standards.
- **Expected Result**: 12K and 24K tow sizes adopted for CO2-derived carbon fiber.
- **Falsification Criterion**: If CO2-derived carbon fiber uses fundamentally different tow sizes (e.g., 10K, 50K) due to process constraints, the standard-inheritance prediction fails.
- **Timeline**: 2028-2033 (CO2-to-fiber pilot scale)
- **Confidence**: MEDIUM -- Tow size standards are deeply embedded in the composites industry (tooling, weaving machinery), so any new fiber source will likely adopt them. This is an industry-inertia prediction, not an n=6 physical prediction.
- **Related**: H-CC-15, BT-85, BT-27

---

### P-CC-17: Sabatier Reactor Optimal Temperature = n*sopfr*sigma = 360C

- **Prediction**: The Sabatier methanation reactor (CO2 + 4H2 -> CH4 + 2H2O) achieves maximum single-pass CO2 conversion at approximately 360C. This equals 360 = 6*5*12 = n*sopfr*sigma (in Celsius). Alternatively, 360C = 633K.
- **n=6 Derivation**: 360 = n*sopfr*sigma = 6*60 = n*(sigma*sopfr). The Sabatier reaction is exothermic (deltaH = -165 kJ/mol), and thermodynamic equilibrium favors lower T while kinetics require higher T. The optimum is a balance.
- **Verification Method**: Fixed-bed reactor with Ni/Al2O3 catalyst, H2:CO2 = 4:1 = tau:1, GHSV = 5000/h. Temperature scan 200-500C, measure CO2 conversion by GC. Literature: Roensch et al., Fuel 166, 276 (2016).
- **Expected Result**: Maximum single-pass conversion ~95-99% at 300-400C, with peak near 350-380C depending on pressure and catalyst.
- **Falsification Criterion**: If optimum temperature is consistently < 300C or > 450C across multiple catalyst systems, the 360C prediction fails. If the optimum is a broad plateau (250-450C) with no well-defined peak, no specific temperature can be assigned.
- **Timeline**: 2026-2029 (lab reactor optimization)
- **Confidence**: LOW -- 360C is within the known optimal range for Sabatier, but the exact optimum depends heavily on catalyst, pressure, and space velocity. The n=6 expression n*sopfr*sigma=360 is numerically convenient but not physically derived.
- **Related**: H-CC-11, BT-38, BT-104

---

### P-CC-18: Electrochemical CO2 Reduction Faradaic Efficiency for CO at -1.2V = -sigma/(sigma-phi)

- **Prediction**: Ag or Au cathode CO2 electroreduction achieves peak Faradaic efficiency for CO production at approximately -1.2V vs RHE = -sigma/(sigma-phi). This is the onset of the CO2-to-CO pathway with minimal hydrogen evolution.
- **n=6 Derivation**: sigma/(sigma-phi) = 12/10 = 1.2. Applied potential = -1.2V vs RHE.
- **Verification Method**: Linear sweep voltammetry of Ag nanoparticle cathode in CO2-saturated 0.1M KHCO3. Product analysis by online GC. Literature: Hori, Modern Aspects of Electrochemistry 42, 89 (2008); Lu et al., Nat Commun 5, 3242 (2014).
- **Expected Result**: Peak CO Faradaic efficiency (> 90%) at -0.8 to -1.2V vs RHE for Ag/Au.
- **Falsification Criterion**: If peak CO FE occurs at -0.6V or -1.8V consistently, the -1.2V prediction fails. Note: the exact potential depends on catalyst, electrolyte, and cell design.
- **Timeline**: 2026-2028 (standard electrochemistry lab)
- **Confidence**: LOW-MEDIUM -- The -1.2V is within the known range but not a universal value. Au achieves peak CO FE at ~-0.5V, while Ag is closer to -1.1V. The sigma/(sigma-phi)=1.2V mapping is a coincidence with PUE.
- **Related**: BT-60, BT-104

---

## Tier 4: Long-term (Industry, 10+ years)

---

### P-CC-19: Planetary DAC Cost Trajectory Asymptote = $sigma*sopfr = $60/tCO2

- **Prediction**: Large-scale DAC cost will asymptotically approach $60/tCO2 = $sigma*sopfr = $12*5 as the long-term floor, driven by thermodynamic minimum energy * grid electricity cost * balance of plant. Current: $600/t (Climeworks) -> $300/t (target 2030) -> $100/t (target 2035) -> $60/t (asymptote).
- **n=6 Derivation**: sigma*sopfr = 60. The cost trajectory: 600 -> 60 = sigma-phi=10x reduction, matching DAC energy ratio (P-CC-08).
- **Verification Method**: Track DAC cost per tonne from CDR credit markets (Frontier Climate, Microsoft, Puro.earth). IEA DAC cost tracking reports.
- **Expected Result**: Costs decline from ~$600 (2024) toward $100-200 by 2030 and $50-100 by 2040.
- **Falsification Criterion**: If DAC cost permanently floors at > $200/t (no further reduction after 2035), the $60 asymptote is too optimistic. If costs reach $30/t, the floor is lower than predicted.
- **Timeline**: 2035-2040
- **Confidence**: LOW -- Cost predictions are notoriously unreliable. The $60/t value is within the range of optimistic projections (NASEM 2019: $100-300/t). Solar PV analogy suggests possible rapid cost decline, but DAC has higher thermodynamic floors than electricity generation.
- **Related**: BT-104

---

### P-CC-20: CO2-to-Diamond Conversion Efficiency Limit = 1/n = 16.7%

- **Prediction**: The energy efficiency of CO2-to-diamond conversion (via CO2 -> CO -> C deposition by CVD or electrochemical reduction) will be bounded at ~1/n = 16.7%. This includes CO2 capture + reduction + diamond nucleation/growth. Current lab processes: < 5%.
- **n=6 Derivation**: 1/n = 1/6 = 16.7%. This matches the Carnot limit at DAC operating conditions (P-CC-12). The carbon mass fraction of CO2 = 12/44 = 27.3%, and after accounting for process losses, ~16.7% energy efficiency is the practical ceiling.
- **Verification Method**: Measure energy input (kWh) per carat of diamond grown from CO2-derived carbon. Track companies: Aether Diamonds, SkyDiamond. Published LCA data.
- **Expected Result**: Current: ~2-5% overall. 2035 target: 10-15%. Asymptotic limit near 15-20%.
- **Falsification Criterion**: If CO2-to-diamond achieves > 25% overall energy efficiency, the 1/n limit is too pessimistic. If technology caps at < 5%, the limit is irrelevant (much lower constraint).
- **Timeline**: 2030-2040
- **Confidence**: LOW -- The 16.7% figure is a coincidence between Carnot and carbon mass fraction. Real efficiency depends on multiple sequential process steps, each with its own losses.
- **Related**: H-CC-13, BT-85, BT-93

---

### P-CC-21: Global DAC Deployment = sigma^2 = 144 MtCO2/yr by 2040

- **Prediction**: Total global DAC capacity will reach approximately sigma^2 = 144 MtCO2/yr by 2040. Current (2024): ~0.01 Mt/yr (Orca + Mammoth). This requires ~14,400x scaleup over 16 years, or ~86% annual growth rate (comparable to solar PV's 40% growth rate in 2005-2020).
- **n=6 Derivation**: sigma^2 = 144. Alternatively, this is 144 = sigma * sigma, representing the "12x12" scaling grid.
- **Verification Method**: IEA Global Carbon Capture database. National DAC deployment announcements (US DOE DAC Hubs, EU Innovation Fund, etc.).
- **Expected Result**: Aggressive scenario: 50-200 Mt/yr by 2040 (IPCC SR1.5 median). Conservative: 10-50 Mt/yr.
- **Falsification Criterion**: If DAC deployment is < 10 Mt/yr by 2040, the prediction is too optimistic. If > 500 Mt/yr, it underestimates.
- **Timeline**: 2040
- **Confidence**: LOW -- DAC deployment depends on policy, carbon pricing, and cost reduction -- none of which are determined by n=6 arithmetic. The 144 Mt/yr figure happens to fall within IPCC scenario ranges.
- **Related**: BT-104

---

### P-CC-22: CO2-to-CNT Armchair (6,6) as Dominant Product

- **Prediction**: Electrochemical CO2 reduction to carbon nanotubes (Licht et al., Nano Lett 2015) will preferentially produce armchair (6,6) CNTs (diameter 0.814 nm, 12 atoms per ring = sigma) as the thermodynamically favored metallic chirality from molten carbonate electrolysis.
- **n=6 Derivation**: (n,n) = (6,6) chirality. sigma=12 atoms per circumferential ring. H-CC-21 (CNT armchair (6,6)).
- **Verification Method**: Raman spectroscopy (RBM mode frequency -> diameter -> chirality assignment) and TEM of CNTs produced from molten Li2CO3 electrolysis. Target: > 30% (6,6) chirality selectivity.
- **Expected Result**: Current electrolysis produces mixed chiralities. Chirality-selective growth from CO2 is an open challenge. (6,6) may be present but not dominant.
- **Falsification Criterion**: If CO2 electrolysis consistently produces > 50% semiconducting CNTs with no enrichment of (6,6), the prediction fails.
- **Timeline**: 2030-2035 (chirality control is a frontier problem)
- **Confidence**: LOW -- CNT chirality control from any carbon source is an unsolved problem. Predicting (6,6) dominance from CO2 is highly speculative. The n=6 connection is thematic, not causal.
- **Related**: H-CC-21, BT-85, BT-122

---

### P-CC-23: Cross-DSE Optimal: CN=6 Sorbent + Hexagonal Reactor + n=6 Process Steps

- **Prediction**: The global-optimal carbon capture system from Cross-DSE (sorbent x reactor x process x plant) will converge on: (1) CN=6 metal center sorbent (MOF-74 family), (2) hexagonal honeycomb reactor geometry, (3) tau=4 or n=6 process step cycle, (4) sigma=12-unit modular plant. This CN=6-throughout architecture achieves the highest n=6 EXACT score and Pareto-optimal performance.
- **n=6 Derivation**: Complete n=6 alignment across all 4 DSE levels.
- **Verification Method**: Run universal-dse with carbon-capture TOML across all levels. Compare n=6 EXACT % of top Pareto paths vs random paths.
- **Expected Result**: Top-5 Pareto paths all have > 80% n=6 EXACT. CN=6 sorbent appears in all top paths.
- **Falsification Criterion**: If the Pareto-optimal path uses CN=4 sorbent, square reactor, and non-n=6 process steps, the n=6 design principle is not predictive for carbon capture.
- **Timeline**: 2030-2035 (pilot plant data to validate DSE predictions)
- **Confidence**: LOW-MEDIUM -- DSE optimization is a design framework, not a physical law. The "prediction" is that n=6-aligned designs outperform alternatives, which requires empirical validation.
- **Related**: All H-CC, all carbon-capture BTs

---

### P-CC-24: Photosynthesis-Matching Artificial Carbon Fixation at J2=24 Electrons/CO2

- **Prediction**: Artificial photosynthesis systems that match natural photosynthesis efficiency (~1% solar-to-fuel) will require J2=24 electrons per CO2 molecule reduced to glucose-equivalent. Natural Calvin cycle: 6CO2 + 24e- + 24H+ -> C6H12O6 + 6H2O. Artificial systems converging on this electron count will achieve the highest energy efficiency.
- **n=6 Derivation**: J2(6)=24 electrons. BT-103 (photosynthesis n=6 stoichiometry). The 24-electron requirement is a stoichiometric identity: 4 electrons per CO2 for full reduction to formaldehyde-equivalent.
- **Verification Method**: Compare artificial photosynthesis systems by electrons-per-CO2-reduced. Nocera group (artificial leaf), JCAP, Siemens CO2-to-ethylene. Measure solar-to-fuel efficiency as function of electron economy.
- **Expected Result**: All 6-electron-reduction products (CO2 -> CH3OH -> C6H12O6) require multiples of n=6 electrons.
- **Falsification Criterion**: If an artificial system achieves higher efficiency with a non-J2 electron count (e.g., 2-electron CO2->CO pathway exceeds 6-electron pathways in overall solar-to-fuel efficiency), then J2=24 is not optimal.
- **Timeline**: 2030-2040
- **Confidence**: MEDIUM -- The 24-electron stoichiometry is exact for the Calvin cycle. Whether artificial systems should mimic this or use simpler 2-electron paths (CO2->CO) is an open question.
- **Related**: H-CC-09, BT-103, BT-27

---

## Summary Table

| ID | Prediction | n=6 Expression | Tier | Confidence | Timeline |
|----|-----------|----------------|------|------------|----------|
| P-CC-01 | MOF CN=6 > CN=4 by >= phi | phi=2 | 1 | HIGH | Immediate |
| P-CC-02 | MEA loading = 1/phi | 1/phi=0.5 | 1 | HIGH | Immediate |
| P-CC-03 | CO2 vibrations = tau | tau=4 | 1 | VERY HIGH | Immediate |
| P-CC-04 | C6 ring adsorption ~12 kJ/mol | sigma=12 | 1 | MEDIUM | Immediate |
| P-CC-05 | Sabatier coefficients {1,4,1,2} | {mu,tau,mu,phi} | 1 | HIGH/LOW | Immediate |
| P-CC-06 | CO2 MW = 44 | tau*(sigma-mu) | 1 | EXACT/LOW | Immediate |
| P-CC-07 | Diamond 4 bonds, 8 atoms/cell | tau, sigma-tau | 1 | VERY HIGH | Immediate |
| P-CC-08 | DAC energy / minimum = 10 | sigma-phi=10 | 2 | MEDIUM-HIGH | 2026-2028 |
| P-CC-09 | MECS voltage = 1.2V | sigma/(sigma-phi) | 2 | MEDIUM | 2026-2028 |
| P-CC-10 | PEI optimal loading = 12 wt% | sigma=12 | 2 | LOW-MEDIUM | 2026-2027 |
| P-CC-11 | Hex monolith 15% lower dP | n=6 geometry | 2 | MEDIUM | 2026-2027 |
| P-CC-12 | Carnot = 1/6 at 360K | 1/n | 2 | MEDIUM | 2026-2027 |
| P-CC-13 | Ca CN=6 across all phases | CN=n=6 | 2 | MEDIUM-HIGH | 2026-2028 |
| P-CC-14 | Climeworks Gen3 = 6 or 12 kt/yr | n or sigma | 3 | LOW-MEDIUM | 2028-2030 |
| P-CC-15 | CCS pipeline 8-12 MPa | sigma-tau to sigma | 3 | LOW-MEDIUM | 2027-2032 |
| P-CC-16 | CO2 carbon fiber 12K/24K tow | sigma, J2 | 3 | MEDIUM | 2028-2033 |
| P-CC-17 | Sabatier optimal 360C | n*sopfr*sigma | 3 | LOW | 2026-2029 |
| P-CC-18 | CO2-to-CO at -1.2V | sigma/(sigma-phi) | 3 | LOW-MEDIUM | 2026-2028 |
| P-CC-19 | DAC cost floor $60/t | sigma*sopfr=60 | 4 | LOW | 2035-2040 |
| P-CC-20 | CO2-to-diamond 16.7% efficiency | 1/n | 4 | LOW | 2030-2040 |
| P-CC-21 | Global DAC 144 Mt/yr by 2040 | sigma^2=144 | 4 | LOW | 2040 |
| P-CC-22 | CO2-to-CNT (6,6) dominant | (n,n) | 4 | LOW | 2030-2035 |
| P-CC-23 | Cross-DSE optimal = CN=6 throughout | n=6 alignment | 4 | LOW-MEDIUM | 2030-2035 |
| P-CC-24 | Artificial photosynthesis 24 e-/CO2 | J2=24 | 4 | MEDIUM | 2030-2040 |

---

## Confidence Distribution

```
  VERY HIGH  ██          2  (established science, not predictions)
  HIGH       ████        4  (known chemistry with n=6 labels)
  MEDIUM-HIGH ███        3  (reasonable extrapolations)
  MEDIUM     █████       5  (plausible but uncertain)
  LOW-MEDIUM ██████      6  (speculative within range)
  LOW        ████        4  (highly speculative)
```

## Honesty Assessment

This prediction set spans a wide range of confidence levels. The honest breakdown:

1. **Verified facts labeled as predictions** (P-CC-02, 03, 05, 06, 07): These are established chemistry/physics. The n=6 connection is post-hoc labeling of small integers that naturally appear in molecular stoichiometry. They cannot be "falsified" because they are known.

2. **Reasonable engineering predictions** (P-CC-01, 08, 11, 12, 13): These make testable claims within known ranges, where the n=6 value falls within the physically expected interval.

3. **Speculative industrial predictions** (P-CC-09, 10, 14, 15, 17, 18, 19): These assign n=6 expressions to engineering parameters that depend on many non-n=6 factors (economics, policy, catalyst development).

4. **Frontier technology predictions** (P-CC-16, 20, 21, 22, 23, 24): These project n=6 patterns onto technologies that do not yet exist at scale. These are the true tests of whether n=6 arithmetic is predictive or merely descriptive.

The strongest predictions are P-CC-01 (CN=6 superiority for CO2 capture, physically motivated) and P-CC-08 (DAC 10x energy ratio, empirically observed). The weakest is P-CC-22 (CNT chirality from CO2, no physical mechanism).

---

## Related Documents

- [hypotheses.md](hypotheses.md) -- 30 hypotheses (100% EXACT, v4)
- [verification.md](verification.md) -- Independent verification of all 30 hypotheses
- [goal.md](goal.md) -- 8-level architecture roadmap
- [hexa-sorbent.md](hexa-sorbent.md) -- Level 0: CN=6 sorbent materials
- [hexa-process.md](hexa-process.md) -- Level 1: Capture process design
- [hexa-reactor.md](hexa-reactor.md) -- Level 2: Reactor core
- [dse-results.md](dse-results.md) -- DSE exploration results
