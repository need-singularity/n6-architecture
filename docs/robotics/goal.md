# 궁극의 로봇 — DSE 후보군 정의

> 목표: 가사·간병·배달 로봇 최적 아키텍처
> 체인: 구조체 → 구동기 → 센서 → 제어 → 지능 → 시스템 (6단)
> 전수 조합: 6×6×5×5×5×5 = 22,500 → 호환 필터 → 6,472 유효

## DSE 체인

```
  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐
  │  구조체  │──▶│ 구동기   │──▶│  센서    │──▶│  제어    │──▶│  지능    │──▶│  시스템  │
  │  Body    │   │ Actuator │   │  Sensor  │   │ Control  │   │   AI     │   │ System   │
  │  K₁=6   │   │  K₂=6   │   │  K₃=5   │   │  K₄=5   │   │  K₅=5   │   │  K₆=5   │
  └──────────┘   └──────────┘   └──────────┘   └──────────┘   └──────────┘   └──────────┘
```

## n=6 핵심 패턴

```
  ┌────────────────────────────────────────────────────────────────┐
  │ SE(3) = n = 6 DOF: 모든 로봇의 기본 자유도                    │
  │ 6축 IMU = n: 최소 자세 추정                                   │
  │ Quadruped 4다리×3관절 = σ=12 DOF                             │
  │ Humanoid 4지×6관절 = J₂=24 DOF                               │
  │ Quadrotor τ=4 로터                                            │
  │ 12-bit PWM = σ, 24-bit DirectDrive = J₂                      │
  │ 배달 6대 = n, 공장 12대 = σ, 스웜 24대 = J₂                  │
  │ HEXA-1: 12W=σ, 6계층=n, 1ms=μ                               │
  └────────────────────────────────────────────────────────────────┘
```

## DSE 결과 요약

```
  전수: 22,500 조합
  유효: 6,472 조합
  n6 EXACT 최대: 76.9%
  n6 EXACT 평균: 46.3%

  ┌───────────────────────────────────────────────────────┐
  │ 가사·간병 최적 경로 (Top 3)                           │
  │                                                       │
  │ #1: Humanoid + Pneumatic + Basic-6IMU                 │
  │     + HEXA-1 + N6-Egyptian + Home-Care                │
  │     n6=69% | Perf=100 | Score=80.70                  │
  │                                                       │
  │ #2: Humanoid + BLDC-Gear + Basic-6IMU                 │
  │     + HEXA-1 + N6-Egyptian + Home-Care                │
  │     n6=73% | Perf=100 | Score=80.62                  │
  │                                                       │
  │ #3: Humanoid + Pneumatic + Basic-6IMU                 │
  │     + Cloud-Edge + N6-Egyptian + Home-Care            │
  │     n6=58% | Perf=100 | Score=80.11                  │
  ├───────────────────────────────────────────────────────┤
  │ 배달 최적 경로 (Top 3)                                │
  │                                                       │
  │ #1: Quadruped + Pneumatic + Basic-6IMU                │
  │     + HEXA-1 + N6-Egyptian + Delivery-6               │
  │     n6=69% | Perf=100 | Score=78.38                  │
  │                                                       │
  │ #2: ModularCube + Pneumatic + Basic-6IMU              │
  │     + HEXA-1 + N6-Egyptian + Delivery-6               │
  │     n6=65% | Perf=100 | Score=78.31                  │
  │                                                       │
  │ #3: Quadruped + BLDC-Gear + Basic-6IMU                │
  │     + HEXA-1 + N6-Egyptian + Delivery-6               │
  │     n6=73% | Perf=100 | Score=78.29                  │
  └───────────────────────────────────────────────────────┘
```

## Cross-DSE 후보

- **chip-architecture**: HEXA-1 칩 + 로봇 제어 SoC
- **learning-algorithm**: RL 정책 + Egyptian 리워드 분할
- **battery-architecture**: 로봇 배터리 최적화
