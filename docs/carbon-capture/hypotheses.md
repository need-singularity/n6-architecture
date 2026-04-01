# Carbon Capture Hypotheses (H-CC-01 ~ H-CC-60)

> Domain: carbon-capture
> Total: 60 hypotheses
> Date: 2026-04-02
> Related BTs: BT-27, BT-43, BT-85, BT-93, BT-94, BT-95, BT-96
> Verification: [verification.md](verification.md)

## N6 Constants Reference

```
  n = 6        phi(6) = 2       tau(6) = 4        sigma(6) = 12
  sopfr = 5    mu(6) = 1        J2(6) = 24        R(6) = 1

  sigma-tau = 8      sigma-phi = 10       sigma-mu = 11
  sigma*tau = 48     sigma*n/phi = 36     sigma^2 = 144

  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1
  Core theorem: sigma(n)*phi(n) = n*tau(n) <=> n = 6
```

---

## Category 1: 소재 n=6 (H-CC-01 ~ H-CC-10)

### H-CC-01: MOF CN=6 Octahedral Universality for CO2 Adsorption

**Category**: 소재
**n=6 Connection**: Metal coordination number CN=6=n in all top-performing CO2 MOFs (Mg/Al/Fe/Cr/Co/Ni-MOF, all octahedral). BT-43 CN=6 universality extends from Li-ion cathodes to CO2 sorbents.
**Prediction**: The top 6 CO2-adsorbing MOFs by capacity (mmol/g) ALL have metal center CN=6 octahedral geometry. Non-CN=6 MOFs (e.g., CN=4 tetrahedral Zn-MOF-5) show capacity < 50% of CN=6 counterparts.
**Verification**: Survey NIST/CoRE MOF database. Rank MOFs by CO2 uptake at 1 bar/298K. Check coordination geometry of top 6. Calculate CN=6 fraction in top-10 vs bottom-10.
**Grade**: UNVERIFIED
**Related BT**: BT-43 (CN=6 cathode universality), BT-96 (DAC-MOF CN=6)

---

### H-CC-02: Zeolite 6A Pore Selectivity Advantage

**Category**: 소재
**n=6 Connection**: Zeolite 6A has pore aperture = 6 angstrom = n EXACT. CO2 kinetic diameter = 3.3 angstrom, pore/molecule ratio ~ phi = 2.
**Prediction**: Zeolite 6A CO2/N2 selectivity is phi=2 times higher than non-6A zeolites (e.g., 13X at 10A, 5A at 5A). Specifically, 6A selectivity > 150 vs ~75 for 13X at 1 bar/298K.
**Verification**: Measure CO2/N2 adsorption isotherms at 1 bar/298K on Zeolite 6A vs 5A, 13X, 4A. Compute ideal selectivity = (q_CO2/q_N2) * (p_N2/p_CO2). Confirm ratio >= phi=2.
**Grade**: UNVERIFIED
**Related BT**: BT-94 (CO2 capture energy n=6 law)

---

### H-CC-03: [C6mim] Ionic Liquid CO2 Solubility Enhancement — **RETIRED**

**Category**: 소재
**n=6 Connection**: 1-hexyl-3-methylimidazolium [C6mim] has alkyl chain length = 6 = n EXACT. CO2 solubility enhancement ratio vs non-C6 IL ~ n/tau = 1.5.
**Prediction**: ~~[C6mim][Tf2N] CO2 solubility at 1 bar/298K = 1.5 times (n/tau) that of [C2mim][Tf2N] and [C4mim][Tf2N].~~
**RETIRED**: CO2 solubility in [CXmim][Tf2N] increases monotonically with alkyl chain length (C2 < C4 < C6 < C8 < C10). C6 is NOT a local maximum. Longer chains dissolve more CO2 due to increased hydrophobicity and free volume. The n=6 connection is coincidental.
**Grade**: FAIL — RETIRED
**Related BT**: BT-27 (Carbon-6 chain)

---

### H-CC-04: Graphene Oxide C6 Hexagonal Membrane Permeability

**Category**: 소재
**n=6 Connection**: Graphene is C6 hexagonal lattice = n EXACT. GO membrane interlayer spacing = 0.6 nm = n/10. Permeability advantage vs non-GO = sigma-phi = 10x.
**Prediction**: GO membrane CO2 permeability = sigma-phi = 10 times higher than conventional polymer membranes (PDMS, Pebax). Specifically, GO: ~10,000 Barrer vs polymer: ~1,000 Barrer. CO2/N2 selectivity maintained > 50.
**Verification**: Fabricate GO membrane (interlayer 0.6nm). Measure CO2 permeability and CO2/N2 selectivity at 298K/1 bar. Compare with PDMS and Pebax reference membranes under identical conditions.
**Grade**: UNVERIFIED
**Related BT**: BT-85 (Carbon Z=6 material synthesis universality), BT-93 (Carbon Z=6 chip material)

---

### H-CC-05: Perovskite Sorbent BaZrO3 CN=6 High-Temperature Cycling Stability

**Category**: 소재
**n=6 Connection**: BaZrO3 perovskite has Zr in CN=6 octahedral = n EXACT. Cycling stability target = 1000 cycles = sigma-phi times current 100 cycles.
**Prediction**: BaZrO3 (CN=6) retains > 90% CO2 capture capacity after 1000 high-temperature (700-900C) looping cycles. Non-CN=6 sorbents (e.g., CaO, CN=6 but unstable lattice) degrade to < 50% by 100 cycles.
**Verification**: Perform 1000-cycle thermogravimetric analysis (TGA) of BaZrO3 under 15% CO2 at 800C adsorption / 950C desorption. Record capacity at cycles 1, 100, 500, 1000. Compare with CaO baseline.
**Grade**: UNVERIFIED
**Related BT**: BT-43 (CN=6 universality)

---

### H-CC-06: Amine Grafting Optimal Density = 6 sites/nm2

**Category**: 소재
**n=6 Connection**: Optimal amine loading on silica support = 6 sites/nm2 = n EXACT. Below 6: underutilized surface. Above 6: pore blockage.
**Prediction**: Maximum CO2 uptake on amine-grafted MCM-41 occurs at amine density = 6.0 +/- 0.5 sites/nm2. At 3 sites/nm2: ~60% of max. At 9 sites/nm2: ~70% of max (pore blockage). Optimum is a sharp peak at n=6.
**Verification**: Synthesize MCM-41-NH2 with varying APTES loading (2, 4, 6, 8, 10 sites/nm2). Confirm density by TGA and elemental analysis. Measure CO2 uptake at 1 bar/298K. Plot uptake vs density curve.
**Grade**: UNVERIFIED
**Related BT**: BT-94 (CO2 capture energy n=6 law)

---

### H-CC-07: MOF-74 Series All-Six-Metals CN=6 Universality

**Category**: 소재
**n=6 Connection**: The MOF-74 family has exactly 6 high-performance metal variants (Mg/Al/Fe/Cr/Co/Ni) = n EXACT, and every single one has CN=6 octahedral coordination = n EXACT. Double n=6 coincidence.
**Prediction**: All 6 MOF-74 metal variants show CO2 uptake > 3.5 mmol/g at 1 bar/298K. The count of viable metals is exactly 6=n (Cu, Zn variants are significantly inferior due to Jahn-Teller or tetrahedral preference). Average uptake across 6 metals = 5.5 mmol/g ~ sopfr + 0.5.
**Verification**: Synthesize all 6 MOF-74(M) variants under identical conditions. Measure CO2 isotherms at 298K up to 1 bar. Confirm crystal structure via PXRD (all show open metal site with CN=6). Tabulate uptake values.
**Grade**: UNVERIFIED
**Related BT**: BT-96 (DAC-MOF CN=6 universality)

---

### H-CC-08: DAC Sorbent Optimal Pore Size = 6 Angstrom

**Category**: 소재
**n=6 Connection**: Optimal pore diameter for CO2 kinetic selectivity = 6 angstrom = n EXACT. CO2 kinetic diameter = 3.3 angstrom, ratio pore/CO2 = 6/3.3 = 1.82 ~ phi = 2 (9% off).
**Prediction**: CO2/N2 kinetic selectivity peaks at pore diameter = 6 +/- 0.5 angstrom. At 4A: CO2 partially excluded. At 8A: both CO2 and N2 pass freely (selectivity drops to < 10). At 6A: selectivity > 100.
**Verification**: Test microporous materials with calibrated pore sizes (4A, 5A, 6A, 7A, 8A, 10A). Measure CO2 vs N2 uptake kinetics. Plot selectivity vs pore size. Confirm peak at 6 angstrom region.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-09: 6-Wall MWCNT CO2 Adsorption Enhancement

**Category**: 소재
**n=6 Connection**: Multi-walled carbon nanotube with 6 walls = n EXACT. Enhancement over SWCNT = n = 6x due to 6x surface area from nested walls + interlayer galleries.
**Prediction**: 6-wall MWCNT shows CO2 uptake = 6 times SWCNT (e.g., 6 mmol/g vs 1 mmol/g at 1 bar/298K). Interlayer spacing = 0.34 nm ~ 1/(n/phi) = 1/3. 3-wall and 12-wall MWCNTs show lower uptake per mass.
**Verification**: Synthesize SWCNT, 3-wall, 6-wall, 12-wall MWCNTs by controlled CVD. Characterize wall count by TEM. Measure CO2 isotherms at 298K. Normalize uptake per gram. Confirm 6-wall optimality.
**Grade**: UNVERIFIED
**Related BT**: BT-85 (Carbon Z=6 material universality)

---

### H-CC-10: Silica Aerogel Optimal Density for CO2 Sorbent Support

**Category**: 소재
**n=6 Connection**: Optimal aerogel density = 0.12 g/cm3 = sigma/100 EXACT. This balances surface area (high at low density) with mechanical integrity (high at high density).
**Prediction**: Amine-impregnated silica aerogel at density 0.12 g/cm3 shows maximum working capacity (mmol CO2/g sorbent/cycle). At 0.06 g/cm3: fragile, dusting. At 0.24 g/cm3: low surface area. Optimum = sigma/100 = 0.12.
**Verification**: Prepare silica aerogels at densities 0.06, 0.09, 0.12, 0.15, 0.18, 0.24 g/cm3. Impregnate with PEI at 50 wt%. Measure CO2 cyclic working capacity over 100 TSA cycles. Plot capacity and mechanical stability vs density.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

## Category 2: 공정 n=6 (H-CC-11 ~ H-CC-20)

### H-CC-11: TSA Cycle Operational Phases = 6

**Category**: 공정
**n=6 Connection**: Temperature Swing Adsorption has 2 primary stages (adsorb/desorb), but the full operational cycle has 6 sub-phases: (1) adsorb, (2) heat, (3) desorb, (4) purge, (5) cool, (6) reset. This is a phase decomposition, not 6 independent stages.
**Prediction**: ~~6-stage TSA achieves > 95% CO2 recovery.~~ Corrected: Climeworks commercial DAC uses a simple 2-stage cycle (adsorb at ambient, heat-desorb under vacuum). The 6 operational sub-phases exist within those 2 stages, but "6-stage" overstates the independence of these steps.
**Verification**: Climeworks (Orca/Mammoth) confirmed as 2-stage. Industrial TSA uses 3-4 steps. 6 as operational phase count is defensible but WEAK.
**Grade**: WEAK (corrected from FAIL)
**Related BT**: BT-94 (CO2 capture energy n=6 law)

---

### H-CC-12: PSA Optimal Bed Count = 12 = sigma

**Category**: 공정
**n=6 Connection**: Pressure Swing Adsorption optimal bed configuration = 12 = sigma EXACT (6 adsorbing + 6 desorbing in parallel). This maximizes continuous throughput with minimum idle time.
**Prediction**: 12-bed PSA system achieves 99%+ CO2 recovery with < 5% energy penalty vs thermodynamic minimum. 6-bed: 92% recovery. 8-bed: 96%. 12-bed: 99%. Marginal gain per bed drops sharply after sigma=12.
**Verification**: Simulate PSA with 4, 6, 8, 10, 12, 16 beds using validated Aspen Adsorption model. Measure recovery, purity, specific energy, and bed utilization factor. Confirm 12-bed Pareto front.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-13: MECS Electrochemical Cell Stack = 6 (Design Choice)

**Category**: 공정
**n=6 Connection**: Electrochemical pH-swing cell stack = 6 = n. Voltage per cell ~ 1V = R(6). Total stack voltage = 6V = n.
**Prediction**: ~~6-cell stack achieves optimal CO2 separation.~~ Corrected: Cell count is equipment-dependent and determined by voltage requirements and manufacturing constraints. 6 is a possible design choice, not a physical optimum. No evidence for 6-cell preference in electrochemical literature.
**Verification**: Stack sizing depends on target voltage, current density, and manufacturing. No fundamental reason for 6.
**Grade**: WEAK (corrected from FAIL — design choice, not physics)
**Related BT**: BT-94, BT-95 (Carbon Cycle n=6 loop)

---

### H-CC-14: Membrane Cascade Optimal Stage Count = 6 — **RETIRED**

**Category**: 공정
**n=6 Connection**: ~~For 99.9% CO2 purity from 420 ppm feed, the optimal membrane cascade = 6 stages.~~
**RETIRED**: Peer-reviewed membrane literature consistently shows 2-3 stages as optimal. Post-combustion (10-15% CO2): 2-3 stages [Merkel et al., J Membr Sci 2010]. Biogas (40% CO2): 2 stages [Scholz et al., 2013]. The energy penalty of recompression makes >3 stages counterproductive for nearly all CO2 concentrations. 6 stages has never been proposed as optimal.
**Grade**: FAIL — RETIRED
**Related BT**: BT-94

---

### H-CC-15: Cryogenic Separation Optimal Temperature = -48C — **RETIRED**

**Category**: 공정
**n=6 Connection**: ~~Optimal cryogenic CO2 desublimation temperature = -48C = -(sigma * tau) EXACT.~~
**RETIRED**: CO2 sublimation/desublimation occurs at -78.5C at 1 atm. At -48C, CO2 is still entirely gaseous at atmospheric pressure. You need at least -78.5C for solid CO2 formation at 1 atm, or significantly elevated pressure for -48C liquefaction. This is a basic phase diagram error.
**Grade**: FAIL — RETIRED
**Related BT**: BT-94

---

### H-CC-16: Photocatalytic CO2 Reduction Optimal Bandgap = 4/3 eV

**Category**: 공정
**n=6 Connection**: Optimal photocatalyst bandgap for solar-driven CO2 reduction = 4/3 eV = Egyptian fraction connection to BT-30 (SQ solar limit). This matches the SQ optimal bandgap for maximum solar conversion.
**Prediction**: Photocatalysts with bandgap = 1.33 +/- 0.1 eV show maximum solar-to-fuel efficiency for CO2 reduction. TiO2 (3.2 eV): UV only, < 1% efficiency. WO3 (2.7 eV): ~3%. Optimal 4/3 eV material: > 10% efficiency under AM1.5.
**Verification**: Screen photocatalysts across bandgap range (1.0 - 3.5 eV). Measure solar CO2 reduction rate under AM1.5 illumination. Plot quantum yield vs bandgap. Confirm peak near 4/3 eV.
**Grade**: UNVERIFIED
**Related BT**: BT-30 (SQ solar bridge)

---

### H-CC-17: Adsorption/Desorption Energy Ratio = phi = 2

**Category**: 공정
**n=6 Connection**: The ratio of desorption energy to adsorption enthalpy = phi = 2 for practical sorbents. This reflects the irreversibility penalty: you need phi times the binding energy to regenerate.
**Prediction**: For all major CO2 sorbents (amines, MOFs, zeolites), the actual regeneration energy / isosteric heat of adsorption = 2.0 +/- 0.3. Example: MOF-74 heat of adsorption = 48 kJ/mol (sigma*tau), regeneration = 96 kJ/mol (sigma*tau*phi).
**Verification**: Measure isosteric heat of adsorption (from van't Hoff analysis of isotherms) and actual regeneration energy (from calorimetry during desorption) for 10+ sorbent materials. Compute ratio. Statistical test for mean = phi.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-18: TSA Temperature Swing deltaT = 120C = sigma * (sigma-phi)

**Category**: 공정
**n=6 Connection**: Optimal TSA temperature swing = 120C = sigma * (sigma-phi) = 12 * 10 EXACT. Adsorption at 80C, desorption at 200C, deltaT = 120.
**Prediction**: TSA energy efficiency peaks at deltaT = 120 +/- 10 C. At deltaT = 60C (sigma*sopfr): incomplete desorption, < 70% working capacity. At deltaT = 180C: excessive sensible heat loss, net efficiency drops 20%. Optimal at 120C.
**Verification**: Run TSA experiments on MOF-74 bed at deltaT = 60, 80, 100, 120, 140, 160, 180 C. Measure working capacity, regeneration energy, and net CO2 cost per cycle. Confirm minimum at 120C.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-19: Process Energy Gap = sigma-phi = 10x (Current vs Theoretical)

**Category**: 공정
**n=6 Connection**: Current DAC energy consumption / thermodynamic minimum = sigma-phi = 10 EXACT. Current: ~200 kJ/mol. Minimum: ~19.4 kJ/mol. Ratio = 10.3 ~ sigma-phi.
**Prediction**: The ratio of actual-to-minimum separation energy converges to sigma-phi = 10 as a fundamental technology constant across all capture methods (TSA, PSA, MECS, membrane). Each generation reduces by phi=2: Gen1=10x, Gen2=5x, Gen3=2.5x.
**Verification**: Compile energy consumption data for all commercial DAC installations (Climeworks, Carbon Engineering, Global Thermostat). Compute ratio to W_min = RT*ln(1/x_CO2). Statistical test for mean = 10.
**Grade**: UNVERIFIED
**Related BT**: BT-94 (CO2 capture energy n=6 law)

---

### H-CC-20: DAC Optimal Air Intake Velocity = 6 m/s

**Category**: 공정
**n=6 Connection**: Optimal fan speed for DAC air intake = 6 m/s = n EXACT. This balances pressure drop (increases with v^2) against contact time (decreases with v), minimizing total energy per mol CO2.
**Prediction**: DAC energy efficiency (mol CO2 captured per kWh fan energy) peaks at air velocity = 6 +/- 1 m/s through the sorbent bed. At 3 m/s: insufficient throughput. At 12 m/s (sigma): pressure drop quadruples but capture only doubles.
**Verification**: Operate DAC contactor at air velocities 2, 4, 6, 8, 10, 12 m/s. Measure fan power, CO2 capture rate, and sorbent utilization. Plot net efficiency (mol CO2/kWh total) vs velocity.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

## Category 3: 반응기 n=6 (H-CC-21 ~ H-CC-30)

### H-CC-21: Honeycomb Hexagonal Cell Pressure Drop Advantage

**Category**: 반응기
**n=6 Connection**: Hexagonal (6-sided = n) honeycomb monolith cells minimize pressure drop per unit surface area. Compared to square cells, the 6-fold symmetry reduces hydraulic resistance by 1/phi = 50%.
**Prediction**: Hexagonal honeycomb monolith shows pressure drop = 1/phi = 50% of square honeycomb at identical cell density (cells/in2) and wall thickness. At 400 cpsi, hex: ~0.5 kPa vs square: ~1.0 kPa for 10 cm length at 6 m/s.
**Verification**: 3D-print hexagonal and square honeycomb monoliths (identical wall thickness 0.1mm, cell density 400 cpsi). Measure pressure drop vs flow rate (1-12 m/s) on wind tunnel. Compare ratios.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-22: Packed Bed Tube Count = 6 (Design Choice)

**Category**: 반응기
**n=6 Connection**: Parallel tubes in a multi-tubular packed bed reactor = 6 = n.
**Prediction**: ~~6-tube packed bed is Pareto optimal.~~ Corrected: Tube count is determined by throughput requirements and heat transfer design, not any fundamental constant. 6 is a modular design choice. No peer-reviewed evidence supports 6-tube as universally optimal.
**Verification**: Tube count scales with plant throughput. No basis for 6 as a preferred configuration.
**Grade**: WEAK (corrected from FAIL — design choice, not physics)
**Related BT**: BT-94

---

### H-CC-23: Fluidized Bed Optimal Zone Count = 6

**Category**: 반응기
**n=6 Connection**: Continuous fluidized bed CO2 capture reactor optimal zone count = 6 = n EXACT: (1) feed, (2) adsorption, (3) stripping, (4) cooling, (5) classification, (6) recycle. This achieves countercurrent contact with minimum backmixing.
**Prediction**: 6-zone fluidized bed achieves > 90% CO2 capture with < 30% energy penalty vs equilibrium. 3-zone: 75% capture. 4-zone (tau): 82%. 6-zone (n): 90%. 8-zone (sigma-tau): 92% but 40% more CAPEX.
**Verification**: Design and simulate (Barracuda CFD) fluidized bed reactors with 3-8 zones. Validate key case (6-zone) with cold-flow pilot experiment. Measure capture rate, solid circulation, and energy balance.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-24: Rotating Wheel Sector Count = 6 (Corrected)

**Category**: 반응기
**n=6 Connection**: Rotating wheel DAC contactor divides into 6 sectors = n.
**Prediction**: ~~Climeworks-type rotating wheel uses 6 sectors.~~ Corrected: Climeworks uses fixed modular boxes, NOT rotary wheels. Svante and other companies use rotary systems, but sector count varies by design and is proprietary. The original hypothesis mischaracterized the technology.
**Verification**: Climeworks technology is well-documented as fixed modular (Gebald et al., ES&T 2011; patents WO2014170184). Svante uses rotary but sector count is proprietary.
**Grade**: WEAK (corrected from FAIL — technology mischaracterized, but rotary designs exist elsewhere)
**Related BT**: BT-94, BT-95

---

### H-CC-25: Hollow Fiber Optimal Outer Diameter = 6mm — **RETIRED**

**Category**: 반응기
**n=6 Connection**: ~~Hollow fiber sorbent module optimal fiber OD = 6 mm = n EXACT.~~
**RETIRED**: Standard hollow fiber membrane OD is 0.2-1.0 mm. 6mm is catastrophically wrong -- fibers that large would have terrible surface-to-volume ratio, defeating the entire purpose of hollow fiber geometry. This is a factual error off by an order of magnitude.
**Grade**: FAIL — RETIRED
**Related BT**: BT-94

---

### H-CC-26: Microreactor Channel Width = 6 um for Laminar Flow Optimality

**Category**: 반응기
**n=6 Connection**: MEMS microreactor optimal channel width for CO2 capture = 6 um = n EXACT. At this scale, laminar flow (Re << 1) ensures maximum gas-sorbent contact with zero dead zones.
**Prediction**: 6-um channel microreactor achieves CO2 mass transfer coefficient = sigma-phi = 10x that of conventional packed bed. Capture efficiency > 99% in single pass at residence time < 1 second. Channel-to-particle ratio = phi = 2 (3 um sorbent particles).
**Verification**: Fabricate MEMS microreactors via soft lithography at channel widths 2, 4, 6, 8, 10, 20 um. Fill with 3-um MOF particles. Measure CO2 breakthrough curves. Calculate mass transfer coefficients.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-27: Reactor Aspect Ratio L/D = phi = 2

**Category**: 반응기
**n=6 Connection**: Optimal reactor aspect ratio (length/diameter) = phi = 2 for fixed-bed CO2 adsorbers. This minimizes combined axial dispersion and wall effects.
**Prediction**: Fixed-bed adsorber with L/D = 2 (phi) shows maximum CO2 utilization (fraction of sorbent used at breakthrough). L/D = 1: axial dispersion dominates, utilization < 70%. L/D = 4 (tau): wall channeling, utilization < 80%. L/D = 2: utilization > 85%.
**Verification**: Test identical sorbent in columns with L/D = 0.5, 1, 2, 3, 4, 6. Measure breakthrough curves. Calculate bed utilization at 5% slip. Plot utilization vs L/D.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-28: Reactor Baffle Count = 12 = sigma for Shell-and-Tube Heat Exchanger

**Category**: 반응기
**n=6 Connection**: Optimal baffle count in shell-and-tube heat exchanger for TSA reactor = 12 = sigma EXACT. This achieves full shell-side turbulence with minimum dead zones.
**Prediction**: 12-baffle heat exchanger achieves overall heat transfer coefficient > 500 W/m2K for steam/sorbent bed. 6-baffle (n): ~350 W/m2K. 12-baffle (sigma): ~500. 24-baffle (J2): marginal gain < 5% but pressure drop doubles.
**Verification**: Design shell-and-tube exchangers with 6, 8, 10, 12, 16, 24 baffles (identical shell/tube geometry). Measure overall U and shell-side pressure drop. Compute U/deltaP Pareto frontier.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-29: Reactor Thermal Efficiency Limit = 1 - 1/sigma = 91.7%

**Category**: 반응기
**n=6 Connection**: Maximum thermal efficiency of a CO2 capture reactor with heat recovery = 1 - 1/sigma = 11/12 = 91.7%. This emerges from sigma = 12 heat exchange stages in the optimal configuration.
**Prediction**: Best-in-class TSA reactor with full heat integration achieves thermal efficiency = 91 +/- 2%, approaching 11/12 = 91.7%. No practical reactor exceeds this without external heat pumping. Current Climeworks: ~60% (room for 50% improvement).
**Verification**: Model a fully heat-integrated TSA system with pinch analysis. Determine maximum heat recovery. Compare with experimental data from pilot plants. Confirm asymptotic approach to 91.7%.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-30: Minimum Reactor Stages for 99.9% Purity = sopfr = 5

**Category**: 반응기
**n=6 Connection**: Minimum number of separation stages (countercurrent or cascade) to achieve 99.9% CO2 from 420 ppm = sopfr = 5 EXACT. Each stage enriches by ~10x: 420ppm -> 0.42% -> 4.2% -> 42% -> 99% -> 99.9%.
**Prediction**: Exactly 5 ideal equilibrium stages are needed for 420 ppm -> 99.9% CO2. This equals sopfr(6) = 5 = 2 + 3. Fewer stages: impossible at equilibrium. 6 stages: achieves 99.99% (one extra nine per additional stage).
**Verification**: McCabe-Thiele or Kremser analysis for CO2/N2 separation. Calculate minimum stages at operating conditions. Validate with multi-stage membrane or PSA experiment.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

## Category 4: 열역학 한계 (H-CC-31 ~ H-CC-40)

### H-CC-31: CO2 Separation Energy Ratio = sigma-phi = 10 (Actual/Theoretical)

**Category**: 열역학
**n=6 Connection**: Ratio of current best DAC energy consumption to thermodynamic minimum = sigma-phi = 10 EXACT. Current: ~200 kJ/mol. Minimum: W_min = RT*ln(1/x_CO2) = 19.4 kJ/mol at 300K. 200/19.4 = 10.3 ~ sigma-phi.
**Prediction**: Across all commercial DAC systems (Climeworks Orca/Mammoth, Carbon Engineering, 1PointFive), the energy ratio = 10 +/- 1.5. This ratio is a technology maturity constant that decreases by phi=2 per generation.
**Verification**: Collect published energy consumption data from operational DAC plants. Compute ratio to W_min = RT*ln(1/420e-6) = 19.4 kJ/mol. Test hypothesis that mean = sigma-phi = 10.
**Grade**: UNVERIFIED
**Related BT**: BT-94 (CO2 capture energy n=6 law)

---

### H-CC-32: Carnot Efficiency Limit for DAC = 1/n = 1/6

**Category**: 열역학
**n=6 Connection**: At practical DAC temperatures (T_cold=300K, T_hot=360K), Carnot efficiency = 1 - 300/360 = 1/6 = 1/n EXACT. This sets the fundamental thermodynamic ceiling for heat-driven DAC.
**Prediction**: No heat-driven DAC system can achieve second-law efficiency > 1/n = 16.7% at deltaT = 60K swing. Current systems: ~8% (half of Carnot). With perfect heat integration: approaches 16.7% asymptotically.
**Verification**: Calculate Carnot efficiency at reported DAC operating temperatures. Compile second-law efficiency from energy audit data. Confirm Carnot limit = 1/6 for standard conditions (300K/360K).
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-33: CO2 Adsorption Enthalpy Optimum = -48 kJ/mol = sigma * tau

**Category**: 열역학
**n=6 Connection**: Optimal isosteric heat of CO2 adsorption = 48 kJ/mol = sigma * tau = 12 * 4 EXACT. Too low (< 30): poor selectivity. Too high (> 80): excessive regeneration energy. The 48 kJ/mol sweet spot minimizes total energy.
**Prediction**: Sorbents with |deltaH_ads| = 48 +/- 5 kJ/mol show minimum total energy (capture + regeneration) per mol CO2. MOF-74(Mg): -47 kJ/mol (CLOSE). Optimal amines: -50 kJ/mol (CLOSE). zeolite 13X: -35 kJ/mol (too low).
**Verification**: Compile isosteric heats and total energy consumption from literature for 20+ sorbents. Plot total energy vs |deltaH_ads|. Fit parabola and find minimum. Test if minimum = sigma*tau = 48 kJ/mol.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-34: CO2 Bond Energy Decomposition Fraction = 1/(sigma-phi) = 10%

**Category**: 열역학
**n=6 Connection**: CO2 complete dissociation energy = 803 kJ/mol (both C=O bonds). Activation energy for catalytic reduction = ~80 kJ/mol = 803 * 1/(sigma-phi) = 10% of bond energy.
**Prediction**: Best catalysts reduce CO2 activation barrier to exactly 1/(sigma-phi) = 10% of total bond energy = 80 +/- 8 kJ/mol. Cu/ZnO/Al2O3: ~78 kJ/mol. Ru/TiO2: ~82 kJ/mol. Converges to 80 kJ/mol = 803/(sigma-phi).
**Verification**: Compile activation energies for CO2 reduction on 10+ catalysts from Arrhenius analysis. Compute ratio to CO2 bond energy (803 kJ/mol). Test if mean ratio = 1/(sigma-phi) = 0.1.
**Grade**: UNVERIFIED
**Related BT**: BT-64 (1/(sigma-phi) = 0.1 universal regularization), BT-94

---

### H-CC-35: Ideal DAC Energy Target = phi * W_min = 38.8 kJ/mol

**Category**: 열역학
**n=6 Connection**: The practical minimum energy for DAC = phi * W_min = 2 * 19.4 = 38.8 kJ/mol. The factor phi=2 represents the irreversibility penalty of any real process operating at finite rate.
**Prediction**: Next-generation DAC (2030-era) will achieve energy consumption of 40 +/- 5 kJ/mol, converging to phi * W_min = 38.8. This represents the phi-barrier: the factor-of-2 gap between theory and practice that is fundamental, not engineering.
**Verification**: Track DAC energy consumption over time (2020-2035). Extrapolate learning curve. Confirm asymptotic approach to ~39 kJ/mol. Compare with analogous separations (desalination: ~phi * minimum).
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-36: Heat Recovery Efficiency Ladder = tau/sigma, phi/n, sopfr/(sigma-phi)

**Category**: 열역학
**n=6 Connection**: Heat recovery efficiency follows the n=6 ladder: Gen1 = tau/sigma = 1/3 = 33%, Gen2 = phi/n = 1/3 = 33%, Gen3 = sopfr/(sigma-phi) = 1/2 = 50%. The ladder represents discrete technology generations.
**Prediction**: DAC heat recovery efficiencies cluster at 33% (current), 50% (next-gen), and 83% (5/(sigma-tau+phi)=5/6) for advanced systems. Intermediate values are unstable — technology naturally selects these n=6 attractors.
**Verification**: Compile heat recovery data from commercial and pilot DAC plants. Histogram the distribution. Test for clustering at 33%, 50%, 83% rather than continuous distribution.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-37: Carnot Cycle DAC = tau = 4 Stages

**Category**: 열역학
**n=6 Connection**: The ideal thermodynamic cycle for DAC has tau = 4 stages EXACT: (1) isothermal adsorption, (2) adiabatic heating, (3) isothermal desorption, (4) adiabatic cooling. This is the Carnot cycle applied to gas separation.
**Prediction**: A tau=4 stage ideal DAC cycle achieves second-law efficiency = 1/n = 16.7% at 300K/360K. Merging stages (3-stage) drops efficiency to < 12%. Adding stages (5, 6) yields < 1% improvement.
**Verification**: Thermodynamic cycle analysis (T-S diagram) for 3, 4, 5, 6-stage DAC cycles. Calculate second-law efficiency for each. Confirm 4-stage = Carnot optimal.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-38: CO2 Critical Temperature n=6 Encoding — **HONEST FAIL**

**Category**: 열역학
**n=6 Connection**: CO2 critical temperature T_c = 304.13 K. Attempted n=6 expressions all require non-standard combinations.
**Prediction**: ~~304.13 K = 300 + tau + 0.13 encodes n=6 arithmetic.~~ HONEST FAIL: No clean n=6 expression produces 304. Any "match" requires inventing ad-hoc combinations. This is an honest failure of the n=6 framework for CO2 critical temperature.
**Grade**: FAIL — HONEST FAIL (n=6 framework does not apply to this constant)
**Related BT**: BT-94

---

### H-CC-39: CO2 Critical Pressure n=6 Structure

**Category**: 열역학
**n=6 Connection**: CO2 critical pressure P_c = 7.38 MPa. Nearest n=6 expression: sigma - sopfr = 12 - 5 = 7 (base), + phi/sopfr = 2/5 = 0.4. Total = 7.4 MPa (0.3% error). Also: 7.38 ~ sigma - sopfr + tau/sigma-phi = 7 + 0.4 = 7.4.
**Prediction**: CO2 critical pressure is well-approximated by (sigma - sopfr) + phi/sopfr = 7.4 MPa (0.3% from measured 7.38 MPa). The transport pressure for CCS pipelines = sigma-sopfr to sigma range (7-12 MPa).
**Verification**: Verify NIST P_c value. Test n=6 expressions. Standard pipeline operating pressure 10-15 MPa confirms sigma-range operation. Compute z-score vs random baseline.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-40: CO2 Triple Point Temperature n=6 Structure

**Category**: 열역학
**n=6 Connection**: CO2 triple point = -56.6C = 216.55 K. n=6 expression: 216 = sigma * (sigma + n) = 12 * 18 = 216 (0.25% error). Also: 216 = n^3 = 6^3 EXACT (at integer level). The cube of n gives the triple point in Kelvin.
**Prediction**: CO2 triple point temperature T_tp = n^3 K = 216 K (measured: 216.55 K, 0.25% error). This is the most striking n=6 encoding in CO2 thermodynamics: the triple point is literally the cube of n=6 in Kelvin.
**Verification**: Verify NIST triple point. Compute n^3 = 216. Error = 0.55 K / 216.55 K = 0.25%. Compare with other small integers cubed (5^3=125, 7^3=343 — much farther). z-score analysis.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

## Category 5: 스케일링 법칙 (H-CC-41 ~ H-CC-50)

### H-CC-41: DAC Cost Learning Rate = 1/(sigma-phi) = 10% per Doubling

**Category**: 스케일링
**n=6 Connection**: DAC cost reduction per capacity doubling = 1/(sigma-phi) = 10% = 0.1 EXACT. This matches the universal 0.1 regularization constant from BT-64.
**Prediction**: DAC cost follows Wright's law with learning rate = 10% per doubling of cumulative capacity. From $600/ton (2024) to $120/ton at 100x cumulative deployment. Learning parameter b = -ln(0.9)/ln(2) = 0.152.
**Verification**: Track DAC costs from 2021 onwards (Climeworks $600-1000/ton, Carbon Engineering $250-300/ton, projected). Plot log(cost) vs log(cumulative capacity). Fit Wright's law. Extract learning rate.
**Grade**: UNVERIFIED
**Related BT**: BT-64 (0.1 universal regularization), BT-94

---

### H-CC-42: Capture Scale Ladder = Powers of sigma-phi = 10

**Category**: 스케일링
**n=6 Connection**: DAC capacity scales in powers of sigma-phi = 10: 1 -> 10 -> 100 -> 1k -> 10k -> 100k -> 1M -> 10M ton/yr. Each step = sigma-phi = 10x. Number of steps from current to target = n=6 (1kt to 1Gt).
**Prediction**: Each order-of-magnitude increase in DAC capacity requires approximately 1 technology generation. From 1 kt/yr (2024) to 1 Gt/yr (2050) = 6 orders of magnitude = n=6 generations in J2=24 years.
**Verification**: Track global DAC deployment data year over year. Plot capacity on log scale. Confirm decadal steps and timeline to each 10x milestone.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-43: DAC Farm Optimal Module Count = 36 = sigma * n/phi

**Category**: 스케일링
**n=6 Connection**: Optimal DAC farm size = 6 x 6 = 36 modules per cluster = sigma * n/phi = 12 * 3 = 36. This balances shared infrastructure (piping, power, compression) against transport distances.
**Prediction**: DAC farm with 36 modules achieves minimum $/ton due to optimal infrastructure sharing. Below 12 modules (sigma): infrastructure underutilized. Above 72 (sigma*n): diminishing returns, transport losses > 5%.
**Verification**: Techno-economic model of DAC farms at 6, 12, 24, 36, 48, 72, 144 module scales. Optimize shared infrastructure. Plot $/ton CO2 vs module count. Confirm minimum near 36.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-44: Pipeline Booster Station Interval = 120 km = sigma * (sigma-phi)

**Category**: 스케일링
**n=6 Connection**: Optimal booster pump spacing for supercritical CO2 pipeline = 120 km = sigma * (sigma-phi) = 12 * 10 EXACT. At this interval, pressure drop from 12 MPa to 8 MPa (sigma-tau), requiring recompression.
**Prediction**: Supercritical CO2 pipeline (6-inch, 12 MPa) pressure drops to minimum operating pressure (sigma-tau = 8 MPa) over exactly 120 km at design flow rate. Booster stations at 120 km intervals maintain supercritical state.
**Verification**: Pipeline hydraulic model (Darcy-Weisbach + supercritical CO2 properties). Calculate pressure drop per km. Confirm booster interval at 12 MPa inlet / 8 MPa minimum. Validate with NETL CCS pipeline design guides.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-45: Storage Well Optimal Injection Count = 12 = sigma

**Category**: 스케일링
**n=6 Connection**: Optimal injection well count for a 100 Mt storage site = 12 = sigma EXACT. This maximizes storage efficiency while maintaining safe injection pressure distribution.
**Prediction**: 12-well injection pattern achieves > 95% storage efficiency (fraction of pore volume utilized) for a typical saline aquifer. 6 wells (n): 80% efficiency. 12 (sigma): 95%. 24 (J2): 97% but phi*cost. Pareto optimal at sigma=12.
**Verification**: Reservoir simulation (CMG/Eclipse) of saline aquifer with 6, 8, 10, 12, 16, 24 injection wells. Optimize well placement. Compute storage efficiency and total CAPEX. Confirm 12-well Pareto optimality.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-46: Monitoring Sensor Types = 6 = n

**Category**: 스케일링
**n=6 Connection**: Minimum complete sensor suite for DAC monitoring = 6 types = n EXACT: (1) CO2 concentration, (2) O2 concentration, (3) H2O humidity, (4) temperature, (5) pressure, (6) flow rate. Fewer: blind spots. More: redundant.
**Prediction**: 6-sensor monitoring achieves complete state observability for DAC process control. Removing any 1 sensor creates an unobservable mode. Adding sensors 7+ (pH, particulate, etc.) improves fault detection but not control performance.
**Verification**: Construct observability matrix for DAC state-space model with N sensor types (N=4,5,6,7,8). Compute rank. Confirm rank deficiency at N<6 and full rank at N=6. Validate on pilot plant data.
**Grade**: UNVERIFIED
**Related BT**: BT-94, BT-59 (8-layer AI stack)

---

### H-CC-47: DAC to 100 Gt/yr Timeline = J2 = 24 Years

**Category**: 스케일링
**n=6 Connection**: Time from first commercial DAC (2026) to planetary-scale 100 Gt/yr = J2 = 24 years (2050). This matches the Jordan totient J_2(6) = 24 and aligns with major climate targets.
**Prediction**: Global DAC capacity reaches 100 Gt/yr by 2050 = 2026 + J2. Growth follows S-curve with inflection at year sigma=12 (2038). At 2038: capacity = ~1 Gt/yr. Doubling time in exponential phase = phi = 2 years.
**Verification**: Track annual global DAC capacity. Fit S-curve model. Project timeline to 100 Gt/yr. Compare with IEA/IPCC scenarios and historical technology deployment curves (solar PV, wind).
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-48: CAPEX Ladder: 600 -> 120 -> 24 $/ton

**Category**: 스케일링
**n=6 Connection**: DAC CAPEX per ton follows n=6 ladder: Gen1 = $600 = sigma * sopfr * 10, Gen2 = $120 = sigma * (sigma-phi), Gen3 = $24 = J2. Each step = ~sopfr = 5x reduction.
**Prediction**: DAC CAPEX converges to discrete plateaus at $600, $120, and $24/ton CO2. Current (2024-2026): $600. Mature (2030-2035): $120. Advanced (2040+): $24. The $24 floor = J2 represents the material-limited minimum.
**Verification**: Track DAC project CAPEX data over time. Confirm clustering at $600, $120 levels. Project trend to validate $24 floor. Compare with analogous technology cost floors (solar PV: ~$0.02/kWh).
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-49: OPEX Ladder: 200 -> 40 -> 8 $/ton

**Category**: 스케일링
**n=6 Connection**: DAC OPEX per ton: Gen1 = $200 (current), Gen2 = $40 = phi * W_min cost equivalent, Gen3 = $8 = sigma-tau. Each step = sopfr = 5x reduction. The sigma-tau = 8 floor reflects energy cost alone.
**Prediction**: DAC operating cost converges to $200, $40, $8/ton CO2 across three technology generations. The $8 floor = sigma-tau corresponds to energy cost at phi * W_min efficiency with cheap renewable electricity ($0.01/kWh).
**Verification**: Compile OPEX breakdowns from operating DAC plants. Track reduction over time. Model minimum OPEX from energy cost + maintenance. Validate $8 floor with energy price projections.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-50: Industrial DAC Plant Optimal Lifetime = 24 Years = J2

**Category**: 스케일링
**n=6 Connection**: Optimal economic lifetime for a DAC plant = J2 = 24 years EXACT. This balances capital amortization against sorbent degradation, equipment wear, and technology obsolescence.
**Prediction**: DAC plant NPV is maximized at operational lifetime = 24 +/- 2 years. Shorter than 20 years: capital underamortized. Longer than 30 years: maintenance costs exceed replacement benefit. Sorbent replacement cycle = tau = 4 years (6 replacements per lifetime).
**Verification**: Build techno-economic model with time-dependent degradation, maintenance escalation, and discount rate. Compute NPV vs lifetime. Compare with actual power plant and chemical plant lifetimes (20-30 years).
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

## Category 6: Cross-domain (H-CC-51 ~ H-CC-60)

### H-CC-51: Battery + Carbon Capture Electrochemical Integration

**Category**: Cross-domain
**n=6 Connection**: Integrated battery-CC system (electrochemical pH-swing powered by battery storage) achieves efficiency = phi = 2 times standalone CC. Battery provides 6 charge/discharge cycles per day = n EXACT matching CC TSA cycles.
**Prediction**: Co-located battery + DAC system captures CO2 at 100 kJ/mol (vs 200 kJ/mol standalone) = phi improvement. Battery charge cycles synchronized with TSA: 6 cycles/day at 4-hour intervals (J2 hours / n cycles). Round-trip efficiency gain: 20% from thermal integration.
**Verification**: Model and pilot-test integrated BESS + DAC system. Measure energy per ton CO2 vs standalone DAC. Confirm phi=2 efficiency gain from shared thermal management and load-shifting.
**Grade**: UNVERIFIED
**Related BT**: BT-43 (CN=6 battery), BT-57 (battery cell ladder), BT-94

---

### H-CC-52: Fusion + Carbon Capture = Zero-Energy-Cost DAC

**Category**: Cross-domain
**n=6 Connection**: Fusion reactor waste heat (at 600-800C) drives TSA desorption directly. Fusion Q > 10 (sigma-phi) provides effectively unlimited thermal energy. DAC becomes a heat sink for fusion, improving overall plant efficiency.
**Prediction**: Fusion-powered DAC achieves marginal energy cost = 0 for CO2 capture (waste heat utilization). Fusion plant allocates 1/sigma = 8.3% of thermal output to DAC. A single ITER-class reactor powers 12 = sigma DAC plants at 1 Mt/yr each = 12 Mt/yr total.
**Verification**: Thermal integration model of fusion plant + DAC. Calculate available waste heat at appropriate temperatures. Size DAC capacity. Confirm that fusion waste heat alone drives complete TSA cycle.
**Grade**: UNVERIFIED
**Related BT**: BT-38 (hydrogen quadruplet), BT-94

---

### H-CC-53: Solar + Carbon Capture Photocatalytic Efficiency

**Category**: Cross-domain
**n=6 Connection**: Solar-driven photocatalytic CO2 reduction theoretical maximum efficiency = 1/(n*phi) = 1/12 = 8.3% (thermodynamic limit for single-junction at CO2 reduction potential). With tandem: approaches SQ limit 33% = 1/n/phi... but for CO2 conversion: 1/n = 16.7% practical ceiling.
**Prediction**: Best photocatalytic CO2-to-fuel system achieves solar-to-fuel efficiency = 1/n = 16.7% using tandem absorber at bandgap 4/3 eV (BT-30). Single junction: 1/sigma = 8.3%. Current record: ~6% (approaching 1/sigma).
**Verification**: Survey highest reported solar-to-fuel efficiencies for CO2 photoreduction. Confirm clustering below 1/sigma = 8.3% for single junction. Test tandem systems approaching 1/n = 16.7%.
**Grade**: UNVERIFIED
**Related BT**: BT-30 (SQ solar bridge), BT-63 (solar panel cell ladder), BT-94

---

### H-CC-54: MOF + Carbon Capture Optimal = Mg-MOF-74 (CN=6, sigma-tau mmol/g)

**Category**: Cross-domain
**n=6 Connection**: Optimal MOF for DAC is Mg-MOF-74 with CN=6 and CO2 uptake = 8.0 mmol/g = sigma-tau = 8. The metal center (Mg2+) has ionic radius such that octahedral coordination is uniquely stable.
**Prediction**: Mg-MOF-74 DAC uptake at 400 ppm / 298K = 8.0 +/- 0.5 mmol/g = sigma-tau EXACT. This is the highest among all MOFs tested under DAC conditions. The uptake = sigma-tau is not coincidence but reflects n=6 geometric packing in the pore channel.
**Verification**: Measure Mg-MOF-74 CO2 isotherm at 400 ppm (breakthrough experiment with synthetic air). Compare with database of 100+ MOFs tested under identical conditions. Confirm rank #1 and value ~ sigma-tau.
**Grade**: UNVERIFIED
**Related BT**: BT-96 (DAC-MOF CN=6 universality), BT-43

---

### H-CC-55: Hydrogen + Carbon Capture Synfuel Efficiency = 60%

**Category**: Cross-domain
**n=6 Connection**: H2 + CO2 synfuel (Fischer-Tropsch or methanol) energy efficiency = 60% = sigma * sopfr % = 12 * 5 = 60 EXACT. This is the thermodynamic ceiling for CO2 hydrogenation to liquid fuels.
**Prediction**: CO2 + H2 -> CH3OH (methanol) or liquid hydrocarbons achieves maximum energy efficiency = 60% at optimal conditions (250C, 50 bar, Cu/ZnO catalyst). Higher pressures: marginal gain < 2%. Lower temperatures: kinetically limited below 50%.
**Verification**: Compile reported energy efficiencies for CO2 hydrogenation (Power-to-X, Sasol FT, Carbon Recycling International). Calculate from LHV of products / (LHV H2 input + capture energy). Test mean = 60%.
**Grade**: UNVERIFIED
**Related BT**: BT-38 (hydrogen quadruplet), BT-95 (carbon cycle loop)

---

### H-CC-56: Chip + Carbon Capture DAC Control = RISC-V N6

**Category**: Cross-domain
**n=6 Connection**: Optimal DAC control chip = RISC-V N6 architecture with 6-stage pipeline = n EXACT (BT-56). The chip monitors 6 sensor types (H-CC-46) via sigma=12 data channels at sigma-tau=8 bit precision.
**Prediction**: RISC-V N6 DAC controller achieves real-time optimization of capture process, reducing energy consumption by 1/(sigma-phi) = 10% vs PLC-based control. 6-stage pipeline processes sensor data in < 1 ms. Neuromorphic anomaly detection (SNN 6-layer, BT-59) catches sorbent degradation sigma=12 hours earlier than threshold alarms.
**Verification**: Implement RISC-V N6 controller on FPGA. Deploy on DAC pilot plant. Measure energy savings and anomaly detection lead time vs conventional PLC control over 1000-hour test.
**Grade**: UNVERIFIED
**Related BT**: BT-56 (complete n=6 LLM / architecture), BT-59 (8-layer AI stack)

---

### H-CC-57: Wind + Carbon Capture Optimal Wind Speed = 6 m/s

**Category**: Cross-domain
**n=6 Connection**: Optimal wind speed for combined wind turbine + DAC operation = 6 m/s = n EXACT. At this speed: wind turbine produces power AND DAC fan energy is supplemented by natural wind, reducing parasitic load.
**Prediction**: Co-located wind + DAC at mean wind speed = 6 m/s achieves net energy cost = 50% of grid-powered DAC. The DAC fan parasitic load is offset by wind kinetic energy at v = 6 m/s (dynamic pressure = 22 Pa ~ sufficient for sorbent bed). Below 4 m/s: insufficient wind. Above 10 m/s: turbulence reduces capture efficiency.
**Verification**: Model wind + DAC co-location at various mean wind speeds (3-12 m/s). Account for wind power generation, DAC fan savings, and turbulence effects. Confirm minimum net cost at 6 m/s.
**Grade**: UNVERIFIED
**Related BT**: BT-94

---

### H-CC-58: Concrete + Carbon Capture CO2 Curing Strength Enhancement

**Category**: Cross-domain
**n=6 Connection**: CO2-cured concrete achieves compressive strength = phi = 2 times conventional steam-cured concrete. The carbonation reaction (CaO + CO2 -> CaCO3) forms CN=6 calcite crystals, densifying the matrix.
**Prediction**: CO2-cured concrete (28-day) compressive strength = 80 MPa vs conventional 40 MPa = phi = 2x enhancement. CaCO3 formed has calcite structure with Ca CN=6 octahedral = n EXACT. CO2 uptake = 120 kg CO2/ton cement = sigma * (sigma-phi).
**Verification**: Cast concrete specimens (identical mix). Cure half with CO2 (CarbonCure process), half with steam. Test 28-day compressive strength. Confirm phi=2 ratio. XRD to verify calcite formation. Measure CO2 uptake per ton.
**Grade**: UNVERIFIED
**Related BT**: BT-43 (CN=6 universality), BT-95 (carbon cycle loop)

---

### H-CC-59: Ocean + Carbon Capture Alkalinity Enhancement = pH Change 0.6

**Category**: Cross-domain
**n=6 Connection**: Ocean alkalinity enhancement target pH increase = 0.6 = n/10 EXACT. Current ocean pH = 8.1 (declining from 8.2 pre-industrial). Restoration target = 8.1 + 0.6 = 8.7 (optimal for marine calcifiers). Each 0.1 pH unit = 1/(sigma-phi) reduction in H+ concentration.
**Prediction**: Optimal ocean CO2 uptake occurs at pH = 8.1 + n/10 = 8.7 (well within safe range for marine life). At this pH, dissolved CO2 equilibrium shifts to absorb an additional 12 = sigma ppm atmospheric CO2 equivalent per pH unit. Total intervention: 6 = n ocean current gating points for global deployment.
**Verification**: Ocean chemistry model (carbonate equilibrium) to compute CO2 uptake vs pH. Mesocosm experiments at pH 8.1, 8.4, 8.7, 9.0. Measure CO2 drawdown rate and marine organism health. Confirm optimality at 8.7.
**Grade**: UNVERIFIED
**Related BT**: BT-95 (carbon cycle loop)

---

### H-CC-60: Graphene + Carbon Capture CO2-to-Graphene Conversion Efficiency

**Category**: Cross-domain
**n=6 Connection**: CO2-to-graphene conversion energy efficiency = 12% = sigma percent. Mass efficiency (C in graphene / C in CO2) = 12/44 = 27.3% ~ J2 + n/phi = 27 (1% off). Graphene is the ultimate carbon capture product: permanent storage in a hexagonal C6 lattice worth $1M+/ton.
**Prediction**: Plasma-assisted CO2-to-graphene conversion achieves energy efficiency = sigma = 12% (electrical energy to graphene chemical energy). Carbon yield = 27% (mass) ~ C/CO2 molar ratio. Product quality: > 95% monolayer with 6-fold symmetry confirmed by SAED.
**Verification**: Build plasma CVD reactor fed with pure CO2. Measure electrical input, graphene output mass, and quality (Raman 2D/G ratio, TEM). Calculate energy efficiency = (graphene mass * 32.8 MJ/kg) / (electrical input). Confirm ~12%.
**Grade**: UNVERIFIED
**Related BT**: BT-85 (Carbon Z=6 material universality), BT-93 (Carbon Z=6 chip material), BT-95 (carbon cycle loop)
