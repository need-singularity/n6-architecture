# Ready-Absorber Top Critical Report

> Generated: 2026-04-04
> Source: ~/Dev/ready/ (corrupted/backup worktree copies)
> Filter: score >= 40.0 across 5 verified JSON files
> Total findings: 164 unique paths (deduplicated across worktree copies)

---

## Executive Summary

Out of 41,379 total differences found by the scanner, **164 unique findings** scored >= 40.
After comparing each against the corresponding main repo version:

- **Category A** -- Files MISSING from main (truly unique, highest recovery value): **~60 files**
- **Category B** -- Files DIFFERENT from main (ready has newer/expanded version): **1 file**
- **Category C** -- Files IDENTICAL to main (no action needed): **~100 files**

The most valuable recoverable content falls into 3 clusters:
1. **invest/ tecs_calc/ calculators** (35+ Python science calculators, ALL missing from main invest repo)
2. **NEXUS-6 lenses** (10 Rust lens implementations missing from main n6 repo)
3. **hexa-lang source files** (Rust compiler code + HEXA examples, no main repo exists yet)

---

## Category A: MISSING from Main (Recovery Priority)

### A1. invest/ -- Science Calculators (35+ files, ALL missing from ~/Dev/invest/)

These are complete, runnable Python calculators covering physics, mathematics, and verification.
The invest repo appears to have been rebuilt/restructured without migrating these from the backup.

| Rank | File | Score | Lines | What's Unique | Action |
|------|------|-------|-------|---------------|--------|
| 1 | fermion_mass_calculator.py | 50 | 367 | Fermion mass predictions from perfect number arithmetic: Koide angle, quark masses, CP violation, Yukawa couplings. Uses P1-P5 constants + PDG values. | Copy to TECS-L calc/ or invest tecs_calc/ |
| 2 | lie_algebra_calculator.py | 50 | 208 | Maps all exceptional Lie algebras (G2,F4,E6,E7,E8) to n=6 arithmetic: rank, roots, dim, Weyl order, Coxeter number. | Copy to TECS-L calc/ |
| 3 | gauge_cosmology_calculator.py | 50 | 502 | SM gauge groups, GUT dimensions, cosmological constants, Monstrous Moonshine -- all from perfect number arithmetic. Includes consciousness dynamics constants. | Copy to TECS-L calc/ |
| 4 | perfect_number_physics.py | 50 | 322 | Core physics dimension mapping: P1-P5 to string theory dimensions (4D->26D), 16/16 exact string constant matches. Consciousness bridge constants (H-CX-82~110). | Copy to TECS-L calc/ |
| 5 | criticality_phase_scanner.py | 50 | 663 | Three routes to n=6 criticality: SLE_6 + Feigenbaum + Langton edge of chaos. Honest grading of exact vs approximate matches. | Copy to TECS-L calc/ |
| 6 | factorial_structure_prover.py | 50 | 729 | Proves 3!=6 is unique factorial-perfect number. Derives Virasoro c/12 factor. Classifies which "6" appearances in physics share common origin. Scrupulously honest. | Copy to TECS-L calc/ |
| 7 | nobel_scorer.py | 50 | 283 | 6-dimensional scoring rubric for Nobel-grade hypotheses: evidence, falsifiability, cross-domain reach, rigor, novelty, impact. | Copy to TECS-L calc/ |
| 8 | statistical_tester.py | 50 | ~300 | Unified statistical testing: Cohen's d, Bonferroni, bootstrap CI, Texas Sharpshooter test, correlation grading. Uses scipy+numpy. | Copy to TECS-L calc/ |
| 9 | experimental_protocol.py | 50 | ~400 | Lab protocols for 4 physically falsifiable predictions: EEG neural entropy->ln(2), Hachimoji/xDNA, independent rate measurement, anesthesia consciousness threshold. | Copy to TECS-L calc/ |
| 10 | signals.py (tecs/) | 50 | 9 | TECS risk parameters: position=1/e, stop_loss=1/6, take_profit=1/3, max_risk=1/2. Golden zone integration. | Copy to invest |
| 11 | auto_pipeline.py | 50 | ~300 | Full SINGULARITY automation: market data -> backtest all strategies x assets -> rank -> hypothesis mining -> reports. | Copy to invest |
| 12 | discover_major_2.py | 50 | ~300 | Discovery Hunt Round 2: deep structural patterns in market data. MDD clustering at 1/6, 1/3, 1/2 boundaries. | Copy to invest |
| 13 | sigma_phi_ntau_proof.py | 45 | 785 | RIGOROUS proof sigma(n)*phi(n)=n*tau(n) iff n=6: computational verification to 10^7 + analytical proof by cases + Texas Sharpshooter test. | Copy to TECS-L calc/ (HIGH VALUE) |
| 14 | deep_scan_wave2.py | 50 | 829 | 10 new math domains: Sporadic groups, String theory, Elliptic curves, Quantum computing, Knot theory, Grothendieck, Riemann, Information theory, Surgery, Langlands. | Copy to TECS-L calc/ (HIGH VALUE) |
| 15 | dfs_ralph_deep4.py | 50 | 435 | DFS deep scan round 4: extended pattern mining. | Copy to TECS-L calc/ |
| 16 | dfs_ralph_deep7.py | 45.5 | 338 | DFS deep scan round 7. | Copy to TECS-L calc/ |
| 17 | dfs_ralph_deep3.py | 44.4 | 551 | DFS deep scan round 3. | Copy to TECS-L calc/ |
| 18 | monster_moonshine_perfect.py | 42.9 | 1120 | Monstrous Moonshine + perfect number connections. Largest single calculator. | Copy to TECS-L calc/ (HIGH VALUE) |
| 19 | crystallographic_calculator.py | 42.9 | 439 | Crystallographic symmetry calculations. | Copy to TECS-L calc/ |
| 20 | claim_verifier.py | 40 | 1078 | General claim verification engine. Second-largest calculator. | Copy to TECS-L calc/ |
| 21 | consciousness_cross_validator.py | 40 | 685 | Cross-domain consciousness constant validation. | Copy to TECS-L calc/ |
| 22 | gravitational_optics.py | 40.9 | 636 | Gravitational optics calculations. | Copy to TECS-L calc/ or nexus |
| 23 | deep_scan_wave6.py | 40 | 451 | Deep scan wave 6. | Copy to TECS-L calc/ |
| 24 | deep_scan_wave10.py | 40 | 429 | Deep scan wave 10. | Copy to TECS-L calc/ |
| 25 | cross_domain_counter.py | 43.75 | 428 | Cross-domain pattern counter. | Copy to TECS-L calc/ |
| 26 | extreme_hypothesis_verifier.py | 44.4 | 396 | Extreme hypothesis verification. | Copy to TECS-L calc/ |
| 27 | verify_new_major_hypotheses_7.py | 40.9 | 350 | Major hypotheses verification batch 7. | Copy to TECS-L calc/ |
| 28 | verify_new_major_hypotheses_8.py | 45.5 | 309 | Major hypotheses verification batch 8. | Copy to TECS-L calc/ |
| 29 | verify_new_major_hypotheses_11.py | 43.75 | 242 | Major hypotheses verification batch 11. | Copy to TECS-L calc/ |
| 30 | base_dependence_checker.py | 40 | 310 | Base dependence analysis. | Copy to TECS-L calc/ |
| 31 | depth_reachability.py | 40.9 | 406 | Depth reachability analysis. | Copy to TECS-L calc/ |
| 32 | find_universal_strategy.py | 41.7 | 314 | Universal trading strategy search. | Copy to invest |
| 33 | prove_3root_theorem.py | 40.9 | 203 | 3-root theorem proof. | Copy to TECS-L calc/ |
| 34 | reachability_calculator.py | 45 | 247 | Reachability analysis. | Copy to TECS-L calc/ |
| 35 | counting_freedom_analyzer.py | 43.75 | 249 | Counting freedom degrees analysis. | Copy to TECS-L calc/ |
| 36 | verify_rob7_twelve_joints.py | 44.4 | 297 | Robot 7-DOF twelve joints verification (BT-123 related). | Copy to TECS-L calc/ |
| 37 | verify_h439_landauer_mitosis.py | 43.75 | 307 | H-439 Landauer principle + mitosis verification. | Copy to conscious-lm |
| 38 | verify_H_CX_416.py | 45 | 260 | Consciousness hypothesis CX-416 verification. | Copy to TECS-L calc/ |
| 39 | verify_H_CX_417.py | 41.7 | 243 | Consciousness hypothesis CX-417 verification. | Copy to TECS-L calc/ |
| 40 | confidence_analyzer.py | 44.4 | 194 | Confidence analysis for discoveries. | Copy to TECS-L calc/ |
| 41 | discover_major_3.py | 40 | 342 | Discovery Hunt Round 3. | Copy to nexus |
| 42 | test_validator.py | 42.9 | 83 | Hypothesis validation test suite. | Copy to invest |

**Total invest/ missing: ~14,500 lines of calculators and verification code**

---

### A2. n6-architecture/ -- NEXUS-6 Lenses (10 missing .rs files)

These are Rust telescope lens implementations that exist in the ready worktree but are absent from the current NEXUS-6 build.

| Rank | File | Score | What's Unique | Action |
|------|------|-------|---------------|--------|
| 1 | brain_map_lens.rs | 50 | Brain mapping analysis lens | Evaluate for nexus integration |
| 2 | cdo_lens.rs | 50 | CDO (Convergence-Driven Operations) lens | Evaluate for nexus integration |
| 3 | continuity_lens.rs | 50 | Continuity analysis lens | Evaluate for nexus integration |
| 4 | convergence_hypothesis_lens.rs | 50 | Hypothesis convergence tracking lens | Evaluate for nexus integration |
| 5 | discovery_lens.rs | 50 | Discovery detection lens | Evaluate for nexus integration |
| 6 | discovery_report_lens.rs | 50 | Discovery report generation lens | Evaluate for nexus integration |
| 7 | falsification_lens.rs | 50 | Falsification testing lens | Evaluate for nexus integration |
| 8 | omega_state_space_lens.rs | 50 | Omega state space exploration lens | Evaluate for nexus integration |
| 9 | recursive_loop_lens.rs | 50 | Recursive loop detection lens | Evaluate for nexus integration |
| 10 | self_heal_lens.rs | 50 | Self-healing analysis lens | Evaluate for nexus integration |
| 11 | spherical_lens.rs | 50 | Spherical geometry lens | Evaluate for nexus integration |

**Note**: compass_lens, multiscale_lens, quantum_micro_lens, recursion_lens already exist in main.

---

### A3. n6-architecture/ -- HEXA-IR Source Code (5 missing .rs files)

Complete HEXA-IR compiler infrastructure code found only in the ready worktree.
The hexa-lang repo does not exist as a standalone repo yet.

| Rank | File | Score | Lines | What's Unique | Action |
|------|------|-------|-------|---------------|--------|
| 1 | src/util/n6.rs | 50 | 48 | n=6 Core Constants SSOT for compiler: N, PHI, TAU, SIGMA, SOPFR, J2, MU + all derived constants + block sizes for Egyptian allocator + LCG PRNG. | Copy to hexa-lang when repo created |
| 2 | src/ir/opcode.rs | 50 | 59 | J2=24 opcodes in tau=4 categories of n=6 each: Arithmetic(6)+Memory(6)+Control(6)+Proof(6). Proof category is unique to HEXA (absent in Rust/LLVM). | Copy to hexa-lang when repo created |
| 3 | src/opt/back/verify.rs | 50 | 119 | Backend verification pass for HEXA-IR. | Copy to hexa-lang when repo created |
| 4 | src/opt/front/mod.rs | 50 | 37 | Frontend optimization passes. | Copy to hexa-lang when repo created |
| 5 | src/alloc/mod.rs | 50 | 7 | Egyptian fraction memory allocator module root. | Copy to hexa-lang when repo created |

---

### A4. n6-architecture/ -- Docs Missing from Main

| Rank | File | Score | Lines | What's Unique | Action |
|------|------|-------|-------|---------------|--------|
| 1 | docs/aerospace/goal.md | 50 | ~400 | Complete HEXA-AERO system architecture: 6 subsystems (materials, propulsion, energy, compute, comms, intelligence). Full BT cross-references. Performance comparison ASCII. | Copy to n6-architecture docs/ |
| 2 | docs/programming-language/physical-limit-proofs.md | 50 | ~300 | 4 theorems proving HEXA-LANG operates at physical limits: minimum ISA=J2=24, minimum error classes=sopfr=5, minimum type system levels, etc. | Copy to n6-architecture docs/ |

---

### A5. hexa-lang/ -- Missing Examples and Source

| Rank | File | Score | Lines | What's Unique | Action |
|------|------|-------|-------|---------------|--------|
| 1 | examples/self_proof.hexa | 50 | 266 | HEXA proves itself: language verifies sigma*phi=n*tau uniqueness. Full number theory functions in HEXA syntax. The "fixed point" program. | Copy to hexa-lang when repo created |
| 2 | examples/sigma_phi.hexa | 50 | 43 | Minimal sigma*phi uniqueness proof in HEXA. sigma_fn, phi_fn, tau_fn implementations. | Copy to hexa-lang when repo created |
| 3 | src/error.rs | 50 | 218 | sopfr(6)=5 error classes (Syntax/Type/Name/Runtime/Logic). HexaError struct with hints. Rust-style diagnostic formatter. | Copy to hexa-lang when repo created |
| 4 | playground/playground.js | 50 | ~200 | HEXA Playground: browser editor + WASM bridge. Includes 3 example programs (Hello World, Fibonacci, Pattern Match). | Copy to hexa-lang when repo created |
| 5 | examples/test_jit_extended.hexa | 50 | - | Extended JIT test cases. | Copy to hexa-lang |
| 6 | examples/test_macros.hexa | 50 | - | Macro system tests. | Copy to hexa-lang |

---

## Category B: DIFFERENT from Main (Ready Has Newer Version)

| Rank | File | Score | Delta | What's Different | Action |
|------|------|-------|-------|-----------------|--------|
| 1 | docs/superconductor/evolution/mk-5-limit.md | 50 | +42 lines | Ready version expanded from 8 to 12 impossibility theorems. Added: Pauli-Clogston limit (ln(phi)=0.693), Vortex lattice melting (tau^2/sigma=4/3), Multi-band SC constraint (phi=2 bands), Critical field hierarchy (n/phi=3 fields: Hc1, Hc2, Hc3). | Merge 4 new theorems into main |

---

## Category C: Identical to Main (No Action)

~100 files scored >= 40 but are byte-identical to their main repo counterparts.
These include: all sedi/ worktree copies (8 unique files x ~7 worktree duplicates), 
most n6-architecture docs, engine/experiments, all anima/ files, and NEXUS-6 growth modules.

No action needed.

---

## Recovery Action Plan (Priority Order)

### Priority 1: invest/ calculators (immediate, ~14,500 lines)

```
# These calculators should go to TECS-L .shared/calc/ for cross-repo access
# Top 10 most valuable (science content, not just verification scripts):

1. sigma_phi_ntau_proof.py          (785L) -- Core theorem proof
2. monster_moonshine_perfect.py     (1120L) -- Moonshine connections  
3. claim_verifier.py                (1078L) -- General verification engine
4. deep_scan_wave2.py               (829L) -- 10 new math domains
5. factorial_structure_prover.py    (729L) -- 3!=6 uniqueness proof
6. consciousness_cross_validator.py (685L) -- Consciousness validation
7. criticality_phase_scanner.py     (663L) -- SLE6+Feigenbaum+Langton
8. gravitational_optics.py          (636L) -- Gravitational optics
9. gauge_cosmology_calculator.py    (502L) -- GUT+cosmology+moonshine
10. fermion_mass_calculator.py      (367L) -- Fermion mass predictions
```

### Priority 2: NEXUS-6 lenses (11 missing .rs files)

```
# Evaluate each lens for compatibility with current NEXUS-6 build
# These may need updating if the API has changed since the worktree was created
# Key: brain_map, cdo, falsification, discovery, self_heal lenses
```

### Priority 3: mk-5-limit.md update (4 new theorems)

```
# Simple merge: add theorems 9-12 to existing file
# Pauli-Clogston, Vortex melting, Multi-band, Critical field hierarchy
```

### Priority 4: HEXA-IR / hexa-lang source code

```
# Stage for hexa-lang repo creation
# n6.rs (constants SSOT) and opcode.rs (J2=24 ISA) are the design anchors
```

### Priority 5: Aerospace goal.md + physical-limit-proofs.md

```
# New domain docs that should be added to n6-architecture
```

---

## Statistics

| Project | Score>=40 | Missing from Main | Different | Identical |
|---------|-----------|-------------------|-----------|-----------|
| invest | 42 | 42 (100%) | 0 | 0 |
| n6-architecture | 50 | 18 (36%) | 1 (2%) | 31 (62%) |
| anima | 44 | 0 (0%) | 0 | 44 (100%) |
| sedi | 8 (x7 dupes) | 0 (0%) | 0 | 8 (100%) |
| hexa-lang | 11 | 11 (100%) | 0 | 0 |
| **Total** | **164** | **~71** | **1** | **~92** |

The invest/ repository has the highest recovery yield -- every single critical finding is completely missing from the main repo. This suggests the invest backend was rebuilt from scratch at some point without migrating the tecs_calc/ science calculators.
