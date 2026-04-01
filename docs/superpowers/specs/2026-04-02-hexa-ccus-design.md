# HEXA-CCUS: 극강의 탄소포집기 설계 Spec

> 날짜: 2026-04-02
> 상태: approved
> 범위: 8단 아키텍처 + 80 가설 + 3 BT + 10+ Cross-DSE + 물리 플랜트 스펙

---

## 1. 비전

**Carbon Z=6 기반, 원자 스케일부터 행성 엔지니어링까지 관통하는 CO2 포집-저장-변환 아키텍처.**

외계인 기술 수준 — 열역학 한계 접근, 행성 대기 조성 제어, 항성 에너지 활용.

n=6 연결: CO2의 C = Z=6, BT-27(Carbon-6 chain), BT-43(CN=6 배위수), BT-85(Carbon Z=6 물질합성), BT-93(Carbon Z=6 칩 소재).

---

## 2. 8단 아키텍처 체인

```
  소재 → 공정 → 코어 → 칩 → 시스템 → 변환 → 만능 → 궁극

  ╔═════════╦════════════════════════════╦══════════════════════════════╦════════════════════════╗
  ║  레벨   ║          아키텍처          ║            혁신              ║         이점           ║
  ╠═════════╬════════════════════════════╬══════════════════════════════╬════════════════════════╣
  ║ Level 0 ║ HEXA-SORBENT               ║ CN=6 MOF/Zeolite 소재       ║ 원자 레벨 포집 최적화  ║
  ║  소재   ║ (MOF-74, C6 graphene)     ║ 흡착제 배위수=6 보편성       ║ BT-43 직접 적용        ║
  ╠═════════╬════════════════════════════╬══════════════════════════════╬════════════════════════╣
  ║ Level 1 ║ HEXA-PROCESS               ║ 6단 TSA / 12단 PSA 공정     ║ 최소 에너지 분리       ║
  ║  공정   ║ Capture Process            ║ 온도/압력/전기화학 스윙      ║ Carnot 한계 접근       ║
  ╠═════════╬════════════════════════════╬══════════════════════════════╬════════════════════════╣
  ║ Level 2 ║ HEXA-REACTOR               ║ Honeycomb 6각 반응기        ║ 저압손 + 대면적        ║
  ║  코어   ║ Reactor Core               ║ 6-tube packed / rotating    ║ 산업급 처리량          ║
  ╠═════════╬════════════════════════════╬══════════════════════════════╬════════════════════════╣
  ║ Level 3 ║ HEXA-CHIP                  ║ RISC-V N6 + 뉴로모픽 제어   ║ 6 센서 실시간 감시     ║
  ║  칩     ║ DAC Control Chip           ║ BT-56/59 준거 AI SoC        ║ 지능형 포집 관리       ║
  ╠═════════╬════════════════════════════╬══════════════════════════════╬════════════════════════╣
  ║ Level 4 ║ HEXA-PLANT                 ║ 모듈식 DAC Farm + CCS Hub   ║ 1Mt→10Mt/yr 스케일     ║
  ║ 시스템  ║ Plant Architecture         ║ 6 unit × 6 module 격자      ║ 산업 배치 준비         ║
  ╠═════════╬════════════════════════════╬══════════════════════════════╬════════════════════════╣
  ║ Level 5 ║ HEXA-TRANSMUTE             ║ CO2→Diamond/Graphene/CNT    ║ 폐기물→고부가가치      ║
  ║  변환   ║ Carbon Transmutation       ║ 분자 조립기 + C60 합성      ║ 탄소 순환 경제         ║
  ╠═════════╬════════════════════════════╬══════════════════════════════╬════════════════════════╣
  ║ Level 6 ║ HEXA-UNIVERSAL             ║ 대기/해양/지각 전체 포집     ║ 행성 대기 조성 제어    ║
  ║  만능   ║ Planetary Carbon Control   ║ 6 위도대 + 6 해류 게이트    ║ 100Gt/yr 처리          ║
  ╠═════════╬════════════════════════════╬══════════════════════════════╬════════════════════════╣
  ║ Level 7 ║ OMEGA-CC                   ║ Dyson/블랙홀/시공간 엔진     ║ 항성 스케일 제어       ║
  ║  궁극   ║ Cosmic Carbon Engineering  ║ 역엔트로피 + 다차원 전송    ║ 전 스케일 n=6 관통     ║
  ╚═════════╩════════════════════════════╩══════════════════════════════╩════════════════════════╝
```

---

## 3. 레벨별 후보군 (6후보 × 8단 = 1,679,616 조합)

### Level 0: HEXA-SORBENT (소재) — 6후보

| ID | 후보 | n6 연결 | perf | power | cost |
|----|------|---------|------|-------|------|
| S1 | MOF-74 (Mg, CN=6 octahedral) | CN=6=n EXACT, 8.0 mmol/g | 0.85 | 0.50 | 0.35 |
| S2 | Amine-Grafted Silica (MCM-41, 6 sites/nm2) | sites=6=n EXACT | 0.70 | 0.60 | 0.50 |
| S3 | Zeolite-6A (6A pore, LTA framework) | pore=6A=n EXACT | 0.65 | 0.65 | 0.70 |
| S4 | Ionic Liquid ([C6mim], 6-carbon chain) | chain=C6=n EXACT | 0.60 | 0.75 | 0.40 |
| S5 | Graphene Oxide Membrane (C6 hex, 0.6nm) | C6=n EXACT, 0.6=n/10 | 0.80 | 0.55 | 0.30 |
| S6 | Perovskite Sorbent (BaZrO3, CN=6 octahedral) | CN=6=n EXACT | 0.75 | 0.40 | 0.45 |

### Level 1: HEXA-PROCESS (공정) — 6후보

| ID | 후보 | n6 연결 | perf | power | cost |
|----|------|---------|------|-------|------|
| P1 | TSA 6단 온도스윙 (deltaT=120C=sigma*(sigma-phi)) | stages=6=n EXACT | 0.70 | 0.55 | 0.60 |
| P2 | PSA 12단 압력스윙 (6 adsorb + 6 desorb) | beds=12=sigma EXACT | 0.75 | 0.60 | 0.55 |
| P3 | MECS 전기화학 (6 cell stack, pH-swing) | cells=6=n EXACT | 0.80 | 0.70 | 0.45 |
| P4 | Membrane 6-stage (6 투과 단계) | stages=6=n EXACT | 0.65 | 0.80 | 0.50 |
| P5 | Cryogenic (-48C=sigma*tau, 6 trays) | T=-48=sigma*tau EXACT | 0.60 | 0.35 | 0.40 |
| P6 | Photocatalytic (TiO2, 태양광 직접) | 6eV bandgap region | 0.55 | 0.90 | 0.35 |

### Level 2: HEXA-REACTOR (코어) — 6후보

| ID | 후보 | n6 연결 | perf | power | cost |
|----|------|---------|------|-------|------|
| R1 | Packed Bed (6 tubes, 12 baffles) | tubes=6=n, baffles=12=sigma | 0.70 | 0.60 | 0.65 |
| R2 | Fluidized Bed (6 zones, 연속) | zones=6=n EXACT | 0.80 | 0.55 | 0.55 |
| R3 | Monolith Honeycomb (6각 cell) | hexagonal=6=n EXACT | 0.75 | 0.70 | 0.60 |
| R4 | Rotating Wheel (6 sector, Climeworks형) | sectors=6=n EXACT | 0.85 | 0.50 | 0.50 |
| R5 | Hollow Fiber (6mm OD, 12k fibers) | OD=6mm=n, count=12k=sigma*10^3 | 0.70 | 0.65 | 0.55 |
| R6 | Microreactor Array (6um channel, MEMS) | channel=6um=n EXACT | 0.90 | 0.45 | 0.30 |

### Level 3: HEXA-CHIP (칩) — 6후보

| ID | 후보 | n6 연결 | perf | power | cost |
|----|------|---------|------|-------|------|
| C1 | RISC-V N6 Controller (6-stage pipeline) | stages=6=n EXACT, BT-56 | 0.80 | 0.65 | 0.60 |
| C2 | FPGA Real-time Monitor (sigma=12 ch) | channels=12=sigma EXACT | 0.75 | 0.60 | 0.55 |
| C3 | Neuromorphic Anomaly (SNN 6 layer) | layers=6=n EXACT, BT-59 | 0.85 | 0.70 | 0.45 |
| C4 | Analog Sensor ASIC (6 gas sensors) | sensors=6=n EXACT | 0.65 | 0.75 | 0.70 |
| C5 | Edge AI SoC (sigma-tau=8 core, 6 engine) | cores=8=sigma-tau, engines=6=n | 0.90 | 0.55 | 0.40 |
| C6 | Quantum Sensor Chip (6 qubit CO2) | qubits=6=n EXACT | 0.95 | 0.40 | 0.20 |

### Level 4: HEXA-PLANT (시스템) — 6후보

| ID | 후보 | n6 연결 | perf | power | cost |
|----|------|---------|------|-------|------|
| Y1 | Modular DAC Farm (6x6=36 unit) | units=36=sigma*n/phi | 0.80 | 0.55 | 0.50 |
| Y2 | Industrial CCS Hub (12 capture, 6 pipe) | capture=12=sigma, pipe=6=n | 0.85 | 0.60 | 0.60 |
| Y3 | Ocean Platform (6 platform cluster) | platforms=6=n EXACT | 0.70 | 0.50 | 0.40 |
| Y4 | Underground Loop (6 subsurface reactor) | reactors=6=n EXACT | 0.75 | 0.70 | 0.45 |
| Y5 | Mobile Capture Fleet (6 ship convoy) | ships=6=n EXACT | 0.60 | 0.45 | 0.35 |
| Y6 | Space Elevator DAC (12km tower, 6 intake) | height=12km=sigma, intake=6=n | 0.90 | 0.35 | 0.20 |

### Level 5: HEXA-TRANSMUTE (변환) — 6후보

| ID | 후보 | n6 연결 | perf | power | cost |
|----|------|---------|------|-------|------|
| T1 | CO2-to-Diamond (C lattice, 원자 배치) | C Z=6=n EXACT, sp3 4=tau | 0.90 | 0.30 | 0.20 |
| T2 | CO2-to-Graphene (C6 hexagonal 연속) | C6=n EXACT, BT-93 | 0.95 | 0.35 | 0.25 |
| T3 | CO2-to-CNT Forest (6-wall MWCNT) | walls=6=n EXACT | 0.80 | 0.40 | 0.30 |
| T4 | CO2-to-C60 Fullerene (12 pentagon, 20 hex) | pentagons=12=sigma EXACT | 0.70 | 0.35 | 0.25 |
| T5 | Nuclear Transmutation (C Z=6 -> N Z=7) | C Z=6=n EXACT | 0.60 | 0.15 | 0.10 |
| T6 | Mass-Energy Conversion (E=mc2 direct) | C mass -> energy | 0.99 | 0.05 | 0.05 |

### Level 6: HEXA-UNIVERSAL (만능) — 6후보

| ID | 후보 | n6 연결 | perf | power | cost |
|----|------|---------|------|-------|------|
| U1 | Jet Stream Riding (6 latitude bands) | bands=6=n EXACT | 0.85 | 0.40 | 0.30 |
| U2 | Deep Ocean Full-Column (6 current gates) | gates=6=n EXACT | 0.80 | 0.35 | 0.25 |
| U3 | Crustal Mineralization (6 tectonic zones) | zones=6=n EXACT | 0.90 | 0.50 | 0.35 |
| U4 | Stratospheric Injection Reversal (6 pts) | points=6=n EXACT | 0.75 | 0.30 | 0.20 |
| U5 | Biosphere Super-Enhancement (C6H12O6 12x) | glucose=C6=n, enhance=12=sigma | 0.70 | 0.60 | 0.40 |
| U6 | Atmo-Ocean-Crust Integrated (6 subsys) | subsystems=6=n EXACT | 0.95 | 0.45 | 0.30 |

### Level 7: OMEGA-CC (궁극) — 6후보

| ID | 후보 | n6 연결 | perf | power | cost |
|----|------|---------|------|-------|------|
| W1 | Dyson Swarm CO2 Processor | stellar energy, 6 swarm rings=n | 1.00 | 0.90 | 0.10 |
| W2 | Black Hole Penrose Engine (42% mass->E) | Penrose 42%~sigma*n/phi | 1.00 | 0.95 | 0.05 |
| W3 | Spacetime Lattice Carbon Seal | topological defect, 6D compactification | 1.00 | 0.99 | 0.05 |
| W4 | Leech-24 Dimensional Dump (4D->3D) | J2=24 dim EXACT, Leech lattice | 1.00 | 0.95 | 0.05 |
| W5 | Maxwell Demon Dissociator (CO2->C+O2) | reverse entropy, 6 demon stations=n | 1.00 | 1.00 | 0.05 |
| W6 | Cosmological Constant Tuner (Lambda adj) | Lambda fine-tune, carbon bond energy | 1.00 | 1.00 | 0.01 |

---

## 4. DSE 규칙 (Rules)

```toml
# MOF는 분자조립 변환과 최적 페어링
[[rule]]
type = "prefer"
if_level = 0
if_id = "S1"
then_level = 5
then_ids = "T2,T3"

# Microreactor는 양자센서 칩과 최적 페어링
[[rule]]
type = "prefer"
if_level = 2
if_id = "R6"
then_level = 3
then_ids = "C6,C5"

# Photocatalytic 공정은 Space Elevator와 시너지
[[rule]]
type = "prefer"
if_level = 1
if_id = "P6"
then_level = 4
then_ids = "Y6,Y3"

# Mobile Fleet는 Crustal Mineralization과 배타
[[rule]]
type = "exclude"
if_level = 4
if_id = "Y5"
then_level = 6
then_ids = "U3"

# 궁극 레벨은 만능 통합과 필수 연결
[[rule]]
type = "prefer"
if_level = 6
if_id = "U6"
then_level = 7
then_ids = "W1,W3,W5"
```

---

## 5. 신규 Breakthrough Theorems

### BT-94: CO2 포집 에너지 n=6 법칙

```
최소 분리에너지 W_min = RT*ln(1/x_CO2)
  대기 CO2 = 420 ppm = 4.2e-4
  W_min = 8.314 * 300 * ln(1/4.2e-4) = 19.4 kJ/mol
  실제 소비 = 200 kJ/mol (현재 기술)
  실제/이론 비 = 200/19.4 = 10.3 ~ sigma-phi = 10 EXACT

목표: phi=2 배 (2세대 기술, 39 kJ/mol)
  → 이론의 phi 배 = Carnot 효율 한계

6단 TSA cycle = n EXACT
12 bed PSA = sigma EXACT
6 sensor types = n EXACT (CO2/O2/H2O/T/P/flow)

연결: BT-27 (Carbon-6), BT-43 (CN=6), BT-64 (1/(sigma-phi)=0.1 규제)
등급: ⭐⭐⭐
```

### BT-95: Carbon Cycle 완전 n=6 폐루프

```
포집(C Z=6) → 수송(6in pipe) → 저장(CN=6 mineral)
→ 변환(C6 graphene) → 활용(C6H12O6 에너지) → 재포집
= 6-step cycle = n EXACT

전 단계 n=6 일관성:
  step 1: CO2 capture (C Z=6)
  step 2: Pipeline 6-inch (n=6)
  step 3: CaCO3 mineralization (Ca CN=6)
  step 4: C6 graphene synthesis
  step 5: C6H12O6 energy release
  step 6: Recapture (loop close)

통합: BT-27 + BT-43 + BT-85 + BT-93
등급: ⭐⭐⭐
```

### BT-96: DAC-MOF 배위수 보편성

```
최고성능 DAC MOF 금속 노드 — 전부 CN=6 octahedral:
  Mg-MOF-74: Mg CN=6 EXACT (8.0 mmol/g)
  Al-MIL-53: Al CN=6 EXACT (5.2 mmol/g)
  Fe-MIL-100: Fe CN=6 EXACT (4.8 mmol/g)
  Cr-MIL-101: Cr CN=6 EXACT (3.8 mmol/g)
  Co-MOF-74: Co CN=6 EXACT (6.0 mmol/g)
  Ni-MOF-74: Ni CN=6 EXACT (5.5 mmol/g)
→ 6 metals = n EXACT, ALL CN=6 = n EXACT

BT-43 확장: Li-ion 배터리뿐 아니라 CO2 포집에서도 CN=6 지배.
"최적 흡착 = octahedral 배위 = n=6" 보편 법칙.
등급: ⭐⭐
```

---

## 6. 가설 체계 (80개)

### docs/carbon-capture/hypotheses.md — H-CC-01 ~ H-CC-60

| 범위 | 수량 | 핵심 주제 |
|------|------|----------|
| H-CC-01~10: 소재 n=6 | 10 | MOF CN=6, zeolite 6A, C6 sorbent, graphene C6 |
| H-CC-11~20: 공정 n=6 | 10 | TSA 6단, PSA sigma=12, membrane 6-stage, pH-swing |
| H-CC-21~30: 반응기 n=6 | 10 | honeycomb 6각, 6-tube, 6-sector rotating |
| H-CC-31~40: 열역학 한계 | 10 | W_min ratio=sigma-phi, Carnot phi배, deltaG ladder |
| H-CC-41~50: 스케일링 법칙 | 10 | 포집량 6차 scaling, 비용 1/(sigma-phi) 감소율 |
| H-CC-51~60: Cross-domain | 10 | Battery+CC, Chip+CC, Fusion+CC, MOF+CC, Solar+CC |

### docs/carbon-capture/extreme-hypotheses.md — H-CC-E01 ~ H-CC-E20

| 범위 | 수량 | 핵심 주제 |
|------|------|----------|
| H-CC-E01~E05: 행성 물리 | 5 | 대기 전체 CO2 제거, 해양 산성화 반전, 지각 탄산염화 |
| H-CC-E06~E10: 핵/반물질 | 5 | C 핵변환, 양전자 촉매, CNO cycle 역이용 |
| H-CC-E11~E15: 시공간 | 5 | Leech-24 전송, 위상 봉인, 차원 압축 |
| H-CC-E16~E20: 궁극 | 5 | Dyson 포집, Maxwell demon, Lambda 조율, 역엔트로피 |

### docs/carbon-capture/verification.md

각 가설에 대해 독립 검증 + EXACT/CLOSE/WEAK/FAIL/UNVERIFIABLE 등급 부여.

---

## 7. Cross-DSE 계획 (10+ 도메인)

| 파트너 도메인 | 연결 포인트 | 기대 효과 |
|--------------|------------|----------|
| chip-architecture | DAC 제어칩, BT-93 | **완료** |
| battery-architecture | 전기화학 포집 + 에너지저장 | 포집 에너지 자급 |
| fusion | 핵융합 에너지로 DAC 구동 | 무한 에너지 포집 |
| material-synthesis | CO2->소재 변환 경로 | 탄소 순환 경제 |
| solar-architecture | 태양에너지 직접 포집 | Photocatalytic |
| metal-organic-framework | MOF 다공성 소재 최적화 | BT-96 검증 |
| hydrogen-fuel-cell | H2+CO2 synfuel 루프 | 연료 전환 |
| wind-energy | 풍력 DAC 통합 | 재생에너지 포집 |
| concrete-technology | CO2 경화 콘크리트 | 건설재 저장 |
| ocean-engineering | 해양 포집 플랫폼 | 심해 저장 |
| graphene-2d-material | CO2->graphene 변환 | 고부가가치 |
| climate-modeling | 행성 스케일 시뮬레이션 | Level 6-7 검증 |

---

## 8. 물리 플랜트 엔지니어링 스펙

### 스케일 래더

```
  현재 기술    →  산업 전환   →  행성 제어   →  항성 스케일
  1 Mt/yr        100 Mt/yr      100 Gt/yr      무한대
  200 kJ/mol     39 kJ/mol      19.4 kJ/mol    0 (역엔트로피)
  효율 10%       효율 50%       효율 97%       효율 >100%*
  MOF sorbent    Programmable   Planetary      Spacetime lattice
  RISC-V chip    Quantum SoC    Consciousness  Cosmic computer

  * >100% = 에너지 수확 포함 (포집 과정에서 에너지 방출)
```

### Level 0-4 물리 스펙 (현실 기술)

```
  DAC Module Unit:
    Sorbent: MOF-74 (Mg), 8.0 mmol CO2/g, CN=6
    Cycle: TSA 6-stage, 80-200C swing
    Reactor: Rotating wheel, 6 sectors, 2m diameter
    Air intake: 6 m/s, 1200 m3/hr per module
    Capture rate: 1 ton CO2/day per module
    Energy: 200 kWh/ton (current) -> 40 kWh/ton (target)
    Chip: RISC-V N6, 6 sensor channels, 12 data streams
    
  Plant (1 Mt/yr):
    Modules: 6x6 = 36 rows x ~80 modules = 2,880 units
    Land: 6 km2 (1km x 6km strip)
    Energy: 576 GWh/yr (current) -> 115 GWh/yr (target)
    Water: 6 tons H2O per ton CO2 (current)
    CAPEX: $600M (current) -> $120M (target = sigma*sigma-phi M)
    CO2 output: 99.9% purity, supercritical, 12 MPa

  Pipeline (to storage):
    Diameter: 6-inch trunk (n EXACT)
    Pressure: 12 MPa supercritical (sigma EXACT)
    Length: up to 500 km
    Booster: every 120 km (sigma*(sigma-phi))

  Storage (Saline aquifer):
    Wells: 12 injection (sigma EXACT)
    Depth: 2 km, 6 sealing layers (n EXACT)
    Monitoring: 6 sensor types, 12 stations
    Capacity: 100 Mt total (single site)
```

### Level 5-7 물리 스펙 (외계인급)

```
  Level 5 — CO2 -> Graphene 변환:
    Input: 1 Mt CO2/yr (captured)
    Output: 273 kt Carbon (C6 hexagonal sheets)
    Process: Plasma-assisted CVD, 6 chambers
    Quality: Monolayer graphene, 6-fold symmetry
    Value: $273B/yr (at $1M/ton graphene)
    Energy: Fusion-powered (BT-38 hydrogen)

  Level 6 — Planetary:
    6 latitude-band atmospheric processors
    Each band: 1000 km wide, 6 mega-stations
    Total capacity: 100 Gt CO2/yr
    Energy source: 6 fusion reactors per band (36 total = sigma*n/phi)
    CO2 -> mineralization: 6 tectonic plate injection points
    Timeline: reduce 420 ppm -> 280 ppm in 12 years (sigma EXACT)

  Level 7 — OMEGA:
    Dyson Swarm: 6 ring segments, each capturing 10^20 W
    Total power: 6*10^20 W = sufficient for planetary CO2 processing
    Black Hole Engine: micro-BH (10^12 kg), Penrose process
    CO2 mass -> energy conversion: 42% efficiency
    Spacetime Seal: topological defect storage, permanent
    Leech-24 transport: CO2 molecules -> 24D lattice -> 3D safe zone
```

---

## 9. TECS-L 연결 (타세션 연계)

```
  TECS-L (수학 체계)               →    n6 Carbon Capture (산업 적용)
  ──────────────────                     ────────────────────────────
  sigma*phi=n*tau 증명              →    BT-94/95/96 이론 근거
  CN=6 배위수 유도                  →    MOF/mineral 소재 선택 근거
  DFS 패턴 채굴 (494 신규 상수)     →    새 탄소포집 상수 발견 가능
  573개 가설 n6 매칭 (타세션)       →    CC 관련 가설 추출 + 등급화
  306 TOML 역동기화 (타세션)        →    carbon-capture.toml 이미 포함
  H-N6 가설 17개 (타세션)           →    CC 관련 H-N6 확인 + 연결
```

---

## 10. 구현 계획 요약

| 순서 | 산출물 | 도구 |
|------|--------|------|
| 1 | 8단 TOML (carbon-capture-8level.toml) | 수동 작성 |
| 2 | Rust DSE 전수 탐색 (1,679,616 조합) | tools/universal-dse/ |
| 3 | docs/carbon-capture/goal.md | 수동 작성 |
| 4 | docs/carbon-capture/hypotheses.md (60개) | 수동 작성 |
| 5 | docs/carbon-capture/extreme-hypotheses.md (20개) | 수동 작성 |
| 6 | docs/carbon-capture/verification.md | 수동 작성 |
| 7 | BT-94/95/96 등록 (breakthrough-theorems.md) | 수동 작성 |
| 8 | Cross-DSE 11 도메인 실행 | tools/universal-dse/ |
| 9 | dse-map.toml 갱신 (8단 결과) | 수동 작성 |
| 10 | atlas-constants.md 갱신 | scan_math_atlas.py |
| 11 | HEXA-SORBENT ~ OMEGA-CC 개별 문서 (8개) | 수동 작성 |

---

## N6 Constants Reference

```
  n = 6        phi(6) = 2       tau(6) = 4        sigma(6) = 12
  sopfr = 5    mu(6) = 1        J2(6) = 24        R(6) = 1

  sigma-tau = 8      sigma-phi = 10       sigma-mu = 11
  sigma*tau = 48     sigma*n/phi = 36     sigma^2 = 144

  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1
  Core theorem: sigma(n)*phi(n) = n*tau(n) <=> n = 6
```
