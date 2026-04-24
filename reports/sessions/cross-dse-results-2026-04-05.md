# Cross-DSE Results — 2026-04-05

## Overview

3 Cross-DSE .
: `tools/universal-dse/universal-dse`

| Cross-DSE | Domain A | Domain B | A Combos | B Combos | Top n6% | |-----------|----------|----------|----------|----------|---------| | 1 | fusion (6,482) | sc (3,155) | 12,348 total | 14,406 total | 100.0% | | 2 | chip (89,250) | battery (2,400) | 96,000 total | 4,500 total | 100.0% | | 3 | chip (89,250) | energy_gen (1,327) | 96,000 total | 4,500 total | 100.0% | | 4 | sc (3,155) | plasma-physics (10,157) | 14,406 total | 14,406 total | 100.0% | | 5 | fusion (6,482) | battery (2,400) | 12,348 total | 4,500 total | 100.0% | | 6 | chip (89,250) | solar (1,624) | 96,000 total | 5,400 total | 100.0% |

---

## 1. Fusion x Superconductor

**TOML**: `fusion.toml` x `sc.toml`

### Top 3 Pareto Paths

| Rank | Fusion Path | SC Path | n6% | Perf | Power | Cost | Score | |------|-------------|---------|-----|------|-------|------|-------| | 1 | DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + N6_Fusion_Magnet + N6_Cryo4K | **100.0%** | 0.880 | 0.705 | 0.525 | 0.8575 | | 2 | DT + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + N6_Fusion_Magnet + N6_Cryo4K | **99.5%** | 0.882 | 0.705 | 0.530 | 0.8566 | | 3 | DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + MRI-Magnet + N6_Cryo4K | **99.0%** | 0.880 | 0.705 | 0.530 | 0.8540 |

### Key Findings
- **n6 EXACT 100%** : DT_Li6 + N6 + N6 + N6 Li6 + N6 **x** N6 MgB2 + N6 IBAD_RCE + N6 + N6 + N6 4K
- Fusion Pareto: 91 
- SC Pareto: 120 
- : (SC) (Fusion) n=6 

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos | |--------|---------|---------|---------|--------| | Fusion | 100.0 | 76.5 | 85.0 | 6,482 | | SC | 100.0 | 73.4 | 84.0 | 3,155 |

---

## 2. Chip x Battery

**TOML**: `chip.toml` x `battery.toml`

### Top 3 Pareto Paths

| Rank | Chip Path | Battery Path | n6% | Perf | Power | Cost | Score | |------|-----------|--------------|-----|------|-------|------|-------| | 1 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | LFP + Graphite-Wet + Hex6_Prismatic + Integrated-12ch + 48V-ESS | **100.0%** | 0.792 | 0.890 | 0.544 | 0.8700 | | 2 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | LFP + Graphite-Wet + Hex6_Prismatic + Wireless-12ch + 48V-ESS | **100.0%** | 0.794 | 0.880 | 0.555 | 0.8697 | | 3 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | LFP + Si-SSB + Hex6_Prismatic + Integrated-12ch + 48V-ESS | **100.0%** | 0.861 | 0.805 | 0.504 | 0.8697 |

### Key Findings
- **n6 EXACT 100%** : Diamond(Z=6) + TSMC N2(48nm=sigma*tau) + HEXA-P(144SM=sigma^2) + HEXA-1(288GB=sigma*J2) + Topo_DC **x** LFP(CN=6) + Hex6_Prismatic + 12ch BMS(sigma=12) + 48V ESS
- Chip Pareto: 99 
- Battery Pareto: 303 
- : Diamond Z=6 LFP CN=6 n=6 . 48V ESS(=sigma*tau) EXACT.

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos | |--------|---------|---------|---------|--------| | Chip | 100.0 | 87.4 | 96.0 | 89,250 | | Battery | 100.0 | 75.5 | 86.6 | 2,400 |

---

## 3. Chip x Energy Generation

**TOML**: `chip.toml` x `energy_gen.toml`

### Top 3 Pareto Paths

| Rank | Chip Path | Energy Path | n6% | Perf | Power | Cost | Score | |------|-----------|-------------|-----|------|-------|------|-------| | 1 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | Nuclear_Fission + Combined_Cycle + Medium_10MW + Battery_ESS + Microgrid_AC | **100.0%** | 0.910 | 0.787 | 0.480 | 0.8784 | | 2 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Photonic_DC | Nuclear_Fission + Combined_Cycle + Medium_10MW + Battery_ESS + Microgrid_AC | **100.0%** | 0.905 | 0.786 | 0.490 | 0.8777 | | 3 | Graphene + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | Nuclear_Fission + Combined_Cycle + Medium_10MW + Battery_ESS + Microgrid_AC | **100.0%** | 0.905 | 0.785 | 0.485 | 0.8770 |

### Key Findings
- **n6 EXACT 100%** : Diamond/Graphene(Z=6) + Nuclear Fission + Combined Cycle + Battery ESS
- Energy Pareto: 122 
- : (6 ) + (n=6 ) + HEXA Topo DC(PUE=1.01) - n=6 

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos | |--------|---------|---------|---------|--------| | Chip | 100.0 | 87.4 | 96.0 | 89,250 | | Energy | 100.0 | 79.5 | 93.4 | 1,327 |

---

## 4. SC x Plasma Physics

**TOML**: `sc.toml` x `plasma-physics.toml`

### Top 3 Pareto Paths

| Rank | SC Path | Plasma Path | n6% | Perf | Power | Cost | Score | |------|---------|-------------|-----|------|-------|------|-------| | 1 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + N6_Fusion_Magnet + N6_Cryo4K | N6_Tokamak + N6_SuperH + N6_TriHeat + N6_DivControl + N6_DEMO | **100.0%** | 0.884 | 0.635 | 0.500 | 0.8422 | | 2 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + N6_Fusion_Magnet + N6_Cryo4K | Tokamak + N6_SuperH + N6_TriHeat + N6_DivControl + N6_DEMO | **99.5%** | 0.884 | 0.635 | 0.500 | 0.8402 | | 3 | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + N6_Fusion_Magnet + N6_Cryo4K | N6_Tokamak + H_mode + N6_TriHeat + N6_DivControl + N6_DEMO | **99.0%** | 0.882 | 0.640 | 0.510 | 0.8396 |

### Key Findings
- **n6 EXACT 100%** : N6_MgB2 + N6 
- SC Pareto: 120 , Plasma Pareto: 76 
- : MgB2(phi=2 ) N6 (TF=3n=18) n=6 . Super H-mode(H=phi=2, beta_N=n/phi=3) + N6 - EXACT.

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos | |--------|---------|---------|---------|--------| | SC | 100.0 | 73.4 | 84.0 | 3,155 | | Plasma | 100.0 | 76.9 | 86.0 | 10,157 |

---

## 5. Fusion x Battery

**TOML**: `fusion.toml` x `battery.toml`

### Top 3 Pareto Paths

| Rank | Fusion Path | Battery Path | n6% | Perf | Power | Cost | Score | |------|-------------|--------------|-----|------|-------|------|-------| | 1 | DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | LFP + Graphite-Wet + Hex6_Prismatic + Integrated-12ch + 48V-ESS | **100.0%** | 0.777 | 0.833 | 0.569 | 0.8566 | | 2 | DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | LFP + Graphite-Wet + Hex6_Prismatic + Wireless-12ch + 48V-ESS | **100.0%** | 0.779 | 0.823 | 0.580 | 0.8563 | | 3 | DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 | LFP + Si-SSB + Hex6_Prismatic + Integrated-12ch + 48V-ESS | **100.0%** | 0.846 | 0.748 | 0.529 | 0.8563 |

### Key Findings
- **n6 EXACT 100%** : DT_Li6 + LFP(CN=6) 
- Fusion Pareto: 91, Battery Pareto: 303
- : Li-6 (A=6=n) LiFePO4(CN=6) -6 . + 48V(=sigma*tau) ESS . N6 (eta=sigma/J2=50%) Hex6 .

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos | |--------|---------|---------|---------|--------| | Fusion | 100.0 | 76.5 | 85.0 | 6,482 | | Battery | 100.0 | 75.5 | 86.6 | 2,400 |

---

## 6. Chip x Solar

**TOML**: `chip.toml` x `solar.toml`

### Top 3 Pareto Paths

| Rank | Chip Path | Solar Path | n6% | Perf | Power | Cost | Score | |------|-----------|------------|-----|------|-------|------|-------| | 1 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | GaAs + HJT + N6_Tandem_6J + DC-Optimizer + HC-120 | **100.0%** | 0.951 | 0.855 | 0.415 | 0.8978 | | 2 | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | GaAs + PERC + N6_Tandem_6J + DC-Optimizer + HC-120 | **100.0%** | 0.946 | 0.854 | 0.425 | 0.8971 | | 3 | Graphene + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC | GaAs + HJT + N6_Tandem_6J + DC-Optimizer + HC-120 | **100.0%** | 0.946 | 0.853 | 0.420 | 0.8964 |

### Key Findings
- **n6 EXACT 100%** : Diamond(Z=6) + GaAs 6 
- Chip Pareto: 99, Solar Pareto: 67
- **Cross-DSE **: 0.8978 (6 1)
- : Diamond(Z=6) + GaAs 6 (n=6 EXACT) + HC-120(=sigma*sopfr) -AI . Topo_DC(PUE=1.01) DC .

### Domain Statistics
| Domain | n6% max | n6% avg | n6% p90 | Combos | |--------|---------|---------|---------|--------| | Chip | 100.0 | 87.4 | 96.0 | 89,250 | | Solar | 100.0 | 78.3 | 88.0 | 1,624 |

---

## Summary Table

| Cross-DSE | Top Score | Top n6% | Pareto Size (A) | Pareto Size (B) | Best Synergy | |-----------|----------|---------|-----------------|-----------------|--------------| | Fusion x SC | 0.8575 | 100.0% | 91 | 120 | DT_Li6 + N6_MgB2_Hex | | Chip x Battery | 0.8700 | 100.0% | 99 | 303 | Diamond + LFP (Z=6 x CN=6) | | Chip x Energy | 0.8784 | 100.0% | 99 | 122 | Diamond + Nuclear + Topo_DC | | SC x Plasma | 0.8422 | 100.0% | 120 | 76 | N6_MgB2_Hex + N6_Tokamak | | Fusion x Battery | 0.8566 | 100.0% | 91 | 303 | DT_Li6 + LFP (Li-6 x CN=6) | | Chip x Solar | 0.8978 | 100.0% | 99 | 67 | Diamond + GaAs 6J Tandem |

### Cross-Domain n=6 Resonance Patterns

1. **Z=6 **: Diamond(chip), Carbon(battery cathode CN=6), Carbon-6 (fusion) -- BT-93/85/27
2. **sigma=12 **: 12ch BMS, 12 HBM layers, 12 TF coils -- BT-57/28/302
3. **48 = sigma*tau **: 48V ESS, 48nm gate pitch, 48kHz sampling -- BT-325/37/48
4. **J2=24 **: 24 EUV layers, 24-cell pack, 24 alpha energy -- BT-55/57/291
5. **Li-6 - **: Li-6(A=6) + LFP (CN=6) -- BT-296/43
6. **MgB2-Tokamak **: phi=2 + TF=3n=18 -- BT-301/302
7. **GaAs 6J + Diamond Z=6**: 6(n=6 EXACT) + (Z=6) - -- BT-30/93

---

*Generated: 2026-04-05*
*Tool: tools/universal-dse/universal-dse (Cross-DSE mode)*
*Domains: fusion.toml, sc.toml, chip.toml, battery.toml, energy_gen.toml, plasma-physics.toml, solar.toml*
