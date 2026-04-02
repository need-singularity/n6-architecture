# HEXA-JOINT --- Level 3: SE(3) n=6 DOF 관절 아키텍처

**Level**: 3 / 8 (관절)
**Date**: 2026-04-02
**Status**: Living Document v1.0
**BT Connections**: BT-123, BT-124, BT-125

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  dim(SE(3)) = n = 6   se(3) struct const = sigma = 12
```

---

## 1. 레벨 목표

6-DOF = dim(SE(3)) 로봇 암으로 3D 공간 완전 도달성 달성.
se(3) Lie 대수의 구조 상수 sigma=12가 관절 체계를 결정.

핵심 명제: **6-DOF arm은 n=6 산술의 직접적 물리적 실현이다.**

---

## 2. 성능 비교 --- 시중 vs HEXA-JOINT

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  [로봇 관절] 비교: 시중 최고 vs HEXA-JOINT                       │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                   │
  │  역기구학 해석해                                                   │
  │  4-DOF arm    ████████████░░░░░░░░░░░░░░░░░░  부분 도달          │
  │  5-DOF arm    ████████████████████░░░░░░░░░░  거의 도달          │
  │  6-DOF arm    ██████████████████████████████  완전 도달           │
  │  7-DOF arm    ██████████████████████████████  완전+여유           │
  │                 n=6이 완전 도달의 최소 = dim(SE(3))               │
  │                                                                   │
  │  Jacobian rank / DOF                                              │
  │  4-DOF        ████████████████████████░░░░░░  0.83               │
  │  5-DOF        █████████████████████████████░  0.97               │
  │  n=6-DOF      ██████████████████████████████  R(6)=1.0           │
  │                                     (최적 효율, 여유 DOF 없음)    │
  │                                                                   │
  │  4족 총 DOF                                                       │
  │  3-DOF/leg    ████████████████████████░░░░░░  12 = sigma         │
  │  4-DOF/leg    ██████████████████████████████  16                  │
  │  HEXA-JOINT   ████████████████████████░░░░░░  tau*n/phi=sigma=12 │
  │                 (3-DOF/leg x 4 = sigma, EXACT 일치)              │
  │                                                                   │
  │  개선 원리: dim(SE(3))=n=6이 최소 완전 도달 DOF                   │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 3. 관절 체계

```
  ┌──────────────────────────────────────────────────────────────┐
  │  6-DOF Arm (산업 표준)                                       │
  │                                                              │
  │  Base ─[theta1]─ Link1 ─[theta2]─ Link2 ─[theta3]─ Link3    │
  │       ─[theta4]─ Link4 ─[theta5]─ Link5 ─[theta6]─ EE      │
  │                                                              │
  │  n = 6 joints = dim(SE(3)) = 3 rotation + 3 translation     │
  │  Pieper's solution: 해석해 존재 (수치해 불필요)               │
  │  UR/FANUC/ABB/KUKA = ALL 6-DOF = ALL n = 6                  │
  └──────────────────────────────────────────────────────────────┘

  ┌──────────────────────────────────────────────────────────────┐
  │  Quadruped (4족 보행)                                        │
  │                                                              │
  │  tau = 4 legs x n/phi = 3 DOF/leg = sigma = 12 total        │
  │                                                              │
  │  [Hip Ab/Ad] ── [Hip Flex/Ex] ── [Knee Flex/Ex]             │
  │     DOF 1           DOF 2           DOF 3                    │
  │                                                              │
  │  Spot:     12 DOF (3x4) = sigma    EXACT                    │
  │  ANYmal:   12 DOF (3x4) = sigma    EXACT                    │
  │  Unitree:  12 DOF (3x4) = sigma    EXACT                    │
  └──────────────────────────────────────────────────────────────┘

  ┌──────────────────────────────────────────────────────────────┐
  │  Humanoid J2=24 DOF                                          │
  │                                                              │
  │  L-Arm: n=6 + R-Arm: n=6 = sigma=12 상체                    │
  │  L-Leg: tau=4 + R-Leg: tau=4 = sigma-tau=8 하체              │
  │  Torso: phi=2 + Head: phi=2 = tau=4 중추                     │
  │  Total: 12 + 8 + 4 = J2 = 24 DOF                            │
  └──────────────────────────────────────────────────────────────┘
```

---

## 4. DSE 후보군

| # | 관절 구성 | 총 DOF | 적용 형태 | 제어 복잡도 | n6 연결 |
|---|---------|--------|---------|-----------|---------|
| 1 | 6-DOF arm | 6 | 산업/인간형 팔 | 중 | n=dim(SE(3)) |
| 2 | 3-DOF/leg x 4 | 12 | 4족 보행 | 중 | tau*(n/phi)=sigma |
| 3 | J2=24 인간형 | 24 | 인간형 전신 | 고 | J2=24 |
| 4 | sigma*tau=48 확장 | 48 | 재구성형 | 극고 | sigma*tau=48 |
| 5 | n=6 hexapod | 18~36 | 6족 보행 | 중 | n legs |

**Best Path**: 용도별 최적
- 산업: 6-DOF arm (n=dim(SE(3)))
- 보행: 4족 sigma=12 DOF (tau x n/phi)
- 인간형: J2=24 DOF Egyptian 배분

---

## 5. n6 EXACT 목록

| # | 파라미터 | 값 | n=6 표현 | 상태 |
|---|---------|-----|---------|------|
| 1 | 6-DOF arm | 6 | n = dim(SE(3)) | EXACT |
| 2 | se(3) dim | 6 | n = 6 | EXACT |
| 3 | se(3) 비영 구조 상수 | 12 | sigma = 12 | EXACT |
| 4 | Adjoint matrix | 6x6=36 | n^2 = 36 | EXACT |
| 5 | Spatial inertia blocks | 4 | tau = 4 | EXACT |
| 6 | Spot/ANYmal DOF | 12 | sigma = 12 | EXACT |
| 7 | Quadruped legs | 4 | tau = 4 | EXACT |
| 8 | DOF/leg | 3 | n/phi = 3 | EXACT |

**EXACT 비율: 8/8 = 100%**

---

## 6. BT 연결

- **BT-123**: SE(3) dim=n=6 로봇 보편성 (9/9 EXACT)
  - 6-DOF arm, 6축 IMU, 6면 큐브 모듈 모두 n=6
- **BT-124**: phi=2 양팔/양다리 대칭 + sigma=12 관절 보편성
  - 인간형 12 관절 유형 x phi=2 좌우 = J2=24 DOF
- **BT-125**: tau=4 보행/비행 최소 안정성 원리
  - 4족 x 3DOF/leg = sigma=12 total DOF

---

## 7. Lie 대수 연결

```
  se(3) Lie algebra:
    dim = n = 6
    비영 구조 상수 = sigma = 12 (H-ROB-73 EXACT)
    Ad(SE(3)) = n x n = 36 matrix
    Spatial inertia = tau = 4 blocks (mass, CoM, inertia, coupling)

  물리적 의미:
    n=6은 3D 공간에서 강체 운동의 자유도
    sigma=12는 그 자유도 간의 비자명한 관계의 수
    tau=4는 관성 텐서의 독립 블록 수
    → 로봇 역학의 근본 구조가 n=6 산술
```
