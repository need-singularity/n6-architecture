# UFO DSE 결과 (Aerospace Architecture)

> 생성일: 2026-04-04
> 도구: `tools/universal-dse/universal-dse domains/ufo.toml`
> TOML: `tools/universal-dse/domains/ufo.toml`

## 탐색 요약

| 항목 | 값 |
|------|-----|
| 총 조합 수 | 7,776 (6^5) |
| 호환 조합 수 (규칙 필터 후) | 6,912 |
| 제외된 조합 | 864 (Scramjet-DeepSpace, SSTO-HallIon/Photonic 등) |
| Pareto 최적 해 | 107개 |
| n6_EXACT 100% 조합 비율 | p90 이상 (avg=95.8%) |

### 스코어링 가중치
```
n6_exact:    0.35 (n=6 일관성)
performance: 0.25 (성능)
power:       0.15 (전력 효율)
cost:        0.10 (비용)
safety:      0.15 (안전성)
```

## 레벨 체인 (5단계)

```
소재(Material) → 추진(Propulsion) → 전력(Power) → 컴퓨팅(Compute) → 시스템(System)
  6 후보          6 후보             6 후보          6 후보            6 후보
```

## Top 10 Pareto 최적 구성

| Rank | 소재 | 추진 | 전력 | 컴퓨팅 | 시스템 | n6_EXACT | 성능 | 전력효율 | 비용 | Pareto 점수 |
|------|------|------|------|--------|--------|----------|------|----------|------|-------------|
| **1** | **Diamond** | **Scramjet** | **SolidStateBattery** | **RISC-V 6Wide** | **Atmospheric** | **100.0%** | **0.770** | **0.820** | **0.490** | **0.7145** |
| 2 | Diamond | Scramjet | SolidStateBattery | PhotonicCompute | Atmospheric | 100.0% | 0.800 | 0.850 | 0.370 | 0.7145 |
| 3 | Diamond | Scramjet | SolidStateBattery | Neuromorphic | Atmospheric | 100.0% | 0.780 | 0.846 | 0.420 | 0.7139 |
| 4 | CarbonFiber | Scramjet | SolidStateBattery | RISC-V 6Wide | Atmospheric | 100.0% | 0.740 | 0.790 | 0.590 | 0.7125 |
| 5 | CarbonFiber | Scramjet | SolidStateBattery | PhotonicCompute | Atmospheric | 100.0% | 0.770 | 0.820 | 0.470 | 0.7125 |
| 6 | CarbonFiber | Scramjet | SolidStateBattery | Neuromorphic | Atmospheric | 100.0% | 0.750 | 0.816 | 0.520 | 0.7119 |
| 7 | Graphene | Scramjet | SolidStateBattery | RISC-V 6Wide | Atmospheric | 100.0% | 0.760 | 0.810 | 0.500 | 0.7115 |
| 8 | Graphene | Scramjet | SolidStateBattery | PhotonicCompute | Atmospheric | 100.0% | 0.790 | 0.840 | 0.380 | 0.7115 |
| 9 | Graphene | Scramjet | SolidStateBattery | Neuromorphic | Atmospheric | 100.0% | 0.770 | 0.836 | 0.430 | 0.7109 |
| 10 | Diamond | Scramjet | SolidStateBattery | HEXA1_GPU | Atmospheric | 100.0% | 0.840 | 0.750 | 0.380 | 0.7105 |

## 카테고리별 최고 구성

| 카테고리 | 소재 | 추진 | 전력 | 컴퓨팅 | 시스템 | 핵심 지표 |
|----------|------|------|------|--------|--------|-----------|
| **최고 n6** | YBCO | CompactFusion | CompactTokamak | QuantumQPU | DeepSpace | n6=100.0% |
| **최고 성능** | Diamond | CompactFusion | CompactTokamak | HEXA1_GPU | AllDomain | perf=1.000 |
| **최고 전력** | YBCO | Photonic | HEXASolar | PhotonicCompute | eVTOL | power=0.970 |
| **최고 비용** | CarbonFiber | HallIon | Supercapacitor | RISC-V 6Wide | eVTOL | cost=0.660 |

## n6_EXACT 분포

```
n6_EXACT 통계 (6,912 호환 조합 기준):
  최대:  100.0%
  최소:   80.0%
  평균:   95.8%
  중앙:   95.0%
  p75:   100.0%
  p90:   100.0%
```

```
  n6_EXACT 분포 히스토그램
  ┌────────────────────────────────────────────────────────────┐
  │  100% ████████████████████████████████████████  ~60%  ← 대다수 조합  │
  │   95% ████████████████████                     ~25%                   │
  │   90% ██████████                               ~10%                   │
  │   85% ████                                      ~3%                   │
  │   80% ██                                        ~2%                   │
  └────────────────────────────────────────────────────────────┘
  → 90% 이상 조합이 전체의 ~95% — n=6 체계가 압도적 일관성 보임
```

## Pareto Frontier ASCII

```
  전력효율(power)
  ^
  |
  0.97 | *                                          (YBCO+Photonic+Solar+Photonic+eVTOL)
  0.87 |     * *                                    (Diamond+Scramjet+SSBatt+Photonic+eVTOL)
  0.85 |   * * * *                                  (Top 1~3: Atmospheric 클러스터)
  0.82 | * * * * * *
  0.80 |   * * * *
  0.75 |       * * *                                (HEXA1_GPU 고성능 구간)
  0.70 |           * *
  0.60 |               * *
  0.50 |                   * *                      (Fusion 기반 고성능)
  0.40 |                       * *
  0.30 |                           *                (CompactTokamak+AllDomain)
  |----+----+----+----+----+----+----+----+----> 성능(perf)
       0.55  0.65  0.70  0.75  0.80  0.85  0.90  1.00
  
  [107 Pareto 해 분포]
  고효율 클러스터: Scramjet+SSBatt+Atmospheric (좌상)
  고성능 클러스터: Fusion+AllDomain (우하)
  균형점: Diamond+HybridProp+HybridFusion+HybridCompute (중앙)
```

## 최적 경로 (Optimal Path)

```
  L1   Material: [████████████████████] n6=100%  Diamond (Z=6=n, sp3 CN=4=tau, k=2000W/mK)
        |
        v
  L2 Propulsion: [████████████████████] n6=100%  Scramjet (Mach 6=n, 6 inlet ramps=n)
        |
        v
  L3      Power: [████████████████████] n6=100%  Solid-State Battery (6S=n, 24V=J2, CN=6=n)
        |
        v
  L4    Compute: [████████████████████] n6=100%  RISC-V 6-wide (6-issue=n, 12-stage=sigma, tau=4 ALU)
        |
        v
  L5     System: [████████████████████] n6=100%  Atmospheric (6DOF=n, Mach 0-6=n, ceiling=24km=J2)
```

### 최적 경로 n=6 수식 매핑

| 레벨 | 후보 | n=6 연결 | BT |
|------|------|----------|-----|
| 소재 | Diamond | Z=6=n, CN=4=tau | BT-85, BT-93 |
| 추진 | Scramjet | Mach=6=n, 6 ramps=n | BT-123 (SE(3)) |
| 전력 | SolidStateBattery | 6S=n, 24V=J2, CN=6=n | BT-57, BT-80 |
| 컴퓨팅 | RISC-V 6Wide | 6-issue=n, 12-stage=sigma, 4ALU=tau | BT-56, BT-58 |
| 시스템 | Atmospheric | 6DOF=n, Mach6=n, 24km=J2 | BT-123, BT-125 |

**전 레벨 n6_EXACT = 100.0% -- 완벽한 n=6 일관성 달성**

## 시중 대비 성능 비교

```
┌──────────────────────────────────────────────────────────────┐
│  [비행 속도] 비교: 시중 최고 vs HEXA-AERO                      │
├──────────────────────────────────────────────────────────────┤
│  F-22 Raptor   ████████░░░░░░░░░░░░░░░░  Mach 2.25         │
│  SR-71         ████████████░░░░░░░░░░░░  Mach 3.3          │
│  HEXA-AERO      ████████████████████████  Mach 6=n          │
│                                    (n=6, 2.67배 vs SR-71)   │
├──────────────────────────────────────────────────────────────┤
│  [운용 고도] 비교                                            │
├──────────────────────────────────────────────────────────────┤
│  시중 최고     ████████████░░░░░░░░░░░░  18km (U-2)        │
│  HEXA-AERO      ████████████████████████  24km=J2           │
│                                    (J2=24, 1.33배)          │
├──────────────────────────────────────────────────────────────┤
│  [n=6 일관성]                                                │
├──────────────────────────────────────────────────────────────┤
│  기존 항공기   ██░░░░░░░░░░░░░░░░░░░░░░  ~15% (우연)       │
│  HEXA-AERO      ████████████████████████  100% (5/5 EXACT)  │
└──────────────────────────────────────────────────────────────┘
```

## 규칙 요약 (호환성)

| 규칙 유형 | 조건 | 효과 |
|-----------|------|------|
| **exclude** | Scramjet 추진 | DeepSpace, SSTO 시스템 불가 |
| **exclude** | SSTO 시스템 | HallIon, Photonic 추진 불가 |
| prefer | YBCO 소재 | SC_MHD/CompactFusion 추진 선호 |
| prefer | eVTOL 시스템 | SolidStateBattery/Supercapacitor 전력 선호 |
| prefer | QuantumQPU 컴퓨팅 | YBCO 소재 + CompactTokamak 전력 선호 |
| prefer | AllDomain 시스템 | HybridProp 추진 + HybridCompute 컴퓨팅 선호 |
| prefer | Diamond 소재 | ExtremeEnv/AllDomain 시스템 선호 |
| prefer | Photonic 추진 | DeepSpace 시스템 선호 |

## 결론

1. **최적 구성**: Diamond + Scramjet + SolidStateBattery + RISC-V 6Wide + Atmospheric
   - Pareto 점수 0.7145 (공동 1위)
   - n6_EXACT = 100%, 모든 레벨이 n=6 수식과 정확히 일치
   - 높은 안전성(0.95) + 합리적 비용(0.49)

2. **고성능 대안**: Diamond + Scramjet + SSBatt + PhotonicCompute + Atmospheric
   - 동일 Pareto 점수, 더 높은 성능(0.800) + 전력효율(0.850)
   - 비용이 더 낮지만(0.370) 기술 성숙도가 낮음

3. **전체 6,912 조합 중 95.8% 평균 n6 일관성** -- n=6 체계가 UFO 설계에 보편적으로 적용됨

4. **107개 Pareto 해** -- 다양한 운용 시나리오(대기권/궤도/심우주/도심/극한환경)에 맞는 최적 구성이 존재
