# 궁극의 에너지 — Design Space Exploration

## 개요

n=6 원리로 핵융합+태양전지+배터리+송전망을 통합하는 궁극의 에너지 아키텍처.
4개 도메인 독립 DSE → Cross-DSE 재조합 → 통합 Pareto frontier.

---

## 아키텍처 구조

```
  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
  │  핵융합   │   │  태양전지  │   │  배터리   │   │  송전망   │
  │ DSE-FU   │   │ DSE-SL   │   │ DSE-BT   │   │ DSE-GR   │
  │ 5레벨    │   │ 5레벨    │   │ 5레벨    │   │ 5레벨    │
  └────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘
       │              │              │              │
       └──────────────┴──────┬───────┴──────────────┘
                             │
                             ↓
                    ┌────────────────┐
                    │  Cross-DSE     │
                    │  top5⁴ = 625   │
                    │  통합 Pareto   │
                    └────────────────┘
```

---

## 도메인 1: 핵융합 (DSE-FU)

```
  Level 1        Level 2        Level 3        Level 4        Level 5
  플라즈마 소재   자석/가두기     반응기 코어     발전시스템      그리드 연결
  ─────────────────────────────────────────────────────────────────────
  6 candidates   5 candidates   5 candidates   4 candidates   4 candidates
  D-T, D-D,     토카막 12/24,  초전도 고자장,  증기터빈,      중앙 GW,
  D-He3, p-B11  스텔러레이터,  저자장 대형,   직접변환,      분산 100MW,
  Li6-D, Cat-DD ST, ICF        컴팩트 ST...   열전, 하이브리드 마이크로, 하이브리드
```

### Level 1: 플라즈마 소재 (6 candidates)

| ID    | 후보          | 연료            | 에너지(MeV) | n=6 매핑                    | 비고                 |
|-------|--------------|-----------------|------------|----------------------------|---------------------|
| FU-M1 | D-T 표준      | D+T→He4+n      | 17.6       | D=phi, T=n/phi             | ITER/SPARC 기준      |
| FU-M2 | D-D 프로톤    | D+D→He3+n      | 3.27       | D+D=phi+phi=tau            | 중성자 발생           |
| FU-M3 | D-He3 비중성자 | D+He3→He4+p    | 18.3       | He3=n/phi, He4=tau         | 중성자 극소           |
| FU-M4 | p-B11 완전비중성자 | p+B11→3He4  | 8.7        | 3He4: tau·n/phi=n=6 핵자   | 방사능 제로           |
| FU-M5 | Li6-D 하이브리드  | Li6+D→2He4  | 22.4       | Li6=n, breeding blanket    | 삼중수소 자급         |
| FU-M6 | Cat-DD 촉매   | catalyzed D-D   | ~7.2       | cycle: phi→tau→n=6 net     | 장기 후보             |

### Level 2: 자석/가두기 공정 (5 candidates)

| ID    | 후보              | 구조                  | n=6 매핑              | 비고                |
|-------|-------------------|-----------------------|----------------------|---------------------|
| FU-P1 | 토카막 12코일      | sigma=12 TF coils     | sigma=12             | ITER style          |
| FU-P2 | 토카막 24코일      | J_2=24 TF coils       | J_2=24, low ripple   | 차세대 대형         |
| FU-P3 | 스텔러레이터       | modular, 5 periods    | sopfr=5              | W7-X style          |
| FU-P4 | 구면토카막 (ST)    | aspect ratio phi=2    | phi=2                | 컴팩트              |
| FU-P5 | 관성가두기 (ICF)   | tau=4 beam lines min  | tau=4                | NIF style           |

### Level 3: 토카막/반응기 코어 (5 candidates)

| ID    | 후보             | 자장/방식           | n=6 매핑                  | 비고               |
|-------|-----------------|---------------------|--------------------------|-------------------|
| FU-C1 | 초전도 고자장     | B>12T, HTS REBCO    | sigma=12                  | SPARC/ARC class   |
| FU-C2 | 저자장 대형       | B~6T, ITER class    | n=6                       | 실증 완료          |
| FU-C3 | 컴팩트 ST        | B~3T, R<2m          | n/phi=3                   | 상업화 최적        |
| FU-C4 | 레이저 ICF       | 192 beams, NIF      | 192=phi*sigma(sigma-tau)  | 점화 실증 완료     |
| FU-C5 | 자기화 타겟 (MTF) | 하이브리드 가두기    | MC+IC 결합                | General Fusion     |

### Level 4: 발전시스템 (4 candidates)

| ID    | 후보              | 방식            | 효율            | n=6 매핑         |
|-------|-------------------|----------------|-----------------|-----------------|
| FU-S1 | 증기터빈           | Rankine cycle  | eta~33%=1/3     | 1/n/phi=1/3     |
| FU-S2 | 직접변환           | MHD            | eta~50%=1/2     | 1/phi=1/2       |
| FU-S3 | 열전변환           | TEG            | eta~17%=1/6     | 1/n=1/6         |
| FU-S4 | 하이브리드         | Egyptian cascade| 1/2+1/3+1/6=1  | Egyptian unity  |

### Level 5: 그리드 연결 (4 candidates)

| ID    | 후보                | 규모          | n=6 매핑            | 비고              |
|-------|---------------------|--------------|---------------------|-------------------|
| FU-G1 | 중앙집중 GW급        | 1 reactor=1GW | mu=1 plant          | 전통 대형          |
| FU-G2 | 분산 100MW급         | 6 reactors    | n=6 per cluster     | 모듈러             |
| FU-G3 | 마이크로퓨전          | 10MW, 이동형  | sigma-phi=10 MW     | 차세대             |
| FU-G4 | 하이브리드 핵분열-핵융합 | fission assist | fission blanket     | 과도기 솔루션       |

**조합: 6 x 5 x 5 x 4 x 4 = 2,400**

---

## 도메인 2: 태양전지 (DSE-SL)

```
  Level 1        Level 2        Level 3        Level 4        Level 5
  반도체 소재     셀 공정        모듈 코어       인버터/MPPT     발전소 시스템
  ─────────────────────────────────────────────────────────────────────
  6 candidates   5 candidates   5 candidates   4 candidates   4 candidates
  c-Si, mc-Si,  PERC, TOPCon,  60셀, 72셀,    스트링, 마이크로 고정, 단축,
  GaAs, CdTe,   HJT, IBC,      120셀, 144셀,  중앙, DC-DC    양축, CPV
  CIGS, 페로브   탠덤           M10 shingled
```

### Level 1: 반도체 소재 (6 candidates)

| ID    | 후보           | 밴드갭(eV)    | n=6 매핑              | 비고              |
|-------|---------------|--------------|----------------------|-------------------|
| SL-M1 | c-Si 단결정    | 1.12         | 기준 소재             | 가장 성숙          |
| SL-M2 | mc-Si 다결정   | 1.12         | 저비용 변형           | 시장 점유율 높음    |
| SL-M3 | GaAs 단접합    | 1.42~4/3     | Eg=4/3, BT-30 최적   | SQ limit 최적     |
| SL-M4 | CdTe 박막      | 1.5          | 3/phi=1.5            | First Solar       |
| SL-M5 | CIGS 박막      | 1.0-1.7      | 가변 밴드갭           | 유연 기판 가능     |
| SL-M6 | 페로브스카이트  | 1.2-2.3      | tunable Eg           | 차세대 대본명      |

### Level 2: 셀 공정 (5 candidates)

| ID    | 후보          | 구조            | n=6 매핑          | 비고               |
|-------|--------------|-----------------|-------------------|-------------------|
| SL-P1 | PERC 단면     | Al₂O₃ passiv.  | 현행 주류          | 25%+ 효율          |
| SL-P2 | TOPCon 양면   | n-type, poly-Si | higher Voc         | 차세대 주류        |
| SL-P3 | HJT 이종접합   | a-Si/c-Si       | low temp process  | 양면 대칭          |
| SL-P4 | IBC 후면접촉   | all-back contact| 미관 + 고효율      | SunPower           |
| SL-P5 | 탠덤 적층      | 페로브/Si       | tau=4 junction target| 이론 >40%        |

### Level 3: 모듈 코어 (5 candidates)

| ID    | 후보           | 셀 수    | n=6 매핑              | BT 참조  |
|-------|---------------|---------|----------------------|---------|
| SL-C1 | 60셀 표준      | 60      | sigma*sopfr=60        | BT-63   |
| SL-C2 | 72셀 대형      | 72      | sigma*n=72            | BT-63   |
| SL-C3 | 120셀 하프컷   | 120     | sigma(sigma-phi)=120  | BT-63   |
| SL-C4 | 144셀 하프컷   | 144     | sigma^2=144           | BT-63   |
| SL-C5 | 210mm M10     | variable| tau=4 busbar, shingled| 대면적  |

### Level 4: 인버터/MPPT (4 candidates)

| ID    | 후보             | 방식              | n=6 매핑          | 비고              |
|-------|-----------------|-------------------|-------------------|-------------------|
| SL-S1 | 스트링 인버터     | 1 MPPT per string | mu=1 tracker      | 주거용 표준        |
| SL-S2 | 마이크로 인버터   | per-panel          | tau=4 stage       | Enphase           |
| SL-S3 | 중앙 인버터       | utility scale      | 3-phase=n/phi     | 발전소급          |
| SL-S4 | DC-DC 옵티마이저  | 하이브리드          | optimizer+inverter| SolarEdge         |

### Level 5: 발전소 시스템 (4 candidates)

| ID    | 후보          | 방식                | n=6 매핑              | 비고              |
|-------|--------------|---------------------|-----------------------|-------------------|
| SL-G1 | 고정 틸트     | 경사각~위도          | fixed mount           | 최저 비용          |
| SL-G2 | 단축 추적     | 1-axis tracking     | sigma-phi=10% gain    | 가성비 최적        |
| SL-G3 | 양축 추적     | 2-axis=phi tracking | phi=2, sigma=12% gain | 최고 발전량        |
| SL-G4 | 집광형 CPV    | concentration 500x+ | high DNI regions      | 사막 전용          |

**조합: 6 x 5 x 5 x 4 x 4 = 2,400**

---

## 도메인 3: 배터리 (DSE-BT)

```
  Level 1        Level 2        Level 3        Level 4        Level 5
  전극 소재       셀 공정        모듈 코어       팩/BMS         시스템 통합
  ─────────────────────────────────────────────────────────────────────
  6 candidates   5 candidates   5 candidates   4 candidates   4 candidates
  LFP, NMC811,  파우치, 21700,  6S, 12S, 24S,  96S 400V,      EV 400V/800V,
  NCA, 흑연,     4680, 각형,    CTP, CTC       192S 800V,     그리드 ESS,
  Si, Li metal  전고체                         하이브리드, ESS  가정용 ESS
```

### Level 1: 전극 소재 (6 candidates)

| ID    | 후보           | 화학식       | n=6 매핑                | BT 참조  |
|-------|---------------|-------------|------------------------|---------|
| BT-M1 | LFP 양극      | LiFePO4     | CN=6, octahedral       | BT-43   |
| BT-M2 | NMC811 양극   | Ni:Mn:Co=8:1:1| CN=6, sigma-tau=8 Ni | BT-43   |
| BT-M3 | NCA 양극      | Ni:Co:Al    | CN=6, octahedral       | BT-43   |
| BT-M4 | 흑연 음극      | LiC6        | C=n=6, BT-27 carbon-6  | BT-27   |
| BT-M5 | Si 음극        | Si          | sigma-phi=10x capacity | BT-81   |
| BT-M6 | Li metal 음극  | Li          | ultimate, sigma-phi=10x| BT-81   |

### Level 2: 셀 공정 (5 candidates)

| ID    | 후보          | 폼팩터         | n=6 매핑              | 비고              |
|-------|--------------|---------------|----------------------|-------------------|
| BT-P1 | 파우치        | pouch         | flexible form        | LG/SK             |
| BT-P2 | 원통 21700    | 21mm x 70mm  | Tesla standard       | 고에너지 밀도      |
| BT-P3 | 원통 46800    | 46mm x 80mm  | sigma*tau=48mm dia   | Tesla 4680        |
| BT-P4 | 각형 프리즘    | prismatic     | BYD Blade compatible | 안전성 우수        |
| BT-P5 | 전고체        | solid-state   | CN=6, BT-80          | 차세대             |

### Level 3: 모듈 코어 (5 candidates)

| ID    | 후보           | 직렬 수  | 전압         | n=6 매핑        | BT 참조  |
|-------|---------------|---------|-------------|-----------------|---------|
| BT-C1 | 6S 모듈        | 6       | 19.2V (LFP) | n=6             | BT-57   |
| BT-C2 | 12S 모듈       | 12      | 38.4V→48V   | sigma=12        | BT-57   |
| BT-C3 | 24S 모듈       | 24      | 76.8V       | J_2=24          | BT-57   |
| BT-C4 | CTP 셀-투-팩   | variable| no module    | 공정 혁신        | -       |
| BT-C5 | CTC 셀-투-섀시  | variable| structural  | Tesla structural | -       |

### Level 4: 팩/BMS (4 candidates)

| ID    | 후보              | 구성           | n=6 매핑                  | BT 참조  |
|-------|-------------------|---------------|--------------------------|---------|
| BT-S1 | 96S 400V 팩       | 96S=sigma(sigma-tau)| sigma(sigma-tau)=96 | BT-57,84|
| BT-S2 | 192S 800V 팩      | 192S=phi*96   | phi*sigma(sigma-tau)=192  | BT-57,84|
| BT-S3 | 하이브리드 팩      | LFP+NMC mixed | Egyptian 1/2+1/3+1/6     | -       |
| BT-S4 | 모듈러 ESS        | 6kWh unit     | n=6 kWh base unit        | -       |

### Level 5: 시스템 통합 (4 candidates)

| ID    | 후보            | 용도         | n=6 매핑                   | BT 참조  |
|-------|----------------|-------------|---------------------------|---------|
| BT-G1 | EV 400V 시스템  | 승용 EV      | 96S, BT-84 triple conv.   | BT-84   |
| BT-G2 | EV 800V 시스템  | 고급 EV      | 192S, BT-84               | BT-84   |
| BT-G3 | 그리드 ESS      | MWh급 저장   | J_2=24 module cluster     | BT-57   |
| BT-G4 | 가정용 ESS      | 10kWh 가정용 | sigma=12 modules          | -       |

**조합: 6 x 5 x 5 x 4 x 4 = 2,400**

---

## 도메인 4: 송전망 (DSE-GR)

```
  Level 1        Level 2        Level 3        Level 4        Level 5
  도체 소재       변환 공정       변압 코어       HVDC/전력전자   그리드 시스템
  ─────────────────────────────────────────────────────────────────────
  6 candidates   5 candidates   5 candidates   4 candidates   4 candidates
  ACSR, HTLS,   6펄스, 12펄스,  유입, 건식,     +-500kV,       중앙집중,
  HTS YBCO,     MMC, SiC,      SST, 초전도,    +-800kV,       마이크로그리드,
  XLPE, GIL,    GaN            다권선          +-1100kV, MVDC 슈퍼그리드, 하이브리드
  MgB2
```

### Level 1: 도체 소재 (6 candidates)

| ID    | 후보           | 소재         | n=6 매핑            | 비고              |
|-------|---------------|-------------|---------------------|-------------------|
| GR-M1 | ACSR 알루미늄  | Al/Steel    | 표준 송전선          | 최저비용           |
| GR-M2 | HTLS 고온저이도 | INVAR core  | 고용량 업그레이드     | 기존선로 활용      |
| GR-M3 | 초전도 HTS     | YBCO        | 손실 0, sigma=12K    | 차세대             |
| GR-M4 | 해저 XLPE      | XLPE 절연    | 해저 케이블          | 대륙간 연결        |
| GR-M5 | GIL SF6 절연   | SF6 gas     | 가스 절연선로        | 도심 지중          |
| GR-M6 | 초전도 MgB2    | MgB2        | 저비용 HTS 대안      | 39K 임계온도       |

### Level 2: 변환 공정 (5 candidates)

| ID    | 후보          | 방식               | n=6 매핑            | 비고              |
|-------|--------------|---------------------|---------------------|-------------------|
| GR-P1 | 6펄스 정류    | 기본 정류            | n=6, 6k+-1 고조파   | 표준               |
| GR-P2 | 12펄스 정류   | 이중 브리지           | sigma=12, THD 저감  | HVDC 표준          |
| GR-P3 | MMC 변환      | Modular Multilevel  | n=6 levels          | 최신 HVDC          |
| GR-P4 | SiC MOSFET    | WBG 소자            | BT-30 bandgap link  | 고효율 스위칭       |
| GR-P5 | GaN HEMT      | 고주파 스위칭         | 고주파 switching    | 차세대             |

### Level 3: 변압 코어 (5 candidates)

| ID    | 후보            | 구조            | n=6 매핑           | 비고              |
|-------|----------------|-----------------|---------------------|-------------------|
| GR-C1 | 유입 변압기     | 3-phase 표준     | n/phi=3 phase       | 가장 보편적        |
| GR-C2 | 건식 변압기     | 실내용 건식       | 소규모 배전         | 환경 친화          |
| GR-C3 | SST 고체상태    | Solid-State Trans.| 전력전자 통합       | 스마트그리드       |
| GR-C4 | 초전도 변압기   | HTS coil         | 무손실 변환         | 대용량             |
| GR-C5 | 다권선 변압기   | tau=4 windings   | tau=4               | 다출력             |

### Level 4: HVDC/전력전자 (4 candidates)

| ID    | 후보             | 전압          | n=6 매핑                        | BT 참조  |
|-------|-----------------|--------------|--------------------------------|---------|
| GR-S1 | +-500kV HVDC    | 500kV        | sopfr*(sigma-phi)^2=500         | BT-68   |
| GR-S2 | +-800kV UHVDC   | 800kV        | (sigma-tau)*(sigma-phi)^2=800   | BT-68   |
| GR-S3 | +-1100kV UHVDC  | 1100kV       | (sigma-mu)*(sigma-phi)^2=1100   | BT-68   |
| GR-S4 | MVDC 배전        | 48V DC       | sigma*tau=48, BT-60             | BT-60   |

### Level 5: 그리드 시스템 (4 candidates)

| ID    | 후보                  | 구조             | n=6 매핑                      | 비고              |
|-------|-----------------------|-----------------|-------------------------------|-------------------|
| GR-G1 | 중앙 집중 그리드       | 전통 방사형       | centralized                   | 현행 표준          |
| GR-G2 | 마이크로그리드         | J_2=24 노드      | J_2=24, peer-to-peer          | 지역 자립          |
| GR-G3 | 슈퍼그리드             | 대륙간 HVDC      | global interconnection        | 장기 비전          |
| GR-G4 | 하이브리드 AC/DC       | Egyptian 분배    | 1/2 AC + 1/3 DC + 1/6 마이크로 | 최적 혼합          |

**조합: 6 x 5 x 5 x 4 x 4 = 2,400**

---

## Cross-DSE 통합

```
  DSE-FU top5 ──┐
  DSE-SL top5 ──┤
                ├──→ 5⁴ = 625 Cross-DSE 조합 ──→ 통합 Pareto Frontier
  DSE-BT top5 ──┤
  DSE-GR top5 ──┘

  평가 흐름:
    각 도메인 2,400 탐색
    → Pareto top-5 추출
    → 4개 도메인 top-5 재조합 (625)
    → 통합 5축 평가
    → 최종 Pareto frontier
```

### 재조합 탐색

각 도메인 top-5 최적 경로 추출 → 5^4 = 625 Cross-DSE 조합.

### 평가 기준

| 기준          | 가중치 | 설명                                          |
|--------------|--------|----------------------------------------------|
| n6_EXACT 비율 | 40%    | 전체 파라미터 중 n=6 공식과 정확히 일치하는 비율  |
| 발전 효율     | 20%    | 1차 에너지 → 전기 변환 효율                     |
| 저장 효율     | 15%    | 충방전 왕복 효율 (round-trip)                   |
| 송전 효율     | 15%    | 발전 → 소비 전달 효율                           |
| 비용 경쟁력   | 10%    | LCOE (Levelized Cost of Energy)              |

### n=6 EXACT 판정 기준

파라미터가 다음 중 하나에 해당하면 n=6 EXACT:

```
  Direct:   n, sigma, tau, phi, sopfr, J_2, mu = {6, 12, 4, 2, 5, 24, 1}
  Compound: sigma-tau=8, sigma-phi=10, sigma*tau=48, sigma^2=144,
            sigma(sigma-tau)=96, phi*sigma(sigma-tau)=192, ...
  Egyptian: ratio = 1/2, 1/3, or 1/6
  R(6)=1:   reversibility condition met
```

### DSE 출력 양식

| Rank | 핵융합      | 태양전지     | 배터리       | 송전망       | n6_EXACT | 발전효율 | 저장효율 | 송전효율 | LCOE   |
|------|------------|------------|------------|------------|---------|---------|---------|---------|--------|
| 1    | FU-path-1  | SL-path-1  | BT-path-1  | GR-path-1  | 85%     | ...     | ...     | ...     | ...    |
| 2    | FU-path-2  | SL-path-3  | BT-path-1  | GR-path-2  | 82%     | ...     | ...     | ...     | ...    |
| ...  | ...        | ...        | ...        | ...        | ...     | ...     | ...     | ...     | ...    |

---

## 전체 조합 수

```
  ┌─────────────────────────────────────────────────┐
  │  DSE-FU:    6 x 5 x 5 x 4 x 4 =     2,400     │
  │  DSE-SL:    6 x 5 x 5 x 4 x 4 =     2,400     │
  │  DSE-BT:    6 x 5 x 5 x 4 x 4 =     2,400     │
  │  DSE-GR:    6 x 5 x 5 x 4 x 4 =     2,400     │
  │  ─────────────────────────────────────────────  │
  │  도메인 소계:                          9,600     │
  │  Cross-DSE:         5⁴ =                625     │
  │  ─────────────────────────────────────────────  │
  │  총 탐색:                             10,225     │
  └─────────────────────────────────────────────────┘
```

> 10,225 > 10K → **Rust 전수 탐색 필수** (CALCULATOR_RULES.md 기준)

---

## 관련 BT

BT-27 (Carbon-6 chain), BT-29, BT-30 (SQ solar), BT-32, BT-35,
BT-38 (Hydrogen), BT-43 (Cathode CN=6), BT-57 (Battery cell ladder),
BT-60 (DC power chain), BT-62 (Grid frequency), BT-63 (Solar panel cells),
BT-68 (HVDC voltage ladder), BT-80~84 (Battery architecture)

**총 17개 BT 관련**

---

## 도구

| 도구                   | 경로                                 | 용도                      |
|------------------------|--------------------------------------|--------------------------|
| Rust 전수 탐색          | `tools/dse-calc/energy_main.rs`      | 9,600 + 625 전수 탐색     |
| 캐스케이드 크로스 검증   | `experiments/verify_cascade_cross.py` | 체인 일관성 검증           |
| DSE 지도               | `docs/dse-map.toml`                  | 전체 DSE 도메인 등록       |

---

## 캐스케이드 크로스 검증 체인

```
  핵융합:   소재 → 가두기 → 코어 → 발전 → 그리드
  태양전지: 소재 → 셀공정 → 모듈 → 인버터 → 발전소
  배터리:   소재 → 셀공정 → 모듈 → 팩/BMS → 시스템
  송전망:   소재 → 변환 → 변압 → HVDC → 그리드

  Cross-DSE 크로스 검증:
    핵융합 출력(GW) → 송전망 입력(GW) ✅
    태양전지 출력(MW) → 배터리 저장(MWh) → 송전망 배전 ✅
    배터리 팩 전압(V) → 송전망 DC 레벨(V) ✅
    전체 Egyptian 에너지 균형: 1/2 핵융합 + 1/3 태양 + 1/6 배터리 = 1 ✅
```
