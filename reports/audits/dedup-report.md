# Test Deduplication Report

Generated: 2026-04-05

## Summary

| Metric | Count |
|--------|-------|
| Total test files | 2 |
| Total test classes | 25 |
| Total test methods | 170 |
| Top-level parametrized tests | 3 |
| Unique test bodies (by AST hash) | 170 |
| Exact body duplicates | 0 |
| Same-name tests across classes | 3 groups |
| Phi6Simple overlap candidates | 5 groups |

## File Breakdown

### tests/test_engine.py (66 tests, 7 classes)

| Class | Tests |
|-------|-------|
| TestThermodynamicFrame | 15 |
| TestLeech24Surface | 11 |
| TestEmergentN6Trainer | 7 |
| TestPhiEfficiencyBridge | 6 |
| TestSEDITrainingMonitor | 9 |
| TestAnimaTensionLoss | 7 |
| TestConsciousnessConstraints | 11 |

### tests/test_techniques.py (104 tests, 18 classes + 3 top-level)

| Class | Tests |
|-------|-------|
| TestPhi6Simple | 8 |
| TestHCNDimensions | 4 |
| TestPhiBottleneck | 4 |
| TestPhiMoE | 4 |
| TestEntropyEarlyStop | 5 |
| TestRFilterPhase | 6 |
| TestTakensDim6 | 6 |
| TestFFTMixAttention | 5 |
| TestZetaLn2Activation | 8 |
| TestEgyptianMoE | 5 |
| TestDedekindHead | 6 |
| TestJordanLeechMoE | 5 |
| TestMobiusSparse | 7 |
| TestCarmichaelLR | 4 |
| TestBoltzmannGate | 5 |
| TestMertensDropout | 4 |
| TestEgyptianAttention | 7 |
| TestEdgeCases | 8 |
| (top-level parametrized) | 3 |

## Exact Duplicates

No tests share an identical AST body. All 170 test methods are structurally unique.

## Same-Name Tests Across Classes

These tests share the same method name but live in different classes. They are NOT duplicates -- each tests a different module's version. Listed for awareness only.

### Group 1: `test_import` (16 occurrences)

Each class has its own `test_import` that verifies a different module can be imported.
This is an expected pattern, not a real duplicate.

- `test_engine.py:40` -- TestThermodynamicFrame
- `test_engine.py:143` -- TestLeech24Surface
- `test_engine.py:229` -- TestEmergentN6Trainer
- `test_engine.py:297` -- TestPhiEfficiencyBridge
- `test_engine.py:345` -- TestSEDITrainingMonitor
- `test_engine.py:426` -- TestAnimaTensionLoss
- `test_engine.py:502` -- TestConsciousnessConstraints
- `test_techniques.py:82` -- TestPhi6Simple
- `test_techniques.py:268` -- TestEntropyEarlyStop
- `test_techniques.py:308` -- TestRFilterPhase
- `test_techniques.py:354` -- TestTakensDim6
- `test_techniques.py:405` -- TestFFTMixAttention
- `test_techniques.py:453` -- TestZetaLn2Activation
- `test_techniques.py:507` -- TestEgyptianMoE
- `test_techniques.py:548` -- TestDedekindHead
- `test_techniques.py:640` -- TestMobiusSparse

**Verdict: NOT duplicates** -- each imports a different module.

### Group 2: `test_constants` (5 occurrences)

- `test_techniques.py:235` -- TestPhiMoE
- `test_techniques.py:594` -- TestJordanLeechMoE
- `test_techniques.py:700` -- TestCarmichaelLR
- `test_techniques.py:744` -- TestBoltzmannGate
- `test_techniques.py:828` -- TestEgyptianAttention

**Verdict: NOT duplicates** -- each verifies different module constants (e.g., PhiMoE checks N_EXPERTS, JordanLeech checks JORDAN_J2, etc.).

### Group 3: `test_charlm_forward` (2 occurrences)

- `test_techniques.py:577` -- TestDedekindHead
- `test_techniques.py:686` -- TestMobiusSparse

**Verdict: POTENTIAL OVERLAP** -- both test a `CharLM` forward pass, but from different technique modules (`dedekind_head.CharLM` vs `mobius_sparse.CharLM`). Different implementations, same test name. Not a true duplicate.

## Conceptual Overlaps (Manual Review Recommended)

### Phi6Simple activation tested in 5+ classes

The `Phi6Simple` activation (x^2 - x + 1, clamped) is tested across multiple technique modules because each module re-exports or re-implements it. These are not duplicates per se, but represent redundant coverage of the same mathematical function.

| File:Line | Class | Method | What it tests |
|-----------|-------|--------|---------------|
| test_engine.py:242 | TestEmergentN6Trainer | test_phi6simple_output | engine's Phi6Simple: f(0)=1, f(1)=1, f(-1)=3 |
| test_techniques.py:93 | TestPhi6Simple | test_phi6_polynomial_values | phi6simple module: f(0)=1, f(1)=1, f(-1)=3, f(2)=3 |
| test_techniques.py:205 | TestPhiBottleneck | test_phi6simple_class | phi_bottleneck.Phi6Simple: f(0)=1, f(1)=1, f(-1)=3, f(0.5)=0.75 |
| test_techniques.py:464 | TestZetaLn2Activation | test_phi6_simple | zetaln2_activation.phi6_simple: f(0)=1, f(0.5)=0.75, f(1)=1 |
| test_techniques.py:570 | TestDedekindHead | test_phi6simple | dedekind_head.Phi6Simple: f(0)=1, f(0.5)=0.75, f(1)=1 |

**Recommendation**: These test the same math (x^2 - x + 1) but from different module imports. If modules were refactored to share a single Phi6Simple implementation, these could be consolidated. No action needed now.

### Tau/sigma/phi constants verified multiple times

Several classes independently verify n=6 constants (sigma=12, phi=2, tau=4, J2=24). This is intentional -- each technique module defines its own constants and they should each be tested independently.

## Conclusion

- **No exact duplicates found** -- all 170 tests have unique AST bodies
- **3 same-name groups** identified, all confirmed as non-duplicates (different modules)
- **5 Phi6Simple overlap candidates** -- same math tested from different imports; consolidation possible if modules share a common implementation
- **No tests recommended for deletion** at this time
