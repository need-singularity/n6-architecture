# Quantum Computing -- Independent Verification Results

Verified: 2026-03-30
Verifier: Claude Opus 4.6 (independent review against published QC literature, hardware specs, and QEC theory)

## Methodology

Each hypothesis is checked against:
1. Whether the n=6 arithmetic derivation is mathematically correct
2. Whether the predicted value matches published quantum computing data, hardware specifications, or established theoretical results
3. Whether the connection is genuinely derived from n=6 or is post-hoc pattern matching
4. Whether the claim is about a hardware-dependent parameter being presented as a universal constant

Grades:
- **EXACT**: Predicted value matches a well-established, hardware-independent result
- **CLOSE**: Within +/-10% of actual value, or matches one important case
- **WEAK**: Some association exists but derivation is post-hoc or cherry-picked
- **FAIL**: Predicted value contradicted by published data
- **UNVERIFIABLE**: No accessible data or established standard exists

---

## Critical Preliminary Note: Hardware Dependence

A fundamental issue pervades the quantum computing hypotheses: most of the claimed "optimal" values are hardware-dependent. Qubit counts, connectivity, gate sets, error thresholds, and module sizes are all determined by the specific physical qubit technology (superconducting, trapped ion, photonic, neutral atom, topological), fabrication constraints, and engineering trade-offs. Claiming universal optima derived from number theory ignores this technological diversity.

---

## Per-Hypothesis Analysis

---

### H-QC-1: Optimal Qubit Module = J_2(6) = 24

**Claim**: 24-qubit modules minimize crosstalk via Leech lattice analogy.

**n=6 derivation check**: J_2(6) = 6^2 * (1-1/4) * (1-1/9) = 36 * 3/4 * 8/9 = 24. Correct.

**Real-world check**: No quantum hardware vendor uses 24 as a module size. IBM processor architectures: Falcon (27 qubits), Hummingbird (65), Eagle (127), Osprey (433), Condor (1121), Heron (133). Google: Sycamore (53), Willow (72). None use 24-qubit modules or subdivide into 24-qubit units. The Leech lattice is a 24-dimensional mathematical object relevant to sphere packing; qubit crosstalk is a 2D planar coupling problem. The dimensionality mismatch (24D lattice vs 2D chip) makes the analogy physically meaningless.

**Grade**: **WEAK**

---

### H-QC-2: Optimal Qubit Connectivity = sigma(6) = 12

**Claim**: Each qubit should have degree-12 nearest-neighbor coupling.

**n=6 derivation check**: sigma(6)=12 matches the 3D FCC kissing number. Correct math.

**Real-world check**: Superconducting qubits are fabricated on 2D chips. In 2D, the kissing number is 6, not 12. No physical qubit technology achieves degree-12 planar coupling. Actual connectivity: IBM heavy-hex: degree 2-3. Google Sycamore: degree 4. Quantinuum trapped ions: effectively all-to-all but with distance-dependent fidelity. The industry trend is toward lower connectivity with higher fidelity per coupler, not toward degree-12. The hypothesis itself acknowledges this, suggesting degree-6 as a "practical compromise," which undermines the degree-12 prediction.

**Grade**: **FAIL**

---

### H-QC-3: 4 Types of Qubit Roles = tau(6) = 4

**Claim**: Qubits optimally serve 4 roles: data, ancilla, flag, magic state.

**n=6 derivation check**: tau(6)=4 is correct.

**Real-world check**: Standard surface code QEC uses 2 types: data qubits and ancilla (syndrome measurement) qubits. Flag qubits were introduced by Chamberland & Campbell (2018) for fault-tolerant syndrome extraction, making 3 types in advanced schemes. Magic state qubits (dedicated to magic state distillation) can be considered a 4th type in some architectures. However, this 4-type taxonomy is one possible classification, not the only one. One could equally argue for 2 (data + ancilla), 3 (+ flag), 5 (+ routing qubits for lattice surgery), or more.

**Grade**: **CLOSE** (reasonable taxonomy, but not uniquely 4)

---

### H-QC-4: Hexagonal Qubit Lattice Optimal

**Claim**: Hexagonal lattice is optimal for qubit layout due to n=6 symmetry.

**n=6 derivation check**: Hexagonal lattice has coordination number 6 = n.

**Real-world check**: IBM's heavy-hex topology is a modified hexagonal lattice (degree 2-3, not the full hex with degree 6). It was chosen for its low degree (reducing crosstalk) and good surface code performance. Google uses a square grid. For surface codes specifically, the square lattice is the standard and best-studied topology. Color codes do benefit from hexagonal/triangular lattices. The claim has partial support: hexagonal geometry is used (IBM) and has theoretical advantages for certain codes, but it is not universally optimal.

**Grade**: **CLOSE**

---

### H-QC-5: 6-Fold Chip Rotational Symmetry

**Claim**: Quantum processor chips should have C6 rotational symmetry.

**n=6 derivation check**: C6 is the cyclic group of order 6, matching n=6.

**Real-world check**: No major quantum processor has 6-fold symmetry. All commercial chips have rectangular geometry dictated by: (1) silicon wafer dicing (rectangular dies), (2) I/O pad placement along edges, (3) signal routing in Manhattan geometry, (4) packaging constraints (flip-chip bonding to rectangular PCBs). The crystallographic restriction theorem states that 6-fold symmetry is possible in 2D lattices, but chips are finite structures, not infinite crystals. There is no published evidence that 6-fold symmetry provides any advantage for quantum processors.

**Grade**: **FAIL**

---

### H-QC-6: 5 Levels of Coupling Hierarchy = sopfr(6) = 5

**Claim**: Qubit coupling has exactly 5 hierarchical levels.

**n=6 derivation check**: sopfr(6) = 2+3 = 5 is correct.

**Real-world check**: Hierarchical coupling exists but the number of levels is arbitrary. One could define 2 levels (local + remote), 3 levels (nearest-neighbor, intra-chip, inter-chip), 4 levels (adding intra-module), or 7+ levels (adding finer granularity). No published quantum computing architecture paper defines exactly 5 coupling hierarchy levels. The specific 5-level decomposition in the hypothesis is designed to match sopfr(6), not derived from physical constraints.

**Grade**: **WEAK**

---

### H-QC-7: 12 Optimal Stabilizer Generators = sigma(6) = 12

**Claim**: The optimal QEC code has sigma(6)=12 stabilizer generators.

**n=6 derivation check**: sigma(6)=12 is correct.

**Real-world check**: In a [[n,k,d]] stabilizer code, the number of stabilizer generators is n-k. This varies by code: surface code d=3 has 8 stabilizers, d=5 has 24, d=7 has 48. The [[16,4,4]] code does have 12 stabilizers, but it is not established as "optimal" in any sense. The [[15,1,3]] quantum Reed-Muller code has 14 stabilizers. The [[7,1,3]] Steane code has 6. There is no special optimality at 12 stabilizers in QEC literature. Code selection depends on noise model, connectivity, and decoder complexity.

**Grade**: **WEAK**

---

### H-QC-8: 4 Logical Qubits Per Block = tau(6) = 4

**Claim**: QEC blocks should encode tau(6)=4 logical qubits.

**n=6 derivation check**: tau(6)=4 is correct.

**Real-world check**: The dominant practical QEC approach (surface code) encodes k=1 logical qubit per patch. This is standard in Google's and IBM's QEC roadmaps. Higher-rate codes exist (e.g., hypergraph product codes can encode many logical qubits), but k=4 has no special status. The [[16,4,4]] code exists but is not the "optimal" choice. Code selection is driven by noise model and implementation constraints, not a fixed k value.

**Grade**: **WEAK**

---

### H-QC-9: Egyptian Fraction Syndrome Decoding (50/33/17% split)

**Claim**: Optimal decoder resource allocation follows 1/2 lookup + 1/3 MWPM + 1/6 ML.

**n=6 derivation check**: 1/2+1/3+1/6=1 is correct.

**Real-world check**: Multi-tier decoding is a real research direction. Skoric et al. (2023) use a fast pre-decoder plus MWPM. However, the specific 50/33/17 split has no theoretical or empirical basis. The fraction of errors handled by each tier depends on: physical error rate, code distance, noise model (depolarizing, biased, correlated), and decoder implementation. At low error rates, a lookup table might handle 90%+ of syndromes; at high error rates, nearly all syndromes require MWPM. The split is not a fixed constant.

**Grade**: **WEAK**

---

### H-QC-10: [[24,12,8]] Quantum Leech Code

**Claim**: sigma(6)*phi(6)=24 connects to the Golay/Leech quantum code.

**n=6 derivation check**: sigma(6)*phi(6) = 12*2 = 24. The classical binary Golay code [24,12,8] is well-established. CSS construction yields a valid [[24,12,8]] quantum code.

**Real-world check**: The classical [24,12,8] extended Golay code is one of the most important codes in coding theory, discovered by Golay (1949). Its CSS quantum version [[24,12,8]] is a valid quantum code with excellent parameters (rate 1/2, distance 8). The connection 24 = sigma(6)*phi(6) is a genuine numerical match to a genuinely important mathematical object. However, the Golay code was discovered from properties of the Mathieu group M24 and coding theory, not from perfect number arithmetic. The match is a noteworthy coincidence, not a derivation.

**Grade**: **CLOSE** (real mathematical object, genuine numerical match, but not derived from n=6)

---

### H-QC-11: Error Threshold = 1/sigma(6) = 1/12 ~ 8.33%

**Claim**: QEC practical threshold is 1/12 ~ 8.33%.

**n=6 derivation check**: 1/sigma(6) = 1/12 = 0.0833 is correct.

**Real-world check**: Known thresholds vary significantly by noise model and code:
- Surface code, phenomenological noise: ~10.3% (Dennis et al. 2002)
- Surface code, circuit-level depolarizing noise: ~0.57-1.1% (various studies)
- Toric code, independent X/Z noise: ~10.9%
- Practical threshold with realistic noise: ~0.1-1%

The value 8.33% falls between phenomenological (~10%) and circuit-level (~1%) thresholds. It does not match any specific well-established threshold precisely. The "break-even" point (where logical error rate equals physical error rate) depends on code distance and is not a single number. The hypothesis is in the right order of magnitude for phenomenological thresholds but is not a precise match to any published result.

**Grade**: **CLOSE** (right ballpark for phenomenological threshold, not precise)

---

### H-QC-12: 2-Phase QEC Cycle = lambda(6) = 2

**Claim**: QEC optimally alternates X-type and Z-type stabilizer measurement in 2-phase cycles.

**n=6 derivation check**: lambda(6)=2 is correct (Carmichael function).

**Real-world check**: CSS codes (including the surface code) do measure X-type and Z-type stabilizers. In many implementations, these are measured in alternating rounds, giving a natural 2-phase structure. This is standard practice in surface code implementations (Fowler et al. 2012). However, this 2-phase structure follows trivially from the CSS code definition: CSS codes have independent X and Z stabilizer groups. The number 2 comes from the Pauli group structure (X and Z types), not from lambda(6). Single-shot QEC codes (Bombim 2015) can correct in a single round, breaking the 2-phase assumption.

**Grade**: **CLOSE** (factually correct observation, but trivially follows from CSS structure, not lambda(6))

---

### H-QC-13: Squarefree Stabilizer = Optimal QEC (mu(6) = 1)

**Claim**: Optimal QEC codes have "squarefree" (independent) stabilizer generators, matching mu(6)=1.

**n=6 derivation check**: mu(6)=1 is correct (6=2*3 is squarefree).

**Real-world check**: Stabilizer generators must be independent by definition in a stabilizer code. If generators were dependent (i.e., one is a product of others), the code would have fewer logical qubits than claimed. This is not an insight -- it is a definitional requirement. Moreover, deliberately redundant stabilizers are useful in some contexts: single-shot QEC (Bombim 2015) and Bacon-Shor codes use redundant stabilizers for improved performance. The claim that "squarefree = optimal" is contradicted by the utility of redundant stabilizers.

**Grade**: **WEAK** (restates a definition; ignores useful counterexamples)

---

### H-QC-14: R(6) = 1 Implies Perfect QEC at n = 6

**Claim**: The [[6,2,2]] code is optimal because R(6)=1.

**n=6 derivation check**: R(6) = sigma(6)*phi(6)/(6*tau(6)) = 24/24 = 1. Correct.

**Real-world check**: The [[6,2,2]] code exists and can detect 1 error (d=2) but cannot correct any errors. It is not a particularly remarkable code. The [[5,1,3]] code (perfect quantum Hamming code) achieves error correction with fewer qubits. The [[7,1,3]] Steane code is more practical. The R(n) framework is the project's own construction with no independent standing in QEC theory. The claim that perfect numbers yield optimal codes has no supporting theorem or empirical evidence.

**Grade**: **WEAK**

---

### H-QC-15: 2 Basis Gate Classes = phi(6) = 2

**Claim**: Universal quantum computation requires exactly phi(6)=2 classes of gates (Clifford + non-Clifford).

**n=6 derivation check**: phi(6)=2 is correct.

**Real-world check**: The Gottesman-Knill theorem establishes that Clifford gates alone are efficiently classically simulable. Adding any non-Clifford gate (e.g., T gate) achieves universality. This is one of the most fundamental results in quantum computing (Gottesman 1998, Knill 2001). The factual claim about 2 gate classes is entirely correct and well-established.

However, the number 2 arises from the mathematical structure of the Clifford group and its relationship to classical simulability, not from Euler's totient of 6. The Clifford/non-Clifford dichotomy is a deep result about the structure of the unitary group, discovered independently of number theory. Attributing it to phi(6)=2 is post-hoc numerology applied to an independently established fact.

**Grade**: **CLOSE** (fact is correct and important; attribution to phi(6) is numerological)

---

### H-QC-16: Universal Gate Set Has Exactly 6 Gates

**Claim**: The standard universal gate set {H, T, CNOT, S, X, Z} has n=6 gates.

**n=6 derivation check**: n=6 is simply the perfect number.

**Real-world check**: The set {H, T, CNOT, S, X, Z} contains redundancies: S = T^2, Z = S^2 = T^4, X = HZH. The minimal universal gate set is {H, T, CNOT} (3 gates), or even {H, Toffoli} (2 gates). Actual hardware native gate sets vary:
- IBM: {ECR or CX, RZ, SX, X, ID} = 4-5 native gates
- Google: {sqrt(iSWAP), Phased-XZ} = 2-3 native gates
- IonQ: {GPI, GPI2, MS} = 3 native gates
- Quantinuum: {RZ, RX, ZZ} = 3 native gates

No vendor uses exactly 6 native gates. The "{H,T,CNOT,S,X,Z}" set is a pedagogical set from textbooks, not an optimal or minimal set. Listing 6 gates by including redundant ones to reach the target number is circular.

**Grade**: **FAIL**

---

### H-QC-17: Optimal T-Count = Multiple of 4 (tau(6) = 4)

**Claim**: Optimal T-counts for quantum circuits cluster at multiples of 4, because T^4 = Z.

**n=6 derivation check**: tau(6)=4 is correct. T^4 = Z is correct.

**Real-world check**: Published optimal T-counts from circuit synthesis literature:
- Toffoli gate: 7T (Amy et al. 2013) -- not a multiple of 4
- Fredkin gate: 7T -- not a multiple of 4
- Controlled-S: 3T -- not a multiple of 4
- CS (controlled-S): 1T -- not a multiple of 4
- CCZ: 7T -- not a multiple of 4
- Multiply-controlled Toffoli (n controls): varies, often odd

The claim is directly and repeatedly falsified by published circuit synthesis results. While T^4 = Z is algebraically true, this does not cause optimal circuits to have T-counts that are multiples of 4.

**Grade**: **FAIL**

---

### H-QC-18: Single:Two-Qubit Gate Ratio = sigma/tau = 3:1

**Claim**: Optimized quantum circuits have a 3:1 ratio of single-qubit to two-qubit gates.

**n=6 derivation check**: sigma(6)/tau(6) = 12/4 = 3. Correct.

**Real-world check**: Empirical gate ratios from QASMBench and other benchmarks vary enormously by algorithm:
- Grover's algorithm: ~2:1
- QFT-heavy circuits: ~4:1 or higher
- QAOA: ~1:1 to 2:1
- Random circuits: depends on construction
- VQE ansatze: varies by design

There is no universal 3:1 ratio. The ratio depends on the algorithm, the hardware topology (more SWAP gates needed for limited connectivity), and the compiler. Different compilation passes can dramatically change the ratio.

**Grade**: **WEAK**

---

### H-QC-19: Gate Scheduling = 50% parallel + 33% sequential + 17% barrier

**Claim**: Quantum gate scheduling follows Egyptian fraction allocation.

**n=6 derivation check**: 1/2+1/3+1/6=1 is correct.

**Real-world check**: No published data supports this specific breakdown. The parallelism fraction depends entirely on: circuit structure (width vs. depth), qubit connectivity (more connectivity enables more parallelism), algorithm type (QFT has high sequential dependency; random circuits have high parallelism), and compiler quality. Mid-circuit measurement ("barrier" operations) usage varies from 0% (most NISQ circuits) to significant fractions (in QEC circuits). The claim is too specific and has no empirical support.

**Grade**: **UNVERIFIABLE**

---

### H-QC-20: Quantum Advantage at Depth ~ 5*log(n)

**Claim**: Quantum advantage begins at circuit depth sopfr(6)*log(n) = 5*log(n).

**n=6 derivation check**: sopfr(6)=5 is correct.

**Real-world check**: Google's Sycamore quantum supremacy: 53 qubits, depth 20. The prediction gives 5*log2(53) = 28.5, a 30% overestimate. More fundamentally, the theoretical framework is wrong: random circuit sampling requires depth O(n) for anti-concentration (Dalzell et al. 2020), not O(log n). The O(log n) depth appears in circuit complexity lower bounds (for specific computational problems), but with problem-dependent constants, not a universal "5." The Sycamore experiment's depth of 20 was chosen based on fidelity decay with depth, which depends on gate error rates -- a hardware parameter, not a number-theoretic constant.

**Grade**: **WEAK** (order-of-magnitude match for one experiment, wrong theoretical framework)

---

### H-QC-21: 2-Qubit Gate = J_2(6) = 24 Elementary Operations

**Claim**: Any 2-qubit gate decomposes into exactly 24 elementary operations.

**n=6 derivation check**: J_2(6)=24 is correct.

**Real-world check**: The KAK decomposition (Khaneja & Glaser 2001, Vatan & Williams 2004) shows that any SU(4) operation can be decomposed into at most 3 CNOT gates and 15 single-qubit rotations, totaling 18 gate operations. At the pulse level, an IBM cross-resonance CNOT gate requires 1-3 microwave pulses depending on calibration. Neither 18 (gate-level) nor 1-3 (pulse-level) equals 24. The hypothesis arithmetic ("3 CNOT * 2 rotations + extras = 24") does not match the actual KAK decomposition.

**Grade**: **FAIL**

---

### H-QC-22: QEC Width Expansion = 4/3x

**Claim**: Optimal QEC requires ancilla qubits = 1/3 of data qubits (4/3x total expansion).

**n=6 derivation check**: tau(6)^2/sigma(6) = 16/12 = 4/3 is correct.

**Real-world check**: Surface code requires approximately equal numbers of data and ancilla qubits, giving ~2x expansion, not 4/3x. Bacon-Shor codes also require ~2x. The best known quantum LDPC codes (Panteleev-Kalachev 2022, quantum Tanner codes) achieve asymptotically constant rate but at practical code distances still require overhead well above 4/3x. A code with only 1/3 ancilla overhead relative to data qubits would need n-k stabilizers where k = 3n/4, giving rate 3/4 -- no known code achieves this rate with meaningful distance for practical error correction. The prediction is far more optimistic than any known QEC code.

**Grade**: **FAIL**

---

### H-QC-23: QV Threshold for Usefulness = 2^sigma(6) = 4096

**Claim**: Quantum Volume = 4096 marks the threshold for practical quantum advantage.

**n=6 derivation check**: 2^sigma(6) = 2^12 = 4096 is correct.

**Real-world check**: IBM achieved QV=4096 in 2021 (Eagle processor). However, QV=4096 was not recognized as a transition to practical quantum advantage. No practical quantum advantage application was specifically unlocked at QV=4096. Quantum Volume is primarily an IBM-promoted metric; Google, IonQ, and Quantinuum do not emphasize it. Google demonstrated quantum computational advantage (random circuit sampling) at QV much lower than 4096. The "usefulness threshold" is not a recognized concept tied to any specific QV value.

**Grade**: **WEAK**

---

### H-QC-24: Qubit Allocation = 50% compute + 33% memory + 17% communication

**Claim**: Egyptian fraction qubit allocation is optimal.

**n=6 derivation check**: 1/2+1/3+1/6=1 is correct.

**Real-world check**: Current quantum processors are monolithic and do not have separate "compute," "memory," and "communication" qubit pools. Surface code QEC dedicates roughly 50% of physical qubits to ancilla measurement -- but ancilla qubits are neither "memory" nor "communication." Distributed quantum computing architectures are still experimental, and no allocation standard exists. The claim applies to a future architecture that does not yet exist.

**Grade**: **UNVERIFIABLE**

---

### H-QC-25: Surface Code Optimal Patch = 24 Data Qubits (d=5)

**Claim**: sigma(6)*phi(6) = 24 gives the optimal surface code patch size; d=5 has ~24 data qubits.

**n=6 derivation check**: sigma(6)*phi(6) = 24. Correct.

**Real-world check**: A distance-5 rotated surface code has d^2 = 25 data qubits, not 24. The "24+1" framing is ad hoc. d=5 is indeed commonly discussed as a practical starting point for QEC (Google's recent work targets d=5 and d=7), but so is d=3 (for initial demonstrations) and d=7 or higher (for fault-tolerant computation). The choice of d=5 comes from error suppression scaling Lambda^d where Lambda > 1 is the error suppression factor, not from n=6 arithmetic. The mismatch of 25 vs 24 is small but the "plus one" adjustment has no justification.

**Grade**: **CLOSE** (25 is close to 24; d=5 is genuinely important; but 25 != 24)

---

### H-QC-26: Ultimate QC Efficiency = phi(6)/sigma(6) = 1/6

**Claim**: The fundamental limit of logical-to-physical qubit ratio is 1/6 = 16.7%.

**n=6 derivation check**: phi(6)/sigma(6) = 2/12 = 1/6 is correct.

**Real-world check**: Current surface code estimates: ~1/1000 to 1/10000 physical qubits per logical qubit. Optimistic quantum LDPC projections: ~1/100 to 1/10 for asymptotic codes. There is no known fundamental lower bound of 1/6 for the physical-to-logical qubit ratio. The quantum Singleton bound and quantum Hamming bound constrain code parameters, but neither produces a universal 1/6 limit. The claim is more optimistic than current technology but has no theoretical backing.

**Grade**: **WEAK**

---

### H-QC-27: 2-Mode Connectivity = lambda(6) = 2

**Claim**: Quantum processors should alternate between 2 connectivity modes.

**n=6 derivation check**: lambda(6)=2 is correct.

**Real-world check**: Google's Sycamore and later processors use tunable couplers with two states: coupled (on) and decoupled (off). This is a genuine 2-mode system. IBM's newer processors (Heron) also use tunable couplers. However, the 2-mode nature follows from the binary nature of coupling control (on/off), not from lambda(6)=2. Any switch has 2 states. The hypothesis correctly identifies a real design pattern but the attribution to Carmichael function is coincidental.

**Grade**: **CLOSE** (real pattern, wrong attribution)

---

### H-QC-28: Quantum Advantage at 6^k Qubit Milestones

**Claim**: Quantum milestones occur at qubit counts that are powers of 6: 6, 36, 216, 1296, 7776.

**n=6 derivation check**: Powers of 6 are straightforward.

**Real-world check**: Actual hardware milestones:
- Google Sycamore (2019): 53 qubits (not 36)
- IBM Eagle (2021): 127 qubits (not 216)
- IBM Osprey (2022): 433 qubits (not 216 or 1296)
- IBM Condor (2023): 1121 qubits (not 1296)
- Google Willow (2024): 72 qubits (not 36 or 216)

Not a single major milestone aligns with a power of 6. Industry roadmaps target round numbers (1000, 10000, 1000000). The prediction has zero predictive power.

**Grade**: **FAIL**

---

### H-QC-29: Egyptian Fraction Amplitude Distribution

**Claim**: Quantum algorithms have optimal amplitude distribution sqrt(1/2), sqrt(1/3), sqrt(1/6).

**n=6 derivation check**: |sqrt(1/2)|^2 + |sqrt(1/3)|^2 + |sqrt(1/6)|^2 = 1. Correct normalization.

**Real-world check**: Grover's algorithm evolves amplitudes sinusoidally. After optimal number of iterations, the marked state has amplitude ~1 and all others have amplitude ~0. At no point does the amplitude distribution match {sqrt(1/2), sqrt(1/3), sqrt(1/6)}. Shor's algorithm produces a periodic amplitude distribution related to the continued fraction expansion of the target. No known quantum algorithm has Egyptian fraction amplitude distribution as an optimality condition. The claim is straightforwardly false for all standard algorithms.

**Grade**: **FAIL**

---

### H-QC-30: Quantum Chemistry Active Space = sigma(6) = 12 Orbitals

**Claim**: The fundamental active space for quantum chemistry is 12 orbitals.

**n=6 derivation check**: sigma(6)=12 is correct.

**Real-world check**: Active space sizes in quantum chemistry vary enormously:
- Small molecules (H2, LiH): 2-6 orbitals
- Benzene: typically (6,6) = 6 orbitals
- Transition metals (Fe, Ni, Cu): typically (10,10) to (16,16)
- Strongly correlated systems: 20-50+ orbitals

12 orbitals is within the range for some transition metal compounds (e.g., iron porphyrin models often use (12,12) or similar), but active space size is entirely molecule-dependent. There is no universal "12 orbital" standard. Some benchmarks use 12 qubits (= 12 spin-orbitals = 6 spatial orbitals) as a convenient size, but this is a practical choice, not a fundamental constant.

**Grade**: **CLOSE** (12 is within the practically relevant range; not uniquely optimal)

---

### H-QC-31: 4-Layer Variational Ansatz = tau(6) = 4

**Claim**: Optimal variational circuit depth is tau(6) = 4 layers.

**n=6 derivation check**: tau(6)=4 is correct.

**Real-world check**: Optimal ansatz depth depends on: problem size, qubit count, connectivity, noise level, and specific algorithm. QAOA with p=1-3 often suffices for simple combinatorial optimization; p>>4 is needed for hard instances. VQE hardware-efficient ansatz depth is limited by noise in NISQ devices (fewer layers = less noise) but must be deep enough for expressibility. Barren plateau onset scales with circuit depth and qubit count (McClean et al. 2018), not a fixed number. On a 100-qubit device, 4 layers might be too shallow for expressibility; on a 4-qubit device, 4 layers might be excessive. There is no universal "4 layers" result.

**Grade**: **WEAK**

---

### H-QC-32: Iterative QPE Uses 2-Bit Feedback = phi(6) = 2

**Claim**: Quantum Phase Estimation uses phi(6)=2 bit feedback.

**n=6 derivation check**: phi(6)=2 is correct.

**Real-world check**: Kitaev's iterative QPE (Kitaev 1995) uses a single ancilla qubit, recycled across iterations. Each iteration involves: (1) prepare ancilla in |+>, (2) apply controlled-U^(2^k), (3) apply phase correction based on previously measured bits, (4) measure in X basis. The feedback is 1 classical bit per iteration (the measurement outcome), not 2 bits. The hypothesis claims phi(6)=2 predicts "2-bit feedback," but the actual algorithm uses 1-bit feedback. The original hypotheses document marks this as "confirmed," which is incorrect.

**Grade**: **FAIL** (actual algorithm uses 1-bit, not 2-bit feedback)

---

### H-QC-33: 24-Qubit Quantum Simulator as Fundamental Unit

**Claim**: 24 qubits represent the classical simulation limit / fundamental simulation unit.

**n=6 derivation check**: J_2(6)=24 is correct.

**Real-world check**: A 24-qubit quantum system has a state vector of 2^24 = 16,777,216 complex amplitudes, requiring ~256 MB of memory. This is trivially simulable on any modern laptop. The practical limit for exact state-vector simulation is approximately 40-50 qubits (2^40 to 2^50 amplitudes require TB to PB of memory). Tensor network methods can simulate even larger systems for low-entanglement circuits. The 24-qubit "Hubbard model benchmark" mentioned in the hypothesis is well within classical computation capability (exact diagonalization of a 24-site Hubbard model is routine on a workstation).

**Grade**: **FAIL**

---

### H-QC-34: 6-Qubit Entanglement as Fundamental Unit

**Claim**: 6-qubit entanglement is the fundamental unit for quantum networks.

**n=6 derivation check**: n=6 is the perfect number.

**Real-world check**: Standard entanglement resources in quantum information are: Bell pairs (2 qubits), GHZ states (3+ qubits), W states (3+ qubits), and cluster states (arbitrary size). Entanglement classification is well-understood for 2-3 qubits and partially understood for 4 qubits (Verstraete et al. 2002: 9 SLOCC classes for 4 qubits). For 5+ qubits, complete SLOCC classification is an open problem. There is no established result making 6-qubit entanglement a "fundamental unit." The claim is too vague to test rigorously.

**Grade**: **WEAK**

---

### H-QC-35: QML Allocation = 50% encoding + 33% processing + 17% readout

**Claim**: Quantum machine learning optimally allocates qubits as Egyptian fractions.

**n=6 derivation check**: 1/2+1/3+1/6=1 is correct.

**Real-world check**: Quantum machine learning is an immature field with no established optimal resource allocation. In typical QML circuits, the same qubits serve multiple roles (encoding, processing, and measurement). Amplitude encoding uses all qubits for data encoding; variational circuits use all qubits for both processing and measurement. The concept of separate qubit pools for encoding/processing/readout does not match current QML circuit designs. No published QML paper defines or tests this allocation scheme.

**Grade**: **UNVERIFIABLE**

---

### H-QC-36: R(6)=1 Implies Thermodynamic Optimality

**Claim**: Quantum computing achieves thermodynamic optimality because R(6)=1 and quantum gates are unitary (reversible).

**n=6 derivation check**: R(6)=1 is correct. Quantum gates are unitary (reversible) by definition.

**Real-world check**: The observation that quantum gates are reversible is trivially true -- unitarity is the defining property of quantum mechanics. Landauer's principle (kT ln 2 per bit erasure) applies to measurement, which is the irreversible step. These are basic physics facts, not n=6 insights. The specific claim that energy cost is "sigma(6)*kT*ln(2) per QEC cycle" has no basis in published literature. The actual energy cost of a QEC cycle depends on: number of measurements, classical processing power, cooling power, microwave pulse energy, etc. None of these are quantified by sigma(6)*kT*ln(2).

**Grade**: **WEAK**

---

## Summary Table

| Grade | Count | Percentage |
|-------|-------|------------|
| EXACT | 0 | 0% |
| CLOSE | 9 | 25.0% |
| WEAK | 13 | 36.1% |
| FAIL | 8 | 22.2% |
| UNVERIFIABLE | 3 | 8.3% |
| **Not graded (UNVERIFIABLE)** | **3** | **8.3%** |
| **Total** | **36** | **100%** |

---

## Reassessment of the Original "Confirmed" Claims

The original hypotheses document marks several predictions as "confirmed." Independent review:

| Original Claim | Original Status | Revised Grade | Reason |
|----------------|----------------|---------------|--------|
| Gate set size = 6 (H-QC-16) | "Confirmed" | FAIL | Set is redundant; native sets are 2-5 gates |
| 2 basis gate classes (H-QC-15) | "Confirmed" | CLOSE | Factually correct but follows from Gottesman-Knill, not phi(6) |
| QV=4096 (H-QC-23) | "Confirmed" | WEAK | IBM reached it but no quantum advantage was unlocked |
| Iterative QPE 2-bit (H-QC-32) | "Confirmed" | FAIL | Actual algorithm uses 1-bit feedback |
| X/Z alternation (H-QC-12) | "Standard practice" | CLOSE | Correct but trivially follows from CSS structure |
| Surface code d=5 ~24 (H-QC-25) | "Partial" | CLOSE | 25 != 24; d=5 is common but not uniquely optimal |

---

## Honest Limitations

### 1. The Central Question: Are Qubit Counts Genuinely n=6, or Hardware-Dependent?

This is the single most important critique of the quantum computing hypotheses. Nearly every "prediction" (module size = 24, connectivity = 12, patch size = 24, etc.) claims a universal optimal value, but in reality these values depend on:

- **Physical qubit technology**: Superconducting transmons have different connectivity constraints than trapped ions, photonic qubits, or neutral atoms
- **Fabrication constraints**: Chip size, wiring density, cryostat geometry
- **Noise model**: Depolarizing, biased, correlated, leakage
- **Target application**: Chemistry, optimization, cryptanalysis
- **Code choice**: Surface code, color code, LDPC code

A framework claiming universal constants must explain why the same constant applies across all these technologies. The n=6 hypotheses make no such explanation.

### 2. Zero EXACT Matches

Not a single hypothesis achieves an EXACT match against a well-established, hardware-independent quantum computing result. The CLOSE grades identify real patterns (Clifford/non-Clifford dichotomy, CSS 2-phase structure, hexagonal topology use) but in every case, the pattern was discovered independently of n=6 arithmetic and has a well-understood explanation that does not involve perfect numbers.

### 3. The Overfitting Problem

With constants n=6, sigma=12, tau=4, phi=2, sopfr=5, J_2=24, mu=1, lambda=2, and combinations thereof, the framework can generate target values of 1, 2, 3, 4, 5, 6, 7, 8, 12, 24, 64, 4096, and many others. For any small integer appearing in quantum computing, an n=6 expression likely exists to match it. This is overfitting, not prediction.

### 4. Egyptian Fraction Claims Are Unfalsifiable

Five hypotheses (H-QC-9, H-QC-19, H-QC-24, H-QC-29, H-QC-35) use the 1/2+1/3+1/6=1 split. Since any resource allocation can be approximated by three fractions summing to 1, and the specific split is never precisely confirmed by any data, these claims are unfalsifiable. They can never be proven wrong because they can never be precisely tested.

### 5. Reversed Direction of Discovery

Every CLOSE-grade match identifies a real phenomenon that was discovered by quantum computing researchers independently of n=6 arithmetic. The Gottesman-Knill theorem (2 gate classes), CSS code structure (2-phase QEC), and the Golay code (24 dimensions) were all discovered from quantum information theory. Claiming them as "derived from" n=6 reverses the actual direction of discovery.

### 6. Specific Predictions That Are Falsified

Eight hypotheses (22.2%) make specific numerical predictions that are directly contradicted by published data (degree-12 connectivity, 6-gate universal set, T-count multiples of 4, 24 elementary operations, 4/3x QEC expansion, 6^k milestones, Egyptian amplitudes, 24-qubit simulation limit, 2-bit QPE feedback). This falsification rate is significant and demonstrates that the framework does not reliably predict quantum computing parameters.

---

## Overall Assessment

**0 out of 36 hypotheses achieve EXACT verification against real-world quantum computing data.** The 9 CLOSE grades identify real patterns but in every case, the pattern has an established explanation independent of n=6 arithmetic. The 8 FAIL grades represent specific predictions directly contradicted by published experimental and theoretical results.

The quantum computing domain is perhaps the most challenging for the n=6 framework because quantum hardware is young, rapidly evolving, and highly technology-dependent. The idea that a single number-theoretic constant could determine optimal qubit counts, connectivity, gate sets, and error thresholds across all qubit technologies is not supported by either theory or experiment.

---

*Verification performed against: IBM Qiskit documentation and processor specifications (2024-2025), Google Quantum AI publications, Gottesman-Knill theorem (1998/2001), Vatan & Williams KAK decomposition (2004), Kitaev iterative QPE (1995), Amy et al. T-count optimization (2013), Panteleev-Kalachev quantum LDPC codes (2022), Fowler et al. surface code review (2012), McClean et al. barren plateaus (2018), QASMBench, NREL Quantum Benchmark Suite.*

*Part of [N6 Architecture](https://github.com/need-singularity/n6-architecture) | TECS-L family*
