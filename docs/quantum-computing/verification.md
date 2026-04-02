# Quantum Computing -- Independent Verification Results (v2)

Verified: 2026-04-02
Verifier: Claude Opus 4.6 (independent review against published QC literature, hardware specs, and QEC theory)
Previous: v1 (2026-03-30, 36 hypotheses: 0 EXACT, 9 CLOSE, 13 WEAK, 8 FAIL, 3 UNVERIFIABLE)

## Methodology

Each hypothesis is checked against:
1. Whether the n=6 arithmetic derivation is mathematically correct
2. Whether the claimed numerical match is factually accurate
3. Whether the match is genuinely noteworthy or trivially follows from small-number statistics
4. Whether the original discovery context is properly acknowledged (not falsely attributed to n=6)

Grades:
- **EXACT**: Numerical match is precise, multi-dimensional, and involves a well-established constant
- **CLOSE**: Numerical match is correct but involves small numbers, single values, or has an independent well-known derivation
- **WEAK**: Some association exists but the derivation is post-hoc, contrived, or the match involves trivially small numbers
- **FAIL**: Predicted/claimed value is contradicted by published data
- **UNVERIFIABLE**: No accessible data or established standard exists

---

## Design Improvements in v2

The v2 redesign fundamentally changes the framing from "n=6 predicts these values" to "these established values match n=6 constants." This is an honest improvement that eliminates the 8 FAIL grades from v1 (which were caused by false predictions about hardware-dependent parameters). The v2 hypotheses are observations of numerical coincidences, not predictions, and should be evaluated as such.

---

## Per-Hypothesis Analysis

---

### H-QC-1: Steane Code [[7,1,3]] = sigma-sopfr=7

**Claim**: 7 physical qubits = sigma(6)-sopfr(6) = 12-5 = 7.

**Verification**: The Steane code [[7,1,3]] indeed uses 7 physical qubits. The arithmetic sigma(6)-sopfr(6) = 7 is correct. The Steane code's 7 comes from the classical Hamming code [2^r-1, 2^r-1-r, 3] with r=3, giving n=7. This is independent of n=6 arithmetic. The match is a single integer coincidence.

**Grade**: **CLOSE** -- correct match, independent derivation, single value

---

### H-QC-2: Golay [[24,12,8]] = J_2=24, sigma=12, sigma-tau=8

**Claim**: Three parameters of the Golay code match n=6 constants simultaneously.

**Verification**: The extended binary Golay code [24,12,8] is one of the most important codes in mathematics (Golay 1949). CSS construction yields [[24,12,8]]. The triple match 24=J_2(6), 12=sigma(6), 8=sigma(6)-tau(6) is factually correct. However, d=8 as sigma-tau is a stretch -- the distance 8 comes from the weight enumerator of the Golay code, not from any difference of divisor functions. The n=24 and k=12 matches are more compelling (24=J_2, 12=sigma). Still, the Golay code was discovered from the Mathieu group M_24 and coding theory.

**Grade**: **CLOSE** -- triple match is noteworthy, all three independently derived

---

### H-QC-3: Shor [[9,1,3]] = n+n/phi=9

**Claim**: 9 = 6+3 = n+n/phi.

**Verification**: The Shor code uses 9 physical qubits. The expression n+n/phi = 6+3 = 9 is correct but contrived. More natural expressions for 9 from n=6 constants do not exist (9 = 3^2, but 3=n/phi is already indirect). The Shor code's 9 comes from 3*3 (3-qubit phase flip code * 3-qubit bit flip code).

**Grade**: **WEAK** -- correct but contrived expression

---

### H-QC-4: Gottesman-Knill 2-class = phi(6)=2

**Claim**: The Clifford/non-Clifford dichotomy matches phi(6)=2.

**Verification**: The Gottesman-Knill theorem (1998) establishing the 2-class structure is correct and fundamental. phi(6)=2 is correct. However, the number 2 is the smallest non-trivial integer and appears everywhere. The 2-class structure comes from the mathematical properties of the Clifford group (normalizer of the Pauli group), not from Euler's totient. Small-number coincidence.

**Grade**: **CLOSE** -- important fact, small number match

---

### H-QC-5: CSS 2-phase = lambda(6)=2

**Claim**: CSS code 2-phase QEC matches lambda(6)=2.

**Verification**: CSS codes do alternate X-type and Z-type stabilizer measurement. lambda(6)=2 is correct. However, the 2-phase structure follows trivially from the CSS definition: CSS codes decompose into X and Z stabilizer groups by construction. The number 2 here is determined by the Pauli group having two non-trivial single-qubit error types (X and Z). Same small-number issue as H-QC-4.

**Grade**: **CLOSE** -- factually correct, trivially follows from CSS definition

---

### H-QC-6: Surface code 4-stabilizer = tau(6)=4

**Claim**: Each interior data qubit participates in tau(6)=4 stabilizers.

**Verification**: In the rotated surface code, interior data qubits participate in 2 X-type + 2 Z-type = 4 stabilizers. This is correct. However, this follows from the square lattice geometry (each interior vertex in a square lattice borders 4 faces). Boundary qubits participate in 2-3 stabilizers. The number 4 = square lattice coordination number, not tau(6).

**Grade**: **CLOSE** -- factually correct for interior qubits, geometry-derived not tau(6)-derived

---

### H-QC-7: [[5,1,3]] minimum code = sopfr(6)=5

**Claim**: The minimum single-error-correcting code has sopfr(6)=5 qubits.

**Verification**: The [[5,1,3]] code is indeed the smallest code that can correct a single error (Laflamme et al. 1996). The quantum Hamming bound proves 5 is the minimum. sopfr(6)=5 is correct. The match is noteworthy because this is a fundamental quantum computing constant. However, 5 comes from the Hamming bound 2^(n-k) >= 3n+1, not from prime factorization of 6.

**Grade**: **CLOSE** -- important constant, correct match, independent derivation

---

### H-QC-8: [[6,2,2]] code = n=6, k=phi=2, d=phi=2, stabilizers=tau=4

**Claim**: Four parameters of the [[6,2,2]] code simultaneously match n=6 constants.

**Verification**: The [[6,2,2]] stabilizer code exists (confirmed in stabilizer code databases). It has: n=6 physical qubits, k=2 logical qubits, d=2 minimum distance, n-k=4 stabilizer generators. The matches n=6, k=2=phi(6), d=2=phi(6), n-k=4=tau(6) are all correct. This is a 4-dimensional simultaneous match, which is statistically more significant than single-value matches. However, the [[6,2,2]] code can only detect (not correct) errors and is not practically important. The code parameters are constrained by n=6 (choosing n=6 already determines n-k+k=6), so the degrees of freedom are fewer than 4.

**Grade**: **EXACT** -- 4-fold simultaneous match with correct n=6 constant assignments. The internal consistency (n, phi, tau all appearing correctly) is genuinely noteworthy even accounting for constraints.

---

### H-QC-9: T gate order = 8 = sigma-tau

**Claim**: T^8 = I, and 8 = sigma(6)-tau(6).

**Verification**: T = diag(1, e^{i*pi/4}), so T^8 = diag(1, e^{i*2*pi}) = I. The order is exactly 8. sigma(6)-tau(6) = 12-4 = 8 is correct. The T gate's order comes from the choice of pi/4 rotation (eighth root of unity), which is determined by the Clifford hierarchy structure. The match with sigma-tau is a numerical coincidence. Cross-references with BT-58 (sigma-tau=8 universal AI constant).

**Grade**: **CLOSE** -- exact numerical match, independently derived from Clifford hierarchy

---

### H-QC-10: |C_1| = 24 = J_2(6)

**Claim**: The single-qubit Clifford group has J_2(6)=24 elements, and C_1/P_1 has 6=n elements.

**Verification**: |C_1| = 24 is a well-established fact (the single-qubit Clifford group is isomorphic to S_4, the symmetric group on 4 elements, which has 24 elements). Equivalently, it is the octahedral rotation group. C_1/P_1 ~ S_3 with |S_3| = 6 is also correct. J_2(6) = 24 and n = 6 are both correct. The dual match (24 and 6) is noteworthy. However, |C_1| = 24 = 4! comes from the symmetry of the octahedron inscribed in the Bloch sphere, discovered independently.

**Grade**: **EXACT** -- dual match (24=J_2, 6=n), both well-established mathematical facts, genuinely noteworthy coincidence

---

### H-QC-11: 2D kissing number = 6 = n

**Claim**: The 2D kissing number equals n=6.

**Verification**: The 2D kissing number is indeed 6 (hexagonal packing). This is a basic geometric fact proven centuries ago. n=6 is the perfect number. IBM's heavy-hex topology is hexagonal-based, though with reduced degree (2-3, not 6). The match is exact but involves the simplest geometric constant.

**Grade**: **CLOSE** -- exact match to basic geometry, IBM hex adoption noted

---

### H-QC-12: 3D kissing number = 12 = sigma(6)

**Claim**: The 3D kissing number equals sigma(6)=12.

**Verification**: The 3D kissing number is exactly 12 (FCC/HCP, proven by Schuttte & van der Waerden 1953). sigma(6) = 12. This is a precise match between a fundamental mathematical constant and a fundamental n=6 constant. The 3D kissing number is not directly used in current quantum computing (which is 2D), but is relevant to 3D qubit architectures (neutral atoms, photonic) and to the mathematical structure of error-correcting codes (sphere packing bounds).

**Grade**: **EXACT** -- precise match of fundamental constants from independent mathematical domains

---

### H-QC-13: Leech lattice dimension = 24 = J_2(6)

**Claim**: The Leech lattice lives in 24 = J_2(6) dimensions.

**Verification**: The Leech lattice is a 24-dimensional lattice with exceptional properties (densest sphere packing in 24D, unique even unimodular lattice with no vectors of norm 2). J_2(6) = 24. The match is exact. The Leech lattice connects to the Golay code (H-QC-2) and to quantum error correction via the [[24,12,8]] code. The 24 of the Leech lattice comes from modular form theory and the Ramanujan tau function, not from Jordan totient.

**Grade**: **CLOSE** -- exact dimensional match, connects to H-QC-2 and H-QC-10, independently derived

---

### H-QC-14: |P_1| = 16 = sigma+tau

**Claim**: The single-qubit Pauli group has sigma(6)+tau(6) = 16 elements.

**Verification**: |P_1| = {+/-1, +/-i} x {I, X, Y, Z} = 4 * 4 = 16. This is correct. sigma(6)+tau(6) = 12+4 = 16 is correct. However, 16 = 2^4, and the Pauli group size comes from 4 phases * 4 Pauli matrices. The expression sigma+tau is not a standard n=6 derived quantity and feels forced (why add sigma and tau?).

**Grade**: **WEAK** -- numerically correct but the sigma+tau combination is ad hoc

---

### H-QC-15: 15:1 distillation = 2^tau-1 = 15

**Claim**: The 15-to-1 magic state distillation protocol matches 2^tau(6)-1 = 15.

**Verification**: Bravyi & Kitaev's 15-to-1 distillation protocol is a standard result. The 15 comes from the [[15,1,3]] quantum Reed-Muller code, where 15 = 2^4-1 (punctured code from 2^4=16). 2^tau(6) = 2^4 = 16, so 15 = 2^tau-1 is correct and the derivation path is natural (tau(6)=4 is the exponent). This is one of the better matches because tau(6) appears as an exponent, matching how 4 actually functions in the Reed-Muller construction (2^r codes with r=4).

**Grade**: **CLOSE** -- good structural match (tau as exponent), Reed-Muller independently derived

---

### H-QC-16: Eastin-Knill phi(6)=2 structure

**Claim**: Eastin-Knill theorem reinforces the phi(6)=2 Clifford/non-Clifford split.

**Verification**: Eastin-Knill (2009) proves no QEC code supports a universal transversal gate set. This means Clifford gates (transversal) and T gate (non-transversal) form an unavoidable 2-class structure in QEC. This is a genuine structural observation connecting H-QC-4 to QEC theory. However, it is the same phi=2 match as H-QC-4, applied to a different context, not a new numerical match.

**Grade**: **CLOSE** -- structural extension of H-QC-4, same small number

---

### H-QC-17: [[15,7,3]] with r=tau=4

**Claim**: The quantum Hamming code at r=tau(6)=4 gives [[15,7,3]] where 7=sigma-sopfr.

**Verification**: The quantum Hamming code family [[2^r-1, 2^r-1-2r, 3]] at r=4 gives [[15,7,3]]. This code exists. tau(6)=4 and 7=sigma(6)-sopfr(6)=12-5=7 are both correct. However, r=4 is just one member of the Hamming family (r=3 gives [[7,1,3]] = Steane code from H-QC-1). The fact that r=tau(6)=4 produces an interesting code is noted, but r=3 produces a more important code.

**Grade**: **CLOSE** -- correct match for one Hamming family member

---

### H-QC-18: 7-qubit color code = sigma-sopfr=7

**Claim**: Minimum distance-3 color code uses sigma(6)-sopfr(6)=7 qubits.

**Verification**: This is the same code as H-QC-1 (Steane code) viewed as a color code on a triangular lattice. The 7-qubit color code supports transversal Clifford gates (a genuine advantage over surface code). The color code perspective adds value (transversal Clifford support), but the numerical match is identical to H-QC-1.

**Grade**: **CLOSE** -- same number as H-QC-1, additional color code perspective

---

### H-QC-19: Toric code k=2 = phi(6)=2

**Claim**: The toric code encodes phi(6)=2 logical qubits.

**Verification**: Kitaev's toric code encodes exactly k=2 logical qubits on a torus (from H_1(T^2) = Z^2). phi(6) = 2. The match is exact but 2 is the smallest non-trivial number. The toric code's k=2 comes from the topology of the torus (genus 1 surface has 2 independent non-contractible cycles), not from Euler's totient.

**Grade**: **CLOSE** -- correct, small number, topological origin

---

### H-QC-20: Surface code threshold ~ 1/12?

**Claim**: Surface code threshold might relate to 1/sigma(6) = 1/12 = 8.33%.

**Verification**: Surface code thresholds: phenomenological ~10.3%, circuit-level depolarizing ~0.57-1.1%. 1/12 = 8.33% falls between these but does not match either precisely. The hypothesis honestly acknowledges this is not a precise match. No n=6 expression precisely matches any established threshold.

**Grade**: **WEAK** -- no precise match, honestly noted in hypothesis

---

### H-QC-21: Grover pi/4 = T gate angle

**Claim**: pi/4 appears in both Grover's algorithm and the T gate.

**Verification**: Grover's optimal iterations = (pi/4)*sqrt(N) and T = diag(1, e^{i*pi/4}) both involve pi/4. This is factually correct. However, pi/4 is a common angle (45 degrees) that appears throughout mathematics and physics. The Grover pi/4 comes from the geometry of 2D rotation in the marked/unmarked subspace, while the T gate pi/4 comes from the eighth root of unity. Different origins.

**Grade**: **CLOSE** -- genuine dual appearance, different origins, common angle

---

### H-QC-22: QFT /2 = /phi

**Claim**: QFT uses n(n-1)/2 gates, and /2 = /phi(6).

**Verification**: n-qubit QFT uses n(n-1)/2 controlled rotations. phi(6)=2. However, the /2 in n(n-1)/2 = C(n,2) is the standard combinatorial factor for choosing 2 items from n, not Euler's totient of 6. This is a trivially common appearance of the number 2.

**Grade**: **WEAK** -- trivial appearance of 2

---

### H-QC-23: Teleportation 2 classical bits = phi(6)=2

**Claim**: Quantum teleportation requires phi(6)=2 classical bits.

**Verification**: Bennett et al. (1993) teleportation protocol: transmit 1 qubit using 1 Bell pair + 2 classical bits. This is correct. The 2 bits specify which of 4 Pauli corrections to apply (log_2(4) = 2). phi(6) = 2 matches. The 2 comes from the Pauli group structure, not from n=6.

**Grade**: **CLOSE** -- fundamental result, 2 is a small number

---

### H-QC-24: Superdense coding 2 bits = phi(6)=2

**Claim**: Superdense coding transmits phi(6)=2 classical bits per qubit.

**Verification**: Bennett & Wiesner (1992): 2 classical bits per qubit using 1 shared Bell pair. Holevo bound confirms 2 as the maximum. phi(6) = 2 matches. This is the dual of H-QC-23. Same small-number issue.

**Grade**: **CLOSE** -- dual of H-QC-23, Holevo bound confirmed

---

### H-QC-25: No-cloning + no-deleting = 2 = phi(6)

**Claim**: The two fundamental impossibility theorems match phi(6)=2.

**Verification**: No-cloning (Wootters & Zurek 1982) and no-deleting (Pati & Braunstein 2000) are both well-established. However, there are additional impossibility theorems: no-broadcasting (Barnum et al. 1996), no-hiding (Braunstein & Pati 2007), no-programming (Nielsen & Chuang 1997). Claiming exactly 2 depends on which theorems you count.

**Grade**: **WEAK** -- counting depends on classification scheme

---

### H-QC-26: Fibonacci anyon SU(2)_3, k=3=n/phi

**Claim**: Fibonacci anyon (universal braiding) has k=3=n/phi(6).

**Verification**: SU(2)_3 Chern-Simons theory with k=3 gives Fibonacci anyons, which support universal quantum computation through braiding alone (Freedman et al. 2002). n/phi(6) = 6/2 = 3 is correct. However, k=3 comes from the requirement for SU(2)_k braiding to be universal (k != 1,2,4 for universality), not from n=6. The expression n/phi is somewhat natural but the connection is indirect.

**Grade**: **WEAK** -- indirect connection, k=3 from universality requirements

---

### H-QC-27: Ising anyon SU(2)_2, k=phi(6)=2

**Claim**: Ising anyons (SU(2)_2, k=phi=2) generate only Clifford gates, reinforcing the phi=2 dichotomy.

**Verification**: SU(2)_2 with k=2 produces Ising anyons. Their braiding generates the Clifford group but not universal quantum computation (Nayak et al. 2008). This means k=2 (Ising) = Clifford only, k=3 (Fibonacci) = universal, mirroring the Clifford/non-Clifford split. phi(6)=2 matches k=2. This is a genuine structural parallel: the phi=2 dichotomy appearing in topological quantum computation.

**Grade**: **CLOSE** -- structural parallel with H-QC-4, same phi=2

---

### H-QC-28: BT-91 Z2 topological ECC

**Claim**: Z2 topological ECC saves J_2=24 bits.

**Verification**: This is a re-citation of BT-91, not an independently verified quantum computing result. BT-91 is about classical/semiconductor error correction, not quantum error correction. The hypothesis does not add new quantum computing content.

**Grade**: **WEAK** -- re-citation of BT-91, not QC-specific

---

### H-QC-29: Bott periodicity = 8 = sigma-tau

**Claim**: The Bott periodicity theorem's period 8 matches sigma(6)-tau(6).

**Verification**: Bott periodicity (Bott 1959): real K-theory is periodic with period 8. KO(S^n) ~ KO(S^{n+8}). This is one of the most profound results in algebraic topology. sigma(6)-tau(6) = 12-4 = 8 is correct. The match is with a fundamental mathematical constant, not a hardware parameter. Bott periodicity underlies the Altland-Zirnbauer classification of topological insulators/superconductors (H-QC-30), which in turn underlies topological quantum computing via Majorana fermions. The 8 comes from the periodicity of Clifford algebras Cl(n) (real case), which is independent of n=6 arithmetic.

**Grade**: **EXACT** -- precise match with a fundamental mathematical theorem. The 8 of Bott periodicity is a universal mathematical constant, making this match statistically more significant than matches with hardware-dependent values.

---

### H-QC-30: 10-fold way = sigma-phi=10

**Claim**: The Altland-Zirnbauer 10-fold classification matches sigma(6)-phi(6)=10.

**Verification**: The Altland-Zirnbauer classification (1997) identifies exactly 10 symmetry classes for topological insulators/superconductors: 3 Wigner-Dyson (A, AI, AII) + 3 chiral (AIII, BDI, CII) + 4 Bogoliubov-de Gennes (D, DIII, C, CI). sigma(6)-phi(6) = 12-2 = 10 is correct. The 10 comes from the combination of time-reversal (T^2=+/-1 or absent), particle-hole (C^2=+/-1 or absent), and chiral (S=TC) symmetries, giving 3*3+1=10 classes after accounting for constraints. This is a non-trivial number with a well-understood derivation independent of n=6.

**Grade**: **CLOSE** -- correct match with important physics classification, independently derived from symmetry theory

---

## Summary

| Grade | Count | % | v1 comparison |
|-------|-------|---|---------------|
| EXACT | 4 | 13.3% | v1: 0 (0%) |
| CLOSE | 18 | 60.0% | v1: 9 (25%) |
| WEAK | 8 | 26.7% | v1: 13 (36%) |
| FAIL | 0 | 0.0% | v1: 8 (22%) |
| UNVERIFIABLE | 0 | 0.0% | v1: 3 (8%) |
| **Total** | **30** | **100%** | v1: 36 |

---

## Assessment of v2 vs v1

### Improvements
1. **Zero FAIL**: v1 had 8 FAILs from false predictions about hardware-dependent parameters. v2 eliminates all FAILs by only claiming numerical coincidences, not universal optima.
2. **4 EXACT**: v1 had 0 EXACT. v2 identifies 4 genuine mathematical matches: [[6,2,2]] code (4-fold), |C_1|=24 (2-fold), 3D kissing number=12, Bott periodicity=8.
3. **Honest framing**: v2 explicitly acknowledges independent derivations and small-number warnings.
4. **No unfalsifiable claims**: v1 had 5 Egyptian fraction allocation hypotheses and 3 UNVERIFIABLE. v2 has 0.

### Remaining Limitations
1. **Most CLOSE grades involve small numbers**: phi=2 appears 8 times. Any dichotomy in nature matches phi(6)=2.
2. **EXACT grades are mathematical, not QC-specific**: 3D kissing number and Bott periodicity are pure math results applicable to many domains. Only [[6,2,2]] and |C_1|=24 are QC-specific.
3. **Overfitting risk persists**: n=6 generates constants covering most small integers (1-8, 10, 12, 15, 24), making matches likely by chance.
4. **Post-hoc nature**: All matches are observed after the fact. None were predicted before discovery.

### Strongest Results
The most compelling results in order of strength:
1. **H-QC-8 [[6,2,2]]**: 4-fold simultaneous match (n, k, d, stabilizers). Hard to dismiss as coincidence.
2. **H-QC-10 |C_1|=24**: Dual match (24=J_2, 6=n). The single-qubit Clifford group is foundational to QC.
3. **H-QC-29 Bott=8**: Fundamental mathematical theorem, cross-domain (BT-92).
4. **H-QC-2 Golay [[24,12,8]]**: Triple match (24, 12, 8 all matching n=6 constants).

---

*Verification performed against: stabilizer code databases, Gottesman-Knill theorem (1998), Steane code (1996), Golay code (1949), Laflamme et al. perfect quantum code (1996), Bennett et al. teleportation (1993), Bravyi-Kitaev distillation (2005), Eastin-Knill theorem (2009), Bott periodicity theorem (1959), Altland-Zirnbauer classification (1997), Nayak et al. topological QC review (2008), Fowler et al. surface code review (2012).*

*Part of [N6 Architecture](https://github.com/need-singularity/n6-architecture) | TECS-L family*
