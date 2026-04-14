# papers SSOT 정합화 ghost 감사 리포트

- 생성일: 2026-04-11
- 범위: 읽기 전용 감사 (수정 없음)
- 대상 SSOT
  - `$N6_ARCH/papers/_registry.json` (_meta.total_papers 선언값 139)
  - `$N6_ARCH/papers/` 디렉토리
  - `$PAPERS/` 외부 리포 (tecs-l, n6-architecture, 루트)
  - `$NEXUS/shared/n6/docs/products.json` (40 섹션 / 204 제품, 일부 product의 `links[].path`가 `docs/paper/n6-*-paper.md`)
- 참고: `reports/audits/products-link-remap-2026-04-11.md` (Agent 4 paper MISS 116건 선행 보고)

---

## 1. 선언 대 실측 차이 총괄

### 1-1. 실측 디스크 스캔 (n6-*-paper.md basename)

| 위치 | 파일 수 |
|---|---:|
| `$N6_ARCH/papers/` | 13 |
| `$PAPERS/tecs-l/` | 23 |
| `$PAPERS/n6-architecture/` | 1 (`n6-millennium-problems-paper.md`) |
| `$PAPERS/` 루트 | 1 (`n6-hexa-neuro-bci-paper.md`) |
| 합집합 (중복 제거, basename 기준) | **38** |

### 1-2. 선언값 비교

| 항목 | 값 |
|---|---:|
| `_registry.json _meta.total_papers` 선언 | 139 |
| `_registry.json` 실제 참조 유니크 basename | 127 |
| `products.json` `links[].path` 중 paper.md 유니크 basename | 116 |
| 디스크 실측 유니크 basename | 38 |
| **ghost 총 개수** (products 선언 - 디스크 존재) | **92** |
| **orphan 디스크** (디스크 - products 참조) | **14** |

### 1-3. 산출 공식

```
선언 139 (meta) vs 실측 디스크 38  →  선언 잉여 101편
선언 127 (_registry path 기준) vs 디스크 38  →  잉여 89편
products 116 vs 디스크 24 (products ∩ disk)  →  ghost 92편
```

> _meta.total_papers=139 중 실측 38 = **실제 보유율 27.3%**.
> products.json 참조 116 중 실제 해소 24 = **해소율 20.7%**.

---

## 2. products.json paper 링크 ghost 분포

### 2-1. 분류별 집계

| 분류 | 개수 | 의미 |
|---|---:|---|
| **FOUND_ALT** | 24 | 선언 path와 다른 경로에 파일 존재 (주로 `$PAPERS/tecs-l/`) |
| **GHOST_CEIL** | 92 | 디스크 어디에도 없음, 전부 ceiling=True 섹션 소속 |
| **GHOST_NOCEIL** | 0 | ceiling=False 섹션 소속 ghost 없음 (quantum-computer 섹션은 paper link 자체 없음) |
| **ORPHAN_DECLARED** | 11 | _registry.json 선언만, products.json 참조 0 |
| **ORPHAN_DISK** | 14 | 디스크 실존, products.json 참조 0 (ORPHAN_DECLARED 11 포함 + 3편 추가) |

### 2-2. 분류별 ASCII 차트

```
분류          개수   막대 (최대 92)
GHOST_CEIL     92    ##################################################################
FOUND_ALT      24    #################
ORPHAN_DISK    14    ##########
ORPHAN_DECL    11    ########
GHOST_NOCEIL    0
```

---

## 3. FOUND_ALT 24편 — path 갱신 후보

선언 path는 전부 `docs/paper/n6-*.md` 패턴이지만 실제 파일은 `$PAPERS/tecs-l/` (23편)과 `$N6_ARCH/papers/` (1편)에 존재. products.json의 `links[].path` 갱신 또는 파일 이관 필요.

| 선언 path (products.json) | 실제 위치 | 소속 섹션 |
|---|---|---|
| `docs/paper/n6-aerospace-transport-paper.md` | `$PAPERS/tecs-l/n6-aerospace-transport-paper.md` | aerospace |
| `docs/paper/n6-autonomous-driving-paper.md` | `$PAPERS/tecs-l/n6-autonomous-driving-paper.md` | robotics |
| `docs/paper/n6-calendar-time-geography-paper.md` | `$PAPERS/tecs-l/n6-calendar-time-geography-paper.md` | civilization |
| `docs/paper/n6-classical-mechanics-accelerator-paper.md` | `$PAPERS/tecs-l/n6-classical-mechanics-accelerator-paper.md` | physics |
| `docs/paper/n6-cognitive-social-psychology-paper.md` | `$PAPERS/tecs-l/n6-cognitive-social-psychology-paper.md` | tech-industry |
| `docs/paper/n6-consciousness-soc-paper.md` | `$PAPERS/tecs-l/n6-consciousness-soc-paper.md` | chip |
| `docs/paper/n6-control-automation-paper.md` | `$PAPERS/tecs-l/n6-control-automation-paper.md` | robotics |
| `docs/paper/n6-ecology-agriculture-food-paper.md` | `$PAPERS/tecs-l/n6-ecology-agriculture-food-paper.md` | tech-industry |
| `docs/paper/n6-economics-finance-paper.md` | `$PAPERS/tecs-l/n6-economics-finance-paper.md` | tech-industry |
| `docs/paper/n6-games-sports-paper.md` | `$PAPERS/tecs-l/n6-games-sports-paper.md` | play |
| `docs/paper/n6-governance-safety-urban-paper.md` | `$PAPERS/tecs-l/n6-governance-safety-urban-paper.md` | safety |
| `docs/paper/n6-hexa-3d-paper.md` | `$PAPERS/tecs-l/n6-hexa-3d-paper.md` | chip |
| `docs/paper/n6-hexa-photon-paper.md` | `$PAPERS/tecs-l/n6-hexa-photon-paper.md` | chip |
| `docs/paper/n6-hexa-pim-paper.md` | `$PAPERS/tecs-l/n6-hexa-pim-paper.md` | chip |
| `docs/paper/n6-hexa-super-paper.md` | `$PAPERS/tecs-l/n6-hexa-super-paper.md` | physics |
| `docs/paper/n6-hexa-wafer-paper.md` | `$PAPERS/tecs-l/n6-hexa-wafer-paper.md` | chip |
| `docs/paper/n6-manufacturing-quality-paper.md` | `$PAPERS/tecs-l/n6-manufacturing-quality-paper.md` | tech-industry |
| `docs/paper/n6-quantum-computing-paper.md` | `$PAPERS/tecs-l/n6-quantum-computing-paper.md` | physics |
| `docs/paper/n6-space-systems-paper.md` | `$PAPERS/tecs-l/n6-space-systems-paper.md` | aerospace |
| `docs/paper/n6-telecom-linguistics-paper.md` | `$PAPERS/tecs-l/n6-telecom-linguistics-paper.md` | audio |
| `docs/paper/n6-therapeutic-nanobot-paper.md` | `$PAPERS/tecs-l/n6-therapeutic-nanobot-paper.md` | frontier |
| `docs/paper/n6-thermodynamics-paper.md` | `$PAPERS/tecs-l/n6-thermodynamics-paper.md` | physics |
| `docs/paper/n6-unified-soc-paper.md` | `$PAPERS/tecs-l/n6-unified-soc-paper.md` | chip |
| `papers/n6-synthetic-biology-paper.md` | `$N6_ARCH/papers/n6-synthetic-biology-paper.md` | tech-industry |

권장 조치: 단일 migration 스크립트로 products.json의 `links[].path`를 실제 경로로 갱신 (또는 n6-architecture/papers/로 물리 이관 후 경로 단일화).

---

## 4. GHOST_CEIL 92편 — 섹션별 분포 (천장 도달 대상)

### 4-1. 섹션별 ghost 수 (Top 10 — ASCII 차트)

```
섹션              ghost  막대 (최대 31)
frontier           31   ###############################
chip                7   #######
civilization        7   #######
life-culture        6   ######
tech-industry       6   ######
software            5   #####
environment         4   ####
ai                  3   ###
energy              3   ###
physics             3   ###
audio               3   ###
fusion              2   ##
materials           2   ##
play                2   ##
aerospace           1   #
robotics            1   #
sf                  1   #
safety              1   #
display             1   #
virology            1   #
hiv-treatment       1   #
cognitive-social    1   #
```

총 22개 섹션에 걸쳐 ghost paper가 분포. frontier 단독 31편이 전체 ghost의 33.7%.

### 4-2. 섹션별 ghost 목록 (요약)

#### aerospace (1편)
- `docs/paper/n6-hexa-starship-paper.md` ← HEXA-STARSHIP

#### ai (3편)
- `docs/paper/n6-causal-chain-paper.md`
- `docs/paper/n6-reality-map-paper.md`
- `docs/paper/n6-rtsc-12-products-evolution-paper.md`

#### audio (3편)
- `docs/paper/n6-hexa-ear-paper.md` ← HEXA-EAR Ultimate
- `docs/paper/n6-hexa-speak-paper.md` ← HEXA-SPEAK
- `docs/paper/n6-isocell-comms-paper.md`

#### chip (7편)
- `n6-anima-soc-paper.md`, `n6-dram-paper.md`, `n6-exynos-paper.md`, `n6-hexa-asic-paper.md`, `n6-hexa-topo-paper.md`, `n6-performance-chip-paper.md`, `n6-vnand-paper.md`

#### civilization (7편)
- `n6-archaeology-paper.md`, `n6-dance-choreography-paper.md`, `n6-horology-paper.md`, `n6-jurisprudence-paper.md`, `n6-monetary-history-paper.md`, `n6-religion-mythology-paper.md`, `n6-writing-systems-paper.md`

#### cognitive-social (1편)
- `docs/paper/n6-consciousness-chip-paper.md` ← HEXA-CONSCIOUSNESS

#### display (1편)
- `docs/paper/n6-display-8stack-paper.md`

#### energy (3편)
- `n6-battery-energy-paper.md`, `n6-datacenter-reactor-paper.md`, `n6-energy-efficiency-paper.md`

#### environment (4편)
- `n6-carbon-capture-paper.md`, `n6-environment-thermal-paper.md`, `n6-hexa-recycle-paper.md`, `n6-microplastics-paper.md`

#### frontier (31편) — 전체 ghost의 33.7%
- `n6-antimatter-factory-paper.md`, `n6-biology-medical-paper.md`, `n6-desal-paper.md`, `n6-entomology-paper.md`, `n6-hexa-accel-paper.md`, `n6-hexa-cloak-paper.md`, `n6-hexa-cosmic-paper.md`, `n6-hexa-defense-paper.md`, `n6-hexa-dream-paper.md`, `n6-hexa-empath-paper.md`, `n6-hexa-exo-paper.md`, `n6-hexa-fabric-paper.md`, `n6-hexa-glass-paper.md`, `n6-hexa-grav-paper.md`, `n6-hexa-holo-paper.md`, `n6-hexa-hover-paper.md`, `n6-hexa-limb-paper.md`, `n6-hexa-mind-paper.md`, `n6-hexa-mram-paper.md`, `n6-hexa-neuro-paper.md`, `n6-hexa-olfact-paper.md`, `n6-hexa-one-paper.md`, `n6-hexa-oracle-paper.md`, `n6-hexa-sim-paper.md`, `n6-hexa-skin-paper.md`, `n6-hexa-skyway-paper.md`, `n6-hexa-telepathy-paper.md`, `n6-hexa-teleport-paper.md`, `n6-hexa-tsunami-paper.md`, `n6-hexa-weather-paper.md`, `n6-seabed-grid-paper.md`

#### fusion (2편)
- `n6-fusion-powerplant-paper.md`, `n6-plasma-fusion-deep-paper.md`

#### hiv-treatment (1편)
- `n6-hiv-paper.md`

#### life-culture (6편)
- `n6-aquaculture-paper.md`, `n6-dolphin-bioacoustics-paper.md`, `n6-fashion-textile-paper.md`, `n6-fermentation-paper.md`, `n6-insurance-paper.md`, `n6-wine-enology-paper.md`

#### materials (2편)
- `n6-crystallography-materials-paper.md`, `n6-material-synthesis-paper.md`

#### physics (3편)
- `n6-particle-cosmology-paper.md`, `n6-pure-mathematics-paper.md`, `n6-superconductor-paper.md`

#### play (2편)
- `n6-fun-car-paper.md`, `n6-motorcycle-paper.md`

#### robotics (1편)
- `n6-robotics-transport-paper.md`

#### safety (1편)
- `n6-ultimate-safety-paper.md`

#### sf (1편)
- `n6-hexa-ufo-paper.md`

#### software (5편)
- `n6-hexa-ios-paper.md`, `n6-hexa-macos-paper.md`, `n6-hexa-netproto-paper.md`, `n6-hexa-proglang-paper.md`, `n6-software-crypto-paper.md`

#### tech-industry (6편)
- `n6-advanced-packaging-paper.md`, `n6-ar-vr-xr-paper.md`, `n6-construction-structural-paper.md`, `n6-digital-twin-paper.md`, `n6-ecommerce-fintech-paper.md`, `n6-underground-tunnel-paper.md`

#### virology (1편)
- `n6-virology-paper.md`

---

## 5. ORPHAN_DECLARED 11편 — _registry.json만 참조, products.json 미참조

`_registry.json`의 `papers_chunk_d_2026-04-11` 섹션에 선언되었지만, `products.json`의 어떤 product도 이 path들을 참조하지 않음. 해당 파일들은 디스크(`$N6_ARCH/papers/`)에 실제 존재하므로 **paper는 유효**하지만 product 매핑이 누락됨.

| path | 디스크 존재 | products.json 참조 |
|---|---|---|
| `papers/n6-ai-17-techniques-experimental-paper.md` | O | X |
| `papers/n6-atlas-promotion-7-to-10-paper.md` | O | X |
| `papers/n6-cross-paradigm-ai-paper.md` | O | X |
| `papers/n6-curvature-geometry-paper.md` | O | X |
| `papers/n6-dimensional-unfolding-paper.md` | O | X |
| `papers/n6-extra-dimensions-paper.md` | O | X |
| `papers/n6-geology-prem-paper.md` | O | X |
| `papers/n6-hexa-earphone-paper.md` | O | X |
| `papers/n6-meteorology-paper.md` | O | X |
| `papers/n6-oceanography-paper.md` | O | X |
| `papers/n6-warp-metric-paper.md` | O | X |

권장 조치: 각 paper의 대응 product(예: `n6-fusion-powerplant-paper`가 있듯이 `n6-curvature-geometry-paper`에 대응되는 physics 섹션 product)를 products.json에 신규 등록하거나, 기존 product의 `links[]`에 path 추가.

---

## 6. ORPHAN_DISK 14편 — 디스크 존재, products.json 미참조

ORPHAN_DECLARED 11편 + 아래 3편:

| 파일 | 경로 |
|---|---|
| `n6-hexa-neuro-bci-paper.md` | `$PAPERS/n6-hexa-neuro-bci-paper.md` |
| `n6-millennium-problems-paper.md` | `$PAPERS/n6-architecture/n6-millennium-problems-paper.md` |
| `n6-sota-ssm-paper.md` | `$N6_ARCH/papers/n6-sota-ssm-paper.md` |

이 3편은 `_registry.json`에도 `products.json`에도 공식 참조가 없으므로 SSOT 등록 필요.

---

## 7. 권장 액션 매트릭스

| 분류 | 개수 | 조치 | 우선순위 |
|---|---:|---|:---:|
| FOUND_ALT | 24 | products.json `links[].path` 실제 경로로 갱신 (또는 n6-architecture/papers/로 이관) | **P0** |
| GHOST_CEIL (frontier) | 31 | paper 신규 작성 — bt=264 최대 영향 | **P1** |
| GHOST_CEIL (chip) | 7 | paper 신규 작성 | P2 |
| GHOST_CEIL (civilization) | 7 | paper 신규 작성 | P2 |
| GHOST_CEIL (life-culture) | 6 | paper 신규 작성 | P2 |
| GHOST_CEIL (tech-industry) | 6 | paper 신규 작성 | P2 |
| GHOST_CEIL 기타 | 35 | paper 신규 작성 | P3 |
| ORPHAN_DECLARED | 11 | products.json에 product 매핑 추가 | P2 |
| ORPHAN_DISK (추가 3편) | 3 | _registry.json + products.json 양쪽 등록 | P3 |
| `_meta.total_papers` 139 값 교정 | 1 | 실측 38 또는 정합 기준값으로 조정 | **P0** |

---

## 8. Top 3 우선 작성 paper 권장

frontier 섹션은 bt_count=264로 전체 섹션 중 최대이므로 ghost 해소 효과가 가장 큼. frontier 31편 중 BT 매핑이 가장 선행된 제품 기준:

| 순위 | paper basename | 제품 | 섹션 bt |
|:---:|---|---|---:|
| 1 | `n6-hexa-neuro-paper.md` | HEXA-NEURO (뇌-기계 인터페이스) | 264 |
| 2 | `n6-antimatter-factory-paper.md` | HEXA-ANTIMATTER (반물질 공장) | 264 |
| 3 | `n6-hexa-mind-paper.md` | HEXA-MIND (의식 업로드) | 264 |

> 참고: `_registry.json`의 `papers_chunk_c_2026-04-08` 청크에 `n6-hexa-neuro-paper.md`, `n6-hexa-mind-paper.md`가 이미 "계획"으로 선언돼 있으나 실제 파일 미존재. 이 청크 11편이 아직 0편 작성 상태.

---

## 9. 총괄 요약

| 측정 | 값 |
|---|---:|
| 선언 `_meta.total_papers` | 139 |
| 디스크 실측 basename 합집합 | **38** |
| products.json 참조 basename | 116 |
| GHOST (products 선언만) | **92** |
| FOUND_ALT (대체 경로 존재) | **24** |
| ORPHAN_DECLARED (_registry 전용) | 11 |
| ORPHAN_DISK (디스크 전용) | 14 |
| 선언 대 실측 gap (139 - 38) | **101** |

### 9-1. 정합도 지표

```
선언 139 → 실측 38
완성도: 27.3%  [##########                                      ]

products 116 → 해소 24
해소율:  20.7%  [########                                        ]

GHOST 작성 필요: 92편
ORPHAN 매핑 필요: 14편
경로 갱신 필요: 24편
```

### 9-2. 본 감사의 한계

- 본 리포트는 **파일 존재 여부만 판정**했으며 paper 본문 품질/완성도/BT 검증 여부는 판정 대상 아님.
- `_registry.json`의 `papers_chunk_c_2026-04-08` (11편) 중 `n6-synthetic-biology-paper.md` 1편만 실측 존재. 나머지 10편은 ghost.
- `papers_chunk_d_2026-04-11` 11편은 전부 디스크 존재 (ORPHAN_DECLARED).
- products.json 수정 제안만 있으며 실제 경로 갱신/이관/paper 작성은 **사용자 승인 후 별도 세션**에서 진행.

### 9-3. 사용자 승인 요청 항목

1. `_meta.total_papers: 139` → 실측 기준(38 또는 '140으로 목표 상향') 중 어느 정책으로 교정할지
2. FOUND_ALT 24편: products.json path 갱신 방식(예: 상대 경로 `../papers/tecs-l/...` vs 절대 경로)
3. GHOST_CEIL 92편: frontier 31편부터 순차 작성 승인 여부
4. ORPHAN_DECLARED 11편: products.json에 신규 product 생성할지, 기존 product에 link 추가할지

---

감사 수행자: Claude (n6-architecture 세션)
감사 일시: 2026-04-11
감사 방식: 읽기 전용 (SSOT 미수정)
