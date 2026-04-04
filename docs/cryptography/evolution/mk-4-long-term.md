# HEXA-CRYPT Mk.IV — Long-Term Cryptography (2050~2075)

**Evolution Checkpoint**: Mk.IV
**Date**: 2026-04-04
**Status**: 장기 비전
**Feasibility**: 🔮 30~50년 (양자 암호 + 정보이론 보안)
**BT Connections**: BT-114
**Delta vs Mk.III**: 양자 암호 통합, IT-secure 프로토콜

---

## 1. 목표

Mk.IV는 양자 키 분배(QKD)와 포스트-양자 암호의 통합으로 정보이론적 안전성을 달성한다.

---

## 2. 스펙

```
  ┌──────────────────────────────────────────────────────────────────┐
  │           HEXA-CRYPT Mk.IV — Long-Term Specs                    │
  ├──────────────┬──────────┬──────────────┬────────────────────────┤
  │ 파라미터      │ 목표     │ n=6 표현     │ 근거                   │
  ├──────────────┼──────────┼──────────────┼────────────────────────┤
  │ Security     │ IT-secure│ QKD+OTP     │ 정보이론 보안          │
  │ QKD rate     │ 1 Mbps   │ 10^n bps    │ 위성 QKD               │
  │ Key storage  │ quantum  │ 양자 메모리  │ 원자 앙상블            │
  │ FHE overhead │ φ = 2x   │ 거의 무오버  │ 전용 하드웨어          │
  │ Protocols    │ 6 layer  │ n = 6       │ QKD+PQ+FHE+MPC+ZK+OTP│
  └──────────────┴──────────┴──────────────┴────────────────────────┘
```

## 3. 필요 기술 돌파

1. 글로벌 QKD 인프라 (위성+광섬유)
2. 양자 메모리 장기 저장 (coherence > σ=12시간)
3. FHE 오버헤드 φ=2배 이하
4. 양자-클래식 하이브리드 보안 아키텍처 표준
5. 프라이버시 보존 AI 학습 (FHE+MPC 기반)
