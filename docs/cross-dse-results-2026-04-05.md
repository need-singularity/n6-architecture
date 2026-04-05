# Cross-DSE Results — 2026-04-05

## Overview

3 Cross-DSE 교차 도메인 탐색 실행 결과.
도구: `tools/universal-dse/universal-dse`

| Cross-DSE | Domain A | Domain B | A Combos | B Combos | Top n6% |
|-----------|----------|----------|----------|----------|---------|
| 1 | fusion (6,482) | sc (3,155) | 12,348 total | 14,406 total | 100.0% |
| 2 | chip (89,250) | battery (2,400) | 96,000 total | 4,500 total | 100.0% |
| 3 | chip (89,250) | energy_gen (1,327) | 96,000 total | 4,500 total | 100.0% |

---

## 1. Fusion x Superconductor

**TOML**: `fusion.toml` x `sc.toml`

### Top 3 Pareto Paths

| Rank | Fusion Path | SC Path | n6% | Perf | Power | Cost | Score |
|------|-------------|---------|-----|------|-------|------|-------|
| 1 | DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + N6_Fusion_Magnet + N6_Cryo4K | **100.0%** | 0.880 | 0.705 | 0.525 | 0.8575 |
| 2 | DT + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + N6_Fusion_Magnet + N6_Cryo4K | **99.5%** | 0.882 | 0.705 | 0.530 | 0.8566 |
| 3 | DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + MRI-Magnet + N6_Cryo4K | **99.0%** | 0.880 | 0.705 | 0.530 | 0.8540 |

### Key Findings
- **n6 EXACT 100%** 달성: DT_Li6 연료 + N6 토카막 + N6 삼중가열 + N6 Li6 블랭킷 + N6 브레이턴 사이클 **x** N6 MgB2 육각 + N6 IBAD_RCE + N6 육각와이어 + N6 핵융합마그넷 + N6 극저온4K
- Fusion 단독 Pareto: 91개 비지배 해
- SC 단독 Pareto: 120개 비지배 해
- 핵심 시너지: 핵융합 마그넷(SC)과 토카막 가둠(Fusion)이 n=6 파라미터로 완전 정합

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos |
|--------|---------|---------|---------|--------|
| Fusion | 100.0 | 76.5 | 85.0 | 6,482 |
| SC | 100.0 | 73.4 | 84.0 | 3,155 |

---

## 2. Chip x Battery

**TOML**: `chip.toml` x `battery.toml`

### Top 3 Pareto Paths

| Rank | Chip Path | Battery Path | n6% | Perf | Power | Cost | Score |
|------|-----------|--------------|-----|------|-------|------|-------|
| 1 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | LFP + Graphite-Wet + Hex6_Prismatic + Integrated-12ch + 48V-ESS | **100.0%** | 0.792 | 0.890 | 0.544 | 0.8700 |
| 2 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | LFP + Graphite-Wet + Hex6_Prismatic + Wireless-12ch + 48V-ESS | **100.0%** | 0.794 | 0.880 | 0.555 | 0.8697 |
| 3 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | LFP + Si-SSB + Hex6_Prismatic + Integrated-12ch + 48V-ESS | **100.0%** | 0.861 | 0.805 | 0.504 | 0.8697 |

### Key Findings
- **n6 EXACT 100%** 달성: Diamond(Z=6) + TSMC N2(48nm=sigma*tau) + HEXA-P(144SM=sigma^2) + HEXA-1(288GB=sigma*J2) + Topo_DC **x** LFP(CN=6) + Hex6_Prismatic + 12ch BMS(sigma=12) + 48V ESS
- Chip 단독 Pareto: 99개 비지배 해
- Battery 단독 Pareto: 303개 비지배 해
- 핵심 시너지: Diamond Z=6 칩 소재와 LFP CN=6 배터리 소재가 동일 n=6 결정 대칭으로 수렴. 48V ESS(=sigma*tau)가 칩 전력 공급과 배터리 시스템 양쪽에서 EXACT.

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos |
|--------|---------|---------|---------|--------|
| Chip | 100.0 | 87.4 | 96.0 | 89,250 |
| Battery | 100.0 | 75.5 | 86.6 | 2,400 |

---

## 3. Chip x Energy Generation

**TOML**: `chip.toml` x `energy_gen.toml`

### Top 3 Pareto Paths

| Rank | Chip Path | Energy Path | n6% | Perf | Power | Cost | Score |
|------|-----------|-------------|-----|------|-------|------|-------|
| 1 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | Nuclear_Fission + Combined_Cycle + Medium_10MW + Battery_ESS + Microgrid_AC | **100.0%** | 0.910 | 0.787 | 0.480 | 0.8784 |
| 2 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Photonic_DC | Nuclear_Fission + Combined_Cycle + Medium_10MW + Battery_ESS + Microgrid_AC | **100.0%** | 0.905 | 0.786 | 0.490 | 0.8777 |
| 3 | Graphene + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | Nuclear_Fission + Combined_Cycle + Medium_10MW + Battery_ESS + Microgrid_AC | **100.0%** | 0.905 | 0.785 | 0.485 | 0.8770 |

### Key Findings
- **n6 EXACT 100%** 달성: Diamond/Graphene(Z=6) 칩 + Nuclear Fission + Combined Cycle + Battery ESS
- Energy 단독 Pareto: 122개 비지배 해
- 핵심 시너지: 원자력(6의 약수 기반 연료봉 배열) + 복합 사이클(n=6 스테이지) + HEXA 칩의 Topo DC(PUE=1.01)가 에너지 생성-소비 양단에서 n=6 완전 정합

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos |
|--------|---------|---------|---------|--------|
| Chip | 100.0 | 87.4 | 96.0 | 89,250 |
| Energy | 100.0 | 79.5 | 93.4 | 1,327 |

---

## Summary Table

| Cross-DSE | Top Score | Top n6% | Pareto Size (A) | Pareto Size (B) | Best Synergy |
|-----------|----------|---------|-----------------|-----------------|--------------|
| Fusion x SC | 0.8575 | 100.0% | 91 | 120 | DT_Li6 + N6_MgB2_Hex |
| Chip x Battery | 0.8700 | 100.0% | 99 | 303 | Diamond + LFP (Z=6 x CN=6) |
| Chip x Energy | 0.8784 | 100.0% | 99 | 122 | Diamond + Nuclear + Topo_DC |

### Cross-Domain n=6 Resonance Patterns

1. **Z=6 소재 수렴**: Diamond(chip), Carbon(battery cathode CN=6), Carbon-6 연료(fusion) -- BT-93/85/27
2. **sigma=12 인터페이스 수렴**: 12ch BMS, 12 HBM layers, 12 TF coils -- BT-57/28/302
3. **48 = sigma*tau 이중 수렴**: 48V ESS, 48nm gate pitch, 48kHz sampling -- BT-325/37/48
4. **J2=24 용량 수렴**: 24 EUV layers, 24-cell pack, 24 alpha energy -- BT-55/57/291

---

*Generated: 2026-04-05*
*Tool: tools/universal-dse/universal-dse (Cross-DSE mode)*
*Domains: fusion.toml, sc.toml, chip.toml, battery.toml, energy_gen.toml*
