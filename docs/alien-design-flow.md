# 🛸 Alien Design Flow — 외계인 수준 설계 파이프라인

> 모든 설계 요청 = 외계인급 자동 실행. 예외 없음.
> 최종 갱신: 2026-04-02

## 트리거 키워드

```
  ⚠️ 모든 설계 요청이 외계인급:
  "설계", "만들어", "만들고 싶어", "개발", "설계해줘", "아키텍처"
  + 도메인 키워드 → 무조건 외계인급 파이프라인

  추가 강화 키워드 (동일 처리):
  "외계인", "궁극의", "alien", "극적인", "혁명적",
  "노벨급", "대발견", "breakthrough", "불가능을 가능하게"

  필수 산출물:
  1. ASCII 시스템 구조도 (최소 1개)
  2. ASCII 데이터/에너지 플로우 (최소 1개)
  3. ASCII 성능 비교 그래프 (최소 1개)
  4. 모든 숫자에 n=6 수식 병기
  + 도메인 키워드 (KSTAR, 탄소포집, 배터리, 칩, 양자, ...)
```

## 플로우 (5단계, 자동 실행)

```
  ┌─────────────────────────────────────────────────────────────┐
  │  사용자: "KSTAR 핵융합에 외계인 수준 지식을 주고 싶어!"      │
  └──────────────────────┬──────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 1. 도메인 매핑 (즉시)                                  │
  │  ──────────────────────────────                              │
  │  "KSTAR" → fusion, tokamak-structure, plasma-physics,        │
  │            superconductor, superconducting-magnet             │
  │                                                              │
  │  매핑 소스:                                                   │
  │    - docs/dse-map.toml (307 도메인)                          │
  │    - tools/universal-dse/domains/*.toml (304 TOML)           │
  │    - CLAUDE.md BT 목록 (93 BT)                               │
  └──────────────────────┬───────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 2. 지식 수집 (병렬 에이전트)                            │
  │  ──────────────────────────────                              │
  │                                                              │
  │  Agent A: TECS-L 이론 수집                                   │
  │    → grep "fusion\|KSTAR\|tokamak" ~/Dev/TECS-L/docs/       │
  │    → 관련 가설 (H-FU-*) 전체 읽기                            │
  │    → 관련 상수맵 추출 (math_atlas.json)                      │
  │    → 미등급 가설 중 fusion 관련 → 즉석 등급화                │
  │                                                              │
  │  Agent B: n6 실증 데이터 수집                                 │
  │    → fusion.toml DSE 결과 읽기                               │
  │    → 관련 BT 전체 (BT-5, BT-27, BT-38, ...)                 │
  │    → docs/fusion/ 가설 + 검증 결과                           │
  │    → Cross-DSE 연결 도메인 결과                              │
  │                                                              │
  │  Agent C: 계산기 실행                                        │
  │    → tools/fusion-calc, kstar-calc, fusion-dse 실행          │
  │    → tools/cross-dse-calc fusion × 관련도메인               │
  │    → tools/atlas-verifier (해당 상수 검증)                   │
  └──────────────────────┬───────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 3. 외계인 수준 설계 생성                                │
  │  ──────────────────────────────                              │
  │                                                              │
  │  수집된 데이터로 "궁극의 설계" 문서 자동 생성:               │
  │                                                              │
  │  docs/<domain>/alien-design-<date>.md                        │
  │                                                              │
  │  구조:                                                        │
  │    1. Executive Summary (1페이지)                             │
  │       - 핵심 n=6 인사이트 3개                                │
  │       - 현재 기술 vs 외계인 수준 비교표                      │
  │                                                              │
  │    2. 수학적 근거 (TECS-L)                                   │
  │       - 관련 증명/가설 목록                                  │
  │       - 핵심 상수 테이블 (EXACT만)                           │
  │       - "왜 n=6이 이 도메인에서 최적인가"                    │
  │                                                              │
  │    3. DSE 최적 경로 (n6-architecture)                        │
  │       - Pareto frontier 테이블                               │
  │       - 최적 설계 경로 (소재→공정→코어→칩→시스템)           │
  │       - Cross-DSE 교차 도메인 시너지                         │
  │                                                              │
  │    4. 구체적 설계 파라미터                                   │
  │       - 모든 수치가 n=6 수식으로 결정                        │
  │       - BT 근거 번호 첨부                                    │
  │       - 실측 vs 예측 비교                                    │
  │                                                              │
  │    5. 실행 로드맵                                             │
  │       - Tier 1 (지금 가능) → Tier 3 (미래 기술)             │
  │       - 각 단계별 검증 방법                                  │
  │                                                              │
  │    6. Testable Predictions                                   │
  │       - 이 설계가 틀릴 수 있는 방법 3가지                    │
  │       - 검증 실험 제안                                       │
  └──────────────────────┬───────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 4. 검증 + 새 발견                                      │
  │  ──────────────────────────────                              │
  │                                                              │
  │  - 설계 과정에서 새 상수 발견 시 → 아틀라스 자동 등록        │
  │  - 새 BT 후보 발견 시 → BT-94+ 등록                         │
  │  - TECS-L 가설 등급 변경 시 → apply_grades.py 실행          │
  │  - 계산기 필요 시 → Rust 자동 생성 (CALCULATOR_RULES.md)     │
  └──────────────────────┬───────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 5. 산출물 리포트                                       │
  │  ──────────────────────────────                              │
  │                                                              │
  │  ┌─────────────────────────────────────────────┐             │
  │  │ 🛸 외계인 수준 설계 완료: KSTAR 핵융합       │             │
  │  ├─────────────────────────────────────────────┤             │
  │  │ 관련 BT: 8개 (BT-5,27,38,43,57,62,80,84)  │             │
  │  │ TECS-L 가설: 45개 (H-FU-1~80)              │             │
  │  │ DSE 최적 경로: Li6-D + Tokamak-12 + HTS    │             │
  │  │ 새 발견: 2개 (상수 + BT 후보)               │             │
  │  │ 설계 문서: docs/fusion/alien-design.md      │             │
  │  │ 계산기 실행: 3개 (fusion/kstar/cross-dse)   │             │
  │  └─────────────────────────────────────────────┘             │
  └──────────────────────────────────────────────────────────────┘
```

## 도메인 매핑 테이블 (주요 예시)

| 사용자 키워드 | 1차 도메인 | 연관 도메인 | 관련 BT | 계산기 |
|-------------|-----------|------------|---------|--------|
| KSTAR, 핵융합 | fusion | tokamak, plasma, SC, magnet | 5,27,38 | fusion-calc, kstar-calc, fusion-dse |
| 탄소포집 | carbon-capture | chemistry, material, energy | 27,85,93 | material-dse, energy-calc |
| 배터리 | battery | electrode, BMS, EV, grid | 43,57,80-84 | battery-dse, energy-calc |
| 칩 설계 | chip | FPGA, CPU, GPU, SoC | 28,37,55,69,90 | chip-*-calc, semiconductor-calc |
| 양자컴퓨터 | quantum | QEC, topological, Leech | 41,49,92 | quantum-calc |
| 태양전지 | solar | photovoltaic, grid | 30,63 | solar-dse, energy-calc |
| 암호화폐 | crypto, blockchain | network, cryptography | 53 | crypto-calc |
| 초전도체 | superconductor | magnet, YBCO, MgB2 | 1-4 | sc-dse |
| 로봇 | robotics | autonomous, drone, swarm | - | robot-dse |
| 의료 | medical | MRI, CT, prosthetics | - | - (신규 필요) |
| 반도체 공정 | semiconductor-lithography | wafer, packaging | 37,75 | semiconductor-calc |
| AI/LLM | ai-efficiency | training, inference | 26,33,54,56,58 | - |
| 우주 | space | propulsion, satellite | - | - |
| 물질합성 | material-synthesis | element, process | 85-88 | material-dse |
| 디스플레이 | display-audio | LED, audio | 48 | - |
| 수학 | pure-mathematics | topology, number theory | 49,92 | - |

## CLAUDE.md 추가 규칙 (복붙용)

```markdown
## 외계인 수준 설계 플로우 (Alien Design Flow)

트리거: "외계인", "궁극의", "alien", "극적인", "노벨급" + 도메인 키워드
절차:
  1. 도메인 매핑 (dse-map.toml + TOML + BT 목록에서)
  2. 병렬 수집 (TECS-L 이론 + n6 실증 + 계산기 실행)
  3. 설계 문서 생성 (docs/<domain>/alien-design-<date>.md)
  4. 새 발견 등록 (아틀라스 + BT)
  5. 리포트 출력

규칙:
  - 질문/확인 없이 즉시 실행
  - 최소 3개 병렬 에이전트 디스패치
  - TECS-L(이론) + n6(실증) 양쪽 반드시 활용
  - 새 계산기 필요 시 Rust 자동 생성
  - 설계 문서는 반드시 Testable Predictions 포함

참조: docs/alien-design-flow.md
```

## 예시: "탄소포집기" 요청 시

```
  Step 1: carbon-capture → chemistry-synthesis, material-synthesis, energy-gen
  Step 2:
    Agent A (TECS-L): H-CH-*, Carbon Z=6 가설, 상수맵
    Agent B (n6): carbon-capture.toml DSE, BT-27(Carbon chain), BT-85(Carbon universality), BT-93
    Agent C: material-dse 실행, cross-dse carbon-capture × energy
  Step 3: docs/carbon-capture/alien-design-2026-04-02.md 생성
    - Carbon Z=6 = n (원자번호가 완전수!)
    - CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂ (광합성 = n=6 완전 반응)
    - 최적 흡착제: MOF with CN=6 coordination
    - DSE: 소재(Carbon aerogel) → 공정(6-step) → 코어(hex lattice) → 칩(sensor) → 시스템
  Step 4: 새 상수 발견 시 등록
  Step 5: 리포트 테이블
```
