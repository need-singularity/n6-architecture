# 궁극의 우주공학 — DSE 후보군 정의

## Vision
n=6 완전수 산술이 우주공학의 추진 기술, 궤도 역학, 구조/열방호,
통신/관제, 미션 시스템 전 계층을 관통하는 최적 아키텍처 경로를
전수 탐색한다.

## 핵심 n=6 연결
- Kepler 궤도 요소: n=6개 (a, e, i, Omega, omega, nu) EXACT
- Carbon 구조체: Z=6=n EXACT (BT-27), CFRP 우주 구조재
- D-T 핵융합: J₂=24 MeV 에너지 (BT-27), 궁극의 우주 추진
- 강체 자유도: n=6 DOF (3 translation + 3 rotation)
- 수소 연료: LHV=120=sigma(sigma-phi), HHV=142=sigma^2-phi (BT-38)
- Lagrange 점: sopfr=5개 (L1~L5)
- DSN 주파수: sigma-tau=8 대역 (S/X/Ka/Ku/...)
- 궤도면: sigma=12 orbital planes (LEO constellation)

## 기반 가설/BT
- BT-27: Carbon-6 chain (LiC₆ + C₆H₁₂O₆ + C₆H₆ → 24e = J₂)
- BT-30: SQ solar bridge (bandgap=4/3eV, 우주 태양전지)
- BT-38: Hydrogen quadruplet (LHV=120, HHV=142, 4/4 EXACT)
- BT-63: Solar panel cell ladder (우주 태양전지 어레이)
- Cross-DSE: chip (우주등급 프로세서), battery (우주 전원), fusion (추진)

## DSE 체인: 5단계

```
  추진 기술 → 궤도/항법 → 구조/열방호 → 통신/관제 → 미션 시스템
  Foundation   Process     Core           Engine       System
  (6 후보)     (5 후보)    (6 후보)       (5 후보)     (5 후보)
  = 6x5x6x5x5 = 4,500 raw combos
```

```
  ┌──────────────────────────────────────────────────────────────────┐
  │                   궁극의 우주공학 DSE 체인                        │
  │                                                                  │
  │  L1 Foundation    L2 Process    L3 Core       L4 Engine   L5 Sys │
  │  ┌──────────┐    ┌─────────┐   ┌──────────┐  ┌────────┐  ┌────┐ │
  │  │ChemRocket│───▶│Kepler6  │──▶│CarbonComp│─▶│DeepSpc │─▶│LEO │ │
  │  │IonDrive  │    │Hohmann  │   │TiAlloy   │  │LaserCom│  │Moon│ │
  │  │NuclearTh │    │LowThrust│   │HeatShield│  │AutoNav │  │Mars│ │
  │  │SolarSail │    │Lagrange │   │Inflatable│  │Relay   │  │Ast │ │
  │  │ElectricP │    │Gravity  │   │RadShield │  │QuantCom│  │Deep│ │
  │  │FusionDrv │    └─────────┘   │SelfHeal  │  └────────┘  └────┘ │
  │  └──────────┘                  └──────────┘                      │
  │                                                                  │
  │  Compatibility Rules:                                            │
  │  - FusionDrive → MarsColony/DeepExplore only                     │
  │  - SolarSail ✗ DeepExplore (insufficient photon pressure)        │
  │  - DeepExplore → LaserComm/QuantumComm required                  │
  └──────────────────────────────────────────────────────────────────┘
```

---

## Level 0: Foundation — 추진 기술 (K₁=6)

우주 미션의 근간이 되는 추진 시스템. 어떤 추진 방식을 사용하는가?

| ID | n6 | perf | power | cost | 핵심 연결 |
|----|-----|------|-------|------|----------|
| ChemRocket | 0.67 | 0.90 | 0.50 | 0.50 | LOX/LH2 Isp~450s, n=6 DOF 강체 |
| IonDrive | 0.83 | 0.70 | 0.80 | 0.45 | Hall thruster Isp~3000s, sigma-tau=8 kW |
| NuclearThermal | 1.00 | 0.85 | 0.40 | 0.30 | NTP Z=6 Carbon moderator, Isp~900s |
| SolarSail | 0.83 | 0.50 | 0.95 | 0.60 | Photon pressure, phi=2 reflectivity |
| ElectricProp | 0.67 | 0.75 | 0.70 | 0.50 | VASIMR variable Isp |
| FusionDrive | 1.00 | 0.60 | 0.35 | 0.20 | D-T fusion J₂=24 MeV (BT-27) |

## Level 1: Process — 궤도/항법 (K₂=5)

목적지까지의 최적 경로. 어떤 궤도 역학을 활용하는가?

| ID | n6 | perf | power | cost | 핵심 연결 |
|----|-----|------|-------|------|----------|
| Kepler6 | 1.00 | 0.85 | 0.70 | 0.65 | n=6 궤도요소 EXACT |
| Hohmann | 0.67 | 0.80 | 0.60 | 0.60 | phi=2 burn transfer |
| LowThrust | 0.83 | 0.75 | 0.75 | 0.55 | 연속 추력 나선 궤도 |
| Lagrange | 0.83 | 0.70 | 0.80 | 0.50 | sopfr=5 Lagrange points |
| Gravity | 0.67 | 0.88 | 0.85 | 0.70 | 중력 보조, Voyager 계보 |

## Level 2: Core — 구조/열방호 (K₃=6)

우주 환경에서 생존하는 구조체. 어떤 소재와 방호 기술을 사용하는가?

| ID | n6 | perf | power | cost | 핵심 연결 |
|----|-----|------|-------|------|----------|
| CarbonComp | 1.00 | 0.90 | 0.60 | 0.45 | Carbon Z=6=n EXACT (BT-27) |
| TitaniumAlloy | 0.67 | 0.85 | 0.55 | 0.40 | Ti-6Al-4V 고온 내성 |
| HeatShield | 0.83 | 0.80 | 0.50 | 0.45 | PICA ablative, phi=2 layers |
| Inflatable | 0.50 | 0.65 | 0.70 | 0.60 | BEAM 팽창식 모듈 |
| RadShield | 0.83 | 0.75 | 0.55 | 0.35 | sigma=12 layer 방사선 차폐 |
| SelfHeal | 0.67 | 0.60 | 0.65 | 0.30 | 자가 치유 소재 |

## Level 3: Engine — 통신/관제 (K₄=5)

우주에서의 데이터 전송과 자율 항법. 어떤 통신/관제 체계를 사용하는가?

| ID | n6 | perf | power | cost | 핵심 연결 |
|----|-----|------|-------|------|----------|
| DeepSpace | 0.83 | 0.85 | 0.50 | 0.40 | DSN sigma-tau=8 대역 |
| LaserComm | 1.00 | 0.90 | 0.60 | 0.35 | 광통신 sigma=12 Gbps 목표 |
| AutoNav | 0.83 | 0.80 | 0.65 | 0.45 | 자율항법 AI, n=6 센서 융합 |
| Relay | 0.67 | 0.75 | 0.70 | 0.50 | TDRS/Mars 중계 위성 |
| QuantumComm | 0.50 | 0.60 | 0.45 | 0.25 | QKD 양자 보안 통신 |

## Level 4: System — 미션 시스템 (K₅=5)

최종 미션 목표. 어떤 우주 시스템을 구축하는가?

| ID | n6 | perf | power | cost | 핵심 연결 |
|----|-----|------|-------|------|----------|
| LEO_Const | 0.83 | 0.85 | 0.60 | 0.50 | sigma=12 궤도면 메가 컨스텔레이션 |
| LunarBase | 0.67 | 0.80 | 0.50 | 0.35 | n=6 거주 모듈 달 기지 |
| MarsColony | 0.83 | 0.70 | 0.45 | 0.25 | 화성 정착, phi=2 year 윈도우 |
| Asteroid | 1.00 | 0.75 | 0.55 | 0.40 | n=6 자원 유형 소행성 채굴 |
| DeepExplore | 0.50 | 0.60 | 0.40 | 0.20 | 외행성/성간 탐사 |

## Scoring Weights
- n6 일관성: 35% (n=6 연결 강도)
- 성능: 25% (추력, 대역폭, 미션 성공률)
- 전력: 20% (전력 효율, 에너지 자급)
- 비용: 20% (개발/운용비)

## Compatibility Rules
1. FusionDrive → MarsColony 또는 DeepExplore만 허용 (과도한 추진력)
2. SolarSail ✗ DeepExplore (태양 복사압 부족)
3. DeepExplore → LaserComm 또는 QuantumComm 필수 (심우주 통신)
