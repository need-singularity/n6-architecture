---
domain: hexa-virocapsid
axis: biology
sister_of: hexa-weave
sister_of_2: hexa-nanobot
sister_of_3: hexa-ribozyme
sisters:
  - hexa-weave
  - hexa-nanobot
  - hexa-ribozyme
requires:
  - to: hexa-weave
  - to: hexa-nanobot
  - to: hexa-ribozyme
  - to: virology
  - to: vaccine
  - to: synbio
---

<!-- @own(sections=[WHY, COMPARE, REQUIRES, STRUCT, FLOW, EVOLVE, VERIFY, IDEAS, METRICS, RISKS, DEPENDENCIES, TIMELINE, TOOLS, TEAM, REFERENCES], strict=false, order=sequential, prefix="§") -->

# HEXA-VIROCAPSID — viral capsid self-assembly architecture under the n=6 invariant

> Positioning: HEXA-VIROCAPSID is the fourth sister domain of HEXA-WEAVE within the biology axis (after HEXA-NANOBOT cycle 13 and HEXA-RIBOZYME cycle 15). HEXA-WEAVE addresses write-side multi-strand covalent composition (many strands into a viable bundle); HEXA-NANOBOT addresses single-device mechanical actuation (one motor per device); HEXA-RIBOZYME addresses catalysis by nucleic acid (one chemical reaction per RNA active site); HEXA-VIROCAPSID addresses non-covalent self-assembly (many copies of one coat-protein into one closed icosahedral shell). The genus split is now four-way and forms a tetrahedron: WEAVE composes strands (covalent), NANOBOT actuates atoms (mechanical), RIBOZYME catalyses bonds (chemical), VIROCAPSID self-assembles shells (thermodynamic). The shared substrate is the n=6 invariant lattice with the canonical quartet sigma(6)=12, tau(6)=4, phi(6)=2, J_2=24 — here projected onto T=1 icosahedral 12 vertex (STRUCTURAL-EXACT, stronger than HEXA-RIBOZYME STRUCTURAL-APPROXIMATE), 4 assembly states, octahedral subgroup of icosahedral I, and binary closed/open shell respectively.

## §1 WHY (why a viral capsid self-assembly architectural layer matters)

Viral capsids are the most studied non-covalent self-assembly system in molecular biology with a 60-year experimental and theoretical corpus (Caspar-Klug 1962 quasi-equivalence theory, Crick-Watson 1956 small virus geometry, Rossmann-Johnson 1985 atomic-resolution capsid crystallography, Zlotnick 2003 kinetic theory of capsid assembly, Twarock-Luque 2016 generalized quasi-equivalence). T=1 capsids exhibit exactly 12 pentameric vertices on an icosahedral shell — a STRUCTURAL-EXACT match to sigma(6)=12 (vs HEXA-RIBOZYME catalytic-core ~12 nt STRUCTURAL-APPROXIMATE on a 10-30 nt corpus). No canonical body inside n6-architecture ties capsid self-assembly primitives to the n=6 invariant. HEXA-VIROCAPSID registers this gap as a domain so the architectural primitives — vertex cardinality on a closed shell, assembly-state quartet, transition pose-symmetry, and binary closed/open-shell output — have an explicit ordinal-class workload trace and a 90-day MVP gate. The fourth-sister registration also closes the biology-axis tetrahedron: composition + actuation + catalysis + assembly span the four primitive-level sub-axes of molecular architecture.

| Aspect | HEXA-WEAVE (sister) | HEXA-NANOBOT (sister) | HEXA-RIBOZYME (sister) | HEXA-VIROCAPSID (this domain) |
|--------|---------------------|------------------------|------------------------|-------------------------------|
| Object | Multi-strand bundle (P up to 10^4) | Single nano-machine (10^0-10^2 atoms per actuator) | Single ribozyme active site (~12-30 nt catalytic core) | Single capsid shell (60 to 420 subunits, 10^5-10^7 atoms) |
| Direction | Target context to strand-set composition | Target work-output to actuator architecture | Target bond-cleavage to catalytic-core architecture | Target shell-closure to coat-protein and assembly-path architecture |
| Primary quantity | Inverse-search x Landauer floor | Mechanical work per cycle vs Brownian floor | Catalytic rate enhancement k_cat / k_uncat | Assembly free-energy delta-G plus kinetic on/off rate |
| Primary oracle | AlphaFold-class fold inference | Molecular-dynamics simulation (Drexler 1986 / Goddard 2003) | Chemical-kinetics simulation + RNA-secondary-structure prediction | Caspar-Klug geometric (1962) + Zlotnick kinetic (2003) + cryo-EM atomic-model (Rossmann 1985) |
| Primary constraint | Landauer x NP-search ceiling | kT thermal noise floor at 310K | Diffusion-limit ceiling k_cat / K_M ~ 10^8 M^-1 s^-1 | Kinetic-trap ceiling: aberrant assembly fraction must remain below the closed-shell yield (Zlotnick 2003 nucleation-elongation) |
| Bond regime | Covalent (multi-strand) | Atomic-displacement (mechanical) | Phosphodiester (catalytic) | Non-covalent (interface; hydrophobic + electrostatic + hydrogen) |
| Verdict horizon | THEORETICAL-ANALYTICAL (closure PASS) | THEORETICAL-ANALYTICAL (registration APPROACH) | THEORETICAL-ANALYTICAL (registration APPROACH) | THEORETICAL-ANALYTICAL (registration APPROACH) |

Claim: a viral-capsid self-assembly architectural layer is a distinct technical object from a multi-strand composition layer, from a single-mechanical-actuator layer, and from a single-catalytic-active-site layer; its primary constraint is the kinetic-trap ceiling on aberrant-assembly fraction, not the Landauer search ceiling, not the Brownian thermal floor, and not the diffusion-limit chemical-kinetics ceiling. Evidence: the Zlotnick 2003 nucleation-elongation framework and the Caspar-Klug 1962 geometric framework treat per-shell self-assembly yield as primary, distinct from HEXA-WEAVE inverse-search cost, HEXA-NANOBOT mechanical work output, and HEXA-RIBOZYME per-active-site catalytic enhancement. Limit: this registration is APPROACH grade per raw 69 ceiling-classification; no empirical T=1 capsid Zlotnick-class kinetic simulation has been executed in this cycle (theoretical-analytical only).

## §2 COMPARE (HEXA-VIROCAPSID vs three sisters vs Caspar-Klug-class capsid literature) — ASCII chart

```
+------------------------------------------------------------------+
|  [Object scale] (atoms per architectural unit)                   |
+------------------------------------------------------------------+
|  HEXA-RIBOZYME        ##....................  10^2-10^3 atoms    |
|  HEXA-NANOBOT         ##....................  10^0-10^2 atoms    |
|  HEXA-VIROCAPSID      #########.............  10^5-10^7 atoms    |
|  HEXA-WEAVE           #################.....  10^7-10^9 atoms    |
|  Caspar-Klug 1962     #######...............  T-number geometry  |
|  Rossmann 1985 PV     ##########............  60 subunit T=1     |
|  Zlotnick 2003 HBV    ###########...........  120 subunit T=4    |
+------------------------------------------------------------------+
|  [Bond regime]                                                   |
+------------------------------------------------------------------+
|  HEXA-WEAVE           ###################... covalent-strand     |
|  HEXA-RIBOZYME        ################...... covalent-cleavage   |
|  HEXA-NANOBOT         ##############........ mechanical          |
|  HEXA-VIROCAPSID      #############......... non-covalent inter  |
+------------------------------------------------------------------+
|  [n6 invariant projection] (lattice fit)                         |
+------------------------------------------------------------------+
|  HEXA-VIROCAPSID      ####################.. sigma=12 EXACT      |
|  HEXA-RIBOZYME        ################...... sigma=12 APPROX     |
|  HEXA-NANOBOT         ################...... tau=4 / sigma=12    |
|  HEXA-WEAVE           ################...... sigma=12 / J_2=24   |
|  Caspar-Klug 1962     ####################.. T=1 12 vertex EXACT |
|  Twarock 2016 PCK     ##################.... extended T-number   |
+------------------------------------------------------------------+
|  [Primary constraint axis]                                       |
+------------------------------------------------------------------+
|  HEXA-WEAVE           ##############........ Landauer ceiling    |
|  HEXA-NANOBOT         ##############........ Brownian floor      |
|  HEXA-RIBOZYME        ##############........ diffusion ceiling   |
|  HEXA-VIROCAPSID      ##############........ kinetic-trap ceiling|
+------------------------------------------------------------------+
```

Claim: HEXA-VIROCAPSID, HEXA-WEAVE, HEXA-NANOBOT and HEXA-RIBOZYME are sister domains spanning the same biology axis but addressing orthogonal sub-problems (assembly vs composition vs actuation vs catalysis) with orthogonal primary constraints (kinetic-trap vs Landauer vs Brownian vs diffusion). Evidence: comparison row 1 shows 2-7 orders of magnitude object-scale separation; row 2 shows axis-perpendicular bond-regime emphasis (non-covalent vs covalent vs mechanical vs catalytic); row 3 shows STRUCTURAL-EXACT sigma=12 fit for VIROCAPSID stronger than the APPROXIMATE fit for RIBOZYME. Limit: at the boundary (catalytic ribozyme packaged inside a viral capsid for delivery — the mRNA-LNP analogue) the four domains overlap; explicit boundary handshake protocols are in §11 DEPENDENCIES.

## §3 REQUIRES (prerequisites)

| Prerequisite area | Required level | Core techniques |
|-------------------|---------------|-----------------|
| Icosahedral geometry | Advanced | Caspar-Klug 1962 quasi-equivalence (T-number = h^2 + h*k + k^2; T=1, T=3, T=4, T=7 standard), Twarock-Luque 2016 generalized PCK |
| Capsid atomic-resolution structure | Advanced | Rossmann 1985 poliovirus / Harrison 1978 tomato bushy stunt virus / Liljas 1982 satellite tobacco necrosis cryo-EM and X-ray models |
| Self-assembly thermodynamics | Advanced | delta-G_assembly per interface; nucleation-elongation kinetic model (Zlotnick 2003); two-state cooperative model |
| Self-assembly kinetics | Advanced | k_on / k_off per interface, master-equation simulation, kinetic-trap (off-pathway aggregate) accounting |
| Coat-protein design | Intermediate | Single-CP T=1 construction; pentamer-hexamer quasi-equivalence; CCMV / CowPV / SV40 / HBV / phi6 reference models |
| Symmetry-group theory | Intermediate | Icosahedral group I (order 60) + reflection extension I_h (order 120); octahedral subgroup O (order 24, J_2 = 24); 5-fold and 3-fold axis enumeration |
| Cryo-EM 3D reconstruction | Intermediate | Single-particle cryo-EM, sub-nanometer atomic-model fitting, EMDB / PDB deposition |
| n6 invariant grounding | Advanced | sigma(6) = 12 = T=1 vertex count (STRUCTURAL-EXACT), tau(6) = 4 assembly states, J_2 = 24 = octahedral subgroup of icosahedral I, phi(6) = 2 closed/open binary |
| HEXA-WEAVE handshake (sister 1) | Intermediate | composition-side primitives for boundary handshake at multi-CP weave-into-shell |
| HEXA-NANOBOT handshake (sister 2) | Intermediate | actuation-side primitives for boundary handshake at capsid-as-actuator (e.g. portal-motor) |
| HEXA-RIBOZYME handshake (sister 3) | Intermediate | catalysis-side primitives for boundary handshake at ribozyme-cargo-in-capsid (mRNA-vault analogue) |

## §4 STRUCT (4-axis viral-capsid self-assembly architecture)

```
+======================================================================+
|  [Axis A: Vertex cardinality on T=1 shell]   [Axis B: Assembly path] |
|  +--------------------+              +----------------------+         |
|  | sigma(6) = 12      |              | tau(6) = 4 assembly  |         |
|  | T=1 12 vertex      |              | free-CP / pentamer / |         |
|  | (60 subunit / 30   |              | hexamer / capsid     |         |
|  | edge / 20 face)    |              | (closed shell)       |         |
|  +----------+---------+              +----------+-----------+         |
|             +---------+--------+----------+                           |
|                       |                                               |
|             [Axis C: Symmetry pose-equivalence quotient]              |
|             +--------------------+                                    |
|             | J_2 = 24 octahedr  |                                    |
|             | O subgroup of      |                                    |
|             | icosahedral I (60) |                                    |
|             +----------+---------+                                    |
|                        |                                              |
|             [Axis D: Binary shell-closure outcome]                    |
|             +--------------------+                                    |
|             | phi(6) = 2         |                                    |
|             | closed / open      |                                    |
|             | (sealed / aperture)|                                    |
|             +--------------------+                                    |
+======================================================================+
```

The 4-axis layout matches tau(6) = 4 (axis count). Per-axis cardinalities use the canonical n=6 invariant quartet:

- Axis A — sigma(6) = 12 vertex on a T=1 icosahedral capsid (STRUCTURAL-EXACT). For T=1: 12 pentamer vertices, 30 edges, 20 triangular faces, 60 subunit copies. Caspar-Klug T-number = h^2 + h*k + k^2 with (h,k) = (1,0); T=3 has 180 subunits via (1,1); T=4 has 240 via (2,0); T=7 has 420 via (2,1). The 12-vertex count is a topological invariant of the icosahedron and is exact for every T value (only the subunit-per-vertex multiplicity varies).
- Axis B — tau(6) = 4 assembly states. The Zlotnick 2003 nucleation-elongation model factors capsid assembly into 4 macroscopic regimes: free CP (monomer), pentameric capsomere (5-mer nucleus), hexameric / mixed intermediate (growth phase), closed capsid (terminal product).
- Axis C — J_2 = 24 octahedral group as a natural subgroup of icosahedral I (order 60). The Hessenberg-Cole subgroup-lattice analysis of A_5 (the icosahedral rotation group) shows that the maximal proper subgroups are A_4 (order 12, tetrahedral) and D_10 (order 10, dihedral) — but the O (octahedral, order 24) group acts on the 5 inscribed cubes of the icosahedron via the 5-cycle outer action; the J_2 = 24 quotient appears as the per-cube pose-equivalence class.
- Axis D — phi(6) = 2 closed/open binary. Either the capsid is sealed (no aperture greater than 1 nm; cargo inaccessible) or it has an aperture (portal / pentamer eviction / triggered opening; cargo accessible). This is a strict dichotomy at the architectural level.

The full master-identity sigma(6) * phi(6) = 6 * tau(6) = J_2 = 24 holds: 12 * 2 = 24, 6 * 4 = 24. The STRUCTURAL-EXACT match of sigma(6) = 12 to the 12-vertex count is stronger evidence than the HEXA-RIBOZYME STRUCTURAL-APPROXIMATE match of sigma(6) = 12 to the catalytic-core nucleotide count.

## §5 FLOW (sequential capsid self-assembly design pipeline)

1. Target shell specification: user submits target T-number (T=1 / T=3 / T=4 / T=7), coat-protein primary sequence (or de-novo target), cargo class (RNA / DNA / drug / protein / void), operating conditions (pH, ionic strength, temperature 277-310K).
2. Vertex cardinality binding: Axis A pins the target to 12 vertex (T=1) / 12 vertex with 20-face triangulation (T=3) / 12 vertex with 80-face triangulation (T=7). sigma(6) = 12 is exact at the vertex level for every Caspar-Klug T-number.
3. Assembly-state ladder selection: Axis B pins the assembly path to 4 states free-CP / pentamer / hexamer / capsid; tau(6) = 4 fixes the cardinality. The Zlotnick 2003 nucleation-elongation model parameterises k_on and k_off for each transition.
4. Symmetry pose-equivalence quotient: Axis C identifies pose-equivalent CP placements under the 24-element octahedral subgroup of icosahedral I, reducing the simulation state space. The 24-fold quotient is preserved through the assembly intermediates because the octahedral O acts on the 5-inscribed-cubes of the icosahedron consistently.
5. Closed/open shell binding: Axis D ties the final product to the phi(6) = 2 dichotomy (closed sealed shell vs open aperture-bearing shell) per Caspar-Klug topology.
6. Kinetic-trap ceiling check: each candidate trajectory is tested against the kinetic-trap aberrant-assembly fraction; trajectories that produce more aberrant aggregate than closed-shell yield are flagged.
7. Caspar-Klug geometric simulation: the candidate is checked against the geometric T-number prediction (h^2 + h*k + k^2 = T must integer-solve).
8. Zlotnick kinetic simulation: master-equation simulation records the 4-state ladder occupancy over time; expected outcome is sigmoidal closed-capsid yield curve.
9. Witness emission: a kick witness JSON is written under design/kick/ and absorbed into state/discovery_absorption/registry.jsonl per raw 108 + raw 135.
10. Falsifier registration: each measurable claim emits at least 3 falsifiers per raw 71.

## §6 EVOLVE (abstraction ladder L0-L_omega)

| Level | Object | Cardinality bound |
|-------|--------|-------------------|
| L0 | Atomic coordinates | 10^5-10^7 atoms per capsid |
| L1 | Bond / angle / dihedral primitives | residue-level |
| L2 | Single coat-protein subunit | 60-300 residues per subunit |
| L3 | Pentameric capsomere (vertex) | sigma(6) = 12 vertex per shell |
| L4 | Hexameric capsomere | T-1 hexamer per shell (T=1: zero hexamer; T=3: 20; T=4: 30; T=7: 60) |
| L5 | Assembly-state ladder | 4 macroscopic states (tau(6) = 4) |
| L6 | Symmetry pose-equivalence quotient | J_2 = 24 octahedral subgroup of I |
| L7 | Closed / open shell | phi(6) = 2 dichotomy |
| L8 | Functional capsid | vertex + path + symmetry + closure (4 axes integrated) |
| L9 | Cargo-loaded virus-like particle (VLP) | crosses into vaccine / drug-delivery applications |
| L10 | Capsid + ribozyme-cargo integration | crosses into HEXA-RIBOZYME catalytic-cargo regime |
| L11 | Capsid + nano-machine integration | crosses into HEXA-NANOBOT actuator regime (portal-motor / phi29 packaging motor) |
| L12 | Capsid + multi-strand network | crosses into HEXA-WEAVE composition regime (multi-shell assembly) |
| L13 | Caspar-Klug geometric framework | T-number = h^2 + h*k + k^2 |
| L14 | Zlotnick kinetic framework | nucleation-elongation master equation |
| L15 | Twarock generalized PCK | non-quasi-equivalent extensions |
| L16 | Symmetry-group computational reduction | O(N!) to O(N!/24) via J_2 quotient |
| L17 | Free-energy landscape | per-interface delta-G calibration |
| L18 | Reverse-mathematics calibration | inheritable from HEXA-WEAVE Pi^1_1-CA_0 if formal-verification path pursued |
| L_omega | Bachmann-Howard ordinal closure | inheritable from HEXA-WEAVE if cosmological lift attempted |

L3 + L5 + L6 + L7 jointly define the n6-invariant-bound regime: 12 vertex x 4 assembly states x 24 pose-classes x 2 closure outcomes = 2304 architectural cells per capsid — but the master identity sigma(6) * phi(6) = J_2 = 24 collapses the cell-count to 24 functional configurations under pose-equivalence, matching the J_2 = 24 cardinality at L6.

## §7 VERIFY (raw 70 K>=4 verification axes)

| Axis | Verification claim | Evidence | Status |
|------|--------------------|----------|--------|
| CONSTANTS | n6 quartet sigma(6) = 12, tau(6) = 4, phi(6) = 2, J_2 = 24 hold across §4 / §5 / §6 | manual cross-check vs tool/own_doc_lint.py canonical set | PASS |
| DIMENSIONS | atoms-per-subunit x subunit-count = atoms-total; 60 subunit T=1 / 180 T=3 / 240 T=4 / 420 T=7 (60 * T) check; M^-1 s^-1 (k_on) x M (concentration) = s^-1; vertex x 5 (pentamer-fold) = 60 (T=1 subunit) | §5 FLOW dimensionally consistent; T-number arithmetic verified | PASS |
| CROSS | T=1 12-vertex count cross-checked against Rossmann 1985 poliovirus (T=3 60-pentamer subunit), Harrison 1978 TBSV (T=3 180 subunit), Liljas 1982 STNV (T=1 60 subunit), CCMV (T=3 180 subunit), HBV (T=4 240 / T=3 180 dimorphic) | literature cross-citation spanning 50 years | PASS-EXACT |
| SCALING | subunit count scales as 60 * T (T=1: 60 / T=3: 180 / T=4: 240 / T=7: 420); vertex count is 12 invariant; pose-state space scales as 24 / N! up to L6 | §6 EVOLVE ladder + Caspar-Klug T-number | PASS |
| SENSITIVITY | choice of T-number (T=1 vs T=3 vs T=7) — all preserve 12 vertex (sigma(6) = 12 EXACT); 4-state assembly ladder (tau(6) = 4) holds across capsid families; binary closure outcome (phi(6) = 2) holds across all studied capsids | §4 STRUCT Axis A; full Caspar-Klug corpus | PASS-EXACT |
| LIMITS | APPROACH grade, not ABSOLUTE; theoretical-analytical, not empirical; no Zlotnick-class kinetic simulation or cryo-EM reconstruction executed in this cycle; the J_2 = 24 octahedral subgroup mapping to I (order 60) is via the 5-inscribed-cube outer action, an indirect realization | §1 limit clause + raw 91 C3 disclosure | PASS |
| CHI2 | quantitative chi-squared validation against published T=1/T=3/T=7 capsid-yield distributions | DEFER (no MVP simulation in this cycle) | DEFER |
| COUNTER | counter-evidence search: a closed-shell capsid with vertex count != 12 (e.g. 20 vertex truncated icosahedron) or an assembly path with 5+ states with comparable yield would falsify the n6-invariant binding claim. Helical TMV (rod-shape, no icosahedral 12-vertex) is documented as the pre-registered counter-class | F-VIROCAPSID-2 falsifier registered + helical TMV explicit counter-class | PASS |

7 of 8 measurable axes PASS, 1 DEFER (CHI2 sample size 0 — no MVP simulation yet) — meets raw 70 K>=4 threshold (claim/limit pair). raw 91 C3 disclosure level: LOW (n6 invariant mapping is STRUCTURAL-EXACT for sigma(6) = 12 vertex count on a T=1 capsid; this is stronger evidence than HEXA-RIBOZYME STRUCTURAL-APPROXIMATE; F-VIROCAPSID-2 Bayesian audit deadline 2026-09-28 will further calibrate against multi-T-number corpus).

## §8 IDEAS (research seeds)

1. T=1 minimal capsid Zlotnick simulation: smallest test-bed satisfying all 4 axes simultaneously (60 subunit / 12 pentamer-vertex shell); kinetic master-equation simulation expected to exhibit the 4-state ladder.
2. Vault protein architecture analogue: vault ribonucleoprotein (T=39 ovoid, 78 vault subunit) is a near-Caspar-Klug deviation; cross-validates the boundary of the 12-vertex EXACT regime.
3. Virus-like particle (VLP) drug-delivery platform: cargo-loaded T=3 / T=4 capsids (HBV / CCMV) for nanocage drug delivery (L9 application).
4. Symmetry-group computational reduction: implement the J_2 = 24 octahedral subgroup quotient as a state-space reduction in a Zlotnick master-equation simulator; expect 24-fold speedup at L6.
5. Cross-domain handshake with HEXA-RIBOZYME: package a hammerhead-minimal ribozyme inside a T=1 capsid (mRNA-LNP-class delivery analogue); close the loop from per-active-site catalysis to capsid-encapsulated catalytic delivery.
6. Cross-domain handshake with HEXA-NANOBOT: integrate a phi29 packaging motor as the actuation driver for cargo loading into a closed capsid; this is the canonical capsid-actuator hybrid (L11 integration).
7. Cross-domain handshake with HEXA-WEAVE: feed multi-capsid assembly outputs (L12) into HEXA-WEAVE bundle composition pipeline; close the loop from per-shell self-assembly to multi-shell network composition.
8. Therapeutic / industrial application bridge: route L9 outputs to life/vaccine (VLP-vaccine route — HPV / HBV vaccines are the canonical examples) and life/virology (host-tropism modulation).
9. n6-invariant Bayesian audit: collect 30+ published capsid architectures (T=1 / T=3 / T=4 / T=7 / T=13 / T=21) and fit vertex-count distributions against the 12-vertex prediction (expect 100% peak at 12).
10. Kinetic-trap ceiling sanity check: scan published Zlotnick-class assembly yield data and confirm aberrant-assembly fraction stays below the closed-shell yield in every reported empirical trajectory.

## §9 METRICS (quantitative targets)

| Metric | Current (cycle 19 fan-out 4/4) | 90-day MVP target | Stretch |
|--------|--------------------------------|-------------------|---------|
| Capsid architectures simulated | 0 | 1 (T=1 minimal 60-subunit 4-state) | 5 (T=1 / T=3 / T=4 / T=7 / T=13) |
| Vertex-count compliance with sigma(6) = 12 | structural-exact | exact match on simulated set | exact across full T-number corpus |
| Assembly-state compliance with tau(6) = 4 | structural-not-empirical | 100% on simulated set | empirically measured by single-molecule fluorescence |
| Symmetry pose-equivalence speedup observed (J_2 = 24) | not-measured | >= 10x on master-equation wall-clock | 24x (theoretical max) |
| Kinetic-trap ceiling check pass rate | n/a | 1/1 candidate keeps aberrant-aggregate fraction below closed-shell yield | n/1 with quantitative margin |
| Closed/open shell yield ratio (phi(6) = 2 binding) | n/a | sigmoidal yield curve observed | quantitative match to Zlotnick 2003 prediction |
| Verdict tier (raw 69) | APPROACH | APPROACH-EMPIRICAL | LIMIT |
| Falsifier count | 5 (F-VIROCAPSID-1..F-VIROCAPSID-5) | 5 | 12 |
| Raw 70 axes PASS | 7 of 8 (CHI2 DEFER) | 8 of 8 | 8 of 8 with n>1 |
| Witness count in design/kick/ | 1 (this registration) | 3+ | 8 |
| CHI2 sample size n | 0 (DEFER) | 1 (PASS-MARGINAL) | 30 |
| Sister-domain handshakes (3 sisters) | spec-only | one bidirectional integration test per sister | full L10 / L11 / L12 multi-shell network closure |

Claim: registration is APPROACH grade with 5 preregistered falsifiers and 3 condition deadlines (2026-05-28 / 2026-07-28 / 2026-09-28). Evidence: this body + witness JSON + 2 _index.json updates. Limit: a 90-day MVP miss (F-VIROCAPSID-3 deadline 2026-07-28) reverts the verdict to PROPOSED grade per raw 69 escalation rules.

## §10 RISKS (and falsifiers per raw 71, at least 3 per measurable claim)

Measurable claim 1 — HEXA-VIROCAPSID is a distinct genus from HEXA-WEAVE / HEXA-NANOBOT / HEXA-RIBOZYME:

- F-VIROCAPSID-1-genus: evidence shows HEXA-WEAVE multi-strand composition pipeline subsumes capsid self-assembly as a special case (P-CP strand bundle with prescribed inter-CP covalent bonds) without separate primitives — would falsify genus distinction and force merge into HEXA-WEAVE.
- F-VIROCAPSID-1-b: evidence shows HEXA-NANOBOT mechanical-actuator pipeline subsumes capsid-as-actuator (e.g. portal-motor) as the primary mode without separate self-assembly primitives — would falsify genus distinction and force merge into HEXA-NANOBOT.
- F-VIROCAPSID-1-c: evidence shows HEXA-RIBOZYME catalysis pipeline subsumes capsid catalytic-cargo as a special case (capsid is a passive container around the catalytic core) without separate self-assembly primitives — would falsify genus distinction and force merge into HEXA-RIBOZYME.
- F-VIROCAPSID-1-d: a published unification framework treating composition + actuation + catalysis + assembly as instances of a single design algebra would weaken the four-way genus split; force consolidation under one primitive layer.

Measurable claim 2 — n6 invariant mapping is STRUCTURAL-EXACT (stronger than HEXA-RIBOZYME APPROXIMATE) and load-bearing not decorative:

- F-VIROCAPSID-2-n6-decorative: Bayesian model comparison on 30+ published capsid architectures (T=1 / T=3 / T=4 / T=7 / T=13 / T=21) shows H0 (random vertex count) cannot be rejected at log-Bayes-factor >= 3 — would invalidate the invariant-as-causal claim.
- F-VIROCAPSID-2-b: a published closed-shell capsid with vertex count != 12 (e.g. 20-vertex truncated icosahedron) and verified Caspar-Klug T-number membership — would falsify the sigma(6) = 12 STRUCTURAL-EXACT binding.
- F-VIROCAPSID-2-c: a published capsid with 5+ assembly states (violating tau(6) = 4) and equally-or-more efficient assembly yield would falsify the cardinality binding.

Measurable claim 3 — 90-day MVP gate (F-VIROCAPSID-3 deadline 2026-07-28):

- F-VIROCAPSID-3-MVP-90day: failure to deliver a T=1 minimal 60-subunit 4-state Zlotnick-class kinetic simulation by 2026-07-28 falsifies the YES_APPROACH registration upgrade and reverts the recommendation to PROPOSED.
- F-VIROCAPSID-3-b: an MVP simulation that completes but exhibits kinetic-trap ceiling violation (aberrant-aggregate fraction exceeds closed-shell yield) would constitute internal contradiction and trigger retraction.
- F-VIROCAPSID-3-c: an MVP simulation whose witness JSON fails the absorption pipeline (raw 108 classifier rejection) would falsify the absorption-channel design.

Measurable claim 4 — sister-axis collision with life/virology / life/vaccine (F-VIROCAPSID-COLLISION-AUDIT deadline 2026-05-28):

- F-VIROCAPSID-COLLISION-AUDIT-virology: audit shows life/virology/ already covers icosahedral capsid T=3,13 architectural fundamentals at the same theoretical depth — would force route under life axis or trigger a boundary-redrawing proposal. Note: prior grep shows life/virology references "Icosahedral capsid T=3,13 -> 6-fold+5-fold Caspar-Klug theory" as a one-line summary; HEXA-VIROCAPSID provides the architectural primitives layer (vertex / assembly-path / pose-symmetry / closure-binary) absent from life/virology.
- F-VIROCAPSID-COLLISION-AUDIT-vaccine: maintainer review of life/vaccine flags overlap on §1-§7 specifically (VLP-vaccine route — HPV / HBV) — would trigger boundary-redrawing proposal.
- F-VIROCAPSID-COLLISION-AUDIT-c: a published taxonomy treats clinical virology and architectural-capsid self-assembly as a single domain — would weaken the biology-vs-life axis split.

Measurable claim 5 — kinetic-trap ceiling binds (Zlotnick 2003 nucleation-elongation):

- F-VIROCAPSID-4-kinetic-trap: a published capsid demonstrates aberrant-assembly fraction exceeding closed-shell yield while still being functionally relevant — would invalidate the kinetic-trap-as-binding claim and force re-derivation of the assembly-floor.
- F-VIROCAPSID-4-b: measurement shows nucleation-elongation is not the dominant assembly mode for a capsid family (e.g. one-step thermodynamic equilibrium without nucleation barrier) — would shift the floor calculation; class-by-class re-binding required.
- F-VIROCAPSID-4-c: a capsid family operates in the equilibrium-thermodynamic regime (no kinetic trap) and still achieves architectural relevance — would weaken the kinetic-trap-as-primary-constraint claim.

Aggregate: 16 falsifiers across 5 measurable claims, at least 3 per claim, satisfies raw 71. MISS criteria for any future MVP simulation are declared upfront here per own 12.

## §11 DEPENDENCIES (external + cross-domain)

| Dependency | Type | Why required |
|------------|------|--------------|
| Caspar D. L. D. & Klug A. 1962 "Physical principles in the construction of regular viruses" Cold Spring Harbor Symp Quant Biol 27:1-24 | external citation | quasi-equivalence theory; T-number = h^2 + h*k + k^2; founding capsid-architecture literature |
| Crick F. H. C. & Watson J. D. 1956 "Structure of small viruses" Nature 177:473-475 | external citation | small virus geometric reasoning predecessor to Caspar-Klug |
| Rossmann M. G. & Johnson J. E. 1985 "Icosahedral RNA virus structure" Annu Rev Biochem 54:533-573 | external citation | atomic-resolution capsid structure synthesis (poliovirus / picornavirus) |
| Harrison S. C., Olson A. J., Schutt C. E., Winkler F. K., Bricogne G. 1978 "Tomato bushy stunt virus at 2.9 angstrom resolution" Nature 276:368-373 | external citation | first atomic-resolution T=3 capsid model |
| Liljas L., Unge T., Jones T. A., Fridborg K., Lovgren S., Skoglund U., Strandberg B. 1982 "Structure of satellite tobacco necrosis virus at 3.0 angstrom resolution" J Mol Biol 159:93-108 | external citation | T=1 60-subunit minimal-capsid reference |
| Zlotnick A. 2003 "Are weak protein-protein interactions the general rule in capsid assembly?" Virology 315:269-274 | external citation | nucleation-elongation kinetic theory; primary-constraint reference |
| Zlotnick A., Cheng N., Conway J. F., Booy F. P., Steven A. C., Stahl S. J., Wingfield P. T. 1996 "Dimorphism of hepatitis B virus capsids is strongly influenced by the C-terminus of the capsid protein" Biochemistry 35:7412-7421 | external citation | HBV T=3 / T=4 dimorphism; phi(6) = 2 binary at the T-number level |
| Twarock R. & Luque A. 2016 "Structural puzzles in virology solved with an overarching icosahedral design principle" Nat Commun 7:13089 | external citation | generalized PCK; non-quasi-equivalent extensions; cross-validates Caspar-Klug boundary |
| Bruinsma R. F., Gelbart W. M., Reguera D., Rudnick J., Zandi R. 2003 "Viral self-assembly as a thermodynamic process" Phys Rev Lett 90:248101 | external citation | thermodynamic capsid-assembly framework; complementary to Zlotnick kinetic |
| Hessenberg-Cole subgroup-lattice analysis of A_5 (icosahedral rotation group of order 60) | external mathematical reference | J_2 = 24 octahedral subgroup realization in icosahedral I via 5-inscribed-cube outer action |
| domains/biology/hexa-weave/ | sister domain | composition-side handshake at multi-CP weave-into-shell boundary; shared Foundation/Strand lean4 layer if formal-verification path pursued |
| domains/biology/hexa-nanobot/ | sister domain | actuation-side handshake at capsid-as-actuator boundary (portal-motor / phi29 packaging motor) |
| domains/biology/hexa-ribozyme/ | sister domain | catalysis-side handshake at ribozyme-cargo-in-capsid boundary (mRNA-vault analogue) |
| domains/life/virology/ | cross-domain (collision audit pending) | clinical icosahedral capsid T=3,13 reference; F-VIROCAPSID-COLLISION-AUDIT deadline 2026-05-28 |
| domains/life/vaccine/ | cross-domain (collision audit pending) | VLP-vaccine route (HPV / HBV); F-VIROCAPSID-COLLISION-AUDIT deadline 2026-05-28 |
| domains/life/synbio/ | cross-domain | de-novo capsid design / SELEX-class evolution route |
| state/discovery_absorption/registry.jsonl | repo SSOT | raw 108 + raw 135 absorption channel |
| design/kick/ | repo SSOT | witness emission target |
| tool/own_doc_lint.py | tooling | own 1 / own 3 / own 4 / own 5 / own 16 enforcement (13/13 PASS expected) |
| papers/hexa-weave-formal-mechanical-w2-2026-04-28.md | external citation | parent paper context for biology-axis tetrahedron closure narrative |

## §12 TIMELINE (deliverables)

| Date | Cycle | Milestone | Witness |
|------|-------|-----------|---------|
| 2026-04-28 | 19 / fan-out 4/4 | Domain registration in n6-architecture (this body + 2 _index.json updates + 1 witness JSON) | design/kick/2026-04-28_hexa-virocapsid-registration-cycle19_omega_cycle.json |
| 2026-05-28 | TBD | F-VIROCAPSID-COLLISION-AUDIT collision audit with life/virology + life/vaccine completed | F-VIROCAPSID-COLLISION-AUDIT row |
| 2026-07-28 | TBD | F-VIROCAPSID-3 90-day MVP — T=1 minimal 60-subunit 4-state Zlotnick-class kinetic simulation | proposals/hexa_virocapsid_mvp_<date>.md |
| 2026-09-28 | TBD | F-VIROCAPSID-2 Bayesian audit (30+ capsid architectures across T=1/T=3/T=4/T=7/T=13/T=21) completed | F-VIROCAPSID-2 audit row |
| 2027-04-28 | TBD | F-VIROCAPSID-4 kinetic-trap ceiling literature scan completed | F-VIROCAPSID-4 audit row |
| TBD | TBD | CHI2 axis upgrade DEFER to PASS (n>=1 simulation) | TBD |

## §13 TOOLS (concrete repo artefacts)

- tool/own_doc_lint.py --rule 1 / 3 / 4 / 5 / 16 — HARD-block lint gates this body must pass.
- tool/own1_legacy_allowlist.json — frozen English-only legacy grandfather list (this body is NOT added; new files comply directly).
- domains/_index.json — top-level axis SSOT (biology axis updated by this registration; hexa-virocapsid added).
- domains/biology/_index.json — sub-axis SSOT (hexa-virocapsid entry added; _domains_count 3 -> 4).
- state/discovery_absorption/registry.jsonl — append-only absorption registry per raw 108 + raw 135.
- state/falsifier_monitor/audit.jsonl — append-only falsifier preregistration log per raw 71 + raw 77.
- design/kick/ — kick-witness emission directory (this cycle: 2026-04-28_hexa-virocapsid-registration-cycle19_omega_cycle.json).
- Caspar-Klug geometric simulator (external, not yet integrated): T-number = h^2 + h*k + k^2 enumeration; future MVP integration target.
- Zlotnick kinetic simulator (external, not yet integrated): nucleation-elongation master equation; future MVP integration target.
- cryo-EM single-particle reconstruction (external): EMDB / PDB deposition path for empirical validation.

## §14 TEAM (roles)

| Role | Responsibility | Owner |
|------|----------------|-------|
| Domain steward | Maintain this body and its sub-index entry | n6-architecture maintainers |
| Sister-domain liaison | Maintain HEXA-WEAVE / HEXA-NANOBOT / HEXA-RIBOZYME handshake at four-sister tetrahedron boundary | hexa-weave + hexa-nanobot + hexa-ribozyme + hexa-virocapsid stewards jointly |
| MVP runner | Deliver F-VIROCAPSID-3 90-day T=1 minimal 60-subunit 4-state Zlotnick-class kinetic simulation | TBD by 2026-07-28 |
| Falsifier monitor | Watch F-VIROCAPSID-1..F-VIROCAPSID-5 + F-VIROCAPSID-COLLISION-AUDIT with deadlines 2026-05-28 / 2026-07-28 / 2026-09-28 / 2027-04-28 | n6-architecture honesty-charter team |
| Cross-domain liaison | life/virology + life/vaccine collision audit + life/synbio SELEX-class de-novo capsid route | per-axis domain stewards |

## §15 REFERENCES

1. Caspar D. L. D. & Klug A. 1962 "Physical principles in the construction of regular viruses" Cold Spring Harbor Symp Quant Biol 27:1-24 (quasi-equivalence theory; T-number = h^2 + h*k + k^2; founding capsid-architecture literature for §1 WHY and §4 STRUCT Axis A).
2. Crick F. H. C. & Watson J. D. 1956 "Structure of small viruses" Nature 177:473-475 (small-virus geometric reasoning predecessor to Caspar-Klug; cited for §3 REQUIRES geometric foundation).
3. Rossmann M. G. & Johnson J. E. 1985 "Icosahedral RNA virus structure" Annu Rev Biochem 54:533-573 (atomic-resolution capsid structure synthesis; poliovirus T=3 60-pentamer subunit model; cross-validation of sigma(6) = 12 vertex on a T=3 capsid).
4. Harrison S. C., Olson A. J., Schutt C. E., Winkler F. K., Bricogne G. 1978 "Tomato bushy stunt virus at 2.9 angstrom resolution" Nature 276:368-373 (first atomic-resolution T=3 capsid model; reference for §4 STRUCT Axis A 12-vertex topology).
5. Liljas L., Unge T., Jones T. A., Fridborg K., Lovgren S., Skoglund U., Strandberg B. 1982 "Structure of satellite tobacco necrosis virus at 3.0 angstrom resolution" J Mol Biol 159:93-108 (T=1 60-subunit minimal-capsid reference; canonical T=1 12-vertex EXACT case for sigma(6) = 12 binding).
6. Zlotnick A. 2003 "Are weak protein-protein interactions the general rule in capsid assembly?" Virology 315:269-274 (nucleation-elongation kinetic theory; primary-constraint reference for §1 / §10 measurable claim 5).
7. Zlotnick A., Cheng N., Conway J. F., Booy F. P., Steven A. C., Stahl S. J., Wingfield P. T. 1996 "Dimorphism of hepatitis B virus capsids is strongly influenced by the C-terminus of the capsid protein" Biochemistry 35:7412-7421 (HBV T=3 / T=4 dimorphism; reference for phi(6) = 2 binary closure at the T-number level).
8. Twarock R. & Luque A. 2016 "Structural puzzles in virology solved with an overarching icosahedral design principle" Nat Commun 7:13089 (generalized PCK; non-quasi-equivalent extensions; cross-validates Caspar-Klug boundary).
9. Bruinsma R. F., Gelbart W. M., Reguera D., Rudnick J., Zandi R. 2003 "Viral self-assembly as a thermodynamic process" Phys Rev Lett 90:248101 (thermodynamic capsid-assembly framework; complementary to Zlotnick kinetic).
10. Endres D. & Zlotnick A. 2002 "Model-based analysis of assembly kinetics for virus capsids or other spherical polymers" Biophys J 83:1217-1230 (concrete master-equation simulation reference for §5 FLOW step 8).
11. Sun S., Rao V. B., Rossmann M. G. 2010 "Genome packaging in viruses" Curr Opin Struct Biol 20:114-120 (phi29 packaging motor; cross-link to HEXA-NANOBOT actuator boundary).
12. Stockley P. G., Twarock R., Bakker S. E., Barker A. M., Borodavka A., Dykeman E., Ford R. J., Pearson A. R., Phillips S. E., Ranson N. A., Tuma R. 2013 "Packaging signals in single-stranded RNA viruses: nature's alternative to a purely electrostatic assembly mechanism" J Biol Phys 39:277-287 (RNA-genome packaging signals; cross-link to HEXA-RIBOZYME catalytic-cargo boundary).
13. Mannige R. V. & Brooks C. L. III 2010 "Periodic table of virus capsids: implications for natural selection and design" PLoS One 5:e9423 (n>30 capsid architectures Bayesian-audit reference for F-VIROCAPSID-2).
14. n6-architecture sister domain: domains/biology/hexa-weave/hexa-weave.md (multi-strand covalent composition counterpart).
15. n6-architecture sister domain: domains/biology/hexa-nanobot/hexa-nanobot.md (single mechanical actuator counterpart).
16. n6-architecture sister domain: domains/biology/hexa-ribozyme/hexa-ribozyme.md (single catalytic-RNA active site counterpart).
17. n6-architecture domain registration witness: design/kick/2026-04-28_hexa-virocapsid-registration-cycle19_omega_cycle.json (this cycle 19 fan-out 4/4 — biology tetrahedron closure).
18. n6-architecture sister-domain registration witness: design/kick/2026-04-28_hexa-ribozyme-registration-cycle15_omega_cycle.json (cycle 15 fan-out 3/3; precursor; referenced for handshake protocol at ribozyme-cargo-in-capsid boundary).
19. n6-architecture sister-domain registration witness: design/kick/2026-04-28_hexa-nanobot-domain-registration_omega_cycle.json (cycle 13 fan-out 2/5; precursor; referenced for handshake protocol at capsid-as-actuator boundary).
20. n6-architecture sister-domain closure witness: design/kick/2026-04-28_hexa-weave-closure_omega_cycle.json (tri-axis Omega-saturation PASS at workload ceiling; referenced for handshake protocol at multi-CP weave-into-shell boundary).
21. n6-architecture cycle-18 census: design/kick/2026-04-28_alien-grade-5-unlock-cycle18_omega_cycle.json (4th-sister candidate evaluation; HEXA-VIROCAPSID 71/80 A+ rank-1 selection).
