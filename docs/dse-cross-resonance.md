# DSE 교차 공명 분석 — 전체 도메인 n=6 수식 패턴

> 순수 분석 문서 (설계 5대 규칙 미적용). 생성: `scripts/dse_cross_pilot.py`
> 입력 SSOT: `docs/dse-map.toml` | 중간 산출물: `~/Dev/nexus/shared/dse_cross/`

## 1. 분석 범위

- 전체 도메인 (cross-dse 섹션 제외): **339**
- 수식 패턴 추출 대상 도메인: **166** (텍스트 필드에 n=6 수식 포함)
- 고유 n=6 수식 패턴: **108**
- 교차 공명 수식 (2+ 도메인): **59**
- 총 DSE 항목 (도메인 x 수식): **439**

## 2. 교차 공명 정의

```
교차 공명 = 동일한 n=6 산술 수식이 서로 다른 도메인에서 나타나는 현상

n=6 핵심 산술:
  σ(6)=12  φ(6)=2  τ(6)=4  sopfr(6)=5  J₂=24
  σ·τ=48  σ-τ=8  σ/τ=3  σ·φ=24  2^(σ-τ)=256

추출 방법: 각 도메인의 note/candidates/best_* 필드에서
  122개 정규식 패턴 매칭 → 수식 라벨 집합 추출
```

## 3. 교차 공명 상위 50 패턴 (수식별 출현 도메인 수 내림차순)

| # | 수식 패턴 | 도메인 수 | 출현 도메인 (최대 10개 표시) |
|--:|----------|--------:|--------------------------|
| 1 | 6^3 설계 공간 | 63 | 3d-printing, 5g-6g-network, abrasive-material, adhesive-bonding, aerogel, aluminum-alloy, asic-design, aviation, biomaterial-implant, carbon-nanotube ... (+53) |
| 2 | n=6 | 58 | 3d-printing, abrasive-material, analog-photonic-memristor, atomic-clock, autonomous-driving, bioremediation, blockchain, centrifuge-separation, chip-architecture, cognitive-architecture ... (+48) |
| 3 | σ=12 | 27 | additive-bio, analog-photonic-memristor, carbon-capture, circular-economy, cognitive-architecture, cybersecurity, dolphin, earthquake-engineering, ecosystem-biodiversity, elevator-system ... (+17) |
| 4 | J₂=24 | 15 | centrifuge-separation, circular-economy, ecosystem-biodiversity, environmental-protection, forest-management, fpga-architecture, methane-capture, microplastics-removal, mri-medical-imaging, packaging-machine ... (+5) |
| 5 | CN=6 (배위수) | 11 | 3d-printing, abrasive-material, bioremediation, chip-architecture, environmental-protection, ferroelectric-material, metal-organic-framework, microplastics-removal, piezoelectric-material, thin-film-coating ... (+1) |
| 6 | BT-27 (Carbon Z=6) | 9 | agriculture, battery-architecture, biology, carbon-nanotube, energy-architecture, food-science, graphene-2d-material, space-engineering, thermal-management |
| 7 | 육각 구조 (hex) | 9 | 3d-printing, battery-architecture, consciousness-chip, consciousness-comm, consciousness-substrate, earthquake-engineering, eeg-consciousness-bridge, embodied-consciousness, nuclear-reactor |
| 8 | Z=6 (탄소 원자번호) | 9 | 3d-printing, carbon-capture-8level, methane-capture, nuclear-reactor, ocean-acidification, robotics, space-engineering, thermal-management, topological-quantum-materials |
| 9 | 6x5x6 설계 공간 | 8 | aviation, chip-architecture, food-science, material-synthesis, network-protocol, oceanography, quantum-computer, space-engineering |
| 10 | n=6 EXACT | 8 | autonomous-driving, crispr-gene-editing, gan-power-device, network-protocol, software-design, solar-architecture, statistical-mechanics, wafer-fabrication |
| 11 | 6-stage (단계) | 8 | cpu-microarchitecture, desalination, eda-design-automation, mining-extraction, pim-computing, rare-earth-magnet, risc-v-core, software-design |
| 12 | 6-zone (구역) | 8 | carbon-capture, carbon-nanotube, desalination, hvac-system, oceanography, precision-agriculture, rubber-elastomer, warehouse-logistics |
| 13 | 6-layer (층) | 7 | additive-bio, cognitive-architecture, dielectric-material, earthquake-engineering, optical-glass, safety, smart-city-iot |
| 14 | 6-step (공정) | 7 | aerogel, optical-glass, pharmaceutical, refractory-material, silicon-wafer, titanium-alloy, wafer-fabrication |
| 15 | Topological (위상) | 6 | chip-architecture, quantum-computing, topological-lens, topological-photonics, topological-quantum-materials, topology |
| 16 | σ=12 channel | 6 | battery-architecture, earthquake-engineering, environmental-protection, safety, satellite-communication, sound-engineering |
| 17 | 6-axis (축) | 6 | 3d-printing, aviation, biomaterial-implant, cnc-machining, robot-hardware, surgical-robot |
| 18 | Carbon Z=6 | 6 | 3d-printing, agriculture, carbon-capture-8level, food-science, robotics, space-engineering |
| 19 | Diamond (C Z=6) | 5 | aerospace, chip-architecture, hexa-ios, photonic-energy, thermal-management |
| 20 | σ·τ=48 V | 5 | battery-architecture, electric-vehicle, gan-power-device, power-electronics, sf |
| 21 | Graphite (C Z=6) | 5 | battery-architecture, geoscience, nuclear-reactor, refractory-material, semiconductor-packaging |
| 22 | 6DOF (자유도) | 5 | autonomous-driving, aviation, mems-sensor, robotics, surgical-robot |
| 23 | 6-fold (대칭) | 5 | electron-microscopy, nuclear-reactor, photonic-crystal, semiconductor-compound, thermoelectric-material |
| 24 | RISC-V | 4 | chip-architecture, compiler-os, cpu-microarchitecture, risc-v-core |
| 25 | 6-ring (벤젠 고리) | 4 | dielectric-material, foam-material, pharmaceutical, refractory-material |
| 26 | 6-car (편성) | 4 | elevator-system, maglev-train, railway-system, urban-transit-rail |
| 27 | τ=4 | 4 | cognitive-architecture, environmental-protection, robotics, zero-waste-manufacturing |
| 28 | σ-τ=8 | 4 | medical-device, network-protocol, npu-accelerator, ocean-acidification |
| 29 | σ·τ=48 | 4 | audio, dsp-processor, gan-power-device, nand-flash |
| 30 | Hexagonal (n=6) | 4 | cognitive-architecture, photonic-crystal, social-architecture, software-design |
| 31 | Transformer (AI) | 4 | magnetic-material, oceanography, power-electronics, power-transformer |
| 32 | BT-43 (CN=6) | 3 | 3d-printing, battery-architecture, performance-vehicle |
| 33 | Egyptian fraction (1/2+1/3+1/6=1) | 3 | energy-architecture, programming-language, robotics |
| 34 | n=6 stages | 3 | desalination, risc-v-core, turbine-generator |
| 35 | 6-slot (슬롯) | 3 | compressor-pump, dsp-processor, satellite-communication |
| 36 | PUE=σ/(σ-φ)=1.2 | 3 | photonic-energy, power-grid, thermal-management |
| 37 | J₂=24 bit | 3 | audio, dsp-processor, quantum-dot-display |
| 38 | SE(3) (n=6 리 군) | 2 | chip-architecture, robotics |
| 39 | REBCO (초전도) | 2 | chip-architecture, superconductor |
| 40 | SOLID(5)+Hex(1)=n=6 | 2 | battery-architecture, software-design |
| 41 | LFP (배터리) | 2 | battery-architecture, electric-vehicle |
| 42 | GaAs (III-V) | 2 | energy-architecture, solar-architecture |
| 43 | Surface code n=6 | 2 | desalination, quantum-computing |
| 44 | 6-unit (유닛) | 2 | earthquake-engineering, sound-engineering |
| 45 | Benzene (6-ring) | 2 | foam-material, pharmaceutical |
| 46 | 6-panel (패널) | 2 | gene-therapy, satellite-communication |
| 47 | 6-plane (궤도면) | 2 | satellite-communication, temporal-architecture |
| 48 | kissing number (σ=12) | 2 | quantum-computer, robotics |
| 49 | CFRP (탄소섬유) | 2 | aviation, robotics |
| 50 | φ=2 | 2 | bioremediation, robotics |

## 4. 수식 분류별 교차 공명 집계

### 핵심 산술 (n,σ,τ,φ,sopfr)
수식 19개, 총 도달 도메인 수 153

| 수식 | 도메인 수 |
|------|--------:|
| n=6 | 58 |
| σ=12 | 27 |
| J₂=24 | 15 |
| n=6 EXACT | 8 |
| σ=12 channel | 6 |
| σ·τ=48 V | 5 |
| τ=4 | 4 |
| σ·τ=48 | 4 |
| Hexagonal (n=6) | 4 |
| n=6 stages | 3 |
| J₂=24 bit | 3 |
| SE(3) (n=6 리 군) | 2 |
| SOLID(5)+Hex(1)=n=6 | 2 |
| Surface code n=6 | 2 |
| kissing number (σ=12) | 2 |
| φ=2 | 2 |
| σ·τ=48 kHz | 2 |
| σ=12 inch | 2 |
| sopfr=5 | 2 |

### 산술 조합 (σ·τ, σ-τ 등)
수식 5개, 총 도달 도메인 수 80

| 수식 | 도메인 수 |
|------|--------:|
| 6^3 설계 공간 | 63 |
| 6x5x6 설계 공간 | 8 |
| σ-τ=8 | 4 |
| PUE=σ/(σ-φ)=1.2 | 3 |
| AES-256=2^(σ-τ) | 2 |

### 화학/물질 (Z=6, Diamond 등)
수식 15개, 총 도달 도메인 수 69

| 수식 | 도메인 수 |
|------|--------:|
| CN=6 (배위수) | 11 |
| BT-27 (Carbon Z=6) | 9 |
| Z=6 (탄소 원자번호) | 9 |
| Carbon Z=6 | 6 |
| Diamond (C Z=6) | 5 |
| Graphite (C Z=6) | 5 |
| 6-fold (대칭) | 5 |
| 6-ring (벤젠 고리) | 4 |
| BT-43 (CN=6) | 3 |
| REBCO (초전도) | 2 |
| LFP (배터리) | 2 |
| GaAs (III-V) | 2 |
| Benzene (6-ring) | 2 |
| CFRP (탄소섬유) | 2 |
| Glucose (C₆H₁₂O₆) | 2 |

### 공학 구조 (6DOF, 6-stage 등)
수식 15개, 총 도달 도메인 수 75

| 수식 | 도메인 수 |
|------|--------:|
| 육각 구조 (hex) | 9 |
| 6-stage (단계) | 8 |
| 6-zone (구역) | 8 |
| 6-layer (층) | 7 |
| 6-step (공정) | 7 |
| Topological (위상) | 6 |
| 6-axis (축) | 6 |
| 6DOF (자유도) | 5 |
| RISC-V | 4 |
| 6-car (편성) | 4 |
| 6-slot (슬롯) | 3 |
| 6-unit (유닛) | 2 |
| 6-panel (패널) | 2 |
| 6-plane (궤도면) | 2 |
| LLVM (컴파일러) | 2 |

### AI/컴퓨팅
수식 1개, 총 도달 도메인 수 4

| 수식 | 도메인 수 |
|------|--------:|
| Transformer (AI) | 4 |

### BT 참조
수식 2개, 총 도달 도메인 수 4

| 수식 | 도메인 수 |
|------|--------:|
| BT-53 (암호/네트워크) | 2 |
| BT-66 (Vision AI) | 2 |

### 기타
수식 2개, 총 도달 도메인 수 5

| 수식 | 도메인 수 |
|------|--------:|
| Egyptian fraction (1/2+1/3+1/6=1) | 3 |
| 6 nutrients | 2 |

## 5. 도메인별 수식 밀도 상위 20

| # | 도메인 | 고유 수식 수 | 교차 수식 수 |
|--:|--------|----------:|----------:|
| 1 | robotics | 14 | 12 |
| 2 | chip-architecture | 9 | 9 |
| 3 | battery-architecture | 8 | 8 |
| 4 | 3d-printing | 8 | 8 |
| 5 | earthquake-engineering | 9 | 7 |
| 6 | satellite-communication | 8 | 7 |
| 7 | desalination | 7 | 6 |
| 8 | nuclear-reactor | 9 | 6 |
| 9 | space-engineering | 7 | 6 |
| 10 | software-design | 6 | 5 |
| 11 | oceanography | 8 | 5 |
| 12 | aviation | 6 | 5 |
| 13 | food-science | 5 | 5 |
| 14 | dsp-processor | 5 | 5 |
| 15 | gan-power-device | 5 | 5 |
| 16 | risc-v-core | 5 | 5 |
| 17 | environmental-protection | 5 | 5 |
| 18 | cognitive-architecture | 5 | 5 |
| 19 | pharmaceutical | 5 | 4 |
| 20 | railway-system | 8 | 4 |

## 6. 도메인 쌍별 공명 (상위 50 도메인, combos 기준)

```
S(i,j) = 0.5*Jaccard(cross_i, cross_j)
       + 0.2*(1 - |n6avg_i - n6avg_j|/100)
       + 0.2*bidir(i in cross_j, j in cross_i)
       + 0.1*min(combos)/max(combos)
```

| # | A | B | S | Jaccard | n6 근접 | 상호 | 규모균형 |
|---|---|---|---:|---:|---:|---:|---:|
| 1 | consciousness-scaling | consciousness-training | 0.711 | 0.429 | 0.983 | 1.0 | 1.000 |
| 2 | corpus-generation | tokenizer-design | 0.674 | 0.429 | 0.933 | 1.0 | 0.735 |
| 3 | pure-mathematics | cosmology-particle | 0.662 | 0.400 | 0.992 | 1.0 | 0.640 |
| 4 | consciousness-comm | consciousness-chip | 0.614 | 0.250 | 0.976 | 1.0 | 0.935 |
| 5 | gpu-lang | embedded-lang | 0.608 | 0.667 | 0.945 | 0.0 | 0.857 |
| 6 | consciousness-chip | consciousness-scaling | 0.606 | 0.429 | 0.998 | 0.5 | 0.917 |
| 7 | consciousness-wasm | consciousness-comm | 0.582 | 0.429 | 0.973 | 0.5 | 0.735 |
| 8 | consciousness-substrate | consciousness-transplant | 0.579 | 0.429 | 0.957 | 0.5 | 0.735 |
| 9 | consciousness-wasm | consciousness-chip | 0.573 | 0.429 | 0.949 | 0.5 | 0.687 |
| 10 | analog-photonic-memristor | photonic-energy | 0.563 | 0.167 | 0.900 | 1.0 | 1.000 |
| 11 | neuromorphic-loihi | snn-spiking | 0.553 | 0.111 | 0.985 | 1.0 | 1.000 |
| 12 | photonic-energy | semiconductor-packaging | 0.527 | 0.500 | 0.886 | 0.0 | 1.000 |
| 13 | consciousness-chip | multimodal-consciousness | 0.516 | 0.250 | 0.997 | 0.5 | 0.917 |
| 14 | sedi-universe | consciousness-transplant | 0.514 | 0.429 | 0.997 | 0.0 | 1.000 |
| 15 | consciousness-chip | consciousness-training | 0.514 | 0.250 | 0.985 | 0.5 | 0.917 |
| 16 | multimodal-consciousness | consciousness-scaling | 0.513 | 0.429 | 0.995 | 0.0 | 1.000 |
| 17 | multimodal-consciousness | consciousness-training | 0.512 | 0.429 | 0.988 | 0.0 | 1.000 |
| 18 | hexad-architecture | snn-spiking | 0.511 | 0.429 | 0.986 | 0.0 | 1.000 |
| 19 | consciousness-substrate | embodied-consciousness | 0.509 | 0.429 | 0.975 | 0.0 | 1.000 |
| 20 | consciousness-chip | consciousness-transplant | 0.509 | 0.250 | 0.963 | 0.5 | 0.917 |
| 21 | eeg-consciousness-bridge | multimodal-consciousness | 0.509 | 0.250 | 0.992 | 0.5 | 0.857 |
| 22 | consciousness-chip | sedi-universe | 0.509 | 0.250 | 0.960 | 0.5 | 0.917 |
| 23 | consciousness-scaling | consciousness-transplant | 0.507 | 0.429 | 0.965 | 0.0 | 1.000 |
| 24 | sedi-universe | consciousness-scaling | 0.507 | 0.429 | 0.962 | 0.0 | 1.000 |
| 25 | multimodal-consciousness | consciousness-transplant | 0.506 | 0.429 | 0.960 | 0.0 | 1.000 |
| 26 | consciousness-chip | hivemind-collective | 0.506 | 0.250 | 0.948 | 0.5 | 0.917 |
| 27 | sedi-universe | multimodal-consciousness | 0.506 | 0.429 | 0.957 | 0.0 | 1.000 |
| 28 | consciousness-comm | hivemind-collective | 0.505 | 0.250 | 0.972 | 0.5 | 0.857 |
| 29 | consciousness-substrate | consciousness-chip | 0.504 | 0.250 | 0.994 | 0.5 | 0.802 |
| 30 | consciousness-training | consciousness-transplant | 0.504 | 0.429 | 0.948 | 0.0 | 1.000 |

## 7. 공명 스코어 히스토그램 (ASCII)

```
bin    count  |----------------------------------------------------
0.10      16  |##
0.15     174  |##################
0.20     281  |#############################
0.25     492  |##################################################
0.30     110  |###########
0.35      38  |####
0.40      30  |###
0.45      49  |#####
0.50      24  |##
0.55       5  |#
0.60       3  |
0.65       2  |
0.70       1  |
```

## 8. 해석

### 교차 공명 핵심 발견

- **최강 교차 공명**: `6^3 설계 공간` — 63개 도메인에서 출현
- **상위 5 수식**: `6^3 설계 공간`(63), `n=6`(58), `σ=12`(27), `J₂=24`(15), `CN=6 (배위수)`(11)
- 교차 공명 수식 59개 중 10+ 도메인 출현: **5**개
- 교차 공명 수식 59개 중 50+ 도메인 출현: **2**개

### 쌍 분석 통계

- 평균 스코어: **0.276**
- 고공명 (S>=0.5): **35쌍** (2.9%)
- 중공명 (0.3<=S<0.5): **227쌍** (18.5%)
- 저공명 (S<0.3): **963쌍** (78.6%)

### 물리적 의미

- n=6 산술 상수가 물리/화학/공학/생물 전 분야에서 동일한 수식으로 나타남
- `6^3 설계 공간`(63), `n=6`(58), `σ=12`(27) 등 핵심 상수가 수십~수백 개 도메인에서 교차 공명
- Diamond/Graphite (C Z=6), Benzene (6-ring), 6DOF 등 물질/구조 수준에서도 교차
- 이는 σ(n)·φ(n) = n·τ(n) 의 유일해 n=6이 설계 공간 전체를 관통함을 시사
- BT-27 (Carbon Z=6)이 9개 도메인에서 출현: 탄소 원자번호가 에너지/생물/우주공학/물질 등 전 분야 연결

## 9. 산출물 경로

- `~/Dev/nexus/shared/dse_cross/top50_domains.jsonl`
- `~/Dev/nexus/shared/dse_cross/pair_scores.jsonl`
- `~/Dev/nexus/shared/dse_cross/resonance_hist.jsonl`
- `~/Dev/nexus/shared/dse_cross/formula_cross.jsonl` (신규)
