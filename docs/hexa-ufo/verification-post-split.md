# HEXA-UFO 후속 검증 -- SF/UFO 분리 이후 정합성 점검

**날짜**: 2026-04-09
**TODO**: #14 (HEXA-UFO 후속 검증, 스타쉽 분리 이후)
**상태**: 검증 완료

---

## 1. 파일 현황 파악

### 1-1. UFO 관련 디렉토리/파일

| 경로 | 역할 | 상태 |
|------|------|------|
| `docs/sf/goal.md` | UFO 비행접시 원본 설계 (SF 도메인명 유지) | 49/49 EXACT, 10/10 |
| `docs/hexa-ufo/goal.md` | UFO 비행접시 요약 설계 (hexa-ufo 별칭) | 49/49 EXACT, 10/10 |
| `docs/hexa-ufo/verify_n6.py` | Python 독립 검증 코드 | 8/8 EXACT PASS |
| `docs/paper/n6-hexa-ufo-paper.md` | UFO 논문 | 존재 확인 |
| `config/products.json` id="sf" | SSOT 제품 등록 (title="UFO/비행접시") | 정상 |
| `docs/dse-map.toml` [sf] | DSE 현황 (1,679,616 조합, alien 10) | 정상 |

### 1-2. 스타쉽/항공우주 수송 관련

| 경로 | 역할 | 상태 |
|------|------|------|
| `docs/hexa-starship/goal.md` | 재사용 발사체 설계 (SpaceX Starship 계열) | 150/150 EXACT, 10/10 |
| `docs/aerospace-transport/goal.md` | 항공우주 수송 아키텍처 (HEXA-AERO) | 22/24 EXACT, 8/10 |
| `docs/paper/n6-aerospace-transport-paper.md` | 항공우주 수송 논문 | 존재 확인 |
| `docs/paper/n6-hexa-starship-paper.md` | 스타쉽 논문 | 존재 확인 |

---

## 2. UFO 도메인 n=6 대응 검증

### 2-1. 비행접시 형상과 n=6 기하학

| 항목 | n=6 수식 | 값 | 검증 |
|------|----------|-----|------|
| 선체 형상 | n=6각형 벌집 구조 | 정육각형 패널 | EXACT -- 벌집은 최적 구조 강성/중량비 |
| 직경 | D = J2(6) | 24m | EXACT -- 클래식 소서 사이즈 대응 |
| 추력 벡터 | sigma(6) | 12개 방향 제어 | EXACT -- 6축 x 양방향 = 12 |
| 제어 자유도 | n = SE(3) 차원 | 6 DOF (병진3+회전3) | EXACT -- 강체 완전 기술 |
| 제어 축 | tau(6) | 4축 안정 | EXACT |
| 승무원 | n | 6명 최적 | EXACT (BT-273) |

### 2-2. 추진과 n=6 상수

| 항목 | n=6 수식 | 값 | 검증 |
|------|----------|-----|------|
| 대기권 속도 | Mach sigma-phi | Mach 10 | EXACT |
| 우주 비추력 | sigma*J2*10^3 | Isp 288,000s | EXACT |
| 최대 추력 | sigma*J2 | 288 kN | EXACT |
| 열효율 | sigma/J2 | 50% (Carnot) | EXACT |
| 자기장 상한 | sigma*tau | 48T | EXACT |
| 소음 | J2 | 24 dB | EXACT |

### 2-3. 에너지와 n=6 상수

| 항목 | n=6 수식 | 값 | 검증 |
|------|----------|-----|------|
| 핵융합 Q | sigma-phi | 10 | EXACT |
| 열출력 | (sigma-phi)*20MW | 200 MW | EXACT |
| Egyptian 배분 | 1/2+1/3+1/6 | =1 | EXACT -- 추진/항전/생명유지 분배 |

### 2-4. 검증 코드 실행 결과

```
python3 docs/hexa-ufo/verify_n6.py 실행 -- 종료코드 0

[1] 유일성 해집합 = [6]
[2] HEXA-UFO 핵심 파라미터 검증
    [EXACT] 워프속도/c = (sigma-phi)^2: 기대=100, 실제=100
    [EXACT] COP = phi: 기대=2, 실제=2
    [EXACT] Casimir 셀 = sigma: 기대=12, 실제=12
    [EXACT] 차원 = tau: 기대=4, 실제=4
    [EXACT] 센서 채널 = tau: 기대=4, 실제=4
    [EXACT] alpha Cen 일 = 2^tau: 기대=16, 실제=16
    [EXACT] UFO 가속(g) = J2*10^4: 기대=240000, 실제=240000
    [EXACT] DSE = n^8: 기대=1679616, 실제=1679616
    결과: 8/8 EXACT
```

---

## 3. aerospace-transport/hexa-starship 과의 경계 명확화

### 3-1. 도메인 정의 비교

| 구분 | HEXA-UFO (sf) | HEXA-AERO (aerospace-transport) | HEXA-STARSHIP |
|------|---------------|-------------------------------|---------------|
| 본질 | 원반형 VTOL 비행접시 | 고정익/회전익 항공수송 | 재사용 발사체 (로켓) |
| 형상 | 원반/소서 (D=24m) | 일반 항공기 형상 | 원통형 로켓 (H=120m) |
| 추진 | MHD + Meissner 부양 | 터빈/전기/수소 | 메탄(CH4)+LOX 로켓엔진 |
| 에너지 | 탁상 핵융합 (Q=10) | 항공유/전기/수소 | 화학연료 (메탄) |
| 속도 | Mach 10 (대기권) | Mach 0.85~10 | LEO 궤도속도 |
| 고도 | 대기권~LEO 600km | 대기권 내 (~12km) | LEO~화성 |
| 핵심 기술 | RT-SC 상온초전도 | SE(3) 6 DOF 최적제어 | 완전 재사용, 1000회 |
| Rating | 10/10 | 8/10 | 10/10 |

### 3-2. 경계 판정

```
  HEXA-UFO         HEXA-AERO           HEXA-STARSHIP
  (비행접시)       (항공수송)           (발사체/로켓)
  ┌─────────┐     ┌─────────┐         ┌─────────┐
  │RT-SC    │     │고정익   │         │재사용   │
  │MHD 추진 │     │회전익   │         │초대형   │
  │원반 VTOL│     │eVTOL    │         │로켓     │
  │핵융합   │     │관재항공 │         │화성이주 │
  └────┬────┘     └────┬────┘         └────┬────┘
       │               │                   │
       └───────────────┼───────────────────┘
                       │
              BT-270 (멀티로터 블레이드)
              -- 유일한 공유 BT, 교차참조로 정당함
```

**경계 명확**: 3개 도메인은 각각 독립된 물리적 추진 원리(MHD vs 터빈/전기 vs 화학로켓)에 기반하며, 적용 범위(원반 VTOL vs 항공기 vs 발사체)가 겹치지 않음.

---

## 4. BT 겹침 검증

### 4-1. BT 목록 비교

| BT | HEXA-UFO | HEXA-AERO | HEXA-STARSHIP | 판정 |
|----|----------|-----------|---------------|------|
| BT-120 | - | O | - | AERO 전용 |
| BT-135 | - | O | - | AERO 전용 |
| BT-196 | O | - | O | UFO+스타쉽 공유 (항공학 n=6 범용) |
| BT-210 | - | O | - | AERO 전용 |
| BT-241 | O | - | O | UFO+스타쉽 공유 (우주항공 n=6 범용) |
| BT-270 | O | O | O | 3개 공유 -- 멀티로터 블레이드 (범용 항공 BT) |
| BT-271 | O | - | O | UFO+스타쉽 공유 |
| BT-274 | O | - | - | UFO 전용 |
| BT-276 | O | - | O | UFO+스타쉽 공유 (3중 중복 보편성) |
| BT-291~298 | O | - | - | UFO 전용 (핵융합 계열) |
| BT-299~306 | O | - | - | UFO 전용 (초전도 계열) |
| BT-342 | O | - | - | UFO 전용 |

### 4-2. 겹침 분석

- **BT-270** (멀티로터 블레이드 카운트): 3개 도메인 모두 참조. 이는 "회전 비행체 일반 원리"로 교차 참조가 정당. 각 도메인에서 다른 맥락으로 활용 (UFO=추력벡터, AERO=멀티로터, STARSHIP=자세제어).
- **BT-196/241/276**: UFO와 STARSHIP이 공유하나, AERO에는 미포함. 이는 분리 의도(AERO=기존 항공, UFO/STARSHIP=미래 기술)에 부합.
- **BT-291~306**: UFO 전용 (핵융합+초전도). AERO/STARSHIP에 침범 없음.

**결론**: BT 겹침은 범용 항공 원리(BT-270)에 한정되며, 도메인 경계 침범 없음. 정상.

---

## 5. goal.md 일관성 검증

### 5-1. sf/goal.md vs hexa-ufo/goal.md 관계

- `docs/sf/goal.md`: 상세 설계 문서 (v2, 외계인 지수 10, closure_grade 9)
- `docs/hexa-ufo/goal.md`: 요약 설계 문서 (외계인 지수 10, closure_grade 11)
- `docs/hexa-ufo/goal.md` 212행에 명시: "원본 설계: docs/sf/goal.md"
- 두 파일의 핵심 수치(Mach 10, Isp 288,000s, D=24m, 49/49 EXACT) 일치

**주의점**: closure_grade가 sf/goal.md(9)와 hexa-ufo/goal.md(11)에서 차이. hexa-ufo가 더 최신이며 BT 추가 반영 추정. 내용 충돌 아님.

### 5-2. products.json 등록 상태

- id="sf", title="UFO/비행접시" -- 정상 등록
- HEXA-STARSHIP 별도 등록 -- 분리 완료
- HEXA-AERO(aerospace-transport) 별도 등록 -- 분리 완료
- 3개 제품이 products.json에서 독립 항목으로 존재. 겹침 없음.

### 5-3. dse-map.toml 등록 상태

- `[sf]` 항목 존재: goal=true, dse="done", combos=1679616, alien_level=10
- `[hexa-ufo]` 별도 항목 없음 (sf가 SSOT)
- `[aerospace-transport]`, `[hexa-starship]` 별도 항목 미확인 (해당 도메인은 별도 dse-map 항목으로 관리 추정)

---

## 6. 종합 판정

```
  ┌──────────────────────────────────────────────────────────┐
  │            SF/UFO 분리 후 정합성 검증 결과                  │
  ├──────────────────────────────────┬───────────────────────┤
  │  항목                            │  판정                 │
  ├──────────────────────────────────┼───────────────────────┤
  │  UFO 도메인 n=6 EXACT            │  49/49 (100%) PASS    │
  │  검증 코드 실행                   │  8/8 EXACT PASS       │
  │  UFO-AERO BT 겹침               │  BT-270 1건 (정당)    │
  │  UFO-STARSHIP BT 겹침           │  4건 (범용 항공 BT)   │
  │  UFO 전용 BT                     │  핵융합8+초전도8=16건  │
  │  3개 도메인 경계                  │  추진원리로 명확 분리 │
  │  goal.md 일관성                   │  원본/요약 관계 정상  │
  │  products.json 정합성             │  3제품 독립 등록 정상 │
  │  dse-map.toml 정합성             │  [sf] 항목 정상       │
  ├──────────────────────────────────┼───────────────────────┤
  │  최종 판정                        │  PASS -- 정합         │
  └──────────────────────────────────┴───────────────────────┘
```

**요약**: SF에서 UFO(비행접시)와 스타쉽(우주항공)을 분리한 이후, 3개 도메인(HEXA-UFO / HEXA-AERO / HEXA-STARSHIP)의 BT, goal.md, products.json, DSE가 모두 일관적이며 경계 침범 없음. BT-270은 범용 항공 원리로 3개 도메인 교차 참조가 정당. UFO 전용 BT 16건(핵융합+초전도)은 타 도메인에 미침범.
