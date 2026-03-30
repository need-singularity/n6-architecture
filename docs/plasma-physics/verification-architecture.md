# Fusion Architecture -- Independent Verification

Independent verification of hypotheses H-FA-1 through H-FA-5 from `fusion-architecture.md` (Part 3),
checked against real-world tokamak engineering data.

Verification date: 2026-03-30

---

## Reference Data Used

| Parameter | ITER | KSTAR | SPARC |
|-----------|------|-------|-------|
| TF coils | 18 | 16 | 18 |
| PF coils | 6 | 14 | -- |
| CS modules | 6 | -- | -- |
| R_0 (m) | 6.2 | 1.8 | 1.85 |
| a (m) | 2.0 | 0.5 | 0.57 |
| A (aspect ratio) | 3.1 | 3.6 | 3.25 |
| kappa (elongation) | 1.7 | 2.0 | -- |
| B_T (T) | 5.3 | 3.5 | 12.2 |
| Q target | 10 | -- | >10 |

Additional facts:
- Negative triangularity: demonstrated ELM-free in TCV and DIII-D
- Hexagonal cross-section: no tokamak has ever used this; it is entirely speculative
- Egyptian fraction field split (50/33/17): real tokamaks use ~70-80% toroidal, ~15-25% poloidal, ~5% correction
- HTS REBCO: laboratory demonstrations exceed 20 T; commercial tokamak maximum is ~12 T (SPARC design)

---

## Verification Summary

| ID | Claim | Self-Grade | Independent Grade | Notes |
|----|-------|------------|-------------------|-------|
| H-FA-1 | Hexagonal (rounded) cross-section using 6 PF coils at hexagonal vertices | UNVERIFIABLE | **SPECULATIVE -- downgrade to D** | No tokamak has ever used a hexagonal cross-section. The D-shape is backed by 50 years of MHD stability theory and experiment. ITER has 6 PF coils, but KSTAR has 14 -- the "6 PF" premise is not universal. The document correctly identifies ELM suppression as motivation but provides no MHD stability analysis. A hexagonal shape would reduce elongation (kappa), which hurts plasma volume and confinement. The proposal is creative but physically unjustified beyond the n=6 pattern. |
| H-FA-2 | Egyptian fraction magnetic field split: B_T:B_P:B_corr = 1/2:1/3:1/6 (50%:33%:17%) | WEAK | **FAIL -- downgrade from WEAK to F** | Real tokamaks use roughly 70-80% toroidal, 15-25% poloidal, and 3-5% correction field energy. The proposed 50:33:17 split would cut toroidal confinement by ~30 percentage points, which is catastrophic -- toroidal field is the primary confinement mechanism. Boosting correction coil allocation from 5% to 17% is a factor of 3.4x increase with no physics basis. The document itself acknowledges "BT를 50%로 줄이면 toroidal confinement 약화" but still presents the ratio. The self-grade of WEAK is too generous; the fixed ratio directly contradicts known tokamak physics. |
| H-FA-3 | HTS coils achieving sigma(6)=12 T, enabling reduction to 12 TF coils | SPECULATIVE | **SPECULATIVE -- agree, with caveats** | The 12 T claim has partial support: SPARC targets 12.2 T on-axis using REBCO HTS, which is a genuine match to sigma(6)=12. However, reducing to 12 TF coils is a separate and much weaker claim. All modern large tokamaks (ITER, JT-60SA, SPARC) use 18 TF coils. Reducing to 12 would increase field ripple exponentially. The document's own ripple formula shows this is problematic. The 12 T field strength match is CLOSE; the 12-coil reduction is FAIL. The self-grade of SPECULATIVE is fair for the combined claim but masks that the two sub-claims have very different validity. |
| H-FA-4 | Negative triangularity + hexagonal hybrid: alternating positive/negative triangularity on 6 hexagonal faces | HIGHLY SPECULATIVE | **HIGHLY SPECULATIVE -- agree, note additional problems** | The document correctly states that negative triangularity (NT) achieves ELM-free operation in TCV and DIII-D. This is confirmed experimentally. However, the hybrid proposal of applying different triangularity values to different faces of a hexagonal cross-section has never been studied and may be physically incoherent: plasma equilibrium is governed by Grad-Shafranov, which produces smooth flux surfaces, not faceted shapes. "Triangularity" is a global shape parameter of a flux surface, not a local property that can be varied face-by-face. The proposal conflates coil positions with flux surface geometry. The self-grade is honest. |
| H-FA-5 | Integrated "N6 Tokamak" design combining H-FA-1 through H-FA-4 | (implicit: SPECULATIVE) | **SPECULATIVE-TO-FAIL -- downgrade** | This is a summary design that inherits the problems of H-FA-1 through H-FA-4. Specific issues: (1) "TF coils: 12? or 18" -- the document hedges, but 12 is unsupported as verified above; (2) Egyptian field split 1/2:1/3:1/6 contradicts real tokamak design; (3) hexagonal cross-section is unverified; (4) the parts that match reality (6 PF, 6 CS, A~3, Q=10, ~10 keV) are drawn from ITER's existing design, not predicted by n=6 arithmetic. The components that are genuinely new (hexagonal shape, Egyptian field split, 12 TF coils) range from speculative to physically incorrect. |

---

## Detailed Analysis by Hypothesis

### H-FA-1: Hexagonal Cross-Section

**Self-grade**: UNVERIFIABLE
**Independent grade**: SPECULATIVE (D)

The proposal rests on two pillars:
1. ITER uses 6 PF coils, so arrange them as a hexagon.
2. Hexagons have favorable perimeter-to-area ratio (second only to circles).

Problems identified:

- **PF coil count is not universal**: ITER has 6 PF coils, but KSTAR has 14. The "6 PF" premise only holds for one machine.
- **Elongation loss**: Current tokamaks use kappa = 1.7-2.0. A hexagonal cross-section would tend toward kappa ~ 1.15 (a regular hexagon is nearly circular with aspect ratio ~1.15). This reduces plasma volume and fusion power for a given major radius.
- **No MHD precedent**: Every tokamak ever built uses a D-shaped or circular cross-section. The D-shape maximizes vertical stability and beta limits via the Shafranov shift.
- **Divertor geometry**: The claim of "increased divertor access area" is unsubstantiated. Current divertors rely on the X-point geometry of D-shaped plasmas.
- **Credit where due**: The document honestly lists risks and calls for MHD simulation. Downgraded from UNVERIFIABLE to SPECULATIVE (D) because enough is known about MHD stability to say this is unlikely to outperform D-shape, even though formal simulation has not been done.

### H-FA-2: Egyptian Fraction Magnetic Field Split

**Self-grade**: WEAK
**Independent grade**: FAIL (F)

Real-world field energy allocation:
- Toroidal: ~70-80% (ITER, KSTAR, SPARC all confirm this range)
- Poloidal: ~15-25%
- Correction: ~3-5%

The proposed 50:33:17 split would:
- Reduce toroidal confinement field energy by 20-30 percentage points -- this directly degrades the key confinement mechanism.
- Triple the correction coil allocation with no demonstrated physics benefit.
- The document suggests HTS could compensate by maintaining absolute B_T while increasing B_P and B_corr, but this means total magnetic energy increases substantially, raising cost and engineering complexity with no clear gain.

The self-grade of WEAK is too generous. This is a FAIL because the ratio is derived from mathematical aesthetics (1/2+1/3+1/6=1) rather than plasma physics, and it directly contradicts well-established engineering ratios.

### H-FA-3: HTS Coils at sigma(6) = 12 T

**Self-grade**: SPECULATIVE
**Independent grade**: SPECULATIVE (agree)

Two sub-claims require separate evaluation:

1. **12 T field strength**: SPARC is designed for 12.2 T using REBCO HTS. This is a genuine near-match to sigma(6)=12. Grade for this sub-claim: **CLOSE**. However, 12 T is not a universal target -- it is specific to SPARC's compact design philosophy. ITER operates at 5.3 T, and KSTAR at 3.5 T.

2. **12 TF coils**: No modern tokamak uses 12 TF coils. Field ripple scales as ~exp(-N * sqrt(2 * r/R)), and reducing from 18 to 12 coils would increase ripple by roughly an order of magnitude at the plasma edge, causing fast-ion losses and reduced confinement. DIII-D uses 24 TF coils (matching J_2(6)=24, but this is coincidental).

The self-grade of SPECULATIVE is fair for the combined claim.

### H-FA-4: Negative Triangularity + Hexagonal Hybrid

**Self-grade**: HIGHLY SPECULATIVE
**Independent grade**: HIGHLY SPECULATIVE (agree)

- Negative triangularity is real and experimentally demonstrated (TCV, DIII-D).
- However, triangularity is a global flux-surface shape parameter defined by the Grad-Shafranov equilibrium. It cannot be varied locally on different "faces" of a cross-section. Plasma flux surfaces are smooth, nested curves determined by the equilibrium -- not faceted polygons.
- The concept of "alternating positive/negative triangularity on 6 faces" reflects a misunderstanding of how plasma shaping works. PF coil positions influence the global equilibrium, not local surface segments independently.
- The self-grade is honest and appropriate.

### H-FA-5: Integrated N6 Tokamak Concept

**Self-grade**: implicit SPECULATIVE
**Independent grade**: SPECULATIVE-TO-FAIL

The integrated design bundles verified facts with unverified speculation:

| Component | Status |
|-----------|--------|
| 6 PF coils | Matches ITER (but not KSTAR's 14) |
| 6 CS modules | Matches ITER |
| A = 3 | Reasonable; within standard range (3-4) |
| Q = 10 | Matches ITER target; = sopfr*phi is a numerical coincidence |
| ~10 keV | Standard D-T operating temperature; not a prediction |
| Hexagonal cross-section | SPECULATIVE -- no precedent |
| Egyptian field split | FAIL -- contradicts real ratios |
| 12 TF coils | FAIL -- ripple unacceptable |
| 12 T HTS | CLOSE -- matches SPARC |
| NT hybrid on hex faces | HIGHLY SPECULATIVE -- likely physically incoherent |

The design's credible elements are inherited from ITER's existing parameters. The novel elements (hexagonal shape, Egyptian split, 12 TF coils, face-local triangularity) range from unsupported to contradicted by known physics.

---

## Grade Comparison Summary

| ID | Self-Grade | Independent Grade | Direction |
|----|------------|-------------------|-----------|
| H-FA-1 | UNVERIFIABLE | SPECULATIVE (D) | Downgrade |
| H-FA-2 | WEAK | FAIL (F) | Downgrade |
| H-FA-3 | SPECULATIVE | SPECULATIVE | No change |
| H-FA-4 | HIGHLY SPECULATIVE | HIGHLY SPECULATIVE | No change |
| H-FA-5 | ~SPECULATIVE | SPECULATIVE-TO-FAIL | Downgrade |

**Overall assessment**: The document is commendably honest about failures (especially the TF coil count). However, H-FA-2 deserves a harder grade than WEAK -- the Egyptian fraction field split directly contradicts known tokamak engineering. H-FA-1 is downgraded because enough MHD theory exists to assess (not merely "unverifiable"). H-FA-3 and H-FA-4 self-grades are fair. H-FA-5 inherits the weaknesses of its components.
