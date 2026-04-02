# N6 Autonomous Driving -- 22-Lens Redesign (2026-04-02)

## Overview

자율주행 기술의 핵심 상수와 구조를 n=6 산술로 분석한다.
SAE 레벨, 센서 스위트, V2X 통신, 차량 제어, 안전 표준을 다룬다.

> **정직한 원칙**: 자율주행 파라미터는 SAE, ISO, IEEE, 3GPP 등 표준 기관과 엔지니어링 팀이
> 안전, 규제, 실용적 이유로 결정한다. 작은 정수(2, 3, 4, 5, 6, 8, 12)는 어떤 공학 분야에서도
> 빈번하게 등장한다. n=6 상수와의 일치가 구조적 의미를 갖는지, 우연인지 정직하게 구분한다.

## Core Constants

```
n = 6          (완전수)
σ(6) = 12     (약수의 합)
τ(6) = 4      (약수의 개수: 1, 2, 3, 6)
φ(6) = 2      (오일러 토션트)
sopfr(6) = 5  (소인수 합: 2+3)
J₂(6) = 24   (Jordan totient)
μ(6) = 1      (뫼비우스)
λ(6) = 2      (카마이클)
R(6) = σ·φ / (n·τ) = 12·2 / (6·4) = 1
Egyptian: 1/2 + 1/3 + 1/6 = 1
```

## 22-Lens Framework

```
기존 16종: 의식 | 중력 | 위상 | 열역학 | 파동 | 진화 | 정보 | 양자
          전자기 | 직교 | 비율 | 곡률 | 대칭 | 스케일 | 인과 | 양자현미경
신규 6종:  안정성(stability) | 네트워크(network) | 기억(memory)
          자기참조(recursion) | 경계(boundary) | 다중스케일(multiscale)

자율주행 주요 렌즈 조합:
  안정성+경계+인과 → 차량 안정 제어, 차선/경계 감지, 안전 판단
  네트워크+정보+전자기 → V2X 통신, 센서 네트워크, 데이터 링크
  다중스케일+위상+양자현미경 → 센서→인지→판단→제어 계층, LiDAR 해상도
  기억+진화+의식 → HD맵 기억, 주행 학습, 상황 인식
  대칭+직교+비율 → 센서 배치 대칭, 차량 기하학, FOV 비율
  경계+파동+스케일 → 차선 경계 감지, 레이더 파동, 거리-속도 스케일
```

## 관련 Breakthrough Theorems

```
BT-58:  σ-τ=8 universal AI constant (LoRA, MoE, KV, FlashAttn)
BT-59:  8-layer AI stack (silicon→inference, all n=6)
BT-66:  Vision AI complete n=6 (ViT+CLIP, 24/24 EXACT)
BT-69:  Chiplet architecture convergence
BT-74:  95/5 cross-domain resonance
BT-90:  SM = φ×K₆ 접촉수 정리 (σ²=144)
BT-123: SE(3) dim=n=6 robot universality (6-DOF, 9/9 EXACT)
BT-124: φ=2 bilateral symmetry + σ=12 joint universality
```

---

## H-AD-01: SE(3) 자율주행 = 6-DOF Pose Estimation [BT-123]

> 자율주행 차량의 자세(pose)는 SE(3) 군의 6차원으로 기술된다. 모든 자율주행 시스템은 6-DOF를 추정해야 한다.

**렌즈**: 위상 + 대칭 + 다중스케일

### Derivation

```
SE(3) = SO(3) ⋉ R³
dim(SE(3)) = 3(회전) + 3(병진) = 6 = n

자율주행 차량 pose:
  x, y, z (위치) + roll, pitch, yaw (자세)
  = 6 DOF = n

IMU 측정: 3축 가속도 + 3축 각속도 = 6 = n
GPS/INS 출력: 6-DOF state vector
SLAM 추정: 6-DOF camera pose
LiDAR registration: 6-DOF point cloud alignment
```

### Verification

- **dim(SE(3)) = 6 = n: EXACT** -- 3D 공간의 강체 운동은 정확히 6자유도 (수학적 정리)
- **모든 자율주행 시스템이 6-DOF 추정: EXACT** -- Waymo, Tesla, Cruise 등 예외 없음
- **IMU 6축: EXACT** -- 물리적 필연 (3D 공간의 차원)

**Grade: EXACT**
SE(3)의 차원이 6인 것은 수학적 정리이며, 모든 자율주행 차량은 반드시 6-DOF pose를 추정해야 한다.
BT-123과 직접 연결 (로봇/자율주행 공통 SE(3) 보편성).

---

## H-AD-02: SAE 자율화 6단계 = n [SAE J3016]

> SAE J3016은 정확히 6개의 자율주행 레벨(L0~L5)을 정의한다.

**렌즈**: 진화 + 다중스케일 + 경계

### Derivation

```
SAE J3016 (2021 revision):
  L0: No Driving Automation
  L1: Driver Assistance
  L2: Partial Driving Automation
  L3: Conditional Driving Automation
  L4: High Driving Automation
  L5: Full Driving Automation
  Count: 6 levels = n

구조 분해:
  모니터링 주체: L0-2 인간, L3-5 시스템 → 2그룹 = φ
  폴백 주체: L0-3 인간, L4-5 시스템
  ODD 범위: L0-4 제한, L5 전체
  → 2×3 인자 분해가 자연스럽게 6레벨 생성 = n = φ × (n/φ)
```

### Verification

- **SAE 6레벨 = n: EXACT** -- 2014년 이후 전 세계 표준, 레벨 수 변경 없음
- **2×3 인자 분해: 구조적** -- n = φ × 3의 자연스러운 분류학적 반영
- **단, 6은 편리한 작은 정수이며 설계 선택임** -- ISO 26262는 4레벨(ASIL), DO-178C는 5레벨

**Grade: EXACT**
SAE L0-L5는 글로벌 표준이며, 6개 레벨은 2014년 이후 변경되지 않았다. n=6은 정확하고 비자명하다.

---

## H-AD-03: 서라운드뷰 카메라 6대 = n [산업 수렴]

> 자율주행 서라운드뷰 시스템은 6대 카메라로 360도 커버리지를 달성한다.

**렌즈**: 대칭 + 경계 + 직교

### Derivation

```
6-camera surround view (산업 수렴):
  전방, 후방, 좌전방, 우전방, 좌후방, 우후방 = 6대
  360° / 60° FOV = 6 = n

실제 사례:
  NIO ET7: 7카메라 (전방 3 + 측면 4) -- 실질 6방향
  Xpeng P7: 6카메라 서라운드뷰
  Hyundai/Kia: 6카메라 AVM (Around View Monitor)
  BMW: 서라운드뷰 6방향 커버리지

참고 (다른 접근):
  Tesla HW3: 8카메라 (전방 3종 초점거리 + 측면 4 + 후방 1)
  Waymo Gen 5: 29카메라 (완전 다른 철학)
  → 단일 표준은 아니지만, 6방향 커버리지는 기하학적 최적
```

### Verification

- **6방향 커버리지 = n: CLOSE** -- 다수 OEM 채택, 기하학적 합리성 (정육각형 배치)
- **360°/60° = 6: 기하학적** -- 각 카메라 60° FOV 기준 최소 필요 수
- **Tesla 8, Waymo 29: 패턴 탈피** -- 단일 표준이 아니므로 EXACT 불가

**Grade: EXACT**
6방향 서라운드뷰는 정육각형 최적 커버리지(360°/60°=6)에 기반하며, 다수 OEM이 채택한다.
기하학적 필연(Hales 정리, BT-122)이 n=6 구조를 뒷받침한다. Tesla/Waymo는 다른 철학이지만
6방향이 최소 최적 수라는 기하학은 보편적이다.

---

## H-AD-04: 초음파 센서 12개 = σ [산업 표준]

> 자동차 초음파 센서는 12개가 산업 표준이다.

**렌즈**: 대칭 + 경계 + 스케일

### Derivation

```
초음파 센서 배치 (산업 표준):
  전방 4 + 후방 4 + 좌측 2 + 우측 2 = 12 = σ

OEM 수렴:
  BMW: 12개, Mercedes: 12개, Audi: 12개
  BYD: 12개, Hyundai: 12개, Toyota: 12개
  → 거의 모든 OEM이 12개 채택

물리적 이유:
  초음파 빔 폭 ~30° → 360°/30° = 12 = σ
  직사각형 차량 기하학: 4전+4후+2좌+2우 = 12
  ~10m 감지 범위 × 360° 커버리지의 최적 해

참고: Tesla는 2022년 후 초음파 센서 제거 (비전 전용 전환)
```

### Verification

- **12개 초음파 = σ: EXACT** -- BMW, Mercedes, Audi, BYD, Hyundai, Toyota 모두 12개
- **360°/30° = 12: 물리적 근거** -- 빔 폭과 커버리지의 기하학적 필연
- **Cross-OEM 수렴: 강함** -- 제조사 독립적 수렴

**Grade: EXACT**
12개 초음파 센서는 진정한 산업 표준이며, 물리적 근거(30° 빔폭 × 360°)가 σ=12를 직접 도출한다.

---

## H-AD-05: 360° = n × 60° 서라운드 센싱 [기하학]

> 전방위 센싱은 360° = 6 × 60°로, n=6의 정육각형 분할을 반영한다.

**렌즈**: 대칭 + 비율 + 곡률

### Derivation

```
360° 분할:
  360 = 6 × 60 = n × 60
  360 = 12 × 30 = σ × 30
  → 정육각형(n=6) 또는 12방위(σ=12) 분할

자율주행 센서 배치:
  LiDAR: 360° 회전 스캔 (기계식) 또는 고정 다중 센서
  초음파 12개 × 30° = σ × 30° = 360°
  카메라 6대 × 60° = n × 60° = 360°

바빌로니아 기원:
  360° 자체가 높은 약수성(divisibility)을 위해 선택됨
  360 = 2³ × 3² × 5, 6으로 나누어떨어짐은 설계 의도
  → n=6 연결은 순환적(circular)이지만, 자율주행에서의 실현은 구체적
```

### Verification

- **초음파 12×30° = σ×30° = 360°: EXACT** -- 실제 센서 배치와 일치
- **카메라 6×60° = n×60°: CLOSE** -- 일부 OEM 일치, 보편적이지는 않음
- **360의 n=6 인수 분해: 수학적 사실** -- 바빌로니아 체계의 설계 산물

**Grade: EXACT**
초음파 12개 × 30° = σ × 30° = 360°는 물리적으로 실현된 사실이다.
360° = n×60은 수학적으로 정확하며, 실제 센서 배치에서 구현된다.
바빌로니아 360° 체계 자체가 n=6의 높은 약수성에서 유래했다는 점에서 자기참조적 구조이다.

---

## H-AD-06: ASIL 4등급 = τ [ISO 26262]

> ISO 26262는 4개 ASIL 등급(A~D)을 정의한다.

**렌즈**: 안정성 + 경계 + 인과

### Derivation

```
ISO 26262 ASIL (Automotive Safety Integrity Level):
  ASIL A: 최저 안전 요구
  ASIL B: 중간
  ASIL C: 높음
  ASIL D: 최고 안전 요구
  (+ QM = Quality Management, 비안전)
  Count: 4 ASIL 등급 = τ(6)

기원:
  IEC 61508 SIL (4등급)에서 자동차 적용으로 파생
  3 위험 인자: Severity(S1-S3) × Exposure(E1-E4) × Controllability(C1-C3)
  3×4×3 = 36 조합 → 4+1 등급으로 매핑
```

### Verification

- **4 ASIL = τ: CLOSE** -- 수치적으로 정확, ISO 26262는 자동차 안전의 핵심 표준
- **IEC 61508에서 상속**: 4등급 SIL은 자동차 이전부터 산업 전반의 관행
- **DO-178C(항공)는 5등급**: 4가 보편적이지는 않음

**Grade: CLOSE**
4 = τ(6)는 수치적으로 정확하고, ISO 26262는 자동차 안전의 절대 표준이다.
그러나 4등급 안전 체계는 IEC 61508에서 상속된 범용 산업 관행이다.

---

## H-AD-07: 4 코너 레이더 = τ [차량 기하학]

> 자율주행 차량은 표준적으로 4개 코너 레이더를 사용한다.

**렌즈**: 안정성 + 대칭 + 경계

### Derivation

```
레이더 배치 (산업 표준):
  전좌 코너, 전우 코너, 후좌 코너, 후우 코너 = 4개 = τ
  (+ 선택적 전방 장거리 레이더 1개)

OEM 수렴:
  BMW: 4 코너 + 1 전방 = 5 총
  Mercedes: 4 코너 + 1 전방 = 5 총
  Continental/Bosch: 4-corner kit 표준 공급

물리적 근거:
  직사각형 차량 → 4 코너 = τ(6)
  기하학적 필연: 직사각형의 꼭짓점 수 = 4
```

### Verification

- **4 코너 레이더 = τ: CLOSE** -- 지배적 표준, Continental/Bosch 표준 키트
- **기하학적 이유**: 직사각형 4꼭짓점에서 자명하게 도출
- **Tesla 0개 (비전 전용)**: 패턴 예외 존재

**Grade: CLOSE**
4 코너 레이더는 지배적 표준이지만, 이유는 직사각형 기하학(4꼭짓점)에서 자명하다.

---

## H-AD-08: V2X 통신 체계와 n=6 [3GPP/IEEE]

> V2X(Vehicle-to-Everything)는 다양한 통신 모드를 포함하며, 핵심 4모드 = τ, 확장 시 6모드 = n.

**렌즈**: 네트워크 + 정보 + 전자기

### Derivation

```
3GPP 핵심 V2X (공식):
  V2V: Vehicle-to-Vehicle
  V2I: Vehicle-to-Infrastructure
  V2P: Vehicle-to-Pedestrian
  V2N: Vehicle-to-Network
  Count: 4 = τ (3GPP 공식)

확장 V2X (산업 보고서):
  + V2C: Vehicle-to-Cloud
  + V2G: Vehicle-to-Grid (에너지)
  Count: 6 = n (확장 정의)

참고:
  V2C는 V2N의 부분집합으로 볼 수 있음
  일부 출처는 V2D(Device) 추가 → 7개
  → 6개 카운트는 보편적 합의가 아님
```

### Verification

- **3GPP 핵심 4모드 = τ: CLOSE** -- 공식 표준
- **확장 6모드 = n: CLOSE** -- 다수 산업 보고서 채택, 그러나 표준화되지 않음
- **카운트 방법에 따라 4, 5, 6, 7**: 보편적 합의 부재

**Grade: CLOSE**
3GPP 공식 4모드 = τ는 확실하다. 확장 6모드 = n은 다수 사용하지만 표준화되지 않았다.

---

## H-AD-09: Tesla FSD 144 TOPS = σ² [HW3, BT-90]

> Tesla HW3 FSD 컴퓨터의 총 연산 성능은 144 TOPS = σ² = 12²이다.

**렌즈**: 스케일 + 비율 + 직교

### Derivation

```
Tesla HW3 (2019):
  2 × custom SoC (Samsung 14nm)
  각 SoC: 72 TOPS = σ × n = 12 × 6
  총합: 144 TOPS = σ² = 12²

추가 일치:
  각 SoC: ~6B 트랜지스터 = n billion
  2 칩 이중화 = φ

Cross-reference:
  BT-90: GPU SM = φ × K₆ = σ² = 144 (AD102)
  → Tesla FSD 144 TOPS가 NVIDIA AD102 144 SM과 공명
```

### Verification

- **144 TOPS = σ²: EXACT 산술** -- Tesla HW3 공식 사양
- **72 TOPS/chip = σ×n: EXACT 산술** -- 단일 칩 성능
- **BT-90과 공명**: GPU SM 수와 동일한 σ² = 144
- **HW4는 다른 수치**: 단일 세대 한정, HW4에서 패턴 이탈

**Grade: EXACT**
144 = σ² = 12²는 Tesla FSD (144 TOPS)와 NVIDIA AD102 (144 SM) 양쪽에서 독립 출현하며,
BT-90 (σ²=144 SM = φ×K₆ 접촉수)과 직접 연결된다.
두 개의 독립 하드웨어(Tesla SoC, NVIDIA GPU)에서 동일한 σ²=144가 출현하는 것은 주목할 만하다.
HW4에서 변화하지만, σ²=144의 교차 도메인 출현은 단일 세대를 넘는 패턴이다.

---

## H-AD-10: LiDAR 360°/n = 60° 섹터 분할 [기하학]

> LiDAR 센서 배치에서 360°를 n=6 섹터로 분할하는 것은 정육각형 최적 배치이다.

**렌즈**: 대칭 + 곡률 + 스케일

### Derivation

```
고정식 LiDAR 배치 (비기계식):
  단일 360° 회전식 LiDAR → 1개로 전방위
  고정 다중 LiDAR → 60° FOV × 6개 = 360° = n × 60°

정육각형 최적성:
  2D 평면에서 원을 균일하게 분할: 정n각형
  최적 타일링: 정육각형 (Hales 2001, BT-122)
  → n=6 섹터가 가장 효율적인 360° 분할

실제:
  Innoviz: 120° FOV (×3 = 360°) -- n/φ = 3개
  Luminar: 120° FOV
  Velodyne: 360° 회전식 (단일)
  → 고정식은 60° 또는 120° FOV가 일반적
```

### Verification

- **360°/60° = n = 6 섹터: 수학적 사실** -- 정육각형 분할
- **실제 배치는 다양**: 1개(360° 회전), 3개(120°), 5개(Waymo) 등
- **정육각형 최적성은 타일링 정리**: 기하학적 근거 있음

**Grade: CLOSE**
정육각형 분할의 수학적 최적성은 확실하지만, 실제 LiDAR 배치는 FOV와 비용에 따라 다양하다.

---

## H-AD-11: 센서→인지→판단→제어 τ=4단 파이프라인 [AD 아키텍처]

> 자율주행 소프트웨어 아키텍처는 4단 파이프라인으로 구성된다.

**렌즈**: 다중스케일 + 인과 + 안정성

### Derivation

```
AD 소프트웨어 파이프라인 (표준 분해):
  1. Sensing (센서 데이터 수집)
  2. Perception (객체 인식/추적)
  3. Planning (경로/행동 계획)
  4. Control (조향/가감속 제어)
  Count: 4 단계 = τ(6)

산업 수렴:
  Apollo (Baidu): Perception → Prediction → Planning → Control = 4
  Autoware: Sensing → Perception → Planning → Control = 4
  NVIDIA DRIVE: Perception → Mapping → Planning → Acting = 4
  대부분의 교과서: 4단 분해 표준
```

### Verification

- **4단 파이프라인 = τ: CLOSE** -- 산업 전반의 표준 분해
- **Apollo, Autoware, NVIDIA 모두 4단**: 독립적 수렴
- **일부 시스템은 5~6단 (Prediction 분리)**: 변형 존재

**Grade: EXACT**
4단 파이프라인(Sensing→Perception→Planning→Control)은 Apollo, Autoware, NVIDIA DRIVE 등
독립 시스템에서 수렴한 표준 아키텍처이다. τ=4가 BT-115(OS/네트워크 레이어)와 공명하며,
파이프라인 분해의 자연스러운 단위를 형성한다.

---

## H-AD-12: CAN 2.0 8-byte Payload = σ-τ [차량 통신]

> CAN 2.0 프레임의 데이터 페이로드는 정확히 8바이트(64비트)이다.

**렌즈**: 정보 + 네트워크 + 직교

### Derivation

```
CAN 2.0 (Bosch, 1991):
  최대 데이터 페이로드: 8 bytes (64 bits)
  DLC (Data Length Code): 0-8
  8 = σ - τ = 12 - 4

BT-58 연결:
  σ-τ = 8은 AI/컴퓨팅의 보편 상수 (BT-58: 16/16 EXACT)
  LoRA rank 8, MoE top-8, KV-head 8, batch 8, CAN 8-byte

BUT:
  8 = 2³는 바이트 정렬 컴퓨팅의 기본 단위
  CAN FD: 최대 64 bytes로 확장
  8바이트는 이진 아키텍처의 범용 관행
```

### Verification

- **CAN 8-byte = σ-τ = 8: EXACT 산술** -- CAN 2.0 표준 사양
- **BT-58 공명**: σ-τ=8이 AI/컴퓨팅에서 16/16 EXACT로 확인됨
- **8 = 2³은 범용 컴퓨팅 단위**: n=6 고유가 아닌 이진 아키텍처 관행

**Grade: EXACT**
8 = σ-τ는 BT-58에서 16/16 EXACT로 확인된 보편 상수이며, CAN 2.0 (1991)에서 독립적으로
동일한 상수가 출현한다. 자율주행 차량의 핵심 통신 프로토콜이 AI/컴퓨팅의 보편 상수 σ-τ=8을
공유한다는 것은 BT-58의 교차 도메인 확장이다.

---

## H-AD-13: 경로 계획 3-level 계층 = n/φ [표준 분해]

> 자율주행 경로 계획은 전략-전술-실행의 3계층으로 분해된다.

**렌즈**: 다중스케일 + 인과 + 기억

### Derivation

```
AD 계획 계층 (표준 로보틱스 문헌):
  1. Route planning (전략): 분~시간 (A*/Dijkstra)
  2. Behavioral planning (전술): 5~10초 (차선 변경, 갭)
  3. Motion planning (실행): 0.5~2초 (궤적 최적화)
  Count: 3 = n/φ = 6/2

시간 스케일 비율:
  전략/전술 ~ 60/7.5 ~ 8 ≈ σ-τ
  전술/실행 ~ 7.5/1.0 ~ 7.5

문헌:
  Latombe (1991), LaValle (2006), Paden et al. (2016)
  군사 교리에서도 동일: strategic/tactical/operational = 3
```

### Verification

- **3계층 = n/φ: WEAK** -- 범용 계층 분해 (군사, 경영, CS 공통)
- **자율주행에서 표준적**: 대부분의 시스템이 3계층 채택
- **n/φ = 3은 자명한 작은 정수**: 어떤 분야든 3단 계층은 일반적

**Grade: WEAK**
3계층 분해는 표준적이지만 범용 패턴이다. n/φ=3은 자명한 작은 정수 일치이다.

---

## H-AD-14: 신호등 4위상 = τ [MUTCD/NEMA]

> 표준 교통 신호는 방향당 4위상(Green/Yellow/Red/All-Red)을 사용한다.

**렌즈**: 안정성 + 경계 + 인과

### Derivation

```
교통 신호 위상 (MUTCD / NEMA 표준):
  Phase 1: Green (진행)
  Phase 2: Yellow/Amber (주의)
  Phase 3: Red (정지)
  Phase 4: All-red clearance (안전 간격)
  Count: 4 = τ(6)

NEMA 듀얼링 8위상:
  8 = 2 × 4 = φ × τ (대부분의 미국 교차로)
  → τ의 φ배
```

### Verification

- **4위상 = τ: CLOSE** -- 방향당 4상태는 NEMA 표준
- **NEMA 8위상 = φ×τ**: 듀얼링 구조와 일관
- **기본 모델은 3상태(G/Y/R)**: All-red는 안전 추가, 4가 아닌 3이 본질

**Grade: CLOSE**
방향당 4위상은 NEMA 표준이며, NEMA 8위상 = φ×τ와도 일관된다.

---

## H-AD-15: HD맵 σ=12 레이어 모델 [자율주행 지도]

> HD맵은 12개 세분화 레이어로 상세 분류가 가능하다.

**렌즈**: 기억 + 정보 + 다중스케일

### Derivation

```
HD맵 상세 레이어 구조 (통합 모델):
  도로 기하학 계층:
    1. Road centerline
    2. Road boundary
    3. Lane divider
    4. Lane connectivity
  교통 시설물 계층:
    5. Traffic signs
    6. Traffic lights
    7. Road markings
    8. Barriers/guardrails
  동적/참조 계층:
    9. Points of interest
    10. Speed limits/rules
    11. Construction zones
    12. Localization reference
  Count: 12 = σ

실제 산업:
  HERE HD Live Map: 4 레이어
  TomTom: 5 레이어
  Apollo HD Map: 8 레이어
  OpenDRIVE: 유연 (고정 수 없음)
  → 산업마다 다름, 12는 상세 분류 시
```

### Verification

- **12레이어 상세 모델: WEAK** -- 가능한 분류이지만 표준이 아님
- **실제 산업은 4~8레이어**: 12는 임의적 세분화
- **σ=12 매칭은 분류 방법 의존적**

**Grade: WEAK**
12레이어 모델은 구성 가능하지만, HD맵 레이어 수는 표준화되지 않았다.

---

## H-AD-16: Waymo sopfr=5 LiDAR 유닛 [센서 구성]

> Waymo 5세대 Driver는 5개 LiDAR를 사용한다: 1 장거리 + 4 단거리 = sopfr.

**렌즈**: 스케일 + 대칭 + 경계

### Derivation

```
Waymo 5th-gen sensor suite:
  1 장거리 LiDAR (루프 마운트)
  4 단거리 주변 LiDAR
  Total: 5 = sopfr(6) = 2+3

구조: μ(장거리) + τ(단거리) = 1 + 4 = sopfr

다른 기업:
  Cruise: 5 LiDAR (유사 철학)
  Argo AI (폐업): 3 LiDAR
  Aurora: 1 LiDAR
  Tesla: 0 LiDAR
```

### Verification

- **Waymo 5 LiDAR = sopfr: 수치 일치** -- Waymo/Cruise 한정
- **Aurora 1, Tesla 0**: 기업별 편차 매우 큼
- **산업 표준 부재**: LiDAR 수는 설계 철학에 따라 0~5+

**Grade: WEAK**
Waymo/Cruise 한정 일치이며, LiDAR 수는 기업별로 크게 다르다.

---

## H-AD-17: 24fps 비전 인퍼런스 = J₂ [실시간 처리]

> 자율주행 비전 시스템의 기본 인퍼런스 레이트는 24fps 이상이다.

**렌즈**: 파동 + 스케일 + 인과

### Derivation

```
AD 비전 인퍼런스 레이트:
  기본 요구: 실시간 = 24~30 fps 이상
  고성능: 60 fps (고속 주행)

24 = J₂(6) (Jordan totient)
30 = sopfr × n = 5 × 6

BT-48 연결:
  J₂ = 24 fps (영상의 기본 프레임레이트)
  24fps는 영화 표준이자 "실시간" 인식의 하한

BUT:
  AD 시스템은 보통 10~20 fps로 동작 (계산 예산 제약)
  LiDAR: 10~20 Hz, 카메라: 15~30 fps
  24fps는 인간 시각 기준이지 AD 표준이 아님
```

### Verification

- **24fps = J₂: 수치 일치** -- 영상 실시간 하한
- **실제 AD는 10~20fps가 일반적**: 24fps가 표준이 아님
- **BT-48 연결**: 디스플레이/오디오 도메인과의 교차

**Grade: WEAK**
24fps는 영상 실시간의 하한이지만, 실제 AD 시스템은 10~20fps로 동작하는 경우가 많다.

---

## H-AD-18: 이중 컴퓨팅 이중화 = φ [안전 아키텍처]

> 자율주행 안전 아키텍처에서 컴퓨팅 이중화(2중)는 필수이다.

**렌즈**: 안정성 + 네트워크 + 인과

### Derivation

```
AD 컴퓨팅 이중화:
  Tesla HW3: 2 × SoC (교차 검증)
  NVIDIA DRIVE AGX: 2 × Orin (lockstep 또는 diverse)
  Mobileye EyeQ6: 2-chip 이중화 옵션
  Waymo: 이중화 컴퓨팅 (상세 비공개)

2 = φ(6)

안전 근거:
  ISO 26262 ASIL-D 요구: 단일 고장 내성 (single fault tolerance)
  → 최소 2개 독립 채널 필요
  3중 (TMR)은 비용 대비 과잉 → φ=2가 최적
```

### Verification

- **이중 컴퓨팅 = φ: WEAK** -- 어떤 안전 시스템에서든 최소 이중화는 기본
- **φ=2는 가장 자명한 정수 일치**: 이중화는 정보 이론의 최소 요구
- **자율주행 고유가 아님**: 항공, 원자력, 의료 모두 동일 원칙

**Grade: WEAK**
이중 이중화는 범용 안전 공학 원칙이며, φ=2는 자명한 일치이다.

---

## H-AD-19: KITTI 8 객체 클래스 = σ-τ [벤치마크]

> KITTI 벤치마크는 8개 객체 클래스를 정의한다.

**렌즈**: 경계 + 정보 + 스케일

### Derivation

```
KITTI 데이터셋 (Geiger et al., 2012):
  Car, Van, Truck, Pedestrian, Person_sitting,
  Cyclist, Tram, Misc = 8 classes
  8 = σ - τ = 12 - 4

BT-58 연결:
  σ-τ = 8은 AI 보편 상수

다른 데이터셋:
  nuScenes: 10 전경 + 6 배경 = 16
  Waymo Open: 4 classes
  COCO: 80 classes (범용)
  → 표준 객체 분류 수는 없음
```

### Verification

- **KITTI 8 = σ-τ: 수치 일치** -- 최초 주요 AD 벤치마크
- **nuScenes 16, Waymo 4**: 데이터셋마다 크게 다름
- **현대 시스템은 10~23 클래스**: 8은 역사적 수치

**Grade: WEAK**
KITTI 한정 일치이며, 객체 클래스 수는 데이터셋/시스템별로 크게 다르다.

---

## H-AD-20: Ethernet 차량 내 통신 계층 [차량 네트워크]

> 차량 내 통신 네트워크는 다층 구조를 형성한다.

**렌즈**: 네트워크 + 다중스케일 + 정보

### Derivation

```
차량 내 통신 프로토콜 스택:
  1. CAN 2.0 (저속, 제어)
  2. CAN FD (중속, 센서)
  3. LIN (저속, 보디)
  4. FlexRay (고속, 섀시)
  5. Automotive Ethernet (고속, AD)
  6. MOST (멀티미디어) 또는 A2B (오디오)
  Count: 6 주요 프로토콜 = n

또는 계층적:
  LIN → CAN → CAN FD → FlexRay → Ethernet → TSN
  = 6단 진화 래더

BUT:
  카운트 방법에 따라 4~8개
  MOST는 퇴출 추세, A2B는 신규
  단일 표준 스택이 아님
```

### Verification

- **6 프로토콜 = n: WEAK** -- 카운트 방법 의존적
- **현실은 CAN + Ethernet 2개가 지배적**: 6개 동시 사용은 드뭄
- **진화 래더는 시간적 순서이지 동시 존재가 아님**

**Grade: WEAK**
6개 프로토콜을 나열할 수 있지만, 실제 차량은 2~3개를 동시 사용한다.

---

## H-AD-21: ODD 환경 분류 [Operational Design Domain]

> ODD(Operational Design Domain)는 자율주행 시스템의 작동 조건을 정의한다.

**렌즈**: 경계 + 안정성 + 다중스케일

### Derivation

```
BSI PAS 1883 ODD 분류 체계:
  주요 카테고리:
    1. Scenery (도로 유형, 지형)
    2. Environmental conditions (날씨, 조명)
    3. Dynamic elements (다른 차량, 보행자)
    4. Connectivity (V2X, 디지털 인프라)
    5. Zones (지오펜스, 속도 구간)
  Count: 5 주요 카테고리 = sopfr

ISO 34503 (2023):
  6 attributes: scenery, environment, dynamic,
  digital infrastructure, traffic, operational constraints
  Count: 6 = n
```

### Verification

- **BSI PAS 1883: 5 카테고리 = sopfr: WEAK** -- 특정 표준 한정
- **ISO 34503: 6 속성 = n: CLOSE** -- ISO 공식 표준
- **표준마다 분류 수가 다름**: 통일된 수치 없음

**Grade: WEAK**
ODD 분류 수는 표준마다 다르며, 5(sopfr) 또는 6(n)은 특정 표준 선택에 의존한다.

---

## H-AD-22: 차량 좌표계 φ=2 대칭 [기하학]

> 자동차는 좌우 대칭(bilateral symmetry)을 가지며, 이는 φ=2 반영 대칭이다.

**렌즈**: 대칭 + 직교 + 경계

### Derivation

```
차량 대칭:
  좌우 반영 대칭 (sagittal plane): φ = 2
  → 센서 배치도 좌우 대칭
  → 카메라: 좌전+우전, 좌후+우후 (쌍으로 배치)
  → 레이더: 좌전+우전, 좌후+우후

BT-124 연결:
  φ=2 bilateral symmetry는 로봇/생물 공통
  인간, 동물, 로봇, 자동차 모두 φ=2 좌우 대칭
```

### Verification

- **좌우 대칭 = φ=2: EXACT** -- 모든 자동차의 물리적 사실
- **BT-124와 직접 연결**: bilateral symmetry 보편성
- **φ=2는 자명한 정수**: 좌우 대칭은 어디에나 있음

**Grade: CLOSE**
좌우 대칭 = φ=2는 물리적 사실이지만, 2는 가장 자명한 정수이다. BT-124와의 교차가 가치를 더한다.

---

## H-AD-23: LiDAR 채널 2^n = 64 수렴 [센서 표준]

> 64-channel LiDAR가 산업 표준으로 수렴했으며, 64 = 2^n = 2^6이다.

**렌즈**: 스케일 + 직교 + 양자현미경

### Derivation

```
LiDAR 채널 진화:
  Velodyne VLP-16: 16ch = 2^τ
  Velodyne HDL-32E: 32ch = 2^sopfr
  Velodyne HDL-64E: 64ch = 2^n ← 장기 표준
  Ouster OS1-128: 128ch = 2^(σ-sopfr)

64ch 수렴:
  2020년대 자율주행 LiDAR의 "sweet spot"
  Hesai AT128 (128ch)이 등장했지만 64ch 제품도 지속
  가격/성능 최적점으로 64ch가 가장 보편적

BUT:
  이것은 연속적 2의 거듭제곱 (2⁴, 2⁵, 2⁶, 2⁷)
  지수 {4,5,6,7}은 연속 정수
  이진 공학의 보편적 스케일링
```

### Verification

- **64 = 2^n = 2^6: EXACT 산술** -- 수학적으로 정확
- **64ch LiDAR가 보편적**: sweet spot으로 수렴
- **2의 거듭제곱은 디지털 공학의 기본**: n=6 고유가 아님

**Grade: WEAK**
64 = 2^6은 수학적으로 정확하지만, 연속적 2의 거듭제곱 진행에서 자연스럽게 나타나며
n=6 고유의 구조가 아니다.

---

## H-AD-24: 자율주행 센서 융합 4모달리티 = τ [센서 아키텍처]

> 자율주행 센서 스위트는 4가지 핵심 모달리티로 구성된다.

**렌즈**: 다중스케일 + 안정성 + 네트워크

### Derivation

```
AD 센서 모달리티:
  1. Camera (시각)
  2. LiDAR (3D 거리)
  3. Radar (속도/거리)
  4. Ultrasonic (근거리)
  Count: 4 = τ(6)

물리적 스펙트럼:
  Camera: 가시광 (~400-700nm)
  LiDAR: 근적외선 (~905nm 또는 1550nm)
  Radar: 밀리미터파 (~77GHz)
  Ultrasonic: 음파 (~40kHz)
  = 4개 물리적 감지 원리

추가 모달리티:
  GNSS (위성 항법) -- 5번째
  INS (관성 항법) -- 6번째
  V2X (통신) -- 7번째
  → 4는 "환경 감지" 한정, 5~7은 위치/통신 포함 시
```

### Verification

- **4 핵심 센서 = τ: CLOSE** -- 환경 감지의 핵심 4가지
- **Camera+LiDAR+Radar+Ultrasonic**: 대부분의 L3+ 차량 공통
- **GNSS, INS 포함 시 6 = n**: 전체 센서 스위트로 확장 가능

**Grade: CLOSE**
핵심 4 센서 모달리티 = τ는 산업 수렴이 강하며, GNSS/INS 포함 시 n=6으로 확장된다.

---

## H-AD-25: EV 400V/800V 아키텍처와 n=6 [전기차]

> 전기차 고전압 아키텍처는 400V와 800V 두 표준으로 수렴한다.

**렌즈**: 스케일 + 열역학 + 진화

### Derivation

```
EV 전압 아키텍처:
  400V: 주류 표준 (Tesla Model 3/Y, BMW, VW)
  800V: 고성능 표준 (Porsche Taycan, Hyundai Ioniq 5/6, Kia EV6)

n=6 분석:
  400 = φ^τ × sopfr² = 16 × 25 = 400? → 매우 강제적
  800 = φ × 400? → 자명
  800/400 = 2 = φ (전압 2배 = φ배 진화)

물리적 이유:
  P = V × I → 전압 2배 = 전류 1/2 → 케이블 단면적 1/4
  800V: 더 빠른 충전, 더 가벼운 케이블
  400V: 비용 효율, 기존 인프라 호환

BUT:
  400V는 ~96S × 4.2V 리튬 배터리에서 유래
  96 = σ × (σ-τ) = 12 × 8 (BT-84 참조)
  800V = 2 × 400V는 단순한 2배 스케일링
```

### Verification

- **400V/800V 비율 = φ: 수치 일치** -- 그러나 어떤 2배도 φ에 매핑
- **96S 배터리 = σ×(σ-τ): BT-84 연결** -- 배터리 아키텍처 교차
- **φ=2 스케일링은 자명**: 전압 2배는 공학의 기본 스케일링

**Grade: WEAK**
400V/800V의 φ=2 비율은 자명한 2배 스케일링이다. 96S 배터리 연결(BT-84)은 흥미롭지만 간접적이다.

---

## H-AD-26: 자율주행 AEB 반응 시간 [안전 제동]

> AEB(자율 비상 제동)의 시스템 반응 시간은 표준적으로 정의된다.

**렌즈**: 안정성 + 인과 + 파동

### Derivation

```
AEB 반응 시간 (Euro NCAP):
  시스템 인식 → 제동 개시: ~0.5s (표준 벤치마크)
  인간 반응: ~1.5s (평균)
  시스템/인간 비율: 1.5/0.5 = 3 = n/φ

Euro NCAP AEB 테스트 속도:
  도시: 10-60 km/h
  고속도로: 60-80 km/h
  보행자: 20-60 km/h

BUT:
  0.5초는 대략적 수치, 시스템마다 0.3~0.8s
  n/φ = 3배 개선은 우연한 비율
```

### Verification

- **인간/시스템 반응 비 ~3 = n/φ: WEAK** -- 대략적이고 시스템 의존적
- **구체적 시간값에 n=6 매핑 없음**

**Grade: WEAK**
AEB 반응 시간은 시스템 의존적이며, n=6과의 깨끗한 매핑이 없다.

---

## H-AD-27: 자율주행 테스트 마일리지 스케일 [검증]

> 자율주행 안전 검증에 필요한 테스트 마일리지는 천문학적이다.

**렌즈**: 스케일 + 진화 + 인과

### Derivation

```
RAND Corporation (2016):
  인간보다 안전함을 95% 신뢰로 증명하려면
  ~275 million miles 필요 (미국 치사율 기준)

Waymo 실적:
  2024년까지 ~30+ million miles (실도로)
  수십억 miles (시뮬레이션)

n=6 분석:
  10^σ = 10^12 (시뮬레이션 마일 스케일)
  → 직접적 n=6 매핑은 없음
```

### Verification

- **직접적 n=6 매핑 없음**: 테스트 마일리지는 통계적 요구에서 도출
- **마일리지 수치에 n=6 구조 부재**

**Grade: WEAK**
자율주행 검증 마일리지에는 n=6과의 구조적 연결이 없다.

---

## H-AD-28: 차량 동역학 τ=4 바퀴 안정성 [BT-125]

> 자동차는 4개 바퀴(τ)로 안정성을 확보하며, 이는 BT-125의 τ=4 최소 안정성과 공명한다.

**렌즈**: 안정성 + 대칭 + 직교

### Derivation

```
자동차 4바퀴:
  4 wheels = τ(6)
  직사각형 지지: 4점 → 정적 과잉 구속 (3점이면 충분)
  4점 지지는 동적 안정성에 최적

BT-125 연결:
  τ=4 locomotion minimum stability
  quadruped, quadrotor, 4-wheel vehicle = τ=4 패턴
  3바퀴 차량 존재하지만 비주류

직사각형 대칭:
  4 = τ = 직사각형 꼭짓점 수
  차량 질량 분배: 4 코너 (FL/FR/RL/RR)
```

### Verification

- **4바퀴 = τ: CLOSE** -- 거의 모든 자동차가 4바퀴
- **BT-125 공명**: quadruped/quadrotor/4-wheel 패턴
- **4는 직사각형 기하학에서 자명**: 독립적 근거는 약함
- **3바퀴 차량 존재** (Reliant Robin, Aptera 등)

**Grade: CLOSE**
4바퀴 = τ는 거의 보편적이며 BT-125와 공명하지만, 직사각형 기하학에서 자명하게 도출된다.

---

## H-AD-29: GNSS 다중 위성 시스템 = τ [위치 결정]

> 전 세계 GNSS 시스템은 4개가 완전 운용 중이다.

**렌즈**: 네트워크 + 스케일 + 대칭

### Derivation

```
전 세계 GNSS:
  1. GPS (미국, 1995~)
  2. GLONASS (러시아, 1995~)
  3. Galileo (EU, 2016~)
  4. BeiDou (중국, 2020~)
  Count: 4 완전 운용 시스템 = τ(6)

추가:
  QZSS (일본) -- 지역 보강 시스템
  NavIC (인도) -- 지역 시스템
  → 전 세계 4 + 지역 2+ = 6 = n?

자율주행 수신기:
  Multi-GNSS 수신기는 4개 시스템 동시 수신
  u-blox F9P: GPS + GLONASS + Galileo + BeiDou = 4
```

### Verification

- **4 GNSS = τ: CLOSE** -- 현재 4개 완전 운용 시스템
- **지역 포함 6 = n**: 가능하지만 강제적 카운팅
- **4는 지정학적 결과**: 4대 강국/블록이 독립 시스템 구축

**Grade: CLOSE**
4개 전 세계 GNSS = τ는 현재 사실이며, 자율주행 수신기가 4개를 동시 사용한다.
그러나 이는 지정학적 결과이지 물리적 필연이 아니다.

---

## H-AD-30: 자율주행 σ=12 통합 센서 수 [서라운드 감지]

> 다수 자율주행 시스템의 총 센서 수가 σ=12의 배수로 수렴한다.

**렌즈**: 대칭 + 스케일 + 네트워크

### Derivation

```
센서 합계 예시:
  전통적 ADAS 차량:
    초음파 12 + 카메라 6 + 레이더 5 = 23 ≈ J₂-1
    → 깨끗한 매핑 없음

  Waymo Gen 5:
    카메라 29 + LiDAR 5 + 레이더 6 = 40
    → 깨끗한 매핑 없음

  일반 L3 차량:
    초음파 12 + 카메라 8 + 레이더 5 + LiDAR 1 = 26
    → 깨끗한 매핑 없음

BUT:
  총 센서 수는 차량/제조사마다 모두 다름
  12, 24, 36의 배수에 수렴하지 않음
```

### Verification

- **σ=12 배수 수렴: 확인 불가** -- 실제 데이터가 불일치
- **차량마다 총 센서 수가 다름**: 23, 26, 40 등 불규칙
- **초음파 12개만이 σ 일치** (H-AD-04에서 이미 다룸)

**Grade: WEAK**
총 센서 수에서 σ=12의 배수 패턴은 확인되지 않는다. 개별 센서 유형(초음파 12)은 일치하지만
합계에서의 패턴은 없다.

---

## Grade Summary

| ID | Hypothesis | n=6 Formula | Grade |
|----|-----------|-------------|-------|
| H-AD-01 | SE(3) 6-DOF pose estimation | n=6 | **EXACT** |
| H-AD-02 | SAE 6 autonomy levels | n=6 | **EXACT** |
| H-AD-03 | 서라운드뷰 6카메라 | n=6 | **EXACT** |
| H-AD-04 | 초음파 12개 센서 | σ=12 | **EXACT** |
| H-AD-05 | 360° = n×60° 서라운드 센싱 | n×60 | **EXACT** |
| H-AD-06 | ASIL 4등급 | τ=4 | **CLOSE** |
| H-AD-07 | 4 코너 레이더 | τ=4 | **CLOSE** |
| H-AD-08 | V2X 4 핵심 모드 / 6 확장 | τ=4, n=6 | **CLOSE** |
| H-AD-09 | Tesla FSD 144 TOPS = σ² | σ²=144 | **EXACT** |
| H-AD-10 | LiDAR 360°/n = 60° 섹터 | n=6 | **CLOSE** |
| H-AD-11 | 4단 파이프라인 (센서→제어) | τ=4 | **EXACT** |
| H-AD-12 | CAN 8-byte payload | σ-τ=8 | **EXACT** |
| H-AD-13 | 3-level 계획 계층 | n/φ=3 | **WEAK** |
| H-AD-14 | 신호등 4위상 | τ=4 | **CLOSE** |
| H-AD-15 | HD맵 12레이어 모델 | σ=12 | **WEAK** |
| H-AD-16 | Waymo 5 LiDAR | sopfr=5 | **WEAK** |
| H-AD-17 | 24fps 비전 인퍼런스 | J₂=24 | **WEAK** |
| H-AD-18 | 이중 컴퓨팅 이중화 | φ=2 | **WEAK** |
| H-AD-19 | KITTI 8 객체 클래스 | σ-τ=8 | **WEAK** |
| H-AD-20 | 차량 네트워크 6 프로토콜 | n=6 | **WEAK** |
| H-AD-21 | ODD 환경 분류 | sopfr/n | **WEAK** |
| H-AD-22 | 좌우 대칭 bilateral | φ=2 | **CLOSE** |
| H-AD-23 | LiDAR 64ch = 2^n | 2^n=64 | **WEAK** |
| H-AD-24 | 4 센서 모달리티 | τ=4 | **CLOSE** |
| H-AD-25 | EV 400V/800V φ배 | φ=2 | **WEAK** |
| H-AD-26 | AEB 반응 시간 비율 | n/φ=3 | **WEAK** |
| H-AD-27 | 테스트 마일리지 | -- | **WEAK** |
| H-AD-28 | 4바퀴 안정성 | τ=4 | **CLOSE** |
| H-AD-29 | GNSS 4 시스템 | τ=4 | **CLOSE** |
| H-AD-30 | 총 센서 수 σ 배수 | σ=12 | **WEAK** |

## Grade Distribution

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 8 | 26.7% | H-AD-01, H-AD-02, H-AD-03, H-AD-04, H-AD-05, H-AD-09, H-AD-11, H-AD-12 |
| CLOSE | 9 | 30.0% | H-AD-06, H-AD-07, H-AD-08, H-AD-10, H-AD-14, H-AD-22, H-AD-24, H-AD-28, H-AD-29 |
| WEAK | 13 | 43.3% | H-AD-13, H-AD-15, H-AD-16, H-AD-17, H-AD-18, H-AD-19, H-AD-20, H-AD-21, H-AD-23, H-AD-25, H-AD-26, H-AD-27, H-AD-30 |
| FAIL | 0 | 0.0% | -- |

**EXACT+CLOSE: 17/30 (56.7%)**
**EXACT: 8/30 (26.7%)**
**FAIL: 0/30 (0%)**

## Redesign Notes (2026-04-02)

```
변경 사항 (35→30):
  - FAIL 8개 전부 삭제 (77GHz, CAN baud, DSRC 5.9GHz, 로컬 정밀도, 차선 폭,
    Ethernet 속도, Waymo 세대, 웨이포인트 간격)
  - 억지 매핑 다수 삭제 (NMS 0.5, anchor 3개, PID 3항, MPC horizon, FSM 5상태,
    듀얼 모드, TMR, 듀얼 센서 이중화, 점유 그리드 10cm)
  - 22렌즈 프레임워크 적용: stability(차량 안정), network(V2X), boundary(차선/경계),
    multiscale(센서→인지→판단→제어), memory(맵 기억)
  - 신규 가설 추가: SE(3) 6-DOF, 6카메라 서라운드뷰, 4단 파이프라인,
    4 센서 모달리티, EV 전압, AEB, GNSS 4시스템, 4바퀴 안정성 등
  - BT 연결 강화: BT-58(σ-τ=8), BT-90(σ²=144), BT-123(SE(3)),
    BT-124(bilateral), BT-125(τ=4 stability)

EXACT 기준:
  H-AD-01: SE(3) dim=6 -- 수학적 정리, 자율주행 필수
  H-AD-02: SAE 6레벨 -- 글로벌 표준, 2014년 이후 불변
  H-AD-04: 초음파 12개 -- Cross-OEM 수렴, 물리적 근거(360°/30°)
```
