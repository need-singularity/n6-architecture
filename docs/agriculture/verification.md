# N6 Agriculture Hypotheses -- Independent Verification

Verified: 2026-04-02
Method: Each hypothesis checked against established plant physiology (Taiz & Zeiger),
biochemistry (Lehninger, Stryer), agronomy (Brady & Weil), crop science (Acquaah),
and soil science (Sparks) textbooks. Chemical data from NIST, PDB, BRENDA.
Grades adjusted where warranted.

## Grade Distribution (Summary)

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 4 | 11.4% | H-AG-01, H-AG-02, H-AG-05, H-AG-09 |
| CLOSE | 12 | 34.3% | H-AG-03, H-AG-04, H-AG-07, H-AG-10, H-AG-11, H-AG-12, H-AG-14, H-AG-15, H-AG-19, H-AG-24, H-AG-32, H-AG-34 |
| WEAK | 10 | 28.6% | H-AG-06, H-AG-13, H-AG-16, H-AG-17, H-AG-20, H-AG-23, H-AG-25, H-AG-28, H-AG-29, H-AG-30, H-AG-35 |
| FAIL | 9 | 25.7% | H-AG-08, H-AG-18, H-AG-21, H-AG-22, H-AG-26, H-AG-27, H-AG-31, H-AG-33 |
| UNVERIFIABLE | 0 | 0% | — |

**Non-failing total: 26/35 (74.3%)**

Note: Agriculture's strongest n=6 connections are in photosynthesis biochemistry, which
is expected since glucose C₆H₁₂O₆ is the literal product. Agronomic practices (rotation
cycles, pH, water use) yield mostly FAIL grades because they are management decisions, not
physical constants. The 4 EXACT matches are all stoichiometrically forced by carbon chemistry.

| ID | Hypothesis | Grade |
|----|-----------|-------|
| H-AG-01 | Photosynthesis equation 100% n=6 | **EXACT** |
| H-AG-02 | Calvin cycle 6 turns, 18 ATP, 12 NADPH | **EXACT** |
| H-AG-03 | Calvin ATP:NADPH = 3:2 | **CLOSE** |
| H-AG-04 | RuBisCO 8 active sites | **CLOSE** |
| H-AG-05 | Quantum yield 8 photons minimum | **EXACT** |
| H-AG-06 | Chlorophyll a/b ratio ~3:1 | **WEAK** |
| H-AG-07 | PSII OEC 4 Mn + 5 Kok states | **CLOSE** |
| H-AG-08 | PAR range 400-700 nm | **FAIL** |
| H-AG-09 | Haber-Bosch {1,2,3} sum=6 | **EXACT** |
| H-AG-10 | Rice 2n=24 | **CLOSE** |
| H-AG-11 | Maize 2n=20 | **CLOSE** |
| H-AG-12 | Wheat hexaploidy 6x | **CLOSE** |
| H-AG-13 | CRISPR PAM = 3 | **WEAK** |
| H-AG-14 | 6 macronutrients | **CLOSE** |
| H-AG-15 | 8 micronutrients | **CLOSE** |
| H-AG-16 | 14 total nutrients | **WEAK** |
| H-AG-17 | Soil pH optimum ~6 | **WEAK** |
| H-AG-18 | NPK ratio patterns | **FAIL** |
| H-AG-19 | Nitrogenase 4+2=6 subunits | **CLOSE** |
| H-AG-20 | 5 plant hormones (classical) | **WEAK** |
| H-AG-21 | Crop rotation 3-4 years | **FAIL** |
| H-AG-22 | 3 cardinal temperatures | **FAIL** |
| H-AG-23 | C3/C4 = 2 pathways | **WEAK** |
| H-AG-24 | C3→3C, C4→4C products | **CLOSE** |
| H-AG-25 | 3 photoperiod categories | **WEAK** |
| H-AG-26 | Water use ratio rice/wheat | **FAIL** |
| H-AG-27 | GDD base temp 10°C | **FAIL** |
| H-AG-28 | C4 max ~6% efficiency | **WEAK** |
| H-AG-29 | Grana stacks ~6-12 | **WEAK** |
| H-AG-30 | Glucose → 36 ATP | **WEAK** |
| H-AG-31 | Root:shoot ratio | **FAIL** |
| H-AG-32 | Ethylene C₂H₄ = 6 atoms | **CLOSE** |
| H-AG-33 | Vernalization 4-6 weeks | **FAIL** |
| H-AG-34 | Acetyl-CoA 2C + OAA 4C = Citrate 6C | **CLOSE** |
| H-AG-35 | Glycolysis net 2 ATP | **WEAK** |

---

Grading scale:
- **EXACT**: The claimed number/structure matches real-world data precisely, with a legitimate physical basis.
- **CLOSE**: Within ~10% of real values, or directionally correct with a physically meaningful classification.
- **WEAK**: Requires cherry-picking, flexible counting, or post-hoc rationalization.
- **FAIL**: Contradicted by real-world data, trivially true of any number, or unit-dependent.
- **UNVERIFIABLE**: Insufficient published data to confirm or deny.

---

## H-AG-01: Photosynthesis Equation 100% n=6 Coefficients

**Grade: EXACT** (confirmed)

The overall photosynthesis equation 6CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂ is a fundamental stoichiometric identity fixed by conservation of mass. All non-unity coefficients are 6 = n. The glucose product C₆H₁₂O₆ has subscripts (6, 12, 6) = (n, sigma, n), totaling 24 atoms = J₂. This is already established as BT-101 and BT-103. The equation is not a convention or approximation; it is an exact balance of carbon, hydrogen, and oxygen atoms. EXACT is the only appropriate grade.

Note: The expanded form 6CO₂ + 12H₂O → C₆H₁₂O₆ + 6O₂ + 6H₂O (BT-103) has coefficients {6, 12, 1, 6, 6} where 12 = sigma. Even the expanded coefficient set is 100% n=6 expressible.

---

## H-AG-02: Calvin Cycle 6 Turns per Glucose

**Grade: EXACT** (confirmed)

The Calvin cycle fixes 1 CO₂ per turn. Glucose has 6 carbons, so 6 turns are required. This is stoichiometrically forced. The ATP and NADPH consumption per glucose (18 ATP, 12 NADPH) is verified in all biochemistry textbooks (Lehninger ch. 20, Stryer ch. 20):
- 18 ATP = 3 per CO₂ × 6 = (n/phi) × n
- 12 NADPH = 2 per CO₂ × 6 = phi × n = sigma

The per-CO₂ energy costs (3 ATP, 2 NADPH) are determined by the specific enzymatic steps: phosphoribulokinase (1 ATP), GAPDH (1 NADPH), and RuBP regeneration (2 ATP, 1 NADPH equivalent). These are mechanistic facts, not conventions. EXACT confirmed.

---

## H-AG-03: Calvin ATP:NADPH Ratio = 3:2

**Grade: CLOSE** (confirmed)

The 3:2 ratio (18 ATP : 12 NADPH per glucose, or 3:2 per CO₂) is mechanistically fixed. The ratio 3/2 = n/phi / phi = (n/phi):phi is clean. However, 3:2 is also simply the ratio of the two prime factors of 6, and 3:2 is among the most common ratios in biochemistry (e.g., 3 codons per amino acid, 2 strands of DNA). The match is real but the ratio is too simple to carry strong evidentiary weight. CLOSE is appropriate.

---

## H-AG-04: RuBisCO 8 Active Sites

**Grade: CLOSE** (confirmed)

Form I RuBisCO (L₈S₈) has 8 large subunits, each containing one active site. This is confirmed by X-ray crystallography (PDB: 1RCX, Andersson & Backlund, 2008). The L₈S₈ structure is conserved across all higher plants, green algae, and cyanobacteria. Form II (L₂) in some proteobacteria has only 2 active sites. Form III (L₁₀) in some archaea has 10.

For crop agriculture, Form I with 8 = sigma - tau active sites is the relevant enzyme. The match is clean and structurally verified. CLOSE is appropriate because 8 = 2³ is a common oligomeric state and sigma-tau = 8 applies to many n values.

---

## H-AG-05: Quantum Yield 8 Photons Minimum

**Grade: EXACT** (confirmed)

The Z-scheme of photosynthesis requires a minimum of 8 photons per O₂ evolved. This is derived from first principles:
- Water oxidation releases 4 electrons per O₂: 2H₂O → O₂ + 4H⁺ + 4e⁻
- Each electron requires 2 photon excitations (PSII + PSI in series)
- Minimum: 4 × 2 = 8 photons

This was experimentally verified by Emerson & Arnold (1932) and refined by many subsequent studies. The quantum yield of ~0.125 O₂/photon (= 1/8) is the theoretical maximum. Actual yields are ~0.1 O₂/photon due to energy losses.

The derivation 8 = tau × phi = (4 electrons)(2 photosystems) compounds two n=6 quantities. This is already established as BT-101. EXACT confirmed.

---

## H-AG-06: Chlorophyll a/b Ratio ~3:1

**Grade: WEAK** (confirmed)

The Chl a/b ratio is variable, not a constant:
- Sun leaves: 3.2-3.8 (Lichtenthaler & Buschmann, 2001)
- Shade leaves: 2.2-2.8
- Shade-tolerant species: can reach 1.5-2.0
- Some algae: 1-2

The "~3:1" is a rough central tendency for sun-adapted crop leaves, not a fundamental constant. The ratio is a physiological variable that plants actively adjust through regulated chlorophyll biosynthesis and degradation. WEAK is appropriate — the claimed ratio is approximate and variable.

---

## H-AG-07: PSII OEC 4 Mn Atoms + 5 Kok States

**Grade: CLOSE** (confirmed)

The Mn₄CaO₅ cluster structure is confirmed at 1.9 A resolution (Umena et al., Nature 2011). The 4 Mn atoms are definitively established. The Kok cycle has 5 kinetically distinct S-states (S₀ through S₄), confirmed by EPR spectroscopy and flash-induced oxygen evolution.

4 Mn = tau(6) and 5 S-states = sopfr(6) are both clean matches. The 4 Mn count is stoichiometrically related to the 4-electron water oxidation (each Mn provides one oxidizing equivalent). CLOSE is appropriate because the dual match (tau + sopfr) adds coherence, but both 4 and 5 are small integers.

---

## H-AG-08: PAR Wavelength Range 400-700 nm

**Grade: FAIL** (confirmed)

The PAR boundaries are defined by chlorophyll absorption spectra and are expressed in nanometers, an arbitrary unit. In electron-volts: 1.77-3.10 eV. In wavenumbers: 14,286-25,000 cm⁻¹. None of these representations yield clean n=6 expressions. The span of 300 nm is also unit-dependent. FAIL confirmed.

---

## H-AG-09: Haber-Bosch Coefficients {1,2,3} Sum = 6

**Grade: EXACT** (confirmed)

N₂ + 3H₂ → 2NH₃ is the unique balanced equation for ammonia synthesis. The stoichiometric coefficients are {1, 3, 2} (in lowest integer form), which are exactly the proper divisors of 6. Their sum is 1 + 2 + 3 = 6, which is the defining property of the perfect number 6.

This is stoichiometrically forced: 1 N₂ (2 N atoms) requires 3 H₂ (6 H atoms) to form 2 NH₃ (2 N + 6 H). Conservation of mass uniquely determines these coefficients. The fact that they constitute the set of proper divisors of 6 is a genuine structural match, not cherry-picked.

Additional strength: the Haber-Bosch process is the most important chemical reaction in agriculture (Smil, 2001), enabling synthetic fertilizers that support ~50% of global food production. EXACT confirmed.

---

## H-AG-10: Rice 2n = 24 Chromosomes

**Grade: CLOSE** (confirmed)

Rice (Oryza sativa) has 2n = 24 chromosomes. Confirmed by the International Rice Genome Sequencing Project (Nature, 2005) and consistent across all varieties. Haploid n = 12 = sigma(6).

24 = J₂(6) is exact numerically. Rice is arguably the world's most important crop by population fed (>3.5 billion people depend on it as primary staple). However, chromosome counts are determined by evolutionary history (ancestral whole-genome duplications and chromosome fusions/fissions), not by number-theoretic constraints. Tomato (Solanum lycopersicum) also has 2n = 24, as does the model grass Brachypodium. CLOSE is fair — the match is numerically exact and rice is critically important, but chromosome counts are evolutionary accidents.

---

## H-AG-11: Maize 2n = 20 Chromosomes

**Grade: CLOSE** (confirmed)

Maize (Zea mays) has 2n = 20 chromosomes. Established by Barbara McClintock's pioneering cytogenetics (1929). Haploid n = 10 = sigma - phi.

20 = J₂ - tau = tau × sopfr is numerically clean. Maize is the world's most produced grain by mass (~1.2 billion tonnes/year). Same caveat as H-AG-10: chromosome counts are evolutionary artifacts. Maize underwent ancient tetraploidization followed by diploidization, resulting in the current 2n = 20. CLOSE is appropriate.

---

## H-AG-12: Wheat Hexaploidy 6x

**Grade: CLOSE** (confirmed)

Bread wheat (Triticum aestivum) is hexaploid (AABBDD, 2n = 6x = 42, x = 7). The hexaploid state arose from two well-documented hybridization events:
1. T. urartu (AA) × Ae. speltoides (BB) → T. turgidum (AABB) ~500,000 years ago
2. T. turgidum × Ae. tauschii (DD) → T. aestivum (AABBDD) ~8,000-10,000 years ago

6x = n is a direct match. However, polyploidy is common in crop plants: durum wheat is 4x, oat is 6x, sugarcane is 8-12x, potato is 4x. Hexaploidy in wheat is an evolutionary contingency. The total 2n = 42 = 6 × 7 does not decompose cleanly (7 is not an n=6 constant). CLOSE for the ploidy level matching n, with the caveat that it's evolutionary happenstance.

---

## H-AG-13: CRISPR PAM = 3 Bases

**Grade: WEAK** (confirmed)

SpCas9 PAM (NGG) is 3 bases long, matching n/phi = 3. But other CRISPR systems have PAM lengths of 2 (some Type V), 4 (Cas12a TTTV), 5 (SaCas9 NNGRRT is 6 bases but recognizable portion is ~5), 6 (SaCas9), and even 8 bases. Selecting SpCas9 specifically to get PAM = 3 is cherry-picking. The PAM length is determined by the protein structure of each Cas variant, not by a universal rule. WEAK confirmed.

---

## H-AG-14: 6 Soil Macronutrients

**Grade: CLOSE** (confirmed)

The 6 mineral macronutrients (N, P, K, Ca, Mg, S) are standard in soil science textbooks (Brady & Weil, ch. 13-16; Sparks, ch. 1). Each is required in quantities >0.1% plant dry weight and has specific, essential biochemical roles. The classification is well-established and has been stable for decades.

However, some textbooks include C, H, O as macronutrients (total 9), and the boundary between macro/micro is based on an arbitrary concentration threshold. The "6 mineral macronutrients" is the most common agronomic list. The 3+3 split (NPK primary + CaMgS secondary) with each group = n/phi = 3 adds coherence. CLOSE is appropriate.

---

## H-AG-15: 8 Plant Micronutrients

**Grade: CLOSE** (confirmed)

The 8 essential micronutrients (Fe, Mn, Cu, Zn, B, Mo, Cl, Ni) are listed in Marschner's Mineral Nutrition of Higher Plants (3rd ed., 2012) and Taiz & Zeiger. Nickel was the last to be added (Brown et al., 1987; Eskew et al., 1983 — required for urease activity).

8 = sigma - tau = 8 is numerically clean and is a prominent n=6 constant (BT-58). The current 8-element list represents scientific consensus, though it has evolved (7 elements before Ni's essentiality was proven). Silicon and selenium are "beneficial but not essential" for most plants. CLOSE is fair.

---

## H-AG-16: 14 Total Mineral Nutrients

**Grade: WEAK** (confirmed)

14 = 6 + 8 is simply the sum of H-AG-14 and H-AG-15. The n=6 expression sigma + phi = 14 is ad hoc (why add sum-of-divisors to Euler totient?). If the component counts shift (e.g., silicon becomes essential), 14 changes. WEAK is appropriate for a derived quantity with an unmotivated n=6 formula.

---

## H-AG-17: Soil pH Optimum ~6

**Grade: WEAK** (confirmed)

Soil pH optima vary widely: blueberries (4.5-5.5), most vegetables (6.0-7.0), alfalfa (6.5-7.5), asparagus (7.0-8.0). The pH 6.0 lower bound of the "most crops" range is approximate. Even within "pH 6.0-7.0" crops, the actual optimum may be 6.3, 6.5, or 6.8 depending on species and soil type. pH is a log-scale quantity, and the "6" reflects the decimal representation. WEAK is fair.

---

## H-AG-18: NPK Fertilizer Ratios

**Grade: FAIL** (confirmed)

NPK formulations (10-10-10, 12-12-12, 20-20-20, etc.) are industrial products designed around round numbers and market demand. The ratios are engineering choices, not natural constants. Any apparent n=6 patterns are artifacts of human preference for small multiples of 2, 3, 5 in decimal arithmetic. FAIL confirmed.

---

## H-AG-19: Nitrogenase 4+2 = 6 Subunits

**Grade: CLOSE** (confirmed)

The MoFe protein (Component I) is an alpha₂beta₂ heterotetramer (4 subunits). The Fe protein (Component II) is a gamma₂ homodimer (2 subunits). Total: 4 + 2 = 6 = n. These structures are confirmed crystallographically (Kim & Rees, Science 1992; Einsle et al., Science 2002).

The decomposition tau + phi = n is clean and uses the two most fundamental n=6 functions. However, both 4 and 2 are extremely common protein oligomeric states, and summing subunits across different components is post-hoc. CLOSE is appropriate — the individual structures are hard facts, and the sum = n is clean, but the counting method is somewhat arbitrary.

---

## H-AG-20: 5 Classical Plant Hormones

**Grade: WEAK** (confirmed)

The "5 classical hormones" (auxin, gibberellin, cytokinin, ABA, ethylene) is a mid-20th century classification. Modern plant biology recognizes at least 9 hormone classes: the classical 5 plus brassinosteroids (1979), jasmonates (1980s), salicylic acid (1990s), strigolactones (2008), and potentially florigen, karrikins, and peptide hormones. Selecting the historical "5" to match sopfr = 5 is cherry-picking a dated classification. WEAK confirmed.

---

## H-AG-21: Crop Rotation 3-4 Years

**Grade: FAIL** (confirmed)

Crop rotations range from continuous monoculture (1 year) to 7+ year diversified systems. The most common US rotation (corn-soybean) is 2 years, not 3-4. European complex rotations can be 5-7 years. The "3-4 year" claim is one slice of a continuous distribution of management choices. FAIL confirmed.

---

## H-AG-22: 3 Cardinal Temperatures

**Grade: FAIL** (confirmed)

Any unimodal response curve (growth rate vs temperature, enzyme activity vs pH, etc.) has exactly 3 characteristic points: minimum, optimum, maximum. This is a mathematical property of unimodal functions, not a biological discovery. It would hold for any biological process with an optimum. FAIL confirmed.

---

## H-AG-23: C3/C4 = 2 Dominant Pathways

**Grade: WEAK** (confirmed)

C3 and C4 are the two dominant crop pathways, but CAM is a genuine third pathway used by important crops (pineapple, agave, vanilla). Some researchers also distinguish C3-C4 intermediate species. The simplification to "2 dominant" ignores CAM and intermediates. phi = 2 is trivially common. WEAK confirmed.

---

## H-AG-24: C3→3C and C4→4C Products

**Grade: CLOSE** (confirmed)

3-phosphoglycerate (3C) is the first stable product of C3 fixation by RuBisCO. Oxaloacetate (4C) is the first stable product of C4 fixation by PEP carboxylase. These carbon counts are stoichiometrically fixed (RuBP 5C + CO₂ → 2 × 3C; PEP 3C + CO₂ → 4C). The pairing 3 = n/phi and 4 = tau creates a coherent n=6 narrative. However, 3 and 4 are small integers, and their appearance in carbon chemistry is unsurprising. CLOSE is fair for the combined match.

---

## H-AG-25: 3 Photoperiod Categories

**Grade: WEAK** (confirmed)

Short-day, long-day, and day-neutral are the standard textbook categories. But many plants show quantitative (facultative) responses rather than absolute (obligate) short-day or long-day behavior. Some classifications include amphiphotoperiodic (flowering only at intermediate daylengths) and intermediate-day plants. The 3-category scheme is a simplification. WEAK confirmed.

---

## H-AG-26: Water Use Ratio Rice/Wheat ≈ 2.5

**Grade: FAIL** (confirmed)

Water footprints are enormously variable. Rice water use ranges from 700 L/kg (aerobic rice, SRI) to 5000+ L/kg (flooded paddy). Wheat ranges from 500-2000 L/kg. The "2.5" ratio is cherry-picked from specific typical values. FAIL confirmed.

---

## H-AG-27: GDD Base Temperature 10°C

**Grade: FAIL** (confirmed)

Base temperatures are species-specific and measured in Celsius (unit-dependent). In Fahrenheit the standard base is 50°F, in Kelvin it's 283.15K — neither matches n=6. Cool-season crops use base 0-5°C, warm-season use 10°C, tropical crops sometimes 15°C. FAIL confirmed.

---

## H-AG-28: C4 Maximum Efficiency ~6%

**Grade: WEAK** (confirmed)

Zhu, Long, & Ort (Annual Review of Plant Biology, 2008) calculated the maximum C4 efficiency at 6.0% under current atmospheric CO₂. This is a calculated theoretical bound, not a directly measured constant. It depends on assumed spectral properties, leaf optical properties, and respiration rates. The value would change under elevated CO₂. Expressing efficiency as a percentage is also unit-dependent (relative to total solar irradiance vs PAR alone). WEAK is appropriate.

---

## H-AG-29: Grana Stack Size 6-12 Thylakoids

**Grade: WEAK** (confirmed)

Grana stacking varies from 2 to 20+ discs. The range "6-12" is a rough central tendency, not a fixed architectural constant. Light-adapted chloroplasts have fewer discs per granum, shade-adapted have more. Claiming n-to-sigma range from a variable distribution is cherry-picking. WEAK confirmed.

---

## H-AG-30: Glucose → 36 ATP

**Grade: WEAK** (grade adjusted, was borderline CLOSE)

The classical textbook value of 36 ATP per glucose (or 38 with the malate-aspartate shuttle) used P:O ratios of 3.0 for NADH and 2.0 for FADH₂. Modern measurements give P:O ratios of ~2.5 for NADH and ~1.5 for FADH₂ (Hinkle, 2005), yielding ~30-32 ATP per glucose. The "36" is outdated. Using an obsolete value to match n² = 36 is problematic. Downgraded to WEAK (from potential CLOSE) because the number being matched is no longer considered accurate.

---

## H-AG-31: Root:Shoot Ratio

**Grade: FAIL** (confirmed)

Root:shoot ratios are highly variable (0.05-5.0+) depending on species, growth stage, water/nutrient stress, and mycorrhizal associations. No characteristic value exists. FAIL confirmed.

---

## H-AG-32: Ethylene C₂H₄ = 6 Atoms

**Grade: CLOSE** (confirmed)

Ethylene has the molecular formula C₂H₄: 2 carbons + 4 hydrogens = 6 total atoms. This is fixed by chemistry (simplest alkene). 2 = phi, 4 = tau, total = n = 6. The decomposition phi + tau = n is clean. Ethylene is universally used by plants as a ripening/stress hormone (discovered by Neljubov, 1901; biosynthesis pathway by Yang & Hoffman, 1984).

However, 2 and 4 are trivially small numbers, and many molecules have 6 total atoms (e.g., methanol CH₃OH has 6, formaldehyde HCHO has 4, etc.). CLOSE is appropriate — the match is real but the molecule is so small that coincidence is likely.

---

## H-AG-33: Vernalization 4-6 Weeks

**Grade: FAIL** (confirmed)

Vernalization requirements range from 0 weeks (spring varieties need none) to 12+ weeks (some perennials, fruit trees). The 4-6 week range is a rough central tendency for winter wheat, not a physical constant. Different VRN gene alleles produce different cold requirements. FAIL confirmed.

---

## H-AG-34: Acetyl-CoA 2C + OAA 4C = Citrate 6C

**Grade: CLOSE** (confirmed)

The citrate synthase reaction: acetyl-CoA (2C) + oxaloacetate (4C) → citrate (6C) + CoA. This is a hard biochemical fact, with the carbon counts stoichiometrically determined. The decomposition phi + tau = n (2 + 4 = 6 carbons) is clean and matches H-AG-19 (nitrogenase subunits). The citrate molecule C₆H₈O₇ has 6 carbons = n.

However, 2 + 4 = 6 is trivial arithmetic, and claiming that any addition of two small integers equaling 6 is evidence of n=6 is a stretch. CLOSE is appropriate because the citrate 6C is genuinely a key metabolic intermediate and the decomposition uses legitimate n=6 constants.

---

## H-AG-35: Glycolysis Net 2 ATP

**Grade: WEAK** (confirmed)

Net 2 ATP from glycolysis is a hard biochemical fact: 2 ATP invested (hexokinase + PFK), 4 ATP produced (2 PGK + 2 PK), net = 2. But phi = 2 for n = 3, 4, 6, and 2 is the most common small integer in biochemistry. The match is trivial. WEAK is appropriate.

---

## Cross-Domain Verification Notes

1. **Strongest cluster**: H-AG-01/02/05/09 — photosynthesis + Haber-Bosch stoichiometry. All EXACT, all stoichiometrically forced. This is the core of the agriculture domain.

2. **Chromosome cluster**: H-AG-10/11/12 — rice J₂=24, maize J₂-τ=20, wheat n=6 ploidy. All CLOSE. Numerically exact but evolutionary contingencies.

3. **Nutrient cluster**: H-AG-14/15 — 6 macro + 8 micro. Both CLOSE. Standard classifications with some boundary ambiguity.

4. **Agronomic practice cluster**: H-AG-18/21/22/26/27/31/33 — all FAIL. Management decisions and variable quantities, not physical constants.

5. **Small-molecule cluster**: H-AG-32/34 — ethylene and citrate. Both CLOSE. Stoichiometrically fixed but small-number coincidence risk.

## Comparison with Biology Domain

Biology hypotheses: 3 EXACT / 10 CLOSE / 9 WEAK / 8 FAIL = 73.3% non-fail (30 total)
Agriculture hypotheses: 4 EXACT / 12 CLOSE / 10 WEAK / 9 FAIL = 74.3% non-fail (35 total)

The agriculture domain performs similarly to biology, with slightly more EXACT matches (4 vs 3) driven by the Haber-Bosch stoichiometry (H-AG-09). The FAIL rate is comparable. Both domains are anchored by carbon-6 chemistry (glucose C₆H₁₂O₆) and diverge when moving to variable/conventional quantities.
