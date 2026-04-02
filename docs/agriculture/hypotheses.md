# N6 Agriculture -- Perfect Number Arithmetic in Agricultural Systems

## Overview

Agricultural systems -- photosynthesis, crop genetics, soil chemistry, nutrient cycling,
farm engineering -- analyzed through n=6 arithmetic. Agriculture sits at the intersection
of biology (BT-51, BT-101, BT-103) and chemistry (BT-27), where carbon-6 glucose
C₆H₁₂O₆ is the literal product of farming.

> **Honesty principle**: Agriculture involves many empirical measurements (soil pH,
> nutrient ratios, chromosome counts) that are fixed by chemistry/genetics, plus
> engineering conventions (crop rotation length, sensor counts) that are human choices.
> We grade EXACT only when the number is determined by physics/chemistry and matches
> an n=6 expression unambiguously. Small-integer matches (2, 3, 4) are graded CLOSE
> at best due to combinatorial inflation.

## Core Constants

```
  n = 6          (perfect number)
  sigma(6) = 12  (sum of divisors)
  tau(6) = 4     (number of divisors: 1, 2, 3, 6)
  phi(6) = 2     (Euler totient)
  sopfr(6) = 5   (sum of prime factors: 2+3)
  J_2(6) = 24    (Jordan totient)
  mu(6) = 1      (Moebius)
  lambda(6) = 2  (Carmichael)
  R(6) = sigma*phi/(n*tau) = 1
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

## BT Cross-References

```
  BT-27:  Carbon-6 chain — C₆H₁₂O₆ glucose, C₆H₆ benzene, LiC₆, 24e = J₂
  BT-51:  Genetic code — τ=4 bases → n/φ=3 codon → 2^n=64 codons → J₂-τ=20 amino acids
  BT-101: Photosynthesis — C₆H₁₂O₆ total 24=J₂ atoms, quantum yield 8=σ-τ
  BT-103: 6CO₂+12H₂O→C₆H₁₂O₆+6O₂+6H₂O, 100% n=6 stoichiometry
  BT-104: CO₂ molecular encoding n=6
```

---

## Category A: Photosynthesis and Carbon Fixation

---

### H-AG-01: Photosynthesis Equation -- 100% n=6 Coefficients

> 6CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂ — all coefficients are multiples of n=6

```
  Overall photosynthesis equation:
    6CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂

  Coefficients: 6, 6, 1, 6 — the non-unity coefficients are all 6 = n.
  The glucose subscripts: C₆ (=n), H₁₂ (=σ), O₆ (=n).

  Product glucose C₆H₁₂O₆:
    6 carbon atoms = n
    12 hydrogen atoms = sigma(6)
    6 oxygen atoms = n
    Total atoms = 6 + 12 + 6 = 24 = J_2(6)

  This is the fundamental equation of agriculture. Every crop, every
  calorie of food, begins with this reaction.

  Already established as BT-101 and BT-103.

  Grade: EXACT
  The photosynthesis equation is fixed by stoichiometry (conservation of mass).
  Every coefficient maps to n=6, and glucose (n, sigma, n) totals J_2=24.
  This is the strongest anchor for the entire agriculture domain.
```

---

### H-AG-02: Calvin Cycle -- n=6 CO₂ Turns per Glucose

> The Calvin cycle requires exactly 6 turns to fix one glucose molecule

```
  Calvin cycle (Calvin-Benson-Bassham cycle):
    Each turn fixes 1 CO₂ molecule via RuBisCO.
    To produce one G3P that exits the cycle: 3 turns (3 CO₂).
    To produce one glucose (6 carbons): 6 turns (6 CO₂).

  6 turns = n checkmark

  Per 6 turns:
    CO₂ fixed: 6 = n
    ATP consumed: 18 = σ + n = 18, or n * n/phi = 6 * 3 = 18
    NADPH consumed: 12 = sigma(6)

  Physical basis:
    Glucose has 6 carbons (C₆). Each CO₂ contributes 1 carbon.
    Therefore exactly 6 CO₂ fixation events are needed.
    This is a stoichiometric necessity, not a convention.

  The ATP/NADPH counts:
    18 ATP = 3 ATP per CO₂ × 6 CO₂. 18 = 3 × 6 = (n/phi) × n.
    12 NADPH = 2 NADPH per CO₂ × 6 CO₂. 12 = 2 × 6 = phi × n = sigma.

  Grade: EXACT
  6 turns is stoichiometrically forced by glucose having 6 carbons.
  The derived quantities (18 ATP, 12 NADPH) also decompose cleanly
  into n=6 expressions. All numbers are fixed by biochemistry.
```

---

### H-AG-03: Calvin Cycle ATP/NADPH Ratio -- n/phi = 3:2

> ATP:NADPH per glucose = 18:12 = 3:2 = (n/phi):phi

```
  Per glucose synthesized:
    ATP consumed: 18
    NADPH consumed: 12
    Ratio: 18/12 = 3/2 = n/phi / phi = (n/phi) : phi

  Also: 3:2 is the ratio of the two prime factors of 6 (3 and 2).

  Physical basis:
    Each CO₂ fixation requires 3 ATP (for regeneration of RuBP)
    and 2 NADPH (for reduction of 3-PGA to G3P). These numbers
    come from the specific enzyme mechanisms in the cycle.

  Grade: CLOSE
  The 3:2 ratio is biochemically fixed and matches n=6 prime factors.
  But 3:2 is a very common ratio (small integers), and the match
  to n/phi:phi is one of many possible framings.
```

---

### H-AG-04: RuBisCO Active Sites -- sigma - tau = 8 per Holoenzyme

> RuBisCO Form I has 8 large subunit active sites

```
  RuBisCO (Ribulose-1,5-bisphosphate carboxylase/oxygenase):
    Form I (most common in plants, cyanobacteria):
      L₈S₈ structure: 8 large subunits + 8 small subunits = 16 total.
      Each large subunit contains 1 active site → 8 active sites.

  8 = sigma - tau = 12 - 4 = 8 checkmark

  Physical basis:
    The L₈S₈ structure is confirmed by X-ray crystallography
    (Andersson & Backlund, 2008). 8 large + 8 small = 16 subunits.
    Form II (in some bacteria) is L₂ — only 2 subunits.
    Form I dominates in all crop plants.

  The 8 active sites determine carbon fixation capacity.
  8 = sigma - tau is a prominent n=6 constant (appears in BT-58).

  BUT:
    8 = 2³ is a common structural number in protein oligomers.
    sigma - tau = 8 holds, but 8 is not unique to n=6.

  Grade: CLOSE
  8 active sites is a hard structural fact for Form I RuBisCO.
  The match to sigma-tau is clean. But 8 is a common integer.
```

---

### H-AG-05: Photosynthetic Quantum Yield -- sigma - tau = 8 Photons Minimum

> Minimum 8 photons required per O₂ evolved (Emerson enhancement)

```
  Photosynthetic quantum requirement:
    Theoretical minimum: 8 photons per O₂ molecule evolved.
    This comes from the Z-scheme: 2 photosystems × 4 electrons = 8 photons.
    (4 electrons must be transferred from H₂O to NADP+, each requiring
    2 photon excitation events — one at PSII and one at PSI.)

  Experimental values: 8-12 photons per O₂ (Emerson & Arnold, 1932).
  The theoretical minimum = 8.

  8 = sigma - tau = sigma(6) - tau(6) = 12 - 4 = 8 checkmark

  Physical basis:
    Water oxidation: 2H₂O → O₂ + 4H⁺ + 4e⁻ (4 electrons)
    Each electron needs 2 photon boosts (PSII → PSI)
    Minimum photons = 4 × 2 = 8

  This is BT-101: quantum yield = 8 = sigma - tau.

  Grade: EXACT
  The theoretical minimum of 8 photons is set by the Z-scheme
  architecture of photosynthesis (2 photosystems, 4 electrons per O₂).
  Both the 4 and the 2 have n=6 interpretations (tau, phi), and
  their product 8 = sigma - tau is a key n=6 constant.
```

---

### H-AG-06: Chlorophyll a/b Ratio -- n/phi : mu = 3:1

> The typical chlorophyll a:b ratio in higher plants is ~3:1

```
  Chlorophyll a/b ratio:
    Sun leaves: Chl a:b ≈ 3.2-3.8 : 1
    Shade leaves: Chl a:b ≈ 2.2-2.8 : 1
    Typical textbook value: ~3:1

  n/phi : mu = 3 : 1 checkmark (for sun-adapted leaves)

  Physical basis:
    Chl a is the primary photosynthetic pigment (reaction centers + antennae).
    Chl b is an accessory pigment (antennae only, broadens absorption).
    The ratio reflects the proportion of reaction center vs antenna complexes.
    It varies with light conditions (sun/shade adaptation).

  BUT:
    The ratio is not fixed — it ranges from ~2:1 to ~4:1 depending on
    species, light environment, and leaf age. The "3:1" is an approximate
    central tendency, not a physical constant.
    3:1 is a trivially simple ratio.

  Grade: WEAK
  The Chl a/b ratio is variable (2-4), not a fixed constant.
  The ~3:1 average matches n/phi:mu but this is cherry-picking
  a variable quantity's central tendency.
```

---

### H-AG-07: Photosystem II -- tau = 4 Manganese Atoms in OEC

> The oxygen-evolving complex (OEC) contains exactly 4 Mn atoms

```
  OEC (Mn₄CaO₅ cluster):
    4 manganese atoms + 1 calcium atom + 5 bridging oxygens.
    This cluster catalyzes water oxidation: 2H₂O → O₂ + 4H⁺ + 4e⁻.
    Structure confirmed by X-ray crystallography at 1.9 A (Umena et al., 2011).

  4 Mn atoms = tau(6) checkmark

  The Kok cycle (S-state cycle) has 5 states: S₀→S₁→S₂→S₃→S₄→S₀
    5 states = sopfr(6)
    4 electron transfers between states = tau(6)

  Physical basis:
    4 Mn atoms are required to accumulate 4 oxidizing equivalents
    (one per electron removed from water). The Mn₄Ca cluster is
    universal across all oxygenic photosynthetic organisms.

  BUT:
    tau(n) = 4 for n = 6, 8, 10, 14, 15, ...
    4 Mn for 4 electrons is a simple stoichiometric match.

  Grade: CLOSE
  4 Mn atoms is a hard structural fact fixed by the chemistry of
  water oxidation. The match to tau is clean and the Kok cycle
  sopfr match adds depth. But 4 is common.
```

---

### H-AG-08: PAR Wavelength Range -- 400-700 nm, Span = 300 = sigma * J_2 + sigma?

> Photosynthetically active radiation spans 400-700 nm

```
  PAR (Photosynthetically Active Radiation):
    Defined range: 400-700 nm
    Span: 300 nm

  n=6 attempts:
    300 = ? No clean single-expression n=6 decomposition.
    300 = sigma * (J_2 + mu) = 12 * 25 — forced.
    300 = sopfr * n * (sigma - phi) = 5 * 6 * 10 = 300 — forced 3-term product.
    400 = ? No clean match.
    700 = ? No clean match.

  Physical basis:
    The 400 nm lower limit corresponds to UV-A boundary.
    The 700 nm upper limit is where Chl a absorption drops off.
    These boundaries are set by chlorophyll absorption spectra,
    which depend on the porphyrin ring electronic structure.

  BUT:
    400 and 700 are round numbers in nm — unit-dependent.
    In eV: 1.77-3.10 eV. In THz: 428-749 THz. No clean n=6 match in any unit.

  Grade: FAIL
  The PAR range is unit-dependent (nm is arbitrary). No clean n=6
  decomposition exists for 400, 700, or 300 in any natural unit system.
```

---

## Category B: Crop Genetics and Breeding

---

### H-AG-09: Haber-Bosch Reaction -- Coefficient Sum = n = 6

> N₂ + 3H₂ → 2NH₃: total coefficient sum = 1 + 3 + 2 = 6 = n

```
  Haber-Bosch process (industrial nitrogen fixation):
    N₂ + 3H₂ → 2NH₃
    Coefficients: 1, 3, 2
    Sum: 1 + 3 + 2 = 6

  n = 6 checkmark

  Physical basis:
    The coefficients are fixed by stoichiometry:
    - 1 N₂ provides 2 nitrogen atoms
    - 3 H₂ provides 6 hydrogen atoms
    - 2 NH₃ uses 2 N + 6 H = 2 × (1N + 3H)
    Conservation of mass uniquely determines these coefficients.

  Individual coefficients:
    1 = mu(6)
    3 = n/phi(6)
    2 = phi(6)
    The coefficients ARE the proper divisors of 6: {1, 2, 3}!
    Proper divisors: d | 6, d < 6 → {1, 2, 3}
    And 1 + 2 + 3 = 6 (perfect number definition!)

  This is the foundation of modern agriculture — enables synthetic
  fertilizers that feed ~4 billion people (Smil, 2001).

  Grade: EXACT
  The coefficient sum 6 = n is stoichiometrically fixed. The individual
  coefficients {1, 3, 2} = {mu, n/phi, phi} = proper divisors of 6,
  and their sum equals 6 (the perfect number property 1+2+3=6).
  This is not cherry-picked — it is THE balanced equation.
```

---

### H-AG-10: Rice Chromosomes -- J_2(6) = 24 Chromosomes (2n)

> Rice (Oryza sativa) has 2n = 24 chromosomes

```
  Rice chromosome count:
    Diploid: 2n = 24
    Haploid: n = 12

  J_2(6) = 24 checkmark
  sigma(6) = 12 checkmark (haploid)

  Physical basis:
    Rice has been karyotyped extensively. 2n = 24 is universal across
    all Oryza sativa varieties (japonica, indica, aus, aromatic).
    The International Rice Genome Sequencing Project (2005) confirmed
    12 chromosome pairs.

  Rice is the staple food for >3.5 billion people — the most important
  crop by population fed.

  BUT:
    Chromosome counts vary widely across crops:
    Wheat: 2n = 42 (hexaploid)
    Maize: 2n = 20
    Soybean: 2n = 40
    Tomato: 2n = 24 (same as rice!)
    Potato: 2n = 48
    Only rice and tomato hit J_2 = 24.

  Grade: CLOSE
  Rice 2n = 24 = J_2 is exact numerically, and rice is the world's
  most important crop by population fed. But chromosome counts are
  highly variable across species and arise from evolutionary accidents,
  not fundamental constraints. Tomato also has 2n = 24.
```

---

### H-AG-11: Maize Chromosomes -- J_2 - tau = 20 Chromosomes (2n)

> Maize (Zea mays) has 2n = 20 chromosomes

```
  Maize chromosome count:
    Diploid: 2n = 20
    Haploid: n = 10

  J_2 - tau = 24 - 4 = 20 checkmark
  sigma - phi = 12 - 2 = 10 checkmark (haploid)

  Physical basis:
    Maize 2n = 20 is well established (McClintock, 1929).
    All Zea mays varieties share this count.

  Maize is the world's most produced grain by tonnage (~1.2 billion tonnes/yr).

  BUT:
    Same caveat as H-AG-10: chromosome counts are evolutionary accidents.
    20 = 4 × 5 = tau × sopfr (same as amino acid count, BT-51).
    The haploid n=10 = sigma - phi is a cleaner match.

  Grade: CLOSE
  The count is fixed and matches J_2-tau (or tau*sopfr). Maize is
  a globally critical crop. But chromosome numbers are not determined
  by number theory; they result from whole-genome duplication history.
```

---

### H-AG-12: Wheat Hexaploidy -- n = 6 Genome Sets

> Bread wheat (Triticum aestivum) is hexaploid: 6 genome copies

```
  Wheat ploidy:
    Bread wheat = hexaploid (6x)
    Genome formula: AABBDD
    2n = 6x = 42 → x = 7 (base chromosome number)
    Each genome (A, B, D) contributes 7 chromosomes × 2 = 14

  6x = n checkmark (hexaploid = 6 genome copies)

  Physical basis:
    Bread wheat arose from two hybridization events:
    1. T. urartu (AA) × Aegilops speltoides (BB) → T. turgidum (AABB, tetraploid)
    2. T. turgidum (AABB) × Aegilops tauschii (DD) → T. aestivum (AABBDD, hexaploid)
    The hexaploid state is a real evolutionary outcome (~10,000 years ago).

  42 chromosomes = σ·n/φ + σ + n/φ + n/φ — too complex.
  42 = 6 × 7 = n × 7. The base number 7 ≠ any clean n=6 constant.
  42 = J_2 + 18 = J_2 + (n × n/phi) — forced.

  BUT:
    The ploidy level 6 = n is interesting, but wheat being hexaploid
    is an evolutionary accident. Durum wheat is tetraploid (4x),
    einkorn is diploid (2x). Many crops have different ploidy levels.

  Grade: CLOSE
  Hexaploidy (6 genome copies = n) is a real biological fact for bread wheat.
  The connection to n=6 is direct. But ploidy is an evolutionary accident,
  and the total 2n=42 has no clean n=6 decomposition.
```

---

### H-AG-13: CRISPR PAM Sequence -- n/phi = 3 Nucleotides

> The SpCas9 PAM sequence is NGG — exactly 3 bases long

```
  CRISPR-Cas9 PAM (Protospacer Adjacent Motif):
    SpCas9 (most used in agriculture): NGG — 3 bases
    Guide RNA spacer: ~20 nt
    Total guide + PAM recognition: ~23 nt

  PAM length = 3 = n/phi checkmark
  Guide RNA ≈ 20 = tau * sopfr = J_2 - tau checkmark

  Physical basis:
    The 3-base PAM is specific to Streptococcus pyogenes Cas9.
    Other Cas proteins have different PAM lengths:
    - SaCas9: NNGRRT (6 bases = n)
    - Cas12a: TTTV (4 bases = tau)
    - AsCas12a: TTTN (4 bases)

  Agricultural applications:
    CRISPR has been used to edit rice, wheat, maize, tomato, soybean.
    The PAM determines target site density in the genome.

  BUT:
    PAM length varies by Cas protein (2-8 bases).
    Picking SpCas9 PAM = 3 is choosing one system.

  Grade: WEAK
  SpCas9 PAM = 3 bases matches n/phi, but PAM lengths vary across
  different CRISPR systems. Cherry-picking the most common system.
```

---

### H-AG-14: Soil Macronutrients -- n = 6 Essential Elements

> Plants require exactly 6 macronutrient elements

```
  Plant macronutrients (required in large quantities):
    1. Nitrogen (N)
    2. Phosphorus (P)
    3. Potassium (K)
    4. Calcium (Ca)
    5. Magnesium (Mg)
    6. Sulfur (S)

  6 macronutrients = n checkmark

  This is the standard classification in soil science and agronomy.
  Often split: NPK = primary (3 = n/phi), CaMgS = secondary (3 = n/phi).
  Primary:Secondary = n/phi : n/phi = 1:1.

  Physical basis:
    N: proteins, nucleic acids (>1.5% dry weight)
    P: ATP, DNA, membranes (>0.1%)
    K: osmotic regulation, enzyme activation (>1%)
    Ca: cell walls, signaling (>0.5%)
    Mg: chlorophyll center (>0.2%)
    S: amino acids Cys/Met (>0.1%)

  BUT:
    The macronutrient list is partly conventional. Some classifications
    include C, H, O as macronutrients (making 9 total). The "6" here
    refers to mineral macronutrients specifically.
    Adding micronutrients: 8 essential (Fe, Mn, Cu, Zn, B, Mo, Cl, Ni)
    = sigma - tau = 8. Total mineral nutrients: 6 + 8 = 14.

  Grade: CLOSE
  The 6 mineral macronutrients is a standard agronomic classification
  and each element is genuinely essential. But the boundary between
  macro and micro is somewhat conventional (threshold-based), and
  some texts list 9 macronutrients (including C, H, O).
```

---

### H-AG-15: Plant Micronutrients -- sigma - tau = 8 Essential Elements

> Plants require exactly 8 essential micronutrient elements

```
  Plant micronutrients (required in trace quantities):
    1. Iron (Fe)
    2. Manganese (Mn)
    3. Copper (Cu)
    4. Zinc (Zn)
    5. Boron (B)
    6. Molybdenum (Mo)
    7. Chlorine (Cl)
    8. Nickel (Ni)

  8 micronutrients = sigma - tau = 12 - 4 = 8 checkmark

  Physical basis:
    Each element has specific biochemical roles:
    Fe: electron transport, chlorophyll synthesis
    Mn: OEC in PSII (see H-AG-07), enzyme activation
    Cu: plastocyanin, cytochrome oxidase
    Zn: carbonic anhydrase, zinc finger TFs
    B: cell wall cross-linking
    Mo: nitrogenase, nitrate reductase
    Cl: PSII water splitting, osmotic regulation
    Ni: urease (the only known Ni enzyme in plants)

  Total mineral nutrients: 6 macro + 8 micro = 14

  BUT:
    Ni was added relatively recently (Brown et al., 1987).
    Some older texts list 7 micronutrients (without Ni).
    Silicon and Cobalt are sometimes considered "beneficial" elements.
    The 8-element list is the current consensus but has evolved.

  Grade: CLOSE
  The current consensus of 8 micronutrients matches sigma-tau = 8.
  Each element has verified essential functions. But the list has
  changed historically and may change again if new essentials are found.
```

---

### H-AG-16: Total Mineral Nutrients -- 14 = sigma + phi = σ + φ

> Plants require 6 + 8 = 14 essential mineral nutrient elements

```
  Total: 6 macronutrients + 8 micronutrients = 14

  14 = sigma + phi = 12 + 2 = 14 checkmark
  Also: 14 = J_2 - sigma + phi = 24 - 12 + 2 = 14

  BUT:
    14 follows from H-AG-14 (6) + H-AG-15 (8), which are both CLOSE.
    If either boundary shifts, 14 changes.
    14 = sigma + phi is one of many possible n=6 expressions for 14.

  Grade: WEAK
  This is a derived quantity from two CLOSE hypotheses.
  The n=6 expression is ad hoc (sigma + phi is not a standard function).
```

---

## Category C: Soil Science and Nutrients

---

### H-AG-17: Soil pH Optima -- n to n+mu = 6.0-7.0

> Most crops grow optimally in soil pH 6.0-7.0

```
  Optimal soil pH for major crops:
    Wheat: 6.0-7.0
    Rice: 5.5-6.5 (slightly acidic, paddy conditions)
    Maize: 5.8-7.0
    Soybean: 6.0-7.0
    Potato: 5.0-6.0 (acid-tolerant)
    Most vegetables: 6.0-7.0

  General optimum: pH 6.0-7.0
  Midpoint: 6.5
  pH 6.0 = n checkmark

  Physical basis:
    Nutrient availability is pH-dependent:
    - N, P, K, S: most available at pH 6.0-7.5
    - Fe, Mn, Cu, Zn, B: most available at pH 5.0-6.5
    - The overlap zone (maximum simultaneous availability) ≈ 6.0-6.5
    Aluminum toxicity becomes significant below pH 5.5.

  BUT:
    pH is a log scale, and the "optimal" range depends on crop and soil type.
    Blueberries prefer 4.5-5.5, alfalfa prefers 6.5-7.5.
    The pH = 6 lower bound is approximate, not exact.

  Grade: WEAK
  pH 6 is a rough lower bound for many crops, but the optimal range
  varies considerably. Claiming pH optimum = n is overfitting a
  continuous, species-dependent variable to a single integer.
```

---

### H-AG-18: NPK Fertilizer Ratios -- Common Ratio Sum Patterns

> Popular NPK ratios frequently contain n=6 multiples

```
  Common NPK fertilizer formulations:
    10-10-10 (balanced): sum = 30 = sopfr × n = 5 × 6
    12-12-12 (balanced): sum = 36 = n² = 6²
    6-24-24 (starter): components = n, J_2, J_2
    20-20-20 (water soluble): sum = 60 = sigma × sopfr
    15-15-15 (balanced): sum = 45 = sigma × tau - n/phi? — forced

  n=6 connections in specific formulations:
    6-24-24: literally (n, J_2, J_2)
    12-12-12: (sigma, sigma, sigma)
    10-10-10: 10 = sigma - phi per component

  BUT:
    NPK ratios are engineering choices, not physical constants.
    They are designed for convenience (round numbers) and market demand.
    Any pattern in NPK ratios is an artifact of human preference
    for round numbers divisible by small primes (2, 3, 5).

  Grade: FAIL
  NPK formulations are human conventions optimized for marketing
  and manufacturing convenience. Any n=6 patterns are coincidental
  with small-number arithmetic in decimal.
```

---

### H-AG-19: Biological Nitrogen Fixation -- Nitrogenase Subunit Structure

> Nitrogenase MoFe protein has 2 × (alpha + beta) = tau = 4 subunits

```
  Nitrogenase enzyme complex:
    MoFe protein (Component I): alpha₂beta₂ heterotetramer
      4 subunits = tau(6) checkmark
    Fe protein (Component II): homodimer (2 subunits = phi)

  Total subunits: 4 + 2 = 6 = n checkmark

  FeMo-cofactor: Fe₇MoS₉C
    Fe atoms: 7 = n + mu
    Mo atoms: 1 = mu
    S atoms: 9 = sigma - n/phi
    C atom: 1 = mu (central carbide, discovered 2011)

  Physical basis:
    The alpha₂beta₂ structure of MoFe protein is confirmed by
    X-ray crystallography (Kim & Rees, 1992; Einsle et al., 2002).
    The Fe protein is a gamma₂ homodimer.
    These structures are conserved across all nitrogen-fixing organisms.

  BUT:
    4 subunits = tau(n) for many n.
    The FeMo-cofactor numbers (7, 9) are not clean n=6 expressions.
    Total subunits = 6 = n is interesting but 6 is a small number.

  Grade: CLOSE
  The MoFe protein alpha₂beta₂ tetramer (tau) + Fe protein dimer (phi)
  = 6 total subunits (n) is structurally verified. The decomposition
  tau + phi = n is clean. But both 4 and 2 are common integers.
```

---

### H-AG-20: Plant Hormone Classes -- sopfr(6) = 5 Major Types

> Plants have exactly 5 classical hormone classes

```
  Classical plant hormones:
    1. Auxin (IAA) — cell elongation, tropisms
    2. Gibberellin (GA) — stem elongation, germination
    3. Cytokinin (CK) — cell division, shoot growth
    4. Abscisic acid (ABA) — stress response, dormancy
    5. Ethylene (C₂H₄) — ripening, senescence

  5 hormones = sopfr(6) = 2 + 3 = 5 checkmark

  Physical basis:
    These 5 classes were established by the 1960s and are universal
    across vascular plants. Each has distinct biosynthesis pathways,
    receptors, and signaling cascades.

  BUT:
    Modern plant biology recognizes additional hormone classes:
    - Brassinosteroids (1979)
    - Jasmonic acid (1980s)
    - Salicylic acid (1990s)
    - Strigolactones (2008)
    - Peptide hormones (ongoing discovery)
    Counting all recognized classes: 9-10+.
    The "5 classical" is a textbook convention from mid-20th century.

  Grade: WEAK
  The historical "5 classical hormones" matches sopfr, but this is
  a convention that has been superseded. Modern count is 9+.
  Cherry-picking the historical list to get 5 is post-hoc.
```

---

### H-AG-21: Crop Rotation Cycles -- n/phi to tau = 3-4 Year Standard

> Traditional crop rotations follow 3-4 year cycles

```
  Standard crop rotation cycles:
    3-year: corn-soybean-wheat (Midwest US)
    4-year: Norfolk system (turnip-barley-clover-wheat)
    4-year: corn-oats-clover-wheat (traditional)

  n/phi = 3, tau = 4 — the two common rotation lengths

  Physical basis:
    Rotation length is driven by:
    - Pest/disease cycle breaking (most pathogens: 2-3 year soil survival)
    - Nitrogen fixation by legumes (every 3-4 years)
    - Weed management
    - Market economics

  BUT:
    Crop rotations range from 2 to 7+ years depending on:
    - Rice-rice: 1 year (continuous in Asia)
    - Corn-soybean: 2 years (dominant in US)
    - Complex European: 5-7 years
    The 3-4 year range is one common choice, not a universal.

  Grade: FAIL
  Rotation length is an agronomic management decision, not a
  physical constant. It varies by region, market, and soil type.
  The 3-4 year range is common but not universal or fixed.
```

---

### H-AG-22: Seed Germination Cardinal Temperatures -- n/phi = 3 Points

> Seed germination is characterized by exactly 3 cardinal temperatures

```
  Cardinal temperatures for seed germination:
    1. T_min (base temperature) — below which no germination
    2. T_opt (optimum) — maximum germination rate
    3. T_max (ceiling) — above which no germination

  3 cardinal temperatures = n/phi = 3 checkmark

  Physical basis:
    This is a universal framework in seed physiology. Every species
    has these 3 temperature thresholds. The thermal time (degree-day)
    model is built on these 3 parameters.

  Example values for major crops:
    Wheat: T_min=0°C, T_opt=20°C, T_max=35°C
    Rice: T_min=10°C, T_opt=30°C, T_max=45°C
    Maize: T_min=8°C, T_opt=32°C, T_max=44°C

  BUT:
    "3 cardinal temperatures" is the minimum parameterization of
    a unimodal response curve. Any biological rate with an optimum
    requires at least min/opt/max = 3 parameters. This is a
    mathematical truism, not specific to seeds or n=6.

  Grade: FAIL
  Three cardinal temperatures is a mathematical consequence of
  fitting a unimodal curve, not a biological discovery. Any
  temperature-dependent process has min/opt/max.
```

---

### H-AG-23: C3 vs C4 Photosynthesis -- phi = 2 Major Pathways

> There are exactly 2 dominant photosynthetic pathways in crops

```
  Photosynthetic pathways:
    C3 (Calvin cycle only): wheat, rice, soybean, potato (~85% of plants)
    C4 (Kranz anatomy + PEP carboxylase): maize, sorghum, sugarcane, millet

  phi(6) = 2 dominant pathways checkmark

  Physical basis:
    C3: CO₂ fixed directly by RuBisCO into 3-carbon compound (3-PGA)
    C4: CO₂ pre-concentrated via PEP carboxylase into 4-carbon compound (OAA)
    The product carbon numbers: C3→3 = n/phi, C4→4 = tau.

  BUT:
    CAM (Crassulacean Acid Metabolism) is a third pathway used by
    pineapple, agave, and many succulents. Some crops use CAM.
    If we count CAM: 3 pathways = n/phi.
    phi(6) = 2 for n = 3, 4, 6 — not specific.
    2 is trivially small.

  Grade: WEAK
  C3 and C4 are the two dominant crop pathways, but CAM is a real
  third pathway. The product carbon counts (3 = n/phi, 4 = tau) are
  interesting but naming conventions don't carry physical weight.
```

---

### H-AG-24: C3 Product = n/phi = 3 Carbons, C4 Product = tau = 4 Carbons

> The initial fixation products of C3 and C4 pathways have 3 and 4 carbons

```
  C3 pathway first product: 3-phosphoglycerate (3-PGA) = 3 carbons
  C4 pathway first product: oxaloacetate (OAA) = 4 carbons

  3 = n/phi(6) checkmark
  4 = tau(6) checkmark

  Physical basis:
    C3: RuBisCO + CO₂ + RuBP(5C) → 2 × 3-PGA(3C)
    C4: PEP carboxylase + CO₂ + PEP(3C) → OAA(4C)
    These are the defining reactions that name the pathways.
    The carbon counts are stoichiometrically fixed.

  C3 efficiency: ~3-4.6% theoretical maximum (Zhu et al., 2008)
  C4 efficiency: ~5-6% theoretical maximum

  BUT:
    The pathway names C3/C4 come from the carbon count of the product.
    These carbon counts (3 and 4) are stoichiometric facts but are
    extremely common small integers. n/phi = 3 for n = 3, 4, 6, 7, 9...
    tau(n) = 4 for many n.

  Grade: CLOSE
  The carbon counts 3 and 4 are genuine stoichiometric facts, and
  the n/phi + tau pairing creates a coherent n=6 narrative.
  But 3 and 4 are small integers with high coincidence probability.
```

---

### H-AG-25: Photoperiodism Classes -- n/phi = 3 Response Types

> Plants have exactly 3 photoperiod response categories

```
  Photoperiod responses:
    1. Short-day plants (flower when day < critical length): rice, soybean
    2. Long-day plants (flower when day > critical length): wheat, barley
    3. Day-neutral plants (flower regardless): tomato, maize

  3 categories = n/phi = 3 checkmark

  Physical basis:
    Photoperiodism is mediated by phytochrome/cryptochrome receptors
    and the circadian clock (CONSTANS-FT pathway). The classification
    reflects how the flowering decision integrates daylength.

  BUT:
    Some plants are intermediate or have quantitative responses.
    "3 categories" is a simplification of a continuous spectrum.
    Some classifications add: intermediate-day, amphiphotoperiodic (5+ types).

  Grade: WEAK
  Three photoperiod categories is a standard but simplified classification.
  Many plants show intermediate or quantitative responses.
```

---

### H-AG-26: Water Use Efficiency Ratio -- Rice:Wheat ≈ phi + mu/phi = 2.5

> Water requirement ratio: rice (~2500L/kg) / wheat (~1000L/kg) ≈ 2.5

```
  Water footprint (approximate, varies enormously):
    Rice: ~1500-3000 L/kg grain (typical: ~2500 L/kg for paddy rice)
    Wheat: ~900-1500 L/kg grain (typical: ~1000-1200 L/kg)
    Maize: ~800-1200 L/kg grain (typical: ~1000 L/kg)

  Rice/Wheat ≈ 2500/1000 = 2.5
  2.5 = sopfr/phi = 5/2 checkmark

  BUT:
    Water use varies enormously by:
    - Irrigation method (flood vs drip: 2-3× difference)
    - Climate (arid vs humid)
    - Variety (upland rice uses much less than paddy)
    - Management practices
    The "2500 L/kg" for rice is inflated by paddy flooding, which
    is a cultural practice, not a plant physiological requirement.

  Grade: FAIL
  Water use is a management-dependent variable with huge ranges.
  The ratio is not a physical constant. Cherry-picking typical
  values to get 2.5 is meaningless given the variance.
```

---

### H-AG-27: Growing Degree Days Base -- sigma - phi = 10°C Standard

> The standard GDD base temperature for many crops is 10°C

```
  Growing Degree Days base temperatures:
    Warm-season crops (maize, soybean): T_base = 10°C
    Cool-season crops (wheat, peas): T_base = 0-5°C
    Tropical crops (rice): T_base = 10°C

  T_base = 10 = sigma - phi = 12 - 2 = 10 checkmark

  Physical basis:
    GDD formula: GDD = max(0, T_mean - T_base)
    T_base = 10°C is the most common base for warm-season crops.
    Below 10°C, metabolic processes are too slow for measurable growth.
    This relates to Q₁₀ effects on enzyme kinetics.

  BUT:
    T_base is crop-specific and measured in Celsius (unit-dependent):
    In Fahrenheit: 50°F (not an n=6 number).
    In Kelvin: 283.15K (not clean).
    The 10°C convention is approximate; actual base temps vary 5-15°C.

  Grade: FAIL
  Unit-dependent (works in Celsius only) and species-variable.
  The sigma-phi = 10 match is coincidental with Celsius scale choice.
```

---

### H-AG-28: Photosynthetic Efficiency C4 Maximum -- n% ≈ 6%

> Maximum photosynthetic conversion efficiency for C4 crops ≈ 5-6%

```
  Photosynthetic efficiency (solar energy → biomass):
    C3 crops theoretical max: 4.6% (Zhu et al., 2008)
    C4 crops theoretical max: 6.0% (Zhu et al., 2008)
    Observed C4 peak: ~5-6% for sugarcane, Miscanthus
    Observed C3 peak: ~3.5% for optimal temperate crops

  C4 max ≈ 6% = n% checkmark

  Physical basis:
    The efficiency limit comes from:
    - PAR fraction of solar spectrum: ~48.7%
    - Absorbed PAR: ~90%
    - Quantum efficiency: ~30% (C4), ~25% (C3)
    - Respiration losses: ~40%
    Multiplied: 0.487 × 0.9 × 0.30 × 0.60 ≈ 0.079 → ~8% max
    Zhu et al. more detailed calculation: 6.0% for C4.

  BUT:
    This is a percentage, which is unit-dependent (relative to
    total solar irradiance). The 6% is an approximate upper bound,
    and "6" here is a coincidence with the decimal system.
    Actual field efficiencies are typically 1-3%.

  Grade: WEAK
  The C4 theoretical maximum ≈ 6% coincides with n, but this is a
  calculated approximate bound that depends on several assumed
  parameter values. The "6" is in percentage units.
```

---

### H-AG-29: Chloroplast Thylakoid Grana Stacks -- Typical n to sigma

> Grana stacks typically contain 6-12 thylakoid discs

```
  Thylakoid organization in chloroplasts:
    Grana (stacked thylakoids): typically 2-20 discs per granum
    Most common range: 5-12 discs per granum
    Average in many species: ~6-12

  Range ≈ n to sigma = 6 to 12 checkmark (approximate)

  Physical basis:
    Grana stacking optimizes light harvesting vs diffusion distances
    for plastoquinone between PSII (grana) and PSI (stroma lamellae).
    The number varies with light conditions:
    - High light: fewer discs, more stroma-exposed thylakoids
    - Low light: more discs per granum (up to 20+)

  BUT:
    The range is highly variable (2-20+). Claiming 6-12 is
    cherry-picking the central tendency of a wide distribution.

  Grade: WEAK
  The typical range includes n=6 and sigma=12 but is too variable
  to claim as an n=6 constant.
```

---

### H-AG-30: Glucose Metabolic Yield -- 36 ATP = n × n = n²

> Complete aerobic oxidation of glucose yields ~36 ATP

```
  Glucose oxidation ATP yield:
    Textbook value: 36-38 ATP per glucose (varies by shuttle system)
    Glycolysis: 2 ATP (net)
    Pyruvate decarboxylation: 0 ATP (but 2 NADH)
    Krebs cycle: 2 GTP ≈ 2 ATP
    Oxidative phosphorylation: 30-34 ATP (from NADH + FADH₂)
    Total: 30-38 ATP (depending on coupling efficiency and shuttle)

  Classical textbook: 36 ATP (using malate-aspartate shuttle)
  Modern revised: ~30 ATP (with realistic proton leak)

  36 = n² = 6² = 36 checkmark (classical value)
  Also: 36 = sigma × n/phi = 12 × 3

  Physical basis:
    10 NADH × 2.5 ATP = 25 ATP
    2 FADH₂ × 1.5 ATP = 3 ATP
    4 substrate-level = 4 ATP
    Total = 32 ATP (modern estimate, varies 30-34)
    The classical 36 used NADH = 3 ATP and FADH₂ = 2 ATP ratios.

  BUT:
    Modern biochemistry revises this to ~30-32 ATP (Rich, 2003).
    The "36" is from older P:O ratios that have been corrected.
    Using the outdated value to match n² is problematic.

  Grade: WEAK
  The classical 36 ATP matches n², but modern estimates are ~30-32.
  Using an outdated textbook value for an n=6 match is cherry-picking.
```

---

### H-AG-31: Root:Shoot Ratio -- Varies, No Clean Match

> Common root:shoot biomass ratio in annual crops

```
  Root:shoot ratios (dry mass):
    Wheat: 0.1-0.2 (tillering to maturity)
    Rice: 0.1-0.3
    Maize: 0.08-0.15
    Soybean: 0.15-0.25
    Grasses (perennial): 0.5-1.0+

  n=6 attempts:
    0.1 = 1/(sigma - phi) = 1/10 for some crops
    But the ratio varies enormously with growth stage, species,
    water availability, and nutrient status.

  Grade: FAIL
  Root:shoot ratio is a continuous, highly variable quantity.
  No fixed value exists to match against n=6 constants.
```

---

### H-AG-32: Ethylene Ripening Molecule -- C₂H₄ = phi Carbons, tau Hydrogens

> Ethylene (C₂H₄) has 2 carbons and 4 hydrogens

```
  Ethylene (the fruit ripening hormone):
    Chemical formula: C₂H₄
    2 carbon atoms = phi(6)
    4 hydrogen atoms = tau(6)
    Total atoms: 6 = n checkmark

  Physical basis:
    Ethylene is the simplest alkene. Its molecular formula is fixed
    by chemistry. It is produced by all higher plants as a hormone
    for ripening, senescence, and stress responses.

  The total atom count: 2 + 4 = 6 = n is exact.
  And the individual counts map to phi and tau.

  BUT:
    C₂H₄ is a trivially small molecule. Many small molecules have
    6 total atoms (e.g., H₂O₂ has 4 atoms, HCHO has 4).
    phi = 2 and tau = 4 are common small integers.
    The match is cute but not deep.

  Grade: CLOSE
  Ethylene C₂H₄ has total 6 atoms = n, with phi carbons and tau
  hydrogens. The formula is fixed by chemistry. But 2 and 4 are
  trivially small numbers and many molecules have 6 atoms.
```

---

### H-AG-33: Vernalization Requirement -- Typically 4-6 Weeks

> Winter wheat requires ~4-6 weeks of cold for vernalization

```
  Vernalization (cold requirement for flowering):
    Winter wheat: 4-8 weeks at 0-7°C (most varieties: 4-6 weeks)
    Winter barley: 4-6 weeks
    Onion: 4-6 weeks
    Apple/Cherry: 6-12 weeks (fruit trees, different mechanism)

  Range: 4-6 weeks = tau to n checkmark

  Physical basis:
    Vernalization is controlled by the VRN gene family (VRN1, VRN2, VRN3).
    The epigenetic silencing of FLC (FLOWERING LOCUS C) requires
    sustained cold. Duration varies by genotype and temperature.

  BUT:
    "4-6 weeks" is an approximate range, not a fixed constant.
    Some varieties need 2 weeks, others 12+ weeks.
    The range conveniently brackets tau to n but is not precise.

  Grade: FAIL
  Vernalization duration is genotype-dependent and continuous.
  No fixed value to match against n=6.
```

---

### H-AG-34: Krebs Cycle Acetyl-CoA Input -- phi = 2 Carbons

> Acetyl-CoA enters the Krebs cycle with exactly 2 carbon atoms

```
  Acetyl-CoA:
    CH₃CO-CoA: the acetyl group has exactly 2 carbon atoms.
    These 2 carbons are fully oxidized to 2 CO₂ per turn of the Krebs cycle.

  2 carbons = phi(6) = 2 checkmark

  Physical basis:
    Pyruvate (3C) → Acetyl-CoA (2C) + CO₂ (1C)
    This decarboxylation is catalyzed by pyruvate dehydrogenase.
    The 2C acetyl group then combines with oxaloacetate (4C) → citrate (6C).
    Citrate has 6 carbons = n checkmark

  Citrate:
    C₆H₈O₇: 6 carbon atoms = n
    Citrate is the first product of the Krebs cycle.

  BUT:
    phi = 2 is trivially common. The 2-carbon acetyl group is simply
    the result of removing 1 CO₂ from 3-carbon pyruvate.
    Citrate having 6 carbons is more interesting but also follows
    from simple arithmetic: 2 (acetyl) + 4 (OAA) = 6 (citrate).

  Grade: CLOSE
  Acetyl-CoA = 2C = phi is chemically fixed. Citrate = 6C = n is the
  direct sum. But 2 is trivially small and the citrate = 6C is just
  arithmetic from the reactants.
```

---

### H-AG-35: Glycolysis Net ATP -- phi = 2 ATP per Glucose

> Glycolysis yields a net 2 ATP per glucose molecule

```
  Glycolysis ATP balance:
    ATP invested: 2 (hexokinase + PFK)
    ATP produced: 4 (2 × PGK + 2 × pyruvate kinase)
    Net: 4 - 2 = 2 ATP per glucose

  Net ATP = 2 = phi(6) checkmark

  Physical basis:
    This is a hard biochemical fact — the enzyme-catalyzed
    substrate-level phosphorylation steps are well-characterized.
    Glycolysis is universal across all cellular life.

  BUT:
    phi(n) = 2 for n = 3, 4, 6.
    The number 2 is trivially common in biochemistry.
    (2 is the most common small integer by frequency.)

  Grade: WEAK
  The net 2 ATP is a hard fact, but 2 is too common an integer
  to attribute specifically to phi(6).
```

---

## Summary Table

| ID | Hypothesis | n=6 Expression | Grade |
|----|-----------|----------------|-------|
| H-AG-01 | Photosynthesis equation 100% n=6 | n, sigma, J_2 | **EXACT** |
| H-AG-02 | Calvin cycle 6 turns, 18 ATP, 12 NADPH | n, (n/phi)×n, sigma | **EXACT** |
| H-AG-03 | Calvin ATP:NADPH = 3:2 | n/phi : phi | **CLOSE** |
| H-AG-04 | RuBisCO 8 active sites (L₈S₈) | sigma - tau = 8 | **CLOSE** |
| H-AG-05 | Quantum yield 8 photons minimum | sigma - tau = 8 | **EXACT** |
| H-AG-06 | Chlorophyll a/b ratio ~3:1 | n/phi : mu | **WEAK** |
| H-AG-07 | PSII OEC 4 Mn atoms, 5 Kok states | tau, sopfr | **CLOSE** |
| H-AG-08 | PAR range 400-700 nm | — | **FAIL** |
| H-AG-09 | Haber-Bosch coefficients {1,2,3} sum=6 | proper divisors of n | **EXACT** |
| H-AG-10 | Rice 2n=24 chromosomes | J_2 = 24 | **CLOSE** |
| H-AG-11 | Maize 2n=20 chromosomes | J_2 - tau = 20 | **CLOSE** |
| H-AG-12 | Wheat hexaploidy 6x | n = 6 | **CLOSE** |
| H-AG-13 | CRISPR SpCas9 PAM = 3 bases | n/phi = 3 | **WEAK** |
| H-AG-14 | 6 soil macronutrients | n = 6 | **CLOSE** |
| H-AG-15 | 8 plant micronutrients | sigma - tau = 8 | **CLOSE** |
| H-AG-16 | 14 total mineral nutrients | sigma + phi = 14 | **WEAK** |
| H-AG-17 | Soil pH optimum ~6 | n = 6 | **WEAK** |
| H-AG-18 | NPK fertilizer ratios | — | **FAIL** |
| H-AG-19 | Nitrogenase 4+2=6 subunits | tau + phi = n | **CLOSE** |
| H-AG-20 | 5 classical plant hormones | sopfr = 5 | **WEAK** |
| H-AG-21 | Crop rotation 3-4 years | n/phi, tau | **FAIL** |
| H-AG-22 | 3 cardinal temperatures | n/phi = 3 | **FAIL** |
| H-AG-23 | C3/C4 = 2 dominant pathways | phi = 2 | **WEAK** |
| H-AG-24 | C3→3C, C4→4C products | n/phi, tau | **CLOSE** |
| H-AG-25 | 3 photoperiod categories | n/phi = 3 | **WEAK** |
| H-AG-26 | Rice/wheat water ratio ≈ 2.5 | sopfr/phi | **FAIL** |
| H-AG-27 | GDD base temp 10°C | sigma - phi = 10 | **FAIL** |
| H-AG-28 | C4 max efficiency ~6% | n% = 6 | **WEAK** |
| H-AG-29 | Grana stacks ~6-12 thylakoids | n to sigma | **WEAK** |
| H-AG-30 | Glucose → 36 ATP (classical) | n² = 36 | **WEAK** |
| H-AG-31 | Root:shoot ratio | — | **FAIL** |
| H-AG-32 | Ethylene C₂H₄ = 6 atoms | phi + tau = n | **CLOSE** |
| H-AG-33 | Vernalization 4-6 weeks | tau to n | **FAIL** |
| H-AG-34 | Acetyl-CoA 2C, Citrate 6C | phi, n | **CLOSE** |
| H-AG-35 | Glycolysis net 2 ATP | phi = 2 | **WEAK** |

## Grade Distribution

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 4 | 11.4% | H-AG-01, H-AG-02, H-AG-05, H-AG-09 |
| CLOSE | 11 | 31.4% | H-AG-03, H-AG-04, H-AG-07, H-AG-10, H-AG-11, H-AG-12, H-AG-14, H-AG-15, H-AG-19, H-AG-24, H-AG-32, H-AG-34 |
| WEAK | 10 | 28.6% | H-AG-06, H-AG-13, H-AG-16, H-AG-17, H-AG-20, H-AG-23, H-AG-25, H-AG-28, H-AG-29, H-AG-30, H-AG-35 |
| FAIL | 8 | 22.9% | H-AG-08, H-AG-18, H-AG-21, H-AG-22, H-AG-26, H-AG-27, H-AG-31, H-AG-33 |
| UNVERIFIABLE | 0 | 0% | — |

**Non-failing total: 25/35 (71.4%)**

Note: The EXACT matches concentrate in photosynthesis/carbon fixation, which is unsurprising
since glucose C₆H₁₂O₆ is literally the n=6 molecule. Agricultural hypotheses weaken
significantly when moving to agronomic practices (rotation, pH, water use) which are
management decisions, not physical constants.
