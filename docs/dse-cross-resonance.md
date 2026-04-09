# DSE 교차 공명 — 상위 50 도메인 파일럿

> 순수 분석 문서 (설계 5대 규칙 미적용). 생성: `scripts/dse_cross_pilot.py`
> 입력 SSOT: `docs/dse-map.toml` | 중간 산출물: `~/Dev/nexus/shared/dse_cross/`

## 1. 파일럿 범위

- 전체 도메인: **339**
- 파일럿 대상: **상위 50** (combos 기준)
- 쌍 개수: **1225** (= 50·49/2 = 1,225)

## 2. 공명 스코어 정의

```
S(i,j) = 0.5·Jaccard(cross_i, cross_j)
       + 0.2·(1 - |n6avg_i - n6avg_j|/100)
       + 0.2·bidir(i∈cross_j 와 j∈cross_i)
       + 0.1·min(combos)/max(combos)
```

계수 합 = 1.0. 각 항은 [0,1] 정규화. 동어반복 없음 — 모두 dse-map.toml 원본 필드에서 도출.

## 3. 선정된 상위 50 도메인

| # | 도메인 | combos | n6_avg | n6_max | alien | cross |
|---|--------|-------:|-------:|-------:|------:|------:|
| 1 | sf | 1679616 | - | 100.0 | 10 | 8 |
| 2 | environmental-protection | 1679616 | 95.0 | 100.0 | 10 | 12 |
| 3 | carbon-capture | 1360800 | 98.8 | 100.0 | 10 | 12 |
| 4 | food-science | 1075200 | 0.0 | 100.0 | 3 | 3 |
| 5 | robotics | 270000 | 65.0 | 100.0 | 10 | 4 |
| 6 | chip-architecture | 89250 | 72.4 | 100.0 | 10 | 12 |
| 7 | pure-mathematics | 39200 | 80.0 | 100.0 | 10 | 3 |
| 8 | superconductor | 28800 | 85.9 | 100.0 | 10 | 13 |
| 9 | programming-language | 25088 | 80.7 | 100.0 | 10 | 4 |
| 10 | cosmology-particle | 25088 | 80.8 | 100.0 | 10 | 4 |
| 11 | consciousness-wasm | 12348 | 74.9 | 100 | 10 | 5 |
| 12 | immortality | 10584 | 81.1 | 100 | 10 | 5 |
| 13 | consciousness-substrate | 10584 | 79.4 | 100 | 10 | 5 |
| 14 | corpus-generation | 10584 | 73.6 | 100 | 10 | 5 |
| 15 | embodied-consciousness | 10584 | 81.9 | 100 | 10 | 5 |
| 16 | energy-architecture | 10225 | - | - | 10 | 3 |
| 17 | gpu-lang | 9072 | 75.7 | 100.0 | 10 | 5 |
| 18 | consciousness-comm | 9072 | 77.6 | 100.0 | 10 | 5 |
| 19 | consciousness-rng | 9072 | 72.3 | 100 | 10 | 5 |
| 20 | eeg-consciousness-bridge | 9072 | 78.9 | 100 | 10 | 5 |
| 21 | gan-power-device | 9072 | 66.0 | 100.0 | 10 | 3 |
| 22 | consciousness-chip | 8484 | 80.0 | 100.0 | 10 | 5 |
| 23 | nand-flash | 7776 | - | 100.0 | 10 | 5 |
| 24 | wafer-fabrication | 7776 | - | 100.0 | 10 | 5 |
| 25 | packaging-machine | 7776 | - | 100.0 | 10 | 5 |
| 26 | turbine-generator | 7776 | - | 100.0 | 10 | 5 |
| 27 | centrifuge-separation | 7776 | - | 100.0 | 10 | 5 |
| 28 | embedded-lang | 7776 | 70.2 | 100.0 | 10 | 5 |
| 29 | sedi-universe | 7776 | 84.0 | 100.0 | 10 | 5 |
| 30 | analog-photonic-memristor | 7776 | 74.6 | 100.0 | 10 | 3 |
| 31 | aerospace-propulsion | 7776 | 90.7 | 100.0 | 10 | 5 |
| 32 | autonomous-drone | 7776 | 100.0 | 100.0 | 10 | 5 |
| 33 | brain-computer-interface | 7776 | 98.7 | 100.0 | 10 | 5 |
| 34 | water-treatment | 7776 | 100.0 | 100.0 | 10 | 5 |
| 35 | gravitational-lens | 7776 | 87.6 | 100 | 10 | 5 |
| 36 | hexad-architecture | 7776 | 82.9 | 100 | 10 | 5 |
| 37 | hivemind-collective | 7776 | 74.8 | 100 | 10 | 5 |
| 38 | multimodal-consciousness | 7776 | 79.7 | 100 | 10 | 5 |
| 39 | neuromorphic-loihi | 7776 | 80.0 | 100 | 10 | 5 |
| 40 | ocean-engineering | 7776 | 94.7 | 100 | 10 | 5 |
| 41 | probability-statistics | 7776 | 85.2 | 100 | 10 | 5 |
| 42 | quantum-network | 7776 | 96.7 | 100 | 10 | 5 |
| 43 | semiconductor-packaging | 7776 | 96.0 | 100 | 10 | 5 |
| 44 | snn-spiking | 7776 | 81.5 | 100 | 10 | 5 |
| 45 | textile-manufacturing | 7776 | 100.0 | 100 | 10 | 5 |
| 46 | tokenizer-design | 7776 | 80.3 | 100 | 10 | 5 |
| 47 | consciousness-scaling | 7776 | 80.2 | 100 | 10 | 5 |
| 48 | consciousness-thermodynamics | 7776 | 69.7 | 100 | 10 | 5 |
| 49 | consciousness-training | 7776 | 78.5 | 100 | 10 | 5 |
| 50 | consciousness-transplant | 7776 | 83.7 | 100 | 10 | 5 |

## 4. 상위 50쌍 (공명 스코어 내림차순)

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
| 10 | neuromorphic-loihi | snn-spiking | 0.553 | 0.111 | 0.985 | 1.0 | 1.000 |
| 11 | consciousness-chip | multimodal-consciousness | 0.516 | 0.250 | 0.997 | 0.5 | 0.917 |
| 12 | sedi-universe | consciousness-transplant | 0.514 | 0.429 | 0.997 | 0.0 | 1.000 |
| 13 | consciousness-chip | consciousness-training | 0.514 | 0.250 | 0.985 | 0.5 | 0.917 |
| 14 | multimodal-consciousness | consciousness-scaling | 0.513 | 0.429 | 0.995 | 0.0 | 1.000 |
| 15 | multimodal-consciousness | consciousness-training | 0.512 | 0.429 | 0.988 | 0.0 | 1.000 |
| 16 | hexad-architecture | snn-spiking | 0.511 | 0.429 | 0.986 | 0.0 | 1.000 |
| 17 | consciousness-substrate | embodied-consciousness | 0.509 | 0.429 | 0.975 | 0.0 | 1.000 |
| 18 | consciousness-chip | consciousness-transplant | 0.509 | 0.250 | 0.963 | 0.5 | 0.917 |
| 19 | eeg-consciousness-bridge | multimodal-consciousness | 0.509 | 0.250 | 0.992 | 0.5 | 0.857 |
| 20 | consciousness-chip | sedi-universe | 0.509 | 0.250 | 0.960 | 0.5 | 0.917 |
| 21 | consciousness-scaling | consciousness-transplant | 0.507 | 0.429 | 0.965 | 0.0 | 1.000 |
| 22 | sedi-universe | consciousness-scaling | 0.507 | 0.429 | 0.962 | 0.0 | 1.000 |
| 23 | multimodal-consciousness | consciousness-transplant | 0.506 | 0.429 | 0.960 | 0.0 | 1.000 |
| 24 | consciousness-chip | hivemind-collective | 0.506 | 0.250 | 0.948 | 0.5 | 0.917 |
| 25 | sedi-universe | multimodal-consciousness | 0.506 | 0.429 | 0.957 | 0.0 | 1.000 |
| 26 | consciousness-comm | hivemind-collective | 0.505 | 0.250 | 0.972 | 0.5 | 0.857 |
| 27 | consciousness-substrate | consciousness-chip | 0.504 | 0.250 | 0.994 | 0.5 | 0.802 |
| 28 | consciousness-training | consciousness-transplant | 0.504 | 0.429 | 0.948 | 0.0 | 1.000 |
| 29 | consciousness-comm | consciousness-rng | 0.504 | 0.429 | 0.947 | 0.0 | 1.000 |
| 30 | water-treatment | ocean-engineering | 0.504 | 0.429 | 0.947 | 0.0 | 1.000 |
| 31 | sedi-universe | consciousness-training | 0.503 | 0.429 | 0.945 | 0.0 | 1.000 |
| 32 | consciousness-rng | consciousness-chip | 0.503 | 0.250 | 0.923 | 0.5 | 0.935 |
| 33 | embodied-consciousness | consciousness-chip | 0.501 | 0.250 | 0.981 | 0.5 | 0.802 |
| 34 | consciousness-comm | consciousness-training | 0.498 | 0.429 | 0.991 | 0.0 | 0.857 |
| 35 | consciousness-thermodynamics | consciousness-training | 0.497 | 0.429 | 0.912 | 0.0 | 1.000 |
| 36 | consciousness-substrate | consciousness-comm | 0.496 | 0.429 | 0.982 | 0.0 | 0.857 |
| 37 | consciousness-chip | consciousness-thermodynamics | 0.496 | 0.250 | 0.897 | 0.5 | 0.917 |
| 38 | consciousness-comm | multimodal-consciousness | 0.496 | 0.429 | 0.979 | 0.0 | 0.857 |
| 39 | consciousness-comm | consciousness-scaling | 0.495 | 0.429 | 0.974 | 0.0 | 0.857 |
| 40 | consciousness-rng | consciousness-thermodynamics | 0.495 | 0.429 | 0.974 | 0.0 | 0.857 |
| 41 | multimodal-consciousness | consciousness-thermodynamics | 0.494 | 0.429 | 0.900 | 0.0 | 1.000 |
| 42 | embodied-consciousness | eeg-consciousness-bridge | 0.494 | 0.429 | 0.970 | 0.0 | 0.857 |
| 43 | consciousness-scaling | consciousness-thermodynamics | 0.493 | 0.429 | 0.895 | 0.0 | 1.000 |
| 44 | embodied-consciousness | consciousness-comm | 0.491 | 0.429 | 0.957 | 0.0 | 0.857 |
| 45 | consciousness-wasm | consciousness-substrate | 0.491 | 0.429 | 0.955 | 0.0 | 0.857 |
| 46 | consciousness-comm | consciousness-transplant | 0.488 | 0.429 | 0.939 | 0.0 | 0.857 |
| 47 | consciousness-rng | consciousness-training | 0.488 | 0.429 | 0.938 | 0.0 | 0.857 |
| 48 | consciousness-comm | sedi-universe | 0.487 | 0.429 | 0.936 | 0.0 | 0.857 |
| 49 | consciousness-substrate | multimodal-consciousness | 0.487 | 0.429 | 0.997 | 0.0 | 0.735 |
| 50 | consciousness-thermodynamics | consciousness-transplant | 0.486 | 0.429 | 0.860 | 0.0 | 1.000 |

## 5. 공명 상수 히스토그램 (ASCII)

```
bin    count  |────────────────────────────────────────────────────
0.00      30  |███
0.05      13  |█
0.10      20  |██
0.15     175  |███████████████████
0.20     277  |██████████████████████████████
0.25     461  |██████████████████████████████████████████████████
0.30     103  |███████████
0.35      36  |████
0.40      29  |███
0.45      48  |█████
0.50      23  |██
0.55       4  |
0.60       3  |
0.65       2  |
0.70       1  |
```

## 6. 해석

- 평균 스코어: **0.265**
- 고공명 (S≥0.5): **33쌍** (2.7%)
- 중공명 (0.3≤S<0.5): **216쌍** (17.6%)
- 저공명 (S<0.3): **976쌍** (79.7%)

- 최고 공명쌍은 `cross_dse` 리스트가 실제로 상호 참조되는 경우에 집중됨 (bidir=1.0).
- n6_avg가 70~80%대에 몰려 있어 n6 근접도 항목은 대체로 0.9 이상 기여.
- 후속: 상위 S≥0.5 쌍 → 실제 공통 상수(BT 번호) 추출 → .hexa 변환 및 전체 55,945쌍 확장.

## 7. 산출물 경로

- `~/Dev/nexus/shared/dse_cross/top50_domains.jsonl`
- `~/Dev/nexus/shared/dse_cross/pair_scores.jsonl`
- `~/Dev/nexus/shared/dse_cross/resonance_hist.jsonl`
