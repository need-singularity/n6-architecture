# N6 Biology Extreme Hypotheses -- H-BIO-61~80

> Extension of H-BIO-1~30. Cross-applying TECS-L discoveries to deeper biology.
> Exploring molecular structure, evolutionary constants, neural architecture,
> and ecological patterns through n=6 arithmetic.

> **Honesty principle**: The core 30 hypotheses yielded 3 EXACT and 10 CLOSE (73.3% non-fail).
> Biology's abundance of small integers (2-6) inflates coincidental matches.
> These extreme hypotheses probe deeper structures where cherry-picking is harder.

## Core Constants (review)

```
  n = 6          sigma(6) = 12     tau(6) = 4      phi(6) = 2
  sopfr(6) = 5   J_2(6) = 24       mu(6) = 1       lambda(6) = 2
  R(6) = 1       P_2 = 28 (second perfect number)
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

## TECS-L Cross-Reference Discoveries

```
  Verified biology-relevant matches from TECS-L:
    1. Carbon Z=6 = n (life's backbone element)
    2. Glucose C6H12O6 = (n, sigma, n) (energy currency)
    3. 64 codons = tau^3 = phi^n (genetic code)
    4. 6-membered rings: lowest strain, aromatic stability
    5. Neocortex 6 layers (conserved mammalian architecture)
    6. Histone octamer 4x2 = tau*phi (chromatin structure)
```

---

## Category X: Deep Molecular Biology

---

### H-BIO-61: Genetic Code Degeneracy Classes = Divisors of 12 = sigma(6)

> The codon degeneracy values {1, 2, 3, 4, 6} are all divisors of 12 = sigma(6)

```
  Standard genetic code degeneracy classes:
    1 codon:  Met, Trp (2 amino acids)
    2 codons: Phe, Tyr, His, Gln, Asn, Lys, Asp, Glu, Cys (9 amino acids)
    3 codons: Ile (1 amino acid)
    4 codons: Val, Pro, Thr, Ala, Gly (5 amino acids)
    6 codons: Leu, Ser, Arg (3 amino acids)

  Degeneracy set: {1, 2, 3, 4, 6}
  Divisors of 12: {1, 2, 3, 4, 6, 12}

  The degeneracy values are exactly the divisors of 12 minus 12 itself.
  Divisors of sigma(6) = divisors of 12 = {1, 2, 3, 4, 6, 12}.

  Physical basis:
    The 4-base, 3-position codon structure constrains degeneracy.
    With wobble pairing, the maximum per amino acid is 6 (two codon
    boxes of 4, split as 2+2 or used as 4). The available values
    are determined by how the 64 codons partition among 20+3 targets.

  BUT:
    The set {1,2,3,4,6} = divisors of 12 minus {12} is also
    divisors of many other numbers (e.g., divisors of 6 = {1,2,3,6},
    which is close but misses 4).
    The value "4" breaks the "divisors of 6" pattern.
    The connection to sigma(6)=12 is indirect.

  Grade: WEAK
  The degeneracy classes being divisors of 12 is a real observation,
  but it's explained by codon table structure, not number theory.
  The "divisors of sigma(6)" framing is post-hoc.
```

---

### H-BIO-62: tRNA Cloverleaf -- tau(6) = 4 Arms

> The tRNA secondary structure has 4 arms (acceptor, D, anticodon, T-psi-C)

```
  tRNA cloverleaf structure:
    1. Acceptor arm (3' CCA end, carries amino acid)
    2. D arm (dihydrouridine loop)
    3. Anticodon arm (contains anticodon triplet)
    4. T-psi-C arm (contains modified bases)
    + Variable loop (present in some tRNAs)

  tau(6) = 4 checkmark (if you count the 4 main arms)

  Physical basis:
    The 4-arm structure is universal across all known tRNAs.
    Crystal structures confirm the L-shaped 3D fold with
    these 4 structural domains (Kim et al., 1974).
    The variable loop is not considered a "arm" in most descriptions.

  BUT:
    If you count the variable loop: 5 structural elements.
    The 4-arm description is standard but the variable loop
    introduces ambiguity.
    tau(6) = 4 is common (tau(n)=4 for many n).

  Grade: CLOSE
  The 4-arm cloverleaf is well-established and structurally verified.
  The count 4 is robust (variable loop is not a stem-loop in all tRNAs).
```

---

### H-BIO-63: Phospholipid Headgroup Diversity -- sopfr(6) = 5 Major Types

> Cell membranes use 5 major phospholipid headgroups

```
  Major membrane phospholipids:
    1. Phosphatidylcholine (PC) -- most abundant in eukaryotes
    2. Phosphatidylethanolamine (PE)
    3. Phosphatidylserine (PS)
    4. Phosphatidylinositol (PI)
    5. Sphingomyelin (SM)

  sopfr(6) = 5 checkmark

  BUT:
    This list is one common grouping. Others include:
    - Cardiolipin (mitochondrial membranes)
    - Phosphatidylglycerol (bacterial membranes)
    - Cholesterol (not a phospholipid but major membrane component)
    Adding cardiolipin gives 6, adding cholesterol gives 7.
    The "5 major types" is a textbook convention for eukaryotic
    plasma membranes specifically.

  Grade: WEAK
  Convention-dependent. The count varies by membrane type and organism.
  5 is a common textbook simplification, not a universal constant.
```

---

### H-BIO-64: Nucleosome Repeat Length ~200 bp -- Connections?

> The nucleosome repeat length is ~200 bp (147 wrapped + ~50 linker)

```
  Nucleosome structure:
    Core particle: 147 bp wrapped around histone octamer
    Linker DNA: 20-80 bp (average ~50 bp)
    Repeat length: ~200 bp (varies by organism/cell type)

  n=6 attempts:
    147 bp = ? No clean n=6 decomposition.
    200 bp = ? Not a clean n=6 expression.
    147 ≈ 6 * 24 + 3 = 147? (6 * J_2 + n/phi = 147) -- forced.

  Grade: FAIL
  No clean connection. The 147 bp number comes from the geometry
  of DNA wrapping (~1.65 turns around the octamer, determined by
  the histone surface curvature). The linker length varies.
```

---

### H-BIO-65: 6-fold Symmetry in Viral Capsids

> Many viral capsids exhibit 6-fold (hexameric) symmetry elements

```
  Viral capsid architecture (Caspar-Klug theory, 1962):
    Icosahedral capsids have T-number classification.
    T=1: 60 subunits, 12 pentamers only.
    T>1: 12 pentamers + 10(T-1) hexamers.

    For T>1 capsids, hexamers (6-fold local symmetry) dominate:
    - T=3: 12 pentamers + 20 hexamers (180 subunits)
    - T=4: 12 pentamers + 30 hexamers (240 subunits)
    - T=7: 12 pentamers + 60 hexamers (420 subunits)

  n = 6 connections:
    Hexamers have 6-fold symmetry = n
    12 pentamers per capsid = sigma(6)
    T=1 has 60 subunits = sigma * sopfr
    T=3 has 180 subunits = 60 * 3 = 60 * n/phi

  Physical basis:
    The hexagonal arrangement of protein subunits is the most
    efficient way to tile a closed surface (combined with 12 pentamers
    for Gaussian curvature, per Euler's polyhedron formula).
    The 12 pentamers is forced by topology (V - E + F = 2).

  Grade: CLOSE
  Hexameric symmetry (6-fold) is genuinely favored in capsid assembly.
  The 12 pentamers = sigma(6) is topologically forced (Euler formula).
  But hexagonal packing is a geometric universal, not specific to n=6.
```

---

### H-BIO-66: Benzene Ring -- 6 Carbons, 6 H, 6 pi-electrons

> Benzene (C6H6) has triple-6 structure: 6C, 6H, 6 delocalized pi-electrons

```
  Benzene (C6H6):
    6 carbon atoms in a planar ring
    6 hydrogen atoms (one per carbon)
    6 pi electrons (Huckel: 4*1+2 = 6 for n=1)
    Bond order: 1.5 (delocalized)

  n = 6 triple match (C=6, H=6, pi-e=6)

  Physical basis:
    Benzene's extraordinary stability comes from aromatic delocalization.
    The 6 pi-electrons satisfy Huckel's rule (4n+2, n=1).
    The 6-membered ring is the smallest aromatic carbocycle.

  Biological relevance:
    Phenylalanine, tyrosine, tryptophan (aromatic amino acids)
    All nucleotide bases contain aromatic rings
    Drug design heavily uses benzene scaffolds

  BUT:
    Benzene's 6-ness is a chemistry fact, not a biology fact.
    The connection to n=6 is through carbon chemistry, not through
    biological selection. Still, benzene is foundational to biochemistry.

  Grade: EXACT
  C6H6 with 6 pi-electrons is a hard chemical fact.
  All three 6s are physically determined (ring size, H count,
  Huckel rule). This is chemistry, not convention.
```

---

### H-BIO-67: Codon Table Egyptian Fraction -- 1/2 + 1/3 + 1/6 = 1

> Amino acid degeneracy partitions approximately as 1/2 + 1/3 + 1/6

```
  Codon table statistics:
    20 amino acids + 3 stops = 23 entries
    2-fold degenerate: 9 amino acids (39% of 23 ≈ "~1/3"?)
    4-fold degenerate: 5 amino acids (22% of 23)
    6-fold degenerate: 3 amino acids (13% of 23)
    1-fold: 2 amino acids (9%)
    3-fold: 1 amino acid (4%)
    stops: 3 (13%)

  Egyptian fraction attempt:
    No clean partition into 1/2 + 1/3 + 1/6 = 1.
    The degeneracy distribution does not obviously follow the
    Egyptian fraction of 6.

  Grade: FAIL
  The codon table degeneracy distribution does not partition as
  1/2 + 1/3 + 1/6. Forced fitting of the Egyptian fraction.
```

---

### H-BIO-68: DNA Helix Parameters -- 10 bp/turn ≈ 2*sopfr? or sigma-phi?

> B-form DNA has ~10 base pairs per helical turn

```
  DNA helix geometry (B-form):
    10.4 bp per turn (more precisely 10.5 in solution)
    Rise per bp: 0.34 nm
    Pitch: 3.4 nm
    Major groove: ~2.2 nm
    Minor groove: ~1.2 nm

  n=6 attempts:
    10.4 ≈ 10 = sigma(6) - phi(6) = 12 - 2? (4% off)
    10.4 ≈ 2 * sopfr(6) = 10 (4% off)
    3.4 nm pitch = ? No match.
    Minor groove 1.2 nm ≈ sigma(6)/10? (forced unit manipulation)

  BUT:
    10.4 bp/turn is determined by the stacking geometry of base pairs
    and the deoxyribose-phosphate backbone angles. It's a geometric
    property of the double helix, not a number-theoretic quantity.
    The "10" is approximate and varies with sequence and conditions
    (A-form: 11 bp/turn, Z-form: 12 bp/turn).

  Grade: FAIL
  10.4 bp/turn is an approximate geometric quantity. No clean n=6 match.
  The value varies by DNA form and conditions.
```

---

### H-BIO-69: Ribosome Subunits -- phi(6) = 2 (large + small)

> Ribosomes consist of 2 subunits (large and small)

```
  Ribosome structure:
    Prokaryotic: 30S (small) + 50S (large) = 70S
    Eukaryotic: 40S (small) + 60S (large) = 80S
    All ribosomes have exactly 2 subunits.

  phi(6) = 2 checkmark

  Physical basis:
    The 2-subunit architecture is universal and functionally required:
    - Small subunit: decoding (codon-anticodon matching)
    - Large subunit: peptidyl transferase (catalysis)
    Subunits associate during translation, dissociate after.

  BUT:
    phi(6) = 2 is trivially small. Any 2-part structure matches.
    The ribosome's 2-subunit design is explained by the need for
    regulated assembly/disassembly and distinct functional roles.

  Grade: WEAK
  2 subunits is real and universal but phi(6)=2 is too trivial.
```

---

### H-BIO-70: Standard Amino Acid Codons -- sigma(6) * sopfr(6) + 1 = 61 Sense Codons

> 64 - 3 = 61 sense codons. 61 = sigma * sopfr + mu = 12*5 + 1

```
  Sense codons:
    64 total - 3 stops = 61 codons encoding amino acids.

  61 = sigma(6) * sopfr(6) + mu(6) = 60 + 1

  BUT:
    61 is a prime number. The formula 60+1 = sigma*sopfr + mu
    combines three n=6 functions arbitrarily.
    The sense codon count 61 is a consequence of 64 codons - 3 stops,
    both of which are already accounted for in H-BIO-3 and H-BIO-5.
    Not independent.

  Grade: FAIL
  Derivative of H-BIO-3 and H-BIO-5. The formula is forced.
```

---

### H-BIO-71: 6 Classes of Enzymes (EC Classification)

> The Enzyme Commission classifies enzymes into 6 major classes

```
  EC enzyme classification:
    EC 1: Oxidoreductases
    EC 2: Transferases
    EC 3: Hydrolases
    EC 4: Lyases
    EC 5: Isomerases
    EC 6: Ligases
    (EC 7: Translocases -- added in 2018)

  n = 6 if using the original classification

  Physical basis:
    The original 6 classes (1961, IUB) covered all known reaction types.
    EC 7 (translocases) was added in 2018 for membrane transport enzymes.
    The current standard has 7 classes.

  BUT:
    The original 6-class system was a human convention.
    It has already been updated to 7 classes.
    Classification boundaries are arbitrary (why is "translocation"
    not a subclass of "transfer"?).

  Grade: WEAK
  The original 6-class system was widely used for decades and reflected
  a genuine attempt to categorize reaction types. But it is now 7 classes,
  and the boundaries are convention-dependent.
```

---

### H-BIO-72: Amino Acid Codon Box Structure -- 4x4 = 16 codon boxes

> The codon table is organized as 16 codon boxes (4 first-position x 4 second-position)

```
  Codon table structure:
    First position: 4 choices (U, C, A, G)
    Second position: 4 choices (U, C, A, G)
    Each (1st, 2nd) combination defines a "codon box" of 4 codons
    (varying the 3rd position). Total: 4 x 4 = 16 boxes.

  16 = tau(6)^2 = 4^2 checkmark

  Physical basis:
    The 16-box structure is a direct consequence of the 4-base alphabet.
    Within each box, the 4 codons often encode the same or related
    amino acids (wobble hypothesis, Crick 1966).

  BUT:
    16 = 4^2 is trivially derived from H-BIO-1 (4 bases).
    tau(6)^2 for codon boxes is not independent of tau(6) for bases.

  Grade: WEAK
  Numerically correct but entirely derivative of the 4-base alphabet.
  Not an independent match.
```

---

### H-BIO-73: Hexokinase and 6-Carbon Substrate Specificity

> Hexokinase specifically phosphorylates 6-carbon sugars

```
  Hexokinase (EC 2.7.1.1):
    Catalyzes: glucose + ATP -> glucose-6-phosphate + ADP
    Substrate: hexoses (6-carbon sugars) -- glucose, fructose, mannose
    The enzyme name literally means "hexose kinase"

  n = 6 checkmark (substrate carbon count)

  Physical basis:
    Hexokinase's active site is shaped to accommodate 6-carbon sugars.
    The induced-fit mechanism (Koshland) specifically recognizes hexose geometry.
    Glucokinase (hexokinase IV) is even more specific to glucose.

  Connection:
    This is a consequence of H-BIO-16 (glucose C6H12O6).
    The enzyme exists because the substrate has 6 carbons.
    Not independent.

  Grade: WEAK
  Derivative of glucose being a hexose (H-BIO-16).
  The enzyme specificity follows from the substrate.
```

---

### H-BIO-74: Purine Ring = sopfr(6) + tau(6) = 9 atoms

> Purine bases (A, G) have a 9-membered ring system (9 atoms in the bicyclic structure)

```
  Purine ring structure:
    Bicyclic: 6-membered pyrimidine ring fused with 5-membered imidazole ring
    Shared edge: 2 atoms
    Total ring atoms: 6 + 5 - 2 = 9 (5C + 4N)

  sopfr(6) + tau(6) = 5 + 4 = 9 checkmark

  Also:
    6-membered ring + 5-membered ring = n + sopfr fused
    9 ring atoms = 5 carbon + 4 nitrogen = sopfr + tau

  BUT:
    The formula sopfr + tau = 9 is ad hoc.
    The bicyclic structure is determined by the chemistry of
    imidazole-pyrimidine fusion, not by number theory.
    9 = 3^2 has simpler explanations.

  Grade: WEAK
  9 ring atoms is correct but the sopfr+tau formula is arbitrary.
  The ring structure is explained by organic chemistry, not n=6.
```

---

### H-BIO-75: 6 Trophic Levels (Ecological Maximum)

> Ecosystems rarely sustain more than 5-6 trophic levels

```
  Trophic levels in ecology:
    1. Primary producers (plants, phytoplankton)
    2. Primary consumers (herbivores)
    3. Secondary consumers (small predators)
    4. Tertiary consumers (large predators)
    5. Quaternary consumers (apex predators)
    6. Decomposers (fungi, bacteria)

  n = 6 if you include decomposers as a level

  Physical basis:
    Energy transfer efficiency: ~10% per trophic level (Lindeman, 1942).
    After 5-6 levels, insufficient energy remains to support a population.
    This is a thermodynamic constraint (2nd law of thermodynamics).

  BUT:
    Most ecosystems have 3-5 trophic levels, not 6.
    Decomposers are often classified separately (not a "level" in the
    traditional linear chain model).
    Marine food webs can have 6-7 levels; terrestrial typically 4-5.
    The "maximum 6" requires including decomposers, which is non-standard.

  Grade: WEAK
  The 10% efficiency rule does constrain trophic levels to ~4-6, but
  the exact maximum varies by ecosystem. Getting 6 requires including
  decomposers as a trophic level, which is non-standard in ecology.
```

---

### H-BIO-76: DNA Sugar -- Deoxyribose = 5-Carbon Ring = sopfr(6)

> The DNA backbone sugar (deoxyribose) has 5 carbon atoms

```
  2-Deoxyribose (C5H10O4):
    5-carbon sugar (pentose)
    Forms a 5-membered furanose ring in DNA
    Missing the 2'-hydroxyl compared to ribose

  sopfr(6) = 5 checkmark

  Also:
    Ribose (RNA sugar) also has 5 carbons = sopfr(6)
    Both DNA and RNA use pentose sugars.

  Physical basis:
    Pentose sugars provide the structural backbone of nucleic acids.
    The 5-carbon framework is fixed by chemistry:
    - Furanose ring (5-membered) is thermodynamically favored over
      the open chain form.
    - Hexose sugars (6C) in nucleic acids would alter the backbone
      geometry, affecting base stacking and helix formation.

  BUT:
    Pentoses are a general class; the "5" comes from basic
    carbohydrate chemistry, not from n=6.
    sopfr(n)=5 for many n (6, 12, 18, 20, ...).

  Grade: CLOSE
  5 carbons in deoxyribose/ribose is a hard chemical fact.
  The pentose backbone is universal in all nucleic acids.
  sopfr(6) = 5 is a clean match, though 5 is a small number.
```

---

### H-BIO-77: Amino Acid Chirality -- L-form = mu(6) = 1 Handedness

> Life uses almost exclusively L-amino acids (1 chirality)

```
  Amino acid chirality:
    All standard amino acids (except glycine) are L-form.
    D-amino acids are extremely rare in biology.
    This homochirality is one of life's deepest mysteries.

  mu(6) = 1 (one handedness) checkmark

  BUT:
    mu(6) = 1 matching "one chirality" is trivially true.
    Any unique choice matches 1. D-sugars are also the sole chirality
    in biology (so both amino acid and sugar chirality "match" 1).
    The interesting question is WHY one chirality, not that there IS one.

  Grade: FAIL
  "1" matches any unique choice. Zero specificity.
  The homochirality problem is fascinating but has nothing to do with n=6.
```

---

### H-BIO-78: Photosystem Count -- phi(6) = 2 Photosystems

> Oxygenic photosynthesis uses 2 photosystems (PSI and PSII)

```
  Photosystems:
    Photosystem I (P700): reduces NADP+ to NADPH
    Photosystem II (P680): oxidizes water, produces O2
    Z-scheme: PSII -> cytochrome b6f -> PSI (linear electron flow)

  phi(6) = 2 checkmark

  Physical basis:
    The 2-photosystem arrangement enables water oxidation (+0.82V)
    coupled to NADP+ reduction (-0.32V). Neither photosystem alone
    can span the full ~1.14V potential difference efficiently.
    Anoxygenic photosynthesis uses only 1 photosystem.

  BUT:
    phi(6) = 2 is trivially small. Any pair of complementary systems
    "matches" 2. The 2-photosystem design is thermodynamically motivated
    (need 2 photon absorption events per electron for the voltage span),
    which is physics, not number theory.

  Grade: WEAK
  2 photosystems is real and thermodynamically justified, but 2 is
  too trivial for phi(6) to add any insight.
```

---

### H-BIO-79: Insulin -- 2 Chains = phi(6), 6 Molecules per Hexamer = n

> Insulin has 2 polypeptide chains and forms hexamers (6 molecules)

```
  Insulin structure:
    Mature insulin: chain A (21 residues) + chain B (30 residues)
    Linked by 2 disulfide bonds (+ 1 intrachain in A)
    Storage form: hexamer (6 insulin molecules + 2 Zn2+ ions)
    Active form: monomer

  phi(6) = 2 chains, n = 6 molecules per hexamer

  Physical basis:
    The 2-chain structure results from proinsulin processing
    (C-peptide excision). This is a specific evolutionary result.
    The hexamer is stabilized by zinc coordination and is the
    storage/secretion form in beta-cell granules.

  The hexameric storage is notable:
    6 monomers arranged with 3-fold and 2-fold symmetry axes.
    2 Zn2+ ions on the 3-fold axis.

  BUT:
    Many proteins form hexamers (glutamine synthetase, helicase).
    6-fold symmetry is geometrically common.
    The 2-chain structure is specific to insulin, not a general principle.

  Grade: CLOSE
  The insulin hexamer (6 molecules) is a hard structural fact
  verified by X-ray crystallography (PDB: 4INS). The 2-chain
  structure is also fixed. But hexamerization is common in proteins
  and the 2-chain structure is insulin-specific, not a general rule.
```

---

### H-BIO-80: Carbon Valence Network -- tau(6) = 4 Bonds Enable Life

> Carbon's 4 valence bonds (= tau(6)) are the necessary and sufficient condition for biochemical complexity

```
  Carbon bonding:
    4 valence electrons -> 4 covalent bonds maximum
    sp3: tetrahedral (saturated chains, amino acids)
    sp2: trigonal planar (double bonds, aromatic rings)
    sp1: linear (triple bonds, rare in biology)

  tau(6) = 4 checkmark

  Physical argument:
    Why 4 bonds matter:
    - 1-2 bonds (H, O, N): cannot form complex branching structures
    - 3 bonds (N, B): possible but less versatile (N is a key partner though)
    - 4 bonds (C, Si): maximum for period 2; enables chains, branches, rings
    - 5+ bonds: requires d-orbitals (period 3+), bonds are weaker

    Carbon's 4 bonds uniquely enable:
    - Long stable chains (unlike Si-Si bonds which are weaker)
    - Multiple bonding modes (sp3/sp2/sp)
    - Chiral centers (4 different substituents)
    - Ring formation (strain-free at 5-6 members)

  Connection to H-BIO-19:
    This extends H-BIO-19 (carbon Z=6 = n) by focusing on
    the valence = tau(6) = 4 aspect. The combination of
    Z = n and valence = tau(n) in the same element is notable.

  Grade: CLOSE
  Carbon's 4-valence enabling biochemical complexity is well-established.
  The match tau(6) = 4 is exact, and the combination with Z = n = 6
  strengthens the connection. But this is essentially H-BIO-19 extended,
  not fully independent. And tau(n)=4 for many n.
```

---

## Summary Table

| ID | Hypothesis | n=6 Function | Grade |
|----|-----------|-------------|-------|
| H-BIO-61 | Codon degeneracy classes = divisors of 12 | sigma(6) = 12 | **WEAK** |
| H-BIO-62 | tRNA 4 arms | tau(6) = 4 | **CLOSE** |
| H-BIO-63 | 5 phospholipid headgroups | sopfr(6) = 5 | **WEAK** |
| H-BIO-64 | Nucleosome repeat ~200 bp | -- | **FAIL** |
| H-BIO-65 | Viral capsid hexamers + 12 pentamers | n=6, sigma=12 | **CLOSE** |
| H-BIO-66 | Benzene C6H6 + 6 pi-electrons | n = 6 | **EXACT** |
| H-BIO-67 | Codon table Egyptian fraction | 1/2+1/3+1/6 | **FAIL** |
| H-BIO-68 | DNA 10 bp/turn | -- | **FAIL** |
| H-BIO-69 | Ribosome 2 subunits | phi(6) = 2 | **WEAK** |
| H-BIO-70 | 61 sense codons | sigma*sopfr+mu | **FAIL** |
| H-BIO-71 | 6 enzyme classes (EC) | n = 6 | **WEAK** |
| H-BIO-72 | 16 codon boxes = tau^2 | tau(6)^2 = 16 | **WEAK** |
| H-BIO-73 | Hexokinase 6C specificity | n = 6 | **WEAK** |
| H-BIO-74 | Purine 9-atom ring | sopfr+tau = 9 | **WEAK** |
| H-BIO-75 | 6 trophic levels | n = 6 | **WEAK** |
| H-BIO-76 | Deoxyribose 5 carbons | sopfr(6) = 5 | **CLOSE** |
| H-BIO-77 | L-amino acid chirality = 1 | mu(6) = 1 | **FAIL** |
| H-BIO-78 | 2 photosystems | phi(6) = 2 | **WEAK** |
| H-BIO-79 | Insulin hexamer = 6 | n = 6 | **CLOSE** |
| H-BIO-80 | Carbon 4 valence bonds | tau(6) = 4 | **CLOSE** |

## Grade Distribution (H-BIO-61~80)

| Grade | Count | Pct |
|-------|-------|-----|
| EXACT | 1 | 5.0% |
| CLOSE | 5 | 25.0% |
| WEAK | 9 | 45.0% |
| FAIL | 5 | 25.0% |

**Non-failing: 15/20 (75.0%)**

## Combined Distribution (H-BIO-1~30 + H-BIO-61~80)

| Grade | Count | Pct |
|-------|-------|-----|
| EXACT | 4 | 8.0% |
| CLOSE | 15 | 30.0% |
| WEAK | 18 | 36.0% |
| FAIL | 13 | 26.0% |

**Total non-failing: 37/50 (74.0%)**

**All EXACT matches:**
1. H-BIO-3: 64 codons = tau(6)^3 (hard biochemistry)
2. H-BIO-16: Glucose C6H12O6 = (n, sigma, n) (hard chemistry)
3. H-BIO-19: Carbon Z=6 = n (physical constant)
4. H-BIO-66: Benzene C6H6, 6 pi-electrons (hard chemistry)

Note: 3 of 4 EXACT matches involve the number 6 in chemical structures
(carbon, glucose, benzene). This reflects that 6-membered carbon structures
are thermodynamically favored in chemistry, which is a genuine physical
fact independent of whether it connects to the perfect number n=6.
