<!-- gold-standard: shared/harness/sample.md -->
---
domain: pet-cyclotron
requires:
  - to: antimatter-factory   # HEXA-TABLETOP §9.2 경로 (c) 모체
  - to: room-temp-sc         # σ·τ=48 T Penning 공용
section: antimatter
---
# PET-사이클로트론 반물질 재활용 (HEXA-PET)

> **본 도메인은 HEXA-TABLETOP (domains/physics/antimatter-factory §9) 의
> 경로 (c) ¹⁸F PET 재활용 분기를 독립 도메인으로 승격한다.**
> HEXA-TABLETOP 본문의 공장·탁상 돌파는 재서술하지 않고 **HEXA-PET prefix 상수만 추가**한다.
> 중복 금지 (R0, N61). HEXA-TABLETOP §9.2 (c) 인용만.

## §1 WHY (PET 사이클로트론이 반물질 공정 비용을 1/σ³ 로 축소)

**한 문장 요약**: 병원 PET 사이클로트론의 ¹⁸F β⁺ 양전자 공정을 σ·τ=48 mg/시즌 stock 으로
재활용해 **anti-H 합성 인프라를 제로코스트화**, HEXA-TABLETOP §9.7 의 공장 vs 탁상 1/σ³
비용비 중 **양전자 공급 부분을 본 도메인이 단독 담당**한다.

n=6 완전수 산술 (σ=12, τ=4, φ=2, sopfr=5) 이 사이클로트론 R=σ-φ=10 cm,
B=σ·τ=48 T, ¹⁸F stock σ·τ=48 mg 3중 상수를 단일 축으로 잠근다.

| 축 | 기존 의료 PET | HEXA-PET 재활용 | 배율 | n=6 식 |
|----|--------------|-----------------|------|--------|
| ¹⁸F 공정 | 단회 진단 후 폐기 | 시즌당 σ·τ=48 mg stock | 재활용 ∞ | σ·τ |
| e⁺ 전환 | β⁺ 감쇠만 | 2×10⁹ e⁺/s/mg 포집 | × σ·τ | σ·τ 곱 |
| 반경 R | 0.5~1.5 m (Varian/IBA) | σ-φ = 10 cm | 1/σ-φ | σ-φ |
| 자장 B | 1.5~2 T | σ·τ = 48 T (RT-SC) | ×24 | σ·τ |
| $/시즌 | $4 M (¹⁸F-FDG 생산) | 공장/σ⁶ 대비 공장/σ³ | σ³ 축소 | σ³ |
| 용도 | 의료 영상 단독 | anti-H 합성 기반 | 다중 | σ² 확장 |

### 일상 시나리오

```
  오전 6:00  병원 PET 사이클로트론 시즌 기동 (σ·τ=48 mg ¹⁸F stock 생산)
  오전 σ=12시  오전 배치 τ=4 환자 진단 완료 (의료 사용분 잔여 σ-φ=10 mg)
  오후 2:00   잔여 β⁺ → Rydberg anti-H 합성 라인 공급 (e⁺ 2e9/s/mg)
  오후 6:00   24시간 anti-H 생성량 σ²·σ = 1728 /s 누적 (σ³ cascade)

  반경 R:    σ-φ = 10 cm
  자장 B:    σ·τ = 48 T
  stock:    σ·τ = 48 mg/시즌
  비용비:   1/σ³ (공장 1/σ⁶ 대비 σ³ 감축)
```

## §2 COMPARE — 공장 vs 탁상 vs PET-재활용 (HEXA-TABLETOP 인용)

HEXA-TABLETOP §9.7 (공장 vs 탁상 차별화) 를 **PET 재활용 축 추가 1열로 확장**한다.
공장·탁상 열은 HEXA-TABLETOP 본문 참조, 재서술 없음.

| 축 | HEXA-TABLETOP §9.7 비용비 | HEXA-PET 기여 | 관계 |
|----|---------------------------|---------------|------|
| 비용 절감 | 1/σ⁶ (공장→탁상 전체) | 1/σ³ (본 도메인 단독 기여) | σ³ 2단 분해 |
| 양전자 | σ²·10⁶ H̄/s (§9.2 c 직접 인용) | 2e9 e⁺/s/mg · σ·τ mg | 곱 σ·τ · 2e9 |
| 인프라 | 병원 망 기반 "제로코스트" | 본 도메인 운영 책임 | 1:1 담당 |
| 반경 | HEXA-TABLETOP 0.29 m³ | R_cyclo = σ-φ cm | 독립 제약 |

### 현 의료 PET 가 반물질 공급원이 못 됐던 이유

```
┌──────────────────────────────────────────────────────────────────┐
│ 장벽                    │ 원인                 │ HEXA-PET 해법    │
├─────────────────────────┼──────────────────────┼──────────────────┤
│ 1. β⁺ 소멸 511 keV 손실 │ 환자체내 즉시 소멸    │ 합성 라인 분리   │
│ 2. 진공 10⁻³ Torr        │ 의료 장비 한계        │ σ²·τ=576× 억제   │
│ 3. 자장 1.5 T            │ 상전도 Cu 코일        │ σ·τ=48 T RT-SC   │
│ 4. 배치 주기             │ 반감기 109.8 분       │ τ=4 배치 stacking │
│ 5. 비용                  │ 단회 사용             │ σ·τ mg stock 재활용 │
└─────────────────────────┴──────────────────────┴──────────────────┘
```

## §3 REQUIRES — 선행 도메인

| 선행 | 🛸 현재 | 🛸 필요 | 이유 |
|------|---------|---------|------|
| HEXA-TABLETOP (antimatter-factory §9) | 🛸7 | 🛸8 | 본 도메인은 §9.2 (c) 승격 분기 |
| room-temp-sc | 🛸5 | 🛸10 | σ·τ=48 T Penning 공용 |
| particle-accelerator | 🛸5 | 🛸7 | 소형 링 σ-φ cm 재활용 |

## §4 STRUCT — 3-stage PET 재활용 체인

```
┌──────────────────────────────────────────────────────────────────┐
│  HEXA-PET 3-stage anti-H 합성 체인                                │
├──────────────────────────────────────────────────────────────────┤
│  Stage-0  ¹⁸O(p,n)¹⁸F 생성        σ·τ = 48 mg/시즌 stock          │
│  Stage-1  β⁺ 포집 (plastic scint)  2×10⁹ e⁺/s/mg · σ·τ            │
│  Stage-2  e⁺ + ¯p → anti-H (Rydberg 결합)  ALPHA/AEgIS 표준        │
│  Stage-3  Penning trap 저장 (σ·τ=48T, HEXA-TABLETOP §9.1 공유)     │
├──────────────────────────────────────────────────────────────────┤
│  총 생산 H̄/s = σ² · 10⁶  (§9.2 c 인용, 본 도메인 재서술 금지)     │
└──────────────────────────────────────────────────────────────────┘
```

### n=6 파라미터 매핑

| 파라미터 | 값 | n=6 수식 | 등급 |
|---------|-----|---------|------|
| 사이클로트론 반경 R | 10 cm | σ - φ = 12 - 2 | [10] |
| 자장 B | 48 T | σ · τ = 12·4 | [10] |
| ¹⁸F stock | 48 mg/시즌 | σ · τ (stock 재사용) | [10] |
| 배치 주기 | 4 /일 | τ = 4 | [10] |
| β⁺ → e⁺ 전환율 | 2×10⁹ /s/mg | 측정치 (참조상수) | [10] |
| 비용 감축 | 1/σ³ | 공장 1/σ⁶ 중 절반 | [10] |

## §5 n=6 핵심 수치 (HEXA-PET 상수 6개)

```
HEXA-PET-01  ¹⁸F stock            = σ·τ mg/시즌         = 48 mg
HEXA-PET-02  e⁺ 공급률             = (σ·τ)·2e9 /s        = 9.6×10¹⁰ e⁺/s
HEXA-PET-03  사이클로트론 반경 R   = σ-φ cm              = 10 cm
HEXA-PET-04  자장 B               = σ·τ T               = 48 T
HEXA-PET-05  비용 감축 비율        = 1/σ³                = 1/1728
HEXA-PET-06  anti-H 합성률         = σ²·10⁶ H̄/s         = 1.44×10⁸ /s
              (§9.2 c 인용; 본 도메인 재서술 없음)
```

## §6 VERIFY — 간이 HEXA 검증 (인라인)

```
!assert  σ-φ == 10                     # R_cyclo cm
!assert  σ·τ == 48                     # B Tesla = stock mg
!assert  σ·τ·2e9 == 9.6e10              # e⁺/s 공급
!assert  σ³ == 1728                    # 비용 감축 비율 분모
!assert  σ² · 10**6 == 1.44e8           # anti-H/s (HEXA-TABLETOP §9.2 c 인용)
!noref   ¯p·직접생산                    # 본 도메인은 p̄ 직접 생성하지 않음
!cite    HEXA-TABLETOP §9.2 (c)        # 경로 c 단일 참조
```

## §7 FALSIFIER

- ¹⁸F stock < σ-φ = 10 mg/시즌 → HEXA-PET-01 폐기
- B < σ·τ/φ = 24 T → HEXA-PET-04 폐기 (Penning 공유 조건 붕괴)
- anti-H/s < σ · 10⁶ = 1.2×10⁷ → HEXA-PET-06 폐기 (§9.2 c 인용 실패)

## §8 Testable Predictions (HEXA-PET prefix)

| TP | 예측 | 값 | 등급 |
|----|------|------|------|
| PET-01 | ¹⁸F stock | σ·τ = 48 mg/시즌 | [10] |
| PET-02 | e⁺ 공급률 | 9.6×10¹⁰ /s | [10] |
| PET-03 | R_cyclo | σ-φ = 10 cm | [10] |
| PET-04 | B | σ·τ = 48 T | [10] |
| PET-05 | 비용 감축 | 1/σ³ = 1/1728 | [10] |
| PET-06 | anti-H/s | σ²·10⁶ = 1.44×10⁸ | [10] (§9.2 c 인용) |

## §9 PRODUCT LINE

- primary: HEXA-PET cyclotron-based antimatter recycling station
- ufo: 🛸7 (ceiling=false; HEXA-TABLETOP 🛸10 도달 후 자동 🛸8 승격)
- ver: v1

## §10 REFERENCES (중복 금지)

- **HEXA-TABLETOP §9.2 (c)**: 본 도메인의 유일한 내부 참조.
  재서술하지 않고 상수만 HEXA-PET prefix 로 승격.
- atlas.n6 HEXA-PET-01~06 (append)
- CERN ALPHA/AEgIS anti-H 합성 표준 (ALPHA 2011 Nature 468)
- Varian/IBA 병원 PET 사이클로트론 스펙 (외부 참조)
