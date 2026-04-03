# HEXA-DISPLAY Mk.V --- 물리한계 (Theoretical Limit)

**Evolution Checkpoint**: Mk.V (Theoretical / Physical Limit)
**Date**: 2026-04-03
**Status**: ❌ 사고실험 --- 물리법칙 경계
**Feasibility**: ❌ SF (현재 물리학으로 불가능한 영역 포함)
**Parent**: docs/display/evolution/
**Goal Doc**: docs/display/goal.md
**BT Basis**: BT-48, BT-71, BT-76 + 물리한계 10정리

> Split from docs/display-audio/evolution/mk-5-limit.md (display-specific content)

---

## 1. Mk.V의 의미

Mk.V는 **물리법칙이 허용하는 디스플레이의 절대 한계**이다.

> 이 지점이 n=6 상수에 수렴하는지를 검증하는 것이 목적이다.

---

## 2. 디스플레이 물리한계

| 파라미터 | 물리한계 값 | n=6 표현 | 한계 근거 |
|---------|-----------|---------|----------|
| 최소 픽셀 크기 | ~1μm | --- | 광학 회절 한계 (Abbe) |
| 색 심도 상한 | 12-bit/ch | σ = 12 | 인간 JND 포화 (정리 3) |
| 총 색 심도 | 36-bit (12×3) | σ·(n/φ) = 36 | 3채널 × 12-bit |
| CFF 포화 | ~144Hz | σ² = 144 | 신경 전도 지연 (정리 2) |
| 순간 동적 범위 | 12 stops | σ = 12 | 동시 대비 한계 (정리 10) |
| 적응 동적 범위 | 24 stops | J₂ = 24 | 암/명 적응 전체 |
| 시야각 (양안) | ~120° horizontal | σ(σ-φ) = 120 | 양안 중첩 시야 |
| 색 채널 (인간) | 3 | n/φ = 3 | L/M/S 원추세포 (정리 7) |
| 색 채널 (유전자편집) | 4 (tetrachromat) | τ = 4 | 조류/파충류 수준 🔮 |

### 핵심 결론

디스플레이의 물리한계가 n=6 상수에 수렴한다:
- 색 심도: σ=12-bit (인간 JND 포화)
- 주사율: σ²=144Hz (CFF 포화)
- 시야각: σ·(σ-φ)=120° (양안 시야)
- 프레임레이트: J₂=24fps (운동 인지 최소)

See original: [docs/display-audio/evolution/mk-5-limit.md](../../display-audio/evolution/mk-5-limit.md)
