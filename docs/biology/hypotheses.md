# N6 Biology -- Perfect Number Arithmetic in Biological Systems

## Overview

Biological structures -- genetic code, cell division, neural architecture, ecology --
analyzed through n=6 arithmetic. Biology is rich in discrete counts (codons, amino acids,
cortical layers) that can be tested against number-theoretic functions of 6.

> **Honesty principle**: Biology has enormous combinatorial diversity. Many biological
> counts are convention-dependent (e.g., "how many phases of mitosis?" depends on the
> textbook). We grade EXACT only when the number is fixed by chemistry/physics and
> cannot reasonably be counted differently.

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

---

## Category A: Genetic Code

---

### H-BIO-1: DNA Bases -- tau(6) = 4 nucleotides

> DNA uses exactly 4 nucleotide bases: A, T, G, C

```
  DNA alphabet:
    Adenine (A), Thymine (T), Guanine (G), Cytosine (C)
    4 bases encode all genetic information.

  tau(6) = 4 checkmark

  Physical basis:
    Watson-Crick base pairing: A-T (2 H-bonds), G-C (3 H-bonds).
    The 4-base system is universal across all known DNA-based life.
    Why 4? Theoretical arguments exist (error correction, information
    density) but no proof that 4 is the only viable number.
    Synthetic biology has created 6-base and 8-base systems (Hachimoji DNA).

  BUT:
    4 is a very common small integer. tau(n)=4 for n=6,8,10,14,15,...
    Not specific to n=6.

  Grade: CLOSE
  The count 4 is genuinely fixed by biochemistry (not convention) and matches tau(6).
  But 4 is too common an integer and tau(n)=4 for many n.
```

---

### H-BIO-2: Double Helix -- phi(6) = 2 strands

> DNA consists of exactly 2 antiparallel polynucleotide strands

```
  DNA structure (Watson & Crick, 1953):
    2 strands wound in a right-handed double helix.
    Complementary base pairing holds them together.
    Universal across all cellular life.

  phi(6) = 2 checkmark

  Physical basis:
    The double-stranded structure enables:
    - Semiconservative replication
    - Error correction (redundancy)
    - Strand separation for transcription

  BUT:
    phi(6) = 2 is trivially small. phi(n)=2 for n=3,4,6.
    "2" is the most common number in biology (bilateral symmetry,
    diploid, etc.). Single-stranded DNA viruses exist.

  Grade: CLOSE
  The 2-strand structure is physically fundamental and universal.
  But 2 is the smallest prime and matches too many things.
```

---

### H-BIO-3: Genetic Code -- 64 codons = tau(6)^3

> The genetic code uses 64 = 4^3 codons (triplet code)

```
  Codon space:
    4 bases taken 3 at a time = 4^3 = 64 codons.
    This is the universal genetic code for all known life.

  tau(6)^3 = 4^3 = 64 checkmark

  Physical basis:
    4 bases: fixed by chemistry (see H-BIO-1)
    3-letter codons: minimum needed to encode 20+ amino acids
      (4^1=4, 4^2=16, 4^3=64 >= 20+stops)
    The triplet code is universal -- confirmed by Nirenberg, Khorana (1960s).

  Connection quality:
    64 = tau(6)^3 is clean. But the exponent 3 (codon length) has its
    own information-theoretic justification independent of n=6.
    The factorization 64 = 2^6 also connects: 2=phi(6), 6=n.

  Grade: EXACT
  64 codons is a hard biochemical fact. tau(6)^3 = 64 is exact.
  The codon length 3 = n/phi is also meaningful.
```

---

### H-BIO-4: 20 Standard Amino Acids = J_2(6) - tau(6) = 24 - 4

> Life uses exactly 20 standard amino acids

```
  Amino acid count:
    20 standard amino acids are encoded by the genetic code.
    This is universal across all known life (with rare exceptions
    like selenocysteine = 21st, pyrrolysine = 22nd in some organisms).

  J_2(6) - tau(6) = 24 - 4 = 20 checkmark

  BUT:
    The formula J_2 - tau is ad hoc. Why subtract these particular functions?
    20 also = 4 * 5 = tau(6) * sopfr(6), which is cleaner.
    The "exactly 20" is slightly soft: 22 are known if you count Sec and Pyl.
    Most textbooks say 20 and the genetic code table encodes exactly 20.

  Grade: CLOSE
  20 is a hard number (the standard genetic code), and 20 = tau*sopfr = 4*5
  is a clean factorization. But the J_2-tau formula feels post-hoc, and the
  count has edge cases (21-22 with nonstandard amino acids).
```

---

### H-BIO-5: 3 Stop Codons = n/phi(6) = 3

> The genetic code has exactly 3 stop codons (UAA, UAG, UGA)

```
  Stop codons:
    UAA (ochre), UAG (amber), UGA (opal)
    These signal translation termination.
    Standard in the "universal" genetic code.

  n/phi = 6/2 = 3 checkmark

  BUT:
    Mitochondrial genetic codes vary: some organisms use UGA as Trp,
    some use AGA/AGG as stops. The "3 stop codons" is specific to
    the standard nuclear genetic code, not truly universal.
    n/phi = 3 is also just "3" which is extremely common.

  Grade: CLOSE
  3 stop codons is the standard count and n/phi=3 matches.
  But mitochondrial code variations weaken universality, and 3 is trivially small.
```

---

### H-BIO-6: 5 Nucleotide Bases (A, T, G, C, U) = sopfr(6)

> Life uses 5 distinct nucleotide bases total across DNA and RNA

```
  Nucleotide bases:
    DNA: A, T, G, C (4 bases)
    RNA: A, U, G, C (4 bases, U replaces T)
    Total distinct: A, T, U, G, C = 5

  sopfr(6) = 2 + 3 = 5 checkmark

  Physical basis:
    T (thymine) and U (uracil) differ by one methyl group.
    Both are pyrimidines pairing with A.
    The 5-base set is universal.

  BUT:
    Modified bases exist in abundance (m5C, m6A, pseudouridine, etc.).
    If you count those, the number is much larger (>100).
    The "5" only works if you count the canonical bases strictly.
    Also sopfr(n)=5 for n=6,12,18,20,... -- not unique.

  Grade: CLOSE
  5 canonical bases is a defensible count, and sopfr(6)=5 matches.
  But modified bases complicate things, and 5 is a small number.
```

---

### H-BIO-7: Codon Degeneracy -- 61 sense codons + 3 stops = 64

> 64 - 3 = 61 sense codons encode 20 amino acids; degeneracy ratio ~3

```
  Degeneracy:
    64 codons / 20 amino acids = 3.2 average codons per amino acid.
    Most amino acids have 2, 4, or 6 synonymous codons.
    Leucine, Serine, Arginine: 6 codons each.

  n=6 connections:
    Max degeneracy = 6 codons (Leu, Ser, Arg) = n
    Common degeneracy classes: 1, 2, 3, 4, 6 (divisors of 12, not 6)
    3 amino acids with 6-fold degeneracy

  BUT:
    The degeneracy classes are {1,2,3,4,6} not {1,2,3,6}.
    Isoleucine has 3 codons. Met and Trp have 1 each.
    The pattern does not cleanly map to divisors of 6.

  Grade: WEAK
  Maximum degeneracy 6 = n is real but feels cherry-picked.
  The full degeneracy spectrum {1,2,3,4,6} doesn't match divisors of 6.
```

---

### H-BIO-8: Start Codon = mu(6) = 1

> The genetic code has exactly 1 standard start codon (AUG)

```
  Start codon:
    AUG encodes methionine and serves as the universal start codon.
    (In prokaryotes, GUG and UUG can also initiate, but AUG dominates.)

  mu(6) = 1 checkmark

  BUT:
    mu(6) = 1 is trivially true -- mu(n)=1 for all squarefree n with
    even number of prime factors (6=2*3, two primes).
    "1" matches everything. Having one start codon is unremarkable.
    Also, alternative start codons exist (GUG, UUG, CUG in some contexts).

  Grade: FAIL
  1 is trivially common. Any quantity equal to 1 matches mu(6)=1.
  No explanatory power whatsoever.
```

---

### H-BIO-9: 20 amino acids x 3-letter code = 60 = sigma * sopfr

> The product 20 * 3 = 60 equals sigma(6) * sopfr(6) = 12 * 5

```
  Calculation:
    20 amino acids (standard genetic code)
    3 nucleotides per codon (triplet code)
    Product: 20 * 3 = 60

  sigma(6) * sopfr(6) = 12 * 5 = 60 checkmark

  BUT:
    This combines two independent biological facts.
    The product 20*3 has no standalone biological meaning --
    it's not a physical quantity, just a numeric coincidence.
    60 = 2^2 * 3 * 5 has many factorizations.

  Grade: WEAK
  Numerically correct but the product 20*3 is not a meaningful
  biological quantity. Post-hoc combination of two numbers.
```

---

### H-BIO-10: Watson-Crick Base Pairing -- phi(6) = 2 types

> There are 2 types of base pairs: A-T and G-C (purine-pyrimidine)

```
  Base pair types:
    A-T: 2 hydrogen bonds (adenine-thymine)
    G-C: 3 hydrogen bonds (guanine-cytosine)
    Purines (A,G) pair with pyrimidines (T/U,C): Chargaff's rules.

  phi(6) = 2 checkmark

  BUT:
    Non-Watson-Crick pairs exist (Hoogsteen, wobble).
    The "2 types" is a simplification of the A-T vs G-C dichotomy.
    phi(6) = 2 is the smallest meaningful integer.
    Also: 2 H-bonds (A-T) and 3 H-bonds (G-C), 2+3 = sopfr(6) = 5.
    That's slightly more interesting.

  Grade: WEAK
  2 pair types is real but 2 is trivially small.
  The H-bond count {2,3} summing to sopfr(6) is cute but still just small numbers.
```

---

## Category B: Cell Biology

---

### H-BIO-11: Cell Division -- 6 Phases

> The eukaryotic cell cycle has 6 distinct phases

```
  Cell cycle phases:
    Interphase: G1, S, G2 (3 phases)
    Mitosis: Prophase, Metaphase, Anaphase/Telophase
    (or: Prophase, Prometaphase, Metaphase, Anaphase, Telophase)

  Problem:
    If mitosis = 4 phases (P, M, A, T): total = 3 + 4 = 7
    If mitosis = 5 phases (P, ProM, M, A, T): total = 3 + 5 = 8
    If mitosis = 3 (P, M, A+T combined): total = 3 + 3 = 6
    The "6 phases" requires combining anaphase + telophase.

  n = 6?

  Grade: WEAK
  The count depends entirely on how you lump/split phases.
  Standard textbooks list 4-5 mitotic phases, giving 7-8 total.
  Getting 6 requires non-standard combination.
```

---

### H-BIO-12: ATP Energy -- sigma(6) - sopfr(6) = 7 kcal/mol

> ATP hydrolysis releases ~7.3 kcal/mol under standard conditions

```
  ATP hydrolysis:
    ATP + H2O -> ADP + Pi
    DeltaG_standard = -7.3 kcal/mol (-30.5 kJ/mol)
    In vivo: -12 to -14 kcal/mol (varies with conditions)

  sigma(6) - sopfr(6) = 12 - 5 = 7

  Comparison:
    Standard: 7.3 kcal/mol vs 7 (4% off)
    In vivo: 12-14 kcal/mol -- closer to sigma(6)=12

  BUT:
    The standard free energy is defined at pH 7.0, 25C, 1M concentrations.
    Under physiological conditions the value is ~12 kcal/mol.
    The "7.3" is convention-dependent, not a fundamental constant.
    7 = sigma - sopfr is an arbitrary combination.

  Grade: WEAK
  The standard value ~7.3 is close to 7 but is condition-dependent.
  The in vivo value ~12 = sigma(6) is actually a closer match but
  that's also condition-dependent. Neither is a fundamental constant.
```

---

### H-BIO-13: Histone Core -- Octamer of tau(6)*phi(6) = 8 proteins

> The nucleosome core particle contains 8 histone proteins

```
  Nucleosome structure:
    Core particle: 2 copies each of H2A, H2B, H3, H4
    Total: 8 histone proteins in the octamer
    ~147 bp of DNA wraps around the octamer.

  tau(6) * phi(6) = 4 * 2 = 8 checkmark

  Also:
    4 histone types = tau(6)
    2 copies each = phi(6)
    4 * 2 = 8 is clean decomposition.

  BUT:
    H1 linker histone exists (total types = 5 with linker).
    8 = 2^3 is an extremely common number.
    tau*phi = 8 also works for other n values.

  Grade: CLOSE
  The 4-types-times-2-copies = 8 decomposition is clean and the
  nucleosome octamer is a hard structural fact. But 8 is common
  and the tau*phi product is not unique to n=6.
```

---

### H-BIO-14: DNA Codon Reading Frame = n/phi = 3

> DNA is read in reading frames of 3 nucleotides

```
  Reading frame:
    The ribosome reads mRNA in non-overlapping triplets.
    3 nucleotides = 1 codon = 1 amino acid.
    The triplet code is universal across all known life.

  n/phi = 6/2 = 3 checkmark

  Physical basis:
    Why 3? Information theory: 4^3 = 64 >= 20 amino acids.
    4^2 = 16 < 20, so 3 is the minimum codon length.
    This is a genuine constraint from the 4-base alphabet.

  BUT:
    n/phi = 3 is just "3". n/phi(n) = 3 for n = 6, 9, etc.
    The reason for 3 is information-theoretic (4^3 >= 20), not number-theoretic.

  Grade: CLOSE
  3 is genuinely fixed by the information capacity constraint.
  The match with n/phi is real but 3 is a small number and the
  biological reason for 3 is fully explained without n=6.
```

---

### H-BIO-15: Phospholipid Bilayer -- phi(6) = 2 leaflets

> Cell membranes consist of a phospholipid bilayer (2 layers)

```
  Membrane structure:
    All cell membranes have 2 leaflets (inner and outer).
    Amphipathic phospholipids self-assemble into bilayers.
    This is universal across all cellular life.

  phi(6) = 2 checkmark

  BUT:
    Bilayer formation is a thermodynamic inevitability for amphipathic
    molecules in water. The "2" comes from the geometry of hydrophobic
    tails shielding from water. phi(6) = 2 is trivially small.

  Grade: FAIL
  2-layer membranes are explained by amphipathic chemistry.
  Matching with phi(6)=2 has zero explanatory power.
  Every bilateral structure in nature "matches" phi(6).
```

---

### H-BIO-16: Glucose -- C_6 H_12 O_6 = n, sigma, n subscripts

> Glucose molecular formula C6H12O6

```
  Glucose (C6H12O6):
    Carbon: 6 = n
    Hydrogen: 12 = sigma(6)
    Oxygen: 6 = n

  This is striking: the most important biological sugar has
  atom counts (6, 12, 6) matching (n, sigma, n).

  Physical basis:
    Glucose is a hexose (6-carbon sugar). The formula C_n H_{2n} O_n
    is the general formula for hexoses, where n=6 gives C6H12O6.
    Fructose, galactose, mannose all share the same formula.

  BUT:
    The general formula for aldohexoses is C_n H_{2n} O_n.
    So H = 2C = 2*6 = 12 is automatic once C = 6.
    Why 6 carbons? Hexoses happen to be the most metabolically
    useful, but pentoses (5C) and trioses (3C) are also vital.
    The formula is just "hexose" not specifically n=6.

  Grade: EXACT
  C6H12O6 is a hard chemical formula. The subscripts (6, 12, 6)
  correspond exactly to (n, sigma(6), n). The hydrogen count 12 = 2*6
  follows from the aldohexose formula, so sigma(6) = 2n is guaranteed
  for hexoses. This is EXACT as a numerical fact, though the "why 6
  carbons" question is separate.
```

---

### H-BIO-17: 6 Kingdoms of Life = n

> Traditional classification divides life into 6 kingdoms

```
  Six kingdoms (Woese et al., extended):
    1. Bacteria
    2. Archaea
    3. Protista
    4. Fungi
    5. Plantae
    6. Animalia

  n = 6 checkmark

  BUT:
    This is a human classification convention, not a physical law.
    Earlier: 2 kingdoms (Linnaeus), 5 kingdoms (Whittaker 1969).
    Current: 3 domains (Woese 1990), some use 7-8 kingdoms.
    The "6 kingdoms" is one of several competing systems.

  Grade: FAIL
  Human classification convention. The number varies by taxonomic
  framework (2, 3, 5, 6, 7, 8 have all been proposed).
  No physical constraint fixes it at 6.
```

---

### H-BIO-18: 6 Cortical Layers = n

> The mammalian neocortex has 6 layers

```
  Neocortical layers (Brodmann):
    I.   Molecular layer
    II.  External granular
    III. External pyramidal
    IV.  Internal granular
    V.   Internal pyramidal
    VI.  Multiform/polymorphic

  n = 6 checkmark

  Physical basis:
    The 6-layer structure is conserved across all mammals.
    It arises during development via inside-out neuronal migration.
    Each layer has distinct cell types, connectivity, and function.
    The 6-layer pattern is a fundamental feature of neocortical organization.

  BUT:
    Layer boundaries are somewhat gradual (not always sharp).
    Some cortical areas have fewer identifiable layers (e.g., agranular cortex
    lacks layer IV -- effectively 5 layers).
    Non-mammalian brains (birds, reptiles) have different organizations.
    Still, 6 is the canonical and most common count for mammalian neocortex.

  Grade: CLOSE
  6 cortical layers is well-established in neuroanatomy and conserved
  across mammals. More robust than "6 kingdoms" because it's a
  biological structure, not a classification convention.
  But some regions deviate, and "6" may partly reflect the granularity
  of histological observation.
```

---

### H-BIO-19: Carbon = Element 6 as Basis of Life

> Life is carbon-based; carbon has atomic number Z = 6

```
  Carbon:
    Atomic number Z = 6
    Electron configuration: 1s2 2s2 2p2
    4 valence electrons = tau(6)
    Can form 4 bonds (sp3) or fewer (sp2, sp)

  n = 6 checkmark
  Valence electrons = tau(6) = 4 checkmark

  Physical basis:
    Carbon's Z=6 and 4 valence electrons enable:
    - Tetravalent bonding (chains, rings, complex structures)
    - Similar electronegativity to H, O, N (stable covalent bonds)
    - Double/triple bonds (sp2/sp hybridization)
    Silicon (Z=14) is the only plausible alternative, but forms
    weaker bonds and less diverse chemistry.

  Grade: EXACT
  Carbon's atomic number IS 6. This is not a convention or approximation.
  The 4 valence electrons = tau(6) is also exact.
  Carbon is the unique element enabling the complexity of life.
  The strongest single match in the biology domain.
```

---

### H-BIO-20: Amino Acid Classification -- tau(6) = 4 property groups

> Amino acids are classified by 4 side-chain property types

```
  Standard classification:
    1. Nonpolar/hydrophobic (Gly, Ala, Val, Leu, Ile, Pro, Phe, Met, Trp)
    2. Polar uncharged (Ser, Thr, Cys, Tyr, Asn, Gln)
    3. Positively charged (Lys, Arg, His)
    4. Negatively charged (Asp, Glu)

  tau(6) = 4 checkmark

  BUT:
    This 4-way classification is one of many:
    - 2 groups: hydrophobic vs hydrophilic
    - 3 groups: nonpolar, polar, charged
    - 5 groups: add aromatic as a separate class
    - 7+ groups in detailed biochemistry texts
    The "4 types" is common but not the only scheme.

  Grade: WEAK
  The 4-group classification is widely used but not unique.
  Different textbooks use 2, 3, 4, 5, or more categories.
  Convention-dependent counting.
```

---

## Category C: RNA and Protein

---

### H-BIO-21: 3 Types of RNA = n/phi = 3

> Three major functional RNA types: mRNA, tRNA, rRNA

```
  Major RNA types:
    mRNA (messenger): carries genetic information
    tRNA (transfer): carries amino acids
    rRNA (ribosomal): catalytic component of ribosome

  n/phi = 3 checkmark

  BUT:
    Many other RNA types are now known:
    snRNA, snoRNA, miRNA, siRNA, lncRNA, piRNA, circRNA, etc.
    The "3 major types" is a textbook simplification from the 1960s.
    Modern molecular biology recognizes dozens of functional RNA classes.

  Grade: WEAK
  The 3-type classification is outdated. Modern RNA biology has
  far more than 3 functional categories. Cherry-picked framing.
```

---

### H-BIO-22: tRNA Anticodon = n/phi = 3 nucleotides

> The tRNA anticodon loop contains a 3-nucleotide anticodon

```
  tRNA anticodon:
    3 bases in the anticodon pair with the 3-base mRNA codon.
    This is structurally fixed by the ribosome decoding center.

  n/phi = 3 checkmark

  Physical basis:
    The anticodon length must match the codon length (3).
    This is the same as H-BIO-14 (reading frame = 3) applied to tRNA.
    Not an independent match.

  Grade: WEAK
  Same as H-BIO-14. Not independent. The 3 in the anticodon
  is forced by the 3 in the codon. Counts as one match, not two.
```

---

### H-BIO-23: Protein Structure -- tau(6) = 4 levels

> Protein structure has 4 hierarchical levels

```
  Protein structure hierarchy:
    1. Primary: amino acid sequence
    2. Secondary: alpha-helices, beta-sheets (local folding)
    3. Tertiary: 3D fold of single chain
    4. Quaternary: multi-subunit assembly

  tau(6) = 4 checkmark

  Physical basis:
    This 4-level hierarchy is universally taught and reflects
    real physical distinctions in the forces involved:
    - Primary: covalent (peptide bonds)
    - Secondary: backbone H-bonds
    - Tertiary: hydrophobic, disulfide, ionic
    - Quaternary: subunit interfaces

  BUT:
    Not all proteins have quaternary structure (only multimeric ones).
    Some add "supersecondary" or "domain" levels for 5-6 levels.
    But the canonical 4-level hierarchy is the most standard and
    reflects genuine physical distinctions.

  Grade: CLOSE
  The 4-level protein structure hierarchy is well-established,
  reflects real physics, and is universal in biochemistry education.
  The match tau(6)=4 is genuine. But 4 is common and tau(n)=4 for many n.
```

---

### H-BIO-24: Nucleotide Structure -- n/phi = 3 components

> Each nucleotide consists of 3 parts: base + sugar + phosphate

```
  Nucleotide components:
    1. Nitrogenous base (A, T/U, G, C)
    2. Pentose sugar (deoxyribose in DNA, ribose in RNA)
    3. Phosphate group

  n/phi = 3 checkmark

  Physical basis:
    This 3-part structure is universal. The phosphate-sugar backbone
    is invariant; only the base varies. Nucleosides (base+sugar)
    and nucleotides (base+sugar+phosphate) are distinct chemical entities.

  BUT:
    n/phi = 3 is just "3". The 3-component structure of nucleotides
    is a chemistry fact fully explained without n=6.

  Grade: WEAK
  3 components is real but 3 is trivially small. No explanatory gain from n=6.
```

---

### H-BIO-25: Amino Acid Backbone -- n/phi = 3 common atoms

> Every amino acid has 3 backbone components: amino, carboxyl, R-group

```
  Amino acid structure:
    Central alpha-carbon bonded to:
    1. Amino group (-NH2)
    2. Carboxyl group (-COOH)
    3. R group (side chain)
    4. Hydrogen atom

  Problem:
    That's 4 groups on the alpha-carbon, not 3.
    If you count "functional groups": amino + carboxyl + R = 3, ignoring H.
    If you count bonds: 4 (tetrahedral carbon).

  Grade: FAIL
  The count is ambiguous (3 or 4 depending on what you count).
  The alpha-carbon has 4 substituents (tetrahedral), not 3.
```

---

## Category D: Biochemistry and Metabolism

---

### H-BIO-26: Krebs Cycle -- sigma(6) - phi(6) = 10? or 8 steps?

> The citric acid cycle has 8 enzymatic steps

```
  Krebs (TCA) cycle:
    8 enzymatic reactions:
    1. Citrate synthase
    2. Aconitase
    3. Isocitrate dehydrogenase
    4. alpha-Ketoglutarate dehydrogenase
    5. Succinyl-CoA synthetase
    6. Succinate dehydrogenase
    7. Fumarase
    8. Malate dehydrogenase

  n=6 attempt:
    8 = tau(6) * phi(6) = 4 * 2? (matches, but ad hoc product)
    8 != any simple n=6 function directly.

  Grade: FAIL
  8 steps is well-established but no clean single n=6 function gives 8.
  tau*phi = 8 is a post-hoc product. No explanatory value.
```

---

### H-BIO-27: 6-Membered Rings in Biology

> The most important biological ring structures are 6-membered

```
  6-membered rings in biology:
    - Benzene ring (C6): aromatic amino acids (Phe, Tyr, Trp)
    - Pyrimidine ring (C4N2): bases C, T, U
    - Glucose: 6-membered pyranose ring
    - Cyclohexane conformations in steroids
    - Purine bases contain a 6-membered ring (fused with 5-membered)

  n = 6 checkmark

  Physical basis:
    6-membered rings are thermodynamically favored:
    - Bond angles ~120 degrees (sp2) or ~109.5 degrees (sp3)
    - Minimal ring strain for 6-membered rings
    - Aromaticity in planar 6-membered rings (Huckel: 4n+2 pi electrons)

  BUT:
    5-membered rings are equally important (ribose, deoxyribose,
    imidazole in His, furanose sugars, proline).
    7-membered and larger rings exist. The dominance of 6-membered
    rings is real but not exclusive.

  Grade: CLOSE
  6-membered rings genuinely dominate organic/biological chemistry
  due to minimal strain and aromaticity. The thermodynamic preference
  for 6-membered rings is a real physical fact, not just convention.
```

---

### H-BIO-28: Glycolysis -- 2 Phases = phi(6)

> Glycolysis has 2 phases: energy investment and energy payoff

```
  Glycolysis phases:
    Phase 1 (investment): steps 1-5, consumes 2 ATP
    Phase 2 (payoff): steps 6-10, produces 4 ATP

  phi(6) = 2 checkmark

  But glycolysis has 10 enzymatic steps total.

  BUT:
    Almost any metabolic pathway can be divided into 2 phases.
    This is a trivial dichotomy (input vs output, up vs down).
    phi(6) = 2 matching a 2-phase division is meaningless.

  Grade: FAIL
  Trivial. Any pathway has an "investment" and "payoff" phase.
```

---

### H-BIO-29: Water Molecule Geometry -- n/phi = 3 atoms

> Water (H2O) has 3 atoms per molecule

```
  Water molecule:
    H-O-H: 2 hydrogens + 1 oxygen = 3 atoms
    Bond angle: 104.5 degrees
    Essential for all known life.

  n/phi = 3 checkmark

  BUT:
    Every triatomic molecule has 3 atoms (CO2, NO2, H2S, etc.).
    3 atoms in water is a property of the molecular formula, not
    something that needs explanation via n=6.

  Grade: FAIL
  Trivially true. Water having 3 atoms tells us nothing about n=6.
```

---

### H-BIO-30: DNA Replication -- phi(6) = 2 (semiconservative)

> DNA replication is semiconservative: each daughter has phi(6) = 2 strands
> (1 old + 1 new)

```
  Semiconservative replication (Meselson-Stahl, 1958):
    Each new DNA molecule contains:
    - 1 parental (template) strand
    - 1 newly synthesized strand
    Total: 2 strands per daughter molecule.

  phi(6) = 2 checkmark

  BUT:
    This is the same as H-BIO-2 (double helix = 2 strands).
    Semiconservative replication is a consequence of 2-strand structure.
    Not an independent match.

  Grade: FAIL
  Derivative of H-BIO-2. Not independent. The "2" in replication
  is a consequence of the "2" in the double helix.
```

---

## Summary Table

| ID | Hypothesis | n=6 Function | Grade |
|----|-----------|-------------|-------|
| H-BIO-1 | DNA bases = 4 | tau(6) = 4 | **CLOSE** |
| H-BIO-2 | Double helix = 2 strands | phi(6) = 2 | **CLOSE** |
| H-BIO-3 | 64 codons | tau(6)^3 = 64 | **EXACT** |
| H-BIO-4 | 20 amino acids | tau*sopfr = 20 | **CLOSE** |
| H-BIO-5 | 3 stop codons | n/phi = 3 | **CLOSE** |
| H-BIO-6 | 5 nucleotide bases | sopfr(6) = 5 | **CLOSE** |
| H-BIO-7 | Codon degeneracy max = 6 | n = 6 | **WEAK** |
| H-BIO-8 | 1 start codon | mu(6) = 1 | **FAIL** |
| H-BIO-9 | 20 * 3 = 60 | sigma*sopfr = 60 | **WEAK** |
| H-BIO-10 | 2 base pair types | phi(6) = 2 | **WEAK** |
| H-BIO-11 | 6 cell cycle phases | n = 6 | **WEAK** |
| H-BIO-12 | ATP ~7.3 kcal/mol | sigma - sopfr = 7 | **WEAK** |
| H-BIO-13 | Histone octamer = 8 | tau*phi = 8 | **CLOSE** |
| H-BIO-14 | Codon reading frame = 3 | n/phi = 3 | **CLOSE** |
| H-BIO-15 | Membrane bilayer = 2 | phi(6) = 2 | **FAIL** |
| H-BIO-16 | Glucose C6H12O6 | (n, sigma, n) | **EXACT** |
| H-BIO-17 | 6 kingdoms | n = 6 | **FAIL** |
| H-BIO-18 | 6 cortical layers | n = 6 | **CLOSE** |
| H-BIO-19 | Carbon Z = 6 | n = 6 | **EXACT** |
| H-BIO-20 | 4 amino acid groups | tau(6) = 4 | **WEAK** |
| H-BIO-21 | 3 RNA types | n/phi = 3 | **WEAK** |
| H-BIO-22 | Anticodon = 3 bases | n/phi = 3 | **WEAK** |
| H-BIO-23 | 4 protein structure levels | tau(6) = 4 | **CLOSE** |
| H-BIO-24 | Nucleotide = 3 parts | n/phi = 3 | **WEAK** |
| H-BIO-25 | Amino acid = 3 backbone groups | n/phi = 3 | **FAIL** |
| H-BIO-26 | Krebs cycle 8 steps | tau*phi = 8 | **FAIL** |
| H-BIO-27 | 6-membered rings | n = 6 | **CLOSE** |
| H-BIO-28 | Glycolysis 2 phases | phi(6) = 2 | **FAIL** |
| H-BIO-29 | Water = 3 atoms | n/phi = 3 | **FAIL** |
| H-BIO-30 | Semiconservative = 2 | phi(6) = 2 | **FAIL** |

## Grade Distribution

| Grade | Count | Pct |
|-------|-------|-----|
| EXACT | 3 | 10.0% |
| CLOSE | 10 | 33.3% |
| WEAK | 9 | 30.0% |
| FAIL | 8 | 26.7% |

**Non-failing: 22/30 (73.3%)**
**EXACT: H-BIO-3 (64 codons), H-BIO-16 (glucose), H-BIO-19 (carbon Z=6)**
