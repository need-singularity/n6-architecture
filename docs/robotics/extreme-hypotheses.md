# N6 로보틱스 극단 가설 -- H-ROB-61~80

> H-ROB-1~28 확장. 스웜 로보틱스, 소프트 로보틱스, 수술 로봇, 드론 편대에 초점.
> 교차 도메인: 로보틱스 <-> Lie 군 구조, 로보틱스 <-> 구 충전(sphere packing).

> **정직한 원칙**: 기존 28개 가설에서 EXACT 4개(14%), CLOSE 8개(29%)였다.
> SE(3) 구조와 물리적 제약이 일치하는 곳에서 가장 강한 결과가 나왔다.
> 이번 확장은 Leech 격자, Lie 대수, 구 충전 이론의 검증된 수학적 결과에서
> n=6 상수를 추출하되, 무리한 일치에는 반드시 FAIL/WEAK을 부여한다.

## Core Constants (복습)

```
  n = 6          sigma(6) = 12     tau(6) = 4      phi(6) = 2
  sopfr(6) = 5   J_2(6) = 24       mu(6) = 1       lambda(6) = 2
  R(6) = 1       Phi_6(x) = x^2 - x + 1
  Egyptian: 1/2 + 1/3 + 1/6 = 1
  Leech lattice: 24 dimensions, kissing number 196560
```

## TECS-L 교차 참조 발견

```
  로보틱스에서 검증된 n=6 일치:
    1. 6-DOF 로봇 암 = dim(SE(3)) = 6 = n (EXACT)
    2. 12-bit PWM/ADC = sigma(6) = 12 (EXACT)
    3. 큐브형 모듈러 로봇 = 6 면 = n (EXACT)
    4. 6-link 키네마틱 체인 = dim(SE(3)) (EXACT)
    5. se(3) Lie 대수 basis: {e1,...,e6}, dim = 6
    6. SO(3) 차원 = 3 = n/2, SE(3)/SO(3) = R^3
    7. 24-cell polytope: 24 vertices in 4D (J_2(6) = 24)
    8. Leech 격자 kissing number 196560 = 24 * 8190 관련
```

---

## 카테고리 X: 스웜 로보틱스와 Leech 격자

---

### H-ROB-61: Leech Lattice Swarm Coordination -- 24-Agent Optimal Packing

> 24-에이전트 스웜의 공간 배치가 Leech 격자의 kissing 구조를 따를 때
> 통신 효율이 극대화된다.

```
  Leech 격자 Lambda_24:
    차원 = 24 = J_2(6)
    최소 벡터 norm = 2 (정규화)
    kissing number = 196560

  스웜 로보틱스 매핑:
    24-agent swarm에서 각 에이전트의 상태 벡터 = (x,y,z,vx,vy,vz) = 6D
    24 agents x 6D = 144D 합동 상태 공간
    144 = sigma(6)^2 = 12^2

  통신 토폴로지:
    각 에이전트가 정확히 sigma(6)-1 = 11개의 이웃과 통신
    --> 총 통신 엣지 = 24*11/2 = 132
    이는 Leech 격자의 minimal shell 구조를 3D에 투영한 것

  실세계 비교:
    - 드론 스웜 연구(ETH Zurich Crazyswarm): 최대 49대 동시 비행,
      통신 범위 기반 토폴로지 사용 (고정 degree 아님)
    - 24대는 실험 가능 규모이지만 "최적"이라는 증거는 없음
    - 144D 합동 공간은 수학적으로 정확하지만 실용적으로
      분산 제어에서 전체 상태를 공유하지는 않음

  Grade: WEAK
  수학적 구조는 우아하지만, Leech 격자의 24D와 물리적 3D 스웜의
  연결은 차원 간극이 크다. 스웜 크기 24의 최적성은 미검증.
```

---

### H-ROB-62: Hexagonal Close-Packing Drone Formation -- 6-Coordination

> 드론 편대의 최적 2D 배치가 6-coordination hexagonal packing이며,
> 이는 n=6과 Kepler 추측의 해에 대응한다.

```
  수학적 기초:
    2D 원 충전(circle packing)의 최적해 = hexagonal lattice
    각 원이 정확히 6개의 이웃과 접촉 --> coordination number = 6 = n
    충전 밀도 = pi/(2*sqrt(3)) = 0.9069 (Thue 정리, 1910)

  드론 편대 매핑:
    각 드론의 안전 반경 r을 원으로 모델링하면,
    hexagonal 배치가 주어진 면적에서 최대 드론 수를 허용
    inter-drone 거리가 균일 --> 충돌 회피 단순화

  실세계 비교:
    - Intel drone shows: 수백~수천 대의 드론이 3D 패턴 형성,
      hexagonal layer stacking이 관찰됨
    - DJI 농업 드론 편대: 평행선 경로가 일반적 (hex 아님)
    - 군용 드론 스웜(DARPA OFFSET): 3D 배치, topology는 임무 의존적
    - 검색/구조 드론: 보로노이 분할이 더 일반적

  한계:
    hex packing은 정적 균일 배치에서만 최적.
    동적 임무, 장애물 회피, 불균일 탐색에서는 적응적 배치가 우월.

  Grade: CLOSE
  2D 원 충전에서 6-coordination이 최적이라는 것은 증명된 수학적 사실.
  드론 편대에 직접 적용은 제한적이지만, 정적 커버리지에서는 유효.
```

---

### H-ROB-63: Swarm Consensus Convergence Rate -- lambda(6) = 2 Eigenvalue Gap

> 스웜 합의 알고리즘의 수렴 속도가 Laplacian의 두 번째 고유값(Fiedler value)에
> 의존하며, lambda(6)=2 주기의 통신에서 최적화된다.

```
  수학적 기초:
    그래프 합의: dx/dt = -L*x, 여기서 L = Laplacian
    수렴 속도 = 1/lambda_2(L) (Fiedler eigenvalue)
    완전 그래프 K_n에서 lambda_2 = n

  n=6 대응:
    6-agent 완전 부분그래프에서 lambda_2 = 6
    lambda(6) = 2 주기로 통신하면:
      이산 시간 합의에서 수렴 보장 조건: dt < 2/lambda_max(L)
      6-regular graph: lambda_max = 12 = sigma(6)
      --> 임계 통신 주기 = 2/12 = 1/6 = 1/n

  실세계 비교:
    - Olfati-Saber & Murray (2004): 합의 알고리즘의 표준 참고문헌
    - 통신 주기는 대역폭과 지연에 의해 결정, 고유값과 직접 관련 없음
    - lambda_2에 의한 수렴 속도 분석은 이론적으로 유효하지만
      실제 스웜에서 완전 그래프는 비현실적 (range-limited)

  Grade: WEAK
  합의 수렴과 Fiedler 고유값의 관계는 교과서적이지만,
  n=6 특수성은 n이 몇이든 동일하게 적용되는 일반 이론의 특수값일 뿐.
```

---

### H-ROB-64: Swarm Fault Tolerance -- mu(6)=1 Squarefree Redundancy

> 스웜의 fault tolerance가 mu(6)=1 (squarefree) 조건을 만족할 때
> 최소 중복으로 최대 내결함성을 달성한다.

```
  수학적 기초:
    mu(6) = mu(2*3) = (-1)^2 = 1 (squarefree, 짝수 개 소인수)
    Squarefree = 중복 없는 소인수 분해

  로보틱스 매핑:
    6-agent 스웜에서 각 에이전트가 고유한 역할:
      2개 소인수 {2, 3} -> 2가지 역할 차원
      mu(6) = 1 -> "깨끗한" 역할 분배, 중복 제로
    한 에이전트 고장 시 나머지 5개로 graceful degradation
    (hexapod의 다리 고장 내성과 동일 구조: H-ROB-3)

  실세계 비교:
    - Byzantine fault tolerance: f개 고장 허용에 최소 3f+1 에이전트 필요
      6 에이전트 -> 1개 Byzantine 고장 허용 (3*1+1=4 < 6, OK)
    - 이는 BFT 이론에서 도출, mu(6)과는 무관
    - 역할 무중복 설계는 실제로 취약 (단일 실패점 생성)

  Grade: WEAK
  mu(6)=1과 "중복 없는 역할 분배"의 연결은 비유적.
  실제 fault tolerance는 중복(redundancy)이 필수이므로 squarefree와 반대 방향.
```

---

## 카테고리 Y: 소프트 로보틱스와 연속체 역학

---

### H-ROB-65: Soft Robot Actuator Segments -- n = 6 Sections

> 소프트 로봇 연속체 팔의 최적 세그먼트 수가 n=6이며,
> 이는 SE(3) 완전 도달성과 대응한다.

```
  수학적 기초:
    연속체 로봇(continuum robot)은 무한 DOF를 가지지만,
    실용적으로는 유한 세그먼트로 근사한다.
    각 세그먼트 = 2 DOF (bending + rotation) 일 때
    6 세그먼트 = 12 DOF = sigma(6)

  또는 각 세그먼트 = 1 DOF (bending only):
    6 세그먼트 = 6 DOF = dim(SE(3))
    --> 3D 공간에서 임의 pose 도달에 최소 충분

  실세계 비교:
    - STIFF-FLOP (EU 프로젝트): 3-segment soft arm (각 3 DOF = 9 DOF)
    - Festo Bionic Handling Assistant: 3 segments
    - 연구용 소프트 암: 보통 3-4 segments
    - 6 segments는 연구에서도 드물게 사용됨
    - BUT: SE(3) 완전 도달에 6 DOF 필요하다는 논리는 유효

  Grade: CLOSE
  SE(3) 도달성 논리는 유효하지만, 실제 소프트 로봇은 3-4 세그먼트가
  주류. 각 세그먼트의 DOF가 2-3이므로 총 DOF는 이미 6을 초과.
```

---

### H-ROB-66: Pneumatic Muscle Pressure Ratio -- 1/e Activation Threshold

> 공압 인공 근육(McKibben actuator)의 최적 활성화 임계값이
> 1/e ~ 0.368 (최대 압력 대비 비율)이다.

```
  Boltzmann 연결:
    1/e = 0.3679... = Boltzmann 분포에서 E=kT의 점유 확률
    McKibben actuator의 힘-압력 관계:
      F(P) = (pi*D^2/4) * [a*(1-epsilon)^2 - b] * P
      여기서 epsilon = 수축률

  n=6 대응:
    최대 수축 시 epsilon_max ~ 0.25-0.35 (문헌값)
    1/e ~ 0.368은 이 범위의 상한 근처
    "최적 작동점" = 최대 힘-효율 곱의 위치

  실세계 비교:
    - McKibben actuators: 최적 작동 범위는 수축률 20-30%
    - 37%는 약간 높지만 일부 설계에서 도달 가능
    - Festo DMSP-20: 최대 수축 25% (1/e보다 낮음)
    - Shadow Robot Company: 공압 근육 사용, 작동점 비공개

  Grade: WEAK
  1/e와 McKibben 최대 수축률의 수치적 근접은 있으나,
  실제 최적 작동점은 부하와 속도에 의존하며 0.25-0.30 근처가 일반적.
```

---

### H-ROB-67: Soft Gripper Fingers -- sopfr(6) = 5 with phi(6) = 2 States

> 소프트 그리퍼의 최적 구성이 sopfr(6)=5개 finger, 각 phi(6)=2 상태
> (inflated/deflated)이며, 총 2^5 = 32 파지 패턴을 생성한다.

```
  n=6 대응:
    sopfr(6) = 5 fingers
    phi(6) = 2 states per finger (이진 공압 제어)
    총 grasp 패턴 = 2^sopfr(6) = 2^5 = 32
    Feix taxonomy의 33 grasp types와 거의 일치 (32 vs 33)

  수학적 구조:
    32 = 2^5는 Boolean lattice B_5를 형성
    33 = 3 * 11, 소수 분해가 다름
    BUT: 32와 33의 근접은 주목할 만함

  실세계 비교:
    - Soft Robotics Inc. mGrip: 2-4 fingers가 일반적
    - RBO Hand 2: 5 fingers (실리콘 기반) -- 일치!
    - Yale OpenHand: 4 fingers
    - Festo MultiChoiceGripper: 3 fingers
    - 5-finger soft gripper는 존재하지만 표준은 아님

  Grade: CLOSE
  RBO Hand 2 등 5-finger soft gripper가 존재하고,
  32 grasp patterns가 Feix taxonomy(33)에 근접.
  하지만 산업용은 2-4 finger가 주류.
```

---

### H-ROB-68: Continuum Robot Curvature -- Phi_6(kappa) Stability Condition

> 연속체 로봇의 곡률 안정성 조건이 cyclotomic polynomial
> Phi_6(kappa) = kappa^2 - kappa + 1 > 0으로 표현된다.

```
  수학적 기초:
    Phi_6(x) = x^2 - x + 1 (6차 cyclotomic polynomial)
    근: x = e^{i*pi/3}, e^{-i*pi/3} (단위원 위의 primitive 6th roots)
    |근| = 1이므로, 모든 실수 kappa에 대해 Phi_6(kappa) > 0

  로보틱스 매핑:
    곡률 kappa를 정규화하여 Phi_6(kappa) > 0이 항상 성립
    --> "6차 cyclotomic 안정성": 연속체 로봇이 어떤 곡률에서도
        구조적으로 안정 (Phi_6이 실근을 갖지 않으므로)
    최소값: kappa = 1/2에서 Phi_6(1/2) = 3/4

  실세계 비교:
    - 연속체 로봇의 안정성은 Cosserat rod theory로 분석
    - 좌굴(buckling) 한계는 재료 강성과 기하에 의존
    - Phi_6(kappa) > 0은 자명한 사실 (양의 정부호 이차식)
    - 이를 "안정성 조건"으로 부르는 것은 trivially true

  Grade: WEAK
  Phi_6(x) > 0 for all real x는 수학적 사실이지만,
  이것이 연속체 로봇의 물리적 안정성과 연결되는 메커니즘이 없다.
  자명하게 참인 조건은 유용한 예측을 제공하지 않는다.
```

---

## 카테고리 Z: 수술 로봇과 정밀 조작

---

### H-ROB-69: Surgical Robot Arms -- 6-DOF + phi(6) = 2 Redundancy

> 수술 로봇의 팔 구성이 n=6 DOF + phi(6)=2 추가 자유도 = 8 DOF이며,
> 이는 da Vinci 시스템과 정확히 일치한다.

```
  n=6 대응:
    기본 DOF = 6 = dim(SE(3)) (H-ROB-6과 동일)
    여분 DOF = phi(6) = 2 (체내 충돌 회피용 redundancy)
    총 DOF = n + phi(6) = 6 + 2 = 8

  실세계 비교:
    - da Vinci Xi: 4개 암, 각 암 6+1 DOF (6 external + 1 wrist)
      BUT: EndoWrist는 3 DOF 추가 -> 총 instrument DOF = 7
    - da Vinci 전체: 외부 6 DOF + endowrist 3 DOF = 9 DOF (8이 아님)
    - Medtronic Hugo: 7 DOF per arm
    - CMR Versius: 7 DOF per arm
    - 7-DOF가 수술 로봇 업계 표준 (6+1 redundancy)

  Grade: CLOSE
  수술 로봇이 6 기본 DOF + redundancy를 사용하는 것은 맞지만,
  추가 DOF는 phi(6)=2가 아니라 1 (7-DOF 표준) 또는 3 (da Vinci wrist).
  6 기본 = SE(3) 부분은 정확.
```

---

### H-ROB-70: Surgical Trocar Ports -- tau(6) = 4

> 복강경/로봇 수술의 트로카 포트 수가 tau(6) = 4이다.

```
  n=6 대응:
    tau(6) = 4: 약수의 개수 = 포트 수
    4 ports = 1 camera + 3 instruments (Egyptian: 1 = 1/4 camera + 3/4 instruments... 아님)

    대안 해석:
    tau(6) = 4 ports: 2 working + 1 camera + 1 assistant
    --> phi(6) = 2 working ports

  실세계 비교:
    - 로봇 전립선 절제술: 보통 5-6 ports (da Vinci 4-arm + assistant)
    - 로봇 담낭 절제술: 4 ports가 표준! MATCH
    - 단일 포트 수술(SP): 1 port (da Vinci SP)
    - 자연 개구부 수술(NOTES): 0 external ports
    - 4-port는 담낭 절제술 등 일부 시술에서 표준

  Grade: CLOSE
  담낭 절제술 등에서 4-port가 표준이지만, 시술에 따라 3-6 port로 다양.
  보편적 "4 port = 최적"이라고는 할 수 없다.
```

---

### H-ROB-71: Microsurgical Force Resolution -- sigma(6)^2 = 144 mN Range

> 미세 수술 로봇의 힘 감지 해상도가 sigma(6)^2 = 144 구간에서
> 12-bit (sigma=12) ADC로 구현된다.

```
  n=6 대응:
    sigma(6) = 12 --> 12-bit ADC (H-ROB-9와 동일 근거)
    12-bit = 4096 levels
    미세 수술 힘 범위: 0 ~ 2N 전형적
    해상도 = 2N / 4096 = 0.49 mN/level

  sigma(6)^2 = 144 매핑:
    실용 범위를 144 mN (= 0.144 N)로 설정하면
    해상도 = 144 mN / 4096 = 0.035 mN (35 uN)
    이는 안과 수술 힘 범위 (~7.5 mN, Gupta et al. 2005)의
    약 19배 --> 범위가 과도하게 넓음

  실세계 비교:
    - 안과 미세 수술: 0-75 mN 범위, 1 mN 해상도 필요
    - 혈관 봉합: 0-500 mN 범위
    - 12-bit ADC 자체는 실제로 사용됨 (H-ROB-9, EXACT)
    - 144 mN 범위는 특정 시술에 적합할 수 있으나 보편적이지 않음

  Grade: WEAK
  12-bit ADC 사용은 맞지만 (기존 H-ROB-9), sigma^2 = 144를
  힘 범위에 매핑하는 것은 임의적. 실제 범위는 시술에 의존.
```

---

### H-ROB-72: Surgical Workspace -- Phi_6 Dexterity Metric

> 수술 로봇의 dexterity index가 cyclotomic polynomial Phi_6을 통해
> 정의되며, Phi_6(J) = det(J^T J) - tr(J^T J) + 1 > 0이 조작 가능 조건이다.

```
  수학적 기초:
    Yoshikawa manipulability: w = sqrt(det(J*J^T))
    Condition number: kappa = sigma_max / sigma_min
    Phi_6(x) = x^2 - x + 1 구조를 Jacobian에 적용

  제안:
    정규화된 manipulability m에 대해
    Phi_6(m) = m^2 - m + 1
    최소값 = 3/4 (m = 1/2에서)
    --> Phi_6(m) >= 3/4 항상 성립
    "manipulability가 0에 가까울 때도 Phi_6 > 0"
    --> singularity 근처에서 최소한의 조작성 보장?

  실세계 비교:
    - Yoshikawa manipulability는 singularity에서 0에 수렴 -- 이는 물리적 사실
    - Phi_6(0) = 1 > 0이지만, 이는 Phi_6의 성질일 뿐 물리적 singularity가 사라지지 않음
    - 수학 함수의 양수성이 물리적 조작 가능성을 보장하지 않음

  Grade: FAIL
  Phi_6(m) > 0의 자명한 양수성을 singularity 회피와 혼동.
  Jacobian이 rank-deficient하면 Phi_6를 적용해도 물리적 singularity는 존재.
```

---

## 카테고리 W: SE(3) Lie 군과 로봇 제어

---

### H-ROB-73: se(3) Lie Bracket Structure -- [twist_i, twist_j] Closure

> 6-DOF 로봇 제어가 se(3) Lie 대수의 bracket 연산으로 완전히 기술되며,
> 6개 기저 twist의 bracket table이 n=6 약수 구조를 반영한다.

```
  수학적 기초:
    se(3) = Lie algebra of SE(3), dim = 6
    기저: {e1, e2, e3} (rotation) + {e4, e5, e6} (translation)
    Bracket 구조:
      [e_i, e_j] = epsilon_{ijk} e_k  (i,j,k in {1,2,3}, rotation)
      [e_i, e_{j+3}] = epsilon_{ijk} e_{k+3}  (rotation-translation)
      [e_{i+3}, e_{j+3}] = 0  (translation commutes)

  n=6 대응:
    se(3)의 dim = 6 = n --> EXACT (이미 H-ROB-6에서 확인)
    Rotation subalgebra so(3): dim = 3 = n/2
    Translation ideal R^3: dim = 3 = n/2
    구조 상수의 비영 성분 수: 3 + 9 = 12 = sigma(6)

  실세계 비교:
    - Screw theory 기반 로봇 제어 (Murray, Li, Sastry 1994)는
      표준 교과서 접근법
    - Product of Exponentials (PoE) formula는 se(3)를 직접 활용
    - 구조 상수가 12개인 것은 so(3)의 Levi-Civita tensor에서 유래
      3! = 6 비영 epsilon 성분 x 2 (rotation-rotation + rotation-translation) = 12

  Grade: EXACT
  se(3)의 dim = 6, 비영 구조 상수 = 12 = sigma(6).
  이 일치는 Lie 대수의 수학적 구조에서 직접 유도됨.
  로봇 제어에서 se(3)가 표준 프레임워크인 것도 사실.
```

---

### H-ROB-74: Exponential Map se(3) -> SE(3) -- R(6) = 1 Surjectivity

> se(3)에서 SE(3)로의 exponential map이 surjective (전사)이며,
> 이는 R(6) = sigma*phi/(n*tau) = 1과 대응한다.

```
  수학적 기초:
    exp: se(3) -> SE(3)는 surjective (모든 rigid body motion이 도달 가능)
    이는 SE(3)가 connected Lie group이기 때문
    BUT: exp는 일대일이 아님 (회전 2*pi = identity)

  R(6) = 1 대응:
    R(6) = sigma(6)*phi(6)/(n*tau(6)) = 24/24 = 1
    "가역성 비율 = 1" <--> "exp map이 surjective"
    Lie 대수의 모든 원소가 Lie 군의 원소로 매핑 = "빠짐없음" = R=1

  실세계 비교:
    - exp의 surjectivity는 Lie 이론의 표준 정리
    - 이 사실은 로봇 경로 계획에서 중요: 임의의 pose에 도달하는
      twist trajectory가 항상 존재
    - R(6) = 1과의 연결은 해석적 비유 (rigorous하지 않음)

  Grade: CLOSE
  exp: se(3) -> SE(3) surjectivity는 수학적 사실이고 로보틱스에서 활용됨.
  R(6)=1과의 대응은 비유적이지만, "완전수 = 빠짐없는 구조"라는
  해석은 일관성이 있다.
```

---

### H-ROB-75: Adjoint Representation -- dim(Ad(SE(3))) = 6x6 = 36

> SE(3)의 adjoint representation이 6x6 행렬이며,
> 6^2 = 36 = J_2(6) * (sigma(6)/tau(6)^2) 와 관련된다.

```
  수학적 기초:
    Ad: SE(3) -> GL(6, R)
    Adjoint matrix = 6x6 (se(3)의 기저에 대한 선형 변환)
    원소 수 = 36 = 6^2

  n=6 산술:
    36 = 6^2 = n^2
    36 = J_2(6) * (3/2) -- 깨끗하지 않음
    36 = sigma(6) * tau(6) - 12 -- 역시 깨끗하지 않음
    가장 자연스러운 관계: 36 = n^2

  실세계 비교:
    - Adjoint representation은 spatial vector algebra에서 핵심
      (Featherstone, Robot Dynamics Algorithms)
    - 6x6 spatial inertia, spatial force 변환에 직접 사용
    - 모든 multibody dynamics 시뮬레이터가 6x6 행렬 활용

  Grade: EXACT
  Ad(SE(3))가 6x6 행렬인 것은 수학적 사실이며,
  spatial vector algebra로서 모든 로봇 동역학 시뮬레이터에서 표준.
  n^2 = 36은 n=6에서의 필연적 결과.
```

---

### H-ROB-76: Spatial Inertia Matrix -- 6x6 with tau(6) = 4 Independent Blocks

> 공간 관성 행렬(spatial inertia)이 6x6이며 tau(6)=4개의 독립 블록으로
> 구성된다.

```
  수학적 기초:
    Spatial inertia M ∈ R^{6x6}:
      M = [[I, m*c_x],
           [m*c_x^T, m*I_3]]
    여기서 I = 3x3 회전 관성, m = 질량, c = 질량 중심 위치, c_x = skew-symmetric

  블록 구조:
    4개 3x3 블록:
      (1,1): I (회전 관성 텐서)
      (1,2): m*c_x (회전-병진 커플링)
      (2,1): m*c_x^T (병진-회전 커플링, 대칭)
      (2,2): m*I_3 (병진 관성)
    tau(6) = 4 블록!

  독립 파라미터:
    I: 6개 독립 성분 (대칭 3x3)
    m: 1개
    c: 3개
    총 = 10개 = sopfr(6) + sopfr(6) = 5+5? 아님. 10 = tau(6) + n = 4+6.

  실세계 비교:
    - 모든 multibody dynamics (Featherstone, RBDA) 교과서에서
      6x6 spatial inertia를 4개 3x3 블록으로 기술
    - Pinocchio, Drake, MuJoCo 등 모든 시뮬레이터가 이 구조 사용
    - 4-block 구조는 물리적으로 필연 (회전/병진의 2x2 배열)

  Grade: EXACT
  6x6 spatial inertia의 4-block 구조는 물리적, 수학적 사실이며
  모든 로봇 동역학 소프트웨어의 표준. tau(6)=4 일치.
```

---

## 카테고리 V: 드론 편대와 구 충전

---

### H-ROB-77: 3D Drone Stacking -- FCC/HCP Layer Spacing and sigma(6) = 12 Neighbors

> 3D 드론 편대의 최적 적층이 FCC/HCP 구조이며,
> 각 드론이 sigma(6) = 12개의 최근접 이웃을 갖는다.

```
  수학적 기초:
    FCC (Face-Centered Cubic) packing:
      각 구가 12개 최근접 이웃 (kissing number in 3D = 12)
    HCP (Hexagonal Close-Packed):
      역시 12 최근접 이웃
    Kepler 추측 (Hales 2005 증명): 최적 3D 구 충전 밀도 = pi/(3*sqrt(2))

  n=6 대응:
    3D kissing number = 12 = sigma(6) <-- 정확한 수학적 사실
    Newton-Gregory 문제 (1694)의 해: 12
    이것은 sigma(6)과 정확히 일치

  드론 편대 매핑:
    각 드론이 안전 구(sphere)를 가질 때,
    FCC/HCP 배치로 3D 공간에서 최대 밀도 편대 형성
    각 드론의 최근접 이웃 = 12

  실세계 비교:
    - 3D 드론 편대는 일반적으로 임무 기반 배치 (밀도 최적화 아님)
    - 하지만 dense formation flying 연구에서 FCC-like 배치가 등장
    - 12 = 3D kissing number는 수학적으로 증명된 사실

  Grade: EXACT
  3D 구 충전의 kissing number = 12 = sigma(6)은 Newton이래 알려진
  수학적 사실이며, 이를 dense 드론 편대에 적용하는 것은 자연스럽다.
  다만, 실제 드론 편대가 FCC 배치를 사용하는지는 별도 검증 필요.
```

---

### H-ROB-78: Quadrotor Propeller Configuration -- tau(6) = 4 Rotors

> 쿼드로터의 4개 프로펠러가 tau(6) = 4에서 도출되며,
> SE(3) 제어에 필요한 최소 actuator 수이다.

```
  n=6 대응:
    tau(6) = 4 rotors
    SE(3) underactuation: 쿼드로터는 6 DOF 중 4개만 직접 제어
      (x, y, z, yaw) = 4 controlled DOF = tau(6)
      (roll, pitch) = 2 DOF는 indirect (coupled) = phi(6)

  수학적 기초:
    쿼드로터 thrust: 4 rotors -> 4개 독립 입력
      [F_total, tau_roll, tau_pitch, tau_yaw]
    이는 SE(3)의 6 DOF를 4개 입력으로 underactuated 제어
    최소 hovering 조건: 4 rotors (3 rotors = tricopter는 불안정)

  실세계 비교:
    - 쿼드로터는 가장 인기 있는 멀티로터 구성 (DJI, Skydio 등)
    - BUT: 헥사콥터(6 rotors = n)와 옥토콥터(8 rotors)도 널리 사용
    - 트라이콥터(3 rotors): 존재하지만 드묾
    - 쿼드가 인기인 이유: 가격/복잡도 대비 충분한 제어 (SE(3) 무관)
    - 4 제어 입력 = 4 = tau(6)는 정확한 일치

  Grade: CLOSE
  쿼드로터의 4 rotors = tau(6)이고, 4개 직접 제어 DOF = tau(6),
  2개 간접 DOF = phi(6)는 깨끗한 대응. 하지만 4가 "최적"이라기보다
  "최소 충분" (헥사/옥토가 안전성에서 우월).
```

---

### H-ROB-79: Hexacopter Fault Tolerance -- n = 6 Rotors, sopfr(6) = 5 Minimum

> 헥사콥터가 n=6 로터를 가지며, sopfr(6)=5개 로터로도
> 안전 착륙이 가능하다 (1-rotor fault tolerance).

```
  n=6 대응:
    n = 6 rotors (hexacopter)
    1개 rotor 고장 시: sopfr(6) = 5 rotors로 운용
    --> SE(3) 중 5 DOF 제어 가능 (yaw 제어 상실, 나머지 유지)

  수학적 기초:
    6 rotors -> 6개 독립 thrust input
    --> 완전 SE(3) 제어 (roll, pitch, yaw + x, y, z)
    5 rotors -> 5개 input: 1 DOF 상실 (일반적으로 yaw)
    --> 안전 착륙에 충분 (position + attitude 유지)

  실세계 비교:
    - DJI Matrice 600: 6 rotors, 1-rotor fault tolerance 공식 지원
    - 학술 연구 (Mueller & D'Andrea, 2014): hexacopter에서
      1개 rotor 완전 상실 시 안전 비행 시연
    - 쿼드로터: 1 rotor 상실 = 불안정 (controlled spinning descent만 가능)
    - 옥토콥터: 2 rotor 상실까지 가능

  Grade: EXACT
  헥사콥터의 1-rotor fault tolerance는 실제로 시연되고 상용화된 사실.
  n=6 rotors에서 sopfr=5로의 graceful degradation은 정확한 매핑이며,
  쿼드로터(tau=4)는 이 내결함성을 제공하지 못한다.
```

---

### H-ROB-80: Formation Topology Change -- Divisor Lattice {1,2,3,6} as Formation Modes

> 드론 편대의 구조 전환이 6의 약수 격자 {1,2,3,6}을 따르며,
> 4가지 formation mode 사이의 전환이 자연스럽게 정의된다.

```
  약수 격자 구조:
    {1, 2, 3, 6}의 divisibility partial order:
    1 | 2, 1 | 3, 1 | 6, 2 | 6, 3 | 6
    Hasse diagram: 1 -> {2, 3} -> 6

  Formation mode 매핑:
    Mode 1 (약수 1): 단일 군집 (tight formation)
    Mode 2 (약수 2): 2개 분대 (left/right split)
    Mode 3 (약수 3): 3개 분대 (triangle formation)
    Mode 6 (약수 6): 완전 분산 (individual operation)

  전환 규칙:
    약수 관계가 있는 mode 사이만 직접 전환 가능
    1 <-> 2, 1 <-> 3, 2 <-> 6, 3 <-> 6, 1 <-> 6
    2 <-> 3은 직접 전환 불가 (약수 관계 없음)
    --> 2에서 3으로: 반드시 1 또는 6을 경유

  실세계 비교:
    - 군용 편대: platoon(1) -> fire teams(2 or 3) -> individual(all)
      이 전환 패턴은 실제 전술에서 사용됨
    - DARPA OFFSET: formation splitting/merging 연구
    - 하지만 실제 전환은 약수 격자와 무관하게 연속적
    - 2->3 직접 전환 불가 제약은 비현실적

  Grade: CLOSE
  약수 격자를 formation mode hierarchy로 사용하는 것은 창의적이며,
  군사 전술의 분대 분할과 일부 대응. 하지만 2<->3 전환 불가 제약은
  실용적이지 않으며, 실제 편대 전환은 이산적이지 않다.
```

---

## Summary Scorecard

| ID | Hypothesis | Grade | Notes |
|----|-----------|-------|-------|
| H-ROB-61 | Leech 24-agent swarm | WEAK | 24D lattice와 3D swarm 간 차원 간극 |
| H-ROB-62 | Hex drone formation (6-coord) | CLOSE | 2D circle packing 최적은 증명된 사실 |
| H-ROB-63 | Consensus lambda(6)=2 cycle | WEAK | 일반 이론의 n=6 특수값일 뿐 |
| H-ROB-64 | Squarefree fault tolerance | WEAK | mu(6)=1과 내결함성 연결은 비유적 |
| H-ROB-65 | Soft robot 6 segments | CLOSE | SE(3) 논리 유효, 실제는 3-4 세그먼트 |
| H-ROB-66 | McKibben 1/e threshold | WEAK | 수치적 근접이나 실제 작동점은 0.25-0.30 |
| H-ROB-67 | Soft gripper 5 fingers | CLOSE | RBO Hand 2 일치, 32 vs 33 grasp 근접 |
| H-ROB-68 | Phi_6 curvature stability | WEAK | 자명하게 참인 조건 (유용하지 않음) |
| H-ROB-69 | Surgical robot 6+2 DOF | CLOSE | 6-DOF 기본은 맞지만 추가 DOF 수 불일치 |
| H-ROB-70 | Trocar 4 ports | CLOSE | 담낭절제 4-port 표준, 시술별 변동 |
| H-ROB-71 | Microsurgical 144 mN | WEAK | 12-bit ADC는 맞지만 144 범위는 임의적 |
| H-ROB-72 | Phi_6 dexterity | FAIL | Phi_6 양수성과 singularity 회피 혼동 |
| H-ROB-73 | se(3) structure constants=12 | EXACT | Lie 대수 비영 구조 상수 = sigma(6) |
| H-ROB-74 | exp surjectivity = R(6)=1 | CLOSE | 수학적 사실, R(6) 연결은 비유적 |
| H-ROB-75 | Adjoint 6x6 matrix | EXACT | spatial vector algebra 표준 |
| H-ROB-76 | Spatial inertia 4 blocks | EXACT | Featherstone 교과서 표준 |
| H-ROB-77 | 3D kissing number = 12 | EXACT | Newton-Gregory 문제의 해 = sigma(6) |
| H-ROB-78 | Quadrotor 4 rotors | CLOSE | tau(6) = 4 controlled DOF 대응 |
| H-ROB-79 | Hexacopter fault tolerance | EXACT | DJI Matrice 600 등 실증됨 |
| H-ROB-80 | Divisor lattice formations | CLOSE | 군사 전술과 일부 대응, 2<->3 제약 비현실적 |

### Aggregate Statistics

| Grade | Count | Percentage |
|-------|-------|-----------|
| EXACT | 5 | 25% |
| CLOSE | 8 | 40% |
| WEAK | 6 | 30% |
| FAIL | 1 | 5% |
| UNVERIFIABLE | 0 | 0% |

### Key Findings

**Strongest hypotheses (EXACT):**
- H-ROB-73: se(3) Lie 대수의 비영 구조 상수 12 = sigma(6). 교과서적 수학적 사실.
- H-ROB-75: Adjoint representation 6x6. 모든 multibody dynamics의 표준.
- H-ROB-76: Spatial inertia의 4-block 구조 = tau(6). Featherstone 표준.
- H-ROB-77: 3D kissing number 12 = sigma(6). 수학적 증명.
- H-ROB-79: 헥사콥터 1-rotor fault tolerance. 상용 드론에서 실증.

**Cross-domain bridges:**
- SE(3) Lie group <-> n=6 기본 DOF (H-ROB-73, 74, 75, 76): 가장 강한 다리.
  se(3)의 dim=6, 구조 상수 12, Adjoint 6x6, spatial inertia 4-block이
  모두 n=6 산술과 정확히 대응.
- Sphere packing <-> drone formation (H-ROB-77): kissing number 12의 물리적 실현.
- Hexacopter fault tolerance (H-ROB-79): n=6 -> sopfr=5 graceful degradation.

**Notable improvement:**
기존 H-ROB-1~28의 EXACT 비율은 14% (4/28)였으나,
극단 가설 H-ROB-61~80은 25% (5/20). 이는 Lie 군 구조와 구 충전 이론이라는
수학적으로 엄밀한 영역에 집중한 결과이다.
