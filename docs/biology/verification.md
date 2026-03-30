# N6 Biology Hypotheses -- Independent Verification

Verified: 2026-03-30
Method: Each hypothesis checked against established biochemistry (Lehninger, Alberts, Stryer),
molecular biology (Watson et al.), neuroanatomy (Kandel), and ecology (Ricklefs) textbooks.
Chemical data from NIST and PDB. Grades adjusted where warranted.

## Grade Distribution (Summary)

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 3 | 10.0% | H-BIO-3, H-BIO-16, H-BIO-19 |
| CLOSE | 10 | 33.3% | H-BIO-1, H-BIO-2, H-BIO-4, H-BIO-5, H-BIO-6, H-BIO-13, H-BIO-14, H-BIO-18, H-BIO-23, H-BIO-27 |
| WEAK | 9 | 30.0% | H-BIO-7, H-BIO-9, H-BIO-10, H-BIO-11, H-BIO-12, H-BIO-20, H-BIO-21, H-BIO-22, H-BIO-24 |
| FAIL | 8 | 26.7% | H-BIO-8, H-BIO-15, H-BIO-17, H-BIO-25, H-BIO-26, H-BIO-28, H-BIO-29, H-BIO-30 |
| UNVERIFIABLE | 0 | 0% | -- |

**Non-failing total: 22/30 (73.3%)**

Note: The high non-failing rate compared to superconductor (51.7%) reflects that biology has many
small discrete counts (2, 3, 4, 5, 6) that match n=6 functions by sheer probability. The 3 EXACT
matches (codons, glucose, carbon) are genuinely strong; most CLOSE matches exploit small-number bias.

| ID | Hypothesis | Grade |
|----|-----------|-------|
| H-BIO-1 | DNA bases = tau(6) = 4 | **CLOSE** |
| H-BIO-2 | Double helix = phi(6) = 2 strands | **CLOSE** |
| H-BIO-3 | 64 codons = tau(6)^3 | **EXACT** |
| H-BIO-4 | 20 amino acids = tau*sopfr | **CLOSE** |
| H-BIO-5 | 3 stop codons = n/phi | **CLOSE** |
| H-BIO-6 | 5 nucleotide bases = sopfr(6) | **CLOSE** |
| H-BIO-7 | Max codon degeneracy = 6 | **WEAK** |
| H-BIO-8 | 1 start codon = mu(6) | **FAIL** |
| H-BIO-9 | 20*3 = 60 = sigma*sopfr | **WEAK** |
| H-BIO-10 | 2 base pair types = phi(6) | **WEAK** |
| H-BIO-11 | 6 cell cycle phases | **WEAK** |
| H-BIO-12 | ATP ~7.3 kcal/mol ~ 7 | **WEAK** |
| H-BIO-13 | Histone octamer = tau*phi = 8 | **CLOSE** |
| H-BIO-14 | Reading frame = 3 = n/phi | **CLOSE** |
| H-BIO-15 | Membrane bilayer = 2 | **FAIL** |
| H-BIO-16 | Glucose C6H12O6 = (n, sigma, n) | **EXACT** |
| H-BIO-17 | 6 kingdoms of life | **FAIL** |
| H-BIO-18 | 6 cortical layers = n | **CLOSE** |
| H-BIO-19 | Carbon Z=6 = n | **EXACT** |
| H-BIO-20 | 4 amino acid groups | **WEAK** |
| H-BIO-21 | 3 RNA types = n/phi | **WEAK** |
| H-BIO-22 | Anticodon = 3 bases | **WEAK** |
| H-BIO-23 | 4 protein structure levels = tau(6) | **CLOSE** |
| H-BIO-24 | Nucleotide = 3 parts | **WEAK** |
| H-BIO-25 | Amino acid = 3 backbone groups | **FAIL** |
| H-BIO-26 | Krebs cycle 8 steps | **FAIL** |
| H-BIO-27 | 6-membered rings in biology | **CLOSE** |
| H-BIO-28 | Glycolysis 2 phases | **FAIL** |
| H-BIO-29 | Water = 3 atoms | **FAIL** |
| H-BIO-30 | Semiconservative = 2 strands | **FAIL** |

---

Grading scale:
- **EXACT**: The claimed number/structure matches real-world data precisely, with a legitimate physical basis.
- **CLOSE**: Within ~10% of real values, or directionally correct with a physically meaningful classification.
- **WEAK**: Requires cherry-picking, flexible counting, or post-hoc rationalization.
- **FAIL**: Contradicted by real-world data, trivially true of any number, or unit-dependent.
- **UNVERIFIABLE**: Insufficient published data to confirm or deny.

---

## H-BIO-1: DNA Bases = tau(6) = 4

**Grade: CLOSE** (confirmed)

DNA uses exactly 4 canonical bases: adenine, thymine, guanine, cytosine. This is universally conserved across all DNA-based life and is a hard biochemical fact, not a convention. The Watson-Crick base pairing system (purine-pyrimidine complementarity) requires an even number of bases in matched pairs, but does not specifically require 4.

The match tau(6) = 4 is numerically exact. However, tau(n) = 4 for n = 6, 8, 10, 14, 15, 21, 22, ... -- it is not specific to n=6. Synthetic biology has demonstrated functional 6-base (Hachimoji) and 8-base DNA systems, showing 4 is not a physical necessity. The original CLOSE grade is fair: 4 is genuinely fixed in natural biology, but the number is too small and too common to be compelling evidence of an n=6 pattern.

---

## H-BIO-2: Double Helix = phi(6) = 2 Strands

**Grade: CLOSE** (confirmed)

DNA's double-stranded structure was established by Watson and Crick (1953) and confirmed by X-ray crystallography (Franklin, Wilkins). All cellular life uses double-stranded DNA for its genome (though some viruses use ssDNA or ssRNA). The 2-strand structure enables semiconservative replication and error correction.

phi(6) = 2 matches, but phi(n) = 2 for n = 3, 4, 6, and "2" is the most ubiquitous integer in biology and physics. The double helix is genuinely fundamental, but attributing its 2-ness to phi(6) rather than basic polymer thermodynamics is a stretch. CLOSE is appropriate -- the number is real and important but too small to be specific.

---

## H-BIO-3: 64 Codons = tau(6)^3

**Grade: EXACT** (confirmed)

The genetic code comprises exactly 64 codons: 4 bases taken 3 at a time = 4^3 = 64. This has been experimentally verified by the complete decoding of the codon table (Nirenberg, Matthaei, Khorana, 1961-1966; Nobel Prize 1968). The number 64 is a hard fact of molecular biology, universal across all known life using standard DNA/RNA.

tau(6)^3 = 4^3 = 64 is exact. The factorization is clean: 4 = tau(6) is the base alphabet size, and the exponent 3 = n/phi(6) is the codon length (the minimum integer k such that 4^k >= 20 amino acids). This means 64 = tau(6)^(n/phi) which compounds two n=6 connections.

Additionally, 64 = 2^6 = phi(6)^n, providing a second n=6 factorization.

This is the strongest hypothesis in the biology domain. The count 64 is completely fixed by chemistry, not convention-dependent, and admits multiple clean n=6 decompositions. EXACT confirmed.

---

## H-BIO-4: 20 Standard Amino Acids = tau(6) * sopfr(6) = 20

**Grade: CLOSE** (confirmed)

The standard genetic code encodes exactly 20 amino acids. This is listed in every biochemistry textbook (Lehninger ch. 3, Stryer ch. 2). The 20 amino acids are: Gly, Ala, Val, Leu, Ile, Ser, Thr, Cys, Met, Pro, Phe, Tyr, Trp, Asp, Glu, Asn, Gln, Lys, Arg, His.

Complication: Selenocysteine (Sec, the 21st) is encoded by UGA + SECIS element in some organisms. Pyrrolysine (Pyl, 22nd) is found in some archaea. These are minor exceptions; the core 20 is universal.

tau(6) * sopfr(6) = 4 * 5 = 20 is a cleaner formula than J_2 - tau = 24 - 4 = 20. The factorization 20 = 4 * 5 reflects "4 bases, 5 = sum of prime factors," though no biological mechanism connects these.

CLOSE is appropriate: 20 is a hard number (modulo the 21st/22nd edge case), and the factorization tau * sopfr is clean. But 20 is a sufficiently common number that coincidental matches are expected.

---

## H-BIO-5: 3 Stop Codons = n/phi = 3

**Grade: CLOSE** (confirmed)

The standard genetic code has 3 stop codons: UAA (ochre), UAG (amber), UGA (opal/umber). These are recognized by release factors (eRF1 in eukaryotes, RF1/RF2 in prokaryotes) rather than by tRNAs.

Complication: Mitochondrial genetic codes deviate significantly. In vertebrate mitochondria, AGA and AGG are also stops (5 stops total). In Mycoplasma, UGA codes for Trp (2 stops). The "3 stops" is specific to the standard nuclear genetic code.

n/phi = 6/2 = 3 matches. But 3 is a small integer that appears everywhere in biology (triplet code, 3 domains of life, etc.). The stop codon count has a plausible evolutionary explanation (frozen accident) rather than a mathematical necessity. CLOSE is fair for the standard code, recognizing the mitochondrial variation caveat.

---

## H-BIO-6: 5 Nucleotide Bases = sopfr(6) = 5

**Grade: CLOSE** (confirmed)

Counting the 5 canonical bases across DNA and RNA: A, T, G, C (DNA) plus U (RNA, replacing T). Total distinct canonical bases = 5.

This is a reasonable count. T and U are chemically distinct (thymine = 5-methyluracil), so counting them separately is justified. Modified bases (m6A, m5C, pseudouridine, inosine, etc.) number over 100 but are post-transcriptional/post-replicative modifications, not part of the primary genetic alphabet.

sopfr(6) = 2 + 3 = 5 matches. However, sopfr(n) = 5 for n = 6, 12, 18, 20, 32, ... -- not unique to n=6. CLOSE is fair: the count is defensible and the number is genuinely fixed by the DNA/RNA chemistry distinction, but it's a small number and sopfr is not a prominent function.

---

## H-BIO-7: Maximum Codon Degeneracy = 6 = n

**Grade: WEAK** (confirmed)

Three amino acids (Leu, Ser, Arg) are each encoded by 6 codons -- the maximum degeneracy in the genetic code. The degeneracy classes are {1, 2, 3, 4, 6}: Met and Trp have 1 codon each; 9 amino acids have 2; Ile has 3; 5 amino acids have 4; and Leu/Ser/Arg have 6.

The maximum = 6 = n is real. But: (a) the set of degeneracy values {1,2,3,4,6} is not the divisors of 6 ({1,2,3,6}); (b) the maximum being 6 is a consequence of the codon table structure and wobble base pairing, not a fundamental constraint; (c) cherry-picking the maximum from a set of values is post-hoc. WEAK is fair.

---

## H-BIO-8: 1 Start Codon = mu(6) = 1

**Grade: FAIL** (confirmed)

AUG is the canonical start codon, but alternative start codons exist in both prokaryotes (GUG ~14%, UUG ~3% in E. coli) and eukaryotes (CUG, rare). Even in the standard description, "1 main start codon" is a simplification.

More fundamentally: mu(6) = 1 matches literally anything counted as "one." There is zero specificity. Any unique biological entity (1 sun, 1 Earth, 1 genetic code, 1 LUCA) "matches" mu(6) = 1. FAIL confirmed.

---

## H-BIO-9: 20 * 3 = 60 = sigma * sopfr

**Grade: WEAK** (confirmed)

20 amino acids times 3-letter codon = 60. sigma(6) * sopfr(6) = 12 * 5 = 60. Numerically correct.

But this product has no biological meaning. It is not a physical observable -- there is no experiment that measures "amino acids times codon length." The number 60 arises from multiplying two independently justified numbers. Any pair of numbers whose product is 60 could be retroactively matched. WEAK confirmed.

---

## H-BIO-10: 2 Watson-Crick Base Pair Types = phi(6)

**Grade: WEAK** (confirmed)

There are 2 Watson-Crick base pair types: A-T (2 H-bonds) and G-C (3 H-bonds). This is chemically fixed. But "2 types" is a trivial classification -- purine-pyrimidine pairing produces exactly 2 complementary pairs by construction (since we have 2 purines and 2 pyrimidines).

The observation about hydrogen bond counts {2, 3} summing to sopfr(6) = 5 is numerically cute but the H-bond counts are determined by the molecular geometry of the specific bases, not by any organizing principle. Non-Watson-Crick pairs (Hoogsteen, wobble) expand the space significantly. WEAK confirmed.

---

## H-BIO-11: 6 Cell Cycle Phases

**Grade: WEAK** (confirmed)

Standard textbooks (Alberts, Molecular Biology of the Cell, 7th ed.) describe the cell cycle as: G1, S, G2 (interphase, 3 phases) + M phase (mitosis). Mitosis itself is subdivided into prophase, prometaphase, metaphase, anaphase, telophase (5 subphases). Cytokinesis is often listed separately.

Getting "6 phases" requires choosing a specific lumping: G1 + S + G2 + prophase + metaphase + anaphase/telophase (combining the last two). Standard counts are: 4 (G1, S, G2, M), 7-8 (interphase phases + mitotic subphases), or 9 (adding cytokinesis). Six requires non-standard combination. WEAK confirmed.

---

## H-BIO-12: ATP Hydrolysis ~7.3 kcal/mol

**Grade: WEAK** (confirmed)

Standard free energy of ATP hydrolysis: DeltaG' = -7.3 kcal/mol (-30.5 kJ/mol) at pH 7.0, 25C, 1M reactants. Under physiological conditions: DeltaG ~ -12 to -14 kcal/mol (Lehninger, 8th ed.).

sigma(6) - sopfr(6) = 12 - 5 = 7 gives 7, which is 4% off the standard value. The in vivo value ~12 matches sigma(6) better. But: (a) the standard free energy depends on arbitrarily chosen conditions; (b) the formula sigma - sopfr is ad hoc; (c) the in vivo value varies by cell type and metabolic state. Neither 7 nor 12 is a fundamental physical constant. WEAK confirmed.

---

## H-BIO-13: Histone Octamer = tau(6) * phi(6) = 8

**Grade: CLOSE** (confirmed)

The nucleosome core particle contains exactly 8 histone proteins: 2 copies each of H2A, H2B, H3, H4 (Luger et al., Nature 1997, crystal structure at 2.8 A). This is one of the most precisely determined structures in biology (PDB: 1AOI).

The decomposition 4 types x 2 copies = tau(6) x phi(6) = 8 is clean. The 4 histone types and 2-fold symmetry are structurally real (the octamer has a (H3-H4)_2 tetramer + 2x(H2A-H2B) dimers).

However: 8 = 2^3 is common; the linker histone H1 makes 5 types total; and tau*phi = 8 for several n values. CLOSE is fair -- the structural decomposition is real and the match is clean, even if not n=6-specific.

---

## H-BIO-14: Reading Frame = 3 = n/phi

**Grade: CLOSE** (confirmed)

The ribosome reads mRNA in non-overlapping triplets of 3 nucleotides. This is universal and firmly established since the 1960s (Crick, Barnett, Brenner, Watts-Tobin, 1961 -- frameshift experiment).

n/phi = 6/2 = 3 matches. The reading frame length 3 is the minimum k such that 4^k >= 20 (the number of amino acids to encode). This information-theoretic argument fully explains 3 without reference to n=6.

CLOSE rather than WEAK because: the number 3 is genuinely fixed by the 4-base/20-amino-acid constraints, and the formula n/phi = 3 is simple. But the explanatory power of the n=6 connection is nil.

---

## H-BIO-15: Membrane Bilayer = phi(6) = 2

**Grade: FAIL** (confirmed)

Phospholipid bilayers have 2 leaflets. This is a thermodynamic consequence of amphipathic molecules in an aqueous environment -- hydrophobic tails aggregate to exclude water, forming 2 leaflet surfaces. Micelles (1 layer) and multilayer structures also exist depending on lipid geometry.

The number 2 here is explained by geometry and thermodynamics. It has nothing to do with number theory. Every bilateral structure "matches" phi(6) = 2. FAIL confirmed.

---

## H-BIO-16: Glucose C6H12O6 = (n, sigma, n)

**Grade: EXACT** (confirmed)

Glucose molecular formula: C6H12O6. This is a hard chemical fact (molecular weight 180.156 g/mol, CAS 50-99-7). The subscripts (6, 12, 6) map to (n, sigma(6), n).

The formula follows from glucose being an aldohexose: general formula C_m H_{2m} O_m with m=6. So H = 2*6 = 12 = sigma(6) is automatic for any hexose. The connection sigma(6) = 2n = 12 is intrinsic to the perfect number identity.

Why hexoses? 6-carbon sugars are the primary energy currency across all life (glucose, fructose, galactose). Pentoses (C5) serve structural roles (ribose, deoxyribose). The metabolic dominance of hexoses is well-established but not a strict physical necessity.

EXACT confirmed: the formula C6H12O6 is a hard chemical fact, and the (n, sigma, n) mapping is exact.

---

## H-BIO-17: 6 Kingdoms of Life

**Grade: FAIL** (confirmed)

The "6 kingdoms" classification (Bacteria, Archaea, Protista, Fungi, Plantae, Animalia) was one proposal among many. Historical progression: 2 kingdoms (Linnaeus, 1735), 3 (Haeckel, 1866), 4 (Copeland, 1938), 5 (Whittaker, 1969), 6 (Woese, 1977), 3 domains (Woese, 1990), and various 7-8 kingdom systems. Modern systematics increasingly uses cladistics rather than ranked kingdoms.

The count is a human convention that has changed repeatedly. No physical law constrains the number of kingdoms. FAIL confirmed.

---

## H-BIO-18: 6 Cortical Layers = n

**Grade: CLOSE** (confirmed)

The mammalian neocortex has 6 histologically defined layers (Brodmann, 1909). This 6-layer organization is conserved across all mammals examined, from mice to humans, and arises from a stereotyped developmental process (inside-out migration of neurons from the ventricular zone).

The 6-layer pattern is more robust than most biological counts:
- Layers are defined by cell type, density, and connectivity
- Each layer has distinct input/output patterns
- The layering is consistent across species

Caveats: (a) Some cortical regions have fewer distinct layers (agranular cortex effectively lacks layer IV); (b) reptilian cortex has 3 layers; (c) within each layer, sublayers are sometimes identified (e.g., IVa, IVb, IVc in V1).

CLOSE is fair. The 6 layers are a genuine, conserved biological structure -- more robust than a classification convention. But variation exists and the count partly reflects histological resolution.

---

## H-BIO-19: Carbon Z = 6 = n

**Grade: EXACT** (confirmed)

Carbon has atomic number 6 (6 protons, 6 electrons). This is a physical fact, not a convention. Carbon's electronic structure (1s^2 2s^2 2p^2, 4 valence electrons = tau(6)) enables tetravalent bonding, the basis of organic chemistry and life.

The double match -- Z = n = 6 and valence = tau(6) = 4 -- is the strongest single connection in this domain. Carbon is the only element with these properties that can support the combinatorial complexity of life:
- Silicon (Z=14) forms weaker C-equivalent bonds
- No other element near Z=6 has 4 valence electrons with comparable bond diversity

EXACT confirmed. Carbon's atomic number is 6, period.

---

## H-BIO-20: 4 Amino Acid Property Groups = tau(6)

**Grade: WEAK** (confirmed)

The 4-group classification (nonpolar, polar uncharged, positive, negative) is the most common textbook scheme (Lehninger, Stryer). But alternatives abound:
- 2 groups: hydrophobic / hydrophilic (Kyte-Doolittle)
- 3 groups: nonpolar / polar / charged (many introductory texts)
- 5 groups: adding aromatic as distinct (Petsko & Ringe)
- Continuous: hydrophobicity scales have no natural groupings

The "4" is a convention, not a physical constraint. WEAK confirmed.

---

## H-BIO-21: 3 RNA Types = n/phi = 3

**Grade: WEAK** (confirmed)

The "3 types of RNA" (mRNA, tRNA, rRNA) is a 1960s-era classification that predates the RNA revolution. Modern molecular biology recognizes: snRNA, snoRNA, miRNA, siRNA, piRNA, lncRNA, circRNA, ribozymes, riboswitches, CRISPR RNA, tmRNA, and many more.

The "3 major types" persists in introductory textbooks but is a historical simplification, not a physical fact. WEAK confirmed.

---

## H-BIO-22: Anticodon = 3 Bases

**Grade: WEAK** (confirmed)

The tRNA anticodon is 3 nucleotides long, complementary to the 3-nucleotide codon. This is structurally verified by tRNA crystal structures (Kim et al., 1974; Robertus et al., 1974).

However, this is not an independent match -- the anticodon length is forced to equal the codon length (3), which is already counted in H-BIO-14. Double-counting the same biological fact inflates the apparent pattern density. WEAK for lack of independence.

---

## H-BIO-23: 4 Protein Structure Levels = tau(6)

**Grade: CLOSE** (confirmed)

The 4-level hierarchy (primary, secondary, tertiary, quaternary) is universally taught in biochemistry and reflects genuine physical distinctions:
1. Primary: covalent peptide bonds (~350 kJ/mol)
2. Secondary: backbone hydrogen bonds (~8-30 kJ/mol)
3. Tertiary: hydrophobic effect, disulfides, salt bridges (~4-30 kJ/mol)
4. Quaternary: subunit interfaces (same forces as tertiary)

The distinction between levels reflects different energy scales and structural organization. However, not all proteins have quaternary structure, and some texts add "supersecondary" (motifs) or "domain" levels.

CLOSE is fair: the 4-level system is well-established and physically grounded, even if not perfectly universal.

---

## H-BIO-24: Nucleotide = 3 Components

**Grade: WEAK** (confirmed)

Nucleotides comprise base + sugar + phosphate. This is a hard chemical fact. But the number 3 is trivially small, and "3 components" describes any molecule one chooses to decompose into 3 parts. An amino acid also has 3 main groups. A phospholipid has 3 parts (head, glycerol, tails). WEAK confirmed.

---

## H-BIO-25: Amino Acid = 3 Backbone Groups

**Grade: FAIL** (confirmed, downgraded from original)

The alpha-carbon in an amino acid is bonded to 4 groups: amino (-NH2), carboxyl (-COOH), R-group (side chain), and hydrogen (-H). The hypothesis claims 3 by ignoring the hydrogen. Tetrahedral carbon has 4 substituents, not 3. This is incorrect chemistry. FAIL confirmed.

---

## H-BIO-26: Krebs Cycle = 8 Steps

**Grade: FAIL** (confirmed)

The citric acid cycle has 8 enzymatic steps. This is well-established (Lehninger ch. 16). The number 8 does not directly correspond to any single n=6 arithmetic function. The formula tau * phi = 4 * 2 = 8 works numerically but is an arbitrary product of two functions chosen post-hoc.

Moreover, if you count differently: some sources list the pyruvate dehydrogenase complex as a preliminary step (making 9), or count the TCA cycle as producing 3 NADH + 1 FADH2 + 1 GTP per turn. The "8 steps" is standard but the connection to n=6 is forced. FAIL confirmed.

---

## H-BIO-27: 6-Membered Rings in Biology

**Grade: CLOSE** (confirmed)

Six-membered rings genuinely dominate biological chemistry:
- Benzene ring: Phe, Tyr, Trp (aromatic amino acids)
- Pyrimidine: C, T, U bases
- Glucose (pyranose): 6-membered ring
- Purine: contains a 6-membered ring fused with 5-membered
- NAD+, FAD, ATP all contain 6-membered ring components

The preference for 6-membered rings has a physical basis: they have the lowest ring strain among common ring sizes (for sp3 carbon: ideal bond angle 109.5 degrees, cyclohexane angle ~111 degrees; for sp2: ideal 120 degrees, benzene achieves exactly 120 degrees).

However, 5-membered rings are also ubiquitous (ribose, deoxyribose, imidazole, proline, furanose). It is an overstatement to say 6-membered rings exclusively dominate. CLOSE confirmed: 6-membered rings have a genuine thermodynamic advantage, but 5-membered rings are nearly as common.

---

## H-BIO-28: Glycolysis 2 Phases = phi(6)

**Grade: FAIL** (confirmed)

Virtually any process can be divided into an "investment" and "payoff" phase. This is not specific to glycolysis, n=6, or biology. The same logic would match phi(6) = 2 to: breathing (inhale/exhale), heartbeat (systole/diastole), the tides (high/low), or any oscillatory process. Zero specificity. FAIL confirmed.

---

## H-BIO-29: Water = 3 Atoms

**Grade: FAIL** (confirmed)

H2O has 3 atoms. So does CO2, SO2, NO2, H2S, and hundreds of other molecules. The number 3 in a triatomic molecule is explained by chemical valence (oxygen forms 2 bonds), not by n/phi = 3. The match is trivially true and entirely explained by atomic bonding rules. FAIL confirmed.

---

## H-BIO-30: Semiconservative Replication = 2

**Grade: FAIL** (confirmed)

Semiconservative replication produces 2-strand daughter DNA molecules because the parent has 2 strands. This is a logical consequence of H-BIO-2 (double helix), not an independent observation. Counting it separately inflates the match count. Furthermore, conservative and dispersive replication were the alternatives considered by Meselson and Stahl -- all three models produce 2-strand daughters. The "2" is a property of DNA structure, not of the replication mechanism. FAIL confirmed.

---

## Cross-Domain Comparison

| Domain | EXACT | CLOSE | WEAK | FAIL | Non-fail % |
|--------|-------|-------|------|------|------------|
| Superconductor (60) | 2 | 10 | 19 | 29 | 51.7% |
| Biology (30) | 3 | 9 | 9 | 9 | 70.0% |

The higher non-fail rate in biology reflects that biological systems have many small discrete counts (2, 3, 4, 5, 6) that match n=6 arithmetic functions by combinatorial probability. With 8+ arithmetic functions of n=6 producing values in the range 1-24, any biological system with counts in that range has a high probability of coincidental matching.

The 3 EXACT matches are genuinely notable:
1. **H-BIO-3 (64 codons = tau^3)**: Strongest. Hard chemistry, clean formula, multiple n=6 decompositions.
2. **H-BIO-19 (carbon Z=6)**: Fundamental. Carbon's atomic number is a physical constant.
3. **H-BIO-16 (glucose C6H12O6)**: Strong. Hard formula, (n, sigma, n) mapping is exact for hexoses.
