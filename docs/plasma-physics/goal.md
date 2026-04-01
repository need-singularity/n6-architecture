# 궁극의 플라즈마 물리 아키텍처 (Ultimate Plasma Physics)

## Vision
Plasma mastery -- from confinement to control, n=6 governs the fourth state of matter.

## n=6 Foundation
- Matter states = tau(6) = 4 (solid/liquid/gas/plasma) EXACT
- Safety factor q > phi(6) = 2 (Kruskal-Shafranov) EXACT
- Aspect ratio A = sigma/tau = 3 CLOSE
- Triangularity delta = 1/3 EXACT
- beta_plasma = 5% = 1/(J2-tau) = 1/20 (BT-74)
- Confinement modes = phi+mu = 3 (L/H/I)
- D+T mass = sopfr = 5 EXACT
- Heating methods = n/phi = 3 EXACT
- ITER Q=10 = n+tau CLOSE
- Dangerous q-surfaces: div(6) = {1,2,3,6}
- Greenwald density limit: n_G proportional to I_p/a^2
- W7-X field periods = sopfr = 5 EXACT

## DSE Chain (5 Levels)

```
  L1 Foundation ─── 플라즈마 소스 ──── K1=6
  │  Tokamak / Stellarator / RFP / Spheromak / FRC / Z-Pinch
  │
  L2 Process ────── 가둠/안정 공정 ──── K2=6
  │  H-mode / I-mode / QH-mode / Hybrid / AT / Ohmic-L
  │
  L3 Core ────────── 가열/구동 코어 ──── K3=6
  │  NBI / ECRH / ICRH / LHCD / Ohmic / AlfvenWave
  │
  L4 Engine ──────── 제어/진단 엔진 ──── K4=6
  │  MHD_Feedback / NTM_ECCD / RWM_Active / ELM_Pellet / Disruption_Predict / RMP_ELM
  │
  L5 System ──────── 응용 시스템 ────── K5=5
     ITER_Burn / DEMO_Steady / Compact_Pilot / Space_Propulsion / Industrial_Plasma

  Total: 6 x 6 x 6 x 6 x 5 = 6,480 combinations (pre-filter)
```

## Scoring Weights
| Weight | Category | Rationale |
|--------|----------|-----------|
| 0.35   | n6       | n=6 EXACT alignment priority |
| 0.25   | perf     | Plasma performance (Q, confinement time) |
| 0.20   | power    | Power efficiency / recirculating power |
| 0.20   | cost     | Implementation complexity/cost |

## Compatibility Rules
1. Stellarator prefers ECRH (steady-state compatible)
2. FRC/Spheromak excludes ITER_Burn (incompatible scale)
3. Z-Pinch requires Ohmic or AlfvenWave heating only
4. Space_Propulsion excludes Tokamak (too massive)
5. QH-mode requires Tokamak or Stellarator confinement
6. Disruption_Predict relevant only for Tokamak/RFP

## Related Breakthrough Theorems
- **BT-74**: 95/5 cross-domain resonance (beta_plasma=5%, PUE=5%, top-p=0.95)
- **BT-5**: q=1 from Egyptian fraction 1/2+1/3+1/6=1
- **BT-4**: Dangerous q-surfaces from div(6)

## Cross-DSE Targets
- fusion: Plasma source x Fuel x Confinement
- superconductor: Magnet coil material x Plasma confinement
- thermal-management: Divertor heat flux x Cooling system
- chip-architecture: Plasma control FPGA x diagnostics

## Expected Outcomes
- Optimal path: Tokamak + H-mode + NBI + MHD_Feedback + ITER_Burn
- High n6 path: Tokamak + QH-mode + LHCD + RMP_ELM + DEMO_Steady
- n6 EXACT: q>2(phi), A=3(sigma/tau), delta=1/3, beta=5%(BT-74)
- Performance: Q>10, tau_E>5s, T_i>15keV

## Tool
- DSE TOML: `tools/universal-dse/domains/plasma-physics.toml`
- Runner: `tools/universal-dse/universal-dse`
