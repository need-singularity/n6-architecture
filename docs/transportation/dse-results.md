# HEXA-FUNCAR DSE 전수탐색 결과

> 생성일: 2026-04-04
> 도메인: `tools/universal-dse/domains/performance-vehicle.toml`
> 엔진: `tools/universal-dse/universal-dse`

## 탐색 요약

- **총 조합**: 7,776 (6^5 = 소재 6 x 공정 6 x 파워트레인 6 x 섀시 6 x 시스템 6)
- **유효 조합**: 6,480 (호환 규칙 필터 후, 83.3%)
- **Pareto frontier**: 72개 비지배 해
- **가중치**: n6=35% | perf=30% | power=20% | cost=15%

### 제외 규칙
- H2 Fuel Cell + Track-Pure/Drift-Spec: 제외 (수소 충전 인프라 부재)
- Ground Effect chassis: Track-Pure/Hyper-Aero/Street-Legal만 허용

### 보너스 규칙
- InWheel-4x288 + Monocoque-C6: n6 +5% (구조 통합 시너지)

## 카테고리별 최적

| 카테고리 | 소재 | 공정 | 파워트레인 | 섀시 | 시스템 | 값 |
|----------|------|------|-----------|------|--------|-----|
| **Best n6** | CFRP-Z6 | AFP-N6 | InWheel-4x288 | Monocoque-C6 | Street-Legal | **100.0%** |
| **Best Perf** | Diamond-Coat | AFP-N6 | Axial-Flux-4 | Monocoque-C6 | Track-Pure | **0.990** |
| **Best Power** | CFRP-Z6 | Die-Cast-Mega | Hub-2-Range | Monocoque-C6 | Track-Pure | **0.930** |
| **Best Cost** | Steel-UHSS | Die-Cast-Mega | Single-1152 | Skateboard-EV | GT-Tourer | **0.790** |

## Top 10 Pareto 경로

| Rank | 소재 | 공정 | 파워트레인 | 섀시 | 시스템 | n6_EXACT | Perf | Power | Cost | Score |
|------|------|------|-----------|------|--------|---------|------|-------|------|-------|
| 1 | CFRP-Z6 | AFP-N6 | Axial-Flux-4 | Monocoque-C6 | Track-Pure | 98.0% | 0.980 | 0.890 | 0.360 | **0.8690** |
| 2 | CFRP-Z6 | AFP-N6 | InWheel-4x288 | Monocoque-C6 | Track-Pure | 99.0% | 0.970 | 0.880 | 0.350 | **0.8660** |
| 3 | CFRP-Z6 | AFP-N6 | Axial-Flux-4 | Monocoque-C6 | Street-Legal | 99.0% | 0.960 | 0.870 | 0.380 | **0.8655** |
| 4 | CFRP-Z6 | AFP-N6 | InWheel-4x288 | Monocoque-C6 | Street-Legal | 100.0% | 0.950 | 0.860 | 0.370 | **0.8625** |
| 5 | CFRP-Z6 | Forged-Carbon | Axial-Flux-4 | Monocoque-C6 | Track-Pure | 95.0% | 0.960 | 0.910 | 0.390 | **0.8610** |
| 6 | Diamond-Coat | AFP-N6 | Axial-Flux-4 | Monocoque-C6 | Track-Pure | 97.0% | 0.990 | 0.880 | 0.320 | **0.8605** |
| 7 | CFRP-Z6 | AFP-N6 | Dual-576 | Monocoque-C6 | Track-Pure | 97.0% | 0.960 | 0.870 | 0.390 | **0.8600** |
| 8 | CFRP-Z6 | Forged-Carbon | InWheel-4x288 | Monocoque-C6 | Track-Pure | 96.0% | 0.950 | 0.900 | 0.380 | **0.8580** |
| 9 | CFRP-Z6 | Forged-Carbon | Axial-Flux-4 | Monocoque-C6 | Street-Legal | 96.0% | 0.940 | 0.890 | 0.410 | **0.8575** |
| 10 | Diamond-Coat | AFP-N6 | InWheel-4x288 | Monocoque-C6 | Track-Pure | 98.0% | 0.980 | 0.870 | 0.310 | **0.8575** |

## 통계

| 지표 | Max | Min | Avg | P50 | P75 | P90 |
|------|-----|-----|-----|-----|-----|-----|
| n6% | 100.0 | 65.0 | 81.2 | 81.0 | 85.0 | 89.0 |
| Perf | 0.990 | - | 0.806 | - | - | - |

## 최적 경로 (Rank #1) 분석

```
  L1   Material: [####################] n6=100%  CFRP Carbon Z=6 (BT-93, 1600 MPa, 1.55 g/cm3)
        |
        v
  L2    Process: [####################] n6=100%  AFP n=6 orientations (0/30/60/90/120/150 deg)
        |
        v
  L3 Powertrain: [###################-] n6=95%   tau=4 Axial Flux Motors (YASA-type)
        |
        v
  L4    Chassis: [####################] n6=100%  Full Carbon Monocoque Z=6 (F1-grade, 50kNm/deg)
        |
        v
  L5     System: [###################-] n6=95%   Track Pure (minimum weight, roll cage, slicks)
```

### n=6 상수 매핑

| 레벨 | 후보 | n=6 연결 |
|------|------|---------|
| 소재 | CFRP-Z6 | Carbon Z=6 (BT-93) |
| 공정 | AFP-N6 | n=6 배향각 (0/30/60/90/120/150 deg) |
| 파워트레인 | Axial-Flux-4 | tau=4 모터 (최고 토크 밀도) |
| 섀시 | Monocoque-C6 | Carbon Z=6 모노코크 (F1급) |
| 시스템 | Track-Pure | 최소 중량, 극한 성능 |

### 100% n6 EXACT 경로 (Rank #4)

n6=100% 달성 경로: **CFRP-Z6 + AFP-N6 + InWheel-4x288 + Monocoque-C6 + Street-Legal**
- InWheel-4x288: tau=4 인휠 모터 x sigma*J2=288kW 총출력 (72kW/wheel)
- Street-Legal: sigma=12 포인트 하네스, 도로+서킷 겸용
- Score: 0.8625 (Rank #4, 1위 대비 -0.75%)
- 트레이드오프: 성능 약간 희생 (-3%) 대신 n6 일관성 100% + 공도 주행 가능

## Pareto Frontier 해석

72개 비지배 해 중 핵심 패턴:
1. **소재**: CFRP-Z6 압도적 (Top 10 중 8개), Diamond-Coat가 성능 극대화용
2. **공정**: AFP-N6 지배적 (Top 10 중 7개), Forged-Carbon이 power 우위시 대안
3. **파워트레인**: Axial-Flux-4 vs InWheel-4x288 양강 구도
   - Axial-Flux-4: 성능 우위 (perf=1.00)
   - InWheel-4x288: n6 우위 (보너스 +5%)
4. **섀시**: Monocoque-C6 독점 (Top 10 전부)
5. **시스템**: Track-Pure vs Street-Legal (극한 성능 vs 실용성)
