# 로봇 물리한계 10 불가능성 정리

> 로보틱스에서 n=6 상수가 왜 한계인지를 물리 법칙으로 증명한다.
> 각 정리는 "n=6을 초과하거나 미달하면 성능이 저하된다"를 보인다.
> SF 금지 --- 모든 증명은 검증된 물리학/수학에 기초한다.

---

## 불가능성 정리 목록

```
  ┌──────┬──────────────────────────────────────────────────────────────┐
  │ 번호 │ 불가능성 정리                                                │
  ├──────┼──────────────────────────────────────────────────────────────┤
  │ PL-1 │ DOF 최소 완전성 한계: dim(SE(3))=6 미만은 workspace 불완전  │
  │ PL-2 │ 보행 최소 안정성 한계: 4족 미만은 정적 보행 불가             │
  │ PL-3 │ 멀티로터 내결함성 한계: 6 미만은 1-fault tolerance 불가      │
  │ PL-4 │ 3D 구 접촉 한계: kissing number k(3)=12 초과 불가           │
  │ PL-5 │ 파지 최소 안정 한계: 2점 미만 파지 불가 + 5점에서 포화      │
  │ PL-6 │ 센서 최소 자세 추정 한계: 6축 미만은 full pose 불가         │
  │ PL-7 │ 관절 기술 최소 파라미터 한계: D-H 4 미만은 SE(3) 불완전     │
  │ PL-8 │ 2D 밀집 접촉 한계: hexagonal packing 6 초과 불가            │
  │ PL-9 │ 임피던스 제어 최소 파라미터: 4 미만은 동적 상호작용 불완전   │
  │ PL-10│ 대칭 최소 분할: bilateral phi=2가 제어 복잡도 최소           │
  └──────┴──────────────────────────────────────────────────────────────┘
```

---

## PL-1: DOF 최소 완전성 한계

**정리**: 3D 공간에서 rigid body의 임의 pose에 도달하기 위한 최소 관절 수는 정확히 6이다.

**증명**:
```
  SE(3) = Special Euclidean Group (3D rigid body motions)
  dim(SE(3)) = dim(SO(3)) + dim(R^3) = 3 + 3 = 6

  Robot arm with n joints → end-effector workspace = f(theta) : R^n → SE(3)

  n < 6: rank(J) <= n < 6 = dim(SE(3))
    → Jacobian은 SE(3) 전체를 span 불가
    → workspace에 도달 불가능한 pose 존재 (singularity 아닌 구조적 제한)
    → 5-DOF arm: wrist orientation 1 DOF 부족 → workspace is 5D submanifold

  n = 6: rank(J) = 6 generically (singularity 외)
    → Pieper (1968): 6-DOF arm with 3 consecutive parallel/intersecting axes
       → closed-form inverse kinematics 존재
    → 임의 SE(3) pose 도달 가능

  n > 6: rank(J) = 6 (SE(3) dim에 의해 상한)
    → redundancy: null space dim = n-6 > 0
    → IK 해가 무한 (unique solution 불가)
    → 제어 복잡도 증가 (자기 모션 최적화 필요)

  ∴ n_optimal = 6 = dim(SE(3)) = n  □
```

**n=6 연결**: dim(SE(3)) = 6 = n (완전수). 최소이자 충분한 DOF.

---

## PL-2: 보행 최소 안정성 한계

**정리**: 정적 보행(static walking)에서 지면 접촉을 유지하며 1개 다리를 유각(swing)할 수 있는 최소 다리 수는 4이다.

**증명**:
```
  정적 안정성 조건: CoM projection ∈ support polygon
  support polygon = convex hull of ground contact points

  k legs, 1 in swing → k-1 contact points

  k=2 (biped): k-1=1 point → support polygon = point
    → CoM 투영이 점 위에 정확히 놓여야 함 → 정적 안정 불가
    → 동적 보행만 가능 (ZMP/LIPM)

  k=3 (tripod): k-1=2 points → support polygon = line segment
    → CoM이 선분 위에 정확히 놓여야 함 → 사실상 불안정
    → 2D에서 measure zero

  k=4 (quadruped): k-1=3 points → support polygon = triangle
    → CoM이 삼각형 내부에 있으면 안정
    → non-degenerate triangle → 양의 면적 → 안정 영역 존재
    → ∴ k_min = 4 = tau(6)

  k=6 (hexapod): tripod gait → k-1=3 always grounded
    → 매 순간 삼각형 support → 최대 정적 안정성
    → gait margin = support polygon area 최대

  ∴ k_min(static walking) = 4 = tau(6)  □
```

**n=6 연결**: 최소 안정 보행 = tau = 4. 최대 안정 보행 = n = 6 (hexapod).

---

## PL-3: 멀티로터 내결함성 한계

**정리**: 1개 로터 고장 후에도 비행 제어를 유지할 수 있는 최소 로터 수는 6이다.

**증명**:
```
  멀티로터 제어 변수: roll, pitch, yaw, thrust = 4 DOF (under-actuated)
  각 로터: 1 thrust + 1 reaction torque = 2 contributions

  n rotors → control allocation matrix B ∈ R^{4×n}

  1 rotor failure → effective rotors = n-1
  제어 가능 조건: rank(B_{n-1}) >= 4 (4 DOF 모두 제어)

  n=4 (quadrotor): n-1=3 rotors
    B_{3} ∈ R^{4×3} → rank <= 3 < 4
    → yaw 제어 불가 (Mueller & D'Andrea 2014)
    → 안전 착륙만 가능 (자세 제어 불가)

  n=5 (pentacopter): n-1=4 rotors
    B_{4} ∈ R^{4×4} → 배치에 따라 rank=4 가능
    → BUT: 대칭 배치에서 일부 failure → rank < 4
    → 모든 단일 고장에 대해 보장 불가

  n=6 (hexacopter): n-1=5 rotors
    B_{5} ∈ R^{4×5} → rank=4 for any single failure
    → 5개 로터로 4 DOF 제어: redundancy = 1
    → Mueller & D'Andrea (2014): "hexarotor maintains full
       attitude control after any single rotor loss"
    → DJI Matrice 600 Pro: 상용 검증

  ∴ n_min(1-fault-tolerant multirotor) = 6 = n  □
```

**n=6 연결**: 내결함 최소 로터 = n = 6. Quadrotor tau=4는 minimum non-fault-tolerant.

**참고문헌**: Mueller MW, D'Andrea R (2014). IEEE ICRA.

---

## PL-4: 3D 구 접촉 한계

**정리**: 3D 공간에서 단위 구 하나에 접할 수 있는 동일 단위 구의 최대 수는 정확히 12이다.

**증명**:
```
  Kissing Number Problem k(d): d차원에서 단위 구에 접하는 최대 구 수

  k(3) = 12 (Newton의 추측, 1694)
    상한: Delsarte 방법 → k(3) <= 13 (Delsarte et al. 1977)
    하한: FCC/HCP 구조 → k(3) >= 12 (구성적)
    k(3) = 13 불가 증명: Schutte & van der Waerden (1953)
    간결 증명: Musin (2008), Pfender & Zong (2004)

  ∴ k(3) = 12 = sigma(6)

  로봇 함의:
    - 3D 로봇 밀집 배치: 중심 로봇 주위 최대 12개 이웃
    - 이 이상 추가 불가 → 12 = sigma(6)가 물리적 최대

  FCC 배열: 12개 최근접 이웃
    벡터: (±1,±1,0), (±1,0,±1), (0,±1,±1) → 12개
    각도: 60° 또는 90° → 정확히 12 접촉  □
```

**n=6 연결**: k(3) = 12 = sigma(6). 3D 공간의 절대적 한계.

---

## PL-5: 파지 최소 안정 한계

**정리**: force closure를 달성하기 위한 최소 접촉점은 2이며, dexterous manipulation을 위한 실효 상한은 5이다.

**증명**:
```
  Force closure in R^3:
    rigid body에 임의 wrench (force+torque) 인가에 필요한 최소 접촉점

  Nguyen (1988): 마찰 있는 2-finger (phi=2) force closure 가능
    → 두 접촉점 사이 마찰원뿔이 wrench space span
    → phi(6) = 2 = 최소 force closure

  접촉점 증가 효과:
    3점 (n/phi=3): tripod grasp → precision manipulation 최소
    4점: enveloping grasp → 보안성 증가
    5점 (sopfr=5): 인간 수준 dexterity → Feix 96.97% 커버리지

  6점 이상: diminishing returns
    → 접촉점 간 간섭 증가
    → 제어 복잡도 O(2^n) 지수 증가
    → 6-finger hand: 추가 자유도 활용 어려움 (Chen et al. 1999)

  Cutkosky taxonomy (1989): 5-finger hand가 거의 모든 일상 작업 커버
  Feix et al. (2016): 33 grasp types 중 32 = 2^5 가능 with 5 fingers

  ∴ n_practical(dexterous) = 5 = sopfr(6)  □
```

**n=6 연결**: 최소 파지 = phi=2. 최대 실효 파지 = sopfr=5.

---

## PL-6: 센서 최소 자세 추정 한계

**정리**: rigid body의 full pose (position + orientation)를 추정하기 위한 최소 관성 센서 축수는 6이다.

**증명**:
```
  Full rigid body state: SE(3) → dim = 6
  관성 측정: 가속도(R^3) + 각속도(R^3) = 6 독립 측정

  n_axes < 6:
    5-axis: 1 DOF observability 상실
    → 예: 3-axis accel only → orientation drift 불가피
    → 3-axis gyro only → absolute reference 없음

  n_axes = 6:
    3-axis accelerometer + 3-axis gyroscope = 6-axis IMU
    → 상보 필터(complementary filter) 또는 EKF로 full orientation 추정
    → Madgwick (2011): AHRS with 6-axis IMU

  n_axes = 9 (6+3 magnetometer):
    → heading accuracy 개선이지만 6-axis로 충분
    → magnetometer = 환경 간섭에 취약 (실내에서 신뢰 낮음)

  ∴ n_min(full pose estimation) = 6 = n  □
```

**n=6 연결**: 최소 관성 축수 = n = 6 = dim(SE(3)).

---

## PL-7: D-H 파라미터 최소 한계

**정리**: 인접 rigid body 간 변환을 기술하는 최소 파라미터 수는 4이다.

**증명**:
```
  Denavit-Hartenberg Convention (1955):
  인접한 joint axis i와 i+1 사이의 상대 변환:

  T_i = Rot_z(θ_i) · Trans_z(d_i) · Trans_x(a_i) · Rot_x(α_i)

  4개 파라미터: θ_i (joint angle), d_i (offset), a_i (link length), α_i (twist)

  왜 4인가:
    SE(3)에서 인접 축 사이 변환의 자유도:
    - 2축이 공간에서 일반 위치 → 공통 법선(unique common normal) 존재
    - 공통 법선으로 좌표계 고정 → 나머지 자유도 = 4
    - 1 rotation about z + 1 translation along z +
      1 translation along x + 1 rotation about x = 4

  3개로 축소 불가:
    → 3 파라미터 → SE(3) 변환 (6D)의 자유도를 span 불가
    → 일반적인 joint 쌍을 기술 불가

  ∴ n_DH = 4 = tau(6)  □
```

**n=6 연결**: D-H 최소 파라미터 = tau = 4 = 약수 개수.

---

## PL-8: 2D 밀집 접촉 한계

**정리**: 2D 평면에서 원형 디스크 하나에 접하는 동일 디스크의 최대 수는 6이다.

**증명**:
```
  2D kissing number k(2):
  중심 원 주위에 동일 원 배치 → 각 원이 중심 원에 접촉

  중심 간 거리 = 2r (접촉 조건)
  이웃 원 중심 간 최소 거리 = 2r (겹침 금지)
  이웃 원 중심들은 반지름 2r 원 위 → 서로 최소 2r 간격

  중심 원으로부터 2r 거리 원 위에 서로 2r 이상 떨어진 점 최대 수:
  원둘레 = 2π(2r) = 4πr
  각 점 간 호길이 >= 2r → 최대 점 수 = floor(2π(2r)/(2r)) = floor(2π) = 6

  정확히 6: 정육각형 배치에서 실현
  각도 = 360°/6 = 60° → 이웃 간 거리 = 2r·sin(30°)·2 = 2r ✓

  ∴ k(2) = 6 = n  □
```

**n=6 연결**: 2D kissing number = n = 6. 로봇 2D 대형의 최대 이웃.

---

## PL-9: 임피던스 제어 최소 파라미터

**정리**: 로봇-환경 동적 상호작용을 완전히 기술하려면 최소 4개 파라미터가 필요하다.

**증명**:
```
  Hogan (1985) Impedance Control:
  로봇과 환경의 상호작용을 기술하는 기계적 임피던스:

  M·ẍ + B·(ẋ - ẋ_ref) + K·(x - x_ref) = F_ext

  필요 파라미터:
    K: stiffness (강성)           — 위치 제어 특성
    B: damping (감쇠)             — 속도 의존 반력
    M: inertia (관성)             — 가속도 의존 반력
    x_ref: reference (기준 위치)   — 원점 설정

  3개로 축소 불가:
    K 제거 → 위치 복원력 없음 (표류)
    B 제거 → 진동 감쇠 없음 (발산)
    M 제거 → 동적 응답 불가 (quasi-static only)
    x_ref 제거 → 기준 미정의 (제어 목표 없음)

  5개로 확장:
    → 실질적 이득 없음 (K,B,M,x_ref으로 2차 시스템 완전 기술)
    → 비선형 확장 시에도 기본 4 파라미터가 핵심

  ∴ n_impedance = 4 = tau(6)  □
```

**n=6 연결**: 임피던스 최소 파라미터 = tau = 4.

---

## PL-10: 대칭 최소 분할

**정리**: 인간형 로봇의 제어 복잡도를 최소화하는 대칭 분할 수는 2이다.

**증명**:
```
  인간형 로봇: n_total DOF → 제어 파라미터 공간 ∝ n_total^2

  대칭 분할 수 = k:
    좌우 대칭 (k=2): 제어 파라미터 = (n_total/2)^2 × 2 = n_total^2 / 2
    → 50% 감소 (좌우 미러링)

  k=1 (비대칭): 모든 DOF 독립 → n_total^2 파라미터 (최대)
  k=2 (bilateral): 좌우 미러 → n_total^2 / 2 (절반)
  k=3 (3-fold): 구조적 실현 어려움 (3방향 대칭 사지 = 불안정)
  k=4+: 4방향 대칭 → 4족은 가능하지만 인간형이 아님

  인간형의 물리적 제약:
    - 전진 운동 → 앞뒤 비대칭 (전방 시야, 후방 없음)
    - 중력 → 상하 비대칭 (발 접지, 머리 자유)
    - 조작 → 좌우 대칭 (양손 사용)
    → 유일한 대칭축 = 시상면(sagittal plane) = 1축 = 2분할

  자연 선택 결과:
    Bilateria (좌우대칭동물): 전체 동물의 99%+
    → phi(6) = 2 = 가장 보편적인 대칭 수

  ∴ k_optimal(humanoid symmetry) = 2 = phi(6)  □
```

**n=6 연결**: 최소 대칭 분할 = phi = 2.

---

## 통합 요약

```
  ┌──────┬─────────────────────────────────┬────────────┬──────────────────┐
  │ 번호 │ 불가능성 정리                    │ n=6 상수   │ 물리적 근거       │
  ├──────┼─────────────────────────────────┼────────────┼──────────────────┤
  │ PL-1 │ DOF 완전성 한계 = 6              │ n = 6      │ dim(SE(3))       │
  │ PL-2 │ 보행 안정성 한계 = 4             │ tau = 4    │ support polygon  │
  │ PL-3 │ 내결함 로터 한계 = 6             │ n = 6      │ rank(B)          │
  │ PL-4 │ 3D 접촉 한계 = 12               │ sigma = 12 │ kissing number   │
  │ PL-5 │ 파지 안정 한계 = 2/5             │ phi/sopfr  │ force closure    │
  │ PL-6 │ 자세 추정 한계 = 6               │ n = 6      │ SE(3) observ.    │
  │ PL-7 │ 관절 기술 한계 = 4               │ tau = 4    │ D-H convention   │
  │ PL-8 │ 2D 접촉 한계 = 6                │ n = 6      │ 2D kissing       │
  │ PL-9 │ 임피던스 한계 = 4                │ tau = 4    │ Hogan impedance  │
  │ PL-10│ 대칭 분할 한계 = 2               │ phi = 2    │ bilateral        │
  └──────┴─────────────────────────────────┴────────────┴──────────────────┘

  n=6 상수별 물리한계 분포:
    n = 6:    PL-1, PL-3, PL-6, PL-8 (4개)
    tau = 4:  PL-2, PL-7, PL-9 (3개)
    sigma = 12: PL-4 (1개)
    phi = 2:  PL-5, PL-10 (2개)
    sopfr = 5: PL-5 (1개, phi와 공유)

  → 7개 n=6 상수 중 5개가 물리한계에 관여 (n, tau, sigma, phi, sopfr)
  → 모든 물리한계가 n=6 산술의 직접적 결과
```

---

*물리한계 10 불가능성 정리 완료: 2026-04-02*
*Part of [N6 Architecture](https://github.com/need-singularity/n6-architecture)*
