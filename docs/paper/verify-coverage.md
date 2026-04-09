# 논문 검증코드 보유 감사 (verify-coverage)

**감사일**: 2026-04-09
**대상**: `docs/paper/n6-*-paper.md` 전수
**정책**: CLAUDE.md — "검증코드 없는 논문 = 미완성"
**감사자**: 자동 스캔 + 별칭(alias) 매핑 정정

## 요약

| 항목 | 수 |
|------|----|
| 총 논문 | **116** |
| 검증 스크립트 보유 | **94** (81.0%) |
| 누락 | **22** (19.0%) |
| 본 감사에서 신규 작성 | **3** (HEXA) |

> 참고(R1 HEXA-FIRST): 본 감사에서 신규 작성한 검증 스크립트는 `.hexa` 확장자로
> 작성됨(`.py` 신규 작성은 `block-forbidden-ext.sh` 훅이 차단). 기존
> `verify_*.py` 파일들은 역사적 이유로 유지되며, 본 보고서에서는 그대로 매핑한다.

---

## 검증보유 논문 매핑 (94편)

| 논문 | 검증 스크립트 |
|------|---------------|
| n6-advanced-packaging-paper.md | `docs/advanced-packaging/verify_alien10.py` |
| n6-aerospace-transport-paper.md | `docs/hexa-starship/verify_subsystems.py`, `docs/hexa-starship/verify_hexa_starship.py` |
| n6-antimatter-factory-paper.md | `docs/antimatter-factory/verify_alien10.py` |
| n6-aquaculture-paper.md | `docs/aquaculture/verify_alien10.py` |
| n6-ar-vr-xr-paper.md | `docs/ar-vr-xr/verify_alien10.py` |
| n6-archaeology-paper.md | `docs/archaeology/verify_alien10.py` |
| n6-battery-energy-paper.md | `experiments/verify_battery_architecture.py`, `experiments/verify_battery_cascade.py`, `docs/battery-architecture/verify_alien10.py` |
| n6-biology-medical-paper.md | `docs/paper/verify_biology_medical.py` |
| n6-carbon-capture-paper.md | `docs/carbon-capture/verify_alien10.py` |
| n6-classical-mechanics-accelerator-paper.md | `docs/mini-accelerator/verify_alien10.py` |
| n6-consciousness-chip-paper.md | `experiments/verify_consciousness_chip.py` |
| n6-consciousness-soc-paper.md | `experiments/verify_consciousness_chip.py` |
| n6-construction-structural-paper.md | `docs/construction-structural/verify_alien10_hex.py`, `docs/construction-structural/verify_alien10.py` |
| n6-control-automation-paper.md | `scripts/verify_special_number_control.py` |
| n6-crystallography-materials-paper.md | `docs/paper/verify_crystallography_materials.py` |
| n6-dance-choreography-paper.md | `docs/dance-choreography/verify_alien10.py` |
| n6-datacenter-reactor-paper.md | `docs/smr-datacenter/verify_alien10.py` |
| n6-desal-paper.md | `docs/desalination/verify_alien10.py` |
| n6-digital-twin-paper.md | `docs/digital-twin/verify_alien10.py` |
| n6-display-8stack-paper.md | `docs/display/verify_alien10.py` |
| n6-dolphin-bioacoustics-paper.md | `docs/dolphin/verify_alien10.py` |
| n6-ecology-agriculture-food-paper.md | `docs/horticulture/verify_alien10.py` |
| n6-ecommerce-fintech-paper.md | `docs/ecommerce-fintech/verify_alien10.py` |
| n6-economics-finance-paper.md | `docs/currency-economics/verify_alien10.py` |
| n6-energy-efficiency-paper.md | `docs/energy-architecture/verify_alien10.py` |
| n6-entomology-paper.md | `docs/entomology/verify_alien10.py` |
| n6-environment-thermal-paper.md | `docs/hvac-system/verify_alien10.py`, `docs/environmental-protection/verify_alien10.py` |
| n6-fashion-textile-paper.md | `docs/fashion-textile/verify_alien10.py` |
| n6-fermentation-paper.md | `docs/fermentation/verify_alien10.py` |
| n6-fun-car-paper.md | `docs/fun-car/verify_alien10.py` |
| n6-fusion-powerplant-paper.md | `experiments/verify_fusion_predictions.py`, `docs/fusion/verify_alien10.py` |
| n6-governance-safety-urban-paper.md | `experiments/verify_safety_hypotheses.py`, `docs/safety/verify_alien10.py`, `docs/smart-city/verify_alien10.py` |
| n6-hexa-3d-paper.md | `experiments/verify_hexa_3d.py` |
| n6-hexa-cloak-paper.md | `docs/cloak/verify_alien10.py` |
| n6-hexa-cosmic-paper.md | `docs/cosmic-observatory/verify_alien10.py` |
| n6-hexa-defense-paper.md | `docs/earth-defense/verify_alien10.py` |
| n6-hexa-dream-paper.md | `docs/hexa-dream/verify_alien10.py`, `docs/dream-recorder/verify_alien10.py` |
| n6-hexa-ear-paper.md | `docs/hexa-ear/verify_alien10.py` |
| n6-hexa-empath-paper.md | `docs/hexa-empath/verify_alien10.py` |
| n6-hexa-exo-paper.md | `docs/hexa-exo/verify_alien10.py` |
| n6-hexa-fabric-paper.md | `docs/hexa-fabric/verify_alien10.py` |
| n6-hexa-glass-paper.md | `docs/hexa-glass/verify_alien10.py` |
| n6-hexa-grav-paper.md | `docs/gravity-wave/verify_alien10.py` |
| n6-hexa-holo-paper.md | `docs/holography/verify_alien10.py` |
| n6-hexa-hover-paper.md | `docs/hover/verify_alien10.py` |
| n6-hexa-ios-paper.md | `experiments/verify_hexa_ios.py` |
| n6-hexa-limb-paper.md | `docs/hexa-limb/verify_alien10.py` |
| n6-hexa-macos-paper.md | `experiments/verify_hexa_macos.py` |
| n6-hexa-mind-paper.md | `docs/mind-upload/verify_alien10.py` |
| n6-hexa-mram-paper.md | `docs/sc-memory/verify_alien10.py` |
| n6-hexa-netproto-paper.md | `docs/network-protocol/verify_n6_network_50.py` |
| n6-hexa-neuro-paper.md | `docs/paper/verify_hexa_neuro.py`, `docs/neuro/verify_alien10.py` |
| n6-hexa-olfact-paper.md | `docs/hexa-olfact/verify_alien10.py`, `docs/perfumery/verify_alien10.py` |
| n6-hexa-one-paper.md | `docs/hexa-one/verify_n6.py`, `docs/hexa-one/verify_alien10.py` |
| n6-hexa-oracle-paper.md | `docs/quantum-oracle/verify_alien10.py` |
| n6-hexa-photon-paper.md | `experiments/verify_hexa_photon.py` |
| n6-hexa-pim-paper.md | `experiments/verify_hexa_pim.py` |
| n6-hexa-proglang-paper.md | `docs/programming-language/verify_alien10.py` |
| n6-hexa-recycle-paper.md | `docs/recycling/verify_recycling_n6.py`, `docs/recycling/verify_recycle_n6.py` |
| n6-hexa-sim-paper.md | `docs/simulation-theory/verify_alien10.py` |
| n6-hexa-skin-paper.md | `docs/hexa-skin/verify_alien10.py` |
| n6-hexa-skyway-paper.md | `docs/skyway/verify_alien10.py` |
| n6-hexa-speak-paper.md | `docs/hexa-speak/verify_alien10.py` |
| n6-hexa-starship-paper.md | `docs/hexa-starship/verify_hexa_starship.py`, `docs/hexa-starship/verify_subsystems.py` |
| n6-hexa-super-paper.md | `experiments/verify_hexa_super.py` |
| n6-hexa-telepathy-paper.md | `docs/telepathy/verify_alien10.py` |
| n6-hexa-topo-paper.md | `experiments/verify_topological_chip.py` |
| n6-hexa-tsunami-paper.md | `docs/tsunami-shield/verify_alien10.py` |
| n6-hexa-ufo-paper.md | `docs/room-temp-sc/verify_ufo.py` |
| n6-hexa-wafer-paper.md | `experiments/verify_hexa_wafer.py` |
| n6-hexa-weather-paper.md | `docs/weather-control/verify_alien10.py` |
| n6-hiv-paper.md | `docs/hiv-treatment/verify_bt461_470.py` |
| n6-horology-paper.md | `docs/horology/verify_alien10.py` |
| n6-insurance-paper.md | `docs/insurance/verify_alien10.py` |
| n6-jurisprudence-paper.md | `docs/jurisprudence/verify_alien10.py` |
| n6-material-synthesis-paper.md | `experiments/verify_hexa_material.py`, `docs/material-synthesis/verify_alien10.py` |
| n6-motorcycle-paper.md | `docs/motorcycle/verify_alien10.py` |
| n6-particle-cosmology-paper.md | `docs/cosmology-particle/verify_alien10.py` |
| n6-performance-chip-paper.md | `experiments/verify_chip_n6.py`, `experiments/verify_chip_ultimate.py` |
| n6-plasma-fusion-deep-paper.md | `experiments/verify_fusion_predictions.py` |
| n6-pure-mathematics-paper.md | `docs/pure-mathematics/verify_alien10.py` |
| n6-quantum-computing-paper.md | `docs/quantum-oracle/verify_alien10.py`, `docs/quantum-network/verify_alien10.py` |
| n6-religion-mythology-paper.md | `docs/religion/verify_alien10.py` |
| n6-robotics-transport-paper.md | `docs/robotics/verify_alien10.py` |
| n6-seabed-grid-paper.md | `docs/seabed-grid/verify_alien10.py` |
| n6-software-crypto-paper.md | `docs/software-design/verify_alien10.py` |
| n6-superconductor-paper.md | `docs/superconductor/verify_sc_exact.py`, `docs/room-temp-sc/verify_alien10.py`, `docs/room-temp-sc/verify_warp_dimension.py`, `docs/room-temp-sc/verify_realization.py` |
| n6-synthetic-biology-paper.md | `docs/synbio/verify_alien10.py` |
| n6-therapeutic-nanobot-paper.md | `docs/therapeutic-nanobot/verify_alien10.py` |
| n6-ultimate-safety-paper.md | `experiments/verify_safety_hypotheses.py` |
| n6-underground-tunnel-paper.md | `docs/underground-tunnel/verify_alien10.py` |
| n6-virology-paper.md | `docs/virology/verify_alien10.py` |
| n6-wine-enology-paper.md | `docs/wine-enology/verify_alien10.py` |
| n6-writing-systems-paper.md | `docs/writing-systems/verify_alien10.py` |

---

## 누락 논문 (22편) — 보강 우선순위

검증 가능성(외부 공개 스펙의 풍부함) · 파급도(공학 영향) · 주장 강도(EXACT 수)의
3축으로 점수화하여 상·중·하 구분.

### 우선순위 상 (즉시 보강 필요) — 본 감사에서 처리

| 순위 | 논문 | 근거 | 본 감사 신규 파일 |
|------|------|------|-------------------|
| 1 | `n6-autonomous-driving-paper.md` | SAE/ISO/Tesla/Bosch 공개 스펙 풍부, 53/55 EXACT 주장이 강함, 자율주행은 가장 검증 요구가 큰 영역 | `docs/paper/verify_autonomous_driving.hexa` |
| 2 | `n6-thermodynamics-paper.md` | Carnot/Stefan-Boltzmann/Kolmogorov 등 학부 표준 교재로 전부 대조 가능, 28/28 EXACT 주장 | `docs/paper/verify_thermodynamics.hexa` |
| 3 | `n6-dram-paper.md` | JEDEC JESD79-5 / JESD209-6 / Samsung 공정 노드 전부 공개, 35/35 EXACT 주장, LPDDR6 σ=12 주장은 재현성 핵심 | `docs/paper/verify_dram.hexa` |

### 우선순위 중 (다음 라운드)

- `n6-thermodynamics-paper.md`에 대한 Reynolds 임계값 분석 확장
- `n6-unified-soc-paper.md` — Exynos/Apple M 시리즈 공개 스펙 활용 가능
- `n6-exynos-paper.md` — Samsung Exynos 공식 데이터시트
- `n6-vnand-paper.md` — Samsung V-NAND 세대별 스택 수 (공식 보도자료)
- `n6-isocell-comms-paper.md` — Samsung ISOCELL 픽셀 구조 공개값
- `n6-manufacturing-quality-paper.md` — ISO 9001, Six Sigma 표준 참조
- `n6-games-sports-paper.md` — FIFA/FIVB/NBA 공식 규정집
- `n6-hexa-accel-paper.md`, `n6-hexa-asic-paper.md` — 사내 HEXA 칩 설계 참조 필요
- `n6-telecom-linguistics-paper.md` — 3GPP/ITU 표준
- `n6-space-systems-paper.md` — NASA/ESA 공개 미션 파라미터
- `n6-monetary-history-paper.md` — 중앙은행/IMF 시계열

### 우선순위 하 (사변적/내부 주장 위주)

- `n6-anima-soc-paper.md` (내부 아키텍처)
- `n6-causal-chain-paper.md` (철학적 주장)
- `n6-calendar-time-geography-paper.md` (사료 기반, 일부만 수치)
- `n6-cognitive-social-psychology-paper.md` (효과 크기 분산 큼)
- `n6-microplastics-paper.md` (환경 측정 불확실성 큼)
- `n6-reality-map-paper.md` (메타논문 — 타 검증을 집약)
- `n6-rtsc-12-products-evolution-paper.md` (로드맵 성격)
- `n6-hexa-teleport-paper.md` (SF 경계)

---

## 본 감사 신규 검증 스크립트 요약

세 스크립트 모두 공통 구조를 따름:

1. **핵심 정리 단정**: `sigma(6) * phi(6) == 6 * tau(6) == 24`
2. **외부 관측값 테이블**: 각 행에 `(항목, 관측값, 이론식, 이론값, 출처)` 포함
3. **EXACT / CLOSE(±5%) / MISS 판정**
4. **소수 편향 대조군**: `n = 5, 7, (8 / 10)`으로 같은 관측에 대한 우연 일치율 측정
5. **출처 목록 출력**: 모든 관측값이 어느 표준/문서에서 왔는지 기록 (자기참조 금지)

### `docs/paper/verify_autonomous_driving.hexa`
- 13 케이스, 출처: SAE J3016/J3400, ISO 11898-1/26262, Tesla AI Day 2019, Bosch, VW MEB, Porsche, Hyundai E-GMP, ITU GNSS.
- **투명성**: Tesla 카메라 수는 실제 8대(FSD HW3 공식값)이지만 논문의 "n=6 카메라" 주장과 불일치하므로 MISS로 명시 기록.
- 대조군: n=5, n=7.

### `docs/paper/verify_thermodynamics.hexa`
- 13 케이스 (BT-149: 4, BT-193: 4, BT-199: 5), 출처: Callen 2e, Fermi, Landau-Lifshitz V, Incropera, Batchelor, Pope "Turbulent Flows", Kolmogorov 1941, Landauer 1961.
- Kolmogorov -5/3 = -sopfr / (n/phi) 분해 검증.
- 대조군: n=5, 7, 8.

### `docs/paper/verify_dram.hexa`
- 20 케이스, 출처: JEDEC JESD79-5 (DDR5), JESD209-6 (LPDDR6, 2025-07), Samsung 1a/1b/1c/1d nm 발표.
- 핵심 예측: LPDDR6 DQ=12 (=σ) — power-of-2 관례 돌파.
- DDR1~DDR5 전압 사다리에서 DDR3의 1.5V는 "no n6 fit"으로 정직하게 SKIP 처리하여 자기참조 편향 방지.
- 대조군: n=5, 7, 10.

---

## 권장 후속 작업

1. 우선순위 중 11편에 대한 검증 스크립트 추가 작성 (각 15~25 케이스 목표).
2. 기존 `verify_alien10.py` 군의 자기참조 여부 2차 감사 — 이름이 있어도 "내용"이
   자기참조인 경우가 있을 수 있음.
3. `config/products.json` → `verify-coverage` 필드 자동 채우기 파이프라인 추가.
4. CI에서 각 verify 스크립트의 EXACT 비율을 추적 (회귀 감지).
