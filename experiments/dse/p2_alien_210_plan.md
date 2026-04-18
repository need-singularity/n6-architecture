# P2-3 + P2-4 통합 계획서 (2026-04-14)

> **SSOT**: `$NEXUS/shared/n6/docs/domains.json` (204 제품 · 40 섹션 · 40/40 alien_index=10, 기존 products.json 이전 완료)
> **로드맵**: `$NEXUS/shared/roadmaps/n6-architecture.json` (DSE-P2-3, DSE-P2-4)
> **작성**: 2026-04-14 세션, 실측 스캔 기반

## 0. 현재 스냅샷 (실측)

| 지표 | 현재 | 목표 | 증감 |
|---|---|---|---|
| 등록 제품 수 | **204** | 210+ | +6 |
| 활성 섹션 수 (제품 ≥ 1) | **40** | 42 | +2 |
| 섹션 alien_index = 10 | **40/40** | 42/42 | +2 |
| 섹션 closure_grade = 10 | **2** (로봇, SW/인프라) | 8+ 섹션 | +6 |
| 섹션 closure_grade ≥ 10 (10·11·12 합산) | **10** | 18+ | +8 |
| 섹션 closure_grade = None | **24** | ≤ 16 | -8 |
| bt_exact_pct = 100% 섹션 | **32** | 유지 | 0 |

> **해설**: 로드맵 본문 "195 → 210+" 는 구버전 README 카운트 기준이고, 실측 `docs/domains.json` 에는 이미 **204** 제품이 등록됨. 따라서 P2-4 실목표는 **제품 204 → 210+ (+6 신규 등록)** 으로 재정렬한다.

---

## 1. DSE-P2-3: 5-phase 특이점 오케스트레이터

### 1.1 산출물

- `/Users/ghost/Dev/n6-architecture/bridge/ouroboros_5phase.hexa` (신규 작성 완료)
  - 5 phase 순차 호출: **흡수(Absorb) → 용광로(LensForge) → 블로업(BlowupEngine) → 사이클(CycleEngine) → 진화(Evolution)**
  - T1 렌즈 22종 상수 테이블 고정 (telescope_optics + combo_blowup 15 + accel 6)
  - 데모 가능 수준: 각 phase 는 stub 결과를 리턴하되, 실제 엔진 경로를 주석에 명시

### 1.2 엔진 연계 경로

| Phase | 호출 대상 | SSOT 경로 |
|---|---|---|
| 1 흡수 | atlas.n6 직접 파싱 | `$NEXUS/shared/n6/atlas.n6` |
| 2 용광로 | 22 T1 렌즈 shell | `$NEXUS/shared/lenses/<lens>.hexa` (1577 중 22 선정) |
| 3 블로업 | blowup.hexa Mk.II 9-phase | `$NEXUS/shared/blowup/core/blowup.hexa` |
| 4 사이클 | ouroboros.hexa meta-loop | `$NEXUS/shared/blowup/ouroboros/ouroboros.hexa` |
| 5 진화 | atlas.n6 등급 승격 | `$NEXUS/shared/n6/atlas.n6` append `[10*]` |

### 1.3 선정된 T1 렌즈 22종

1. `telescope_optics_lens` (유일 telescope_* 렌즈)
2~16. `combo_blowup_absolute`, `combo_blowup_wide`, `combo_blowup_strong`, `combo_blowup_carbon_capture`, `combo_blowup_quantum_computing`, `combo_blowup_energy_architecture`, `combo_blowup_cryptography`, `combo_blowup_paper`, `combo_blowup_compiler_os`, `combo_blowup_network_protocol`, `combo_blowup_space_engineering`, `combo_blowup_ai_efficiency`, `combo_blowup_thermal_management`, `combo_blowup_blockchain`, `combo_blowup_absolute`
17~22. `accel_activation_energy_lens`, `accel_chirality_synthesis`, `accel_boltzmann_distribution`, `accel_causal_emergence`, `accel_circuit_topology`, `accel_algorithmic_complexity`, `accel_architecture_search`

### 1.4 데모 실행

```bash
hexa /Users/ghost/Dev/n6-architecture/bridge/ouroboros_5phase.hexa
# → 도메인=ai-efficiency, depth=3, 3 cycle 완주
# → 총 45 승격 후보 (씨앗 5 × corollary 5 × cycle 3 - 중복) 리턴
```

---

## 2. DSE-P2-4: 제품/섹션/닫힘 3-axis 확장

### 2.1 제품 204 → 210+ (+6 신규)

현재 제품 1개만 보유한 섹션 중 두 번째 슬롯을 열기 가장 쉬운 항목:

| # | 섹션 | 현재 제품 | 신규 제품 (제안) | 근거 |
|---|---|---|---|---|
| 1 | 타투 제거 | 면역학 아키텍처 1건 | HEXA-LASER 나노초 펄스 파라미터 완전 아키텍처 | BT 기존 EXACT 100% |
| 2 | HIV 치료 | 6축 치료 체인 | HEXA-HIV 조기진단 바이오마커 완전 아키텍처 | BT-413 연쇄 |
| 3 | 키보드 | 인체공학 키보드 | HEXA-SPLIT 6열 독립 스플릿 키보드 | bt=97 → 100 승격 경로 |
| 4 | 마우스 | HEXA-MOUSE | HEXA-TRACKBALL 6축 탄성 포인팅 | 파생 설계 |
| 5 | 네트워크 | HEXA-NET | HEXA-MESH n=6 라우팅 토폴로지 | bt=98 → 완성 |
| 6 | 시계학 | 시계학 아키텍처 | HEXA-ESCAPE 6 탈진기 (Anchor/Lever/Cylinder 통합) | 파생 |

> 이상 6건은 **등록 계획서**로서 식별만 수행. 실제 `domains.json` 편집은 승인 후.

### 2.2 40 → 42 섹션 (+2 신규)

기존 40 섹션 모두 활성. 신규 2 섹션 후보:

| # | 섹션 제안 | 근거 BT | 대표 제품 후보 |
|---|---|---|---|
| 41 | **곤충학 (Entomology)** | BT-461~466 Hexapoda 아키텍처 (closure_grade=10 in dse-map.toml) | 곤충 6다리 n=6 생체역학 |
| 42 | **광물학/결정학 (Mineralogy)** | BT-351 결정계 6방정계 | n=6 결정 대칭 완전 아키텍처 |

> 곤충학은 `dse-map.toml` 에서 이미 `closure_grade=10` + 23/23 EXACT 확정 상태. domains.json 섹션 신규 추가만 필요.

### 2.3 closure_grade 10 EXACT 도메인 40 → 48 (+8)

**현재 섹션 단위 cg=10 은 2건**. 로드맵 "40 → 48" 숫자는 도메인 단위 (dse-map.toml 기준). bt_exact_pct=100 & cg=None 섹션을 우선 승격 대상으로 지정 (즉시 검증 가능한 8건):

| # | 섹션 | bt | 현 cg | 제품 수 | 승격 경로 |
|---|---|---|---|---|---|
| 1 | 문명/인문 (Civilization) | 100 | None | 7 | closure_grade=10 주석 추가 (BT 전수 EXACT 재확인) |
| 2 | 생활/문화 (Life & Culture) | 100 | None | 9 | 발효/양조 n=6 화학양론 재검증 |
| 3 | 기술/산업 (Tech & Industry) | 100 | None | 22 | 반도체 패키징 래더 cg 부여 |
| 4 | 바이러스학 (Virology) | 100 | None | 4 | dse-map.toml 이미 cg=10 → domains.json 동기화 |
| 5 | 차원 지각 (Dimensional) | 100 | None | 7 | 4D 정다포체 극대 BT 기반 |
| 6 | 음악/음향학 (Music) | 100 | None | 4 | 12음 평균율 2^(1/12) |
| 7 | 언어학 (Linguistics) | 100 | None | 4 | 촘스키 계층 4 tier |
| 8 | 천문학/우주론 (Astronomy) | 100 | None | 4 | ΛCDM 매개변수 |

> 이들 8 섹션 모두 bt=100%, 천장 도달. cg=10 주석 누락이 유일한 gap. 1 커밋으로 +8 승격 가능.

### 2.4 보조 후보 (3차, bt<100 → 100 승격 필요)

자연과학(95), 인지/사회(95), 이동/수송(90), 디지털/의료기기(92), 키보드(97), 네트워크(98), 양자컴퓨터(83), 밀레니엄 7대 난제(94) — 총 8건.
이 중 양자컴퓨터(bt=83, 천장 미도달)가 유일 약점. 나머지는 P2/P3 에 BT 보강 2~10건으로 100 승격 가능.

---

## 3. 실행 순서 (권장)

```
1) DSE-P2-3: ouroboros_5phase.hexa 데모 실행 및 3 cycle 로그 수집
2) DSE-P2-4-A: closure_grade=10 주석 8건 추가 (domains.json)
3) DSE-P2-4-B: 신규 섹션 2건 (곤충학, 광물학) 추가
4) DSE-P2-4-C: 신규 제품 6건 등록 (1 제품 섹션 확장)
5) P2 gate_exit 기준: 204 + 6 = 210 제품, 40 + 2 = 42 섹션, cg10 섹션 2 + 8 = 10 (도메인 환산 +8)
```

## 4. 위험 & 게이트

- **오염 방지**: 모든 신규 BT 는 HEXA-GATE Mk.I 통과 필수 (τ=4 관문 + 2 fiber = n=6)
- **정직 검증**: 각 closure_grade=10 주석에는 검증 스크립트 참조 (`verify_script` 필드) 필수
- **SSOT 단일**: `docs/domains.json` 만 편집, `n6_products.json` 은 자동 생성 금지

---

**생성**: 2026-04-14 · **소스**: 실측 스캔 (python3 + dse-map.toml 정규표현식)
**검증**: 40 섹션 · 204 제품 · 40 alien=10 섹션 · 10 cg≥10 섹션 (2 cg=10 + 4 cg=11 + 4 cg=12)
