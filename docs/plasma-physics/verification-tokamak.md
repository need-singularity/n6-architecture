# Tokamak Improvement -- Independent Verification

> Independent audit of H-TK-1 through H-TK-8 against real fusion engineering data.
> Performed 2026-03-30.

---

## Grading Scale

- **EXACT**: Claim matches real-world data precisely.
- **CLOSE**: Claim is in the right ballpark but not uniquely derived from n=6.
- **WEAK**: Directionally plausible but numerically off or not predictive.
- **FAIL**: Contradicted by real engineering data or practice.
- **UNVERIFIABLE**: No experimental or simulation data exists to confirm or deny.

---

## Hypothesis Verification

| ID | Claim | Self-Grade | Independent Grade | Reasoning |
|----|-------|------------|-------------------|-----------|
| H-TK-1 | Egyptian field split: BT:BP:B_corr = 1/2:1/3:1/6 (50:33:17%) | WEAK | **FAIL** | Real tokamaks allocate ~70-80% toroidal, ~15-25% poloidal, ~5% correction. The proposed 50:33:17 split would (a) dangerously underpower the toroidal confinement field and (b) over-allocate to correction coils by ~3x. While stronger ELM correction coils is a reasonable research direction, deriving the ratio from Egyptian fractions has no physical basis. The toroidal field dominance is dictated by the Grad-Shafranov equilibrium, not number theory. Self-grade of WEAK is too generous. |
| H-TK-2 | Safety factor q_95 = sopfr(6) = 5 is optimal | CLOSE | **WEAK** | q_95 = 5 falls within the operational window (KSTAR runs q_95 = 4-7), so it is not wrong. However, ITER's design point is q_95 ~ 3, and the optimal q depends on the specific scenario (H-mode access, NTM avoidance, bootstrap fraction). There is no evidence that q = 5 is a unique optimum; it is merely one point in a broad acceptable range. The n=6 derivation adds no predictive power. Downgraded from CLOSE to WEAK because being "in the range" is not the same as identifying an optimum. |
| H-TK-3 | Aspect ratio A = n/phi = 6/2 = 3 is optimal | CLOSE | **CLOSE** | A = 3 is genuinely near the design point for several major tokamaks: ITER 3.1, ARC 3.0, SPARC 3.25. However, KSTAR is 3.6 and many designs cluster around 3.0-3.5 for well-understood physics reasons (balancing beta limits, bootstrap current, and neoclassical transport). The claim lands in the right range but A = 3 is the lower bound of conventional designs, not a unique optimum. The self-grade of CLOSE is fair, though the n=6 derivation remains coincidental -- the physics of the Troyon beta limit and bootstrap current fraction independently constrain A to this range. |
| H-TK-4 | Hexagonal plasma cross-section | UNVERIFIABLE | **FAIL** | The self-grade of UNVERIFIABLE understates the problem. D-shaped cross-sections are used for deep physics reasons: they maximize plasma beta via vertical elongation, enable single-null divertor geometry for heat exhaust, and are MHD-stable at high pressure. A hexagonal cross-section would (a) introduce sharp corners that concentrate heat flux and create MHD-unstable regions, (b) eliminate the proven divertor geometry, (c) require fundamentally different PF coil control with no demonstrated feasibility. The document's claim that ITER uses 6 PF coils is true but irrelevant -- those 6 coils create a D-shape, not a hexagon. Negative triangularity research (cited as related) actually moves further from hexagonal geometry toward reversed-D shapes. This is not merely unverified; it contradicts established MHD stability physics. |
| H-TK-5 | 12 TF coils (sigma(6) = 12) is cost-optimal | FAIL | **FAIL** | Self-grade is correct. No major tokamak uses 12 TF coils: ITER = 18, KSTAR = 16, JET = 32, SPARC = 18. With 12 coils, toroidal field ripple would be ~2-3%, well above the <1% target needed to prevent fast-ion losses. The document mentions HTS coils could enable fewer coils, but even SPARC (which pioneered HTS TF coils at >12 T) still uses 18. The ripple problem is geometric, not a field-strength problem -- stronger coils do not fix the angular spacing issue. 12 TF coils would cause unacceptable energetic particle losses and reduced confinement. |
| H-TK-6 | Heating split NBI:ICRH:ECRH = 3:2:1 | WEAK | **WEAK** | ITER's plan is 33:20:20 MW (ratio ~1.65:1:1), far from 3:2:1. KSTAR uses roughly 8:6:1. Neither matches. The claim that NBI should dominate is directionally correct for many scenarios, but the fixed 3:2:1 ratio ignores that heating mix must be optimized per-scenario: ECRH is critical for NTM stabilization and current drive, ICRH for minority heating, and the optimal mix changes with plasma phase and target. A fixed ratio from number theory cannot capture this physics. Self-grade of WEAK is fair. |
| H-TK-7 | Energy confinement time tau_E = sigma(6) = 12 s | FAIL | **FAIL** | Self-grade is correct. The required tau_E for ITER Q=10 is ~3.7 s. For DEMO-class reactors, estimates are ~5 s. The document's own calculation shows ITER needs ~5 s and DEMO ~2.2 s. A tau_E of 12 s would be extraordinary and far beyond any demonstrated or required value. The largest achieved tau_E is ~0.3-0.5 s (JET, KSTAR). Claiming 12 s as a design target is off by an order of magnitude from current achievement and 2-6x beyond what is needed. The note that "12 s would enable very high Q" is true but irrelevant to whether n=6 predicts a meaningful requirement. |
| H-TK-8 | 4 density control methods (tau(6) = 4) | EXACT | **CLOSE** | The four methods listed (gas puffing, pellet injection, pumping, NBI fueling) are indeed the four primary density control actuators in modern tokamaks. However, this is a post-hoc observation: any engineer would list these four methods regardless of n=6. Additionally, the categorization is somewhat arbitrary -- one could split pumping into cryopumping and turbomolecular, or add supersonic molecular beam injection (SMBI, used on HL-2A and EAST) as a fifth method distinct from gas puffing, or count ELM-flushing as a density control mechanism. The count of "4" depends on how you group the methods. Downgraded from EXACT to CLOSE because the match is real but the categorization is flexible. |

---

## Summary

| Grade | Count | Hypotheses |
|-------|-------|------------|
| FAIL | 4 | H-TK-1, H-TK-4, H-TK-5, H-TK-7 |
| WEAK | 2 | H-TK-2, H-TK-6 |
| CLOSE | 2 | H-TK-3, H-TK-8 |
| EXACT | 0 | -- |

**Overall assessment**: 4 of 8 hypotheses FAIL independent verification. The two CLOSE matches (aspect ratio A ~ 3, four density control methods) are genuine but coincidental -- they reflect well-known engineering constraints, not predictions from perfect number arithmetic. The document's own self-grading was reasonably honest (correctly marking H-TK-5 and H-TK-7 as FAIL), but was too generous on H-TK-1 (WEAK -> FAIL), H-TK-2 (CLOSE -> WEAK), H-TK-4 (UNVERIFIABLE -> FAIL), and H-TK-8 (EXACT -> CLOSE).

The core issue: tokamak design parameters are determined by MHD equilibrium physics, transport theory, and engineering constraints. Matching a few numbers to divisor functions of 6 does not constitute a predictive framework. Where matches occur (A ~ 3, ~4 density methods), they reflect the independent physical constraints, not n=6 arithmetic.
