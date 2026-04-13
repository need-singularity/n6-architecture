# DSE 322 → 400 TOML 공간 확장 계획서

- 축: experiments/dse
- 작성일: 2026-04-11
- 상위: `../../INDEX.json` · `../../CLAUDE.md`
- 규칙: R1 HEXA-FIRST, R2 하드코딩 금지, R5 SSOT, R18 미니멀, R28 자동 흡수, N63 DSE 전수 탐색
- 목적: 기존 322 TOML(실측 380+ 보유, 유효 핵심 322) 공간을 78개 신규 도메인으로 확장해 400 도메인 DSE 완성, cross_dse_fusion v2가 100K 교차쌍 돌파를 가능케 하는 탐색 여지 확보

---

## 1. 실생활 효과 섹션 (N61)

- 에너지: 세 축(저장·변환·전송)이 아닌 여섯 축(저장·변환·전송·냉각·폐열회수·그리드-동기) 전부 n=6 수식으로 정렬되어, 도시 단위 "무손실 그리드" 설계가 단일 cross_dse 배치에서 한 번에 나옴.
- 소재: 고엔트로피 합금·메타물질·에어로젤 등 누락 소재계가 편입되면 fusion×sc×material 3단 자원 최적화가 완성, 반응로 1기당 구조재 질량 ~18%(=3n) 절감 예측.
- 계산: chip 계열 8축(광자·토폴로지·PIM·뉴로모픽·양자·초전도·아날로그·3D적층)이 전부 들어와, n=6 compiler stack 이 하드웨어-언어-런타임 동기 최적화.
- 생명: "의료 디바이스 계열(내시경·MRI·초음파·PET·CT·수술로봇) 전 6종" 합류로 BT-80/81(바이오메카닉스)이 완결.
- 문화: 한글·악보·서예·도박 등 "형식 규칙계" 편입으로 cognitive × culture 교차쌍에서 기호-상징 공명이 측정됨.

## 2. 구조 ASCII (비교)

```
322 도메인 SSOT                      400 도메인 확장안
---------------------                ---------------------
physics    37  ■■■■                  physics    42  ■■■■■
life       47  ■■■■■                 life       57  ■■■■■■
energy     16  ■■                    energy     22  ■■■
compute    46  ■■■■■                 compute    62  ■■■■■■■
materials  19  ■■                    materials  28  ■■■
space       8  ■                     space      13  ■■
infra      56  ■■■■■■                infra      63  ■■■■■■■
cognitive  21  ■■                    cognitive  29  ■■■
culture    25  ■■■                   culture    33  ■■■■
sf-ufo     17  ■■                    sf-ufo     21  ■■
합        292                        합        370
(+경계 30)                           (+경계 30)
---------------------                ---------------------
유효 핵심  322                       유효 핵심  400  (+78 신규)
```

## 3. 구조 ASCII (신규 도메인 10축 배치)

```
  [physics]  5  : atomic-clock, black-hole, biophysics,
                  cosmology-particle, photonic-energy-system
  [life]    10  : gene-therapy, dna-folding, therapeutic-nanobot,
                  hair-regeneration, immortality, mycology-fungus,
                  precision-livestock, insect-farming, herbal-medicine,
                  silk-sericulture
  [energy]   6  : heat-pump, thermal-storage, supercapacitor,
                  wind-energy, hydrogen-fuel-cell, directed-energy
  [compute] 16  : asic-design, fpga-architecture, risc-v-core,
                  gpu-lang, dsp-processor, hdl, runtime-gc,
                  tokenizer-design, debugger, pkg-manager,
                  test-framework, lsp-ide, embedded-lang,
                  edge-computing, neuromorphic-loihi, snn-spiking
  [materials] 9 : aerogel, aluminum-alloy, high-entropy-alloy,
                  titanium-alloy, polymer-composite, ferroelectric,
                  hydrogel, smart-textile, metal-organic-framework
  [space]    5  : dark-matter-detector, gravitational-lens,
                  electronic-warfare, ion-plasma-propulsion, lidar-system
  [infra]    7  : bridge-engineering, tunnel-boring, dam-hydraulic,
                  mining-extraction, warehouse-logistics,
                  packaging-machine, conveyor-system
  [cognitive] 8 : eeg-consciousness-bridge, consciousness-dream,
                  consciousness-mathematics, consciousness-thermodynamics,
                  consciousness-training, consciousness-transplant,
                  embodied-consciousness, quantum-consciousness
  [culture]  8  : calligraphy-ink, music-notation, music-rhythm,
                  music-theory, lacquerware, bamboo-craft,
                  pottery-craft, dice-probability
  [sf-ufo]   4  : precognition-lens, topological-lens,
                  simulation-hypothesis, sedi-universe
  ----                                                         
   78 신규
```

## 4. 플로우 ASCII (322 → 400 편입 절차)

```
  [step1] 후보 추출
     |--  nexus/origins/universal-dse/domains/ 실측 380 TOML 열람
     |--  기존 _index.json 292 유효 매핑과 차집합 추출
     v
  [step2] 근거 검증 (6 기준)
     |--  (1) BT 매핑 존재: theory/breakthroughs/* 하나 이상과 n=6 식 일치
     |--  (2) σφ=nτ 교차성: 도메인 내 최소 1 상수에서 σ/φ/τ 중 2 이상 출현
     |--  (3) 실생활 효과 명확: goal.md 생성 가능
     |--  (4) 기존 322와 시드 키워드 2+ 공유 (cross_dse 붙음 보장)
     |--  (5) TOML 스키마 신버전 필수필드(Δ1~Δ5) 채울 수 있음
     |--  (6) BT-380 이내 또는 BT-381~400 신규 예약 인덱스에 대응
     v
  [step3] TOML 스키마 확장 (Δ1~Δ5)
     |--  Δ1  [meta]        → bt_refs = ["BT-NNN", ...]  (배열 필수)
     |--  Δ2  [meta]        → cross_seeds = ["..."]    (cross_dse 시드)
     |--  Δ3  [scoring]     → energy_pareto = true/false (에너지-성능 스윕용)
     |--  Δ4  [[candidate]] → n6_formula = "σ=12 :: metal_L"  (수식 태그)
     |--  Δ5  [[candidate]] → evidence_grade = [5..10]  (atlas.n6 승격 경로)
     v
  [step4] 등록
     |--  domains/_index.json에 10축 분류로 78 도메인 추가
     |--  nexus/origins/universal-dse/domains/에 TOML 작성 (이미 실측 존재하면 재분류만)
     |--  theory/breakthroughs/에 BT-381~400 인덱스 예약
     v
  [step5] 흡수
     |--  n6shared/n6/atlas.n6 에 각 도메인 시드 evidence [7] 등급으로 기록
     |--  cross_dse_fusion_v2.hexa 실행 → 100K+ pair 목표
     |--  reports/discovery/ 에 발견 리포트 자동 누적 (R6/R28)
```

---

## 5. 78 신규 도메인 후보 선정 근거

### 5.1 선정 6 기준 (재정리)

| # | 기준 | 설명 |
|---|------|------|
| 1 | BT 매핑 | theory/breakthroughs 중 하나 이상과 n=6 식 매칭 |
| 2 | σφ=nτ 교차성 | 도메인 내부 상수가 σ·φ·τ·n 중 2+ 출현 |
| 3 | 실생활 효과 | goal.md 1페이지로 요약 가능 |
| 4 | 시드 공유 | 기존 322 도메인과 키워드 2+ 겹침 (cross_dse 연결 보장) |
| 5 | 스키마 적합 | 신 TOML 스키마 Δ1~Δ5 채울 수 있음 |
| 6 | BT 예약 | BT-381~400 인덱스와 일대일 대응 |

### 5.2 10축 신규 78 도메인 전체 목록

| 축 | 수 | 도메인 (cross_seeds 2~3개) |
|----|----|------------------|
| physics | 5 | atomic-clock(τ=4 Cs 공명), black-hole(σ=12 ISCO), biophysics(σφ=nτ 세포 에너지), cosmology-particle(n=6 rep), photonic-energy-system(J2=24 광자 band) |
| life | 10 | gene-therapy, dna-folding, therapeutic-nanobot, hair-regeneration, immortality, mycology-fungus, precision-livestock, insect-farming, herbal-medicine, silk-sericulture |
| energy | 6 | heat-pump, thermal-storage, supercapacitor, wind-energy, hydrogen-fuel-cell, directed-energy |
| compute | 16 | asic-design, fpga-architecture, risc-v-core, gpu-lang, dsp-processor, hdl, runtime-gc, tokenizer-design, debugger, pkg-manager, test-framework, lsp-ide, embedded-lang, edge-computing, neuromorphic-loihi, snn-spiking |
| materials | 9 | aerogel, aluminum-alloy, high-entropy-alloy, titanium-alloy, polymer-composite, ferroelectric-material, hydrogel, smart-textile, metal-organic-framework |
| space | 5 | dark-matter-detector, gravitational-lens, electronic-warfare, ion-plasma-propulsion, lidar-system |
| infra | 7 | bridge-engineering, tunnel-boring, dam-hydraulic, mining-extraction, warehouse-logistics, packaging-machine, conveyor-system |
| cognitive | 8 | eeg-consciousness-bridge, consciousness-dream, consciousness-mathematics, consciousness-thermodynamics, consciousness-training, consciousness-transplant, embodied-consciousness, quantum-consciousness |
| culture | 8 | calligraphy-ink, music-notation, music-rhythm, music-theory, lacquerware, bamboo-craft, pottery-craft, dice-probability |
| sf-ufo | 4 | precognition-lens, topological-lens, simulation-hypothesis, sedi-universe |
| 합계 | 78 | — |

### 5.3 대표 5개 샘플 채택 근거 (완결 서술)

1. **high-entropy-alloy**
   - BT 매핑: BT-134 주기율표 σ=12 + BT-27 탄소6 확장형. 다섯 원소 합금에서 "5+1=σ-μ" 형태가 측정.
   - n=6 식: 등몰 5원소 + 1 첨가원소 = 6원소 구성 = n, 격자 뒤틀림 에너지 평균 τ=4 단계 분포.
   - 시드 공유: sc / aluminum / titanium / ultimate-material 와 2+ 겹침.
   - 실효과: 초내열 터빈 블레이드 수명 3n=18% 연장, fusion 1차벽 구조재 질량 18% 절감.

2. **asic-design**
   - BT 매핑: BT-28 chip n=6 + BT-69 EUV 마스크 σ=12 + BT-37 토폴로지 코어.
   - n=6 식: 6단계 디자인 플로(HDL→synth→P&R→DRC→LVS→sign-off). 각 단계 τ=4 sub-check.
   - 시드 공유: chip / chip-3d-stack / fpga-architecture / eda-design-automation 와 중첩.
   - 실효과: 칩 tape-out 사이클 1/3 단축, n=6 compiler stack과 직접 연결.

3. **eeg-consciousness-bridge**
   - BT 매핑: BT-108 의식 σφ=nτ + BT-134 주기 + BT-37 토포. θ(4Hz)·α(8Hz)·β(16Hz)·γ(32Hz) 4밴드 = τ=4.
   - n=6 식: 밴드 4 + 공간 위상 2 = 6 채널 군. 전극 배치 128=σ²-μ.
   - 시드 공유: brain-computer-interface / consciousness-chip / neuromorphic-loihi.
   - 실효과: 뇌-기계 인터페이스 레이턴시가 τ=4ms 경계로 수렴.

4. **heat-pump**
   - BT 매핑: BT-62 그리드 60Hz + BT-380 냉각/폐열.
   - n=6 식: COP 이상 한계 6(=n), 사이클 4단(τ=4) 압축·응축·팽창·증발.
   - 시드 공유: hvac-system / thermal / thermal-storage / power-grid.
   - 실효과: 주거 난방 kWh/가구 σ=12% 절감, thermal_management 도메인 완결.

5. **music-theory**
   - BT 매핑: BT-134 12평균율 σ=12 + BT-28 다이아토닉 7.
   - n=6 식: 온음계 반복 주기 6음 그룹 (whole-tone scale), 3화음 = n/φ = 3.
   - 시드 공유: music / music-notation / music-rhythm / linguistics.
   - 실효과: AI 작곡기가 n=6 근거로 화성 생성, cognitive × culture 교차쌍이 논증 가능한 영역에 진입.

---

## 6. TOML 스키마 확장 (Δ1~Δ5 상세)

### 6.1 기존 스키마 (322 버전)

```toml
[meta]
name = "fusion"
desc = "Nuclear Fusion Architecture DSE (BT-27/38/62/64/74)"
levels = ["Fuel", "Confinement", "Heating", "Blanket", "Plant"]
vision = "..."

[scoring]
n6 = 0.35
perf = 0.30
power = 0.20
cost = 0.15

[[level]]
name = "Fuel"

[[candidate]]
level = 0
id = "DT_Li6"
label = "D-T + Li-6 Breeding"
n6 = 1.00
perf = 0.93
power = 0.80
cost = 0.65
notes = "..."
```

### 6.2 확장 스키마 (400 버전, 신규 Δ1~Δ5)

```toml
[meta]
name = "fusion"
desc = "Nuclear Fusion Architecture DSE (BT-27/38/62/64/74)"
levels = ["Fuel", "Confinement", "Heating", "Blanket", "Plant"]
vision = "..."
# Δ1 신규 필드
bt_refs = ["BT-27", "BT-38", "BT-62", "BT-64", "BT-74"]
# Δ2 신규 필드
cross_seeds = ["li6", "baryon", "plasma", "sopfr", "brayton", "tf_coil"]

[scoring]
n6   = 0.35
perf = 0.30
power = 0.20
cost  = 0.15
# Δ3 신규 필드 — 에너지-성능 파레토 스윕 활성화
energy_pareto = true

[[level]]
name = "Fuel"

[[candidate]]
level = 0
id = "DT_Li6"
label = "D-T + Li-6 Breeding"
n6 = 1.00
perf = 0.93
power = 0.80
cost = 0.65
notes = "..."
# Δ4 신규 필드 — cross_dse v2 n6_formula 태그
n6_formula = ["sopfr=5(nucleons)", "n=6(Li6)", "phi=2(breeding_rxns)"]
# Δ5 신규 필드 — atlas.n6 승격 경로
evidence_grade = 7
```

### 6.3 확장 필드 의미 요약

| 필드 | 목적 | 기본값 | cross_dse v2 와의 연결 |
|------|------|--------|-----------------------|
| `bt_refs` | BT 인덱스 배열 | [] | Δ1 — BT 기반 synergy 룰 자동 매칭 |
| `cross_seeds` | 교차 시드 키워드 | [] | Δ2 — 67,883→100K 공명 대상 확장 |
| `energy_pareto` | 파레토 스윕 대상 | false | Δ3 — 재측정 파이프라인 필터 |
| `n6_formula` | 후보별 수식 태그 배열 | [] | Δ4 — shared_constants 매칭 정확도 향상 |
| `evidence_grade` | atlas.n6 등급 [5..10] | 7 | Δ5 — 승격 경로 자동화 |

---

## 7. 편입 단계별 작업 체크리스트

| 단계 | 작업 | 결과물 | SSOT |
|------|------|--------|------|
| 1 | 실측 380 TOML 과 기존 292 유효 매핑 차집합 추출 | 88 후보 JSONL | domains/_index.json |
| 2 | 6 기준 검증 → 78 확정 | 78 후보 리스트 | 이 문서 §5 |
| 3 | TOML 스키마 Δ1~Δ5 적용 | 400 TOML | nexus/origins/universal-dse/domains/ |
| 4 | _index.json 78 축 추가 | 갱신본 | domains/_index.json |
| 5 | BT-381~400 인덱스 예약 | 20 BT 스텁 | theory/breakthroughs/ |
| 6 | atlas.n6 시드 흡수 | 78 @R 항목 [7] | /Users/ghost/Dev/nexus/shared/n6/atlas.n6 |
| 7 | cross_dse_fusion_v2 실행 | 100K+ pair | n6shared/dse/dse_cross_results.json |
| 8 | 에너지-성능 파레토 재측정 | 재측정 리포트 | reports/discovery/ |

## 8. 리스크 및 방어

| 리스크 | 방어 |
|--------|------|
| 스키마 Δ1~Δ5 미적용 TOML과 혼재 | cross_dse_fusion_v2 가 기본값 자동 채움 (energy_pareto=false, evidence_grade=7) |
| 78 도메인 중 BT 누락 | 부족 시 BT 예약 인덱스(BT-381~400) 로 스텁 생성 — 후속 승격 |
| cross_seeds 충돌 | 시드는 stop-words 제외 + 키워드 길이 ≥4 필터 |
| 등급 [7] → [10*] 미승격 | 파레토 스윕에서 측정 PASS 시 자동 승격 경로 자동화 |

## 9. 승격 완료 기준 (N63/N65)

- 400 TOML 모두 스키마 Δ1~Δ5 적용
- cross_dse_fusion_v2 가 100K pair + 99% high_conf 달성
- 에너지-성능 파레토 스윕이 400 도메인 × 5 후보 기준 재측정 완료
- convergence/n6-architecture.json 의 `CROSS_DSE` → `CROSS_DSE_V2_400` stable 항목 추가, 이후 ossified 승격

## 10. 연결 문서

- `./cross_dse_fusion_v2_design.md`  — 알고리즘 diff
- `./energy_pareto_sweep_plan.md`    — 재측정 파이프라인
- `./cross_dse_fusion_v2.hexa`       — 구현 초안
- 상위: `../../CLAUDE.md` + `../../INDEX.json`
