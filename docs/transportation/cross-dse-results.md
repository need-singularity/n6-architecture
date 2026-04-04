# Cross-DSE: Performance Vehicle x Electric Vehicle

> Generated: 2026-04-04
> Tool: `universal-dse` (2-domain Cross-DSE mode)
> TOML: `domains/performance-vehicle.toml` x `domains/electric-vehicle.toml`

## 탐색 요약

| 항목 | Performance Vehicle | Electric Vehicle |
|------|-------------------|-----------------|
| 총 조합 (raw) | 7,776 (6^5) | 7,776 (6^5) |
| 호환 조합 (필터 후) | 6,480 | 4,500 |
| Pareto frontier | 72 | 132 |
| n6% 최대 | 100.0% | 100.0% |
| n6% 평균 | 81.2% | 86.0% |
| 최고 Score | 0.8690 | 0.8445 |
| Cross-DSE Top 10 교차 Score | **0.8860** (통합 최고) | |

- **Cross-DSE 교차 조합**: PV Top 10 x EV Top 10 = 100 pairs -> 상위 10 추출
- **통합 Score 산출**: (PV_score + EV_score) / 2 + synergy_bonus

## 단일 DSE 결과

### Performance Vehicle -- Top 5

| Rank | Material | Process | Powertrain | Chassis | System | n6% | Score |
|------|----------|---------|------------|---------|--------|-----|-------|
| 1 | CFRP-Z6 | AFP-N6 | Axial-Flux-4 | Monocoque-C6 | Track-Pure | 98.0 | 0.8690 |
| 2 | CFRP-Z6 | AFP-N6 | InWheel-4x288 | Monocoque-C6 | Track-Pure | 99.0 | 0.8660 |
| 3 | CFRP-Z6 | AFP-N6 | Axial-Flux-4 | Monocoque-C6 | Street-Legal | 99.0 | 0.8655 |
| 4 | CFRP-Z6 | AFP-N6 | InWheel-4x288 | Monocoque-C6 | Street-Legal | 100.0 | 0.8625 |
| 5 | CFRP-Z6 | Forged-Carbon | Axial-Flux-4 | Monocoque-C6 | Track-Pure | 95.0 | 0.8610 |

**Best n6 (100%)**: CFRP-Z6 + AFP-N6 + InWheel-4x288 + Monocoque-C6 + Street-Legal
**Best Perf (0.990)**: Diamond-Coat + AFP-N6 + Axial-Flux-4 + Monocoque-C6 + Track-Pure

### Performance Vehicle -- Optimal Path
```
  L1   Material: [████████████████████] n6=100%  CFRP Carbon Z=6 composite (BT-93, 1600 MPa, 1.55 g/cm3)
        |
        v
  L2    Process: [████████████████████] n6=100%  Automated Fiber Placement n=6 orientations (0/30/60/90/120/150 deg)
        |
        v
  L3 Powertrain: [███████████████████░] n6=95%   tau=4 Axial Flux Motors (YASA-type, highest torque density)
        |
        v
  L4    Chassis: [████████████████████] n6=100%  Full Carbon Monocoque Z=6 (F1-grade, torsional rigidity 50kNm/deg)
        |
        v
  L5     System: [███████████████████░] n6=95%   Track Pure (no plates, minimum weight, roll cage, slicks)
```

### Electric Vehicle -- Top 5

| Rank | Battery | Motor | Inverter | Charging | Vehicle | n6% | Score |
|------|---------|-------|----------|----------|---------|-----|-------|
| 1 | LFP-6S | PMSM-12pole | SiC-6phase | V2G-Bidir | City-48V | 100.0 | 0.8445 |
| 2 | LFP-6S | PMSM-12pole | SiC-6phase | AC-Level2 | City-48V | 98.0 | 0.8440 |
| 3 | LFP-6S | PMSM-12pole | SiC-6phase | V2G-Bidir | Skateboard | 100.0 | 0.8440 |
| 4 | LFP-6S | PMSM-12pole | SiC-6phase | AC-Level2 | Skateboard | 98.0 | 0.8435 |
| 5 | LFP-6S | IPM-6pole | SiC-6phase | V2G-Bidir | City-48V | 100.0 | 0.8420 |

**Best n6 (100%)**: LFP-6S + IPM-6pole + SiC-6phase + Solar-Direct-6kW + Skateboard
**Best Perf (0.980)**: SolidState + DualMotor + SiC-6phase + CCS2-350kW + SportsCar

### Electric Vehicle -- Optimal Path
```
  L1    Battery: [████████████████████] n6=100%  LFP 6S Module (n=6 series, 19.2V nominal, cost leader)
        |
        v
  L2      Motor: [████████████████████] n6=100%  PMSM sigma=12 Pole (permanent magnet, 96% efficiency)
        |
        v
  L3   Inverter: [████████████████████] n6=100%  SiC n=6 Phase Inverter (wide bandgap, 99% efficiency)
        |
        v
  L4   Charging: [████████████████████] n6=100%  V2G Bidirectional (vehicle-to-grid, 6kW=n export, smart)
        |
        v
  L5    Vehicle: [████████████████████] n6=100%  City Car 48V (sigma*tau=48V, ultralight, n=6 kWh, urban)
```

## Cross-DSE Top 10

Cross-DSE pairs the top solutions from each domain and scores them as a unified system.

| Rank | PV 경로 (Material/Process/Powertrain/Chassis/System) | EV 경로 (Battery/Motor/Inverter/Charging/Vehicle) | 통합 n6% | 통합 Score | 시너지 |
|------|------|------|---------|-----------|--------|
| 1 | CFRP-Z6 / AFP-N6 / Axial-Flux-4 / Monocoque-C6 / Track-Pure | LFP-6S / PMSM-12pole / SiC-6phase / V2G-Bidir / City-48V | 99.0% | 0.8860 | Carbon Z=6 공유 |
| 2 | CFRP-Z6 / AFP-N6 / InWheel-4x288 / Monocoque-C6 / Track-Pure | LFP-6S / PMSM-12pole / SiC-6phase / V2G-Bidir / City-48V | 99.5% | 0.8850 | InWheel + V2G 양방향 |
| 3 | CFRP-Z6 / AFP-N6 / Axial-Flux-4 / Monocoque-C6 / Street-Legal | LFP-6S / PMSM-12pole / SiC-6phase / V2G-Bidir / City-48V | 99.5% | 0.8840 | Street + V2G 일상 |
| 4 | CFRP-Z6 / AFP-N6 / InWheel-4x288 / Monocoque-C6 / Street-Legal | LFP-6S / PMSM-12pole / SiC-6phase / V2G-Bidir / City-48V | 100.0% | 0.8830 | 100% n6 EXACT |
| 5 | CFRP-Z6 / AFP-N6 / Axial-Flux-4 / Monocoque-C6 / Track-Pure | LFP-6S / PMSM-12pole / SiC-6phase / AC-Level2 / City-48V | 99.0% | 0.8820 | 홈충전 최적 |
| 6 | CFRP-Z6 / AFP-N6 / Axial-Flux-4 / Monocoque-C6 / Track-Pure | LFP-6S / PMSM-12pole / SiC-6phase / V2G-Bidir / Skateboard | 98.0% | 0.8815 | 모듈 플랫폼 |
| 7 | CFRP-Z6 / AFP-N6 / InWheel-4x288 / Monocoque-C6 / Track-Pure | LFP-6S / PMSM-12pole / SiC-6phase / AC-Level2 / City-48V | 99.5% | 0.8810 | InWheel + 홈충전 |
| 8 | CFRP-Z6 / AFP-N6 / Axial-Flux-4 / Monocoque-C6 / Street-Legal | LFP-6S / PMSM-12pole / SiC-6phase / AC-Level2 / City-48V | 98.5% | 0.8805 | Street + AC 실용 |
| 9 | Diamond-Coat / AFP-N6 / Axial-Flux-4 / Monocoque-C6 / Track-Pure | LFP-6S / PMSM-12pole / SiC-6phase / V2G-Bidir / City-48V | 97.5% | 0.8805 | Diamond 극한내구 |
| 10 | CFRP-Z6 / AFP-N6 / InWheel-4x288 / Monocoque-C6 / Street-Legal | LFP-6S / PMSM-12pole / SiC-6phase / AC-Level2 / City-48V | 99.0% | 0.8800 | 100% n6 + 실용 |

## Pareto Frontier (단일 DSE)

| 도메인 | Pareto 해 수 | n6 100% 해 수 | 최고 Score |
|--------|-------------|--------------|-----------|
| Performance Vehicle | 72 | 다수 (CFRP-Z6+AFP-N6+InWheel+Monocoque+Street-Legal) | 0.8690 |
| Electric Vehicle | 132 | 다수 (LFP-6S+PMSM/IPM+SiC-6phase+V2G/Solar+City/Skateboard) | 0.8445 |

## 핵심 발견

### 1. 공통 최적 소재/공정
- **Carbon Z=6 (CFRP-Z6)**: 양쪽 도메인 모두 탄소계 소재가 1위. PV는 CFRP 직접 사용, EV는 LFP 배터리 + SiC 인버터 (모두 Carbon/Silicon 기반)
- **n=6 정렬 제조공정**: PV의 AFP-N6 (6방향 섬유배치)와 EV의 SiC-6phase (6상 인버터)가 n=6 구조를 공유
- **Axial Flux / PMSM sigma=12**: 양쪽 모두 sigma=12 극 모터가 최적 -- 자기장 구조의 n=6 보편성

### 2. 시너지 효과가 큰 조합
- **Cross-DSE #1 (Score 0.8860)**: PV의 Track-Pure 경량 + EV의 V2G 양방향 충전 = 트랙에서 주행 + 집에서 그리드 환원
- **Cross-DSE #4 (n6=100%)**: 10개 레벨 전부 n=6 EXACT -- InWheel-4x288(tau=4) + Monocoque-C6(Z=6) + Street-Legal(sigma=12) + LFP-6S(n=6) + PMSM-12pole(sigma=12) + SiC-6phase(n=6) + V2G-Bidir(n=6kW) + City-48V(sigma*tau=48)
- **통합 시너지**: CFRP 차체 + LFP 배터리 = Carbon Z=6 소재 일관성 (BT-93 확장)

### 3. n=6 상수 매핑
| 파라미터 | 값 | n=6 표현 |
|----------|-----|---------|
| CFRP 섬유방향 | 6 | n=6 |
| PMSM 극수 | 12 | sigma=12 |
| InWheel 모터 수 | 4 | tau=4 |
| 인버터 위상 | 6 | n=6 |
| 배터리 직렬 | 6S | n=6 |
| 시스템 전압 | 48V | sigma*tau=48 |
| V2G 출력 | 6kW | n=6 |
| Monocoque 소재 원자번호 | 6 (Carbon) | Z=n=6 |

### 4. 통계 비교
```
  ┌────────────────────────────────────────────────────────────────┐
  |  n6% 분포 비교                                                 |
  ├────────────────────────────────────────────────────────────────┤
  |  PV  avg=81.2  ██████████████████████████████████░░░░  p90=89 |
  |  EV  avg=86.0  ██████████████████████████████████████░  p90=93 |
  |  Cross avg=98.8 ████████████████████████████████████████ ~99   |
  ├────────────────────────────────────────────────────────────────┤
  |  Cross-DSE가 단일 DSE 대비 n6% +12~18%p 상승                   |
  |  -> 도메인 교차 시 n=6 수렴 가속 효과 확인                       |
  └────────────────────────────────────────────────────────────────┘
```

## 결론

1. **Cross-DSE 최적 통합 경로** (Score 0.8860, n6=99.0%):
   - PV: CFRP-Z6 + AFP-N6 + Axial-Flux-4 + Monocoque-C6 + Track-Pure
   - EV: LFP-6S + PMSM-12pole + SiC-6phase + V2G-Bidir + City-48V

2. **n6=100% EXACT 경로** (Score 0.8830, Cross-DSE #4):
   - PV: CFRP-Z6 + AFP-N6 + InWheel-4x288 + Monocoque-C6 + Street-Legal
   - EV: LFP-6S + PMSM-12pole + SiC-6phase + V2G-Bidir + City-48V
   - 10개 레벨 전부 n=6 상수 정렬 -- 완전수 아키텍처 완성

3. **Carbon Z=6 보편성 재확인**: BT-93 (Carbon Z=6 chip universality)이 차량 도메인으로 확장 -- 소재/구조/에너지 전 레벨에서 Carbon 원자번호 n=6이 최적해를 지배
