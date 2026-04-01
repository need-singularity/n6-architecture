# N6 Anomaly Detection Results

> Generated: 2026-04-02 08:47
> Threshold: n6 < 0.50

## Summary Statistics

| Metric | Value |
|--------|-------|
| TOML files parsed | 305 |
| Total candidates | 9206 |
| Anomalies (n6 < 0.50) | 150 |
| Anomaly rate | 1.6% |
| Genuinely non-n6 | 63 |
| Undiscovered formula candidates | 87 |
| N6 expressions checked | 6153 |

## N6 Score Distribution (all candidates)

| Score Range | Count | Pct |
|-------------|-------|-----|
| 0.0-0.1 | 14 | 0.2%  |
| 0.1-0.2 | 1 | 0.0%  |
| 0.2-0.3 | 60 | 0.7%  |
| 0.3-0.4 | 34 | 0.4%  |
| 0.4-0.5 | 41 | 0.4%  |
| 0.5-0.6 | 647 | 7.0% ####### |
| 0.6-0.7 | 597 | 6.5% ###### |
| 0.7-0.8 | 1069 | 11.6% ########### |
| 0.8-0.9 | 1105 | 12.0% ############ |
| 0.9-1.0 | 204 | 2.2% ## |
| 1.0-1.1 | 5434 | 59.0% ########################################################### |

## Domains with Highest Anomaly Rates

| Domain | Anomalies | Total | Rate |
|--------|-----------|-------|------|
| energy_gen | 12 | 27 | 44% |
| wafer-fabrication | 8 | 30 | 27% |
| chip | 11 | 50 | 22% |
| solar | 6 | 28 | 21% |
| grid | 5 | 24 | 21% |
| risc-v-core | 6 | 30 | 20% |
| compiler-os | 5 | 27 | 19% |
| battery | 4 | 27 | 15% |
| asic-design | 4 | 30 | 13% |
| consciousness-thermodynamics | 4 | 30 | 13% |
| golden-moe-routing | 4 | 30 | 13% |
| gan-power-device | 4 | 31 | 13% |
| dsp-processor | 3 | 30 | 10% |
| mems-sensor | 3 | 30 | 10% |
| polymer-composite | 3 | 30 | 10% |
| railway-system | 3 | 30 | 10% |
| silicon-wafer | 3 | 30 | 10% |
| soc-integration | 3 | 30 | 10% |
| hair-regeneration | 3 | 32 | 9% |
| superconductor | 3 | 34 | 9% |
| noncommutative-geometry | 2 | 29 | 7% |
| agent-platform | 2 | 30 | 7% |
| aluminum-alloy | 2 | 30 | 7% |
| consciousness-measurement | 2 | 30 | 7% |
| consciousness-scaling | 2 | 30 | 7% |
| consciousness-training | 2 | 30 | 7% |
| earthquake-engineering | 2 | 30 | 7% |
| fluid-condensed | 2 | 30 | 7% |
| neuromorphic-loihi | 2 | 30 | 7% |
| consciousness-rng | 2 | 31 | 6% |

## Top 20 Undiscovered Formula Candidates

These anomalies have numeric values in their labels that match n6 expressions within 5%.

| # | Domain | Level | ID | n6 | Value | Matching Expression | Expr Value | Error |
|---|--------|-------|----|----|-------|---------------------|------------|-------|
| 1 | asic-design |  | Intel-18A | 0.25 | 18.0 | n+sigma | 18.0000 | 0.00% |
| 2 | asic-design |  | SMIC-28nm | 0.25 | 28.0 | tau+J2 | 28.0000 | 0.00% |
| 3 | battery | Core | 21700 | 0.33 | 1.0 | mu | 1.0000 | 0.00% |
| 4 | battery | Core | 4680 | 0.33 | 1.0 | mu | 1.0000 | 0.00% |
| 5 | battery | Core | Prismatic | 0.33 | 1.0 | mu | 1.0000 | 0.00% |
| 6 | battery | Core | Pouch | 0.33 | 1.0 | mu | 1.0000 | 0.00% |
| 7 | chaos-dynamical | Bifurcation | SaddleNode | 0.33 | 1.0 | mu | 1.0000 | 0.00% |
| 8 | chip |  | Intel_18A | 0.20 | 18.0 | n+sigma | 18.0000 | 0.00% |
| 9 | chip |  | TSMC_A16 | 0.40 | 6.0 | n | 6.0000 | 0.00% |
| 10 | chip |  | HEXA-Lite | 0.29 | 72.0 | n*sigma | 72.0000 | 0.00% |
| 11 | chip |  | ARM-Big | 0.43 | 16.0 | tau^2 | 16.0000 | 0.00% |
| 12 | chip |  | x86-Zen5 | 0.00 | 6.0 | n | 6.0000 | 0.00% |
| 13 | chip |  | RISC-V | 0.00 | 48.0 | sigma*tau | 48.0000 | 0.00% |
| 14 | chip |  | HEXA-1_Half | 0.00 | 1.0 | mu | 1.0000 | 0.00% |
| 15 | chip |  | H100_Style | 0.00 | 6.0 | n | 6.0000 | 0.00% |
| 16 | chip |  | M4Ultra_Style | 0.00 | 150.0 | (n*sopfr)*sopfr | 150.0000 | 0.00% |
| 17 | chip |  | TPU_Pod | 0.20 | 4.0 | tau | 4.0000 | 0.00% |
| 18 | chip |  | Edge_Compact | 0.00 | 1.0 | mu | 1.0000 | 0.00% |
| 19 | compiler-os | Foundation | x86_64 | 0.40 | 6.0 | n | 6.0000 | 0.00% |
| 20 | compiler-os | Pipeline | AOT_Minimal | 0.20 | 3.0 | n/phi | 3.0000 | 0.00% |

## All New Formula Candidates (n6 < 0.50 with inverse match)

Total: 87 candidates with potential n6 connections

### 1. asic-design / Intel-18A
- Level:  | n6 score: 0.25
- Label: Intel 18A (1.8nm class, RibbonFET, PowerVia backside power)
- Matches:
  - 18.0 ~ n+sigma = 18.0000 (error: 0.00%)
  - 18.0 ~ sigma+n = 18.0000 (error: 0.00%)
  - 18.0 ~ J2-n = 18.0000 (error: 0.00%)
  - 18.0 ~ (n+sigma)*mu = 18.0000 (error: 0.00%)
  - 18.0 ~ (n+sigma)/mu = 18.0000 (error: 0.00%)

### 2. asic-design / SMIC-28nm
- Level:  | n6 score: 0.25
- Label: SMIC 28nm (mature poly/SiON, lowest cost, no EUV needed)
- Matches:
  - 28.0 ~ tau+J2 = 28.0000 (error: 0.00%)
  - 28.0 ~ J2+tau = 28.0000 (error: 0.00%)
  - 28.0 ~ (n*tau)+tau = 28.0000 (error: 0.00%)
  - 28.0 ~ (n-phi)+J2 = 28.0000 (error: 0.00%)
  - 28.0 ~ (n*sopfr)-phi = 28.0000 (error: 0.00%)

### 3. battery / 21700
- Level: Core | n6 score: 0.33
- Label: 21700 Cylindrical (1.15x, thermal 0.82)
- Matches:
  - 21700.0 ~ sigma^4 = 20736.0000 (error: 4.44%)
  - 21700.0 ~ sigma^tau = 20736.0000 (error: 4.44%)
  - 21700.0 ~ sigma^(n-phi) = 20736.0000 (error: 4.44%)
  - 21700.0 ~ (n*phi)^tau = 20736.0000 (error: 4.44%)
  - 21700.0 ~ (n*J2)^phi = 20736.0000 (error: 4.44%)

### 4. battery / 4680
- Level: Core | n6 score: 0.33
- Label: 4680 Cylindrical (1.30x, thermal 0.70)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 5. battery / Prismatic
- Level: Core | n6 score: 0.33
- Label: Prismatic (1.10x, thermal 0.75)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 6. battery / Pouch
- Level: Core | n6 score: 0.33
- Label: Pouch (1.25x, thermal 0.65)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 7. chaos-dynamical / SaddleNode
- Level: Bifurcation | n6 score: 0.33
- Label: 안장-마디 분기
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 8. chip / Intel_18A
- Level:  | n6 score: 0.20
- Label: Intel 18A (44nm gate, 30nm metal, 13L, 4NS, 22 EUV, 70% yield)
- Matches:
  - 18.0 ~ n+sigma = 18.0000 (error: 0.00%)
  - 18.0 ~ sigma+n = 18.0000 (error: 0.00%)
  - 18.0 ~ J2-n = 18.0000 (error: 0.00%)
  - 18.0 ~ (n+sigma)*mu = 18.0000 (error: 0.00%)
  - 18.0 ~ (n+sigma)/mu = 18.0000 (error: 0.00%)

### 9. chip / TSMC_A16
- Level:  | n6 score: 0.40
- Label: TSMC A16 (48nm gate, 24nm metal, 14L, 4NS, 28 EUV, 60% yield)
- Matches:
  - 6.0 ~ n = 6.0000 (error: 0.00%)
  - 6.0 ~ n*mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n/mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n^mu = 6.0000 (error: 0.00%)
  - 6.0 ~ sigma-n = 6.0000 (error: 0.00%)

### 10. chip / HEXA-Lite
- Level:  | n6 score: 0.29
- Label: HEXA-Lite (4P+2E, 72 SM, 12 NPU, 128 FP32/SM, 256 ROB)
- Matches:
  - 72.0 ~ n*sigma = 72.0000 (error: 0.00%)
  - 72.0 ~ sigma*n = 72.0000 (error: 0.00%)
  - 72.0 ~ (n+sigma)*tau = 72.0000 (error: 0.00%)
  - 72.0 ~ (n*sigma)*mu = 72.0000 (error: 0.00%)
  - 72.0 ~ (n*sigma)/mu = 72.0000 (error: 0.00%)

### 11. chip / ARM-Big
- Level:  | n6 score: 0.43
- Label: ARM Big.LITTLE (8P+4E, 16 SM, 8 NPU, 64 FP32/SM, 320 ROB)
- Matches:
  - 16.0 ~ tau^2 = 16.0000 (error: 0.00%)
  - 16.0 ~ 2^tau = 16.0000 (error: 0.00%)
  - 16.0 ~ phi^4 = 16.0000 (error: 0.00%)
  - 16.0 ~ sigma+tau = 16.0000 (error: 0.00%)
  - 16.0 ~ tau+sigma = 16.0000 (error: 0.00%)

### 12. chip / x86-Zen5
- Level:  | n6 score: 0.00
- Label: x86 Zen5 (16P+0E, 0 SM, 0 NPU, 0 FP32/SM, 448 ROB)
- Matches:
  - 6.0 ~ n = 6.0000 (error: 0.00%)
  - 6.0 ~ n*mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n/mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n^mu = 6.0000 (error: 0.00%)
  - 6.0 ~ sigma-n = 6.0000 (error: 0.00%)

### 13. chip / RISC-V
- Level:  | n6 score: 0.00
- Label: RISC-V (12P+6E, 48 SM, 12 NPU, 64 FP32/SM, 128 ROB)
- Matches:
  - 48.0 ~ sigma*tau = 48.0000 (error: 0.00%)
  - 48.0 ~ tau*sigma = 48.0000 (error: 0.00%)
  - 48.0 ~ phi*J2 = 48.0000 (error: 0.00%)
  - 48.0 ~ J2*phi = 48.0000 (error: 0.00%)
  - 48.0 ~ (n*sigma)-J2 = 48.0000 (error: 0.00%)

### 14. chip / HEXA-1_Half
- Level:  | n6 score: 0.00
- Label: HEXA-1 Half (4 HBM, 144GB, 150W TDP, 400mm2, 3 CoWoS)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 15. chip / H100_Style
- Level:  | n6 score: 0.00
- Label: H100 Style (6 HBM, 96GB, 700W TDP, 814mm2, 4 CoWoS)
- Matches:
  - 6.0 ~ n = 6.0000 (error: 0.00%)
  - 6.0 ~ n*mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n/mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n^mu = 6.0000 (error: 0.00%)
  - 6.0 ~ sigma-n = 6.0000 (error: 0.00%)

### 16. chip / M4Ultra_Style
- Level:  | n6 score: 0.00
- Label: M4 Ultra Style (0 HBM, unified, 150W TDP, 400mm2, 2 CoWoS)
- Matches:
  - 150.0 ~ (n*sopfr)*sopfr = 150.0000 (error: 0.00%)
  - 150.0 ~ (n+J2)*sopfr = 150.0000 (error: 0.00%)
  - 150.0 ~ (n*J2)+n = 150.0000 (error: 0.00%)
  - 150.0 ~ (sopfr*n)*sopfr = 150.0000 (error: 0.00%)
  - 150.0 ~ (J2+n)*sopfr = 150.0000 (error: 0.00%)

### 17. chip / TPU_Pod
- Level:  | n6 score: 0.20
- Label: TPU Pod (4 GPU/node, 24 node/rack, 40kW, 3-tier, PUE=1.1)
- Matches:
  - 4.0 ~ tau = 4.0000 (error: 0.00%)
  - 4.0 ~ phi^2 = 4.0000 (error: 0.00%)
  - 4.0 ~ 2^phi = 4.0000 (error: 0.00%)
  - 4.0 ~ n-phi = 4.0000 (error: 0.00%)
  - 4.0 ~ tau*mu = 4.0000 (error: 0.00%)

### 18. chip / Edge_Compact
- Level:  | n6 score: 0.00
- Label: Edge Compact (1 GPU/node, 48 node/rack, 24kW, 2-tier, PUE=1.5)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 19. compiler-os / x86_64
- Level: Foundation | n6 score: 0.40
- Label: x86-64 (variable encoding, 16 GPR, 4 rings)
- Matches:
  - 6.0 ~ n = 6.0000 (error: 0.00%)
  - 6.0 ~ n*mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n/mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n^mu = 6.0000 (error: 0.00%)
  - 6.0 ~ sigma-n = 6.0000 (error: 0.00%)

### 20. compiler-os / AOT_Minimal
- Level: Pipeline | n6 score: 0.20
- Label: AOT Minimal (3-stage, 3-pass, no IR expansion)
- Matches:
  - 3.0 ~ n/phi = 3.0000 (error: 0.00%)
  - 3.0 ~ sigma/tau = 3.0000 (error: 0.00%)
  - 3.0 ~ tau-mu = 3.0000 (error: 0.00%)
  - 3.0 ~ phi+mu = 3.0000 (error: 0.00%)
  - 3.0 ~ sopfr-phi = 3.0000 (error: 0.00%)

### 21. compiler-os / Unikernel
- Level: Runtime | n6 score: 0.10
- Label: Unikernel (2-state, 1-priority, zero-copy, single-thread)
- Matches:
  - 2.0 ~ phi = 2.0000 (error: 0.00%)
  - 2.0 ~ n-tau = 2.0000 (error: 0.00%)
  - 2.0 ~ sigma/n = 2.0000 (error: 0.00%)
  - 2.0 ~ tau-phi = 2.0000 (error: 0.00%)
  - 2.0 ~ tau/phi = 2.0000 (error: 0.00%)

### 22. compiler-os / Cloud_Native
- Level: Ecosystem | n6 score: 0.30
- Label: Cloud-Native (container, K8s scheduling, auto-scaling)
- Matches:
  - 12.0 ~ sigma = 12.0000 (error: 0.00%)
  - 12.0 ~ n*phi = 12.0000 (error: 0.00%)
  - 12.0 ~ sigma*mu = 12.0000 (error: 0.00%)
  - 12.0 ~ sigma/mu = 12.0000 (error: 0.00%)
  - 12.0 ~ sigma^mu = 12.0000 (error: 0.00%)

### 23. consciousness-hardware / DigitalGPU
- Level: Substrate | n6 score: 0.33
- Label: Digital GPU — 3.5/7 consciousness (baseline)
- Matches:
  - 3.5 ~ (n/tau)+phi = 3.5000 (error: 0.00%)
  - 3.5 ~ (n+mu)/phi = 3.5000 (error: 0.00%)
  - 3.5 ~ (sigma+phi)/tau = 3.5000 (error: 0.00%)
  - 3.5 ~ (sigma-sopfr)/phi = 3.5000 (error: 0.00%)
  - 3.5 ~ (phi+sigma)/tau = 3.5000 (error: 0.00%)

### 24. consciousness-measurement / NoOpt
- Level: Optimization | n6 score: 0.25
- Label: No Optimization (single evaluation)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 25. consciousness-scaling / Complete
- Level: Topology | n6 score: 0.25
- Label: 완전 연결 그래프
- Matches:
  - 2.0 ~ phi = 2.0000 (error: 0.00%)
  - 2.0 ~ n-tau = 2.0000 (error: 0.00%)
  - 2.0 ~ sigma/n = 2.0000 (error: 0.00%)
  - 2.0 ~ tau-phi = 2.0000 (error: 0.00%)
  - 2.0 ~ tau/phi = 2.0000 (error: 0.00%)

### 26. consciousness-scaling / Quadratic
- Level: ScalingLaw | n6 score: 0.25
- Label: N^2 이차 스케일링
- Matches:
  - 2.0 ~ phi = 2.0000 (error: 0.00%)
  - 2.0 ~ n-tau = 2.0000 (error: 0.00%)
  - 2.0 ~ sigma/n = 2.0000 (error: 0.00%)
  - 2.0 ~ tau-phi = 2.0000 (error: 0.00%)
  - 2.0 ~ tau/phi = 2.0000 (error: 0.00%)

### 27. consciousness-thermodynamics / Gaussian
- Level: NoiseModel | n6 score: 0.25
- Label: Gaussian White Noise (mu=0, sigma tunable)
- Matches:
  - 2.0 ~ phi = 2.0000 (error: 0.00%)
  - 2.0 ~ n-tau = 2.0000 (error: 0.00%)
  - 2.0 ~ sigma/n = 2.0000 (error: 0.00%)
  - 2.0 ~ tau-phi = 2.0000 (error: 0.00%)
  - 2.0 ~ tau/phi = 2.0000 (error: 0.00%)

### 28. consciousness-training / Single
- Level: PhaseStrategy | n6 score: 0.25
- Label: Single Phase (no transitions)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 29. consciousness-training / Solo
- Level: Federation | n6 score: 0.25
- Label: Solo Training (single GPU/node)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 30. cosmology-particle / Anthropic
- Level: SymmetryOrigin | n6 score: 0.40
- Label: Anthropic Principle
- Matches:
  - 6.0 ~ n = 6.0000 (error: 0.00%)
  - 6.0 ~ n*mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n/mu = 6.0000 (error: 0.00%)
  - 6.0 ~ n^mu = 6.0000 (error: 0.00%)
  - 6.0 ~ sigma-n = 6.0000 (error: 0.00%)

### 31. crystallography / TriclinicN1
- Level: System | n6 score: 0.25
- Label: 삼사정 mu=1 최소 대칭
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 32. dsp-processor / MAC-16bit
- Level:  | n6 score: 0.25
- Label: MAC 16-bit (16×16→32 accumulator, standard fixed-point)
- Matches:
  - 16.0 ~ tau^2 = 16.0000 (error: 0.00%)
  - 16.0 ~ 2^tau = 16.0000 (error: 0.00%)
  - 16.0 ~ phi^4 = 16.0000 (error: 0.00%)
  - 16.0 ~ sigma+tau = 16.0000 (error: 0.00%)
  - 16.0 ~ tau+sigma = 16.0000 (error: 0.00%)

### 33. energy_gen / Wind_Turbine
- Level: Source | n6 score: 0.33
- Label: Wind Turbine
- Matches:
  - 59.3 ~ (sigma*sopfr)-mu = 59.0000 (error: 0.51%)
  - 59.3 ~ (sopfr*sigma)-mu = 59.0000 (error: 0.51%)
  - 59.3 ~ sigma*sopfr = 60.0000 (error: 1.17%)
  - 59.3 ~ sopfr*sigma = 60.0000 (error: 1.17%)
  - 59.3 ~ (n*sigma)-sigma = 60.0000 (error: 1.17%)

### 34. energy_gen / Geothermal
- Level: Source | n6 score: 0.00
- Label: Geothermal Energy
- Matches:
  - 90.0 ~ (n+sigma)*sopfr = 90.0000 (error: 0.00%)
  - 90.0 ~ (sigma+n)*sopfr = 90.0000 (error: 0.00%)
  - 90.0 ~ (tau*J2)-n = 90.0000 (error: 0.00%)
  - 90.0 ~ (J2-n)*sopfr = 90.0000 (error: 0.00%)
  - 90.0 ~ (J2*tau)-n = 90.0000 (error: 0.00%)

### 35. energy_gen / Hydroelectric
- Level: Source | n6 score: 0.33
- Label: Hydroelectric Power
- Matches:
  - 44.0 ~ (n+sopfr)*tau = 44.0000 (error: 0.00%)
  - 44.0 ~ (sigma*tau)-tau = 44.0000 (error: 0.00%)
  - 44.0 ~ (sigma-mu)*tau = 44.0000 (error: 0.00%)
  - 44.0 ~ (tau*sigma)-tau = 44.0000 (error: 0.00%)
  - 44.0 ~ (tau*sopfr)+J2 = 44.0000 (error: 0.00%)

### 36. energy_gen / Gas_Turbine
- Level: Conversion | n6 score: 0.33
- Label: Gas Turbine (Brayton)
- Matches:
  - 40.0 ~ (n+tau)*tau = 40.0000 (error: 0.00%)
  - 40.0 ~ (n+phi)*sopfr = 40.0000 (error: 0.00%)
  - 40.0 ~ (sigma+tau)+J2 = 40.0000 (error: 0.00%)
  - 40.0 ~ (sigma-tau)*sopfr = 40.0000 (error: 0.00%)
  - 40.0 ~ (sigma-phi)*tau = 40.0000 (error: 0.00%)

### 37. energy_gen / Combined_Cycle
- Level: Conversion | n6 score: 0.33
- Label: Combined Cycle Gas Turbine
- Matches:
  - 60.0 ~ sigma*sopfr = 60.0000 (error: 0.00%)
  - 60.0 ~ sopfr*sigma = 60.0000 (error: 0.00%)
  - 60.0 ~ (n*sigma)-sigma = 60.0000 (error: 0.00%)
  - 60.0 ~ (n+tau)*n = 60.0000 (error: 0.00%)
  - 60.0 ~ (n*phi)*sopfr = 60.0000 (error: 0.00%)

### 38. energy_gen / Pumped_Hydro
- Level: Storage | n6 score: 0.00
- Label: Pumped Hydroelectric Storage
- Matches:
  - 80.0 ~ (sigma+tau)*sopfr = 80.0000 (error: 0.00%)
  - 80.0 ~ (tau+sigma)*sopfr = 80.0000 (error: 0.00%)
  - 80.0 ~ (tau*sopfr)*tau = 80.0000 (error: 0.00%)
  - 80.0 ~ (sopfr*tau)*tau = 80.0000 (error: 0.00%)
  - 80.0 ~ (J2-tau)*tau = 80.0000 (error: 0.00%)

### 39. energy_gen / CAES
- Level: Storage | n6 score: 0.00
- Label: Compressed Air Energy Storage
- Matches:
  - 70.0 ~ (n*sigma)-phi = 70.0000 (error: 0.00%)
  - 70.0 ~ (sigma*n)-phi = 70.0000 (error: 0.00%)
  - 70.0 ~ (sigma+phi)*sopfr = 70.0000 (error: 0.00%)
  - 70.0 ~ (phi+sigma)*sopfr = 70.0000 (error: 0.00%)
  - 70.0 ~ (n*sigma)-mu = 71.0000 (error: 1.41%)

### 40. energy_gen / Flywheel
- Level: Storage | n6 score: 0.33
- Label: Flywheel Energy Storage
- Matches:
  - 95.0 ~ (tau*J2)-mu = 95.0000 (error: 0.00%)
  - 95.0 ~ (J2*tau)-mu = 95.0000 (error: 0.00%)
  - 95.0 ~ (J2-sopfr)*sopfr = 95.0000 (error: 0.00%)
  - 95.0 ~ tau*J2 = 96.0000 (error: 1.04%)
  - 95.0 ~ J2*tau = 96.0000 (error: 1.04%)

### 41. energy_gen / Distribution_MV
- Level: GridConnect | n6 score: 0.33
- Label: Distribution Medium Voltage
- Matches:
  - 11.0 ~ n+sopfr = 11.0000 (error: 0.00%)
  - 11.0 ~ sigma-mu = 11.0000 (error: 0.00%)
  - 11.0 ~ sopfr+n = 11.0000 (error: 0.00%)
  - 11.0 ~ (n+tau)+mu = 11.0000 (error: 0.00%)
  - 11.0 ~ (n*phi)-mu = 11.0000 (error: 0.00%)

### 42. fluid-condensed / StokesLow
- Level: FluidBase | n6 score: 0.40
- Label: 스토크스 저레이놀즈 (phi=2 선형/비선형)
- Matches:
  - 2.0 ~ phi = 2.0000 (error: 0.00%)
  - 2.0 ~ n-tau = 2.0000 (error: 0.00%)
  - 2.0 ~ sigma/n = 2.0000 (error: 0.00%)
  - 2.0 ~ tau-phi = 2.0000 (error: 0.00%)
  - 2.0 ~ tau/phi = 2.0000 (error: 0.00%)

### 43. fluid-condensed / RANS_Average
- Level: Turbulence | n6 score: 0.40
- Label: 레이놀즈 평균 (phi=2 평균/변동)
- Matches:
  - 2.0 ~ phi = 2.0000 (error: 0.00%)
  - 2.0 ~ n-tau = 2.0000 (error: 0.00%)
  - 2.0 ~ sigma/n = 2.0000 (error: 0.00%)
  - 2.0 ~ tau-phi = 2.0000 (error: 0.00%)
  - 2.0 ~ tau/phi = 2.0000 (error: 0.00%)

### 44. fusion / Z_Pinch
- Level: Confinement | n6 score: 0.40
- Label: Z-Pinch (pulsed linear)
- Matches:
  - 2.0 ~ phi = 2.0000 (error: 0.00%)
  - 2.0 ~ n-tau = 2.0000 (error: 0.00%)
  - 2.0 ~ sigma/n = 2.0000 (error: 0.00%)
  - 2.0 ~ tau-phi = 2.0000 (error: 0.00%)
  - 2.0 ~ tau/phi = 2.0000 (error: 0.00%)

### 45. fusion / Laser
- Level: Heating | n6 score: 0.45
- Label: Laser Drive (for ICF)
- Matches:
  - 1.0 ~ mu = 1.0000 (error: 0.00%)
  - 1.0 ~ 1/mu = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^2 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^3 = 1.0000 (error: 0.00%)
  - 1.0 ~ mu^4 = 1.0000 (error: 0.00%)

### 46. gan-power-device / Si-200mm
- Level:  | n6 score: 0.25
- Label: Si 200mm (8-inch, lowest cost, thermal mismatch, 200V-650V)
- Matches:
  - 200.0 ~ (sigma+sopfr)*sigma = 204.0000 (error: 1.96%)
  - 200.0 ~ (sopfr+sigma)*sigma = 204.0000 (error: 1.96%)
  - 200.0 ~ (sigma+phi)^phi = 196.0000 (error: 2.00%)
  - 200.0 ~ (phi+sigma)^phi = 196.0000 (error: 2.00%)
  - 200.0 ~ (n+phi)*J2 = 192.0000 (error: 4.00%)

### 47. gan-power-device / Sapphire
- Level:  | n6 score: 0.25
- Label: Sapphire (Al2O3, transparent, LED-heritage, poor thermal k=35)
- Matches:
  - 35.0 ~ (n+sopfr)+J2 = 35.0000 (error: 0.00%)
  - 35.0 ~ (n*sopfr)+sopfr = 35.0000 (error: 0.00%)
  - 35.0 ~ (n+J2)+sopfr = 35.0000 (error: 0.00%)
  - 35.0 ~ (n+mu)*sopfr = 35.0000 (error: 0.00%)
  - 35.0 ~ (sigma-sopfr)*sopfr = 35.0000 (error: 0.00%)

### 48. grid / ACSR
- Level: Conductor | n6 score: 0.00
- Label: Aluminum Conductor Steel Reinforced
- Matches:
  - 5.0 ~ sopfr = 5.0000 (error: 0.00%)
  - 5.0 ~ n-mu = 5.0000 (error: 0.00%)
  - 5.0 ~ tau+mu = 5.0000 (error: 0.00%)
  - 5.0 ~ sopfr*mu = 5.0000 (error: 0.00%)
  - 5.0 ~ sopfr/mu = 5.0000 (error: 0.00%)

### 49. grid / HTLS
- Level: Conductor | n6 score: 0.00
- Label: High-Temperature Low-Sag Conductor
- Matches:
  - 3.0 ~ n/phi = 3.0000 (error: 0.00%)
  - 3.0 ~ sigma/tau = 3.0000 (error: 0.00%)
  - 3.0 ~ tau-mu = 3.0000 (error: 0.00%)
  - 3.0 ~ phi+mu = 3.0000 (error: 0.00%)
  - 3.0 ~ sopfr-phi = 3.0000 (error: 0.00%)

### 50. grid / XLPE_Sub
- Level: Conductor | n6 score: 0.00
- Label: XLPE Submarine Cable
- Matches:
  - 3.0 ~ n/phi = 3.0000 (error: 0.00%)
  - 3.0 ~ sigma/tau = 3.0000 (error: 0.00%)
  - 3.0 ~ tau-mu = 3.0000 (error: 0.00%)
  - 3.0 ~ phi+mu = 3.0000 (error: 0.00%)
  - 3.0 ~ sopfr-phi = 3.0000 (error: 0.00%)


## Genuinely Non-N6 Candidates (no formula match found)

Total: 63 candidates with no plausible n6 connection.

| # | Domain | Level | ID | n6 | Label (truncated) |
|---|--------|-------|----|----|-------------------|
| 1 | agent-platform | Provider | Local | 0.25 | Local Model (Llama/Mistral, self-hosted) |
| 2 | agent-platform | Bridge | None | 0.25 | No Bridge (standalone agent) |
| 3 | aluminum-alloy |  | O_Annealed | 0.40 | O annealed (full anneal, softest condition, maximum ductilit |
| 4 | aluminum-alloy |  | T7_Overaged | 0.40 | T7 overaged (beyond peak, stress corrosion resistant, 7xxx p |
| 5 | asic-design |  | DRC-LVS | 0.25 | DRC-LVS (design rule + layout vs schematic, clean tapeout ga |
| 6 | audio-processing | Encoding | MP3_320 | 0.25 | MP3 320kbps (legacy) |
| 7 | collective-intelligence | Scale | GlobalNetwork | 0.40 | Global Network (unbounded) |
| 8 | compiler-os | Kernel | Exokernel | 0.30 | Exokernel (minimal kernel, library OS, user-level scheduling |
| 9 | conscious-lm | ConsciousnessIntegration | Detach | 0.30 | 분리 (의식 없음) |
| 10 | consciousness-dream | Replay | None_Replay | 0.25 | 리플레이 없음 (기준선) |
| 11 | consciousness-rng | PostProcess | None | 0.25 | 후처리 없음 (raw entropy) |
| 12 | consciousness-rng | Compliance | None_Compliance | 0.25 | 인증 없음 (연구용) |
| 13 | consciousness-thermodynamics | Criticality | Subcritical | 0.25 | Subcritical (ordered phase) |
| 14 | consciousness-thermodynamics | Criticality | Supercritical | 0.25 | Supercritical (chaotic phase) |
| 15 | dram-memory | Stack | Monolithic | 0.40 | Monolithic Die (single layer, planar DRAM, no stacking) |
| 16 | dsp-processor |  | Reconfigurable | 0.25 | Reconfigurable (CGRA mesh, runtime-adaptable datapath) |
| 17 | dsp-processor |  | Saturate-arithmetic | 0.25 | Saturate Arithmetic (clamp on overflow, no wrap-around, guar |
| 18 | earthquake-engineering | Damping | NoDamping | 0.45 | No Added Damping (inherent structural only, budget) |
| 19 | earthquake-engineering | Monitoring | Visual-Inspect | 0.45 | Visual Inspection (post-earthquake, tagging, manual) |
| 20 | eeg-consciousness-bridge | Feedback | None | 0.40 | 피드백 없음 (관측만) |
| 21 | energy_gen | Scale | Utility_1GW | 0.33 | Utility Scale (1GW) |
| 22 | energy_gen | Storage | None | 0.00 | No Storage (Direct Feed) |
| 23 | energy_gen | GridConnect | Island_DC | 0.33 | Island DC (Off-Grid) |
| 24 | gan-power-device |  | Diode-SBD | 0.25 | GaN Schottky Barrier Diode (zero reverse recovery, freewheel |
| 25 | gan-power-device |  | Schottky-gate | 0.25 | Schottky Gate (Ni/Au, normally-on, simplest, RF standard) |
| 26 | golden-moe-routing | Strategy | Hash | 0.25 | Hash Routing (deterministic, no gate) |
| 27 | golden-moe-routing | Gate | Hard | 0.25 | Hard Gate (binary on/off) |
| 28 | golden-moe-routing | Capacity | Unlimited | 0.25 | Unlimited Capacity (no constraint) |
| 29 | golden-moe-routing | LoadBalance | None | 0.25 | No Load Balance |
| 30 | grid | System | Central_Radial | 0.00 | Central Radial Grid |
| 31 | hair-regeneration | Delivery | Topical | 0.25 | Topical Application (국소 도포) |
| 32 | hair-regeneration | Delivery | Oral | 0.25 | Oral Administration (경구) |
| 33 | hyperloop-transport | Levitation | Air-Bearing | 0.45 | Air Bearing Skid (compressed air film, low speed, simple) |
| 34 | learning-algorithm | Optimizer | Shampoo | 0.33 | Shampoo 2nd-Order |
| 35 | material |  | F1 | 0.40 | SingleStation |
| 36 | mems-sensor |  | Cantilever-beam | 0.25 | Cantilever Beam (simplest, AFM-heritage, pressure/bio/chemic |
| 37 | mems-sensor |  | Digital-direct | 0.25 | Digital Direct (frequency/time output, no ADC, oscillator-ba |
| 38 | mems-sensor |  | Pressure-sensor | 0.25 | Pressure Sensor (barometric/industrial, absolute/gauge/diffe |
| 39 | music-notation | Layout | MusicXML | 0.40 | MusicXML 표준 |
| 40 | nand-flash | Structure | Planar_2D | 0.40 | 2D Planar (single layer, legacy, smallest die) |

## Recommendations

### 1. Upgrade Candidates (undiscovered formulas found)
The 87 candidates with inverse matches should be reviewed.
If the n6 expression is genuine, upgrade their n6 score in the TOML file.

### 2. High-Anomaly Domains
These domains have >15% anomaly rate and may need TOML revision:
- **energy_gen**: 12/27 (44%)
- **wafer-fabrication**: 8/30 (27%)
- **chip**: 11/50 (22%)
- **solar**: 6/28 (21%)
- **grid**: 5/24 (21%)
- **risc-v-core**: 6/30 (20%)
- **compiler-os**: 5/27 (19%)

### 3. Genuinely Non-N6
The 63 candidates with no match are either:
- Material/physical constants with no n6 connection (expected)
- Parameters needing deeper (depth-3+) expression search
- Candidates that should be removed or replaced with n6-aligned alternatives

### 4. Average n6 Score
Overall average: 0.876
Non-anomaly average: 0.886
