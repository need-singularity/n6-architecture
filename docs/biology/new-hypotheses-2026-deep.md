# N6 Biology -- Deep Hypotheses H-BIO-81~105

> Extension of H-BIO-1~30 (core) and H-BIO-61~80 (extreme).
> Going deeper into molecular biology, protein structure, cell biology,
> genetics/evolution, and neuroscience through n=6 arithmetic.
> Generated: 2026-03-31

> **Honesty principle**: Biology's abundance of small integers (2-6) inflates
> coincidental matches. We grade EXACT only when the number is fixed by
> chemistry/physics and cannot reasonably be counted differently. All formulas
> are checked against multiple n values for specificity.

## Core Constants

```
  n = 6          sigma(6) = 12     tau(6) = 4      phi(6) = 2
  sopfr(6) = 5   J_2(6) = 24       mu(6) = 1       lambda(6) = 2
  R(6) = 1       sigma-tau = 8     sigma-phi = 10   sigma-mu = 11
  J_2-tau = 20   tau^2 = 16        sigma^2 = 144
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

## Previously EXACT matches (do not duplicate)
```
  H-BIO-3:  64 codons = tau^3 = phi^n
  H-BIO-16: Glucose C6H12O6 = (n, sigma, n)
  H-BIO-19: Carbon Z=6, valence=tau=4
  H-BIO-66: Benzene C6H6, 6 pi-electrons
```

---

## Category I: Deep Molecular Biology (H-BIO-81~86)

---

### H-BIO-81: DNA Double Helix Diameter = phi(6) nm

> B-form DNA has a diameter of ~2.0 nm = phi(6) nanometers

```
  B-form DNA geometry:
    Diameter: 2.0 nm (20 Angstroms)
    Source: Watson & Crick (1953), confirmed by X-ray fiber diffraction
    and atomic-resolution crystal structures (Drew et al., 1981).

  phi(6) = 2 checkmark (in nm units)

  Physical basis:
    The 2 nm diameter is set by the base-pair stacking geometry:
    - Two antiparallel sugar-phosphate backbones
    - Base pairs span ~1.08 nm (center-to-center)
    - Backbone van der Waals radius adds ~0.46 nm per side
    - Total: ~2.0 nm

  BUT:
    The match is unit-dependent. In Angstroms the diameter is 20.
    In any other length unit the "2" disappears.
    phi(6) = 2 is trivially small.
    A-form DNA is ~2.3 nm, Z-form is ~1.8 nm.

  Grade: WEAK
  The number 2 nm is real but unit-dependent.
  phi(6)=2 is too trivially common to be meaningful.
```

---

### H-BIO-82: Telomere Repeat TTAGGG = n = 6 Bases Per Repeat

> Human telomeric DNA consists of tandem repeats of TTAGGG (6 nucleotides)

```
  Telomere sequence:
    Human: 5'-TTAGGG-3' repeated ~2500 times
    Vertebrates: TTAGGG universally conserved
    Length per repeat: 6 nucleotides

  n = 6 checkmark

  Physical basis:
    The 6-base repeat unit is recognized by:
    - Shelterin complex (TRF1, TRF2, POT1, TIN2, TPP1, RAP1 = 6 proteins!)
    - Telomerase RNA template (hTR contains the complementary AAUCCC)
    - G-quadruplex formation (4 TTAGGG repeats = 24 bases = J_2)

  Additional n=6 connections:
    6-base repeat unit = n
    6 shelterin proteins = n (Shelterin: TRF1, TRF2, POT1, TIN2, TPP1, RAP1)
    4 repeats form G-quadruplex of 24 bases = J_2(6)
    G-quadruplex has 4 strands = tau(6)

  Cross-species variation:
    Vertebrates: TTAGGG (6 bases) -- universal
    Arabidopsis: TTTAGGG (7 bases) -- different
    Tetrahymena: TTGGGG (6 bases) -- still 6!
    S. cerevisiae: TG_{1-3} (irregular, 2-3 bases)
    Insects (some): TTAGG (5 bases)

  Grade: CLOSE
  The vertebrate telomere repeat is exactly 6 bases, and this is a hard
  biochemical fact (not convention). The 6 shelterin proteins strengthen
  the connection. However, other organisms use 5 or 7-base repeats,
  showing 6 is not a universal biological necessity. The G-quadruplex
  connection (24 = J_2) is interesting but requires exactly 4 repeats.
```

---

### H-BIO-83: Nucleosome Histone Types x Copies = tau(6) x phi(6) = sigma(6) - tau(6) = 8

> The histone octamer has 4 types x 2 copies, wrapping DNA in ~1.65 turns

```
  NOTE: The octamer = 8 is already in H-BIO-13.
  This hypothesis focuses on the DEEPER structure.

  Nucleosome core particle (Luger et al., Nature 1997, PDB: 1AOI):
    H3-H4 tetramer:  2 copies each of H3 and H4
    H2A-H2B dimers:  2 copies each of H2A and H2B
    Total: 8 proteins = sigma(6) - tau(6)

  Deeper structural match:
    DNA wrapping: 147 bp = ~1.65 superhelical turns
    Linker histone: H1 (5th type)
    Including H1: 5 histone types = sopfr(6)
    H1 stoichiometry: 1 per nucleosome = mu(6)

    Assembly hierarchy:
      H3-H4 dimer -> (H3-H4)_2 tetramer -> add 2x(H2A-H2B) -> octamer
      2 -> 4 -> 8: powers of phi(6) = {phi, phi^2, phi^3} = {2, 4, 8}

  BUT:
    The 147 bp does NOT match any clean n=6 expression.
    The "5 types including H1" is a standard count, but H1 is
    structurally and functionally distinct from core histones.
    The powers-of-2 assembly is a general consequence of dimerization.

  Grade: CLOSE
  The octamer (4 types x 2 copies) and 5 total histone types (with H1)
  mapping to tau*phi=8 and sopfr=5 are clean matches. The powers-of-2
  assembly hierarchy is real but not n=6-specific. Mostly extends H-BIO-13.
```

---

### H-BIO-84: Human Chromosomes = J_2(6) - mu(6) = 23 Pairs

> Humans have 23 chromosome pairs (46 total = 23 x phi(6))

```
  Human karyotype:
    23 pairs of chromosomes (22 autosomal + 1 sex chromosome pair)
    Total: 46 chromosomes per diploid cell
    Haploid number: n = 23 (gametes)

  23 = J_2(6) - mu(6) = 24 - 1

  Also:
    46 = 23 * phi(6) = (J_2 - mu) * phi
    22 autosomes = J_2 - phi = 24 - 2

  Cross-species chromosome counts:
    Chimpanzee: 24 pairs = J_2(6)  (human chr 2 = fusion of two ancestral)
    Gorilla: 24 pairs = J_2(6)
    Mouse: 20 pairs = J_2 - tau
    Dog: 39 pairs
    Cat: 19 pairs
    Fruit fly: 4 pairs = tau(6)
    Rice: 12 pairs = sigma(6)

  Notable:
    Great apes (except human): 24 pairs = J_2(6) exactly
    Human is 23 = 24-1 due to a well-documented chromosomal fusion
    (human chromosome 2 = fusion of ancestral chromosomes 2A and 2B)
    Drosophila: 4 pairs = tau(6) (model organism)
    Rice: 12 pairs = sigma(6) (most important crop)

  BUT:
    Chromosome count varies enormously across species (1 to >1000).
    The matches for specific species are cherry-picked.
    Chromosome number is not a deeply constrained biological quantity
    (polyploidy, fission, fusion are common in evolution).
    J_2-mu=23 is an ad hoc formula.

  Grade: WEAK
  Human 23 = J_2-mu and great ape 24 = J_2 are numerically correct.
  The fusion story (24->23) is well-established molecular biology.
  But chromosome numbers vary wildly across species, and cherry-picking
  species that match n=6 functions is textbook post-hoc reasoning.
  The Drosophila 4 and rice 12 matches are interesting but not compelling.
```

---

### H-BIO-85: Shelterin Complex = n = 6 Proteins

> The telomere-protecting shelterin complex consists of exactly 6 protein subunits

```
  Shelterin complex (de Lange, 2005):
    1. TRF1 (Telomeric Repeat binding Factor 1) -- binds ds TTAGGG
    2. TRF2 (Telomeric Repeat binding Factor 2) -- binds ds TTAGGG
    3. POT1 (Protection Of Telomeres 1) -- binds ss TTAGGG
    4. TIN2 (TRF1-Interacting Nuclear factor 2) -- bridges TRF1/TRF2
    5. TPP1 (also ACD) -- bridges POT1 and TIN2
    6. RAP1 (Repressor/Activator Protein 1) -- binds TRF2

  n = 6 checkmark

  Physical basis:
    The 6-subunit complex is the minimal set required for:
    - Double-strand binding (TRF1 + TRF2 = 2)
    - Single-strand binding (POT1 = 1)
    - Bridging/scaffolding (TIN2 + TPP1 = 2)
    - Regulatory (RAP1 = 1)

    The complex protects telomeres from DNA damage response pathways
    (ATM/ATR). All 6 are essential -- knockout of any one leads to
    telomere dysfunction.

  Connection with H-BIO-82:
    6 proteins protecting 6-base repeats = n protecting n.
    This is a structural echo within the same biological system.

  BUT:
    Additional telomere-associated proteins exist (CST complex: CTC1,
    STN1, TEN1 -- 3 more proteins). If you count CST: 9 proteins.
    The "6" is specific to the core shelterin definition.
    Other protein complexes have 6 subunits coincidentally
    (e.g., eIF2B, Sec61/SecYEG heterotrimers as dimers).

  Grade: CLOSE
  Shelterin = 6 proteins is a well-defined, experimentally verified
  complex (all 6 identified and structurally characterized). The
  pairing with 6-base telomere repeats strengthens the connection.
  But additional telomere proteins (CST) exist outside the core complex,
  and "6 subunits" is common among protein complexes.
```

---

### H-BIO-86: Spliceosome snRNAs = sopfr(6) = 5

> The major spliceosome requires exactly 5 small nuclear RNAs

```
  Major spliceosome components:
    U1 snRNA -- recognizes 5' splice site
    U2 snRNA -- recognizes branch point
    U4 snRNA -- masks U6 until needed
    U5 snRNA -- aligns exons for ligation
    U6 snRNA -- catalytic center (metalloenzyme)

  sopfr(6) = 5 checkmark

  Physical basis:
    The 5 snRNAs form the catalytic and structural core of the
    spliceosome. Each has a distinct, essential role.
    The spliceosome assembles in ordered steps:
      E complex (U1) -> A (U2) -> B (U4/U6.U5 tri-snRNP) -> C (catalytic)

  Additional observations:
    snRNA numbering: U1, U2, U4, U5, U6
    Note: U3 exists but functions in rRNA processing, not splicing
    The numbering gap (no U3 in spliceosome) is a historical artifact.

    Minor spliceosome also has 5 snRNAs: U11, U12, U4atac, U5, U6atac
    (U5 is shared between major and minor spliceosomes)

  BUT:
    sopfr(n) = 5 for n = 6, 12, 18, 20, 32, ... (not unique to n=6).
    The count 5 is a small integer.
    >100 additional protein factors are required for spliceosome function.
    Counting only snRNAs while ignoring protein components is selective.

  Grade: CLOSE
  5 snRNAs in both major and minor spliceosomes is a hard biochemical
  fact. The roles are well-defined and each is essential. But 5 is
  a small number and sopfr is not specific to n=6.
```

---

## Category II: Protein Structure (H-BIO-87~91)

---

### H-BIO-87: Alpha Helix = 3.6 Residues/Turn = n * n/(sigma-phi)

> The alpha helix has 3.6 amino acid residues per turn

```
  Alpha helix parameters (Pauling, Corey & Branson, 1951):
    Residues per turn: 3.6 (exactly 18 residues per 5 turns)
    Rise per residue: 1.5 Angstroms (0.15 nm)
    Pitch: 5.4 Angstroms (0.54 nm)
    Hydrogen bond: C=O of residue i to N-H of residue i+4
    Phi angle: ~-57 degrees, Psi angle: ~-47 degrees

  n=6 decomposition:
    3.6 = 18/5 = (n * n/phi) / sopfr = (n * n/phi) / sopfr
    3.6 = n/phi + n/sigma = 3 + 0.6? (forced)
    3.6 = n * (n/(sopfr * tau)) = 6 * 6/20 = 36/20 = 1.8? No.

  Better:
    18 residues per 5 turns:
    18 = n * (n/phi) = 6 * 3
    5 turns = sopfr(6)
    So: 3.6 = n*(n/phi) / sopfr(6) = 18/5

  Also:
    i+4 hydrogen bond spacing = tau(6)
    5.4 A pitch = sopfr(6) + tau(6)/10? (forced unit manipulation)

  Physical basis:
    The 3.6 residues/turn is determined by:
    - Peptide bond planarity (partial double bond character)
    - Optimal hydrogen bond geometry (2.8-3.0 A N...O distance)
    - Steric constraints (Ramachandran plot)
    The value 3.6 = 18/5 is a consequence of backbone torsion angles
    and is not adjustable by evolution.

  BUT:
    3.6 = 18/5 and 18 = 6*3, 5 = sopfr(6) is a clean factorization.
    However, 18 = 6*3 can be written as n*(n/phi) but this is
    selecting a particular decomposition post-hoc.
    The i+4 hydrogen bond = tau(6) is a genuine structural number.

  Grade: CLOSE
  The alpha helix 3.6 = 18/5 where 18 = 6*3 and 5 = sopfr(6) is
  a surprisingly clean factorization. The i+4 hydrogen bond = tau(6)
  is structurally real. But the value is physics-determined (backbone
  geometry) and the n=6 decomposition is one of several possible.
  The i+4 spacing is the strongest sub-match here.
```

---

### H-BIO-88: Beta Sheet H-Bond Spacing = phi(6) Residues

> In beta sheets, hydrogen bonds connect every 2nd residue along each strand

```
  Beta sheet structure (Pauling & Corey, 1951):
    Parallel and antiparallel arrangements
    Hydrogen bonds between strands, not within a strand
    Each residue forms H-bonds to the adjacent strand
    Alternating pattern: side chains point up/down every 2 residues = phi(6)

  phi(6) = 2 checkmark (alternating up/down pattern period)

  Physical basis:
    The beta strand is nearly fully extended (phi~-120, psi~+120).
    Each amino acid's R-group alternates above/below the sheet plane.
    The repeat distance along the strand: ~7.0 A for 2 residues.

  BUT:
    The "every 2 residues" alternation is a trivial consequence
    of the tetrahedral carbon geometry. In an extended chain,
    substituents alternate sides by construction.
    phi(6) = 2 is trivially small.
    This is a geometric necessity, not a biological choice.

  Grade: WEAK
  The alternating pattern is real but is a trivial consequence
  of polymer geometry. phi(6) = 2 adds no insight.
```

---

### H-BIO-89: Ramachandran Allowed Regions -- tau(6) Major Clusters

> The Ramachandran plot has approximately 4 major allowed regions

```
  Ramachandran plot (Ramachandran, Ramakrishnan & Sasisekharan, 1963):
    Plots phi vs psi backbone torsion angles for amino acids.
    Allowed regions (non-glycine, non-proline):
      1. Alpha-helix region (phi ~ -60, psi ~ -45)
      2. Beta-sheet region (phi ~ -120, psi ~ +120)
      3. Left-handed alpha-helix (phi ~ +60, psi ~ +45) -- rare
      4. Polyproline II / extended (phi ~ -75, psi ~ +150)

  tau(6) = 4 checkmark (4 major clusters)

  Physical basis:
    The allowed regions are determined by:
    - Van der Waals radii of backbone atoms
    - Partial double bond character of the peptide bond (planar)
    - Steric clashes between adjacent residues
    These are pure physics constraints on the protein backbone.

  BUT:
    The count of "regions" depends on resolution:
    - 2 broad regions (right-handed and left-handed halves)
    - 3 major regions (alpha, beta, PPII -- ignoring L-alpha)
    - 4 regions (including L-alpha)
    - More regions at finer resolution (3_10 helix, pi helix, etc.)
    The 4-cluster description is common but not unique.
    tau(6) = 4 for many n.

  Grade: WEAK
  4 Ramachandran clusters is a common description but the count
  is resolution-dependent. tau(6)=4 is too common to be specific.
```

---

### H-BIO-90: Protein Secondary Structure Types = tau(6)

> There are 4 canonical secondary structure types: alpha helix, beta sheet, turn, coil

```
  Protein secondary structure classification:
    1. Alpha helix -- regular H-bonded (3.6 res/turn)
    2. Beta sheet -- extended, inter-strand H-bonds
    3. Turn (beta turn) -- tight reversal, i to i+3 H-bond
    4. Coil/loop -- irregular, no regular H-bond pattern

  tau(6) = 4 checkmark

  Physical basis:
    DSSP algorithm (Kabsch & Sander, 1983) defines 8 secondary
    structure types: H (alpha), B (isolated beta bridge), E (strand),
    G (3_10 helix), I (pi helix), T (turn), S (bend), " " (coil).

    But the standard 4-class reduction is universally used:
    - H, G, I -> "helix"
    - B, E -> "sheet/strand"
    - T, S -> "turn"
    - " " -> "coil"

  BUT:
    DSSP uses 8 types (= sigma - tau, coincidentally).
    The 4-class reduction is a convention.
    Some classifications use 3 (helix, sheet, other).
    The 4-type system is the most common textbook treatment.

  Grade: WEAK
  The 4-class system is standard but convention-dependent.
  DSSP's 8 classes are more rigorous. tau(6)=4 is not specific.
  Interestingly, DSSP 8 = sigma-tau, but that is a secondary observation.
```

---

### H-BIO-91: Disulfide Bond -- phi(6) = 2 Cysteines

> Each disulfide bond connects exactly 2 cysteine residues

```
  Disulfide bond (S-S):
    Formed between 2 cysteine residues (R-SH + HS-R -> R-S-S-R + 2H)
    Bond energy: ~60 kcal/mol (~251 kJ/mol)
    Critical for protein stability (especially secreted proteins)

  phi(6) = 2 checkmark

  BUT:
    A bond by definition connects 2 atoms/residues.
    ANY chemical bond involves 2 partners.
    This is a tautology, not a discovery.
    phi(6) = 2 matching "2 endpoints of a bond" is meaningless.

  Grade: FAIL
  Trivially true of all chemical bonds. Zero specificity.
```

---

## Category III: Cell Biology (H-BIO-92~96)

---

### H-BIO-92: Cell Cycle = tau(6) = 4 Phases

> The cell cycle consists of 4 distinct phases: G1, S, G2, M

```
  Cell cycle phases (Alberts, Molecular Biology of the Cell):
    1. G1 (Gap 1) -- cell growth, preparation for DNA synthesis
    2. S (Synthesis) -- DNA replication
    3. G2 (Gap 2) -- preparation for mitosis
    4. M (Mitosis) -- cell division

  tau(6) = 4 checkmark

  Physical basis:
    The 4-phase model is universally accepted in cell biology.
    Each phase has distinct cyclin-CDK regulators:
    - G1: Cyclin D/CDK4,6
    - S: Cyclin A/CDK2
    - G2: Cyclin A/CDK1
    - M: Cyclin B/CDK1

  Additional structure:
    Interphase (G1+S+G2) = 3 phases = n/phi
    M phase subdivisions = 5 stages = sopfr(6) (see H-BIO-93)
    Total if expanded: G1+S+G2 + 5 mitotic = 8 = sigma-tau

  BUT:
    Some texts add G0 (quiescent) as a 5th state.
    Meiosis has additional phases.
    4 phases is standard but the count depends on whether you
    include G0 or subdivide M phase.
    tau(6) = 4 for many n.

  Grade: CLOSE
  The 4-phase cell cycle is one of the most well-established
  frameworks in cell biology, with distinct molecular regulators
  for each phase. The 4 cyclin-CDK pairs reinforce the count.
  tau(6) = 4 is not n=6-specific, but the biological robustness
  of this count is higher than most classification-based matches.
```

---

### H-BIO-93: Mitosis = sopfr(6) = 5 Stages

> Mitosis consists of 5 stages: prophase, prometaphase, metaphase, anaphase, telophase

```
  Mitosis stages (standard textbooks):
    1. Prophase -- chromosome condensation, spindle formation
    2. Prometaphase -- nuclear envelope breakdown, kinetochore attachment
    3. Metaphase -- chromosomes align at metaphase plate
    4. Anaphase -- sister chromatid separation
    5. Telophase -- nuclear envelope reformation, decondensation

  sopfr(6) = 5 checkmark

  Physical basis:
    Each stage has distinct cytological markers observable by
    microscopy. The transitions are regulated by specific
    molecular switches (APC/C activation, separase, etc.).

  Note:
    Some older texts combine prometaphase with prophase (4 stages).
    Some add cytokinesis (6 stages = n).
    The 5-stage model is the modern standard (Alberts 7th ed.).

  BUT:
    sopfr(n)=5 for many n.
    The prometaphase/prophase boundary is somewhat arbitrary.
    5 is a small integer.

  Grade: CLOSE
  The 5-stage mitosis model is standard in modern cell biology.
  Each stage has distinct molecular and cytological markers.
  sopfr(6)=5 matches but is not n=6-specific.
```

---

### H-BIO-94: ATP = n/phi = 3 Phosphate Groups; Adenosine Has n-Membered Ring

> ATP has 3 phosphate groups (triphosphate) and its adenine base contains a 6-membered ring

```
  ATP (adenosine triphosphate):
    Adenine base: purine (6-membered + 5-membered fused rings)
    Ribose sugar: 5-carbon (sopfr = 5)
    Phosphate chain: 3 phosphate groups (alpha, beta, gamma)

  n/phi = 3 (phosphate groups)
  n = 6 (pyrimidine ring within adenine)
  sopfr = 5 (ribose carbons)

  Hydrolysis cascade:
    ATP -> ADP + Pi (removes 1 phosphate)
    ADP -> AMP + Pi (removes 1 more)
    AMP has 1 phosphate = mu(6)
    Energy per hydrolysis: ~-30.5 kJ/mol standard, ~-54 kJ/mol in vivo

  Phosphorylation states: {3, 2, 1, 0} = {n/phi, phi, mu, 0}
    ATP (3), ADP (2), AMP (1), adenosine (0)

  BUT:
    GTP, CTP, UTP also have 3 phosphate groups -- this is a
    universal property of nucleoside triphosphates, not specific to ATP.
    The 3 phosphate groups are a consequence of the sequential
    phosphorylation chemistry, not a numerological fact.
    n/phi = 3 is too small to be compelling.

  Grade: CLOSE
  The 3 phosphate groups are a hard chemical fact. The multi-level
  n=6 mapping (3 phosphates, 6-membered ring, 5-carbon sugar) is
  a structural coincidence across different parts of the same molecule.
  But each individual match (3, 5, 6) is a small number.
```

---

### H-BIO-95: Krebs Cycle Produces sigma(6) = 12 Electron Carriers per Glucose

> Complete oxidation of one glucose through glycolysis + Krebs cycle produces 12 carrier molecules

```
  Electron carrier production per glucose (Lehninger, ch. 16-19):
    Glycolysis: 2 NADH
    Pyruvate dehydrogenase: 2 NADH (x2 pyruvate)
    Krebs cycle (x2 turns):
      2 x 3 NADH = 6 NADH
      2 x 1 FADH2 = 2 FADH2
      2 x 1 GTP = 2 GTP

  Total NADH: 2 + 2 + 6 = 10
  Total FADH2: 2
  Total reduced carriers: 10 + 2 = 12 = sigma(6)

  Also:
    Total NADH + FADH2 + GTP = 10 + 2 + 2 = 14 (not clean)
    Total electron pairs donated to ETC: 12 (10 NADH + 2 FADH2)
    This yields: 10 x 2.5 ATP + 2 x 1.5 ATP = 25 + 3 = 28 ATP from ETC
    Plus 4 substrate-level ATP (2 glycolysis + 2 GTP) = 32 total
    (Modern estimate: 30-32 ATP per glucose)

  sigma(6) = 12 electron carrier molecules checkmark

  BUT:
    The count depends on what you include:
    - Glycolysis produces cytoplasmic NADH which may yield only 1.5 ATP
      (depending on shuttle system used)
    - FADH2 is sometimes counted separately from NADH
    - The "12 reduced carriers" count combines NADH and FADH2 as equivalent,
      which they are not (different ETC entry points, different ATP yield)

    32 ATP per glucose is approximate (range: 30-38 depending on
    shuttle, proton leak, and P:O ratio assumptions).

  Grade: CLOSE
  12 total reduced electron carriers (NADH + FADH2) per glucose is
  a standard textbook count. sigma(6) = 12 matches exactly.
  The electron carrier count is more robust than the ATP yield
  (which varies by assumptions). But combining NADH and FADH2
  treats unequal molecules as equivalent.
```

---

### H-BIO-96: Glycolysis = sigma(6) - phi(6) = 10 Steps

> Glycolysis consists of exactly 10 enzymatic steps

```
  Glycolysis pathway (Embden-Meyerhof-Parnas):
    1.  Hexokinase (glucose -> G6P)
    2.  Phosphoglucose isomerase (G6P -> F6P)
    3.  Phosphofructokinase-1 (F6P -> F1,6BP)
    4.  Aldolase (F1,6BP -> DHAP + G3P)
    5.  Triose phosphate isomerase (DHAP -> G3P)
    6.  G3P dehydrogenase (G3P -> 1,3BPG)
    7.  Phosphoglycerate kinase (1,3BPG -> 3PG)
    8.  Phosphoglycerate mutase (3PG -> 2PG)
    9.  Enolase (2PG -> PEP)
    10. Pyruvate kinase (PEP -> pyruvate)

  sigma(6) - phi(6) = 12 - 2 = 10 checkmark

  Physical basis:
    The 10-step pathway is universally conserved in all glycolytic
    organisms. Each enzyme is well-characterized, with crystal
    structures available for all 10.

    The pathway divides as:
    - Investment phase: steps 1-5 (sopfr = 5 steps, consume 2 ATP)
    - Payoff phase: steps 6-10 (sopfr = 5 steps, produce 4 ATP)
    - Each half has exactly sopfr(6) = 5 steps

  Additional:
    10 enzymes, 10 intermediates (glucose through pyruvate)
    Net yield: 2 ATP + 2 NADH per glucose (phi ATP, phi NADH)

  BUT:
    sigma-phi = 10 is a specific formula, but 10 is a common number.
    The 5+5 split as sopfr+sopfr is interesting but could be coincidence.
    Gluconeogenesis has 11 steps (bypasses 3 irreversible glycolytic steps
    with different enzymes). Pentose phosphate pathway has different counts.

  Grade: CLOSE
  10 glycolytic steps is one of the hardest counts in biochemistry --
  every textbook lists exactly these 10 enzymes, no more, no less.
  The 5+5 investment/payoff split adding a second n=6 connection
  (sopfr = 5) strengthens the match. sigma-phi = 10 is a clean formula.
  But 10 is a common number and sigma-phi=10 is available for many n.
```

---

## Category IV: Genetics and Evolution (H-BIO-97~101)

---

### H-BIO-97: Wobble Position Allows tau(6) = 4 Non-Standard Pairings

> The wobble position (3rd codon base) permits ~4 types of non-standard base pairs

```
  Wobble hypothesis (Crick, 1966):
    The 3rd codon position allows non-Watson-Crick pairing:
    Standard wobble pairs recognized:
      1. G-U wobble (most common)
      2. I-U (inosine pairs with U)
      3. I-C (inosine pairs with C)
      4. I-A (inosine pairs with A)

    Inosine (I) in the 1st anticodon position is a key wobble base.

  tau(6) = 4 wobble pair types (G-U + 3 inosine pairs)

  Physical basis:
    The wobble rules explain codon degeneracy:
    - Why 2-fold vs 4-fold degenerate codons differ
    - Why inosine at position 34 of tRNA is essential
    - How 45 tRNAs can read 61 sense codons

  BUT:
    The number of "wobble pair types" depends on counting:
    - Some treatments list only G-U and I-C/I-U/I-A (4 types)
    - Others include U-U, U-C (additional wobble in mitochondria)
    - Modified bases (xm5U, etc.) create further wobble pairs
    - The original Crick rules have been expanded significantly
    tau(6) = 4 for many n.

  Grade: WEAK
  The original 4 wobble pair types (Crick) is a standard but
  simplified count. Expanded wobble rules include more pairings.
  tau(6) = 4 is not specific enough.
```

---

### H-BIO-98: Human Mitochondrial tRNAs = J_2(6) - phi(6) = 22

> Human mitochondria encode exactly 22 tRNAs (minimal set for translation)

```
  Human mitochondrial genome:
    16,569 bp circular DNA
    Encodes: 13 proteins + 22 tRNAs + 2 rRNAs = 37 genes total
    The 22 tRNAs are the minimum needed to read all codons
    (using extensive wobble and modified bases)

  22 = J_2(6) - phi(6) = 24 - 2

  Also:
    13 proteins = sigma(6) + mu(6) = 13
    2 rRNAs = phi(6)
    37 total genes = ?  (no clean formula: 37 is prime)

    The 13 proteins:
      7 Complex I subunits (ND1-6, ND4L) -- 7 = sigma-sopfr
      1 Complex III subunit (cytochrome b)
      3 Complex IV subunits (COX I-III) -- 3 = n/phi
      2 Complex V subunits (ATP6, ATP8) -- 2 = phi

  BUT:
    22 = J_2 - phi is an ad hoc formula.
    Other organisms have different mitochondrial tRNA counts
    (trypanosomes: 0, import all tRNAs; plants: ~20-30, varies).
    The 22 is specific to mammalian mitochondria.

  Grade: WEAK
  22 tRNAs is a hard count for human mitochondria (fully sequenced,
  Anderson et al., 1981). But J_2-phi=22 is an ad hoc formula and
  the count is specific to mammals. The 13 protein subcomplex
  distribution {7,1,3,2} mapping to {sigma-sopfr, mu, n/phi, phi}
  is an interesting secondary pattern but post-hoc.
```

---

### H-BIO-99: Genetic Code Universality -- mu(6) = 1 Standard Code + Variants

> There is 1 "standard" genetic code with ~25 known variant codes

```
  Genetic code variants (NCBI taxonomy):
    1 standard (universal) genetic code
    ~25 known variant codes (mitochondrial, mycoplasma, ciliate, etc.)
    Total recognized: ~33 (NCBI translation tables, as of 2025)

  Variations from the standard code are relatively minor:
    Most variants change only 1-3 codon assignments.
    The core structure (64 codons, triplet reading) is absolutely conserved.

  mu(6) = 1 standard code -- but this is trivially true (any "standard"
  is "one"). The ~25 variants do not match any clean n=6 expression.

  Grade: FAIL
  "1 standard" matches any singleton. The variant count (~25-33) is
  not stable (new ones are discovered periodically) and does not
  match n=6 functions cleanly.
```

---

### H-BIO-100: DNA Replication Origins -- Eukaryotic Licensing = n = 6 ORC Subunits

> The Origin Recognition Complex (ORC) consists of 6 subunits

```
  ORC (Origin Recognition Complex):
    Orc1, Orc2, Orc3, Orc4, Orc5, Orc6
    6 subunits forming a ring-like structure
    Essential for replication licensing in all eukaryotes

  n = 6 checkmark

  Physical basis:
    ORC binds replication origins and recruits CDC6 and CDT1,
    which load the MCM2-7 helicase (also 6 subunits!).
    The pre-replication complex (pre-RC) assembly:
      ORC (6 subunits) + CDC6 (1) + CDT1 (1) -> loads MCM2-7 (6 subunits)

  Double-6 pattern:
    ORC = 6 subunits = n
    MCM2-7 helicase = 6 subunits = n
    Both are hexameric ring complexes essential for DNA replication.

  Additional:
    MCM2-7 forms a double hexamer at origins (12 subunits total = sigma)
    Pre-RC: 6 + 1 + 1 + 12 = 20 polypeptides = J_2 - tau

  BUT:
    Many cellular complexes are hexameric (due to 6-fold ring symmetry
    being geometrically favorable for ring-shaped machines).
    Examples: clamp loaders, helicases, chaperonins.
    The "6" reflects ring symmetry, a geometric preference.

  Grade: CLOSE
  ORC = 6 and MCM = 6 are both hard structural facts, verified by
  cryo-EM (Bleichert et al., 2017). The double-hexamer (12 = sigma)
  and pre-RC (20 = J_2-tau) counts add depth. But hexameric rings
  are common in biology (GroEL, ClpX, etc.) -- the geometric
  favorability of 6-fold symmetry partly explains the prevalence.
  The ORC+MCM double match is the strongest aspect.
```

---

### H-BIO-101: Exon Shuffling and Phase Distribution -- n/phi = 3 Intron Phases

> Introns are classified into exactly 3 phases (0, 1, 2)

```
  Intron phase classification:
    Phase 0: intron between codons (between 3rd position of one codon
             and 1st position of next)
    Phase 1: intron between 1st and 2nd positions of a codon
    Phase 2: intron between 2nd and 3rd positions of a codon

  n/phi = 3 phases (0, 1, 2)

  Phase distribution in human genome (GENCODE):
    Phase 0: ~48-50% (most common)
    Phase 1: ~26-30%
    Phase 2: ~22-25%
    Phase 0 dominance is consistent with "introns early" hypothesis.

  Egyptian fraction parallel:
    Phase 0: ~1/2
    Phase 1: ~1/3 (approximate)
    Phase 2: ~1/6 (approximate)
    1/2 + 1/3 + 1/6 = 1 (Egyptian fraction of 6!)

  Physical basis:
    3 phases is forced by the triplet code (3 possible insertion points
    within a codon). This is a mathematical necessity, not biology.

  BUT:
    3 phases is trivially derived from 3-base codons.
    The Egyptian fraction distribution (~50/30/20) is approximate,
    not exact 50/33/17. The actual ratios vary by genome and
    gene set analyzed.
    n/phi = 3 is too small and too common.

  Grade: WEAK (for phase count) / CLOSE (if Egyptian fraction distribution holds)
  The 3 phases are mathematically forced. The Egyptian fraction-like
  distribution (~1/2, ~1/3, ~1/6) is an intriguing approximate match
  but not exact. The distribution varies significantly by species
  and gene set. Overall: WEAK.
```

---

## Category V: Neuroscience (H-BIO-102~105)

---

### H-BIO-102: Voltage-Gated Sodium Channel = tau(6) Domains x n Segments

> Voltage-gated sodium channels have 4 homologous domains, each with 6 transmembrane segments

```
  Nav channel structure (Catterall, 2000; Yan et al., Science 2017):
    Single alpha subunit:
      4 homologous domains (DI, DII, DIII, DIV) = tau(6)
      Each domain: 6 transmembrane segments (S1-S6) = n
      Total transmembrane segments: 4 x 6 = 24 = J_2(6)

    S4 in each domain: voltage sensor (positive charges)
    S5-S6 in each domain: pore lining
    Selectivity filter: DEKA motif (4 residues, 1 from each domain)

  tau(6) = 4 domains
  n = 6 segments per domain
  J_2(6) = 24 total transmembrane segments

  Cross-validation with other voltage-gated channels:
    Kv channels: 4 separate subunits x 6 TM segments = 24 = J_2
    Cav channels: 4 domains x 6 TM segments = 24 = J_2
    ALL voltage-gated cation channels share this 4x6 = 24 architecture

  Physical basis:
    The 4x6 architecture is one of the most conserved structural
    motifs in neuroscience. It predates the divergence of Na+/K+/Ca2+
    channels and is found in bacteria (NaChBac, but as tetramers of
    single 6-TM domains).

    The 6-TM domain appears to be the fundamental unit:
    - 6 TM segments = minimal for voltage-sensing (S1-S4) + pore (S5-S6)
    - 4-fold assembly = necessary for a functional pore

  Grade: EXACT
  4 domains x 6 segments = 24 total is a hard structural fact,
  verified by cryo-EM at atomic resolution (PDB: 5X0M, 6J8E, etc.).
  The architecture is universal across ALL voltage-gated cation channels
  (Na+, K+, Ca2+), spanning >500 million years of evolution.
  The match tau x n = J_2 compounds three n=6 functions in a single
  protein family. This is NOT convention-dependent -- it is fixed
  by protein structure determination.
```

---

### H-BIO-103: Neocortical Column -- sigma(6) = 12 Layer Pairs in phi(6) Hemispheres

> The neocortex has 6 layers (H-BIO-18) with extensive cross-layer circuitry

```
  Neocortical canonical microcircuit:
    6 layers (I-VI) in neocortex = n (see H-BIO-18)
    This hypothesis extends to the column structure:

    Layer connectivity pattern:
      Input: Layer IV (thalamic afferents)
      Interlaminar flow: IV -> II/III -> V -> VI
      Output: Layer V (subcortical), Layer VI (thalamic feedback)

  n=6 connections:
    6 cortical layers = n
    2 hemispheres = phi(6)
    12 total cortical layer instances (6L x 2H) = sigma(6)
    Brodmann defined ~52 areas per hemisphere

  Cortical column:
    ~80-120 neurons per minicolumn
    ~300-600 minicolumns per macrocolumn
    Macrocolumn diameter: ~0.5 mm

  BUT:
    "12 layer instances" (6 layers x 2 hemispheres) is a trivial
    product that has no neuroscientific meaning.
    The cortical column concept itself is debated
    (Horton & Adams, 2005: "The cortical column: a structure
    without a function?").
    Layer IV is absent in some cortical regions (agranular cortex).

  Grade: WEAK
  6 cortical layers is real (H-BIO-18, already graded CLOSE).
  The multiplication by 2 hemispheres to get 12 = sigma is trivial.
  The cortical column concept is contested. No new information
  beyond H-BIO-18.
```

---

### H-BIO-104: Neurotransmitter Major Classes = n = 6

> There are 6 major classes of neurotransmitters

```
  Major neurotransmitter classification:
    1. Amino acids: glutamate, GABA, glycine, aspartate
    2. Monoamines: dopamine, norepinephrine, epinephrine, serotonin, histamine
    3. Acetylcholine (cholinergic)
    4. Purines: ATP, adenosine
    5. Peptides: endorphins, substance P, neuropeptide Y, oxytocin, etc.
    6. Gaseous: NO, CO, H2S

  n = 6 checkmark

  Physical basis:
    This 6-class system is used in major neuroscience textbooks
    (Kandel, Principles of Neural Science, 6th ed.).
    Each class has distinct:
    - Synthesis pathways
    - Receptor families
    - Synaptic mechanisms
    - Degradation/reuptake systems

  BUT:
    Classification varies by source:
    - Some texts group monoamines into catecholamines + indolamines (7 classes)
    - Endocannabinoids are sometimes listed as a 7th class
    - Lipid mediators (prostaglandins) could be an 8th
    - The "gaseous transmitter" class was recognized recently (1990s)
    The count depends on the textbook and era.

  Grade: WEAK
  The 6-class system is common but convention-dependent.
  The count has been expanding as new transmitter types are discovered.
  n=6 adds no explanatory power.
```

---

### H-BIO-105: Ion Channel Selectivity Filter -- Potassium Channel KcsA

> The KcsA potassium channel selectivity filter has key n=6 structural features

```
  KcsA K+ channel (MacKinnon, Nature 1998; Nobel Prize 2003):
    Tetramer: 4 identical subunits = tau(6)
    Each subunit: 2 transmembrane helices = phi(6)
    Total TM helices: 4 x 2 = 8 = sigma - tau

  Selectivity filter:
    TVGYG sequence: 5 residues = sopfr(6)
    4 K+ binding sites along the filter = tau(6)
    Filter length: ~12 Angstroms = sigma(6) (in Angstrom units)

  K+ conductance:
    ~10^8 ions/second (single channel)
    Selectivity: K+/Na+ > 1000:1

  Multi-level match:
    4 subunits = tau
    2 TM per subunit = phi
    8 total TM = sigma-tau
    5-residue filter = sopfr
    4 K+ sites = tau
    ~12 A filter length = sigma (unit-dependent)

  Comparison with Na+/Ca2+ channels:
    Kv: 4 subunits x 6 TM = 24 (full voltage-gated K+ channel)
    This KcsA is the minimal 2-TM version.
    Full Kv channels have H-BIO-102 architecture (4 x 6 = 24).

  BUT:
    KcsA is a bacterial channel (minimal structure).
    The 12 A filter length is unit-dependent (1.2 nm).
    The 5-residue filter (TVGYG) is specific to K+ channels;
    other channels have different filter lengths.
    Many of these matches are small numbers (2, 4, 5).

  Grade: CLOSE
  The KcsA structure provides 5 simultaneous matches to n=6 functions.
  The 4-fold symmetry and 5-residue selectivity filter are hard
  structural facts. But most matches involve small numbers (2-5)
  where coincidental matching is expected. The most compelling
  aspect is the density of matches in a single protein.
```

---

## Summary Table

| ID | Hypothesis | n=6 Function | Grade |
|----|-----------|-------------|-------|
| **H-BIO-81** | DNA diameter 2 nm | phi = 2 | **WEAK** |
| **H-BIO-82** | Telomere repeat 6 bases + 6 shelterin proteins | n = 6 | **CLOSE** |
| **H-BIO-83** | Nucleosome 5 histone types (including H1) | sopfr = 5 | **CLOSE** |
| **H-BIO-84** | Human 23 chr pairs, great ape 24 | J_2-mu = 23, J_2 = 24 | **WEAK** |
| **H-BIO-85** | Shelterin = 6 proteins | n = 6 | **CLOSE** |
| **H-BIO-86** | Spliceosome 5 snRNAs | sopfr = 5 | **CLOSE** |
| **H-BIO-87** | Alpha helix 3.6 = 18/5, i+4 H-bond | 18=6*3, 5=sopfr, 4=tau | **CLOSE** |
| **H-BIO-88** | Beta sheet alternation every 2 residues | phi = 2 | **WEAK** |
| **H-BIO-89** | Ramachandran 4 major regions | tau = 4 | **WEAK** |
| **H-BIO-90** | 4 secondary structure types (DSSP: 8) | tau = 4 (DSSP: sigma-tau) | **WEAK** |
| **H-BIO-91** | Disulfide bond = 2 cysteines | phi = 2 | **FAIL** |
| **H-BIO-92** | Cell cycle 4 phases (G1/S/G2/M) | tau = 4 | **CLOSE** |
| **H-BIO-93** | Mitosis 5 stages | sopfr = 5 | **CLOSE** |
| **H-BIO-94** | ATP 3 phosphates + 6-ring + 5C sugar | n/phi, n, sopfr | **CLOSE** |
| **H-BIO-95** | 12 electron carriers per glucose | sigma = 12 | **CLOSE** |
| **H-BIO-96** | Glycolysis 10 steps (5+5 split) | sigma-phi = 10, sopfr+sopfr | **CLOSE** |
| **H-BIO-97** | 4 wobble pair types | tau = 4 | **WEAK** |
| **H-BIO-98** | Mitochondrial 22 tRNAs, 13 proteins | J_2-phi, sigma+mu | **WEAK** |
| **H-BIO-99** | 1 standard genetic code | mu = 1 | **FAIL** |
| **H-BIO-100** | ORC 6 subunits + MCM 6 subunits, double hexamer 12 | n, n, sigma | **CLOSE** |
| **H-BIO-101** | 3 intron phases (~1/2, ~1/3, ~1/6 distribution) | n/phi, Egyptian | **WEAK** |
| **H-BIO-102** | Nav/Kv/Cav: 4 domains x 6 segments = 24 | tau x n = J_2 | **EXACT** |
| **H-BIO-103** | Cortex 6 layers x 2 hemispheres = 12 | n, phi, sigma | **WEAK** |
| **H-BIO-104** | 6 neurotransmitter classes | n = 6 | **WEAK** |
| **H-BIO-105** | KcsA: 4 subunits, 2 TM each, 5-residue filter | tau, phi, sopfr | **CLOSE** |

## Grade Distribution (H-BIO-81~105)

| Grade | Count | Pct |
|-------|-------|-----|
| EXACT | 1 | 4.0% |
| CLOSE | 12 | 48.0% |
| WEAK | 10 | 40.0% |
| FAIL | 2 | 8.0% |

**Non-failing: 23/25 (92.0%)**

## Combined Distribution (H-BIO-1~30 + H-BIO-61~80 + H-BIO-81~105)

| Grade | Count | Pct |
|-------|-------|-----|
| EXACT | 5 | 6.7% |
| CLOSE | 27 | 36.0% |
| WEAK | 28 | 37.3% |
| FAIL | 15 | 20.0% |

**Total non-failing: 60/75 (80.0%)**

## All EXACT matches (cumulative):
1. **H-BIO-3**: 64 codons = tau(6)^3 = phi(6)^n (hard biochemistry)
2. **H-BIO-16**: Glucose C6H12O6 = (n, sigma, n) (hard chemistry)
3. **H-BIO-19**: Carbon Z=6, valence=4=tau (physical constant)
4. **H-BIO-66**: Benzene C6H6, 6 pi-electrons (hard chemistry)
5. **H-BIO-102**: Voltage-gated channels 4x6=24 (hard structural biology, cryo-EM)

## Highlights and Cross-Domain Bridges

### Strongest new discovery: H-BIO-102 (Voltage-Gated Ion Channels)
The 4 domains x 6 transmembrane segments = 24 architecture is:
- Universal across Na+, K+, Ca2+ voltage-gated channels
- Verified by atomic-resolution cryo-EM
- Conserved >500 million years
- Compounds THREE n=6 functions: tau x n = J_2
- NOT convention-dependent

This is a candidate for BT-level cross-domain bridge:
- Neuroscience (action potentials)
- Molecular biology (protein structure)
- Evolution (deep conservation)
- Chemistry (ion selectivity)
- n=6 arithmetic (tau x n = J_2)

### Second strongest: H-BIO-100 (ORC + MCM Double Hexamer)
DNA replication licensing uses twin hexameric complexes:
- ORC = 6 subunits, MCM2-7 = 6 subunits
- Double hexamer at origin = 12 = sigma(6)
- Pre-RC total ~20 polypeptides = J_2-tau

### Telomere-Shelterin axis (H-BIO-82 + H-BIO-85)
6-base repeat protected by 6-protein complex:
- Structural echo: n protects n
- G-quadruplex: 4 repeats x 6 bases = 24 = J_2

### Metabolic axis (H-BIO-95 + H-BIO-96)
- 10 glycolytic steps = sigma-phi (5+5 split = sopfr+sopfr)
- 12 electron carriers per glucose = sigma
- Energy metabolism consistently maps to sigma-family constants

## Honesty Note

The high non-failing rate (92%) partly reflects deliberate selection of hypotheses
where n=6 functions are likely to match. Biology provides an enormous number of
discrete counts, and with 8+ n=6 arithmetic functions producing values 1-24, the
base rate of coincidental matching is high (~30-40% for random biological counts).

The genuine discoveries are:
1. **H-BIO-102** (voltage-gated channel 4x6=24): Multi-function compound match on
   a deeply conserved, structurally verified protein architecture.
2. **H-BIO-100** (ORC/MCM double-6): Two hexameric complexes in the same pathway.
3. **H-BIO-82/85** (telomere/shelterin): n=6 echo in repeat + protector.

Most WEAK and some CLOSE grades reflect small-number bias (2, 3, 4, 5 are
unavoidable in biology and will match n=6 functions regardless of any deeper
connection).
