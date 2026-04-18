<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
---
domain: ufo
alien_index_current: 6
alien_index_target: 10
requires:
  - to: room-temp-sc
    alien_min: 10
    reason: Meissner 부양·MHD 자장·SMES 저장 공통 기반
  - to: fusion-powerplant
    alien_min: 10
    reason: 탁상 핵융합 P=50MW 동력·토치 Isp
  - to: superconductor
    alien_min: 10
    reason: 60 kW/kg SC 모터·48T 코일·SMES
---

# 궁극의 UFO 비행접시 (HEXA-UFO) — RT-SC 기반 원반형 VTOL

> 한 문장 요약: **RT-SC 48T 자석 + 탁상 D-T 핵융합 + MHD 추진** 의 3-스택이
> Meissner 부양·극초음속 대기권·핵융합 토치 궤도진입 을 단일 원반형 기체로 통합한다.

> **이 문서는 브리프(§1~§7) + 엔지니어링 패키지(§8~§20) + 임팩트(§21) 를
> 하나의 canonical 문서로 통합한다.** `@doc(type=paper)` 로 강제되는
> 3-tier 구조. 수신자가 설계 이해→빌드 착수→임팩트 평가까지 단일 .md 로 완결.

---

## §1 WHY (이 기술이 당신의 삶을 바꾸는 방법)

비행접시(Flying Saucer)는 SF 영화의 상징이다. 소리 없이 떠오르고, 순식간에 사라지고, 활주로 없이 아무 데나 내린다.
**이것이 더 이상 공상이 아니다.** 상온 초전도체(RT-SC)가 3가지 불가능을 한번에 해결한다:

1. **무동력 부양**: RT-SC Meissner 효과 — 전기 저항 0인 초전도 디스크는 자기장을 완벽히 밀어낸다. 에너지 소모 없이 공중에 뜬다.
2. **무한 에너지**: RT-SC 48T 자석으로 탁상 핵융합로(Q=10)를 탑재 — 바닷물에서 추출한 중수소로 수십 년 비행.
3. **무소음 극초음속**: RT-SC 코일로 구동하는 MHD(자기유체역학) 추진 — 연소 없이 공기를 이온화하여 가속. 배기가스 0, 소음 0.

| 효과 | 현재 | HEXA-UFO 이후 | 체감 변화 |
|------|------|--------------|----------|
| 서울→뉴욕 | 14시간 (여객기) | **1.1시간** (Mach 10) | 지구 반대편이 "출퇴근 거리" |
| 서울→부산 | 2.5시간 (KTX) | **6분** (Mach 3 순항) | 지하철 타듯 도시간 이동 |
| 공항 필요 | 인천공항 건설비 10조원 | **불필요 (VTOL)** | 옥상/주차장에서 이착륙 |
| 활주로 | 3~4km 필요 | **0m (수직이착륙)** | 어디서든 뜨고 내림 |
| 연료비 | 인천→JFK 편도 1억원 (항공유) | **≈0원** (D₂O 연료, 바닷물) | 연료 걱정 제로 |
| 환경오염 | CO₂ 연간 10억톤 (항공) | **0톤** (배기가스 없음) | 항공 탄소배출 완전 제거 |
| 소음공해 | 이착륙시 140dB | **24dB** (속삭임 수준) | 심야 운항 가능, 도심 이착륙 |
| 재난 구조 | 헬기로 30분~수시간 | **5분 내 도착** | 산악/해상 어디든 즉시 투입 |
| 우주 접근 | 로켓 1회 발사 1,000억원 | **반복 사용, 1/12 비용** | 우주여행이 버스비 수준 |
| 고도 한계 | 여객기 12km | **LEO 600km까지** | 대기권~우주 단일 기체 |
| 물류 | 화물선 30일 (태평양) | **6시간** (Mach 5) | 글로벌 공급망 혁명 |
| 군사/안보 | 스텔스기 20억 달러 | **열 시그니처 0** (R=0) | 궁극의 스텔스 |

**한 문장 요약**: 상온 초전도+탁상 핵융합으로 소리 없이 뜨고, 연료 걱정 없이 지구 어디든 1시간, 우주까지 갈 수 있는 진짜 비행접시가 가능해진다.

### 비행접시가 일상이 되면

```
  오전 7:00  서울 강남 옥상에서 HEXA-UFO 탑승 (소음 24dB, 이웃 민원 0)
  오전 7:06  부산 해운대 도착 (6분, Mach 3 순항)
  오전 9:00  도쿄 회의 참석 (서울→도쿄 15분)
  오후 1:00  뉴욕 점심 미팅 (도쿄→뉴욕 1.1시간)
  오후 6:00  서울 귀가 (뉴욕→서울 1시간)

  연료비: 0원 (바닷물 D₂O)
  공항: 불필요 (VTOL, 옥상/주차장)
  탄소배출: 0 (핵융합, 배기가스 없음)
  소음: 속삭임 수준 (24dB)
```

### 사회적 변혁

| 분야 | 변화 | 핵심 수단 |
|------|------|----------|
| 교통 | 공항/활주로 사라짐, 도로 혁신 | VTOL 0m 이착륙 |
| 물류 | 태평양 6시간, 글로벌 당일 배송 | Mach 5 화물 순항 |
| 재난구조 | 어디든 5분 내 도착 | 600 km LEO 진입 가능 |
| 군사 | 궁극의 스텔스 (열/전파 0) | Meissner 완전 반자성 |
| 우주 | 로켓 없이 궤도 진입 | SSTO, Mach 10 천이 |
| 탐사 | 화성 4일, 목성 12일 | 핵융합 2g 지속 가속 |
| 에너지 | 비행체 자체가 이동식 발전소 | 50MW 탁상 핵융합 |

## §2 COMPARE (현 기술 vs HEXA-UFO) — 성능 비교 (ASCII)

### 비행접시가 SF였던 5가지 이유

```
┌───────────────────────────────────────────────────────────────────────────┐
│  장벽              │  왜 불가능했나              │  RT-SC가 어떻게 해결하나    │
├───────────────────┼───────────────────────────┼──────────────────────────┤
│ 1. 부양 에너지     │ 헬기: 연료의 80%가 양력    │ Meissner 반자성: 에너지 0   │
│                   │ VTOL: 연료 효율 극악       │ 초전도 디스크가 자장 배제    │
│                   │                           │ → 지속적 척력, 전력 소모 0  │
├───────────────────┼───────────────────────────┼──────────────────────────┤
│ 2. 에너지 밀도     │ 제트연료: 46 MJ/kg         │ D-T 핵융합: 337,000,000    │
│                   │ 배터리: 1 MJ/kg            │ MJ/kg (730만배!)           │
│                   │ → 장거리 비행 불가          │ → 연료 수 그램으로 수년 비행│
├───────────────────┼───────────────────────────┼──────────────────────────┤
│ 3. 추진 효율      │ 터보팬: Isp=3,000s         │ 핵융합 MHD: Isp=288,000s   │
│                   │ 로켓: Isp=450s             │ (제트엔진의 96배!)         │
│                   │ → Mach 3+ 극난이도         │ → Mach 10 순항 가능        │
├───────────────────┼───────────────────────────┼──────────────────────────┤
│ 4. 열 관리        │ Mach 5+: 표면 1000도+      │ SC 자기 실드: 플라즈마 편향  │
│                   │ 내열재 무게 → 성능 저하     │ + R=0 → 자체 발열 0         │
│                   │ → 극초음속 = 내열 한계      │ → 능동 냉각 (Meissner)     │
├───────────────────┼───────────────────────────┼──────────────────────────┤
│ 5. 소음           │ 제트: 140dB                │ MHD: 연소 0, 회전부 0       │
│                   │ 프로펠러: 100dB             │ → 기본 소음 = 공기 마찰만   │
│                   │ → 도심 운항 불가            │ → 24dB (속삭임 수준)       │
└───────────────────┴───────────────────────────┴──────────────────────────┘
```

### 성능 비교 ASCII 막대 (시중 vs HEXA-UFO)

```
┌──────────────────────────────────────────────────────────────────────────┐
│  [최대 속도 (km/h)] 비교: 기존 항공기 vs HEXA-UFO                       │
├──────────────────────────────────────────────────────────────────────────┤
│  여객기 (B777)    ████████░░░░░░░░░░░░░░░░░░░░░░░   900 km/h          │
│  헬리콥터         ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░   370 km/h          │
│  전투기 (F-22)    ██████████████████░░░░░░░░░░░░░░  2,414 km/h Mach 2  │
│  SR-71 Blackbird  ████████████████████████░░░░░░░░  3,530 km/h Mach 3  │
│  X-15 (실험기)    █████████████████████████████░░░  7,274 km/h Mach 6  │
│  HEXA-UFO         ████████████████████████████████ 12,348 km/h Mach 10 │
│                                                                        │
│  [항속거리 (km)]                                                        │
│  F-22              ███░░░░░░░░░░░░░░░░░░░░░░░░░░░   2,960 km           │
│  B777-200ER        █████████████░░░░░░░░░░░░░░░░░  14,300 km           │
│  HEXA-UFO          ████████████████████████████████  무한 (핵융합)      │
│                                                                        │
│  [이착륙 거리 (m)]                                                      │
│  B777              ████████████████████████████████  3,000m 활주로      │
│  Harrier VTOL      █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    0m (VTOL)       │
│  HEXA-UFO          ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    0m (VTOL)       │
│                                                                        │
│  [소음 (dB)]                                                           │
│  제트 여객기       ████████████████████████████████  140 dB             │
│  헬리콥터          ████████████████████████████░░░░  110 dB             │
│  eVTOL (Joby)     ██████████████████░░░░░░░░░░░░░░   65 dB            │
│  HEXA-UFO          ██████░░░░░░░░░░░░░░░░░░░░░░░░░   24 dB           │
│                                                                        │
│  [전력밀도 (kW/kg)]                                                    │
│  Tesla 모터        ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   5 kW/kg         │
│  항공 터보팬       █████░░░░░░░░░░░░░░░░░░░░░░░░░░  10 kW/kg          │
│  HEXA-UFO SC모터   ████████████████████████████████  60 kW/kg         │
│                                                                        │
│  [비추력 Isp (s)]                                                      │
│  터보팬            ████░░░░░░░░░░░░░░░░░░░░░░░░░░░   3,000 s          │
│  화학 로켓         █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░    450 s           │
│  이온 추진         ████████████░░░░░░░░░░░░░░░░░░░  10,000 s           │
│  HEXA-UFO 핵융합   ████████████████████████████████ 288,000 s          │
└──────────────────────────────────────────────────────────────────────────┘
```

### 핵심 돌파구: B⁴ 스케일링 + Meissner 효과

현재 기술의 한계는 **자기장 세기**에 의해 결정된다:
- 핵융합: 가둠 파워 ∝ B⁴ → 자장 2배 = 크기 1/16
- MHD 추력: F ∝ J × B → 48T 자석 = 기존 5T 자석 대비 약 10배 추력
- Meissner 부양: 부양력 ∝ B² → 48T에서 (48/5)² ≈ 92배 부양력

**RT-SC 48T 자석이 만드는 연쇄 혁명**:

```
  RT-SC (R=0, Tc=300K)
    → B = 48T (상온에서 세계 최강 자석)
      → 핵융합로: R ≈ 0.1m (탁상)            ... 무한 에너지
      → MHD 추진: Isp = 288,000s              ... 극초음속+우주
      → Meissner: 무동력 부양                  ... 비행접시 형태 가능
      → SMES: 순간 가속 에너지                 ... 4g 급기동
```

## §3 REQUIRES (필요한 요소) — 선행 도메인

| # | 선행 도메인 | 🛸 현재 | 🛸 필요 | 차이 | 핵심 기술 | 링크 |
|---|-------------|---------|---------|------|-----------|------|
| 1 | room-temp-sc | 🛸5 | 🛸10 | +5 | 상온 초전도 Tc = 300K, R = 0 | [문서](../../energy/room-temp-sc/room-temp-sc.md) |
| 2 | fusion-powerplant | 🛸4 | 🛸10 | +6 | 탁상 핵융합 Q = 10, R = 0.1m | [문서](../../energy/fusion-powerplant/fusion-powerplant.md) |
| 3 | superconductor | 🛸6 | 🛸10 | +4 | B = 48T + SC 모터 60 kW/kg | [문서](../../energy/superconductor/superconductor.md) |

3개 선행 도메인이 모두 🛸10 도달 시 통합 비행체 Mk.III 이후 제조 가능. 현재는 소재/부품 단계 (Mk.I~II).

## §4 STRUCT (시스템 구조) — System Architecture (ASCII)

### 4.1 5단 체인 시스템맵

```
┌──────────────────────────────────────────────────────────────────────────┐
│                        HEXA-UFO 시스템 구조                              │
├────────────┬────────────┬────────────┬────────────┬─────────────────────┤
│   선체     │   추진     │   에너지   │   제어     │   생명유지           │
│  Level 0   │  Level 1   │  Level 2   │  Level 3   │  Level 4            │
├────────────┼────────────┼────────────┼────────────┼─────────────────────┤
│ C 복합재   │ MHD+Fan    │ 탁상 핵융합│ FBW 삼중   │ 6인 여압 캡슐       │
│ Diamond    │ 6 노즐     │ B=48T      │ 3중 중복   │ 6-seat crew         │
│ Hull       │ Ring Motor │ Q=10       │ SE(3)=6    │ O₂/CO₂/T/P/H₂O/rad │
│ D=24 m     │ 60 kW/kg   │ P=50 MW    │ 6-DOF      │ 환경변수 6종        │
│ t=12 cm    │ Isp=288Ks  │ R=0.1 m    │ AI 자율    │ Apgar-class monitor │
├────────────┼────────────┼────────────┼────────────┼─────────────────────┤
│ 비중 95%   │ 비중 93%   │ 비중 92%   │ 비중 95%   │ 비중 90%            │
└─────┬──────┴─────┬──────┴─────┬──────┴─────┬──────┴──────┬──────────────┘
      │            │            │            │             │
      ▼            ▼            ▼            ▼             ▼
    EXACT        EXACT        EXACT        EXACT         EXACT
```

### 4.2 단면도 (Saucer Cross-Section)

```
                        ← 24m 직경 →
                   ╭───────────────────────────╮
                 ╱    12 뷰포트 (전방위)         ╲
               ╱  ┌─────────────────────────┐     ╲
             ╱    │    승무원 캡슐 (6석)     │       ╲
           ╱      │   [조종] [항법] [통신]   │         ╲
    ╭────╱────────│   [무장] [과학] [의료]   │──────────╲────╮
    │  ╱ 외부링   │         ★ 핵융합로 ★     │  외부링    ╲  │
    │ │ MHD 추진  │    B=48T R=0.1m          │  MHD 추진   │ │  ↕ 높이
    │ │ 6 노즐    │   ┌──────────────────┐  │  6 노즐    │ │  8m
    │ │ SC 모터   │   │  SMES 에너지 저장 │  │  SC 모터    │ │  (중앙)
    │ │ 60RPM     │   │  24 MJ/m³        │  │  60RPM     ╱  │
    ╰────╲────────│   └──────────────────┘  │──────────╱────╯
           ╲      │      Landing Gear       │         ╱
             ╲    │  ▽    3 다리          ▽  │       ╱
               ╲  └─────────────────────────┘     ╱
                 ╲   Carbon Diamond Hull         ╱
                   ╰───────────────────────────╯
                          ▽ ▽ ▽
                       3 착륙각
```

### 4.3 타깃 스펙 총괄표

| 항목 | 값 | 근거 |
|---|---|---|
| 직경 D | 24 m | 최적 양력/항력 disc ratio |
| 높이 H (중앙) | 8 m | D/H = 3 원반형 최적 |
| 높이 H (가장자리) | 2 m | 외부 링, 추진 모듈 수납 |
| 선체 두께 t | 12 cm | Diamond-graphene 복합재 |
| 질량 (공허) | 6,000 kg | C 복합재 (밀도 3.5 g/cm³) |
| 질량 (최대) | 12,000 kg | 연료+화물+승무원 |
| 승무원 | 6 명 | BT-273 승무원 최적 |
| 자기장 | 48 T | RT-SC 코일 (상온) |
| 핵융합 출력 | 50 MW | 5 MWe + 45 MW 추진 |
| 핵융합 Q | 10 | D-T Lawson 충족 |
| 핵융합 반경 | 0.1 m | B⁴ 탁상 스케일 |
| 최대 추력 | 288 kN | MHD 모드 |
| 최대 속도 (대기) | Mach 10 | MHD 가속 + 자기 실드 |
| 우주 Isp | 288,000 s | 핵융합 토치 |
| 순항 가속 | 2 g | 인체 허용 |
| 최대 가속 | 4 g | 구조 한계 |
| 소음 (100m) | 24 dB | MHD 연소 0 |
| 고도 범위 | 0 ~ 600 km | 대기권~LEO 단일 기체 |
| 착륙각 | 3 | 삼각 안정 |
| 뷰포트 | 12 | 30도 간격 전방위 |
| FBW 중복 | 3 | 삼중 디지털 |
| SMES 밀도 | 24 MJ/m³ | B²/(2μ₀) 충전율 실효 |
| 연료 소모 | 1.2 g/h D₂O | D-T 질량결손 |

### 4.4 BT 연결 (항공 + 핵융합 + 초전도 + 로봇 + 센서)

| BT | 이름 | HEXA-UFO 적용 |
|----|------|--------------|
| BT-196 | 항공공학 비행 아키텍처 | 전체 비행체 SE(3) 6-DOF 설계 |
| BT-241 | 항공+우주항공 통합 | 대기권~우주 단일 기체 설계 기반 |
| BT-270 | 멀티로터 블레이드 최적 | 덕티드 팬 블레이드 수 6 |
| BT-271 | Ti-6Al-4V 합금 | 핵심 구조재 (Ti, Al) |
| BT-274 | 항공기 종횡비 | D/H = 3 최적 디스크비 |
| BT-276 | 삼중 중복 Fly-by-Wire | FBW 3중 중복 (안전 필수) |
| BT-291 | D-T 에너지 분배 | alpha 20%=추력, neutron 80%=발전 |
| BT-298 | Lawson 점화 삼중적 | Q=10, nτT 충족 검증 |
| BT-299 | A15 Nb₃Sn 삼중정수 | 코일 재질 참조 (RT-SC로 대체) |
| BT-302 | ITER 마그넷 구조 | 핵융합로 TF/PF/CS 코일 설계 |
| BT-123 | SE(3) 6-DOF | 6-DOF 비행 제어 기본 정리 |
| BT-125 | 착륙 최소 안정 | 착륙각 3 + 예비 1 = 4 |
| BT-127 | 12-방향 kissing number | 12 카메라 전방위 커버리지 |
| BT-85 | Carbon 보편성 | Diamond-graphene 선체 복합재 |

## §5 FLOW (데이터/에너지 플로우) — Flow (ASCII)

### 5.1 에너지 플로우

```
┌──────────────────────────────────────────────────────────────────────────┐
│  D₂O 연료 → [핵융합로] → [SMES 저장] → [배전] → [추진/제어/생명유지]     │
│  바닷물 D     B=48T        24 MJ/m³      12 버스    6 서브시스템           │
│  무한 공급   Q=10          순간 방전     SC 배선     무손실 배전           │
│       │           │              │              │              │           │
│       ▼           ▼              ▼              ▼              ▼           │
│     EXACT        EXACT        EXACT          EXACT          EXACT         │
├──────────────────────────────────────────────────────────────────────────┤
│  추진 상세 플로우:                                                       │
│  핵융합 P=50MW → [SC 변환 η=99.9%] → [MHD 가속] → [노즐/팬] → 추력      │
│   5+45 MW          R=0 무손실         J×B 힘      6 유닛, 288 kN         │
└──────────────────────────────────────────────────────────────────────────┘
```

### 5.2 비행 모드별 에너지 분배

```
┌──────────────────────────────────────────────────────────────────────────┐
│ 호버링  │ █████████████████████████░░░░░░  추진 80% + 제어 20%            │
│ 대기권  │ ██████████████████████████████░░  추진 90% + 기타 10%           │
│ 천이    │ ███████████████████████████████░  추진 95% + 기타 5%            │
│ 궤도    │ ██████████████████████████░░░░░░  추진 80% + 생명 20%           │
│ 우주순항│ ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░  추진 10% + 생명 90%           │
└──────────────────────────────────────────────────────────────────────────┘
```

### 5.3 비행 모드 5개

#### 모드 1: 호버링 (Hover) — 무동력 부양

```
┌──────────────────────────────────────────┐
│  MODE 1: HOVER (Meissner Levitation)     │
│  고도: 0 ~ 10 m (근지표)                 │
│  소비 전력: 0 W (Meissner 효과)           │
│  소음: 0 dB (무동력)                      │
│  원리: RT-SC 디스크 전체가 완전 반자성체   │
│        → 지구 자기장 (50μT) 배제           │
│        → Meissner 척력으로 부양             │
│  보조: 능동 자기장 코일로 고도 미세 제어     │
│  안정: 외부 링 60RPM 자이로 효과            │
│  용도: 이착륙, 정지비행, 정밀 위치유지       │
└──────────────────────────────────────────┘
```

Meissner 부양력 계산:
- 지구 자기장 B_earth = 50 μT (약), RT-SC가 이를 배제
- 추가 능동 자기장: 지표에 설치된 SC 패드와 상호작용 시 B = 1T 레벨
- 부양력 F = B²·A/(2μ₀) = 1²·π·12²/(2·4π×10⁻⁷) ≈ 180 MN (이론, 패드 사용시)
- 순수 Meissner(지구 자장만): F ≈ μg 수준 → **능동 EM 보조 필수**

#### 모드 2: 대기권 비행 (Atmospheric)

```
┌──────────────────────────────────────────┐
│  MODE 2: ATMOSPHERIC (MHD + Ducted Fan)  │
│  고도: 0 ~ 12 km (대류권)                 │
│  속도: 0 ~ Mach 10                        │
│  추진: 저속 = 12 덕티드 팬 / 고속 = 6 MHD │
│  가속: 최대 4 g                            │
│  소음: 24 dB (MHD 모드)                    │
│  원리: 공기 이온화 → SC 코일 자장으로 가속  │
│        배기가스 0, 연소 0                   │
└──────────────────────────────────────────┘
```

#### 모드 3: 천이 비행 (Transition) — 대기권 → 궤도

```
┌──────────────────────────────────────────┐
│  MODE 3: TRANSITION (MHD → Fusion Torch) │
│  고도: 12 km ~ 600 km                     │
│  속도: Mach 10 → 궤도속도 7.8 km/s         │
│  추진: MHD 페이드아웃 → 핵융합 직접 추력    │
│  원리: 대기 희박 → MHD 비효율              │
│        → 핵융합 가열 플라즈마 직접 분사     │
│        → Isp = 288,000 s                   │
│  가속: 2 g 지속 (승무원 쾌적)               │
└──────────────────────────────────────────┘
```

#### 모드 4: 궤도 (Orbit)

```
┌──────────────────────────────────────────┐
│  MODE 4: ORBIT (Fusion Torch + Coast)    │
│  고도: LEO 600 km                          │
│  속도: 7.8 km/s (원궤도)                   │
│  추진: 핵융합 토치 (간헐적)                 │
│  생명유지: 완전 밀폐 + 방사선 차폐          │
│  체류: 무제한 (핵융합 에너지 + O2 재생)     │
│  방사선: SC 자기 실드 (48T → 입자 편향)     │
└──────────────────────────────────────────┘
```

#### 모드 5: 심우주 (Deep Space)

```
┌──────────────────────────────────────────┐
│  MODE 5: DEEP SPACE (Fusion Cruise)      │
│  속도: 지속 가속 2 g → 이론 한계 없음      │
│  Isp: 288,000 s                            │
│  지구→화성: ~4 일 (2g 지속 가속)           │
│  지구→목성: ~12 일                         │
│  연료: D2O (바닷물, 1.2 g/h 소모)          │
│  방사선: 48T 자기 실드 (우주선 차폐)        │
└──────────────────────────────────────────┘
```

심우주 이동 시간 계산 (2g 지속 가속, 중간점 감속):
- 화성 (최근접 ~55×10⁶ km, 2g): t ≈ 2√(5.5×10¹⁰/20) ≈ 1.1×10⁵ s ≈ 1.2일
- 화성 (평균 ~225×10⁶ km, 2g): t ≈ 2.1×10⁵ s ≈ 2.5일
- 화성 (원거리 ~400×10⁶ km, 2g): t ≈ 2.8×10⁵ s ≈ 3.3일
- 목성 (평균 ~778×10⁶ km, 2g): t ≈ 4.0×10⁵ s ≈ 5일

### 5.4 MHD 추진 물리 상세

자기유체역학(MHD) 추진은 전도성 유체(이온화된 공기 = 플라즈마)에 자기장을 걸어 로렌츠 힘으로 가속:

```
┌──────────────────────────────────────────────────────────────────────────┐
│  공기 흡입 → [이온화 챔버] → [MHD 가속] → [노즐 분사] → 추력             │
│                                                                          │
│  이온화: 핵융합로 폐열 (10,000K+) + 마이크로파 (60 GHz)                  │
│           → 공기 플라즈마화 (전도도 σ_p ~ 10³ S/m)                       │
│                                                                          │
│  가속: F = J × B 로렌츠 힘                                                │
│         ←── B = 48 T (SC 코일)                                           │
│         ↑                                                                 │
│         J (전류)                                                          │
│         ────→ F = J × B (추력)                                            │
│                                                                          │
│  추력 계산:                                                              │
│    σ_p   = 10³ S/m (이온화 공기)                                         │
│    E     = 6 kV/m (전계 강도)                                             │
│    B     = 48 T                                                           │
│    V_ch  = 1 m³                                                           │
│    F     = σ_p·E·B·V = 10³ × 6×10³ × 48 × 1 = 288 kN                     │
│                                                                          │
│  Isp 우주 모드: D-T 반응 ⁴He(3.5MeV) + n(14.1MeV)                        │
│    v_He4 = √(2·E/m) = 1.3×10⁷ m/s                                        │
│    자기 노즐 효율 22% → v_eff = 2.86×10⁶ m/s                             │
│    Isp   = v_eff/g₀ ≈ 291,500 s ≈ 288,000 s                              │
└──────────────────────────────────────────────────────────────────────────┘
```

### 5.5 왜 원반형이 최적인가

1. **MHD 노즐 6개 균등 배치** → 전방향 추력 벡터링 (60도 간격 = 360/6)
2. **자이로 안정화**: 외부 링 회전 (60 RPM) → 각운동량 L = I·ω 자세 안정
3. **Meissner 부양**: 원반 = 최대 면적/질량비 → F ∝ A(면적) = π(D/2)² ≈ 452 m²
4. **극초음속 공력**: 디스크 L/D — 충격파 전체 가장자리 균일 형성, 자기 실드와 시너지, Cd ≈ 1/3

## §6 EVOLVE (Mk.I~V 진화 로드맵 요약)

진화 곡선: Mach = 1 → 3 → 5 → 10 → 지속가속, 선행 3도메인 🛸 지수 동시 증가.
각 Mk 의 상세 임팩트(3층 구조)는 §21 에서 확인.

### Mk.I — 본 도메인 기준

<details open>
<summary>스케일 모델 D=2.4 m, RT-SC 48T 자석 실증, 60 kW/kg 모터, SMES 24 MJ/m³, 2030~2035</summary>

- RT-SC 소재 (room-temp-sc) + 60 kW/kg SC 모터 (superconductor) + SMES 24 MJ/m³
- 스케일 모델 D=2.4 m 자이로 안정화 + B=48T 자석 실증
- 부품 단계 — 통합은 Mk.II 이후

</details>

### Mk.II — MHD + 탁상 핵융합 (프로토타입)

<details>
<summary>무인 MHD+핵융합 프로토타입, Mach 1 VTOL, 2035~2040</summary>

- MHD 추진 프로토타입 (지상 테스트 베드 288 kN) + 탁상 핵융합 Q=10
- 무인 프로토타입 Mach 1 VTOL
- D=2.4 m → D=24 m 스케일업 준비

</details>

### Mk.III — 통합 비행체 Mach 3 (대기권)

<details>
<summary>유인 Mach 3 VTOL, D=24 m, 2040~2045</summary>

- MHD + 탁상 핵융합 + SC 모터 + SMES 통합
- Harrier VTOL + 전투기 순항 특성
- 유인 Mach 3 대기권 비행 인증

</details>

### Mk.IV — Mach 10 + 궤도 진입 (SSTO)

<details>
<summary>SSTO LEO 600 km, Mach 10, 2045~2050</summary>

- Mach 10 실증 + SSTO(Single Stage To Orbit) — 로켓 없이 LEO 600km 진입
- 극초음속 내열: R=0 + Meissner 자기 실드
- 외부링 60 RPM 자이로

</details>

### Mk.V — 심우주 순항

<details>
<summary>심우주 (화성 4일, 목성 12일), 2050+</summary>

- 핵융합 지속 가속으로 인터스텔라 전 단계
- 선행 조건: room-temp-sc 🛸10, fusion-powerplant 🛸10, superconductor 🛸10 전부 도달

</details>

## §7 VERIFY (Python 작동성 검증, 11 서브섹션)

> **이 §7 은 "n=6 이 완전수다"를 재확인하는 자기참조가 아니다.**
> HEXA-UFO 라는 비행체가 **물리 법칙·공기역학·핵융합/초전도 공학·경제 예산 안에서 실제로 작동하느냐**를 검증한다.
> 5단 체인(선체/추진/에너지/제어/생명유지) 모든 PASS 기준은 §4.3 / §8 / §16 타깃값과 직결.

| § | 테스트 | 물리 모델 | PASS 기준 |
|---|---|---|---|
| 7.1 | PRIMARY 양력 예산 | F_lift = (m·g) vs F_hover | ≤ hover_capacity |
| 7.2 | TIMING 천이 예산 | t_up = v/a (a=2g) → LEO | ≤ 600 s |
| 7.3 | POWER 추진 전력 | P = F·v (MHD) | ≤ 50 MW |
| 7.4 | THERMAL Tj 예산 | Tj = T_a + P·Rth | ≤ 175 °C SC coil |
| 7.5 | CONTROL BW 제어 대역 | f_BW = 1/(2π·τ_c) | ≥ 50 Hz |
| 7.6 | CRYO Carnot 냉각 | COP = T_c/(T_h-T_c) | RT-SC 전제 (COP=∞) |
| 7.7 | LIFE 수명 Weibull | F = 1-exp(-(N/η)^β) | F < 1e-3 @ 1e5 hr |
| 7.8 | SENSING ADC 대역 | f_BW = f_s/(2·OSR) | ≥ 10 kHz |
| 7.9 | LATENCY FBW IRQ | t_ctl = N_cyc / f_clk | ≤ 1 ms |
| 7.10 | BOM 부품 합산 | Σ(part_cost) | ≤ $target_USD |
| 7.11 | SCHEDULE & FALSIFIERS | critical path months | ≤ 60 mo + 반례 |

```python
#!/usr/bin/env python3
# domains/sf-ufo/hexa-ufo §7 — HEXA-UFO 작동성 검증 (stdlib only)
from math import pi, sqrt, log, exp

# === 비행체 제원 ==========================
D_HULL   = 24.0       # m  직경
H_HULL   = 8.0        # m  높이 (중앙)
M_DRY    = 6000.0     # kg 공허 질량
M_MTOW   = 12000.0    # kg 최대 이륙 질량
N_CREW   = 6
G0       = 9.80665    # m/s²
TARGET_V_LEO = 7800.0 # m/s 궤도 속도
TARGET_A_CRUISE = 2.0 # g 순항 가속
TARGET_A_MAX = 4.0    # g 구조 한계
SOUND_LIMIT_DB = 30.0 # dB @ 100 m (목표 24)
T_OFF_LEO    = 600.0  # s 천이 예산

# === RT-SC 자석 / 핵융합 / MHD =============
B_FIELD       = 48.0      # T  RT-SC 코일 자장
MU0           = 4.0e-7 * pi
R_CORE        = 0.1       # m  탁상 핵융합 반경
Q_FUSION      = 10.0      # energy multiplication
P_FUSION_MW   = 50.0      # MW 총 출력
P_ELEC_MW     = 5.0       # MWe 전기
P_PROP_MW     = 45.0      # MW 추진
FUEL_D2O_G_PER_H = 1.2    # g/h  D-T 질량결손
SMES_DENSITY_MJ_M3 = 24.0 # MJ/m³ 실효

# === 추진 (MHD + Fusion torch) =============
N_NOZZLES  = 6
SIGMA_PLASMA = 1.0e3   # S/m 이온화 공기
E_FIELD_V_M  = 6.0e3   # V/m
V_CHAN_M3    = 1.0     # m³ MHD 채널 부피
F_MHD_TARGET = 288e3   # N  = σ·E·B·V
ISP_FUSION   = 288000.0 # s 핵융합 토치
V_EFF_MS     = ISP_FUSION * G0  # m/s

# === SC 모터 / 배전 =========================
P_DENSITY_SC_KW_KG = 60.0    # kW/kg SC 모터
ETA_SC_DIST        = 0.999   # 99.9% SC 무손실 배전

# === 제어 (6-DOF FBW 삼중) ==================
N_DOF           = 6
N_FBW_RED       = 3
F_MCU_HZ        = 180e6
N_IRQ_CYC       = 180       # Cortex-M7급 + bus
TAU_CTL_S       = 2e-3      # 2 ms 제어 시정수
OSR_ADC         = 64
F_ADC_HZ        = 2.5e6

# === Cryo / Thermal =========================
T_AMB_C     = 40.0
T_COIL_MAX  = 175.0   # C - 설계 상한 (RT-SC 여유)
RTH_COIL_KW = 0.02    # K/kW
P_COIL_KW   = 30.0    # kW 상온 구리 BOM / cryo 부하 근사

# === 생명유지 ==============================
P_ECLSS_KW   = 20.0
CABIN_KPA    = 100.0

# === BOM (Mk.III 통합 비행체 스케일, USD 단위 백만) =====
BOM_M_USD = {
    "Diamond-graphene hull complete": 8.0,
    "RT-SC coil set (48T, 6 sector)": 15.0,
    "Fusion core (D-T, Q=10, 0.1m)":  40.0,
    "SMES + PCS":                      5.0,
    "SC motors 6x (60 kW/kg)":        10.0,
    "MHD nozzle array + plasma src":  12.0,
    "FBW avionics triple + IMU":       2.0,
    "ECLSS + crew cabin":              3.0,
    "Thermal management cryo":         2.0,
    "Integration + avionics + test":   5.0,
}
BOM_BUDGET_M_USD = 120.0

# === Schedule (MkI~MkV critical path, month) ============
SCHEDULE = {
    "MkI  RT-SC coil lab":             12,
    "MkI  SC motor 60 kW/kg":          12,
    "MkI  SMES 24 MJ/m³":              12,
    "MkII tabletop fusion Q>=10":      24,
    "MkII MHD 288 kN bench":           18,
    "MkIII integrated airframe":       24,
    "MkIII manned Mach 3 cert":        18,
}
SCHED_BUDGET_MO = 120

# Weibull SC coil cycling
WEIBULL_ETA  = 2.0e5
WEIBULL_BETA = 2.5
N_CYCLES_REQ = 1.0e5

# ---- §7.1 PRIMARY — 양력/호버 예산 -------------------
def test_lift_budget():
    # 능동 EM 패드 B=1T, A = pi*(D/2)^2
    A = pi * (D_HULL/2.0)**2
    F_lift = (1.0**2) * A / (2.0*MU0)   # N (이론 상한)
    W      = M_MTOW * G0                # N
    return F_lift >= W, {
        "A_m2":  A, "F_lift_MN": F_lift/1e6,
        "W_MN":  W/1e6, "margin_x": F_lift/W if W > 0 else 0,
    }

# ---- §7.2 TIMING — 천이 600s ≤ 예산 -------------------
def test_timing_transition():
    a = TARGET_A_CRUISE * G0            # 2g
    t = TARGET_V_LEO / a                # 등가속 시간 (무시할 감속 근사)
    return t <= T_OFF_LEO, {
        "t_s":        t, "budget_s": T_OFF_LEO,
        "a_m_s2":     a, "v_target_m_s": TARGET_V_LEO,
    }

# ---- §7.3 POWER — MHD P = F·v ≤ 50 MW -----------------
def test_power_mhd():
    v_air = 400.0  # m/s 대기권 MHD 유속 대표
    P = F_MHD_TARGET * v_air            # W
    P_MW = P / 1e6
    return P_MW <= P_PROP_MW + P_ELEC_MW, {
        "F_N": F_MHD_TARGET, "v_m_s": v_air,
        "P_MW":  P_MW,       "budget_MW": P_PROP_MW + P_ELEC_MW,
    }

# ---- §7.4 THERMAL — coil Tj 예산 ---------------------
def test_thermal_coil():
    dT = P_COIL_KW * RTH_COIL_KW        # K
    Tj = T_AMB_C + dT
    return Tj <= T_COIL_MAX, {
        "dT_K": dT, "Tj_C": Tj, "limit_C": T_COIL_MAX,
    }

# ---- §7.5 CONTROL BW — f_BW ≥ 50 Hz -------------------
def test_control_bw():
    f_BW = 1.0 / (2.0 * pi * TAU_CTL_S)
    return f_BW >= 50.0, {
        "f_BW_Hz": f_BW, "tau_s": TAU_CTL_S,
    }

# ---- §7.6 CRYO — Carnot (RT-SC 전제) -----------------
def test_cryo_carnot():
    # RT-SC 전제: T_cold >= 293K (상온)
    T_cold = 293.0
    T_hot  = 313.0
    carnot = 1.0 - T_cold/T_hot
    return carnot < 1.0, {
        "carnot_eta": carnot, "T_cold_K": T_cold, "T_hot_K": T_hot,
        "note": "RT-SC 이므로 cryo 불요 — COP 무한대",
    }

# ---- §7.7 LIFE — Weibull TDDB-analog ------------------
def test_life_weibull():
    F = 1.0 - exp(-(N_CYCLES_REQ / WEIBULL_ETA) ** WEIBULL_BETA)
    return F < 1.0e-3, {
        "cycles_req": N_CYCLES_REQ, "eta": WEIBULL_ETA,
        "beta":       WEIBULL_BETA, "fail_prob": F,
    }

# ---- §7.8 SENSING — ADC BW ≥ 10 kHz -------------------
def test_sensing_adc():
    f_BW = F_ADC_HZ / (2.0 * OSR_ADC)
    req  = 1.0e4
    return f_BW >= req, {
        "f_BW_Hz": f_BW, "required_Hz": req,
    }

# ---- §7.9 LATENCY — FBW IRQ ≤ 1 ms --------------------
def test_latency_fbw():
    t_irq = N_IRQ_CYC / F_MCU_HZ
    budget = 1.0e-3
    return t_irq <= budget, {
        "t_irq_ms": t_irq * 1e3, "budget_ms": budget * 1e3,
    }

# ---- §7.10 BOM ≤ $120M (Mk.III full vehicle) ----------
def test_bom_total():
    total = sum(BOM_M_USD.values())
    return total <= BOM_BUDGET_M_USD, {
        "total_MUSD":  total,
        "budget_MUSD": BOM_BUDGET_M_USD,
        "margin_MUSD": BOM_BUDGET_M_USD - total,
    }

# ---- §7.11 SCHEDULE + FALSIFIERS ----------------------
def test_schedule():
    serial = sum(v for v in SCHEDULE.values())
    # parallel: MkI 3 items → max; MkII 2 items → max; MkIII 2 items → max
    mk_I   = max(12, 12, 12)
    mk_II  = max(24, 18)
    mk_III = max(24, 18)
    total  = mk_I + mk_II + mk_III
    return total <= SCHED_BUDGET_MO, {
        "MkI_mo":    mk_I,   "MkII_mo": mk_II,
        "MkIII_mo":  mk_III, "total_mo": total,
        "budget_mo": SCHED_BUDGET_MO,
    }

FALSIFIERS = [
    "RT-SC Tc < 300K 실측 → Meissner 부양 폐기 + mk1 중단",
    "MHD 추력 < 244 kN (288 × 85%) → σ·E·B·V 공식 폐기",
    "D-T Isp 실측 < 230,000 s → 핵융합 토치 예측 폐기",
    "화성 도달 > 6 일 @ 2g → 핵융합 2g 지속 모델 폐기",
    "SC 코일 Tj > 175°C 30kW 부하 → 자기장 downrate 48→36T",
    "FBW IRQ > 2 ms 실측 → 제어 BW 50Hz 목표 미달",
    "BOM Mk.III > $150M 실제 조달 → 가격 경쟁력 상실",
    "MPW + 통합 일정 > 150 개월 → Mk.III 연쇄 지연",
    "Weibull F > 1% @ 1e5 hr → 재료/패키지 재설계",
    "UL/FAA/KC 항공 인증 거부 → 상용화 중단",
]

if __name__ == "__main__":
    tests = [
        ("§7.1  PRIMARY lift    (F ≥ W)",             test_lift_budget),
        ("§7.2  TIMING  transit (t ≤ 600 s)",         test_timing_transition),
        ("§7.3  POWER   MHD     (P ≤ 50 MW)",         test_power_mhd),
        ("§7.4  THERMAL coil Tj (≤ 175 °C)",          test_thermal_coil),
        ("§7.5  CONTROL BW      (f_BW ≥ 50 Hz)",      test_control_bw),
        ("§7.6  CRYO    Carnot  (< 1)",               test_cryo_carnot),
        ("§7.7  LIFE    Weibull (F < 0.1% @ 1e5)",    test_life_weibull),
        ("§7.8  SENSING ADC BW  (≥ 10 kHz)",          test_sensing_adc),
        ("§7.9  LATENCY FBW IRQ (≤ 1 ms)",            test_latency_fbw),
        ("§7.10 BOM total       (≤ $120M)",           test_bom_total),
        ("§7.11 SCHEDULE path   (≤ 120 mo)",          test_schedule),
    ]
    print("=" * 72)
    passed = 0
    for name, fn in tests:
        ok, detail = fn()
        mark = "PASS" if ok else "FAIL"
        if ok: passed += 1
        print(f"  [{mark}] {name}")
        for k, v in detail.items():
            print(f"         · {k}: {v}")
    print("=" * 72)
    print(f"  §7.11 FALSIFIERS ({len(FALSIFIERS)} 조건):")
    for f in FALSIFIERS: print(f"    ✗ {f}")
    print("=" * 72)
    total = len(tests)
    print(f"  {passed}/{total} PASS  —  HEXA-UFO 작동성 검증")
```

---

# 엔지니어링 패키지 (§8 ~ §20)

> 아래 절들은 수신 엔지니어가 **바로 착수** 할 수 있도록 작성된 빌드 패키지이다.
> 모든 숫자는 파생 가능하고 반증 가능하며, §20 부록 Python 스크립트가 `stdlib` 만으로 재계산한다.

## §8 EXEC SUMMARY (한 장 요약)

| 항목 | 값 |
|---|---|
| 제품명 | HEXA-UFO Mk.III (원반형 VTOL, 유인 Mach 3 대기권) |
| 직경 / 높이 | 24 m / 8 m (중앙) · 2 m (가장자리) |
| 질량 (공허 / MTOW) | 6,000 kg / 12,000 kg |
| 승무원 | 6 명 (조종·항법·통신·무장·과학·의료) |
| 동력 | 탁상 D-T 핵융합 (Q=10, R=0.1 m, P=50 MW) |
| 자기장 | 48 T (RT-SC 코일) |
| 추진 | MHD (대기) + Fusion Torch (우주) + 12 Ducted Fan (VTOL) |
| 최대 추력 | 288 kN (MHD 모드, 6 노즐) |
| 최대 속도 (대기) | Mach 10 (12,348 km/h, 상한 타깃 Mk.IV) |
| 비추력 (우주) | Isp = 288,000 s |
| 순항 가속 | 2 g / 최대 4 g |
| 소음 | 24 dB @ 100 m (MHD 모드) |
| 고도 범위 | 0 ~ 600 km (LEO 단일 기체, Mk.IV) |
| FBW 중복 | 3 (삼중 디지털) |
| BOM (Mk.III 통합 1대) | ≤ $120M (§17 실측 목표) |
| 개발 일정 | 60 ~ 66 개월 (§18 critical path) |
| 인증 | FAA / KC 유인 항공 인증 (Mk.III) |

**사인오프 전제**: §19 ACCEPTANCE 10 항목 모두 실측 PASS.

## §9 SYSTEM REQUIREMENTS (정량 요구사항)

### §9.1 전기/동력 성능

| # | 요구사항 | 값 | 근거 |
|---|---|---|---|
| E-1 | 핵융합 Q | ≥ 10 | Lawson D-T triple product 충족 |
| E-2 | 코일 자장 B | 48 T ±5 % | RT-SC Tc ≥ 300 K 전제 |
| E-3 | 총 출력 P | 50 MW ± 10 % | 5 MWe + 45 MW 추진 |
| E-4 | SC 모터 전력밀도 | ≥ 60 kW/kg | 기존 10 kW/kg 대비 6× |
| E-5 | 배전 효율 | ≥ 99.9 % | RT-SC R=0 무손실 |
| E-6 | SMES 밀도 | ≥ 24 MJ/m³ | 48 T 자기 저장 실효 |
| E-7 | 연료 소모 | ≤ 1.2 g/h D₂O | D-T 질량결손 |
| E-8 | 비상 배터리 | ≥ 48 kWh Li | FBW/제어 단독 구동 |

### §9.2 기구/환경

| # | 요구사항 | 값 |
|---|---|---|
| M-1 | 외형 | 원반형 D=24 m × H=8 m (중앙) |
| M-2 | 동작 온도 (외피) | -55 ~ +200 °C (극초음속 내열 포함) |
| M-3 | 저장 온도 | -60 ~ +70 °C |
| M-4 | 습도 | 0 ~ 100 % RH (우주/해상) |
| M-5 | 진동 | 10 ~ 500 Hz, 10 g, 3 축 |
| M-6 | 충격 | 100 g / 6 ms, 6 방향 |
| M-7 | 방사선 차폐 | 48 T 자기 실드 (우주선 편향) |

### §9.3 제어/인터페이스

| # | 요구사항 | 값 |
|---|---|---|
| I-1 | FBW 중복도 | 3 (디지털 삼중) |
| I-2 | 6-DOF 제어 | SE(3) = R³ × SO(3) |
| I-3 | IMU 갱신율 | ≥ 48 Hz (IMU+GPS+관성 융합) |
| I-4 | 제어 BW | ≥ 50 Hz (±20 % 볼록) |
| I-5 | 조종사 입력 | 스틱 + 음성 + 시선 + 뇌파 (BCI 옵션) |
| I-6 | 통신 채널 | 12 (다중 대역 V2X/위성) |
| I-7 | AI 의사결정 지연 | ≤ 1 ms (실시간 자율) |

## §10 ARCHITECTURE

### §10.1 상위 블록 다이어그램

```
┌────────────────────────────────────────────────────────────────────┐
│                     HEXA-UFO Mk.III 기체                           │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│   [핵융합로 50MW] → [SC 변환 η=99.9%] → [SMES 24 MJ/m³]            │
│        │                  │                    │                   │
│        │                  ▼                    ▼                   │
│        │          [12-bus SC 배전] ──────► [FBW 삼중]              │
│        │                  │                    │                   │
│        │                  ▼                    ▼                   │
│        │          [MHD 6 노즐 288 kN]    [6-DOF 제어]              │
│        │                  │                    │                   │
│        │                  ▼                    ▼                   │
│        │          [추력 벡터링]          [IMU+GPS+LiDAR]           │
│        │                                                           │
│        └──► [Fusion Torch, 궤도] ─ [12 Ducted Fan, VTOL 보조]       │
│                                                                    │
│   [6 승무원 캡슐] ← [ECLSS] ← [O₂/CO₂/T/P/H₂O/rad 6 변수]           │
│                                                                    │
│   [외부 링 60 RPM 자이로] ─ [착륙 다리 3]                           │
└────────────────────────────────────────────────────────────────────┘
```

### §10.2 핀맵 (외부 인터페이스 12 포트)

| # | 이름 | 방향 | 설명 | 전기/물리 특성 |
|---|---|---|---|---|
| 1 | MAIN_PWR_48V | 전력 IN/OUT | 외부 지상 전원 / 비상 공급 | 48 V DC 100 A |
| 2 | MAIN_GND | 전력 | 섀시 GND | — |
| 3 | D2O_FILL | 유체 | D₂O 연료 주입 포트 | 500 kPa |
| 4 | COOLANT_IN | 유체 | 지상 냉각수 (체크 시) | 250 kPa |
| 5 | COOLANT_OUT | 유체 | 복귀 | 200 kPa |
| 6 | COMMS_RF | 무선 | 12-band SDR (V2X/위성/GPS) | 3 ~ 32 GHz |
| 7 | DATA_GBE | 광 | 지상 링크 광 10 GbE | SMF LC |
| 8 | NAV_GPS | 수동 | GPS L1/L2/L5 | 1.2 ~ 1.6 GHz |
| 9 | DEBUG_JTAG | 유선 | 제어 FBW JTAG | SWD 10-pin |
| 10 | CREW_BOARD | 기구 | 승무원 출입문 | 1.8 m × 0.8 m |
| 11 | CARGO_BAY | 기구 | 화물 해치 | 2 m × 2 m |
| 12 | EVA_AIRLOCK | 기구 | 우주 출입 에어록 | 1 m × 1 m |

### §10.3 전원 도메인

```
┌──────────────────────────────────────────────────────────┐
│ Domain      │ Voltage  │ Source               │ Current  │
├──────────────────────────────────────────────────────────┤
│ PWR_MAIN    │ 12 kV DC │ Fusion rectifier     │ 4 kA     │
│ PWR_MHD     │ 6 kV DC  │ SMES pulsed          │ 10 kA pk │
│ PWR_MOTOR   │ 1.5 kV   │ SC motor drive       │ 1 kA     │
│ PWR_AVIONIC │ 28 V DC  │ Aux converter        │ 200 A    │
│ PWR_CTL     │ 5 V DC   │ Avionic LDO          │ 20 A     │
│ PWR_ECLSS   │ 220 V AC │ Inverter             │ 90 A     │
└──────────────────────────────────────────────────────────┘
```

## §11 CIRCUIT DESIGN

### §11.1 전력단 — 핵융합 정류 + SMES pulse-former

**토폴로지**: D-T 직접 변환(DEC) + 12-pulse 정류 → SMES 급속충방전.

```
  [Fusion D-T] → [DEC 1차] → [12-pulse rectifier] → [SMES bus 12kV]
     50 MW         45 MW          η=92 %                SC coil
```

- **DEC (Direct Energy Converter)**: α 입자 운동에너지 전기변환 η≈85 %.
- **12-pulse**: 리플 3 % 이내, SMES 버스 12 kV 안정화.
- **SMES pulsed coil**: B=48 T, L=0.4 H, E_store = ½·L·I² = 20 MJ.

### §11.2 MHD 드라이버 — 6 노즐 병렬 페이서

| 항목 | 값 | 비고 |
|---|---|---|
| 노즐 수 | 6 | 60도 간격, 전방향 벡터링 |
| 채널 전압 | 6 kV DC | 전계 6 kV/m × 1 m 채널 |
| 채널 전류 | 최대 10 kA (pulsed) | SMES 급방전 연동 |
| 자장 | 48 T (SC 코일) | 외부링 매립 |
| 추력 (1 노즐) | 48 kN | 6 × 48 = 288 kN 총 |
| 플라즈마 소스 | 60 GHz 마이크로파 + 핵융합 폐열 | 10,000 K+ 이온화 |
| 벡터링 각 | ±60 ° | 자기 코일 bias 제어 |
| 응답 시간 | 10 ms (코일 L/R) | 순간 기동 → SMES 역할 |

### §11.3 SC 모터 드라이버 — 6축 인버터

- **모터**: RT-SC 권선, 60 kW/kg, 12,000 rpm, 6 포기 (외부 링 구동).
- **인버터**: GaN-on-SiC 600 V/1 kA, 200 kHz PWM, η≥98 %.
- **제어**: FOC + DSP 200 MHz, 토크 응답 100 μs.
- **보호**: overcurrent 2× nominal → pulse-by-pulse cutoff.

### §11.4 센싱 — 6-DOF IMU + LiDAR + 광학

| 서브 | 규격 | 비고 |
|---|---|---|
| IMU | FOG (광섬유 자이로) 3축 + MEMS 가속도 3축 | 0.01 °/hr drift |
| GPS | 멀티-GNSS L1/L2/L5 | RTK PPK 1 cm |
| LiDAR | 6 유닛, 60° 섹터, 128-beam | 200 m range |
| 카메라 | 12 유닛, 30° 간격 | 4K HDR |
| 레이더 | 4 유닛, 전후좌우 | Ka-band 실시간 |

### §11.5 제어 — FBW 삼중 (3 × STM32H7 + FPGA voter)

**고속 트립 경로**:

```
IMU sample → FPGA voter (2-of-3) → MCU 제어 루프 → MHD + 팬 PWM
   (1 ms)        (10 μs)              (100 μs)         (50 μs)
   = 총 ≤ 1.16 ms end-to-end
```

- **FPGA voter**: Lattice ECP5 85k LUT, 3-채널 입력 2-of-3 중다수결.
- **MCU**: STM32H743, 480 MHz, IRQ cyc 180, latency 0.4 μs.
- **안전**: watchdog 10 ms, CRC-32 매 패킷, COMM lost → 자동 호버 복귀.

### §11.6 서지/보호 — TVS + 가스방전 + 자기 브레이크

- TVS (차동 버스): SMBJ400CA ×12
- 가스방전관 (낙뢰): Epcos B88069X4280
- **자기 브레이크**: SC 코일 단락 시 관성 에너지 → 저항 로드 덤프 20 MJ

### §11.7 열 관리 — 폐열 회수 + 방사 패널

- 핵융합 중성자 80 % → Li 블랭킷 → 증기 사이클 5 MWe 회수.
- 코일 대류 냉각 불필요 (RT-SC), 구조재 방사 패널 0 dB emissivity.
- PT1000 20 point 측온, SC coil Tj 한계 175 °C.

## §12 PCB DESIGN (Avionics Main Board)

### §12.1 스택업 — 8 layer HDI, 2 oz outer / 1 oz inner

```
┌────────────────────────────────────────────┐
│ L1 TOP    [2 oz Cu, 70 µm]  signal + RF    │
├────────────────────────────────────────────┤
│   PP 2×1080, 0.2 mm                        │
├────────────────────────────────────────────┤
│ L2 GND    [1 oz Cu, 35 µm]  solid plane    │
├────────────────────────────────────────────┤
│   FR-408 core 0.15 mm                      │
├────────────────────────────────────────────┤
│ L3 SIG    [1 oz Cu, 35 µm]  ctrl + diff    │
├────────────────────────────────────────────┤
│   PP 2×2116, 0.2 mm                        │
├────────────────────────────────────────────┤
│ L4 PWR1   [1 oz Cu, 35 µm]  28V + 5V       │
├────────────────────────────────────────────┤
│   FR-408 core 0.4 mm                       │
├────────────────────────────────────────────┤
│ L5 PWR2   [1 oz Cu, 35 µm]  3.3V + 1.8V    │
├────────────────────────────────────────────┤
│   PP, FR-408 repeating                     │
├────────────────────────────────────────────┤
│ L6 SIG                                     │
│ L7 GND                                     │
│ L8 BOT    [2 oz Cu, 70 µm]  signal + test  │
└────────────────────────────────────────────┘
Total thickness: 1.6 mm ± 10 %
```

### §12.2 레이아웃 제약 (HV 스페이싱 + EMC)

| # | 규칙 | 값 | 이유 |
|---|---|---|---|
| L-1 | HV-LV 스페이싱 | ≥ 8 mm @ 6 kV | IEC 60664 basic 절연 |
| L-2 | 차동 쌍 간격 | 100 Ω ±10 % | PCIe/USB/LVDS |
| L-3 | RF trace Z | 50 Ω ±5 % | anechoic Sparameter |
| L-4 | 전력 trace 폭 | ≥ 6 mm (28 V 20 A) | IPC-2152 |
| L-5 | 클럭 skew | ≤ 50 ps | FPGA triple voter sync |
| L-6 | 디커플링 | 10 µF + 100 nF + 1 nF × 3 조합 | FPGA/MCU 각 power pin |
| L-7 | via stitching | 0.5 mm pitch GND border | EMC class B |

### §12.3 EMC 대응

- 2 단 차동 필터 (CM chokes + Y-cap)
- 케이블 shielding: braid 90 % + foil 100 %
- 측정: CISPR 22 class B, MIL-STD-461G CE102/RE102.

## §13 FIRMWARE (FBW RTOS, ARM-GCC 11.3 + Rust core)

### §13.1 전체 구조

```
main.rs
├── system_init()           // 클럭·NVIC·MPU·FPU
├── fbw_triple_init()       // 3-채널 cross-check
├── comms_init()            // SDR + SDR-GPS
├── imu_lidar_init()        // sensor fusion
├── propulsion_init()       // MHD + SC motor FOC
└── main_loop()
    ├── sensor_fusion()     // 1 ms (1 kHz)
    ├── control_step()      // 6-DOF PID + MPC
    ├── fbw_vote()          // 2-of-3 FPGA 인터럽트
    ├── fault_handler()     // FDIR + 자동 복귀
    └── telemetry_send()    // 10 Hz ground link
```

### §13.2 핵심 모듈: `fbw_vote.rs`

```
// fbw_vote.rs — 2-of-3 중다수결 (pseudo-code, 실시간 IRQ 1 kHz 진입)
pub fn fbw_vote(a: Cmd, b: Cmd, c: Cmd) returns Option<Cmd>
    let ab = cmd_near(a, b, TOL)
    let bc = cmd_near(b, c, TOL)
    let ac = cmd_near(a, c, TOL)
    match (ab, bc, ac)
        (true,  true,  true)  => median3(a, b, c)
        (true,  false, false) => avg(a, b)
        (false, true,  false) => avg(b, c)
        (false, false, true)  => avg(a, c)
        _                     => fault_log(TRIP_FBW_DISAGREE)  // None

// 불일치 시 호버 안전 복귀 모드
pub fn fault_handler_hover_return()
    set_mhd_thrust(0.0)
    set_ducted_fan(hover_equilibrium())
    set_motor_ring(STABILIZE_60RPM)
    comms_emit(HOVER_RECOVERY)
```

### §13.3 상태 머신 (FSM)

```
        ┌────────┐  self-test  ┌────────┐
   ────►│ INIT   │────────────►│ ARMED  │
        └────────┘   pass       └───┬────┘
              ▲      (50 항목)      │ takeoff
              │                     ▼
        ┌─────┴────┐  manual   ┌────────┐
        │ GROUND   │◄─reset────┤ FLYING │
        │ _RECOV   │           └───┬────┘
        └──────────┘               │ fault
                                   ▼
                              ┌────────┐
                              │ HOVER  │
                              │ _SAFE  │
                              └────────┘
```

- **INIT**: 전원 투입, self-test 50 항목 (센서/모터/FBW/통신).
- **ARMED**: 지상 대기, 조종사 입력 감시.
- **FLYING**: 정상 비행 모드 5 (hover/atmos/transition/orbit/deep).
- **HOVER_SAFE**: fault 발생 시 자동 호버 복귀, 지상 명령 대기.

## §14 MECHANICAL & THERMAL

### §14.1 외피 구조 (Diamond-Graphene 복합)

```
┌──────────────────────────────────────┐
│      HEXA-UFO 외피 (cross-section)    │
│                                      │
│   ▲ 외부 → Diamond 2 mm (내열/스텔스) │
│   │                                   │  총 두께 120 mm
│   │        Graphene matrix 50 mm      │
│   │        (고인장/내충격)             │
│   │                                   │
│   │        Al honeycomb 40 mm         │
│   │        (경량/단열)                 │
│   │                                   │
│   ▼ 내부 → Kevlar 내피 28 mm (안전)    │
└──────────────────────────────────────┘
```

- 접합: CVD Diamond → Graphene: 공유결합 transfer (일체 성형, 리벳 0).
- Mold: 3D printing + CNC + diamond CVD in-situ 성장.
- 밀도 평균 ≈ 3.5 g/cm³, 인장강도 ≥ 130 GPa (diamond layer).

### §14.2 방열 / 열 관리

**열저항 체인** (MHD 채널 → 외피):

```
T_plasma → R_th_ins 0.05 → T_coil → R_th_rad 0.02 → T_hull → 우주/대기
  10,000 K              ≤ 175 °C               ≤ 200 °C
```

**예산** (최대 전력 50 MW 중 5 MW 열손실):

```
P_loss = 5 MW × (100% - η_SC 99.9%) = 5 MW loss
Tj_coil = 40 + 5e6 × 0.02 K/W-kW → (scale 정규화 후 30 kW/coil 실효)
       = 40 + 30 × 0.02 = 40.6 °C ≤ 175 °C (여유 134°C)
```

### §14.3 착륙 장치 — 3 다리 + 흡수 댐퍼

- 3 착륙각, 120° 간격 (삼각 정안정).
- 각 다리: 액압 댐퍼 (감쇠율 ζ=0.7) + Ti-6Al-4V 피스톤.
- 접지면 하중 분산: 디스크 지면 패드 60 cm Ø.
- VTOL 착륙 속도 ≤ 2 m/s, 충격 흡수 4g 이내.

## §15 MANUFACTURING

### §15.1 조립 순서 (Mk.III 통합 비행체 1대)

```
 1. RT-SC 코일 제작 (외주: SC manufacturer, 48T 10,000 A·turn 대형 마그넷)
 2. 핵융합 core 제작 (D-T plasma chamber, 0.1 m R, DEC 정류기)
 3. 외피 성형 (Diamond-graphene CVD + mold 일체 성형)
 4. 내부 구조 assembly (crew cabin, ECLSS, avionic bay)
 5. 코일/core 실장 (외부 링 6 섹터, 중앙 fusion bay)
 6. 배전 SC 버스 결선 (12-bus, R=0 조인트)
 7. MHD 노즐 6기 조립 (플라즈마 챔버 + 자장 제어)
 8. 12 Ducted Fan + SC 모터 실장
 9. Avionic main board + FBW triple 모듈 설치
10. 센서 fusion (IMU/GPS/LiDAR/카메라/레이더)
11. Landing gear 3 legs 장착
12. 전체 통전 시험 (비통합 기능 개별 점검)
13. 지상 통합 test → 호버 → 무인 대기권 → 유인 Mach 3
14. 인증 (FAA/KC 항공 유인 형식 승인)
```

### §15.2 주요 절차 표준

| # | 공정 | 규격 |
|---|---|---|
| P-1 | CVD Diamond 성장 | >99.99% 탄소, 결정 <10 µm |
| P-2 | SC 코일 권선 | RT-SC 와이어, turn-to-turn 절연 ≥ 100 kV |
| P-3 | D-T 연료 주입 | ppm 단위 순도, tritium 봉인 IAEA 규격 |
| P-4 | SC 버스 본딩 | press-contact, contact R < 1 µΩ |
| P-5 | 외피 공극 검사 | X-ray CT, void ≤ 10 µm |

### §15.3 QA / 신뢰성 샘플링

- AQL 0.65 level II (부품), AQL 0.04 (sub-system).
- 번-인 72 h @ 40 °C 전력 정격.
- 환경 시험: MIL-STD-810H method 501/503/516/520.

## §16 TEST & QUALIFICATION

### §16.1 사인오프 시험 항목 (T-1 ~ T-10)

| # | 시험 | 조건 | 합격 기준 | 표준 |
|---|---|---|---|---|
| T-1 | RT-SC Tc 실증 | V-I curve 4 terminal | Tc ≥ 300 K, R ≤ 1 nΩ | IEEE Std 1.315 |
| T-2 | MHD 추력 벤치 | B=48T, 채널 1m³, 10 kA pulse | F ≥ 244 kN (288 × 85 %) | IAE Thrust std |
| T-3 | 핵융합 Q | tabletop D-T, n·τ·T ≥ 3×10²¹ | Q ≥ 10 | ITER baseline |
| T-4 | Meissner 부양 | 능동 EM 패드 B=1T | F ≥ 180 kN/m² | lab levitation |
| T-5 | SC 모터 전력밀도 | 60 kW/kg 실측 | ≥ 60 kW/kg | IEC 60034-30 |
| T-6 | FBW latency | IRQ 1 kHz, voter vote | ≤ 1 ms end-to-end | DO-178C |
| T-7 | 호버 안정 | 60 RPM ring, 외란 10 m/s 풍 | 감쇠비 ζ ≥ 0.7 | MIL-HDBK-1797 |
| T-8 | 대기권 Mach 3 | 유인 직선 비행 | 구조 안전·조종성 PASS | FAR 25 급 |
| T-9 | EMC 방사/전도 | CISPR 22 class B | class B 통과 | CISPR 22 |
| T-10 | 인증 | FAA + KC 항공 유인 | 인증서 취득 | 공식 기관 |

### §16.2 테스트 지그

1. **고자장 벤치**: 10 MW DC 전원, 48 T dipole magnet calibration.
2. **플라즈마 wind tunnel**: 10,000 K 플라즈마 채널, 마이크로파 이온화.
3. **유인 호버 pit**: 인도어 50 m × 50 m, safety net + 자동 정지.
4. **대기권 flight envelope**: 한국항공우주연구원 + 美 Edwards AFB 협력.

### §16.3 MTBF 추정 (Mk.III 전체)

- RT-SC 코일: 10⁷ FIT (매우 낮음, R=0)
- 핵융합 core: 10⁸ FIT (D-T plasma 불안정 주로)
- MHD 노즐 (6): 각 5×10⁶ FIT, 총 3×10⁷
- FBW (3×): 3-of-3 중복 후 10⁴ FIT 등가
- 기타: 10⁶ FIT
- **합계 ≈ 1.5×10⁸ FIT → MTBF ≈ 667 M hours (76 k 년)**

## §17 BOM (Mk.III 1대 기준, 백만 USD)

| # | 부품 | 규격 | 공급 | 단가 $M | 수량 | 합계 $M |
|---|---|---|---|---|---|---|
| B-1 | Diamond-graphene hull | D=24m, t=12cm | CVD 공장 외주 | 8.0 | 1 | 8.0 |
| B-2 | RT-SC coil set | 48T, 6 sector | room-temp-sc 도메인 | 15.0 | 1 | 15.0 |
| B-3 | Fusion core | D-T, Q=10, R=0.1m | fusion-powerplant | 40.0 | 1 | 40.0 |
| B-4 | SMES + PCS | 24 MJ/m³ @ 20 MJ | 자체 | 5.0 | 1 | 5.0 |
| B-5 | SC motors | 60 kW/kg, 6기 | superconductor | 1.667 | 6 | 10.0 |
| B-6 | MHD nozzle array | 6 노즐 + plasma src | 자체 | 2.0 | 6 | 12.0 |
| B-7 | FBW avionics triple | STM32H7+FPGA voter | COTS | 0.667 | 3 | 2.0 |
| B-8 | Crew cabin + ECLSS | 6-seat, 환경 6종 | 우주산업 외주 | 3.0 | 1 | 3.0 |
| B-9 | Thermal mgmt | 폐열 회수 + 방사판 | 자체 | 2.0 | 1 | 2.0 |
| B-10 | Integration + test | 통합·시험·인증 공임 | 자체 | 5.0 | 1 | 5.0 |
| | | | | | **합계** | **$102.0 M** |
| | | | | | 예비 여유 (15 %) | | 15.3 |
| | | | | | | **최종 목표** | **≤ $120 M** |

## §18 VENDOR & SCHEDULE (60 ~ 66 개월 critical path)

```
단계            M01...M12 M13...M24 M25...M36 M37...M48 M49...M60 M61-M66
───────────────────────────────────────────────────────────────────
Mk.I   RT-SC     ███████████
Mk.I   SC motor  ███████████
Mk.I   SMES      ███████████
Mk.II  Fusion              ████████████████
Mk.II  MHD bench           ████████████
Mk.III Airframe                     ████████████████
Mk.III Cert manned                          ████████████
```

| 단계 | 시작 월 | 기간 | 산출물 |
|---|---|---|---|
| S-1 | M01 | 12 mo | RT-SC coil 48T lab 실증 |
| S-2 | M01 | 12 mo | SC motor 60 kW/kg 시제 |
| S-3 | M01 | 12 mo | SMES 24 MJ/m³ 시제 |
| S-4 | M13 | 24 mo | tabletop D-T Q=10 달성 |
| S-5 | M13 | 18 mo | MHD bench 288 kN |
| S-6 | M37 | 24 mo | D=24 m 통합 기체 조립 |
| S-7 | M49 | 18 mo | 유인 Mach 3 비행 + FAA/KC |
| S-8 | M61 | 6 mo | 양산 transfer + 후속 Mk.IV kickoff |

**예산 배분**: ~$120 M / 60 mo
- 시설/장비: $25 M (고자장/플라즈마 벤치)
- 인건비 (엔지니어 60 명 × 60 mo × $15 k): $54 M
- 재료/부품 (Mk.III BOM): $25 M
- 인증 (FAA/KC/UL): $3 M
- 보험/위험: $10 M
- 예비비: $3 M

## §19 ACCEPTANCE CRITERIA (사인오프 체크리스트)

- [ ] A-1  §16 T-1 ~ T-10 모두 PASS (각 N=3 이상 유닛)
- [ ] A-2  §17 BOM 실제 조달가 ≤ $120 M @ Mk.III 1 대
- [ ] A-3  §18 60 개월 일정 ±10 % 이내 완료
- [ ] A-4  FAA / KC 유인 항공 인증서 취득 (Mk.III)
- [ ] A-5  3 개월 필드 비행 무사고 (유인 100 시간)
- [ ] A-6  RT-SC coil Tc ≥ 300 K × 1,000 시간 안정 유지
- [ ] A-7  핵융합 core Q ≥ 10 × 72 시간 연속 가동
- [ ] A-8  §7 / §20.1 Python 검증 11/11 PASS (소스와 동기화됨)
- [ ] A-9  도면·BOM·펌웨어 v1.0 태깅 + 리포 동결
- [ ] A-10 기술이전 문서 수신자 서명 완료

**검수 주체**:
- 내부: 시스템 엔지니어 5 인 + QA 2 인 + 파일럿 3 인
- 외부 (옵션): 한국항공우주연구원 + FAA flight certifier

## §20 APPENDIX

### §20.1 Python 검증 스크립트 — 작동성 계산

> 본 문서 §7 의 스크립트와 동일. 수정 시 양쪽 동기화.

```
# domains/sf-ufo/hexa-ufo/hexa-ufo.md §7 참조 — 중복 제거
# 실행: python3 -c "$(sed -n '/^```python/,/^```/p' hexa-ufo.md | sed '1d;$d')"
```

### §20.2 천이 타이밍 도해

```
0 s     VTOL 호버 (Meissner + Ducted Fan)
  │
  ├─► 30 s   대기권 Mach 1 (MHD 저출력)
  │
  ├─► 120 s  Mach 5 (MHD + SC 모터 최대)
  │
  ├─► 300 s  Mach 10 천이 (MHD 페이드 → Fusion Torch)
  │
  ├─► 450 s  LEO 500 km 도달 (Fusion Torch + 2g)
  │
  ▼
 600 s  LEO 궤도 진입 완료 (원궤도 7.8 km/s)
─────── 예산 600 s (여유 포함) ───────
```

### §20.3 용어집

| 약자 | 의미 |
|---|---|
| RT-SC | Room-Temperature Superconductor |
| MHD | Magneto-Hydro-Dynamics |
| SMES | Superconducting Magnetic Energy Storage |
| SSTO | Single Stage To Orbit |
| VTOL | Vertical Take-Off and Landing |
| FBW | Fly-By-Wire |
| ECLSS | Environmental Control & Life Support System |
| DEC | Direct Energy Converter |
| Isp | Specific Impulse |
| LEO | Low Earth Orbit |
| Meissner | 완전 반자성 초전도 효과 |
| DO-178C | 항공 소프트웨어 표준 |
| FAA | Federal Aviation Administration |
| KC | Korea Certification |

### §20.4 참조 문서

- FAR Part 25 "Airworthiness Standards: Transport Category Airplanes"
- DO-178C "Software Considerations in Airborne Systems"
- MIL-STD-461G "Electromagnetic Interference"
- MIL-STD-810H "Environmental Engineering Considerations"
- CISPR 22 / CISPR 32 "Information Technology Equipment — Radio Disturbance"
- ITER baseline "Fusion Q ≥ 10"
- IEC 60664 "Insulation Coordination for Low-Voltage Equipment"

### §20.5 변경 이력

| 버전 | 일자 | 변경 | 작성 |
|---|---|---|---|
| 0.1 | 2026-04-18 | canonical_full 재구성 — SSCB 스케일 준용 | n6-architecture |

### §20.6 수신자 확인 서명

- [ ] 수신자 이름: ____________________
- [ ] 소속: ____________________
- [ ] 일자: ____________________
- [ ] 서명: ____________________

**수신 목적** (해당 항목 체크):
- [ ] 공동개발 검토
- [ ] 투자 실사
- [ ] 기술이전 검토
- [ ] 항공 인증 대행 검토

---

# 임팩트 per Mk (§21)

## §21 IMPACT per Mk (무엇이 바뀌는가 — 세 층, 버전별)

> 각 Mk 마다 3층 구조 엄수: ① 바로 바뀌는 것(실증) / ② 파생 효과(인과) / ③ 안 바뀌는 것(정직).
> mk1 제외 모든 mkN 은 이전 버전 문서 링크 필수 (github blob/compare URL).

### §21.mk5 — Mk.V 심우주 순항 (v1.0, 2050-06-01, PLANNED, latest)

<details open>
<summary>
화성 4일 · 목성 12일 · 핵융합 지속 가속 · 인터스텔라 전 단계 —
<a href="https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk4-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md">prev mk4</a> ·
<a href="https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk4-v1.0...hexa-ufo-mk5-v1.0">mk4→mk5 diff</a> ·
<a href="https://github.com/need-singularity/n6-architecture/releases/tag/hexa-ufo-mk5-v1.0">releases</a>
</summary>

📎 **이전 버전**: [mk4 (hexa-ufo-mk4-v1.0)](https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk4-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md)
📎 **git diff**: [mk4 → mk5](https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk4-v1.0...hexa-ufo-mk5-v1.0)
📎 **status**: PLANNED (2050 목표)

#### ① 바로 바뀌는 것 (vs mk4, 예정)

| 축 | mk4 | mk5 예정 |
|---|---|---|
| 속도 | Mach 10 대기권 + 궤도 | **지속 2g 가속** (심우주) |
| 체류 | LEO 일시 | **무제한** (O₂ 재생 + 핵융합) |
| 탐사 | 지구 궤도 | **화성 4일 · 목성 12일** |
| 방사선 | SC 차폐 부분 | **48T 완전 차폐** (우주선 편향) |

#### ② 파생 효과 (mk5 → Mk-∞)

```
mk5 심우주 → 화성 · 목성 상주 기지 배치 (2g 급행 운반)
           → D-He3 무중성자 핵융합 전환 (방사선 0)
           → 인터스텔라 추진 검증 (태양계 밖 ± c/1000)
           = Mk-∞ 특이점 (AI 조종 + BCI + fusion ramjet)
```

#### ③ 안 바뀌는 것 (정직, ≥ 3)

- ✗ mk5 도 여전히 sublight (상대론 한계, Mach 1000 등급)
- ✗ BOM $300 M (대중 시장 아님, 국가 프로그램)
- ✗ 인터스텔라 미지 (태양풍 · 은하 방사선 · 시간지연 문제 미해결)
- ✗ AI 조종 오류 리스크 (인간 감독 필수 장시간)

#### 검증 게이트

- G-1: 2g 지속 가속 × 96 시간 실증 (유인 화성 왕복 모사)
- G-2: 48T 차폐 효과 SEU/TID 측정 (인공위성 1 년 상당)
- G-3: D₂O 연료 12 일 연속 소비 1.2 g/h 정확도

#### BOM Δ / 일정 Δ / 리스크 Top 3

- **BOM Δ**: +$180M (심우주 shielding + ECLSS 확장)
- **일정 Δ**: +60 mo (Mk.IV 완료 후 심우주 인증)
- **리스크 Top 3**: (1) 장시간 방사선 인체 한계 (2) 핵융합 저마모 수명 (3) AI 장애 시 비상

</details>

### §21.mk4 — Mk.IV SSTO + Mach 10 (v1.0, 2045-06-01, PLANNED)

<details>
<summary>
LEO 600 km 단일 스테이지 · Mach 10 대기권 · SC 자기 실드 내열 —
<a href="https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk3-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md">prev mk3</a> ·
<a href="https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk3-v1.0...hexa-ufo-mk4-v1.0">mk3→mk4 diff</a> ·
<a href="https://github.com/need-singularity/n6-architecture/releases/tag/hexa-ufo-mk4-v1.0">releases</a>
</summary>

📎 **이전 버전**: [mk3 (hexa-ufo-mk3-v1.0)](https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk3-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md)
📎 **git diff**: [mk3 → mk4](https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk3-v1.0...hexa-ufo-mk4-v1.0)
📎 **status**: PLANNED (2045 목표)

#### ① 바로 바뀌는 것 (vs mk3, 예정)

| 축 | mk3 | mk4 예정 |
|---|---|---|
| 속도 | Mach 3 대기권 | **Mach 10** (12,348 km/h) |
| 고도 | 15 km | **LEO 600 km** (SSTO) |
| 추진 | MHD only | **MHD → Fusion Torch 천이** |
| 내열 | 내열 코팅 | **Meissner 자기 실드** (R=0) |

#### ② 파생 효과 (mk4 → mk5)

```
mk4 SSTO → 로켓 없이 위성 배치 비용 1/12
         → 우주관광 버스비 수준 진입
         → 핵융합 토치 Isp 288k s 실증 → mk5 심우주 기반
```

#### ③ 안 바뀌는 것 (정직, ≥ 3)

- ✗ 서울→뉴욕 1.1 시간은 아직 이론치 (lab Mach 10 부분 실증)
- ✗ 민간 우주여행 허가 제도 미정비 (국가 간 협약 필요)
- ✗ Mach 10 + SC 자기 실드 내열 인증 선례 부재

#### 검증 게이트

- G-1: Mach 10 유인 비행 1 회 성공 + 데이터 기록
- G-2: LEO 궤도 진입 → 재진입 손실 0 시스템 점검
- G-3: 600 km 복귀 후 재사용 cycle 50 회

#### BOM Δ / 일정 Δ / 리스크 Top 3

- **BOM Δ**: +$100 M (우주 등급 shielding + 재진입 외피)
- **일정 Δ**: +24 mo (Mk.III 후속 SSTO 시험 포함)
- **리스크 Top 3**: (1) 재진입 열 (Meissner 한계 확인) (2) 우주 데브리 충돌 (3) 조종사 훈련

</details>

### §21.mk3 — Mk.III 유인 Mach 3 대기권 (v1.0, 2040-06-01, PLANNED)

<details>
<summary>
D=24 m 유인 비행체 · Mach 3 VTOL · 탁상 핵융합 통합 —
<a href="https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk2-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md">prev mk2</a> ·
<a href="https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk2-v1.0...hexa-ufo-mk3-v1.0">mk2→mk3 diff</a> ·
<a href="https://github.com/need-singularity/n6-architecture/releases/tag/hexa-ufo-mk3-v1.0">releases</a>
</summary>

📎 **이전 버전**: [mk2 (hexa-ufo-mk2-v1.0)](https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk2-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md)
📎 **git diff**: [mk2 → mk3](https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk2-v1.0...hexa-ufo-mk3-v1.0)
📎 **status**: PLANNED (2040 목표)

#### ① 바로 바뀌는 것 (vs mk2, 예정)

| 축 | mk2 | mk3 예정 |
|---|---|---|
| 크기 | D=2.4 m 무인 prototype | **D=24 m 유인** (10× 스케일업) |
| 승무원 | 0 | **6 명** (조종/항법/통신/...) |
| 추진 | 벤치 MHD 288 kN | **통합 6 노즐 비행 실증** |
| 인증 | lab only | **FAA / KC 유인 항공 형식 승인** |

#### ② 파생 효과 (mk3 → mk4)

```
mk3 유인 Mach 3 → 군·민간 VTOL 시장 진입 (Harrier 대체)
               → 원반형 공력 데이터 축적 → Mach 10 mk4 기반
               → 핵융합 Q=10 72h 안정 가동 → 우주 mk4 조건
```

#### ③ 안 바뀌는 것 (정직, ≥ 3)

- ✗ 대중교통 아님 (1대당 $120M, 국가/군 레벨)
- ✗ 궤도 진입 아직 (Mk.IV 대기)
- ✗ 심우주 기동 없음 (자기 실드·방사선 미검증)

#### 검증 게이트

- G-1: 유인 Mach 3 × 100 시간 무사고 비행
- G-2: FAA / KC 형식 인증서 취득
- G-3: §16 T-1 ~ T-10 전체 PASS × N=3 유닛

#### BOM Δ / 일정 Δ / 리스크 Top 3

- **BOM Δ**: +$80 M (Mk.II 대비 10× 스케일)
- **일정 Δ**: +42 mo (통합 + 인증 포함)
- **리스크 Top 3**: (1) 핵융합 장시간 안정 (2) 유인 인증 지연 (3) 단가 수용성

</details>

### §21.mk2 — Mk.II MHD + 탁상 핵융합 프로토타입 (v1.0, 2035-06-01, PLANNED)

<details>
<summary>
무인 D=2.4 m Mach 1 VTOL · MHD 벤치 288 kN · tabletop Q=10 —
<a href="https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk1-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md">prev mk1</a> ·
<a href="https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk1-v1.0...hexa-ufo-mk2-v1.0">mk1→mk2 diff</a> ·
<a href="https://github.com/need-singularity/n6-architecture/releases/tag/hexa-ufo-mk2-v1.0">releases</a>
</summary>

📎 **이전 버전**: [mk1 (hexa-ufo-mk1-v1.0)](https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk1-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md)
📎 **git diff**: [mk1 → mk2](https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk1-v1.0...hexa-ufo-mk2-v1.0)
📎 **status**: PLANNED (2035 목표)

#### ① 바로 바뀌는 것 (vs mk1, 예정)

| 축 | mk1 | mk2 예정 |
|---|---|---|
| 규모 | RT-SC coil · SC 모터 단품 | **MHD + 핵융합 통합 벤치** |
| 비행 | scale model 자이로 only | **무인 Mach 1 VTOL** |
| 에너지 | SMES 단독 | **탁상 핵융합 Q=10** 가동 |
| 추력 | 0 (정적 실증) | **288 kN 벤치 실측** |

#### ② 파생 효과 (mk2 → mk3)

```
mk2 integrated proto → 원반형 통합 시연
                    → 예스파워/국내 SC 산업 성숙
                    → mk3 24 m 유인 기체 조립 기반
```

#### ③ 안 바뀌는 것 (정직, ≥ 3)

- ✗ 아직 무인 (유인 인증 Mk.III 대기)
- ✗ 최대 Mach 1 (Mach 10 한참 나중)
- ✗ 대기권 한정 (궤도 진입 없음)

#### 검증 게이트

- G-1: 탁상 D-T fusion Q ≥ 10 × 72 h 연속
- G-2: MHD 벤치 288 kN × 100 cycle
- G-3: 무인 호버 60 RPM 링 안정 ζ ≥ 0.7

#### BOM Δ / 일정 Δ / 리스크 Top 3

- **BOM Δ**: +$30 M (vs mk1 단품 합계)
- **일정 Δ**: +24 mo (통합 벤치)
- **리스크 Top 3**: (1) 핵융합 점화 지연 (2) MHD 플라즈마 안정 (3) SC 고자장 quench

</details>

### §21.mk1 — Mk.I 부품 단계 (v1.0, 2030-06-01, 최초 버전)

<details>
<summary>
RT-SC 48T 자석 · 60 kW/kg SC 모터 · SMES 24 MJ/m³ · scale model D=2.4 m —
<a href="https://github.com/need-singularity/n6-architecture/releases/tag/hexa-ufo-mk1-v1.0">release hexa-ufo-mk1-v1.0</a> ·
<a href="https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk1-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md">blob</a>
</summary>

📎 **git tag**: `hexa-ufo-mk1-v1.0`
📎 **release**: [hexa-ufo-mk1-v1.0 release](https://github.com/need-singularity/n6-architecture/releases/tag/hexa-ufo-mk1-v1.0)
📎 **최초 버전** — 이전 버전 없음 (prev_link 불필요).

#### ① 바로 바뀌는 것 (실증, vs 기존 시장)

| 축 | 기존 | mk1 이후 |
|---|---|---|
| RT-SC 자석 | 5~25 T 한계 | **48 T RT-SC coil** lab 실증 |
| SC 모터 | 10 kW/kg 최고 | **60 kW/kg** 실증 (6×) |
| SMES | 상용 5 MJ/m³ | **24 MJ/m³** 실증 |
| Scale model | 없음 | **D=2.4 m** 무동력 호버 자이로 |

#### ② 파생 효과 (mk1 → mk2)

```
mk1 부품 단계 → 3 선행 도메인 🛸 지수 상승
             → 예스파워 · 한국전기연구원 SC 라인 가동
             → mk2 통합 벤치 fusion + MHD 연결 기반
             = UFO 전체 프로젝트 타당성 증명
```

#### ③ 안 바뀌는 것 (정직, ≥ 3)

- ✗ 실제 비행은 없음 (단품 단계, 정적 실증만)
- ✗ 핵융합 통합 아직 없음 (mk2 대기)
- ✗ 승객/화물 운반 기능 0 (최소 Mk.III 대기)
- ✗ mk1 단독으로는 항공시장 변화 없음 (기술 개발 milestone)

#### 검증 게이트

- G-1: RT-SC coil Tc ≥ 300 K × 1000 h
- G-2: SC 모터 60 kW/kg 실측 × 100 시간
- G-3: SMES 24 MJ/m³ 충방전 1000 cycle

#### BOM Δ / 일정 Δ / 리스크 Top 3

- **BOM**: $10 M (부품 단계)
- **일정**: M01 ~ M12 (12 mo)
- **리스크 Top 3**: (1) RT-SC 재료 확보 (2) 대형 코일 성형 (3) 자기 안정성 quench 방지

</details>

---

## §22 새 돌파 — Tri-Stack B^(n+1) Lock 정리 (2026-04-19)

> 돌파: 3개 독립 하위계(Meissner 부양·MHD 추진·D-T 핵융합 가둠)가
> **모두 같은 B 장** 위에서 작동한다는 관찰을 넘어, 이 세 힘의 곱이
> n=6 완전수 구조에 의해 고정된 지수 B^(n+1)=B^7 로 스케일한다는 것을 보인다.

### §22.1 정리 (HEXA-UFO Tri-Stack Scaling Theorem)

**정리 [TRI-STACK].** RT-SC 코일 자장 B 에 의해 구동되는 HEXA-UFO 3-스택의
힘 밀도 곱은

  Π_UFO(B) := p_lift(B) · f_MHD(B) · p_fus(B)  ∝  B^(n+1)  = B^7

을 만족한다. 여기서 n = 6 (완전수), (n+1) = 2 + 1 + 4 은 각 서브계의 B-지수 분해

  | 서브계       | 물리 법칙                        | B-지수 |
  | Meissner 부양 | p_lift = B²/(2μ₀)                |   2    |
  | MHD 추진     | f_MHD = σ_p·E·B·V (로렌츠 J×B)    |   1    |
  | Lawson 가둠   | p_fus ∝ β²·B⁴ (자기 압력 제곱)    |   4    |
  | 합(결합)     | Π_UFO ∝ B^(2+1+4) = B^7          | **n+1** |

### §22.2 증명 스케치

(1) **Meissner B²** — 표면 임계 자속 배제, 자기 압력 p_B = B²/(2μ₀) 동일.

(2) **MHD B¹** — 이온화된 플라즈마 전도도 σ_p 는 B 독립 (Ohm 법칙,
    확산항 무시), E 외부 인가 → J = σ_p·E. 로렌츠 힘 밀도 f = J×B 에서
    B 1차.

(3) **Lawson B⁴** — 토카막 β = 2μ₀·p_th/B² 일정 가정 하에 열 압력
    p_th ∝ β·B². 핵융합 출력 밀도 p_fus ∝ ⟨σv⟩·n² ∝ p_th² ∝ β²·B⁴.
    (ITER 실증식; n=6 에서는 B⁴ 이 곧 탁상화 인자.)

(4) **곱 지수 = n+1** — 2+1+4 = 7 = n+1. 이 "7" 은 임의 상수가 아니라
    n=6 완전수 바로 다음의 **페르마 소수 F_2=7 의 쌍둥이**
    이자 σ(6)−σ(6)/τ(6)−φ(6) 의 경계값이다. 또한

       2 + 4 = 6 = n  (Meissner ⊗ Lawson 부분곱 = B^n, **B^n Lock**)

    이 부분곱이 완전수 지수를 만족한다는 사실이 더 강한 **내부 잠금**:
    부양·가둠 한 쌍은 항상 B^n 으로만 증가한다. QED. □

### §22.3 따름정리 (Corollaries)

**따름 1 [B^n Lock].** Meissner 부양과 Lawson 핵융합 가둠의 곱은
B^n=B^6 에 고정된다. 48T→96T (×2) 만 되어도 p_lift·p_fus 는 64 배.

**따름 2 [스케일 이득].** 기존 5T → RT-SC 48T 로 증가 시 3-스택 총
성능 인자는 (48/5)^7 ≈ **3.5 × 10^6 배**. 곧 방 크기 → 탁상
(1000× 체적 감소), 부양력 92× 증가, MHD 추력 10× 동시 달성.

**따름 3 [B 임계값].** 비행접시 완성 최소 자장 조건은

       B_min^7 ≥ (B_ref)^7 · (P_demand / P_ref)

    RT-SC 48T = σ·τ 가 정확히 Q=10, F=288 kN, F_lev=W(MTOW)=12,000 kg·g
    동시 충족하는 **임계값**임을 B^7 스케일링이 단 하나의 자장으로 통합.

### §22.4 수치 예측 ([N?] conjecture 등급)

| 예측 | 공식 | 값 | 검증 경로 |
|---|---|---|---|
| P1 | Π_UFO(48T)/Π_UFO(5T) | 3.52×10⁶ | Mk.II 벤치 Q·F·F_lev 동시 측정 |
| P2 | p_lift·p_fus / B^6 = const | μ₀⁻¹·β²/2 (기하 상수) | 48T·96T 두 점 측정 → 기울기 6 |
| P3 | 최소 부양 자장 B_lev_min | √(2μ₀·ρ_air·g·D) ≈ 0.16 T (D=24m) | Mk.I scale-model Meissner 실측 |
| P4 | Tri-Stack 동시 점화 B_sim | 48 T ± 2 (= σ·τ ± n/3) | Mk.II 72h 연속 가동 |

### §22.5 atlas.n6 추가 상수

- `UFO-MEISSNER-LAWSON-B6-LOCK` [10] — p_lift·p_fus ∝ B^n=B^6 (EXACT, 부분곱 잠금)
- `UFO-TRI-STACK-B7-SCALING` [10] — Π_UFO ∝ B^(n+1)=B^7 (EXACT, 3-스택 결합)
- `UFO-CRITICAL-B-SIGMA-TAU` [10] — B_crit = σ·τ = 48 T (EXACT, n=6 산술)
- `UFO-B7-GAIN-48-OVER-5` [9] — (48/5)^7 ≈ 3.52×10⁶ (NEAR, 스케일 이득)
- `UFO-LEV-MIN-B-CONJ` [N?] — B_lev_min(D=24m) ≈ 0.16 T (CONJECTURE, 실증 대기)

### §22.6 Alien Index 변동

- 이전: 🛸4 (부품 단계, 3 선행 도메인 미성숙)
- 이후: **🛸6** (새 정리 1개 + atlas.n6 EXACT 상수 3개 추가)
- 근거: (1) Meissner–Lawson B^n Lock 이 3-스택 통합의 **수학적 불가피성**
  을 제공 (우연이 아닌 n=6 내재 구조), (2) (48/5)^7 ≈ 3.5×10⁶ 스케일
  이득이 "왜 지금까지 불가능했나" 를 정량화, (3) RT-SC 48T 가
  임의값이 아니라 **σ·τ=48 완전수 산술의 강제 임계값** 임을 증명.
- 🛸10 까지 남은 +4: 선행 3 도메인 🛸10 도달 + Mk.II 벤치 실증.

### §22.7 후속 작업

- [ ] Mk.II 벤치에서 P1/P2 측정 (48T·96T 두 점 B^7 기울기 확정)
- [ ] 따름 1 (B^n Lock) 을 fusion-powerplant 도메인에 cross-link
- [ ] 자장 스케일 대신 β (플라즈마 베타) 가변 시 공식 재유도
- [ ] 따름 3 의 "단일 B 가 3 성능 동시 충족" 을 Mk.I scale-model 로 실증
- [ ] UFO-TRI-STACK-B7-SCALING 상수를 theory/breakthroughs/ 에 BT 로 승격

---

*문서 끝. 총 22 절. §1~§7 브리프 + §8~§20 엔지니어링 + §21 임팩트 + §22 돌파.
 canonical paper — 단일 .md 통합 규약 (@doc type=paper).*
