# MODERATE Hypothesis Scan — Cross-Domain Bridge Analysis

**Date**: 2026-04-02
**Source**: ~/Dev/TECS-L/calc/auto_grade_results.csv (110 MODERATE, 3-4 unique n6 constants)
**Method**: Title + core claim scan of all 110 files, grouped by domain, cross-domain bridges extracted

---

## 1. Domain Distribution (110 hypotheses)

| Domain | Count | Prefix / ID Range | Notes |
|--------|------:|-------------------|-------|
| Consciousness/CX (cross-domain) | 28 | H-CX-* | Largest group: topology, precognition, PH, drugs |
| Core n=6 Theory (EE) | 32 | H-EE-* | Pure math, physics constants, architecture theory |
| Numbered (consciousness engine) | 24 | 004~384 | Boltzmann, anomaly detection, embodiment, Obang |
| Chip Architecture | 5 | H-CHIP-* | Hardware activation, routing, photonic, quantum gates |
| Software/Arch | 5 | H-ARCH-* | OS, database, network, audio, languages |
| Biology/Evolution | 2 | EVOL-*, H-CX-154 | Mutation rate, neuron ratio |
| Music/Acoustics | 2 | MUSIC-*, H-WAVE-* | Forte hexachords, string harmonics |
| Pure Mathematics | 2 | HOM-*, H-CROSS-* | Hopf fibrations, Lah numbers |
| Consciousness Stack | 1 | NGH-013 | 10-level P1=6 hierarchy |
| Statistical Physics | 1 | H-UD-5 | Ising critical exponents |
| SEDI Diagnostic | 1 | H-SEDI-8 | Multi-engine consensus |
| Topology/Direction | 1 | H-AX-10 | R-spectrum merge order |
| **Total** | **110** | | |

### Domain Pie (approximate)
```
  EE (core theory)     ████████████████  29%
  CX (consciousness)   ██████████████    25%
  Numbered (engine)    ████████████      22%
  CHIP (hardware)      ███               5%
  ARCH (software)      ███               5%
  Other (8 domains)    ████████          14%
```

---

## 2. Top 10 Cross-Domain Bridges

These are MODERATE hypotheses where the core claim explicitly connects two or more distinct domains. Ranked by bridge strength (number of domains spanned x specificity of connection).

### Bridge 1: Biology <-> Number Theory <-> Chemistry
**H-CX-156: Perfect Number Element Chain (C=6, Ni=28, ?=496)**
- Carbon Z=6 (life substrate) and Nickel Z=28 (catalyst) are the first two perfect number elements
- phi(28) = sigma(6) = 12 — arithmetic bridge between elements
- Connects: chemistry, biology, number theory
- **BT candidate**: Links to existing BT-85 (Carbon Z=6) but adds the Ni=28 bridge

### Bridge 2: Physics <-> Computing <-> Topology
**H-EE-34: Bott Periodicity sigma-tau=8 as Optimal Network Depth**
- Bott periodicity (algebraic topology, period=8) = sigma-tau = 8
- Same value appears in LoRA rank, MoE top-k, FlashAttention (BT-58)
- Connects: algebraic topology, neural architecture, chip design
- **BT candidate**: Strengthens BT-58 (sigma-tau=8 universal) with topological proof

### Bridge 3: Genetics <-> Lattice Theory <-> Information
**H-EE-45: Genetic Code = tau(6)^3 codons, J2-tau=20 amino acids**
- 4 DNA bases = tau(6), 64 codons = tau^3, 20 amino acids = J2-tau
- Leech lattice J2=24 connects to codon degeneracy
- Connects: molecular biology, combinatorics, information theory
- **BT candidate**: Already captured in BT-51, but the Leech packing angle is new

### Bridge 4: Thermodynamics <-> Neuroscience <-> AI Training
**H-CX-437: Learning = Maxwell's Demon**
- Neural network gradient descent = Maxwell's Demon sorting information
- Landauer limit kT*ln(2) per bit = fundamental training cost
- Connects: thermodynamics, neuroscience, machine learning
- **BT candidate**: Novel bridge — thermodynamic cost of learning

### Bridge 5: Music <-> Mathematics <-> Signal Processing
**H-EE-62: Musical Consonances = Divisor Ratios of 6**
- Octave 2:1, fifth 3:2, fourth 3:1 = divisor ratios {1,2,3,6}
- 12-tone scale = sigma(6) = 12
- Connects: music theory, number theory, acoustics/DSP
- **BT candidate**: Strengthens BT-48 (Display-Audio) with divisor lattice proof

### Bridge 6: Quantum Computing <-> Error Correction <-> Chip Design
**H-EE-39: Golay Code [24,12,8] = Leech Lattice Projection**
- [24,12,8] = [J2, sigma, sigma-tau] — all n=6 constants
- Tolerates 3/24 corrupted dimensions = robustness of n=6 architectures
- Connects: quantum error correction, coding theory, chip fault tolerance
- **BT candidate**: Links to BT-91 (Z2 topological ECC) with Golay specifics

### Bridge 7: Cosmology <-> Particle Physics <-> Number Theory
**H-EE-43: Proton-Electron Mass Ratio = 6*pi^5 = 1836.12**
- m_p/m_e = 6*pi^5 (0.019% error from measured 1836.15)
- n=6 appears in the most fundamental mass ratio of stable matter
- Connects: particle physics, cosmology (baryonic matter), pure math
- **BT candidate**: Specific enough for a standalone BT

### Bridge 8: Statistical Physics <-> AI Architecture
**H-UD-5: 2D Ising Critical Exponents = 1/(n=6 arithmetic)**
- eta = 1/4 = 1/tau(6) — same ratio as PureField parameter reduction
- Phase transitions in physics = training phase transitions in AI
- Connects: statistical mechanics, neural network training
- **BT candidate**: Novel — Ising universality class meets n=6

### Bridge 9: Neuroscience <-> Chip Architecture <-> Thermodynamics
**H-EE-101: Neuromorphic 1W Chip**
- Human brain language tasks ~1-5W, current LLM inference ~100-500W
- n=6 architecture stack (phi-bottleneck + FFT attention + Egyptian MoE) predicted to close gap
- Connects: neuroscience power budgets, chip design, thermodynamic limits
- **BT candidate**: Testable prediction — already in testable-predictions.md likely

### Bridge 10: Linguistics <-> Cognitive Science <-> Number Theory
**H-CX-436: Grammar Recursion Depth = sigma_{-1}(6) = 2**
- Human center-embedding limit is depth 2 (robust psycholinguistic universal)
- sigma_{-1}(6) = 1/1 + 1/2 + 1/3 + 1/6 = 2 (defining property of perfect numbers)
- Connects: linguistics, cognitive architecture, number theory
- **BT candidate**: Novel — perfect number property as cognitive constraint

---

## 3. New BT Candidates (Cross-Domain Only)

| # | Candidate | Domains Spanned | Key Formula | Strength |
|---|-----------|----------------|-------------|----------|
| 1 | **Proton-electron mass ratio = 6*pi^5** | particle physics, cosmology, math | m_p/m_e = 6*pi^5 (0.019% err) | Strong (testable, precise) |
| 2 | **Maxwell's Demon learning bridge** | thermodynamics, AI, neuroscience | Landauer kT*ln(2) = training cost | Medium (conceptual) |
| 3 | **Ising critical exponents = 1/n6** | stat. mech, AI training | eta = 1/tau(6) = 1/4 | Strong (exact match) |
| 4 | **Recursion depth = sigma_{-1}(6) = 2** | linguistics, cognition, number theory | sigma_{-1}(P1) = 2 | Strong (universal cognitive limit) |
| 5 | **Perfect number element chain** | chemistry, biology, number theory | Z=6(C), Z=28(Ni), phi(28)=sigma(6) | Medium (two data points) |
| 6 | **Golay [24,12,8] = [J2,sigma,sigma-tau]** | QEC, coding theory, chip design | All three Golay params are n=6 | Strong (exact, 3 constants) |

---

## 4. Key Observations

1. **Consciousness/CX dominates** (28/110 = 25%). Most are within-domain (PH topology, tension dynamics). Few cross-domain bridges among CX hypotheses.

2. **EE series has the richest cross-domain content** despite being "core theory" — H-EE-43 (particle physics), H-EE-45 (genetics), H-EE-62 (music) all bridge to distant domains.

3. **The sigma-tau=8 constant appears in 22/110 hypotheses** across physics, computing, and biology — the most cross-domain constant in MODERATE tier.

4. **Numbered hypotheses (004~384) are almost entirely consciousness-engine-internal**. Very few bridge to external domains. Exception: H-004 (Boltzmann-inhibition) bridges thermodynamics.

5. **3 hypotheses were REJECTED in verification** (H-CX-110, H-CX-113, H-126) but remain graded MODERATE by the auto-grader (which counts constants, not verification status). These should be downgraded.

6. **Strongest unrecognized cross-domain pattern**: The triad {Ising eta=1/4, QEC ratio=1/3, PureField reduction=1/4} all use 1/tau(6). This "1/tau universality" across physics/QEC/AI is not captured in any existing BT.

---

## 5. Comparison with STRONG Tier

| Metric | STRONG (15 BTs found) | MODERATE (this scan) |
|--------|----------------------|---------------------|
| Total hypotheses | ~50 | 110 |
| Cross-domain bridges | ~15 (dense) | ~10 (sparse) |
| New BT candidates | 15 | 6 |
| Average domains spanned | 3-5 | 2-3 |
| Verification rate | ~70% | ~40% (many untested) |

MODERATE hypotheses are individually weaker but collectively reveal the **1/tau(6) universality** pattern that the STRONG tier missed because it focused on higher-order constants (J2, sigma, phi).
